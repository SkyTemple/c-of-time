//! Traits, structs and functions related to file handling and I/O.
//!
//! You can opt-out of pulling in this module by disabling the `io` feature.
//!
//! On the top-level this module re-exports [`acid_io`].
//!
//! EoS related file operations are in the sub-module [`mod@file`].

// We also provide acid_io.
pub use acid_io::*;

pub mod file {
    //! File related operations.

    use alloc::vec::Vec;
    use core::ffi::CStr;
    use core::mem::MaybeUninit;
    use crate::ctypes::c_void;
    use crate::ffi;
    use super::{Read, Seek, SeekFrom};

    /// Loads a file from ROM by filepath into a heap-allocated buffer.
    ///
    /// # Safety
    /// The file path must be a valid path to an existing file in the ROM file system.
    ///
    /// Additionally, see safety note for [`Vec::from_raw_parts`]. It's probably safer to
    /// use the [`File`] struct instead.
    pub unsafe fn read<C: AsRef<CStr>>(path: C, flags: u32) -> Vec<u8> {
        let mut iov_raw = MaybeUninit::uninit();
        ffi::DataTransferInit();
        ffi::LoadFileFromRom(iov_raw.as_mut_ptr(), path.as_ref().as_ptr(), flags);
        ffi::DataTransferStop();
        let iov = iov_raw.assume_init();
        Vec::from_raw_parts(
            iov.iov_base as *mut u8, iov.iov_len as usize, iov.iov_len as usize
        )
    }

    pub struct File(ffi::file_stream);

    impl File {
        /// Opens a file from the ROM file system at the given path, sort of like C's fopen(3)
        /// library function.
        ///
        /// # Safety
        /// The file path must be a valid path to an existing file in the ROM file system.
        pub unsafe fn open<C: AsRef<CStr>>(path: C) -> Self {
            let mut file_stream = MaybeUninit::uninit();
            ffi::DataTransferInit();
            ffi::FileInit(file_stream.as_mut_ptr());
            ffi::FileOpen(file_stream.as_mut_ptr(), path.as_ref().as_ptr());
            ffi::DataTransferStop();
            Self(file_stream.assume_init())
        }

        /// Gets the size of the open file.
        pub fn len(&self) -> u32 {
            unsafe { ffi::FileGetSize(force_mut_ptr!(&self.0)) }
        }

        /// Checks if the file has size 0.
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl Read for File {
        fn read(&mut self, dst: &mut [u8]) -> super::Result<usize> {
            unsafe {
                ffi::DataTransferInit();
                let len = ffi::FileRead(&mut self.0, dst.as_mut_ptr() as *mut c_void, dst.len() as u32) as usize;
                ffi::DataTransferStop();
                Ok(len)
            }
        }
    }

    impl Seek for File {
        /// Seeking from End might not be properly implemented in the game.
        ///
        /// Additionally the position must fit into an i32.
        fn seek(&mut self, pos: SeekFrom) -> super::Result<u64> {
            unsafe {
                ffi::DataTransferInit();
                match pos {
                    SeekFrom::Start(p) => ffi::FileSeek(&mut self.0, p as i32, 0),
                    SeekFrom::Current(p) => ffi::FileSeek(&mut self.0, p as i32, 1),
                    SeekFrom::End(p) => ffi::FileSeek(&mut self.0, p as i32, 2),
                }
                ffi::DataTransferStop();

                Ok((self.0.current_address as u64) - (self.0.start_address as u64))
            }
        }
    }

    impl Drop for File {
        fn drop(&mut self) {
            unsafe {
                ffi::DataTransferInit();
                ffi::FileClose(&mut self.0);
                ffi::DataTransferStop();
            }
        }
    }
}
