//! Traits, enums, structs and functions related to dungeon mode.

mod dungeon_struct;
mod effects;
mod entity;
mod enums;
mod message_log;
mod monster;
mod moves;
mod random;
mod tile;

pub mod dungeon_generator;

pub use dungeon_struct::*;
pub use effects::*;
pub use entity::*;
pub use enums::*;
pub use message_log::*;
pub use monster::*;
pub use moves::*;
pub use random::*;
pub use tile::*;

use crate::api::overlay::OverlayLoadLease;
use crate::ctypes::*;
use crate::ffi;

// Misc dungeon functions.

/// Seems to zero some damage description struct, which is output by the damage
/// calculation function.
///
/// # Safety
/// This resets some global data. The caller needs to make sure pointers to this space
/// are set up correctly and no references to the area exist.
pub unsafe fn reset_damage_desc(damage_desc: *mut ffi::undefined4, _ov29: &OverlayLoadLease<29>) {
    ffi::ResetDamageDesc(damage_desc);
}

/// [`crate::api::objects::DungeonMonsterRef::calc_damage`] seems to use scratch space of
/// some kind, which this function zeroes.
///
/// # Safety
/// This resets some global data. The caller needs to make sure pointers to this space
/// are set up correctly and no references to the area exist.
pub unsafe fn reset_damage_calc_scratch_space(_ov29: &OverlayLoadLease<29>) {
    ffi::ResetDamageCalcScratchSpace();
}

/// This changes the palettes of windows in both screens to an appropriate value depending on
/// the playthrough.
///
/// If you're in a special episode, they turn green , otherwise, they turn blue or pink depending
/// on your character's sex
///
pub fn set_both_screens_window_color_to_default(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::SetBothScreensWindowColorToDefault() }
}
