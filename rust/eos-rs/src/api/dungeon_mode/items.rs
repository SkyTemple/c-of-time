use crate::api::items::{Item, ItemId};
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

impl Item {
    /// Initializes an item struct with the given information.
    pub fn generate_explicit(
        &mut self,
        item_id: ItemId,
        quantity: u16,
        sticky: bool,
        _ov29: OverlayLoadLease<29>,
    ) {
        unsafe { ffi::GenerateItemExplicit(self, item_id, quantity, sticky as ffi::bool_) }
    }

    /// Initializes an item struct with the given information.
    /// This wraps [`Self::init`], but with extra logic to resolve the item's stickiness.
    /// It also calls [`Self::generate_item_quantity`] for Poké.
    pub fn generate(
        &mut self,
        item_id: ItemId,
        quantity: u16,
        sticky_type: ffi::gen_item_stickiness::Type,
        _ov29: OverlayLoadLease<29>,
    ) {
        unsafe { ffi::GenerateItem(self, item_id, quantity, sticky_type) }
    }

    /// Set the quantity code on an item (assuming it's Poké), given some maximum acceptable
    /// money amount.
    pub fn generate_item_quantity(&mut self, max_money: i32) {
        unsafe { ffi::GenerateMoneyQuantity(self, max_money) }
    }
}
