//! Structs and functions to interact with the data of monster types in a general context.

use crate::ffi;

/// A monster type ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type MonsterTypeId = ffi::type_id;
impl Copy for MonsterTypeId {}

/// This impl provides general metadata about monster types in the game.
impl MonsterTypeId {
    /// Returns the ID struct for the type with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing type),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this type.
    pub const fn id(&self) -> u32 {
        self.0
    }
}

impl From<MonsterTypeId> for u32 {
    fn from(v: MonsterTypeId) -> Self {
        v.0
    }
}
