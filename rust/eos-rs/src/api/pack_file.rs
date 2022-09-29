//! Handling of packed files.

use crate::ctypes::c_void;
use crate::ffi;
use alloc::ffi::CString;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

/// Open a Pack file, to be read later.
///
/// Panics if `file_name` can not be converted to a CString (see [`CString::new`]).
///
/// # Safety
/// The file must exist and be a valid pack file.
pub unsafe fn open_pack_file(file_name: &str) -> ffi::pack_file_opened {
    let mut res: MaybeUninit<ffi::pack_file_opened> = MaybeUninit::zeroed();
    let c_file_name = CString::new(file_name).expect("Expected to convert string.");
    ffi::OpenPackFile(res.as_mut_ptr(), c_file_name.as_ptr());
    res.assume_init()
}

/// Open the 6 files at PACK_FILE_PATHS_TABLE into PACK_FILE_OPENED.
/// Called during game initialisation.
pub fn open_all_pack_files() {
    unsafe { ffi::OpenAllPackFiles() }
}

/// Get the length of a file entry from a Pack archive.
///
/// # Safety
/// Th file number must be valid.
pub unsafe fn get_file_length_in_pack(pack: &mut ffi::pack_file_opened, file_number: u32) -> u32 {
    ffi::GetFileLengthInPack(pack, file_number)
}

/// Call [`get_file_length_in_pack`] after looking up the global pack archive by its number.
///
/// # Safety
/// The pack number and file number must be valid.
pub unsafe fn get_file_length_in_pack_by_pack_number(
    pack_number: ffi::pack_file_id::Type,
    file_number: u32,
) -> u32 {
    ffi::GetFileLengthInPackWithPackNb(pack_number, file_number)
}

/// Load the indexed file from the Pack archive.
///
/// # Safety
/// The file number must be valid.
pub unsafe fn load_file_in_pack(pack: &mut ffi::pack_file_opened, file_number: u32) -> Vec<u8> {
    let size = get_file_length_in_pack(pack, file_number) as usize;
    let mut dst = vec![0; size];
    let actual_size =
        ffi::LoadFileInPack(pack, dst.as_mut_ptr() as *mut c_void, file_number) as usize;
    if actual_size > size {
        panic!("Pack file mismatch while reading.")
    } else {
        dst.resize(actual_size, 0);
    }
    dst
}

/// Call [`load_file_in_pack`] after looking up the global pack archive by its number.
///
/// This is functionally identical to [`alloc_and_load_file_in_pack`], but Rust handles
/// allocation.
///
/// # Safety
/// The pack number and file number must be valid.
pub unsafe fn load_file_in_pack_by_pack_number(
    pack_number: ffi::pack_file_id::Type,
    file_number: u32,
) -> Vec<u8> {
    let size = get_file_length_in_pack_by_pack_number(pack_number, file_number) as usize;
    let mut dst = vec![0; size];
    let actual_size =
        ffi::LoadFileInPackWithPackId(pack_number, dst.as_mut_ptr() as *mut c_void, file_number)
            as usize;
    if actual_size > size {
        panic!("Pack file mismatch while reading.")
    } else {
        dst.resize(actual_size, 0);
    }
    dst
}

/// Allocate a file and load a file from the pack archive inside.
/// This is functionally identical to [`load_file_in_pack_by_pack_number`],
/// but the game handles allocation.
///
/// # Safety
/// The pack number and file number must be valid.
pub unsafe fn alloc_and_load_file_in_pack(
    pack_number: ffi::pack_file_id::Type,
    file_number: u32,
    malloc_flags: u32,
) -> Vec<u8> {
    let mut res: MaybeUninit<ffi::pack_alloc_and_load_result> = MaybeUninit::zeroed();
    ffi::AllocAndLoadFileInPack(pack_number, file_number, res.as_mut_ptr(), malloc_flags);
    let res = res.assume_init();
    Vec::from_raw_parts(
        res.data as *mut u8,
        res.length as usize,
        res.length as usize,
    )
}
