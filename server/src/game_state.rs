use std::collections::HashMap;

use clash::{spatula::Spatula, PlayerId};

pub struct GameState {
    pub spatulas: HashMap<Spatula, Option<PlayerId>>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            spatulas: HashMap::with_capacity(Spatula::COUNT),
        }
    }
}