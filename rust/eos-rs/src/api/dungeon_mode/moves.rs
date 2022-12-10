use crate::api::moves::{Move, MoveId};
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Extension trait for [`Move`] specific to dungeon mode.
pub trait DungeonMoveExt {
    /// Checks if the move isn't a physical move.
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if a move is a Hyper Beam variant that requires a a turn to recharge.
    ///        
    /// Include moves: Frenzy Plant, Hydro Cannon, Hyper Beam, Blast Burn, Rock Wrecker,
    /// Giga Impact, Roar of Time
    fn is_hyper_beam_variant(&self, _ov29: &OverlayLoadLease<29>) -> bool;
}

/// Extension trait for [`MoveId`] specific to dungeon mode.
pub trait DungeonMoveIdExt {
    /// Checks if the move isn't a physical move.
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool;
}

impl DungeonMoveExt for Move {
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        self.id.val().move_is_not_physical(_ov29)
    }

    fn is_hyper_beam_variant(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::IsHyperBeamVariant(force_mut_ptr!(self)) > 0 }
    }
}

impl DungeonMoveIdExt for MoveId {
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::MoveIsNotPhysical(*self) > 0 }
    }
}
