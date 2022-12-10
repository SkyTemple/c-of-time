//! Traits, enums, structs and functions related to dungeon mode.

mod dungeon_struct;
mod effects;
mod entity;
mod message_log;
mod monster;
mod moves;
mod random;
mod tile;

pub mod animations;
pub mod dungeon_generator;
pub mod fixed_room;
pub mod items;
pub mod menus;
pub mod sprites;
pub mod traps;

use crate::api::enums::Direction;
use core::ptr;
pub use dungeon_struct::*;
pub use effects::*;
pub use entity::*;
pub use message_log::*;
pub use monster::*;
pub use moves::*;
pub use random::*;
pub use tile::*;

use crate::api::overlay::OverlayLoadLease;
use crate::ctypes::*;
use crate::ffi;

// Misc dungeon functions.

/// Zeroes the damage data struct, which is output by the damage calculation function.
pub fn reset_damage_data(damage_data: &mut ffi::damage_data, _ov29: &OverlayLoadLease<29>) {
    unsafe {
        ffi::ResetDamageData(damage_data);
    }
}

/// [`DungeonMonsterRef::calc_damage`] seems to use scratch space of
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

/// Fades the screen to black across several frames.
pub fn fade_to_black(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::FadeToBlack() }
}

/// Advances one frame. Does not return until the next frame starts.
pub fn advance_frame(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::AdvanceFrame(0 as ffi::undefined) }
}

/// Gets the direction in which a monster should move to go from the origin position to the target position.
///
/// Returns None if the direction returned is somehow invalid.
pub fn get_direction_towards_position(
    _ov29: &OverlayLoadLease<29>,
    pos1: &ffi::position,
    pos2: &ffi::position,
) -> Option<Direction> {
    unsafe { ffi::GetDirectionTowardsPosition(force_mut_ptr!(pos1), force_mut_ptr!(pos2)) }
        .try_into()
        .ok()
}

/// Returns the Chebyshev distance between two positions. Calculated as max(abs(x0-x1), abs(y0-y1)).
pub fn get_chebyshev_distance(
    _ov29: &OverlayLoadLease<29>,
    pos1: &ffi::position,
    pos2: &ffi::position,
) -> i32 {
    unsafe { ffi::GetChebyshevDistance(force_mut_ptr!(pos1), force_mut_ptr!(pos2)) }
}

/// Checks if a given target position is in sight from a given origin position.
///
/// There's multiple factors that affect this check, but generally, it's true if both
/// positions are in the same room or within 2 tiles of each other.
pub fn is_position_in_sight(
    _ov29: &OverlayLoadLease<29>,
    origin: &ffi::position,
    target: &ffi::position,
    user_has_dropeye: bool,
) -> bool {
    unsafe {
        ffi::IsPositionInSight(
            force_mut_ptr!(origin),
            force_mut_ptr!(target),
            user_has_dropeye as ffi::bool_,
        ) > 0
    }
}

/// Graphically displays any pending actions that have happened but haven't been shown on screen
/// yet.
///
/// All actions are displayed at the same time. For example, this delayed display system is used
/// to display multiple monsters moving at once even though they take turns sequentially.
///
/// Seems to return true if there were any pending actions to display.
pub fn display_actions(_ov29: &OverlayLoadLease<29>, entity: Option<&DungeonEntity>) -> bool {
    let ptr = entity.map(|e| force_mut_ptr!(e)).unwrap_or(ptr::null_mut());
    unsafe { ffi::DisplayActions(ptr) > 0 }
}

/// Do the stuff when you lose in a dungeon.
///
/// end_cond: End condition code? Seems to control what tasks get run and what transition happens
///           when the dungeon ends.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn check_end_dungeon(end_cond: i32, _ov10: &OverlayLoadLease<10>) -> i32 {
    unsafe { ffi::CheckEndDungeon(end_cond) }
}

/// Note: unverified, ported from Irdkwia's notes
pub fn change_dungeon_music(_ov29: &OverlayLoadLease<29>, music_id: ffi::music_id::Type) {
    unsafe { ffi::ChangeDungeonMusic(music_id) }
}

/// Counts the number of digits in a nonnegative integer.
///
/// If the number is negative, it is cast to a u16 before counting digits.
pub fn digit_count(_ov29: &OverlayLoadLease<29>, n: i32) -> i32 {
    unsafe { ffi::DigitCount(n) }
}

/// Note: unverified, ported from Irdkwia's notes
pub fn load_texture_ui(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::LoadTextureUi() }
}

/// Note: unverified, ported from Irdkwia's notes
///
/// Returns xsize.
pub fn display_number_texture_ui(
    _ov29: &OverlayLoadLease<29>,
    x: i16,
    y: i16,
    n: i32,
    ally_mode: i32,
) -> i32 {
    unsafe { ffi::DisplayNumberTextureUi(x, y, n, ally_mode) }
}

/// Note: unverified, ported from Irdkwia's notes
///
/// Return unknown.
///
/// # Safety
/// The caller must make sure the undefined params are valid for this function.
pub unsafe fn display_char_texture_ui(
    call_back_str: *mut ffi::undefined,
    x: i16,
    y: i16,
    char_id: i32,
    param_5: i16,
) -> i32 {
    ffi::DisplayCharTextureUi(call_back_str, x, y, char_id, param_5)
}

/// Note: unverified, ported from Irdkwia's notes
pub fn display_ui(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::DisplayUi() }
}

/// Note: unverified, ported from Irdkwia's notes
pub fn open_fixed_bin(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::OpenFixedBin() }
}

/// Note: unverified, ported from Irdkwia's notes
pub fn close_fixed_bin(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::CloseFixedBin() }
}

/// Note: unverified, ported from Irdkwia's notes
///
/// # Safety
/// The caller must make sure the undefined params are valid for this function.
pub unsafe fn open_menu(
    _ov29: &OverlayLoadLease<29>,
    param_1: ffi::undefined4,
    param_2: ffi::undefined4,
    param_3: bool,
    param_4: ffi::undefined4,
) {
    ffi::OpenMenu(param_1, param_2, param_3 as ffi::bool_, param_4)
}
