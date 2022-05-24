use clash::{room::Room, spatula::Spatula};
use thiserror::Error;

#[derive(Copy, Clone, Debug, Error)]
pub enum InterfaceError {
    #[error("Interface became unhooked")]
    Unhooked,
    #[error("Interface operation failed")]
    Other,
}

impl From<std::io::Error> for InterfaceError {
    fn from(e: std::io::Error) -> Self {
        // For now, treat any error other than InvalidData as being unhooked
        if e.kind() == std::io::ErrorKind::InvalidData {
            return Self::Other;
        }
        Self::Unhooked
    }
}

pub type InterfaceResult<T> = std::result::Result<T, InterfaceError>;

/// Trait for interacting with BfBB in an abstract way.
pub trait GameInterface {
    /// True if the game is currently in a loading screen.
    fn is_loading(&self) -> InterfaceResult<bool>;

    /// Will start a new game when called. Only works when the player is on the main menu and not in the demo cutscene.
    fn start_new_game(&self) -> InterfaceResult<()>;

    /// Unlock the Bubble Bowl and Cruise Bubble
    fn unlock_powers(&self) -> InterfaceResult<()>;

    /// Get the level that the player is currently in
    fn get_current_level(&self) -> InterfaceResult<Room>;

    /// Gets the player's total spatula count
    fn get_spatula_count(&self) -> InterfaceResult<u32>;

    /// Sets the player's total spatula count to `value`
    fn set_spatula_count(&self, value: u32) -> InterfaceResult<()>;

    /// Marks a spatula as "completed" in the pause menu. This has the effect of giving the player access to the task warp.
    fn mark_task_complete(&self, spatula: Spatula) -> InterfaceResult<()>;

    /// True when `spatula` is shown as gold in the pause menu.
    fn is_task_complete(&self, spatula: Spatula) -> InterfaceResult<bool>;

    /// Collect a spatula in the world. This only removes the entity, it will not complete the task or increment the spatula
    /// counter.
    /// # Returns:
    /// `Ok(())` for [Kah-Rah-Tae](clash::spatula::Spatula::KahRahTae) and [The Small Shall Rule... Or Not](Spatula::TheSmallShallRuleOrNot)
    /// without writing memory.
    fn collect_spatula(&self, spatula: Spatula, current_room: Option<Room>) -> InterfaceResult<()>;

    /// True when `spatula`'s collected animation is playing
    /// # Returns:
    /// `Ok(false)` for [Kah-Rah-Tae](clash::spatula::Spatula::KahRahTae) and [The Small Shall Rule... Or Not](Spatula::TheSmallShallRuleOrNot)
    fn is_spatula_being_collected(
        &self,
        spatula: Spatula,
        current_room: Option<Room>,
    ) -> InterfaceResult<bool>;

    /// Changes the number of spatulas required to enter the Chum Bucket Lab.
    fn set_lab_door(&self, value: u32, current_room: Option<Room>) -> InterfaceResult<()>;
}
