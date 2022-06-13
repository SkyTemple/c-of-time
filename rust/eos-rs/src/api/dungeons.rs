//! Structs and functions to interact with the data of dungeons, dungeon groups and fixed rooms
//! in a general context.
use crate::ffi;

/// A dungeon ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type DungeonGroupId = ffi::dungeon_group_id;
impl Copy for DungeonGroupId {}

/// This impl provides general metadata about dungeons in the game.
impl DungeonGroupId {
    /// Returns the ID struct for the dungeon group with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing dungeon group),
    /// otherwise this is UB.
    pub unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this dungeon group.
    pub fn id(&self) -> u32 {
        self.0
    }
}

/// A dungeon group ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type DungeonId = ffi::dungeon_id;
impl Copy for DungeonId {}

/// This impl provides general metadata about dungeons in the game.
impl DungeonId {
    /// Returns the ID struct for the dungeon with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing dungeon),
    /// otherwise this is UB.
    pub unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this dungeon.
    pub fn id(&self) -> u32 {
        self.0
    }
}

/// A fixed room ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type FixedRoomId = ffi::fixed_room_id;
impl Copy for FixedRoomId {}

/// This impl provides general metadata about fixed rooms in the game.
impl FixedRoomId {
    /// Returns the ID struct for the fixed room with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing fixed room),
    /// otherwise this is UB.
    pub unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this fixed room.
    pub fn id(&self) -> u32 {
        self.0
    }
}
