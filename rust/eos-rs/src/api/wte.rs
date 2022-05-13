//! Handling of WTE files.

use core::ffi::CStr;
use core::mem::MaybeUninit;
use crate::ffi;

/// A Rust-owned WTE file.
pub struct OwnedWte(ffi::wte_handle);

impl OwnedWte {
    /// Take ownership of an WTE handle.
    ///
    /// # Safety
    /// The handle must be valid.
    pub unsafe fn from_handle(handle: ffi::wte_handle) -> Self {
        Self(handle)
    }

    /// Loads a SIR0-wrapped WTE file from ROM.
    ///
    /// # Safety
    /// The path must point to a valid WTE file.
    pub unsafe fn load_from_rom<C: AsRef<CStr>>(path: C, malloc_flags: u32) -> Self {
        let mut handle = MaybeUninit::uninit();
        ffi:: LoadWteFromRom(handle.as_mut_ptr(), path.as_ref().as_ptr(), malloc_flags);
        Self(handle.assume_init())
    }

    /// Loads a SIR0-wrapped WTE file from a file directory.
    ///
    /// # Safety
    /// The path must point to a valid WTE file.
    pub unsafe fn load_from_dir(pack_file_id: u16, file_index: u16, malloc_flags: u32) -> Self {
        let mut handle = MaybeUninit::uninit();
        ffi:: LoadWteFromFileDirectory(handle.as_mut_ptr(), pack_file_id, file_index, malloc_flags);
        Self(handle.assume_init())
    }
}

/// Unloads the WTE file.
impl Drop for OwnedWte {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadWte(&mut self.0);
        }
    }
}
