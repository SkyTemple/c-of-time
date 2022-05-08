//! The dungeon generation as a collection of traits with two implementations.
//!
//! - The game's implementation ([`game_builtin`]). Limited in flexibility. Works on the current
//!   global dungeon struct directly.
//!   Some parameters do not work as advertised since the game is hardcoded to expect some values
//!   (probably compiler / linker optimizations).
//! - A reimplementation ([`reimplementation`]). Incomplete but hopefully more flexible and
//!   expandable/modular in the long term.
//!
//! The generator modules may expose some more specifc generation logic to them, but in general
//! it is recommended to work with the [`DungeonFloorGeneration`] and related traits instead of
//! implementation-specific code.

use crate::ffi;

pub mod game_builtin;
pub mod reimplementation;

/// A generation cell in the dungeon grid. This is a temporary artifact used during generation,
/// it clusters the grid of dungeon tiles into multiple areas.
pub type DungeonGridCell = ffi::dungeon_grid_cell;

/// High-level trait for generating new dungeons and replacing the current global dungeon.
///
/// Functions are marked unsafe because they may use global data.
///
/// Implementations may mark some functions as not `unsafe`. In that case they are *not* directly
/// working with the global dungeon struct but rather a copy. If they have a mix of `unsafe` and
/// safe functions, they must make sure that calls to unsafe generation steps flush the generated
/// partial dungeon to the global dungeon struct.
pub trait DungeonFloorGeneration
{
    type EntityGeneration: DungeonEntityGeneration + ?Sized;
    type PiecesGeneration: DungeonPiecesGeneration + ?Sized;
    type LayoutGeneration: DungeonLayoutGeneration + ?Sized;
}

/// Generator for the structure of a specific dungeon layout.
///
/// Functions are marked unsafe because they may use global data.
///
/// Implementations may mark some functions as not `unsafe`. See [`DungeonFloorGeneration`].
pub trait DungeonLayoutGeneration {

}

/// Utility generator to manually generate little bits and pieces into a dungeon floor.
///
/// Functions are marked unsafe because they may use global data.
///
/// Implementations may mark some functions as not `unsafe`. See [`DungeonFloorGeneration`].
pub trait DungeonPiecesGeneration {

}

/// Generator for entities on a dungeon floor.
///
/// Functions are marked unsafe because they may use global data.
///
/// Implementations may mark some functions as not `unsafe`. See [`DungeonFloorGeneration`].
pub trait DungeonEntityGeneration {

}
