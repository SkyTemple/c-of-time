use crate::api::dungeon_mode::*;
use crate::api::objects::*;

/// Extension trait for [`DungeonEntity`].
///
/// # Important safety note
/// Implementations of this trait can assume that overlay 29 is loaded (since this is the
/// only context dungeon entities are actually relevant). If you manually implement this trait,
/// for some reason, you NEED to make sure overlay 29 is loaded when using some functions of this
/// trait.
/// If you use the DungeonEntity structs manually outside of dungeon mode, this trait will be
/// unsafe to use.
/// The trait and its functions are marked safe for convenience, since in it's intended use case,
/// overlay 29 will always be loaded.
pub trait DungeonEntityExt {
    /// Checks if a given entity is actually valid.
    fn is_valid(slf: *mut Self) -> bool;

    /// Entity type. Invalid values will return None.
    fn entity_type(&self) -> Option<DungeonEntityType>;

    /// This returns the monster info struct for the entity,
    /// panics if the entity is not a monster.
    fn info_for_monster(&self) -> Option<DungeonMonsterRef>;

    /// This returns the item info struct for the entity,
    /// panics if the entity is not an item.
    fn info_for_item(&self) -> Option<&DungeonItem>;

    /// This returns the trap info struct for the entity,
    /// panics if the entity is not a trap.
    fn info_for_trap(&self) -> Option<&DungeonTrap>;

    /// This returns the monster info struct for the entity,
    /// panics if the entity is not a monster.
    fn info_for_monster_mut(&mut self) -> Option<DungeonMonsterMut>;

    /// This returns the item info struct for the entity,
    /// panics if the entity is not an item.
    fn info_for_item_mut(&mut self) -> Option<&mut DungeonItem>;

    /// This returns the trap info struct for the entity,
    /// panics if the entity is not a trap.
    fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap>;
}

impl DungeonEntityExt for DungeonEntity {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_valid(slf: *mut Self) -> bool {
        // SAFETY: The lease passed into the function promises us that the overlay is loaded.
        //         Since this function is intended to actually check if the entity is valid,
        //         it's safe to call, even if the `slf` pointer is invalid.
        unsafe { ffi::EntityIsValid(slf) > 0 }
    }

    fn entity_type(&self) -> Option<DungeonEntityType> {
        self.type_.try_into().ok()
    }

    fn info_for_monster(&self) -> Option<DungeonMonsterRef> {
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

    fn info_for_item(&self) -> Option<&DungeonItem> {
        if self.entity_type() == Some(DungeonEntityType::Item) {
            unsafe { Some(&*(self.info as *const DungeonItem)) }
        } else {
            None
        }
    }

    fn info_for_trap(&self) -> Option<&DungeonTrap> {
        if self.entity_type() == Some(DungeonEntityType::Trap) {
            unsafe { Some(&*(self.info as *const DungeonTrap)) }
        } else {
            None
        }
    }

    fn info_for_monster_mut(&mut self) -> Option<DungeonMonsterMut> {
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

    fn info_for_item_mut(&mut self) -> Option<&mut DungeonItem> {
        if self.entity_type() == Some(DungeonEntityType::Item) {
            unsafe { Some(&mut *(self.info as *mut DungeonItem)) }
        } else {
            None
        }
    }

    fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap> {
        if self.entity_type() == Some(DungeonEntityType::Trap) {
            unsafe { Some(&mut *(self.info as *mut DungeonTrap)) }
        } else {
            None
        }
    }
}
