use crate::state::State;
use anyhow::Context;
use clash::game_state::{SpatulaState, SpatulaTier};
use clash::protocol::{self, Connection, Item, Message, ProtocolError};
use clash::spatula::Spatula;
use clash::{PlayerId, GAME_CONSTS};
use std::collections::hash_map::Entry;
use std::ops::IndexMut;
use std::sync::{Arc, RwLock};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::Receiver;

pub struct Client {
    state: Arc<RwLock<State>>,
    connection: Connection,
    player_id: PlayerId,
    lobby_recv: Option<Receiver<Message>>,
}

impl Client {
    pub async fn new(
        state: Arc<RwLock<State>>,
        socket: TcpStream,
    ) -> Result<Self, protocol::FrameError> {
        // Add new player
        let player_id = {
            let mut state = state.write().unwrap();
            state.add_player()
        };

        Ok(Self {
            state,
            connection: Connection::new(socket),
            player_id,
            lobby_recv: None,
        })
    }

    pub async fn run(mut self) {
        loop {
            select! {
                m = async { self.lobby_recv.as_mut().unwrap().recv().await }, if self.lobby_recv.is_some() => {
                    if let Ok(m) = m {
                        self.connection.write_frame(m).await.unwrap();
                    }
                }
                incoming = self.connection.read_frame() => {
                    let incoming = match incoming {
                        Ok(Some(x)) => x,
                        Ok(None) => {
                            log::info!("Player id {:#X} disconnected", self.player_id);
                            break;
                        }
                        Err(e) => {
                            log::error!(
                                "Error reading message from player id {:#X}. Closing connection\n{e:?}", self.player_id
                            );
                            break;
                        }
                    };

                    log::debug!("Received message from player id {:#X} \nMessage: {incoming:#X?}", self.player_id,);
                    if let Err(e) = self.process_incoming(incoming).await {
                        match e.downcast_ref::<ProtocolError>() {
                            Some(m @ ProtocolError::InvalidLobbyId(_)) |
                            Some(m @ ProtocolError::InvalidMessage) => {
                                log::error!("{e:?}");
                                let _ = self.connection.write_frame(Message::Error {error: m.clone() }).await;
                            },
                            Some(m @ ProtocolError::VersionMismatch(_,_)) => {
                                log::error!("{e:?}");
                                let _ = self.connection.write_frame(Message::Error {error: m.clone() }).await;
                                break;
                            }
                            Some(ProtocolError::Disconnected) => {
                                // Close the connection without error
                                log::info!("Player id {:#X} disconnected", self.player_id);
                                break;
                            }
                            _ => {
                                // This error means there is a problem with our internal state
                                // TODO: Figure out how to properly resolve this error
                                log::error!("Disconnecting player {:#X} due to unrecoverable error:\n{e:?}", self.player_id);
                                break;
                            }
                        }
                    }
                }
            };
        }
    }

    async fn process_incoming(&mut self, incoming: Message) -> anyhow::Result<()> {
        match incoming {
            Message::Version { version } => {
                if version != crate::VERSION {
                    return Err(
                        ProtocolError::VersionMismatch(version, crate::VERSION.to_owned()).into(),
                    );
                }

                // Inform player of their PlayerId
                self.connection
                    .write_frame(Message::ConnectionAccept {
                        player_id: self.player_id,
                    })
                    .await?;
                log::info!("New connection for player id {:#X} opened", self.player_id);
            }
            Message::GameHost => {
                let state = &mut *self.state.write().unwrap();
                if state.players.contains_key(&self.player_id) {
                    return Err(ProtocolError::InvalidMessage.into());
                }

                let lobby_id = state.add_lobby();
                let lobby = state
                    .lobbies
                    .get_mut(&lobby_id)
                    .ok_or(ProtocolError::InvalidLobbyId(lobby_id))?;

                self.lobby_recv = Some(lobby.add_player(&mut state.players, self.player_id)?);

                log::info!(
                    "Player {:#X} has hosted lobby {:#X}",
                    self.player_id,
                    lobby.shared.lobby_id
                );
            }
            Message::GameJoin { lobby_id } => {
                let state = &mut *self.state.write().unwrap();
                if state.players.contains_key(&self.player_id) {
                    return Err(ProtocolError::InvalidMessage.into());
                }

                let lobby = state
                    .lobbies
                    .get_mut(&lobby_id)
                    .ok_or(ProtocolError::InvalidLobbyId(lobby_id))?;

                self.lobby_recv = Some(lobby.add_player(&mut state.players, self.player_id)?);

                log::info!(
                    "Player {:#X} has joined lobby {lobby_id:#X}",
                    self.player_id
                );
            }
            Message::GameBegin => {
                let state = &mut *self.state.write().unwrap();
                let lobby = state.get_lobby(self.player_id)?;
                //TODO: Check if we are already in game.
                lobby.start_game();
            }
            Message::GameLeave => {
                // Remove player from their lobby
                let state = &mut *self.state.write().unwrap();

                self.leave_lobby(state);
            }
            Message::PlayerOptions { mut options } => {
                let state = &mut *self.state.write().unwrap();
                let lobby = state.get_lobby(self.player_id)?;

                let player = lobby
                    .shared
                    .players
                    .get_mut(&self.player_id)
                    .ok_or(ProtocolError::InvalidPlayerId(self.player_id))
                    .context("Player not found in lobby specified by the playerlist")?;

                // TODO: Unhardcode player color
                options.color = player.options.color;
                player.options = options;

                let message = Message::GameLobbyInfo {
                    lobby: lobby.shared.clone(),
                };
                let _ = lobby.sender.send(message);
            }
            Message::GameOptions { options } => {
                let state = &mut *self.state.write().unwrap();
                let lobby = state.get_lobby(self.player_id)?;

                if lobby.shared.host_id != Some(self.player_id) {
                    return Err(ProtocolError::InvalidMessage)
                        .context("Only the host can change Lobby Options");
                }
                lobby.shared.options = options;

                let message = Message::GameLobbyInfo {
                    lobby: lobby.shared.clone(),
                };
                let _ = lobby.sender.send(message);
            }
            Message::GameCurrentRoom { room } => {
                let state = &mut *self.state.write().unwrap();
                let lobby = state.get_lobby(self.player_id)?;

                let player = lobby
                    .shared
                    .players
                    .get_mut(&self.player_id)
                    .ok_or(ProtocolError::InvalidPlayerId(self.player_id))
                    .context("Player not found in lobby specified by the playerlist")?;

                player.current_room = room;
                log::info!("Player {:#X} entered {room:?}", self.player_id);

                let message = Message::GameLobbyInfo {
                    lobby: lobby.shared.clone(),
                };
                let _ = lobby.sender.send(message);
            }
            Message::GameItemCollected { item } => {
                let state = &mut *self.state.write().unwrap();
                let lobby = state.get_lobby(self.player_id)?;
                let tier_count = 3;

                match item {
                    Item::Spatula(spat) => {
                        let e = match lobby.shared.game_state.spatulas.get_mut(&spat) {
                            Some(spat) => spat,
                            None => {
                                log::info!("adding spat");
                                lobby.shared.game_state.spatulas.insert(spat, SpatulaState::default());
                                lobby.shared.game_state.spatulas.get_mut(&spat).unwrap()
                            }
                        };
                        if e.tier != SpatulaTier::None {
                            let mut score = lobby.shared.game_state.scores.get_mut(&self.player_id).unwrap_or(&mut 0).clone();
                            score += GAME_CONSTS.spat_scores[e.tier.clone() as usize];
                            lobby.shared.game_state.scores.insert(self.player_id, score);

                            e.collection_vec.insert(e.tier.clone() as usize, Some(self.player_id.clone()));
                            log::info!("Player {:#X} collected {spat:?} with tier {:?}", self.player_id, e.tier);

                            //This probably could be better.
                            e.tier = SpatulaTier::from(e.tier.clone() as i32 + 1);
                            if e.tier.clone() as i32 >= tier_count {
                                e.tier = SpatulaTier::None
                            }

                            if spat == Spatula::TheSmallShallRuleOrNot {
                                lobby.stop_game();
                            }

                            let message = Message::GameLobbyInfo {
                                lobby: lobby.shared.clone(),
                            };
                            let _ = lobby.sender.send(message);
                        }
                    }
                }
            }
            _ => {
                return Err(ProtocolError::InvalidMessage)
                    .context("Client sent a server-only message.")
            }
        }
        Ok(())
    }

    fn leave_lobby(&self, state: &mut State) {
        if let Entry::Occupied(e) = state.players.entry(self.player_id) {
            let lobby_id = e.remove();

            if let Entry::Occupied(mut lobby) = state.lobbies.entry(lobby_id) {
                if lobby.get_mut().rem_player(self.player_id) == 0 {
                    // Remove this lobby from the server
                    log::info!("Closing lobby {:#X}", lobby.key());
                    lobby.remove();
                }
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // This will crash the program if we're dropping due to a previous panic caused by a poisoned lock,
        // and that's fine for now.
        let state = &mut *self.state.write().unwrap();
        self.leave_lobby(state);
    }
}
