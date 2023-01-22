use crate::api::items::ItemId;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Gets the struct `ffi::bar_item` from `ffi::BAR_AVAILABLE_ITEMS` with the specified item ID.
pub fn get_bar_item<'ol, 'a>(
    item_id: ItemId,
    _ov19: &'ol OverlayLoadLease<19>,
) -> &'a mut ffi::bar_item
where
    'a: 'ol,
{
    unsafe { &mut *ffi::GetBarItem(item_id) }
}

/// No description available.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_recruitable_monster_all(_ov19: &OverlayLoadLease<19>) -> i32 {
    unsafe { ffi::GetRecruitableMonsterAll() }
}

/// No description available.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_recruitable_monster_list(_ov19: &OverlayLoadLease<19>) -> i32 {
    unsafe { ffi::GetRecruitableMonsterList() }
}

/// No description available.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_recruitable_monster_list_restricted(_ov19: &OverlayLoadLease<19>) -> i32 {
    unsafe { ffi::GetRecruitableMonsterListRestricted() }
}
