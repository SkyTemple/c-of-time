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

use crate::api::dungeon_mode::GlobalDungeonData;
use crate::ffi;

pub mod game_builtin;
pub mod reimplementation;

/// A generation cell in the dungeon grid. This is a temporary artifact used during generation,
/// it clusters the grid of dungeon tiles into multiple areas.
pub type DungeonGridCell = ffi::dungeon_grid_cell;

/// High-level trait for generating new dungeons and replacing the current global dungeon.
pub trait DungeonFloorGeneration {
    type EntityGenerator: DungeonEntityGeneration;
    type LayoutGenerator: ?Sized;

    /// Generates a floor using the specified floor properties. This will create a fully working
    /// layout, including entities.
    ///
    /// Whether a fixed floor or a floor using a layout is generated is taken from the properties.
    fn generate_floor(
        &mut self,
        width: usize,
        height: usize,
        properties: &ffi::floor_properties,
    ) -> &mut Self;

    /// Generates a floor using the specified layout. This will create a fully working layout.
    ///
    /// Entities are not spawned.
    fn generate_layout(
        &mut self,
        layout: &mut Self::LayoutGenerator,
        properties: &ffi::floor_properties,
    ) -> &mut Self;

    /// Generate entities, the callback will receive [`Self::EntityGenerator`] to generate them.
    ///
    /// # Implementors
    /// Implementations must call `cb`.
    fn entities<F: FnOnce(&mut Self::EntityGenerator)>(&mut self, cb: F) -> &mut Self;

    /// Write the finished generated dungeon to the global dungeon struct.
    ///
    /// **Important:** For the builtin generator, see the note at
    /// [`game_builtin::GlobalDungeonStructureGenerator::generate`].
    fn generate(self, global_dungeon: &mut GlobalDungeonData);
}

/// Generator for entities on a dungeon floor.
pub trait DungeonEntityGeneration {
    /// Spawn all non-enemy entities, which includes stairs, items, traps, and the player.
    fn spawn_non_enemies(
        &mut self,
        floor_properties: &ffi::floor_properties,
        empty_monster_house: bool,
    );

    /// Spawn all enemies, which includes normal enemies and those in Monster Houses.
    fn spawn_enemies(
        &mut self,
        floor_properties: &ffi::floor_properties,
        empty_monster_house: bool,
    );
}
