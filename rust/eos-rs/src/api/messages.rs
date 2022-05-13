//! Functions for formatting and printing game messages and other "message box" and menu related
//! operations.

use alloc::borrow::ToOwned;
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ffi::CStr;
use core::mem::ManuallyDrop;
use az::SaturatingAs;
use crate::ctypes::c_char;
use crate::ffi;
use crate::string_util::str_to_cstring;

static mut NULL: c_char = 0;

enum PreprocessorArgs<'a> {
    Owned(ffi::preprocessor_args),
    Borrowed(&'a mut ffi::preprocessor_args)
}

impl<'a> PreprocessorArgs<'a> {
    pub fn as_mut<'b>(&'b mut self) -> &'b mut ffi::preprocessor_args where 'a: 'b {
        match self {
            PreprocessorArgs::Owned(args) => args,
            PreprocessorArgs::Borrowed(args) => args
        }
    }
}

/// Builds a string using an "enhanced sprintf", which recognizes certain tags and replaces them
/// with appropriate game values.
///
/// The output buffer is calculated to have a capacity of the input format string times 3
/// (or i32::MAX). Should that not be enough, you can use [`Self::output_buffer_size`] to override
/// this.
///
/// See <https://textbox.skytemple.org> for a reference about message tags and a preview tool.
pub struct GameStringBuilder<'a> {
    output_size: Option<i32>,  // Note: Auto
    flags: ffi::preprocessor_flags,
    args: PreprocessorArgs<'a>,
}

impl<'a> GameStringBuilder<'a> {
    pub fn new() -> Self {
        // SAFETY: We assume the game won't try to change the strings values.
        unsafe {
            Self {
                output_size: None,
                flags: ffi::preprocessor_flags { _bitfield_align_1: [], _bitfield_1: Default::default() },
                args: PreprocessorArgs::Owned(ffi::preprocessor_args {
                    flag_vals: [0, 0, 0, 0],
                    id_vals: [0, 0, 0, 0, 0],
                    number_vals: [0, 0, 0, 0, 0],
                    strings: [&mut NULL, &mut NULL, &mut NULL, &mut NULL, &mut NULL],
                    speaker_id: 0
                })
            }
        }
    }

    /// Sets the unknown0 value of the preprocessor flags.
    pub fn set_flag_unknown0(&mut self, value: u16) -> &mut Self {
        self.flags.set_unknown0(value);
        self
    }

    /// Tells the game to show the speaker in the formatted message.
    ///
    /// Sets the speaker value of the preprocessor flags to true.
    ///
    /// If [`Self::set_speaker_name`] was not called, this will probably(...?) show the speech
    /// bubble icon as the speaker.
    pub fn show_speaker(&mut self) -> &mut Self {
        self.flags.set_show_speaker(true as ffi::bool_);
        self
    }

    /// Sets the unknown18 value of the preprocessor flags.
    pub fn set_flag_unknown18(&mut self, value: u32) -> &mut Self {
        self.flags.set_unknown18(value);
        self
    }

    /// Sets the speaker of the message to the given entity (probably actor or monster ID...?).
    ///
    /// To actually also show the speaker, use [`Self::show_speaker`].
    pub fn set_speaker(&'a mut self, speaker: u32) -> &'a mut Self {
        self.args.as_mut().speaker_id = speaker;
        self
    }

    /// Sets flag values. Currently unknown what they do.
    ///
    /// Max flag ID is 3.
    pub fn set_flag_value(&'a mut self, flag_id: usize, value: u32) -> &'a mut Self {
        self.args.as_mut().flag_vals[flag_id] = value;
        self
    }

    /// Sets an ID for a message placeholder. This is *probably* used by the `\[name:id\]`,
    /// `\[item:id\]` etc. placeholders,
    ///
    /// Max ID ID is 4.
    pub fn set_id_value(&'a mut self, id_id: usize, value: u32) -> &'a mut Self {
        self.args.as_mut().id_vals[id_id] = value;
        self
    }

    /// Sets a number for a message placeholder. This is *probably* used by the `\[digits_c:0]`,
    /// etc. placeholders.
    ///
    /// Max number ID is 4.
    pub fn set_number_value(&'a mut self, number_id: usize, value: i32) -> &'a mut Self {
        self.args.as_mut().number_vals[number_id] = value;
        self
    }

    /// Sets a string for message placeholders.
    ///
    /// Replace all occurrences of `\[string:<string_id>\]` with the value of the string passed in.
    ///
    /// Max string ID is 4.
    pub fn set_string(&'a mut self, string_id: usize, string: &'a CString) -> &'a mut Self {
        self.args.as_mut().strings[string_id] = string.as_ptr() as *mut _;
        self
    }

    /// Overwrites the size of the output buffer.
    pub fn output_buffer_size(&mut self, size: i32) -> &mut Self {
        self.output_size = Some(size);
        self
    }

    /// Returns a reference to the internal args. This will panic if [`Self::borrow_args`] was
    /// called before.
    pub fn args(&self) -> &ffi::preprocessor_args {
        match &self.args {
            PreprocessorArgs::Owned(args) => args,
            PreprocessorArgs::Borrowed(_) => panic!("Invalid `args` call.")
        }
    }

    /// Returns a reference to the internal flags.
    pub fn flags(&self) -> &ffi::preprocessor_flags {
        &self.flags
    }

    /// Overwrite the internal flags with the specified ones.
    ///
    /// This is useful if you want to re-use flags.
    /// You probably want to use [`Self::set_flag_unknown0`], [`Self::show_speaker`],
    /// [`Self::set_flag_unknown18`] instead in most cases.
    pub fn set_flags(&mut self, flags: &ffi::preprocessor_flags) -> &mut Self {
        self.flags = ffi::preprocessor_flags {
            _bitfield_align_1: flags._bitfield_align_1,
            _bitfield_1: flags._bitfield_1
        };
        self
    }

    /// Sets the internal arguments to use the arguments passed in.
    ///
    /// This is useful if you want to re-use args.
    ///
    /// Subsequent calls to [`Self::set_speaker`], [`Self::set_flag_value`], [`Self::set_id_value`],
    /// [`Self::set_number_value`],[`Self::set_string`] will manipulate the args passed in.
    ///
    /// You probably don't need to use this method in most cases, just use the above mentioned
    /// methods instead.
    pub fn borrow_args(&'a mut self, args: &'a mut ffi::preprocessor_args) -> &'a mut Self {
        self.args = PreprocessorArgs::Borrowed(args);
        self
    }

    /// Converts the format string to the formatted string.
    ///
    /// Builds String from a str. The input is the format string to use.
    #[allow(clippy::needless_return)]
    pub fn build<S: AsRef<str>>(self, format: S) -> String {
        #[cfg(debug_assertions)]
        return self.build_from_cstr_as_cstring(str_to_cstring(format.as_ref())).into_string().expect("Failed to convert game string to String (invalid UTF-8)");
        // Save some precious size in release mode
        #[cfg(not(debug_assertions))]
        self.build_from_cstr_as_cstring(str_to_cstring(format.as_ref())).into_string().unwrap()
    }

    /// Converts the format string to the formatted string.
    ///
    /// Builds String from a CStr. The input is the format string to use.
    #[allow(clippy::needless_return)]
    pub fn build_from_cstr<S: AsRef<CStr>>(self, format: S) -> String {
        #[cfg(debug_assertions)]
        return self.build_from_cstr_as_cstring(format.as_ref()).into_string().expect("Failed to convert game string to String (invalid UTF-8)");
        // Save some precious size in release mode
        #[cfg(not(debug_assertions))]
        self.build_from_cstr_as_cstring(format.as_ref()).into_string().unwrap()
    }

    /// Converts the format string to the formatted string.
    ///
    /// Builds CString from a str. The input is the format string to use.
    pub fn build_as_cstring<S: AsRef<str>>(self, format: S) -> CString {
        self.build_from_cstr_as_cstring(str_to_cstring(format.as_ref()))
    }

    /// Converts the format string to the formatted string.
    ///
    /// Builds CString from a CStr. The input is the format string to use.
    pub fn build_from_cstr_as_cstring<S: AsRef<CStr>>(self, format: S) -> CString {
        let Self {
            output_size, flags, mut args
        } = self;

        let output_size = match output_size {
            None => (format.as_ref().to_bytes().len() * 3).saturating_as(),
            Some(size) => size,
        };
        // We manually transfer the internals of the output vector later, since we convert it to an
        // u8 vector.
        let mut output = ManuallyDrop::new(vec![0 as c_char; output_size as usize]);
        unsafe {
            let size = ffi::PreprocessString(
                output.as_mut_ptr(),
                output_size,
                format.as_ref().as_ptr(),
                flags,
                args.as_mut()
            );
            output.truncate(size as usize + 1); // + 1 for the null byte.

            // Convert output from Vec<i8> to Vec<u8> at no cost.
            let output = Vec::from_raw_parts(
                output.as_mut_ptr() as *mut u8, output.len(), output.capacity()
            );

            CString::from_vec_with_nul_unchecked(output)
        }
    }
}

impl<'a> Default for GameStringBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Gets the string corresponding to a given message ID.
pub fn get_string_from_message_id(message_id: i32) -> CString {
    // SAFETY: We assume the game returns a valid C-String and does bounds checking.
    unsafe { CStr::from_ptr(ffi::StringFromMessageId(message_id)) }.to_owned()
}

/// Sets the palette of the frames of windows in the specified screen.
pub fn set_screen_windows_color(palette_idx: u8, is_upper_window: bool) {
    unsafe { ffi::SetScreenWindowsColor(palette_idx as i32, is_upper_window as ffi::bool_) }
}

/// Sets the palette of the frames of windows in both screens.
pub fn set_both_screens_windows_color(palette_idx: u8) {
    unsafe { ffi::SetBothScreensWindowsColor(palette_idx as i32) }
}
