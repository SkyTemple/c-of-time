//! Type aliases for common ffi types and modules.
//! These are generally to be preferred over directly
//! using the ffi types and modules.

use crate::ffi;

pub type DungeonEntity = ffi::entity;
pub type DungeonMove = ffi::move_;
pub type DungeonItem = ffi::item;
pub type DungeonTrap = ffi::trap;
pub type DungeonMonster = ffi::monster;

pub use ffi::entity_type as entity_type;
pub use ffi::item_id as item_catalog;
pub use ffi::move_id as move_catalog;
