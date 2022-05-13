//! Functions related to getting information about monster moves.

use crate::api::dungeon_mode::MoveCategory;
use crate::api::objects::{Move};
use crate::ffi;
use crate::ffi::move_target_and_range;

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Move target (i.e., who does a move affect when used?).
pub enum MoveTarget {
    Enemies = ffi::move_target::TARGET_ENEMIES,
    Party = ffi::move_target::TARGET_PARTY,
    All = ffi::move_target::TARGET_ALL,
    User = ffi::move_target::TARGET_USER,
    EnemiesAfterCharging = ffi::move_target::TARGET_ENEMIES_AFTER_CHARGING,
    AllExceptUser = ffi::move_target::TARGET_ALL_EXCEPT_USER,
    Teammates = ffi::move_target::TARGET_TEAMMATES,
    Special = ffi::move_target::TARGET_SPECIAL,
}

impl TryInto<MoveTarget> for ffi::move_target::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveTarget, Self::Error> {
        match self {
            ffi::move_target::TARGET_ENEMIES => Ok(MoveTarget::Enemies),
            ffi::move_target::TARGET_PARTY => Ok(MoveTarget::Party),
            ffi::move_target::TARGET_ALL => Ok(MoveTarget::All),
            ffi::move_target::TARGET_USER => Ok(MoveTarget::User),
            ffi::move_target::TARGET_ENEMIES_AFTER_CHARGING => Ok(MoveTarget::EnemiesAfterCharging),
            ffi::move_target::TARGET_ALL_EXCEPT_USER => Ok(MoveTarget::AllExceptUser),
            ffi::move_target::TARGET_TEAMMATES => Ok(MoveTarget::Teammates),
            ffi::move_target::TARGET_SPECIAL => Ok(MoveTarget::Special),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Move range.
pub enum MoveRange {
    Front = ffi::move_range::RANGE_FRONT,
    FrontAndSides = ffi::move_range::RANGE_FRONT_AND_SIDES,
    Nearby = ffi::move_range::RANGE_NEARBY,
    Room = ffi::move_range::RANGE_ROOM,
    Front2 = ffi::move_range::RANGE_FRONT_2,
    Front10 = ffi::move_range::RANGE_FRONT_10,
    Floor = ffi::move_range::RANGE_FLOOR,
    User = ffi::move_range::RANGE_USER,
    FrontWithCornerCutting = ffi::move_range::RANGE_FRONT_WITH_CORNER_CUTTING,
    IceShard = ffi::move_range::RANGE_ICE_SHARD,
    Special = ffi::move_range::RANGE_SPECIAL,
}

impl TryInto<MoveRange> for ffi::move_range::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveRange, Self::Error> {
        match self {
            ffi::move_range::RANGE_FRONT => Ok(MoveRange::Front),
            ffi::move_range::RANGE_FRONT_AND_SIDES => Ok(MoveRange::FrontAndSides),
            ffi::move_range::RANGE_NEARBY => Ok(MoveRange::Nearby),
            ffi::move_range::RANGE_ROOM => Ok(MoveRange::Room),
            ffi::move_range::RANGE_FRONT_2 => Ok(MoveRange::Front2),
            ffi::move_range::RANGE_FRONT_10 => Ok(MoveRange::Front10),
            ffi::move_range::RANGE_FLOOR => Ok(MoveRange::Floor),
            ffi::move_range::RANGE_USER => Ok(MoveRange::User),
            ffi::move_range::RANGE_FRONT_WITH_CORNER_CUTTING => Ok(MoveRange::FrontWithCornerCutting),
            ffi::move_range::RANGE_ICE_SHARD => Ok(MoveRange::IceShard),
            ffi::move_range::RANGE_SPECIAL => Ok(MoveRange::Special),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Seems to be used to differentiate certain healing moves.
/// This might also be a bitfield rather than an enum?
pub enum HealingMoveType {
    Normal = ffi::healing_move_type::HEALING_MOVE_NORMAL,
    /// For Softboiled, Moonlight, Milk Drink, Synthesis, Swallow, Heal Order, and Roost
    Special = ffi::healing_move_type::HEALING_MOVE_SPECIAL,
    /// For Healing Wish and Lunar Dance
    Faint = ffi::healing_move_type::HEALING_MOVE_FAINT,
}

impl TryInto<HealingMoveType> for ffi::healing_move_type::Type {
    type Error = ();

    fn try_into(self) -> Result<HealingMoveType, Self::Error> {
        match self {
            ffi::healing_move_type::HEALING_MOVE_NORMAL => Ok(HealingMoveType::Normal),
            ffi::healing_move_type::HEALING_MOVE_SPECIAL => Ok(HealingMoveType::Special),
            ffi::healing_move_type::HEALING_MOVE_FAINT => Ok(HealingMoveType::Faint),
            _ => Err(()),
        }
    }
}

/// Game functions related to [`Move`]s.
pub trait MoveExt {
    /// Gets the move target-and-range field. See struct move_target_and_range in the C headers.
    fn get_target_and_range(&self, is_ai: bool) -> (Option<MoveTarget>, Option<MoveRange>, Option<HealingMoveType>, u16);

    /// Gets the base power of the move.
    fn get_base_power(&self) -> i32;

    /// Gets the maximum PP for the move.
    ///
    /// Returns max PP for the given move, capped at 99.
    fn get_max_pp(&self) -> i32;

    /// Gets the critical hit chance of the move.
    fn get_crit_chance(&self) -> i32;

    /// Checks if the move is a recoil move (affected by Reckless).
    fn is_recoil_move(&self) -> bool;

    /// Checks if the move is a punch move (affected by Iron Fist).
    fn is_punch_move(&self) -> bool;

    /// Gets a move's category (physical, special, status). Returns None if the catgeory is invalid.
    fn get_category(&self) -> Option<MoveCategory>;
}

impl MoveExt for Move {
    fn get_target_and_range(&self, is_ai: bool) -> (Option<MoveTarget>, Option<MoveRange>, Option<HealingMoveType>, u16) {
        unsafe { ffi::GetMoveTargetAndRange(force_mut_ptr!(self), is_ai as ffi::bool_) }.into()
    }

    fn get_base_power(&self) -> i32 {
        unsafe { ffi::GetMoveBasePower(force_mut_ptr!(self)) }
    }

    fn get_max_pp(&self) -> i32 {
        unsafe { ffi::GetMaxPp(force_mut_ptr!(self)) }
    }

    fn get_crit_chance(&self) -> i32 {
        unsafe { ffi::GetMoveCritChance(force_mut_ptr!(self)) }
    }

    fn is_recoil_move(&self) -> bool {
        unsafe { ffi::IsRecoilMove(self.id.val()) > 0 }
    }

    fn is_punch_move(&self) -> bool {
        unsafe { ffi::IsPunchMove(self.id.val()) > 0 }
    }

    fn get_category(&self) -> Option<MoveCategory> {
        unsafe { ffi::GetMoveCategory(self.id.val()) }.try_into().ok()
    }
}

impl From<move_target_and_range> for (Option<MoveTarget>, Option<MoveRange>, Option<HealingMoveType>, u16) {
    fn from(tr: move_target_and_range) -> Self {
        (
            tr.target().try_into().ok(),
            tr.range().try_into().ok(),
            tr.type_().try_into().ok(),
            tr.unused(),
        )
    }
}
