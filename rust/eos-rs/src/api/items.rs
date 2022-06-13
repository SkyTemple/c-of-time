//! Structs and functions to interact with the data of items in a general context.

use crate::ffi;
use core::marker::PhantomData;

/// An item ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type ItemId = ffi::item_id;
impl Copy for ItemId {}

/// This impl provides general metadata about items in the game.
impl ItemId {
    /// Returns the ID struct for the item with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing item),
    /// otherwise this is UB.
    pub const unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this item.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Checks if the item is one of the aura bows received at the start of the game.
    pub fn is_aura_bow(&self) -> bool {
        unsafe { ffi::IsAuraBow(*self) > 0 }
    }

    /// Gets the exclusive item offset, which is the item ID relative to that of the first exclusive
    /// item, the Prism Ruff.
    pub fn get_exclusive_item_offset(&self) -> i32 {
        unsafe { ffi::GetExclusiveItemOffset(*self) }
    }

    /// Applies stat boosts from an exclusive item.
    pub fn apply_exclusive_item_stat_boosts(
        &self,
        atk_to_modify: &mut u8,
        sp_atk_to_modify: &mut u8,
        def_to_modify: &mut u8,
        sp_def_to_modify: &mut u8,
    ) {
        unsafe {
            ffi::ApplyExclusiveItemStatBoosts(
                *self,
                atk_to_modify,
                sp_atk_to_modify,
                def_to_modify,
                sp_def_to_modify,
            )
        }
    }
}

/// An exclusive item effect ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type ExclusiveItemEffectId = ffi::exclusive_item_effect_id;
impl Copy for ExclusiveItemEffectId {}

/// This impl provides general metadata about exclusive item effects in the game.
impl ExclusiveItemEffectId {
    /// Returns the ID struct for the exclusive item effect with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing exclusive item effect),
    /// otherwise this is UB.
    pub const unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this exclusive item effect.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Sets the bit for an exclusive item effect.
    pub fn set_exclusive_item_effect(&self, effect_flags: &mut u32) {
        unsafe { ffi::SetExclusiveItemEffect(effect_flags, *self) }
    }

    /// Tests the exclusive item bitvector for a specific exclusive item effect.
    pub fn test_exclusive_item_effect_flag(&self, effect_flags: &mut u32) -> bool {
        unsafe { ffi::ExclusiveItemEffectFlagTest(effect_flags, *self) > 0 }
    }
}

/// The money that the player is carrying.
pub struct MoneyCarried(PhantomData<()>);

impl MoneyCarried {
    /// Returns an internal reference to the money carried. Note that this isn't a reference
    /// to the actual struct in memory (yet).
    ///
    /// # Safety
    /// This is unsafe, since it essentially borrows a global variable mutably, see
    /// safety rules for `static mut`s.
    pub unsafe fn get() -> Self {
        Self(PhantomData)
    }

    /// Sets the amount of money the player is carrying, clamping the value to the range
    /// [0, MAX_MONEY_CARRIED].
    pub fn set_money(&mut self, money: i32) {
        unsafe { ffi::SetMoneyCarried(money) }
    }
}

/// The money that the player is storing at the Duskull Bank.
pub struct MoneyStored(PhantomData<()>);

impl MoneyStored {
    /// Returns an internal reference to the money stored. Note that this isn't a reference
    /// to the actual struct in memory (yet).
    ///
    /// # Safety
    /// This is unsafe, since it essentially borrows a global mutable variable (`static mut`), see
    /// safety rules for `static mut`s.
    pub unsafe fn get() -> Self {
        Self(PhantomData)
    }

    /// Sets the amount of money the player has stored in the Duskull Bank, clamping the value to the
    /// range [0, MAX_MONEY_STORED].
    pub fn set_money(&mut self, money: i32) {
        unsafe { ffi::SetMoneyStored(money) }
    }
}

/// The player's bag.
pub struct InventoryBag(PhantomData<()>);

impl InventoryBag {
    /// Returns an internal reference to the player's bag. Note that this isn't a reference
    /// to the actual struct in memory (yet).
    ///
    /// # Safety
    /// This is unsafe, since it essentially borrows a global mutable variable (`static mut`), see
    /// safety rules for `static mut`s.
    pub unsafe fn get() -> Self {
        Self(PhantomData)
    }

    /// Checks if the player's bag is full.
    pub fn is_full(&self) -> bool {
        unsafe { ffi::IsBagFull() > 0 }
    }

    /// Count the amount of the specified item in the player's bag.
    pub fn count_item_type(&self, item_id: ItemId) -> i32 {
        unsafe { ffi::CountItemTypeInBag(item_id) }
    }

    /// Adds the specified amount of an item to the player's bag. Returns whether or not any
    /// items could be added.
    pub fn add_item(&mut self, item_id: ItemId, amount: u16) -> bool {
        unsafe {
            ffi::AddItemToBag(&mut ffi::owned_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                amount,
            }) > 0
        }
    }
}

/// The player's inventory in the storage.
pub struct InventoryStorage(PhantomData<()>);

impl InventoryStorage {
    /// Returns an internal reference to the player's inventory in the storage.
    /// Note that this isn't a reference to the actual struct in memory (yet).
    ///
    /// # Safety
    /// This is unsafe, since it essentially borrows a global mutable variable (`static mut`), see
    /// safety rules for `static mut`s.
    pub unsafe fn get() -> Self {
        Self(PhantomData)
    }

    /// Special process 0x39.
    ///
    /// This is *probably* is_storage_full: checks if the player's storage is full.
    pub fn is_full(&self) -> bool {
        unsafe { ffi::ScriptSpecialProcess0x39() > 0 }
    }

    /// Count the amount of the specified item in the player's storage.
    pub fn count_item_type(&self, item_id: ItemId) -> i32 {
        unsafe {
            ffi::CountItemTypeInStorage(&mut ffi::owned_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                amount: 0,
            })
        }
    }

    /// Adds the specified amount of an item to the player's bag. Returns whether or not any
    /// items could be added.
    pub fn add_item(&mut self, item_id: ItemId, amount: u16) -> bool {
        unsafe {
            ffi::AddItemToStorage(&mut ffi::owned_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                amount,
            }) > 0
        }
    }

    /// Removes (the specified amount...?) of the given item type from the storage.
    pub fn remove_item(&mut self, item_id: ItemId, amount: u16) -> bool {
        unsafe {
            ffi::RemoveItemsTypeInStorage(&mut ffi::owned_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                amount,
            }) > 0
        }
    }
}
