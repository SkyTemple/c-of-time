//! Structs and functions to interact with the data of items in a general context.

use crate::api::_common::get_faint_reason;
use crate::api::monsters::MonsterSpeciesId;
use crate::api::moves::MoveId;
use crate::api::overlay::OverlayLoadLease;
use crate::api::types::MonsterTypeId;
use crate::ctypes::c_int;
use crate::ffi;
use core::ffi::CStr;
use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;

/// An item ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
pub type ItemId = ffi::item_id;
impl Copy for ItemId {}

/// This impl provides general metadata about items in the game.
impl ItemId {
    /// Returns the ID struct for the item with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing item),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this item.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Returns the category ID of this item.
    pub fn category(&self) -> ItemCategoryId {
        unsafe { ffi::GetItemCategory(*self) }
    }

    // Returns whether or not this item is an item that can be thrown
    // (`ItemCategoryId::CATEGORY_THROWN_LINE` or `ItemCategoryId::CATEGORY_THROWN_ARC`).
    pub fn can_be_thrown(&self) -> bool {
        unsafe { ffi::IsThrownItem(*self) > 0 }
    }

    // Checks if this item ID is valid(?).
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IsItemValid(*self) > 0 }
    }

    // Checks if the given item ID is valid (using `Self::is_valid`).
    // If so, return the given item ID. Otherwise, return `ItemId::ITEM_PLAIN_SEED`.
    pub fn fallback_if_invalid(self) -> Self {
        unsafe { ffi::EnsureValidItem(self) }
    }

    // Returns whether or not this item is `Self::ITEM_POKE`.
    pub fn is_money(&self) -> bool {
        unsafe { ffi::IsNotMoney(*self) == 0 }
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

    /// Returns the action ID that corresponds to an item given its ID.
    ///
    /// The action is based on the category of the item (see `ITEM_CATEGORY_ACTIONS`), unless the
    /// specified ID is 0x16B, in which case `ACTION_UNK_35` is returned.
    ///
    /// Some items can have unexpected actions, such as thrown items, which have `ACTION_NOTHING`.
    /// This is done to prevent duplicate actions from being listed in the menu (since items always
    /// have a "throw" option), since a return value of `ACTION_NOTHING` prevents the option from
    /// showing up in the menu.
    pub fn get_dungeon_item_action(&self, _ov29: OverlayLoadLease<29>) -> ffi::action::Type {
        unsafe { ffi::GetItemAction(self.0 as c_int) }
    }

    /// Gets the exclusive item offset, which is the item ID relative to that of the first
    /// exclusive item, the Prism Ruff.
    ///
    /// If the given item ID is not a valid item ID, `ItemId::ITEM_PLAIN_SEED` (0x55) is returned.
    /// This is a bug, since 0x55 is the valid exclusive item offset for the Icy Globe.
    pub fn get_exclusive_item_offset_checked_for_validity(&self) -> i32 {
        unsafe { ffi::GetExclusiveItemOffsetEnsureValid(*self) }
    }

    /// Get the minimum quantity for this (thrown) item ID.
    pub fn get_thrown_item_quantity_minimum(&self) -> u8 {
        unsafe { ffi::GetThrownItemQuantityLimit(*self, 0) }
    }

    /// Get the maximum quantity for this (thrown) item ID.
    pub fn get_thrown_item_quantity_maximum(&self) -> u8 {
        unsafe { ffi::GetThrownItemQuantityLimit(*self, 1) }
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

    /// Gets the faint reason code (see HandleFaint) for a given move-item combination.
    ///         
    /// If there's no item, the reason code is the move ID. If the item is an orb, return
    /// FAINT_REASON_ORB_ITEM. Otherwise, return FAINT_REASON_NON_ORB_ITEM.
    pub fn get_faint_reason(&self, move_id: MoveId) -> ffi::faint_reason {
        get_faint_reason(move_id, *self)
    }

    /// Checks if the item has an item category of CATEGORY_BERRIES_SEEDS_VITAMINS
    /// or CATEGORY_FOOD_GUMMIES.
    pub fn is_edible(&self) -> bool {
        unsafe { ffi::IsEdible(*self) > 0 }
    }

    /// Checks if the item is an HM.
    pub fn is_hm(&self) -> bool {
        unsafe { ffi::IsHM(*self) > 0 }
    }

    /// Checks if the item is a gummi.
    pub fn is_gummi(&self) -> bool {
        unsafe { ffi::IsGummi(*self) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn normalize_treasure_box(&self) -> ItemId {
        unsafe { ffi::NormalizeTreasureBox(*self) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn exclusive_item_type(&self) -> u8 {
        unsafe { ffi::GetExclusiveItemType(*self) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn exclusive_item_parameter(&self) -> i16 {
        unsafe { ffi::GetExclusiveItemParameter(*self) }
    }

    /// Returns the name of this item
    pub fn name(&self) -> &CStr {
        unsafe {
            let cstr_raw = &mut *ffi::GetItemName(*self);
            CStr::from_ptr(cstr_raw)
        }
    }

    /// Returns the price that the item is sold at.
    pub fn sell_price(&self) -> u16 {
        unsafe { ffi::GetItemSellPrice(*self) }
    }

    /// Returns the price that the item ca be bought at.
    pub fn buy_price(&self) -> u16 {
        unsafe { ffi::GetItemBuyPrice(*self) }
    }

    /// Sprite ID
    pub fn sprite_id(&self) -> u8 {
        unsafe { ffi::GetItemSpriteId(*self) }
    }

    /// Palette ID
    pub fn palette_id(&self) -> u8 {
        unsafe { ffi::GetItemPaletteId(*self) }
    }

    /// ID of the string for the action.
    pub fn action_name_id(&self) -> u8 {
        unsafe { ffi::GetItemActionName(*self) }
    }

    /// Move ID corresponding to this item.
    pub fn move_id(&self) -> MoveId {
        unsafe { ffi::GetItemMoveId(*self) }
    }

    /// Returns true, if the game is also present in Time/Darkness.
    pub fn is_in_time_darkness(&self) -> bool {
        unsafe { ffi::IsItemInTimeDarkness(*self) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn is_exclusive_item_for_monster(
        &self,
        monster_id: MonsterSpeciesId,
        type1: MonsterTypeId,
        type2: MonsterTypeId,
    ) -> bool {
        unsafe { ffi::IsExclusiveItemIdForMonster(*self, monster_id, type1, type2) > 0 }
    }
}

impl From<ItemId> for u32 {
    fn from(v: ItemId) -> Self {
        v.0
    }
}

/// An item slot. It has a quantity if it's stackable
/// and optionally a reference to an entity that holds it.
///
/// A quantity of zero indicates that the item is not stackable.
pub type Item = ffi::item;

impl Item {
    /// Allocates a new item.
    ///
    /// This will resolve the quantity based on the item type:
    ///
    /// - For Poké, the quantity code will always be set to 1.
    /// - For thrown items, the quantity code will be randomly generated on the range of valid
    ///   quantities for that item type.
    /// - For non-stackable items, the quantity code will always be set to 0.
    /// - Otherwise, the quantity will be assigned from the quantity argument.
    pub fn new(item_id: ItemId, quantity: u16, sticky: bool) -> Self {
        // SAFETY: We init the value right after.
        let mut slf: Self = unsafe { mem::zeroed() };
        slf.init(item_id, quantity, sticky);
        slf
    }

    /// Initialize an item struct with the given information.
    ///
    /// This will resolve the quantity based on the item type:
    ///
    /// - For Poké, the quantity code will always be set to 1.
    /// - For thrown items, the quantity code will be randomly generated on the range of valid
    ///   quantities for that item type.
    /// - For non-stackable items, the quantity code will always be set to 0.
    /// - Otherwise, the quantity will be assigned from the quantity argument.
    pub fn init(&mut self, item_id: ItemId, quantity: u16, sticky: bool) {
        unsafe { ffi::InitItem(self, item_id, quantity, sticky as ffi::bool_) }
    }

    /// Returns the price that the item is sold at as it should be displayed in shops.
    pub fn sell_price_display(&self) -> i32 {
        unsafe { ffi::GetDisplayedSellPrice(force_mut_ptr!(self)) }
    }

    /// Returns the price that the item ca be bought at as it should be displayed in shops.
    pub fn buy_price_display(&self) -> i32 {
        unsafe { ffi::GetDisplayedBuyPrice(force_mut_ptr!(self)) }
    }

    /// Returns the price that the item is sold at.
    pub fn sell_price(&self) -> i32 {
        unsafe { ffi::GetActualSellPrice(force_mut_ptr!(self)) }
    }

    /// Returns the price that the item ca be bought at.
    pub fn buy_price(&self) -> i32 {
        unsafe { ffi::GetActualBuyPrice(force_mut_ptr!(self)) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn is_exclusive_item_for_monster(
        &self,
        monster_id: MonsterSpeciesId,
        type1: MonsterTypeId,
        type2: MonsterTypeId,
    ) -> bool {
        unsafe {
            ffi::IsExclusiveItemForMonster(force_mut_ptr!(self), monster_id, type1, type2) > 0
        }
    }
}

/// An exclusive item effect ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
pub type ExclusiveItemEffectId = ffi::exclusive_item_effect_id;
impl Copy for ExclusiveItemEffectId {}

/// This impl provides general metadata about exclusive item effects in the game.
impl ExclusiveItemEffectId {
    /// Returns the ID struct for the exclusive item effect with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing exclusive item effect),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
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

impl From<ExclusiveItemEffectId> for u32 {
    fn from(v: ExclusiveItemEffectId) -> Self {
        v.0
    }
}

/// An item category ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
pub type ItemCategoryId = ffi::item_category;
impl Copy for ItemCategoryId {}

/// This impl provides general metadata about item categories in the game.
impl ItemCategoryId {
    /// Returns the ID struct for the item category with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing item),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this item category.
    pub const fn id(&self) -> u32 {
        self.0
    }
}

impl From<ItemCategoryId> for u32 {
    fn from(v: ItemCategoryId) -> Self {
        v.0
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
    pub fn is_in_bag(&self, item_id: ItemId) -> bool {
        unsafe { ffi::IsItemInBag(item_id) > 0 }
    }

    /// Count the amount of the specified item in the player's bag.
    pub fn count_item_type(&self, item_id: ItemId) -> i32 {
        unsafe { ffi::CountItemTypeInBag(item_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn len(&self) -> i32 {
        unsafe { ffi::GetNbItemsInBag() }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn count_type(&self, item_id: ItemId) -> i32 {
        unsafe { ffi::CountNbItemsOfTypeInBag(item_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn is_item_with_flag_in_bag(&self, item_id: ItemId, flags: i32) -> bool {
        unsafe { ffi::IsItemWithFlagsInBag(item_id, flags) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn capacity(&self) -> i32 {
        unsafe { ffi::GetCurrentBagCapacity() }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn contains_held_item(&self, item: &Item) -> bool {
        unsafe { ffi::IsHeldItemInBag(force_mut_ptr!(item)) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn contains_item_for_special_spawn(&self) -> bool {
        unsafe { ffi::IsItemForSpecialSpawnInBag() > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn has_storable_items(&self) -> bool {
        unsafe { ffi::HasStorableItems() > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn get_index_of_item(&self, item: &Item) -> i32 {
        unsafe { ffi::GetItemIndex(force_mut_ptr!(item)) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn get_index_of_equiv_item(&self, item: &Item) -> i32 {
        unsafe { ffi::GetEquivItemIndex(force_mut_ptr!(item)) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn equipped_throwable_item(&self) -> i32 {
        unsafe { ffi::GetEquippedThrowableItem() }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn first_equipped_throwable_item_of_type(&self, item_id: ItemId) -> i32 {
        unsafe { ffi::GetFirstUnequippedItemOfType(item_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn copy_item(&self, index: i32) -> Option<Item> {
        let mut item: MaybeUninit<Item> = MaybeUninit::zeroed();
        let exists = unsafe { ffi::CopyItemAtIdx(index, item.as_mut_ptr()) > 0 };
        if exists {
            unsafe { Some(item.assume_init()) }
        } else {
            None
        }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn get_item(&self, index: i32) -> Option<&Item> {
        let item_ref = unsafe { ffi::GetItemAtIdx(index) };
        if item_ref.is_null() {
            None
        } else {
            unsafe { Some(&*item_ref) }
        }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn get_item_mut(&mut self, index: i32) -> Option<&mut Item> {
        let item_ref = unsafe { ffi::GetItemAtIdx(index) };
        if item_ref.is_null() {
            None
        } else {
            unsafe { Some(&mut *item_ref) }
        }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_empty_items(&mut self) {
        unsafe { ffi::RemoveEmptyItemsInBag() };
    }

    /// Note: unverified, ported from Irdkwia's notes
    ///
    /// Returns unknown.
    pub fn remove_item_no_hole(&mut self, index: i32) -> bool {
        unsafe { ffi::RemoveItemNoHole(index) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_item_no_hole_check(&mut self, index: i32) -> bool {
        unsafe { ffi::RemoveItemNoHoleCheck(index) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_item(&mut self, index: i32) {
        unsafe { ffi::RemoveItem(index) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_held_item_no_hole(&mut self, held_index: i32) {
        unsafe { ffi::RemoveHeldItemNoHole(held_index) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_held_by_id_and_stack_no_hole(&mut self, item: &mut Item) -> bool {
        unsafe { ffi::RemoveItemByIdAndStackNoHole(item) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_equiv_item(&mut self, item: &mut Item) -> bool {
        unsafe { ffi::RemoveEquivItem(item) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_equiv_item_no_hole(&mut self, item: &mut Item) -> bool {
        unsafe { ffi::RemoveEquivItemNoHole(item) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn decrement_stack_item(&mut self, item: &mut Item) -> bool {
        unsafe { ffi::DecrementStackItem(item) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn decrement_stack_item_no_hole_check(&mut self, item: &mut Item) -> bool {
        unsafe { ffi::DecrementStackItem(item) > 0 }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_first_unequipped_item_of_type(&mut self, item_id: ItemId) -> bool {
        unsafe { ffi::RemoveFirstUnequippedItemOfType(item_id) > 0 }
    }

    /// WARNING! Does not remove from party items
    ///
    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_all_items(&mut self) {
        unsafe { ffi::RemoveAllItems() }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn remove_all_items_starting_at(&mut self, index: i32) {
        unsafe { ffi::RemoveAllItemsStartingAt(index) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn add_item(&mut self, item: &mut Item, holder: Option<i32>) -> bool {
        match holder {
            None => unsafe { ffi::AddItemToBagNoHeld(item) > 0 },
            Some(holder) => unsafe { ffi::AddItemToBag(item, holder) > 0 },
        }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn has_exclusive_item_type_for_monster(
        &mut self,
        excl_type: i32,
        monster_id: MonsterSpeciesId,
        type1: MonsterTypeId,
        type2: MonsterTypeId,
    ) -> ItemId {
        unsafe { ffi::BagHasExclusiveItemTypeForMonster(excl_type, monster_id, type1, type2) }
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
            ffi::CountItemTypeInStorage(&mut ffi::bulk_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                quantity: 0,
            })
        }
    }

    /// Adds the specified amount of an item to the player's bag. Returns whether or not any
    /// items could be added.
    pub fn add_item(&mut self, item_id: ItemId, quantity: u16) -> bool {
        unsafe {
            ffi::AddItemToStorage(&mut ffi::bulk_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                quantity,
            }) > 0
        }
    }

    /// Removes (the specified amount...?) of the given item type from the storage.
    pub fn remove_item(&mut self, item_id: ItemId, quantity: u16) -> bool {
        unsafe {
            ffi::RemoveItemsTypeInStorage(&mut ffi::bulk_item {
                id: ffi::item_id_16 {
                    _bitfield_align_1: [],
                    _bitfield_1: ffi::item_id_16::new_bitfield_1(item_id),
                },
                quantity,
            }) > 0
        }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn is_item_in_treasure_boxes(&self, item_id: ItemId) -> bool {
        unsafe { ffi::IsItemInTreasureBoxes(item_id) > 0 }
    }
}

/// An item slot in the players inventory.
pub enum ItemSlot {
    /// A slot in the bag.
    Bag(i32),
    /// A slot in the storage.
    Storage(i32),
}

/// Returns `Some(ItemSlot::Bag(x))` if `item_id` is at position x in the bag.
/// Returns `Some(ItemSlot::Storage(x))` if `item_id` is at position x in storage.
/// Returns `None` if item is not found.
pub fn find_item_in_inventory(
    _bag: &InventoryBag,
    _storage: &InventoryStorage,
    item_id: ItemId,
) -> Option<ItemSlot> {
    // SAFETY: We have a reference to the bag and storage.
    let raw = unsafe { ffi::FindItemInInventory(item_id) };
    if raw >= 0x8000 {
        Some(ItemSlot::Storage(raw - 0x8000))
    } else if raw > 0 {
        Some(ItemSlot::Bag(raw))
    } else {
        None
    }
}

/// `category_num` and `item_num` are numbers in range 0-10000
///
/// Note: unverified, ported from Irdkwia's notes
///
/// # Safety
/// The parameters must be valid indices.
pub unsafe fn get_item_id_from_list(list_id: i32, category_num: i32, item_num: i32) -> ItemId {
    unsafe { ffi::GetItemIdFromList(list_id, category_num, item_num) }
}
