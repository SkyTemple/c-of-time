use crate::api::items::ItemId;
use crate::api::moves::MoveId;
use crate::ffi;

pub fn get_faint_reason(move_id: MoveId, item_id: ItemId) -> ffi::faint_reason {
    unsafe { ffi::GetFaintReason(move_id, item_id) }
}
