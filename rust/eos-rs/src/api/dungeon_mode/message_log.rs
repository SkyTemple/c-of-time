use crate::api::dungeon_mode::entity::DungeonEntity;
use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::ctypes::*;
use crate::ffi;
use crate::string_util::str_to_cstring;
use core::ffi::CStr;
use core::fmt::Debug;

/// Builder for creating dungeon message log messages.
///
/// By default message will be shown 'quiet', meaning there will be no popup
/// shown when the message is logged. You can force a popup to be shown with [`Self::popup`],
/// but please also note that with some configurations, a popup will always be displayed, even
/// if [`Self::popup`] is not called. See the implementation for more details.
///
/// Also see [`crate::api::messages::GameStringBuilder`]; this is using pretty much the same
/// principles for building the actual message strings.
pub struct LogMessageBuilder<'a> {
    _lease: OverlayLoadLease<29>,
    popup: bool,
    check_user: bool,
    target_check_fainted: Option<&'a DungeonEntity>,
}

impl<'a> CreatableWithLease<29> for LogMessageBuilder<'a> {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self {
            _lease: lease,
            popup: false,
            check_user: false,
            target_check_fainted: None,
        }
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self._lease
    }
}

impl<'a> LogMessageBuilder<'a> {
    /// Show a message popup when the message is displayed.
    pub fn popup(&mut self) -> &mut Self {
        self.popup = true;
        self
    }

    /// Do not show the message if the user is fainted.
    ///
    /// # Note
    /// [`Self::target_check_fainted`] will take precedence over this, both can not be active
    /// at the same time.
    pub fn check_user_fainted(&'a mut self) -> &'a mut Self {
        self.check_user = true;
        self
    }

    // Do not show the message if the target is fainted and an unknown check
    // regarding the user passes.
    pub fn target_check_fainted(&'a mut self, target: &'a DungeonEntity) -> &'a mut Self {
        self.target_check_fainted = Some(target);
        self
    }

    /// Replaces instances of a given placeholder tag by the string representation of the given entity.
    /// Concretely this means that any occurrences of `\[string:<string_id>\]` will be replaced by the
    /// name of the given entity.
    /// Example: If use pass `string_id` with 1, it will replace all occurrences of `\[string:1\]`.
    ///
    /// # Note
    /// As a performance optimization this will immediately reserve that string with the game when
    /// called. This can have weird effects if you expect to show the message built by this builder
    /// at a later time.
    pub fn string(&mut self, string_id: u16, entity: &DungeonEntity) -> &mut Self {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::SubstitutePlaceholderStringTags(string_id as c_int, force_mut_ptr!(entity), 0)
        }
        self
    }

    /// Writes a log entry using the message with the given message ID.
    pub fn log_msg(&mut self, user: &DungeonEntity, message_id: i32) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            match (self.popup, self.check_user, self.target_check_fainted) {
                (false, false, None) => ffi::LogMessageByIdQuiet(force_mut_ptr!(user), message_id),
                (_, true, None) => {
                    ffi::LogMessageByIdWithPopupCheckUser(force_mut_ptr!(user), message_id)
                }
                (false, _, Some(target)) => ffi::LogMessageByIdQuietCheckUserTarget(
                    force_mut_ptr!(user),
                    force_mut_ptr!(target),
                    message_id,
                ),
                (true, false, None) => {
                    ffi::LogMessageByIdWithPopup(force_mut_ptr!(user), message_id)
                }
                (true, _, Some(target)) => ffi::LogMessageByIdWithPopupCheckUserTarget(
                    force_mut_ptr!(user),
                    force_mut_ptr!(target),
                    message_id,
                ),
            }
        }
    }

    pub fn log_str<S: AsRef<str> + Debug>(&mut self, user: &DungeonEntity, message: S) {
        self.log_cstr(user, str_to_cstring(message))
    }

    pub fn log_cstr<S: AsRef<CStr>>(&mut self, user: &DungeonEntity, message: S) {
        let message = message.as_ref().as_ptr() as *const c_char;
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            match (self.popup, self.check_user, self.target_check_fainted) {
                (false, false, None) => ffi::LogMessageQuiet(force_mut_ptr!(user), message),
                (_, true, None) => ffi::LogMessageWithPopupCheckUser(force_mut_ptr!(user), message),
                (true, false, None) => ffi::LogMessageWithPopup(force_mut_ptr!(user), message),
                (_, _, Some(target)) => ffi::LogMessageWithPopupCheckUserTarget(
                    force_mut_ptr!(user),
                    force_mut_ptr!(target),
                    message,
                ),
            }
        }
    }
}
