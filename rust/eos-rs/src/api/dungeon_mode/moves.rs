use crate::api::objects::*;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Extension trait for [`Move`] specific to dungeon mode.
pub trait DungeonMoveExt {
    /// Checks if the move isn't a physical move.
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if the move isn't a physical move.
    fn move_is_not_physical_static(
        move_id: move_catalog::Type,
        _ov29: &OverlayLoadLease<29>,
    ) -> bool;
}

impl DungeonMoveExt for Move {
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        Self::move_is_not_physical_static(self.id.val(), _ov29)
    }

    fn move_is_not_physical_static(
        move_id: move_catalog::Type,
        _ov29: &OverlayLoadLease<29>,
    ) -> bool {
        unsafe { ffi::MoveIsNotPhysical(move_id) > 0 }
    }
}
