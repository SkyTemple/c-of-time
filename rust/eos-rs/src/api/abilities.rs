//! Structs and functions to interact with the data of abilities in a general context.
//!
use crate::ffi;

/// An ability ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type AbilityId = ffi::ability_id;
impl Copy for AbilityId {}

/// This impl provides general metadata about abilities in the game.
impl AbilityId {
    /// Returns the ID struct for the ability with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing ability),
    /// otherwise this is UB.
    pub const unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this ability.
    pub const fn id(&self) -> u32 {
        self.0
    }
}
