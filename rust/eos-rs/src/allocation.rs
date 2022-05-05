use core::alloc::{GlobalAlloc, Layout};
use crate::ctypes::{c_char, c_void};
use crate::ffi;

#[global_allocator]
pub static ALLOCATOR: EoSAllocator = EoSAllocator;

pub struct EoSAllocator;

// TODO: We could be way fancier, supporting different arenas, playing with the flags, etc.
//       Try creating an arena in overlay36 with CreateMemArena + MemLocateSet
unsafe impl GlobalAlloc for EoSAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::MemAlloc(layout.size() as u32, 0) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::MemFree(ptr as *mut c_void);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
    let err = b"[rs] OUT OF MEMORY!\0";
    unsafe { ffi::DebugPrint(2, err.as_ptr() as *const c_char); }
    core::intrinsics::abort();
}
