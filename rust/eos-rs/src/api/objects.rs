//! Type aliases and small wrapper types for common ffi types and modules.
//! These are generally to be preferred over directly
//! using the ffi types and modules.
//!
//! The `..._catalog` re-exports are catalogs of IDs of items, moves, etc.
//! that exist in the base game. The `Type` item of these catalogs specifies
//! the type of the IDs.

use core::ops::{Deref, DerefMut};
use crate::ffi;

pub use ffi::item_id as item_catalog;
pub use ffi::move_id as move_catalog;
pub use ffi::ability_id as ability_catalog;
pub use ffi::type_id as type_catalog;
pub use ffi::iq_skill_id as iq_skill_catalog;
pub use ffi::iq_group_id as iq_group_catalog;
pub use ffi::exclusive_item_effect_id as exclusive_item_effect_catalog;
pub use ffi::dungeon_id as dungeon_catalog;
pub use ffi::fixed_room_id as fixed_room_catalog;

/// A monster move.
pub type Move = ffi::move_;

/// Entity in a dungeon. Has a [`crate::api::dungeon_mode::DungeonEntityType`].
/// Use the [`crate::api::dungeon_mode::DungeonEntityExt`] trait to access dungeon related
/// functionality.
pub type DungeonEntity = ffi::entity;
/// Extended info struct for [`DungeonEntity`] objects that are items.
pub type DungeonItem = ffi::item;
/// Extended info struct for [`DungeonEntity`] objects that are traps.
pub type DungeonTrap = ffi::trap;
/// Reference type for the info struct for [`DungeonEntity`] objects that are monsters.
///
/// This and `DungeonMonsterMut` exist for ease of use with the
/// [`crate::api::dungeon_mode::DungeonMonsterExtRead`] and
/// [`crate::api::dungeon_mode::DungeonMonsterExtWrite`] traits, where some of their functions
/// actually require the entity to work with, we store a reference to the entity struct in our
/// monster wrapper structs.
pub struct DungeonMonsterRef<'a>(pub &'a ffi::monster, pub &'a ffi::entity);
/// Mutable reference type for the info struct for [`DungeonEntity`] objects that are monsters.
pub struct DungeonMonsterMut<'a>(pub &'a mut ffi::monster, pub &'a mut ffi::entity);
/// A struct representing a single dungeon tile.
pub type DungeonTile = ffi::tile;

/// Essentially [`core::convert::AsRef`].
/// (Sadly we can't use the `AsRef` trait, because it doesn't allow explicit lifetimes.)
impl<'a> DungeonMonsterMut<'a> {
    /// Get a [`DungeonMonsterRef`] from this [`DungeonMonsterMut`].
    pub fn as_ref(&'a self) -> DungeonMonsterRef<'a> {
        DungeonMonsterRef(self.0, self.1)
    }
}
impl<'a> Deref for DungeonMonsterRef<'a> {
    type Target = ffi::monster;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> Deref for DungeonMonsterMut<'a> {
    type Target = ffi::monster;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a> DerefMut for DungeonMonsterMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
