//! Structs and functions to interact with the data of dungeons, dungeon groups and fixed rooms
//! in a general context.
use crate::api::overlay::OverlayLoadLease;
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
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this dungeon group.
    pub const fn id(&self) -> u32 {
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
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this dungeon.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Returns whether this dungeon is considered as going upward or not
    pub fn goes_up(&self) -> bool {
        unsafe { ffi::DungeonGoesUp(*self) > 0 }
    }

    /// Returns the maximum rescue attempts allowed in this dungeon,
    /// or -1 if rescues are disabled.
    pub fn get_max_rescue_attempts(&self) -> i8 {
        unsafe { ffi::GetMaxRescueAttempts(*self) }
    }

    /// Returns whether this dungeon as a joined at location is between
    /// [`DungeonId::DUNGEON_JOINED_AT_BIDOOF`] and [`DungeonId::DUNGEON_DUMMY_0xE3`].
    pub fn is_special_joined_at_location(&self) -> bool {
        unsafe {
            ffi::JoinedAtRangeCheck(ffi::dungeon_id_8 {
                _bitfield_align_1: [],
                _bitfield_1: ffi::dungeon_id_8::new_bitfield_1(*self),
            }) > 0
        }
    }

    /// Returns whether a game over should happen when a monster with this dungeon ID as
    /// "joined at" value faints (as long as the other conditions are met).
    /// It might have a more generic meaning.
    pub fn should_cause_game_over_on_faint(&self) -> bool {
        unsafe {
            ffi::ShouldCauseGameOverOnFaint(ffi::dungeon_id_8 {
                _bitfield_align_1: [],
                _bitfield_1: ffi::dungeon_id_8::new_bitfield_1(*self),
            }) > 0
        }
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
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this fixed room.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Checks if this ID corresponds to a fixed, full-floor layout.
    pub fn is_full_floor_fixed_room(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsNotFullFloorFixedRoom(*self) == 0 }
    }
}
