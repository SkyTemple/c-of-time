//! Functions related to getting information about monster moves.

use crate::api::dungeon_mode::MoveCategory;
use crate::api::objects::Move;
use crate::ffi;

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
    Front2WithCornerCutting = ffi::move_range::RANGE_FRONT_2_WITH_CORNER_CUTTING,
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
            ffi::move_range::RANGE_FRONT_WITH_CORNER_CUTTING => {
                Ok(MoveRange::FrontWithCornerCutting)
            }
            ffi::move_range::RANGE_FRONT_2_WITH_CORNER_CUTTING => {
                Ok(MoveRange::Front2WithCornerCutting)
            }
            ffi::move_range::RANGE_SPECIAL => Ok(MoveRange::Special),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Conditions checked by the AI to determine when a move should be used.
/// It does not affect how the move works.
pub enum MoveAiCondition {
    None = ffi::move_ai_condition::AI_CONDITION_NONE,
    /// The AI will consider a target elegible wirh a chance equal to the
    /// move's "ai_condition_random_chance" value.
    Random = ffi::move_ai_condition::AI_CONDITION_RANDOM,
    /// Target has HP <= 25%
    Hp25 = ffi::move_ai_condition::AI_CONDITION_HP_25,
    /// Target has a negative status condition
    Status = ffi::move_ai_condition::AI_CONDITION_STATUS,
    /// Target is asleep, napping or in a nightmare
    Asleep = ffi::move_ai_condition::AI_CONDITION_ASLEEP,
    /// Target is ghost-type and not exposed
    Ghost = ffi::move_ai_condition::AI_CONDITION_GHOST,
    /// Target has HP <= 25% or a negative status condition
    Hp25OrStatus = ffi::move_ai_condition::AI_CONDITION_HP_25_OR_STATUS,
}

impl TryInto<MoveAiCondition> for ffi::move_ai_condition::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveAiCondition, Self::Error> {
        match self {
            ffi::move_ai_condition::AI_CONDITION_NONE => Ok(MoveAiCondition::None),
            ffi::move_ai_condition::AI_CONDITION_RANDOM => Ok(MoveAiCondition::Random),
            ffi::move_ai_condition::AI_CONDITION_HP_25 => Ok(MoveAiCondition::Hp25),
            ffi::move_ai_condition::AI_CONDITION_STATUS => Ok(MoveAiCondition::Status),
            ffi::move_ai_condition::AI_CONDITION_ASLEEP => Ok(MoveAiCondition::Asleep),
            ffi::move_ai_condition::AI_CONDITION_GHOST => Ok(MoveAiCondition::Ghost),
            ffi::move_ai_condition::AI_CONDITION_HP_25_OR_STATUS => {
                Ok(MoveAiCondition::Hp25OrStatus)
            }
            _ => Err(()),
        }
    }
}

/// Range, target and AI data for a move.
/// Values are None, if they are invalid / non-standard.
pub struct MoveTargetAndRange {
    pub target: Option<MoveTarget>,
    pub range: Option<MoveRange>,
    pub ai_condition: Option<MoveAiCondition>,
    pub unused: u16,
}

impl From<ffi::move_target_and_range> for MoveTargetAndRange {
    fn from(tr: ffi::move_target_and_range) -> Self {
        MoveTargetAndRange {
            target: tr.target().try_into().ok(),
            range: tr.range().try_into().ok(),
            ai_condition: tr.ai_condition().try_into().ok(),
            unused: tr.unused(),
        }
    }
}

/// Will fail, if any values are None in MoveTargetAndRange.
impl TryFrom<MoveTargetAndRange> for ffi::move_target_and_range {
    type Error = ();

    fn try_from(value: MoveTargetAndRange) -> Result<Self, Self::Error> {
        if value.target.is_none() || value.range.is_none() || value.ai_condition.is_none() {
            return Err(());
        }
        Ok(ffi::move_target_and_range {
            _bitfield_align_1: [],
            _bitfield_1: ffi::move_target_and_range::new_bitfield_1(
                value.target.unwrap() as ffi::move_target::Type,
                value.range.unwrap() as ffi::move_range::Type,
                value.ai_condition.unwrap() as ffi::move_ai_condition::Type,
                value.unused,
            ),
        })
    }
}

/// Game functions related to [`Move`]s.
pub trait MoveExt {
    /// Gets the move target-and-range field. See struct move_target_and_range in the C headers.
    fn get_target_and_range(&self, is_ai: bool) -> MoveTargetAndRange;

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
    fn get_target_and_range(&self, is_ai: bool) -> MoveTargetAndRange {
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
        unsafe { ffi::GetMoveCategory(self.id.val()) }
            .try_into()
            .ok()
    }
}
