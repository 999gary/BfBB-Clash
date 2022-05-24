use std::collections::{HashMap, HashSet};

use crate::game::{GameInterface, InterfaceResult};
use clash::game_state::SpatulaTier;
use clash::lobby::{GamePhase, SharedLobby};
use clash::PlayerId;
use clash::{
    protocol::{Item, Message},
    room::Room,
    spatula::Spatula,
};
use log::info;
use strum::IntoEnumIterator;

use crate::game::InterfaceError;

pub trait GameStateExt {
    fn update<T: GameInterface>(
        &mut self,
        player_id: PlayerId,
        game: &T,
        network_sender: &mut tokio::sync::mpsc::Sender<Message>,
        local_spat_state: &mut HashSet<Spatula>,
    ) -> InterfaceResult<()>;

    fn can_start(&self) -> bool;
}

impl GameStateExt for SharedLobby {
    /// Process state updates from the server and report back any actions of the local player
    fn update<T: GameInterface>(
        &mut self,
        player_id: PlayerId,
        game: &T,
        network_sender: &mut tokio::sync::mpsc::Sender<Message>,
        local_spat_state: &mut HashSet<Spatula>,
    ) -> InterfaceResult<()> {
        if game.is_loading()? {
            return Ok(());
        }

        // TODO: Use a better error
        // Find the local player within the lobby
        let local_player = self
            .players
            .get_mut(&player_id)
            .ok_or(InterfaceError::Other)?;

        // Detect level changes
        let room = Some(game.get_current_level()?);
        if local_player.current_room != room {
            local_player.current_room = room;
            network_sender
                .blocking_send(Message::GameCurrentRoom { room })
                .unwrap();
        }

        // Don't proceed if the game is not active
        if self.game_phase != GamePhase::Playing {
            return Ok(());
        }

        // Set the cost to unlock the lab door
        if local_player.current_room == Some(Room::ChumBucket) {
            game.set_lab_door(self.options.lab_door_cost.into())?;
        }

        // Check for newly collected spatulas
        for spat in Spatula::iter() {
            // Skip already collected spatulas

            if local_spat_state.contains(&spat) {
                game.collect_spatula(spat)?;
                continue;
            }
            
            if let Some(spat_ref) = self.game_state.spatulas.get_mut(&spat) {
                if spat_ref.tier != SpatulaTier::Golden {
                    game.mark_task_complete(spat)?;
                }
                if spat_ref.tier == SpatulaTier::None {
                    if local_player.current_room == Some(spat.get_room()) {
                        // Sync collected spatulas
                        game.collect_spatula(spat)?;
                    }
                    continue;
                }
            }
            /*
            // Check menu for any potentially missed collection events
            if game.is_task_complete(spat)? && !hack {
                local_spat_state.insert(spat);
                network_sender
                    .blocking_send(Message::GameItemCollected {
                        item: Item::Spatula(spat),
                    })
                    .unwrap();
                info!("Collected (from menu) {spat:?}");
            }
            */

            // Skip spatulas that aren't in the current room
            if local_player.current_room != Some(spat.get_room()) {
                continue;
            }

            // Detect spatula collection events
            if game.is_spatula_being_collected(spat)? {
                local_spat_state.insert(spat);
                network_sender
                    .blocking_send(Message::GameItemCollected {
                        item: Item::Spatula(spat),
                    })
                    .unwrap();
                info!("Collected {spat:?}");
            }
        }

        Ok(())
    }

    /// True when all connected players are on the Main Menu
    fn can_start(&self) -> bool {
        // TODO: Solve the "Demo Cutscene" issue. We can probably detect when players are on the autosave preference screen instead.
        for player in self.players.values() {
            if player.current_room != Some(Room::MainMenu) {
                return false;
            }
        }
        true
    }
}
