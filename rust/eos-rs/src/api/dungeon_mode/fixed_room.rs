use crate::api::dungeons::FixedRoomId;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Extension trait for [`FixedRoomId`] specific to dungeon mode.
pub trait DungeonFixedRoomIdExt {
    /// Note: unverified, ported from Irdkwia's notes
    fn is_boss_fight(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if orbs are usable in the given fixed room.
    ///
    /// Always true if not a full-floor fixed room.
    fn are_orbs_allowed(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if tile jumps (warping, being blown away, and leaping) are allowed in the given fixed room.
    ///
    /// Always true if not a full-floor fixed room.
    fn are_tile_jumps_allowed(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if Trawl Orbs work in the given fixed room.
    ///
    /// Always true if not a full-floor fixed room.
    fn are_trawl_orbs_allowed(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Check if late-game traps (Summon, Pitfall, and Pokémon traps) work in the given fixed room.
    ///
    /// Or disabled? This function, which Irdkwia's notes label as a disable check, check
    /// the struct field labeled in End's notes as an enable flag.
    fn are_late_game_traps_enabled(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if moves (excluding the regular attack) are usable in the given fixed room.
    fn are_moves_enabled(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if the given fixed room is fully illuminated.
    fn is_room_illuminated(&self, _ov29: &OverlayLoadLease<29>) -> bool;
}

impl DungeonFixedRoomIdExt for FixedRoomId {
    fn is_boss_fight(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::IsBossFight(*self) > 0 }
    }

    fn are_orbs_allowed(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::AreOrbsAllowed(*self) > 0 }
    }

    fn are_tile_jumps_allowed(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::AreTileJumpsAllowed(*self) > 0 }
    }

    fn are_trawl_orbs_allowed(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::AreTrawlOrbsAllowed(*self) > 0 }
    }

    fn are_late_game_traps_enabled(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::AreLateGameTrapsEnabled(*self) > 0 }
    }

    fn are_moves_enabled(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::AreMovesEnabled(*self) > 0 }
    }

    fn is_room_illuminated(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::IsRoomIlluminated(*self) > 0 }
    }
}
