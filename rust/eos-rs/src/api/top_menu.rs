//! Top menu / main menu controller.

use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Prepares the top menu and sub menu, adding the different options that compose them.
/// Contains multiple calls to [`add_main_menu_option`] and [`add_sub_menu_option`]. Some of them are
/// conditionally executed depending on which options should be unlocked.
pub fn create_main_menus(_ov01: &OverlayLoadLease<1>) {
    unsafe { ffi::CreateMainMenus() }
}

/// Adds an option to the top menu.
/// This function is called for each one of the options in the top menu.
///
/// It loops the `MAIN_MENU` data field, if the specified action ID does not exist there,
/// the option won't be added.
pub fn add_main_menu_option(_ov01: &OverlayLoadLease<1>, action_id: i32, enable_option: bool) {
    unsafe { ffi::AddMainMenuOption(action_id, enable_option as ffi::bool_) }
}

/// Adds an option to the "Other" submenu on the top menu.
/// This function is called for each one of the options in the submenu.
///
/// It loops the `SUBMENU` data field, if the specified action ID does not exist there,
/// the option won't be added.
pub fn add_sub_menu_option(_ov01: &OverlayLoadLease<1>, action_id: i32, enable_option: bool) {
    unsafe { ffi::AddSubMenuOption(action_id, enable_option as ffi::bool_) }
}
