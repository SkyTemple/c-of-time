//! The dungeon generation as a collection of traits with two implementations.
//!
//! - The game's implementation ([`game_builtin`]). Limited in flexibility. Works on the current
//!   global dungeon struct directly.
//!   Some parameters do not work as advertised since the game is hardcoded to expect some values
//!   (probably compiler / linker optimizations).
//!
//!   To get an instance of the game's implementation, use
//!   [`super::GlobalDungeonData::get_builtin_dungeon_generator`].
//!
//! - A reimplementation ([`reimplementation`]). Incomplete but hopefully more flexible and
//!   expandable/modular in the long term.
//!
//! The generator modules may expose some more specific generation logic to them, but in general
//! it is recommended to work with the [`DungeonFloorGeneration`] and related traits instead of
//! implementation-specific code.

use crate::api::objects::fixed_room_catalog;
use crate::ffi;

pub mod game_builtin;
pub mod reimplementation;

/// A generation cell in the dungeon grid. This is a temporary artifact used during generation,
/// it clusters the grid of dungeon tiles into multiple areas.
pub type DungeonGridCell = ffi::dungeon_grid_cell;

/// High-level trait for generating new dungeons and replacing the current global dungeon.
pub trait DungeonFloorGeneration
{
    type EntityGenerator: DungeonEntityGeneration;
    type PiecesGenerator: DungeonPiecesGeneration;
    type LayoutGenerator: ?Sized;

    /// Generates a floor using the specified floor properties. This will create a fully working
    /// layout, including entities.
    ///
    /// Whether a standard floor or a fixed floor is generated is taken from the properties,
    /// same goes for the layout. To force a specific behaviour, see the other functions.
    fn generate_floor(&mut self, width: usize, height: usize, properties: &ffi::floor_properties) -> &mut Self;

    /// Generates a standard floor using the specified floor properties.
    ///
    /// A standard floor is a floor that is not generated from a single fixed room.
    /// It will usually be a combination of applying a
    /// layout ([`Self::generate_layout`]) and adding bits and pieces to it ([`Self::pieces`]).
    ///
    /// Entities are not spawned (TODO: is that true?).
    fn generate_standard_floor(&mut self, width: usize, height: usize, properties: &ffi::floor_properties) -> &mut Self;

    /// Generates a fixed floor using the specified floor properties.
    ///
    /// A fixed floor is a floor that is generated from a single fixed room.
    ///
    /// Entities are not spawned (TODO: is that true?).
    fn generate_fixed_floor(&mut self, fixed_room_id: fixed_room_catalog::Type, properties: &ffi::floor_properties) -> &mut Self;

    /// Generates a new dungeon floor using a specific layout.
    ///
    /// This will most likely not be fully finished standard floor, see [`Self::pieces`] to finish
    /// the layout.
    ///
    /// Entities are not spawned (TODO: is that true?).
    fn generate_layout(&mut self, width: usize, height: usize, layout: &mut Self::LayoutGenerator, properties: &ffi::floor_properties) -> &mut Self;

    /// Generate individual bits and pieces, the callback will receive
    /// [`Self::PiecesGenerator`] to generate them.
    ///
    /// # Implementors
    /// Implementations must call `cb`.
    fn pieces<F: FnOnce(&mut Self::PiecesGenerator)>(&mut self, cb: F) -> &mut Self;

    /// Generate entities, the callback will receive [`Self::EntityGenerator`] to generate them.
    ///
    /// # Implementors
    /// Implementations must call `cb`.
    fn entities<F: FnOnce(&mut Self::EntityGenerator)>(&mut self, cb: F) -> &mut Self;

    /// Write the finished generated dungeon to the global dungeon struct.
    ///
    /// **Important:** For the builtin generator, see the note at
    /// [`game_builtin::GlobalDungeonStructureGenerator::generate`].
    ///
    /// # Safety
    /// The caller needs to make sure the global dungeon can currently be safely
    /// replaced with a new dungeon.
    unsafe fn generate(self);
}

/// Utility generator to manually generate little bits and pieces into a dungeon floor.
pub trait DungeonPiecesGeneration {

}

/// Generator for entities on a dungeon floor.
pub trait DungeonEntityGeneration {

}
