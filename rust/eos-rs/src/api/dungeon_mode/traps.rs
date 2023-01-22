use crate::ffi;
/// The ID of a trap.
pub type TrapId = ffi::trap_id;
impl Copy for TrapId {}

/// This impl provides general metadata about traps in the game.
impl TrapId {
    /// Returns the ID struct for the trap with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing trap),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }
}

impl From<TrapId> for u32 {
    fn from(v: TrapId) -> Self {
        v.0
    }
}
