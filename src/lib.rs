use game_state::SpatulaTier;
use strum::{EnumCount};

pub mod game_state;
pub mod lobby;
pub mod player;
pub mod protocol;
pub mod room;
pub mod spatula;

pub const MAX_PLAYERS: usize = 6;

pub struct GameRuleConsts {
    pub spat_scores: [u32; SpatulaTier::COUNT],

}

pub const GAME_CONSTS: GameRuleConsts = GameRuleConsts {
    spat_scores: [300, 100, 50, 0]
};

// NOTE: We can considering using the newtype pattern here to avoid the possiblity of mixing up these id types,
//       but it adds a lot of boilerplate and I'm not sure that it's actually worth it at this point.
pub type PlayerId = u32;
pub type LobbyId = u32;
