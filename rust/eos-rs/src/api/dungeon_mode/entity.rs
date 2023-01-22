use crate::api::dungeon_mode::*;
use crate::api::enums::DungeonEntityType;
use crate::api::items::Item;

/// Entity in a dungeon. Has a [`DungeonEntityType`].
pub type DungeonEntity = ffi::entity;

/// Extended info struct for [`DungeonEntity`] objects that are items.
#[deprecated(note = "Use eos_rs::api::items::Item instead.")]
pub type DungeonItem = crate::api::items::Item;
/// Extended info struct for [`DungeonEntity`] objects that are traps.
pub type DungeonTrap = ffi::trap;
/// A struct representing a single dungeon tile.
pub type DungeonTile = ffi::tile;

/// # Important safety note
/// These implementations can assume that overlay 29 is loaded (since this is the
/// only context dungeon entities are actually relevant). If you somehow manually construct types
/// of this struct for some reason, you NEED to make sure overlay 29 is loaded when using some f
/// unctions of this trait.
/// The methods in this impl are marked safe for convenience, since in its intended use case,
/// overlay 29 will always be loaded.
impl DungeonEntity {
    /// Checks if a given entity is actually valid.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn is_valid(slf: *mut Self) -> bool {
        // SAFETY: The lease passed into the function promises us that the overlay is loaded.
        //         Since this function is intended to actually check if the entity is valid,
        //         it's safe to call, even if the `slf` pointer is invalid.
        unsafe { ffi::EntityIsValid(slf) > 0 }
    }

    /// Entity type. Invalid values will return None.
    pub fn entity_type(&self) -> Option<DungeonEntityType> {
        self.type_.try_into().ok()
    }

    /// This returns the monster info struct for the entity,
    /// returns None if the entity is not a monster.
    pub fn info_for_monster(&self) -> Option<DungeonMonsterRef> {
        if self.entity_type() == Some(DungeonEntityType::Monster) {
            unsafe {
                Some(DungeonMonsterRef(
                    &*(self.info as *const ffi::monster),
                    self,
                ))
            }
        } else {
            None
        }
    }

    /// This returns the item info struct for the entity,
    /// returns None if the entity is not an item.
    pub fn info_for_item(&self) -> Option<&Item> {
        if self.entity_type() == Some(DungeonEntityType::Item) {
            unsafe { Some(&*(self.info as *const Item)) }
        } else {
            None
        }
    }

    /// This returns the trap info struct for the entity,
    /// returns None if the entity is not a trap.
    pub fn info_for_trap(&self) -> Option<&DungeonTrap> {
        if self.entity_type() == Some(DungeonEntityType::Trap) {
            unsafe { Some(&*(self.info as *const DungeonTrap)) }
        } else {
            None
        }
    }

    /// This returns the monster info struct for the entity,
    /// returns None if the entity is not a monster.
    pub fn info_for_monster_mut(&mut self) -> Option<DungeonMonsterMut> {
        if self.entity_type() == Some(DungeonEntityType::Monster) {
            unsafe {
                Some(DungeonMonsterMut(
                    &mut *(self.info as *mut ffi::monster),
                    self,
                ))
            }
        } else {
            None
        }
    }

    /// This returns the item info struct for the entity,
    /// returns None if the entity is not an item.
    pub fn info_for_item_mut(&mut self) -> Option<&mut Item> {
        if self.entity_type() == Some(DungeonEntityType::Item) {
            unsafe { Some(&mut *(self.info as *mut Item)) }
        } else {
            None
        }
    }

    /// This returns the trap info struct for the entity,
    /// returns None if the entity is not a trap.
    pub fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap> {
        if self.entity_type() == Some(DungeonEntityType::Trap) {
            unsafe { Some(&mut *(self.info as *mut DungeonTrap)) }
        } else {
            None
        }
    }

    /// Returns the tile, that the entity is located at.
    pub fn get_tile(&self) -> Option<&DungeonTile> {
        let ptr = unsafe { ffi::GetTileAtEntity(force_mut_ptr!(self)) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }

    /// Returns the tile, that the entity is located at.
    pub fn get_tile_mut(&mut self) -> Option<&mut DungeonTile> {
        let ptr = unsafe { ffi::GetTileAtEntity(self) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *ptr })
        }
    }
}
