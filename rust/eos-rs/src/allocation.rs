//! Module related to allocating data on the heap.

use core::alloc::{GlobalAlloc, Layout};
use crate::ctypes;
use crate::ffi;
#[cfg(not(test))]
use crate::panic;

#[cfg(not(test))]
#[global_allocator]
/// Global allocator.
pub static ALLOCATOR: EoSAllocator = EoSAllocator;

/// The game's allocation functions wrapped in a struct. This is set up as the global allocator.
pub struct EoSAllocator;

// TODO: We could be way fancier, supporting different arenas, playing with the flags, etc.
//       Try creating an arena in overlay36 with CreateMemArena + MemLocateSet
unsafe impl GlobalAlloc for EoSAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::MemAlloc(layout.size() as u32, 0) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::MemFree(ptr as *mut ctypes::c_void);
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
    let err = b"[rs] OUT OF MEMORY!\0";
    unsafe { ffi::DebugPrint(2, err.as_ptr() as *const ctypes::c_char); }
    unsafe { panic::WaitForever() }
}

//   functions:
//     - name: InitMemAllocTable
//       address:
//         NA: 0x2000DE0
//         EU: 0x2000DE0
//       description: |-
//         Initializes MEMORY_ALLOCATION_TABLE.
//
//         Sets up the default memory arena, sets the default memory allocator parameters (calls SetMemAllocatorParams(0, 0)), and does some other stuff.
//
//         No params.
//     - name: SetMemAllocatorParams
//       address:
//         NA: 0x2000E70
//         EU: 0x2000E70
//       description: |-
//         Sets global parameters for the memory allocator.
//
//         This includes MEMORY_ALLOCATION_ARENA_GETTERS and some other stuff.
//
//         Dungeon mode uses the default arena getters. Ground mode uses its own arena getters, which are defined in overlay 11 and set (by calling this function) at the start of GroundMainLoop.
//
//         r0: GetAllocArena function pointer (GetAllocArenaDefault is used if null)
//         r1: GetFreeArena function pointer (GetFreeArenaDefault is used if null)
//     - name: GetAllocArenaDefault
//       address:
//         NA: 0x2000EC0
//         EU: 0x2000EC0
//       description: |-
//         The default function for retrieving the arena for memory allocations. This function always just returns the initial arena pointer.
//
//         r0: initial memory arena pointer, or null
//         r1: flags (see MemAlloc)
//         return: memory arena pointer, or null
//     - name: GetFreeArenaDefault
//       address:
//         NA: 0x2000EC4
//         EU: 0x2000EC4
//       description: |-
//         The default function for retrieving the arena for memory freeing. This function always just returns the initial arena pointer.
//
//         r0: initial memory arena pointer, or null
//         r1: pointer to free
//         return: memory arena pointer, or null
//     - name: InitMemArena
//       address:
//         NA: 0x2000EC8
//         EU: 0x2000EC8
//       description: |-
//         Initializes a new memory arena with the given specifications, and records it in the global MEMORY_ALLOCATION_TABLE.
//
//         r0: arena struct to be initialized
//         r1: memory region to be owned by the arena, as {pointer, length}
//         r2: pointer to block metadata array for the arena to use
//         r3: maximum number of blocks that the arena can hold
//     - name: MemAllocFlagsToBlockType
//       address:
//         NA: 0x2000F44
//         EU: 0x2000F44
//       description: |-
//         Converts the internal alloc flags bitfield (struct mem_block field 0x4) to the block type bitfield (struct mem_block field 0x0).
//
//         r0: internal alloc flags
//         return: block type flags
//     - name: FindAvailableMemBlock
//       address:
//         NA: 0x2000F88
//         EU: 0x2000F88
//       description: |-
//         Searches through the given memory arena for a block with enough free space.
//
//         Blocks are searched in reverse order. For object allocations (i.e., not arenas), the block with the smallest amount of free space that still suffices is returned. For arena allocations, the first satisfactory block found is returned.
//
//         r0: memory arena to search
//         r1: internal alloc flags
//         r2: amount of space needed, in bytes
//         return: index of the located block in the arena's block array, or -1 if nothing is available
//     - name: SplitMemBlock
//       address:
//         NA: 0x2001070
//         EU: 0x2001070
//       description: |-
//         Given a memory block at a given index, splits off another memory block of the specified size from the end.
//
//         Since blocks are stored in an array on the memory arena struct, this is essentially an insertion operation, plus some processing on the block being split and its child.
//
//         r0: memory arena
//         r1: block index
//         r2: internal alloc flags
//         r3: number of bytes to split off
//         stack[0]: user alloc flags (to assign to the new block)
//         return: the newly split-off memory block
//     - name: MemAlloc
//       address:
//         NA: 0x2001170
//         EU: 0x2001170
//       description: |-
//         Allocates some memory on the heap, returning a pointer to the starting address.
//
//         Memory allocation is done with region-based memory management. See MEMORY_ALLOCATION_TABLE for more information.
//
//         This function is just a wrapper around MemLocateSet.
//
//         r0: length in bytes
//         r1: flags (see the comment on struct mem_block::user_flags)
//         return: pointer
//     - name: MemFree
//       address:
//         NA: 0x2001188
//         EU: 0x2001188
//       description: |-
//         Frees heap-allocated memory.
//
//         This function is just a wrapper around MemLocateUnset.
//
//         r0: pointer
//     - name: MemArenaAlloc
//       address:
//         NA: 0x200119C
//         EU: 0x200119C
//       description: |-
//         Allocates some memory on the heap and creates a new global memory arena with it.
//
//         The actual allocation part works similarly to the normal MemAlloc.
//
//         r0: desired parent memory arena, or null
//         r1: length of the arena in bytes
//         r2: maximum number of blocks that the arena can hold
//         r3: flags (see MemAlloc)
//         return: memory arena pointer
//     - name: CreateMemArena
//       address:
//         NA: 0x2001280
//         EU: 0x2001280
//       description: |-
//         Creates a new memory arena within a given block of memory.
//
//         This is essentially a wrapper around InitMemArena, accounting for the space needed by the arena metadata.
//
//         r0: memory region in which to create the arena, as {pointer, length}
//         r1: maximum number of blocks that the arena can hold
//         return: memory arena pointer
//     - name: MemLocateSet
//       address:
//         NA: 0x2001390
//         EU: 0x2001390
//       description: |-
//         The implementation for MemAlloc.
//
//         At a high level, memory is allocated by choosing a memory arena, looking through blocks in the memory arena until a free one that's large enough is found, then splitting off a new memory block of the needed size.
//
//         This function is not fallible, i.e., it hangs the whole program on failure, so callers can assume it never fails.
//
//         The name for this function comes from the error message logged on failure, and it reflects what the function does: locate an available block of memory and set it up for the caller.
//
//         r0: desired memory arena for allocation, or null (MemAlloc passes null)
//         r1: length in bytes
//         r2: flags (see MemAlloc)
//         return: pointer to allocated memory
//     - name: MemLocateUnset
//       address:
//         NA: 0x2001638
//         EU: 0x2001638
//       description: |-
//         The implementation for MemFree.
//
//         At a high level, memory is freed by locating the pointer in its memory arena (searching block-by-block) and emptying the block so it's available for future allocations, and merging it with neighboring blocks if they're available.
//
//         r0: desired memory arena for freeing, or null (MemFree passes null)
//         r1: pointer to free
