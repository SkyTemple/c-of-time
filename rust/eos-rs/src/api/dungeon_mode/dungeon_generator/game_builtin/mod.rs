//! The game's built-in dungeon generator.
//!
//! **IMPORTANT to know** for this implementation: A lot of functions will not work as expected,
//! since some low level functions just ignore some parameters passed in completely.

mod grid;
mod global_dungeon;
mod layout;

pub use self::global_dungeon::{GlobalDungeonStructureGenerator, GlobalDungeonEntityGenerator};
pub use self::grid::DungeonGridMutator;

pub use self::layout::*;
