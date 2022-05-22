use std::collections::HashMap;

use crate::{game_state::GameState, player::SharedPlayer, LobbyId, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LobbyOptions {
    pub lab_door_cost: u8,
    pub ng_plus: bool,
}

impl Default for LobbyOptions {
    fn default() -> Self {
        Self {
            lab_door_cost: 75,
            ng_plus: false,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GamePhase {
    Setup,
    Playing,
    Finished,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SharedLobby {
    pub game_state: GameState,
    pub lobby_id: LobbyId,
    pub options: LobbyOptions,
    pub players: HashMap<PlayerId, SharedPlayer>,
    pub game_phase: GamePhase,
    pub host_id: Option<PlayerId>,
}

impl SharedLobby {
    pub fn new(lobby_id: u32, options: LobbyOptions) -> Self {
        Self {
            game_state: GameState::default(),
            lobby_id,
            options,
            players: HashMap::new(),
            game_phase: GamePhase::Setup,
            host_id: None,
        }
    }
}
