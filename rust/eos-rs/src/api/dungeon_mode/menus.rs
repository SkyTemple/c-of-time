//! Dungeon menu controller.

use crate::api::dungeon_mode::DungeonEntity;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Adds an option to the list of actions that can be taken on a monster, item or move to the
/// currently active sub-menu on dungeon mode (team, moves, items, etc.).
pub fn add_dungeon_sub_menu_option(
    _ov29: &OverlayLoadLease<29>,
    action_id: i32,
    enable_option: bool,
) {
    unsafe { ffi::AddDungeonSubMenuOption(action_id, enable_option as ffi::bool_) }
}

/// Called on each frame while the in-dungeon "others" menu is open.
///
/// It contains a switch to determine whether an option has been chosen or not and a second switch
/// that determines what to do depending on which option was chosen.
///
/// Returns an int (Actually, this is probably some sort of enum shared by all the
/// MenuLoop functions).
///        
pub fn others_menu_loop(_ov29: &OverlayLoadLease<29>) -> i32 {
    unsafe { ffi::OthersMenuLoop() }
}

/// Called when the in-dungeon "others" menu is open. Does not return until the menu is closed.
pub fn others_menu(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::OthersMenu() };
}

/// Opens a menu where the user can choose "Yes" or "No" and waits for input before returning.
///
/// Returns true if the user chooses "Yes", false if the user chooses "No".
///
/// # Arguments
///
/// * `_ov29` - A lease on the loaded overlay 29.
/// * `param_1` - ?
/// * `param_1` - ID of the string to display in the textbox
/// * `param_1` - Option that the cursor will be on by default. `true` for "Yes", `false` for "No"
/// * `param_4` - ?
///
/// # Safety
/// The caller must make sure the undefined params are valid for this function.
pub unsafe fn yes_no_menu(
    _ov29: &OverlayLoadLease<29>,
    param_1: ffi::undefined,
    message_id: i32,
    default_option: bool,
    param_4: ffi::undefined,
) -> bool {
    ffi::YesNoMenu(param_1, message_id, !default_option as i32, param_4) > 0
}

/// Called when the in-dungeon "team" menu is open. Does not return until the menu is closed.
///
/// Note that selecting certain options in this menu (such as viewing the details or the moves
/// of a monster) counts as switching to a different menu, which causes the function to return.
pub fn team_menu(_ov31: &OverlayLoadLease<31>, team_leader: &mut DungeonEntity) {
    unsafe { ffi::TeamMenu(team_leader) };
}

/// Called when the in-dungeon "rest" menu is open. Does not return until the menu is closed.
pub fn rest_menu(_ov31: &OverlayLoadLease<31>) {
    unsafe { ffi::RestMenu() }
}

/// Called on each frame while the in-dungeon "recruitment search" menu is open.
///
/// Returns an int (Actually, this is probably some sort of enum shared by all the
/// MenuLoop functions).
pub fn recruitment_search_menu_loop(_ov31: &OverlayLoadLease<31>) -> i32 {
    unsafe { ffi::RecruitmentSearchMenuLoop() }
}

/// Called on each frame while the in-dungeon "help" menu is open.
///
/// The menu is still considered open while one of the help pages is being viewed, so this
/// function keeps being called even after choosing an option.
///
/// Returns an int (Actually, this is probably some sort of enum shared by all the
/// MenuLoop functions).
pub fn help_menu_loop(_ov31: &OverlayLoadLease<31>) -> i32 {
    unsafe { ffi::HelpMenuLoop() }
}
