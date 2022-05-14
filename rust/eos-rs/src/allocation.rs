//! Module related to allocating data on the heap.

use crate::ctypes;
use crate::ffi;
#[cfg(not(test))]
use crate::panic;
use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::NonNull;

#[cfg(feature = "global_allocator")]
#[cfg(not(test))]
#[global_allocator]
/// Global allocator ([`EoSDefaultAllocator`]).
/// You can disable registering this, by disabling the `global_allocator` feature of this crate,
/// you can then register you own global allocator.
pub static ALLOCATOR: EoSDefaultAllocator = EoSDefaultAllocator;

/// The game's allocation functions wrapped in a struct. This is set up as the global allocator,
/// unless the `global_allocator` feature is turned off.
///
/// It uses the default memory arena ([`ffi::MemAlloc`] and [`ffi::MemFree`] functions are used).
pub struct EoSDefaultAllocator;

unsafe impl GlobalAlloc for EoSDefaultAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::MemAlloc(layout.size() as u32, 0) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::MemFree(ptr as *mut ctypes::c_void);
    }
}

/// On allocation errors, you should call [`alloc_error_handler`].
unsafe impl Allocator for EoSDefaultAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let raw_ptr = ffi::MemAlloc(layout.size() as u32, 0) as *mut u8;
            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
            Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        ffi::MemFree(ptr.as_ptr() as *mut ctypes::c_void);
    }
}

/// An allocator that can be used with any specified memory area. You can set up a new arena
/// with [`create_mem_arena`].
pub struct EoSCustomAllocator(*mut ffi::mem_arena);

impl EoSCustomAllocator {
    /// Creates a new custom allocator with the specified memory area.
    ///
    /// # Safety
    /// The memory arena must be valid and stay valid during the lifetime of this struct.
    pub const unsafe fn new(arena: *mut ffi::mem_arena) -> Self {
        EoSCustomAllocator(arena)
    }
}

/// To use a custom allocator as the global allocator, disable the `global_allocator` feature
/// and register your custom allocator as the global allocator.
unsafe impl GlobalAlloc for EoSCustomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::MemLocateSet(self.0, layout.size() as u32, 0) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::MemLocateUnset(self.0, ptr as *mut ctypes::c_void);
    }
}

/// On allocation errors, you should call [`alloc_error_handler`].
unsafe impl Allocator for EoSCustomAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let raw_ptr = ffi::MemLocateSet(self.0, layout.size() as u32, 0) as *mut u8;
            let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
            Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        ffi::MemLocateUnset(self.0, ptr.as_ptr() as *mut ctypes::c_void);
    }
}

/// Creates a new memory arena within a given block of memory. Returns the memory arena.
///
/// # Arguments
/// * `mem` - memory region in which to create the arena, as {pointer, length}
/// * `max_blocks` - maximum number of blocks that the arena can hold
///
/// # Safety
/// `mem` must be a valid pointer + length to an unused memory location. `max_blocks` must fit
/// into the length of `mem`.
pub unsafe fn create_mem_arena(mem: *mut ffi::iovec, max_blocks: u32) -> *mut ffi::mem_arena {
    ffi::CreateMemArena(mem, max_blocks)
}

/// Allocation error handler. Will freeze the game and output an error message.
#[cfg(not(test))]
#[alloc_error_handler]
pub fn alloc_error_handler(_: Layout) -> ! {
    let err = b"[rs] OUT OF MEMORY!\0";
    unsafe {
        ffi::DebugPrint(2, err.as_ptr() as *const ctypes::c_char);
    }
    unsafe { panic::WaitForever() }
}
