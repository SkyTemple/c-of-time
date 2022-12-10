use crate::api::dungeon_mode::*;
use crate::api::enums::{Direction, WarpType};
use crate::api::items::{Item, ItemId};
use crate::api::moves::Move;
use crate::api::overlay::OverlayLoadLease;
use crate::api::types::MonsterTypeId;
use crate::ctypes::*;
use crate::ffi;
use fixed::types::I24F8;

/// Helper struct for emitting move and item effects.
///
/// To get an instance of this, use [`GlobalDungeonData::effects`].
///
/// You may find more things to do with monsters in the [`DungeonMonsterMut`] struct.
pub struct DungeonEffectsEmitter<'a>(pub(crate) &'a OverlayLoadLease<29>);

impl<'a> DungeonEffectsEmitter<'a> {
    /// Low-level functions internal to the dungeon engine.
    /// Consider using one of the other functions instead for most cases.
    pub fn internals(&'a mut self) -> DungeonEffectsInternals<'a> {
        DungeonEffectsInternals(self)
    }

    /// Returns true if the target is within range of the user's move, false otherwise.
    ///
    /// If the user does not have Course Checker, it simply checks if the distance between user
    /// and target is less or equal than the move range.
    ///
    /// Otherwise, it will iterate through all tiles in the direction specified, checking for
    /// walls or other monsters in the way, and return false if they are found.
    ///
    /// `move_range` is the range in number of tiles.
    pub fn is_target_in_range(
        &self,
        user: &DungeonEntity,
        target: &DungeonEntity,
        direction: Direction,
        move_range: i32,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::IsTargetInRange(
                force_mut_ptr!(user),
                force_mut_ptr!(target),
                direction as ffi::direction_id::Type,
                move_range,
            ) > 0
        }
    }

    /// Deals damage from a move or item used by an attacking monster on a defending monster.
    ///
    /// Returns the amount of damage dealt.
    pub fn deal_damage(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        used_move: &Move,
        damage_multiplier: I24F8,
        item_id: Option<ItemId>,
    ) -> i32 {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DealDamage(
                attacker,
                defender,
                force_mut_ptr!(used_move),
                damage_multiplier.to_bits() as c_int,
                item_id.unwrap_or(ItemId::ITEM_NOTHING),
            )
        }
    }

    /// Move effect: Deals damage, inflicting recoil damage on the attacker.
    ///
    /// Relevant moves: Submission, Wood Hammer, Brave Bird
    ///
    /// Returns whether or not damage was dealt.
    pub fn do_move_damage_with_recoil(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        used_move: &Move,
        item_id: Option<ItemId>,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageWithRecoil(
                attacker,
                defender,
                force_mut_ptr!(used_move),
                item_id.unwrap_or(ItemId::ITEM_NOTHING),
            ) > 0
        }
    }

    /// Inflicts the Sleep status condition on a target monster if possible.
    ///
    /// No status is returned.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `number_turns` - How many turns the status should be applied for.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn try_inflict_sleep_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictSleepStatus(attacker, defender, number_turns, log_failure as ffi::bool_)
        }
    }

    /// Inflicts the Nightmare status condition on a target monster if possible.
    ///
    /// No status is returned.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `number_turns` - How many turns the status should be applied for.
    pub fn try_inflict_nightmare_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictNightmareStatus(attacker, defender, number_turns) }
    }

    /// Inflicts the Napping status condition on a target monster if possible.
    ///
    /// No status is returned.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `number_turns` - How many turns the status should be applied for.
    pub fn try_inflict_napping_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictNappingStatus(attacker, defender, number_turns) }
    }

    /// Inflicts the Yawning status condition on a target monster if possible.
    ///
    /// No status is returned.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `number_turns` - How many turns the status should be applied for.
    pub fn try_inflict_yawning_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictYawningStatus(attacker, defender, number_turns) }
    }

    /// Inflicts the Sleepless status condition on a target monster if possible.
    ///
    /// No status is returned.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_sleepless_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictSleeplessStatus(attacker, defender) }
    }

    /// Inflicts the Paused status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `param3` - Unknown.
    /// * `number_turns` - How many turns the status should be applied for.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_paused_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        param3: i32,
        number_turns: i32,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictPausedStatus(
                attacker,
                defender,
                param3,
                number_turns,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Infatuated status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_infatuated_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictInfatuatedStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Burn status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `special_effect` - Flag to apply some special effect alongside the burn?
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_burn_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        special_effect: bool,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictBurnStatus(
                attacker,
                defender,
                special_effect as ffi::bool_,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Burn status condition on all team members if possible.
    pub fn try_inflict_burn_status_whole_team(
        &mut self,
        _global_dungeon_struct: &mut GlobalDungeonData,
    ) {
        // SAFETY: We have a lease on the overlay existing & have a mutable reference to the global
        // dungeon data.
        unsafe { ffi::TryInflictBurnStatusWholeTeam() }
    }

    /// Inflicts the Poisoned status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_poisoned_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictPoisonedStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Badly Poisoned status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_badly_poisoned_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictBadlyPoisonedStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Frozen status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure.
    pub fn try_inflict_frozen_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictFrozenStatus(attacker, defender, log_failure as ffi::bool_) }
    }

    /// Inflicts the Constriction status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `animation_id` - animation ID
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure.
    pub fn try_inflict_constriction_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        animation_id: i32,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictConstrictionStatus(
                attacker,
                defender,
                animation_id,
                log_failure as ffi::bool_,
            )
        }
    }

    /// Inflicts the Shadow Hold (AKA Immobilized) status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure.
    pub fn try_inflict_shadow_hold_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictShadowHoldStatus(attacker, defender, log_failure as ffi::bool_) }
    }

    /// Inflicts the Shadow Hold (AKA Immobilized) status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_ingrain_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictIngrainStatus(attacker, defender) }
    }

    /// Inflicts the Wrapped status condition on a target monster if possible.
    ///
    /// This also gives the user the Wrap status (Wrapped around foe).
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_wrapped_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictWrappedStatus(attacker, defender) }
    }

    /// Inflicts the Petrified status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_petrified_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictPetrifiedStatus(attacker, defender) }
    }

    /// Inflicts the Cringe status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_cringe_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictCringeStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Paralysis status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_paralysis_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictParalysisStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Confused status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_confused_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictConfusedStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Cowering status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_cowering_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictCoweringStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Leech Seed status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `check_only` - Flag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_leech_seed_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryInflictLeechSeedStatus(
                attacker,
                defender,
                log_failure as ffi::bool_,
                check_only as ffi::bool_,
            ) > 0
        }
    }

    /// Inflicts the Destiny Bond status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_destiny_bond_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictDestinyBond(attacker, defender) }
    }

    /// Lowers the specified offensive stat on the target monster.
    ///
    /// `param_5` and `param_6` are unknown.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn lower_offensive_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16,
        param_5: ffi::undefined,
        param_6: ffi::undefined,
    ) {
        ffi::LowerOffensiveStat(attacker, defender, stat_idx, n_stages, param_5, param_6)
    }

    /// Lowers the specified defensive stat on the target monster.
    ///
    /// `param_5` and `param_6` are unknown.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn lower_defensive_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16,
        param_5: ffi::undefined,
        param_6: ffi::undefined,
    ) {
        ffi::LowerDefensiveStat(attacker, defender, stat_idx, n_stages, param_5, param_6)
    }

    /// Boosts the specified offensive stat on the target monster.
    pub fn boost_offensive_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostOffensiveStat(attacker, defender, stat_idx, n_stages) }
    }

    /// Boosts the specified defensive stat on the target monster.
    pub fn boost_defensive_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostDefensiveStat(attacker, defender, stat_idx, n_stages) }
    }

    /// Applies a multiplier to the specified offensive stat on the target monster.
    ///
    /// This affects struct [`ffi::monster_stat_modifiers.offensive_multipliers`], for moves like
    /// Charm and Memento.
    ///
    /// `param_5` is unknown.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn apply_offensive_stat_multiplier(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        multiplier: i32,
        param_5: ffi::undefined,
    ) {
        ffi::ApplyOffensiveStatMultiplier(attacker, defender, stat_idx, multiplier, param_5)
    }

    /// Applies a multiplier to the specified defensive stat on the target monster.
    ///
    /// This affects struct [`ffi::monster_stat_modifiers.defensive_multipliers`], for moves like
    /// Charm and Memento.
    ///
    /// `param_5` is unknown.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn apply_defensive_stat_multiplier(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        multiplier: i32,
        param_5: ffi::undefined,
    ) {
        ffi::ApplyDefensiveStatMultiplier(attacker, defender, stat_idx, multiplier, param_5)
    }

    /// Boosts the specified hit chance stat (accuracy or evasion) on the target monster.
    pub fn boost_hit_chance_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostHitChanceStat(attacker, defender, stat_idx) }
    }

    /// Lowers the specified hit chance stat (accuracy or evasion) on the target monster.
    pub fn lower_hit_chance_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        param_4: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::LowerHitChanceStat(attacker, defender, stat_idx, param_4) }
    }

    /// Resets the specified hit chance stat (accuracy or evasion) back to normal on the
    /// target monster.
    pub fn reset_hit_chance_stat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        param_4: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ResetHitChanceStat(attacker, defender, stat_idx, param_4) }
    }

    /// Boosts the speed of the target monster.
    ///
    /// If the number of turns specified is 0, a random turn count will be selected using the
    /// default SPEED_BOOST_DURATION_RANGE (see symbol table).
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `n_stages` - The number of stages to boost the speed by.
    /// * `n_turns` - The number of turns.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn boost_speed(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        n_stages: i32,
        n_turns: i32,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::BoostSpeed(
                attacker,
                defender,
                n_stages,
                n_turns,
                log_failure as ffi::bool_,
            )
        }
    }

    /// Lowers the speed of the target monster.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `n_stages` - The number of stages to boost the speed by.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn lower_speed(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        n_stages: i32,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::LowerSpeed(attacker, defender, n_stages, log_failure as ffi::bool_) }
    }

    /// Randomly boosts or lowers the speed of the target monster by one stage with equal
    /// probability.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn boost_or_lower_speed(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostOrLowerSpeed(attacker, defender) }
    }

    /// Lowers the speed of the target monster.
    ///
    /// Returns Whether or not a move was sealed.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn try_seal_move(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TrySealMove(attacker, defender, log_failure as ffi::bool_) > 0 }
    }

    /// Activate the Quick Feet ability on the defender, if the monster has it and it's active.
    ///
    /// Returns Whether or not the ability was activated.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_activate_quick_feet(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryActivateQuickFeet(attacker, defender) > 0 }
    }

    /// Restore HP and possibly boost max HP of the target monster if possible.
    ///
    /// Returns Whether or not HP was restored.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `hp_to_restore` - The amount of HP to restore.
    /// * `max_hp_boost` - The max HP boost to attempt to apply.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn try_increase_hp(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        hp_to_restore: i32,
        max_hp_boost: i32,
        log_failure: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryIncreaseHp(
                attacker,
                defender,
                hp_to_restore,
                max_hp_boost,
                log_failure as ffi::bool_,
            ) > 0
        }
    }

    /// Restores the PP of all the target's moves by the specified amount.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `pp_to_restore` - The amount of PP to restore.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn restore_move_pp(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        pp_to_restore: i32,
        log_failure: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::RestoreMovePP(
                attacker,
                defender,
                pp_to_restore,
                (!log_failure) as ffi::bool_,
            )
        }
    }

    /// Apply an item effect.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `item` - The item that was used / thrown.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn apply_item_effect(
        &mut self,
        param_1: ffi::undefined4,
        param_2: ffi::undefined4,
        param_3: ffi::undefined4,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        item: &mut Item,
    ) {
        ffi::ApplyItemEffect(param_1, param_2, param_3, attacker, defender, item)
    }

    /// Applies the Violent Seed boost to an entity.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn violent_seed_boost(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ViolentSeedBoost(attacker, defender) }
    }

    /// Applies the IQ and possible stat boosts from eating a Gummi to the target monster.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `gummi_type` - Type of the Gummi that was eaten.
    /// * `stat_boost` - The amount of stat boost to apply; if a random stat boost occurs.
    pub fn apply_gummi_boosts(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        gummi_type: MonsterTypeId,
        stat_boost: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ApplyGummiBoostsDungeonMode(attacker, defender, gummi_type, stat_boost) }
    }

    /// Applies the IQ and possible stat boosts from eating a Gummi to the target monster.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `direction` - The direction
    pub fn try_pounce(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        direction: Direction,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryPounce(attacker, defender, direction as ffi::direction_id::Type) }
    }

    /// Blows away the target monster in a given direction if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `direction` - The direction
    pub fn try_blow_away(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        direction: Direction,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryBlowAway(attacker, defender, direction as ffi::direction_id::Type) }
    }

    /// Makes the target monster warp if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `warp_type` - The type of warp to apply.
    /// * `position` - The position to warp to (if warp type is position-based).
    pub fn try_warp(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        warp_type: WarpType,
        position: ffi::position,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::TryWarp(
                attacker,
                defender,
                warp_type as ffi::warp_type::Type,
                position,
            )
        }
    }

    /// Move effect: Deal damage.
    /// Relevant moves: Many!
    ///
    /// This just wraps DealDamage with a multiplier of 1 (i.e., the fixed-point number 0x100).
    ///
    /// Returns whether or not damage was dealt
    pub fn do_move_damage(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDamage(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Iron Tail
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_iron_tail(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveIronTail(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Yawn
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_yawn(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveYawn(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Nightmare
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_nightmare(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveNightmare(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Charm
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_charm(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveCharm(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Encore
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_encore(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveEncore(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Super Fang
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_super_fang(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSuperFang(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Pain Split
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_pain_split(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePainSplit(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Torment
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_torment(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTorment(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Swagger
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_swagger(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSwagger(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 30% chance (ROCK_SLIDE_CRINGE_CHANCE) of inflicting the cringe status on the defender.
    /// Relevant moves: Rock Slide, Iron Head, Air Slash, Zen Headbutt, Dragon Rush
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_cringe_30(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageCringe30(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Whirlpool
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_whirlpool(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveWhirlpool(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Fake Tears
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_fake_tears(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFakeTears(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Spite
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_spite(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSpite(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Smokescreen
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_smokescreen(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSmokescreen(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Flatter
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_flatter(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFlatter(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Will-O-Wisp
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_will_o_wisp(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveWillOWisp(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Return
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_return(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveReturn(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Flame Wheel
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_flame_wheel(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFlameWheel(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Gust
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_gust(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveGust(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Paralyze the defender if possible
    /// Relevant moves: Disable, Stun Spore, Glare
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_paralyze(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveParalyze(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 20% chance (CRUNCH_LOWER_DEFENSE_CHANCE) of lowering the defender's defense.
    /// Relevant moves: Crunch, Shadow Ball
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_lower_def_20(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageLowerDef20(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Bite
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_bite(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBite(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 20% chance (THUNDER_PARALYZE_CHANCE) of paralyzing the defender.
    /// Relevant moves: Thunder, Force Palm
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_paralyze_20(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageParalyze20(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Endeavor
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_endeavor(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveEndeavor(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Facade
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_facade(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFacade(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 20% chance (CONSTRICT_LOWER_SPEED_CHANCE) of lowering the defender's speed.
    /// Relevant moves: Constrict, Bubblebeam
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_lower_speed_20(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageLowerSpeed20(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Brick Break
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_brick_break(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBrickBreak(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Rock Tomb
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_rock_tomb(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveRockTomb(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal draining damage, healing the attacker by a proportion of the damage dealt.
    /// Relevant moves: Giga Drain, Drain Punch
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_drain(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDamageDrain(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Reversal
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_reversal(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveReversal(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: SmellingSalt
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_smelling_salt(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveSmellingSalt(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Metal Sound
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_metal_sound(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveMetalSound(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Tickle
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_tickle(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTickle(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Outrage
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_outrage(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveOutrage(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage, multiplied by a weight-dependent factor.
    /// Relevant moves: Low Kick, Grass Knot
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_weight_dependent(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageWeightDependent(attacker, defender, force_mut_ptr!(the_move), item_id)
                > 0
        }
    }

    /// Move effect: AncientPower
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_ancient_power(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveAncientPower(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Rapid Spin
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_rapid_spin(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveRapidSpin(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 15% chance (BLIZZARD_FREEZE_CHANCE) of freezing the defender.
    /// Relevant moves: Blizzard, Ice Beam
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_freeze_15(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageFreeze15(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Scary Face
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_scary_face(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveScaryFace(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Rock Climb
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_rock_climb(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveRockClimb(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Earthquake
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_earthquake(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveEarthquake(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Gets the nature power variant for the current dungeon, based on the tileset ID.
    ///
    /// Returns whether or not the move was successfully used.
    pub fn get_nature_power_variant(&self) -> ffi::nature_power_variant::Type {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::GetNaturePowerVariant() }
    }

    /// Move effect: Nature Power
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_nature_power(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveNaturePower(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Lick
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_lick(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveLick(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Fissure
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_fissure(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFissure(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Extrasensory
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_extrasensory(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveExtrasensory(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Absorb
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_absorb(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveAbsorb(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Skill Swap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_skill_swap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSkillSwap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Headbutt
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_headbutt(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveHeadbutt(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Double-Edge
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_double_edge(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDoubleEdge(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Sand-Attack
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_sand_attack(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSandAttack(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 40% chance (SMOG_POISON_CHANCE) of poisoning the defender.
    /// Relevant moves: Smog, Poison Jab, Cross Poison
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_poison_40(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamagePoison40(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Sacred Fire
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_sacred_fire(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSacredFire(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Sheer Cold
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_sheer_cold(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSheerCold(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 40% chance (MUDDY_WATER_LOWER_ACCURACY_CHANCE) of lowering
    ///              the defender's accuracy.
    /// Relevant moves: Muddy Water, Mud Bomb, Mirror Shot
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_lower_accuracy_40(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageLowerAccuracy40(attacker, defender, force_mut_ptr!(the_move), item_id)
                > 0
        }
    }

    /// Move effect: Twister
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_twister(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTwister(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Twineedle
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_twineedle(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTwineedle(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Seismic Toss
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_seismic_toss(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSeismicToss(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Supersonic
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_supersonic(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSupersonic(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Taunt
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_taunt(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTaunt(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Horn Drill
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_horn_drill(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveHornDrill(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// This is identical to [`Self::do_move_lick`],
    /// except it uses a different data symbol for the paralysis chance (but it's still 10%).
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_thundershock(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveThundershock(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Thunder Wave
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_thunder_wave(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveThunderWave(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Block
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_block(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBlock(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Poison Gas
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_poison_gas(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePoisonGas(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Toxic
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_toxic(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveToxic(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Poison Fang
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_poison_fang(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePoisonFang(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Poison Sting
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_poison_sting(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePoisonSting(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Tri Attack
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_tri_attack(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTriAttack(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Swaps the held items of the attacker and defender.
    /// Relevant moves: Trick, Switcheroo
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_swap_items(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSwapItems(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Triple Kick
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_triple_kick(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTripleKick(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Mud-Slap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_mud_slap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveMudSlap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Thief
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_thief(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveThief(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Role Play
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_role_play(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveRolePlay(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Leer
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_leer(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveLeer(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Fake Out
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_fake_out(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFakeOut(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Pay Day
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_pay_day(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePayDay(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Curse
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_curse(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveCurse(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Superpower
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_superpower(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSuperpower(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: DynamicPunch
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_dynamic_punch(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDynamicPunch(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Knock Off
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_knock_off(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveKnockOff(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Secret Power
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_secret_power(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveSecretPower(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Dizzy Punch
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_dizzy_punch(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDizzyPunch(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Imprison
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_imprison(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveImprison(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: FeatherDance
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_feather_dance(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveFeatherDance(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Beat Up
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_beat_up(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBeatUp(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Blast Burn
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_blast_burn(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBlastBurn(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Crush Claw
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_crush_claw(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveCrushClaw(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Blaze Kick
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_blaze_kick(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBlazeKick(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Present
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_present(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePresent(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Eruption
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_eruption(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveEruption(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Poison Tail
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_poison_tail(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePoisonTail(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Roar
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_roar(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveRoar(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 10% (WHIRLPOOL_CONSTRICT_CHANCE) chance to constrict,
    ///              and with a damage multiplier dependent on the move used.
    /// Relevant moves: Clamp, Bind, Fire Spin, Magma Storm
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_constrict_10(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageConstrict10(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Wrap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_wrap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveWrap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Magnitude
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_magnitude(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveMagnitude(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Mist Ball
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_mist_ball(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveMistBall(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Destiny Bond
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_destiny_bond(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDestinyBond(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Hidden Power
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_hidden_power(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveHiddenPower(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Attract
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_attract(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveAttract(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: The attacker uses the move last used by enemy it's facing.
    /// Relevant moves: Mimic, Copycat
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_copycat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveCopycat(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Frustration
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_frustration(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFrustration(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Leech Seed
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_leech_seed(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveLeechSeed(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Dream Eater
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_dream_eater(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDreamEater(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Dragon Rage
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_dragon_dance(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDragonRage(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage with a 50% (LUSTER_PURGE_LOWER_SPECIAL_DEFENSE_CHANCE) chance
    ///              to lower special defense.
    /// Relevant moves: Luster Purge, Energy Ball
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_lower_special_defence_50(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageLowerSpecialDefense50(
                attacker,
                defender,
                force_mut_ptr!(the_move),
                item_id,
            ) > 0
        }
    }

    /// Move effect: Fling
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_fling(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFling(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Hammer Arm
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_hammer_arm(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoHammerArm(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Gastro Acid
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_gastro_acid(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveGastroAcid(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Close Combat
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_close_combat(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveCloseCombat(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Guard Swap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_guard_swap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveGuardSwap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Thunder Fang
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_thunder_fang(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveThunderFang(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Defog
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_defog(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveDefog(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Trump Card
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_trump_card(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveTrumpCard(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Ice Fang
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_ice_fang(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveIceFang(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Psycho Shift
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_psycho_shift(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePsychoShift(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Embargo
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_embargo(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveEmbargo(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage, with a 2x multiplier if the defender is at or below half HP.
    /// Relevant moves: Brine, Assurance
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_brine(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveBrine(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Natural Gift
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_natural_gift(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveNaturalGift(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Gyro Ball
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_gyro_ball(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveGyroBall(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Charge Beam
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_charge_beam(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveChargeBeam(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deals damage, and eats any beneficial items the defender is holding.
    /// Relevant moves: Pluck, Bug Bite
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_eat_item(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageEatItem(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Last Resort
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_last_resort(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveLastResort(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Deal damage, with a multiplier dependent on the defender's current HP.
    /// Relevant moves: Wring Out, Crush Grip
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_damage_hp_dependent(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::DoMoveDamageHpDependent(attacker, defender, force_mut_ptr!(the_move), item_id) > 0
        }
    }

    /// Move effect: Heart Swap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_heart_swap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveHeartSwap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Power Swap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_power_swap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMovePowerSwap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Feint
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_feint(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFeint(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Flare Blitz
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_flare_blitz(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFlareBlitz(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Fire Fang
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_fire_fang(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveFireFang(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Miracle Eye
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_miracle_eye(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveMiracleEye(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Wake-Up Slap
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_wake_up_slap(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveWakeUpSlap(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Head Smash
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_head_smash(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveHeadSmash(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Move effect: Captivate
    ///
    /// Returns whether or not the move was successfully used.
    pub fn do_move_captivate(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        item_id: ItemId,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DoMoveCaptivate(attacker, defender, force_mut_ptr!(the_move), item_id) > 0 }
    }

    /// Adds to a monster's experience points, subject to experience boosting effects.
    ///
    /// This function appears to be called only under special circumstances. Possibly when granting
    /// experience from damage (e.g., Joy Ribbon)?
    ///
    /// Interestingly, the `attacker` isn't actually used. This might be a compiler
    /// optimization to avoid shuffling registers, since this function might be called alongside
    /// lots of other functions that have both the attacker and defender as the first two arguments.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `base_exp` - base experience gain, before boosts.
    pub fn add_exp_special(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        base_exp: i32,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::AddExpSpecial(attacker, defender, base_exp) }
    }

    /// The user entity attempts to switch places with the target entity (i.e. by the effect of the
    /// Switcher Orb).
    ///
    /// The function checks for the Suction Cups ability for both the user and the target, and for
    /// the Mold Breaker ability on the user.
    pub fn try_switch_place(&mut self, attacker: &mut DungeonEntity, defender: &mut DungeonEntity) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TrySwitchPlace(attacker, defender) }
    }

    /// Cures the target's freeze, shadow hold, ingrain, petrified, constriction or wrap (both as
    /// user and as target) status due to the action of the user.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to cure this status.
    /// * `defender` - The monster that is being cured from this status.
    /// * `log` - Flag to log a message to the dungeon message log.
    pub fn end_frozen_class_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log: bool,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::EndFrozenClassStatus(attacker, defender, log as ffi::bool_) }
    }

    /// Cures the target's cringe, confusion, cowering, pause, taunt, encore or infatuated status
    /// due to the action of the user, and prints the event to the log.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to cure this status.
    /// * `defender` - The monster that is being cured from this status.
    pub fn end_cringe_class_status(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::EndCringeClassStatus(attacker, defender) }
    }

    /// Frees from the wrap status all monsters which are wrapped by/around the monster passed as
    /// parameter.
    pub fn free_other_wrapped_monsters(&mut self, target: &mut DungeonEntity) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::FreeOtherWrappedMonsters(target) }
    }
}

/// Internal functions for battle effect calculations.
pub struct DungeonEffectsInternals<'a>(&'a mut DungeonEffectsEmitter<'a>);

impl<'a> DungeonEffectsInternals<'a> {
    /// Applies damage to a monster. Displays the damage animation, lowers its health and handles
    /// reviving if applicable.
    ///
    /// The EU version has some additional checks related to printing fainting messages under
    /// specific circumstances.
    ///
    /// Returns true if the target fainted (reviving does not count as fainting).
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `damage_data` - Mutable reference to the damage_data struct that contains info about the
    ////           damage to deal
    /// * `param_4` - ?
    /// * `param_5` - ?
    /// * `param_6` - ?
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn apply_damage(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        damage_data: &mut ffi::damage_data,
        param_4: ffi::undefined4,
        param_5: *mut ffi::undefined4,
        faint_reason: ffi::faint_reason,
    ) -> bool {
        ffi::ApplyDamage(
            attacker,
            defender,
            damage_data,
            param_4,
            param_5,
            faint_reason,
        ) > 0
    }

    /// Determine what item a defeated enemy should drop, if any, then (probably?) spawn that
    /// item underneath them.
    ///
    /// This function is called at the time when an enemy is defeated from [`Self::apply_damage`].
    pub fn try_spawn_enemy_item_drop(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TrySpawnEnemyItemDrop(attacker, defender) }
    }

    /// Determines if a move used hits or misses the target.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `the_move` - Reference to move data
    /// * `use_second_accuracy` - True if the move's first accuracy (accuracy1) should be used, false
    ////                   if its second accuracy (accuracy2) should be used instead.
    pub fn move_hit_check(
        &mut self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        the_move: &Move,
        use_second_accuracy: bool,
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            ffi::MoveHitCheck(
                attacker,
                defender,
                force_mut_ptr!(the_move),
                use_second_accuracy as ffi::bool_,
            ) > 0
        }
    }

    /// Handles the effects that happen after a move is used. Includes a loop that is run for
    /// each target, multiple ability checks and the giant switch statement that executes the
    /// effect of the move used given its ID.
    ///
    /// # Arguments
    /// * `param_1` - pointer to some struct
    /// * `attacker` - attacker pointer
    /// * `the_move` - pointer to move data
    /// * `param_4` - ?
    /// * `param_5` - ?
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn execute_move_effect(
        &mut self,
        param_1: *mut ffi::undefined4,
        attacker: &mut DungeonEntity,
        the_move: &Move,
        param_4: ffi::undefined4,
        param_5: ffi::undefined4,
    ) {
        ffi::ExecuteMoveEffect(
            param_1,
            attacker,
            force_mut_ptr!(the_move),
            param_4,
            param_5,
        )
    }
}
