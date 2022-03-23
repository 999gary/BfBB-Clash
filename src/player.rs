use serde::{Deserialize, Serialize};

use crate::room::Room;

pub const COLORS: [(u8, u8, u8); 6] = [
    (195, 247, 58),
    (224, 108, 0),
    (89, 155, 108),
    (16, 130, 168),
    (176, 142, 184),
    (254, 154, 95),
];

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct PlayerOptions {
    pub name: String,
    pub color: (u8, u8, u8),
    // Other options?
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct SharedPlayer {
    pub options: PlayerOptions,
    pub current_room: Option<Room>,
    pub score: u8,
}

impl SharedPlayer {
    pub fn new(options: PlayerOptions) -> Self {
        Self {
            options,
            current_room: None,
            score: 0,
        }
    }
}

impl PlayerOptions {
    pub fn color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.color.0, self.color.1, self.color.2)
    }
}
