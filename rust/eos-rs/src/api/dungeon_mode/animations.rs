use crate::api::dungeon_mode::traps::TrapId;
use crate::api::items::ItemId;
use crate::api::moves::MoveId;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Get an effect animation by effect ID.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_effect_animation(anim_id: i32, _ov10: &OverlayLoadLease<10>) -> &ffi::effect_animation {
    unsafe { &*ffi::GetEffectAnimation(anim_id) }
}

/// Get the move animation corresponding to the given move ID.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_move_animation(move_id: MoveId, _ov10: &OverlayLoadLease<10>) -> &ffi::move_animation {
    unsafe { &*ffi::GetMoveAnimation(move_id) }
}

/// Get the special move animation corresponding to the entity ID.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_special_monster_move_animation(
    ent_id: i32,
    _ov10: &OverlayLoadLease<10>,
) -> &ffi::special_monster_move_animation {
    unsafe { &*ffi::GetSpecialMonsterMoveAnimation(ent_id) }
}

/// Get the trap animation corresponding to the trap ID.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_trap_animation(trap_id: TrapId, _ov10: &OverlayLoadLease<10>) -> i16 {
    unsafe { ffi::GetTrapAnimation(trap_id) }
}

/// Get the item animation corresponding to the item ID.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_item_animation1(item_id: ItemId, _ov10: &OverlayLoadLease<10>) -> i16 {
    unsafe { ffi::GetItemAnimation1(item_id) }
}

/// Get the item animation corresponding to the item ID.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_item_animation2(item_id: ItemId, _ov10: &OverlayLoadLease<10>) -> i16 {
    unsafe { ffi::GetItemAnimation2(item_id) }
}

/// Get the animation speed for a move.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn get_move_animation_speed(move_id: MoveId, _ov10: &OverlayLoadLease<10>) -> i32 {
    unsafe { ffi::GetMoveAnimationSpeed(move_id) }
}
