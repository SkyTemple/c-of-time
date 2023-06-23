//! Structs and functions to interact with the data of dungeons, dungeon groups and fixed rooms
//! in a general context.
use crate::api::items::ItemId;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;
use core::mem::MaybeUninit;

/// A dungeon ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
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

impl From<DungeonGroupId> for u32 {
    fn from(v: DungeonGroupId) -> Self {
        v.0
    }
}

/// A dungeon group ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
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

    /// Returns the number of floors of the given dungeon.
    ///
    /// The result is hardcoded for certain dungeons, such as dojo mazes.
    pub fn number_floors(&self) -> i32 {
        unsafe { ffi::GetNbFloors(*self) }
    }

    /// Returns the dungeon group associated to the given dungeon.
    ///
    /// For IDs greater or equal to [`DungeonId::DUNGEON_NORMAL_FLY_MAZE`],
    /// returns [`DungeonGroupId::DGROUP_MAROWAK_DOJO`].
    pub fn group(&self) -> DungeonGroupId {
        unsafe { ffi::GetDungeonGroup(*self) }
    }

    /// Given a dungeon ID, returns the total amount of floors summed by all the previous dungeons
    /// in its group.
    ///
    /// The value is normally pulled from
    /// [`ffi::dungeon_data_list_entry::n_preceding_floors_group`], except for dungeons with an
    /// ID >= [`DungeonId::DUNGEON_NORMAL_FLY_MAZE`], for which this function always returns 0.
    pub fn number_preceding_floors(&self) -> i32 {
        unsafe { ffi::GetNbPrecedingFloors(*self) }
    }

    /// Returns the total amount of floors among all the dungeons in the dungeon group of the
    /// specified dungeon.
    pub fn number_floors_in_group(&self) -> i32 {
        unsafe { ffi::GetNbFloorsDungeonGroup(*self) }
    }

    /// Given this dungeon ID and a floor number, returns a struct with the corresponding dungeon
    /// group and floor number in that group.
    ///
    /// The function normally uses the data in mappa_s.bin to calculate the result, but there's
    /// some dungeons (such as dojo mazes) that have hardcoded return values.
    pub fn conv_floor_to_group_floor(&self, dungeon_floor_number: u8) -> (DungeonGroupId, u8) {
        let mut out: MaybeUninit<ffi::dungeon_group_and_group_floor> = MaybeUninit::zeroed();
        let mut inp = ffi::dungeon_floor_pair {
            dungeon_id: ffi::dungeon_id_8 {
                _bitfield_align_1: [],
                _bitfield_1: ffi::dungeon_id_8::new_bitfield_1(*self),
            },
            floor_id: dungeon_floor_number,
        };
        unsafe {
            ffi::DungeonFloorToGroupFloor(out.as_mut_ptr(), &mut inp);
            let out = out.assume_init();
            (out.group_id.val(), out.group_floor)
        }
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

    /// Returns true if the flag that allows changing leaders is set in the restrictions of this
    /// dungeon.
    pub fn get_leader_change_flag(&self) -> bool {
        unsafe { ffi::GetLeaderChangeFlag(*self) > 0 }
    }

    /// Returns whether this dungeon has a joined at location between
    /// [`DungeonId::DUNGEON_JOINED_AT_BIDOOF`] and [`DungeonId::DUNGEON_DUMMY_0xE3`].
    pub fn is_special_joined_at_location(&self) -> bool {
        unsafe {
            ffi::JoinedAtRangeCheck(ffi::dungeon_id_8 {
                _bitfield_align_1: [],
                _bitfield_1: ffi::dungeon_id_8::new_bitfield_1(*self),
            }) > 0
        }
    }

    /// Returns whether a certain joined_at field value is equal to [`DungeonId::DUNGEON_BEACH`] or
    /// is between [`DungeonId::DUNGEON_DUMMY_0xEC`] and [`DungeonId::DUNGEON_DUMMY_0xF0`].
    pub fn is_special_joined_at_location2(&self) -> bool {
        unsafe {
            ffi::JoinedAtRangeCheck2(ffi::dungeon_id_8 {
                _bitfield_align_1: [],
                _bitfield_1: ffi::dungeon_id_8::new_bitfield_1(*self),
            }) > 0
        }
    }

    /// Checks if enemy Treasure Box drops are enabled in the dungeon.
    pub fn are_treasure_box_drops_enabled(&self) -> bool {
        unsafe { ffi::TreasureBoxDropsEnabled(*self) > 0 }
    }

    /// Returns whether the given item is available in the dungeon (group?).
    pub fn is_item_available(&self, item_id: ItemId) -> bool {
        unsafe { ffi::IsItemAvailableInDungeonGroup(*self, item_id) > 0 }
    }
}

impl From<DungeonId> for u32 {
    fn from(v: DungeonId) -> Self {
        v.0
    }
}

/// A fixed room ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
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

impl From<FixedRoomId> for u32 {
    fn from(v: FixedRoomId) -> Self {
        v.0
    }
}
