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

    use super::{Read, Seek, SeekFrom};
    use crate::ctypes::c_void;
    use crate::ffi;
    use alloc::vec::Vec;
    use core::ffi::CStr;
    use core::mem::MaybeUninit;

    /// This counter mutex is safe to access by methods of this module, since the NDS is
    /// single-threaded. There are probably some reasons regarding missing atomic support and the
    /// ARM instruction set due to which this isn't true if for example interrupts happen, but
    /// this is marked in the safety note of [`read`] and [`File::open`].
    static mut COUNT_IN_FILE_TRANSFER_MODE: usize = 0;

    /// Loads a file from ROM by filepath into a heap-allocated buffer.
    ///
    /// # Safety
    /// The file path must be a valid path to an existing file in the ROM file system.
    ///
    /// Additionally, see safety note for [`Vec::from_raw_parts`]. It's probably safer to
    /// use the [`File`] struct instead.
    ///
    /// You must make sure this is not called during interrupts.
    pub unsafe fn read<C: AsRef<CStr>>(path: C, flags: u32) -> Vec<u8> {
        let mut iov_raw = MaybeUninit::uninit();
        if COUNT_IN_FILE_TRANSFER_MODE == 0 {
            ffi::DataTransferInit();
        }
        ffi::LoadFileFromRom(iov_raw.as_mut_ptr(), path.as_ref().as_ptr(), flags);
        if COUNT_IN_FILE_TRANSFER_MODE == 0 {
            ffi::DataTransferStop();
        }
        let iov = iov_raw.assume_init();
        Vec::from_raw_parts(
            iov.iov_base as *mut u8,
            iov.iov_len as usize,
            iov.iov_len as usize,
        )
    }

    /// A file in the NDS file-system.
    pub struct File(ffi::file_stream);

    impl File {
        /// Opens a file from the ROM file system at the given path, sort of like C's fopen(3)
        /// library function.
        ///
        /// # Safety
        /// The file path must be a valid path to an existing file in the ROM file system.
        ///
        /// You must make sure the file object or readers made for it are never accessed from code
        /// during interrupts.
        pub unsafe fn open<C: AsRef<CStr>>(path: C) -> Self {
            let mut file_stream = MaybeUninit::uninit();
            if COUNT_IN_FILE_TRANSFER_MODE == 0 {
                ffi::DataTransferInit();
            }
            ffi::FileInit(file_stream.as_mut_ptr());
            ffi::FileOpen(file_stream.as_mut_ptr(), path.as_ref().as_ptr());
            if COUNT_IN_FILE_TRANSFER_MODE == 0 {
                ffi::DataTransferStop();
            }
            Self(file_stream.assume_init())
        }

        /// Returns a reader for the file (implements `Read` and `Seek` for it).
        ///
        /// During the lifetime of the reader the NDS will be put in file-transfer mode, if it isn't
        /// already.
        pub fn reader(&mut self) -> FileReader {
            FileReader::new(self)
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

    /// A reader for a [`File`].
    ///
    /// During the lifetime of the reader the NDS will be put in file-transfer mode, if it isn't
    /// already.
    pub struct FileReader<'a>(&'a mut File);

    impl<'a> FileReader<'a> {
        pub fn new(file: &'a mut File) -> Self {
            unsafe {
                if COUNT_IN_FILE_TRANSFER_MODE == 0 {
                    ffi::DataTransferInit();
                }
                COUNT_IN_FILE_TRANSFER_MODE += 1;
                Self(file)
            }
        }
    }

    impl<'a> Drop for FileReader<'a> {
        fn drop(&mut self) {
            unsafe {
                // This can only be false if this has somehow gotten out of sync, but let's just be
                // safe here...
                if COUNT_IN_FILE_TRANSFER_MODE > 0 {
                    COUNT_IN_FILE_TRANSFER_MODE -= 1;
                }
                if COUNT_IN_FILE_TRANSFER_MODE == 0 {
                    ffi::DataTransferStop();
                }
            }
        }
    }

    impl<'a> Read for FileReader<'a> {
        fn read(&mut self, dst: &mut [u8]) -> super::Result<usize> {
            unsafe {
                let len = ffi::FileRead(
                    &mut self.0.0,
                    dst.as_mut_ptr() as *mut c_void,
                    dst.len() as u32,
                ) as usize;
                Ok(len)
            }
        }
    }

    impl<'a> Seek for FileReader<'a> {
        /// Seeking from End might not be properly implemented in the game.
        ///
        /// Additionally the position must fit into an i32.
        fn seek(&mut self, pos: SeekFrom) -> super::Result<u64> {
            unsafe {
                match pos {
                    SeekFrom::Start(p) => ffi::FileSeek(&mut self.0.0, p as i32, 0),
                    SeekFrom::Current(p) => ffi::FileSeek(&mut self.0.0, p as i32, 1),
                    SeekFrom::End(p) => ffi::FileSeek(&mut self.0.0, p as i32, 2),
                }

                Ok((self.0.0.current_address as u64) - (self.0.0.start_address as u64))
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
