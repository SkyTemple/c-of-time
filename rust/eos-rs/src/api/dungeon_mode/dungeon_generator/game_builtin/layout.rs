use crate::api::dungeon_mode::dungeon_generator::DungeonLayoutGeneration;

mod private {
    pub trait Sealed {}
}

/// Builtin layout generators.
pub trait BuiltinDungeonLayoutGeneration : DungeonLayoutGeneration + private::Sealed {}
