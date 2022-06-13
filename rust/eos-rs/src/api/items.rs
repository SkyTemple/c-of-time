//! Structs and functions to interact with the data of items in a general context.

use crate::ffi;

/// An item ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type ItemId = ffi::exclusive_item_effect_id;
impl Copy for ItemId {}

/// This impl provides general metadata about items in the game.
impl ItemId {
    /// Returns the ID struct for the item with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing item),
    /// otherwise this is UB.
    pub unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this item.
    pub fn id(&self) -> u32 {
        self.0
    }
}

/// An exclusive item effect ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type ExclusiveItemEffectId = ffi::exclusive_item_effect_id;

/// This impl provides general metadata about exclusive item effects in the game.
impl ExclusiveItemEffectId {
    /// Returns the ID struct for the exclusive item effect with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing exclusive item effect),
    /// otherwise this is UB.
    pub unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this exclusive item effect.
    pub fn id(&self) -> u32 {
        self.0
    }
}
