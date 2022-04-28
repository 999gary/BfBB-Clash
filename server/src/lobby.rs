use clash::lobby::{GamePhase, LobbyOptions, SharedLobby};
use clash::player::{PlayerOptions, SharedPlayer};
use clash::protocol::Message;
use clash::{LobbyId, PlayerId, MAX_PLAYERS};
use thiserror::Error;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::{channel, Receiver};

use crate::state::PlayerMap;

#[derive(Copy, Clone, Debug, Error)]
pub enum LobbyError {
    #[error("Attempted to add a player to a full lobby")]
    LobbyFull,
    #[error("Attempted to add a invalid player to a lobby")]
    PlayerInvalid,
}

pub struct Lobby {
    pub shared: SharedLobby,
    pub sender: Sender<Message>,
    pub next_menu_order: u8,
}

impl Lobby {
    pub fn new(new_options: LobbyOptions, lobby_id: LobbyId) -> Self {
        let (sender, _) = channel(100);
        Self {
            shared: SharedLobby::new(lobby_id, new_options),
            sender,
            next_menu_order: 0,
        }
    }

    pub fn reset_game(&mut self) {
        self.shared.game_state.reset_state();
        for (_, player) in self.shared.players.iter_mut() {
            player.score = 0;
        }
    }

    pub fn start_game(&mut self) {
        self.reset_game();
        self.shared.game_phase = GamePhase::Playing;
        let _ = self.sender.send(Message::GameLobbyInfo {
            lobby: self.shared.clone(),
        });
        if self.sender.send(Message::GameBegin).is_err() {
            log::warn!(
                "Lobby {:#X} started with no players in lobby.",
                self.shared.lobby_id
            )
        }
    }

    pub fn stop_game(&mut self) {
        self.shared.game_phase = GamePhase::Setup;
        let _ = self.sender.send(Message::GameLobbyInfo {
            lobby: self.shared.clone(),
        });
        if self.sender.send(Message::GameEnd).is_err() {
            log::warn!(
                "Lobby {:#X} started with no players in lobby.",
                self.shared.lobby_id
            )
        }
    }

    pub fn is_player_in_lobby(&mut self, player_id: &u32) -> bool {
        self.shared.players.contains_key(player_id)
    }

    /// Adds a new player to this lobby. If there is currently no host, they will become it.
    pub fn add_player(
        &mut self,
        players: &mut PlayerMap,
        player_id: PlayerId,
    ) -> Result<Receiver<Message>, LobbyError> {
        if self.shared.players.len() >= MAX_PLAYERS {
            return Err(LobbyError::LobbyFull);
        }

        if self.is_player_in_lobby(&player_id) {
            return Err(LobbyError::PlayerInvalid);
        }

        // Make sure the player isn't already in a different lobby
        // TODO: The lobby probably shouldn't be responsible for this
        if players.get(&player_id).is_some() {
            return Err(LobbyError::PlayerInvalid);
        }
        players.insert(player_id, self.shared.lobby_id);

        // TODO: Unhardcode player color
        let mut player = SharedPlayer::new(PlayerOptions::default(), self.next_menu_order);
        player.options.color = clash::player::COLORS[self.shared.players.len()];
        self.next_menu_order += 1;

        self.shared.players.insert(player_id, player);
        if self.shared.host_id == None {
            self.shared.host_id = Some(player_id);
        }

        // Subscribe early so that this player will receive the lobby update that adds them
        let recv = self.sender.subscribe();

        let _ = self.sender.send(Message::GameLobbyInfo {
            lobby: self.shared.clone(),
        });

        Ok(recv)
    }

    // Removes a player from the lobby, if it exists, returning the number of player's remaining
    pub fn rem_player(&mut self, player_id: PlayerId) -> usize {
        self.shared.players.remove(&player_id);
        if self.shared.host_id == Some(player_id) {
            // Pass host to first remaining player in list (effectively random with a HashMap)
            // NOTE: We could consider passing host based on join order
            self.shared.host_id = self.shared.players.iter().next().map(|(&id, _)| id);
        }

        // Update remaining clients of the change
        let _ = self.sender.send(Message::GameLobbyInfo {
            lobby: self.shared.clone(),
        });
        self.shared.players.len()
    }
}
