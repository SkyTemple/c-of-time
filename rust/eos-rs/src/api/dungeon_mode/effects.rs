use fixed::types::I24F8;
use crate::api::dungeon_mode::*;
use crate::api::objects::*;
use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::ctypes::*;
use crate::ffi;

/// Helper struct for emitting move and item effects.
/// 
/// You may find more things to do with monsters in the [`DungeonMonsterExtWrite`] trait.
/// 
pub struct DungeonEffectsEmitter(OverlayLoadLease<29>);

impl CreatableWithLease<29> for DungeonEffectsEmitter {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self(lease)
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self.0
    }
}

impl DungeonEffectsEmitter {
    /// Deals damage from a move or item used by an attacking monster on a defending monster.
    /// 
    /// Returns the amount of damage dealt.
    pub fn deal_damage(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        used_move: &mut Move,
        damage_multiplier: I24F8,
        item_id: Option<item_catalog::Type>
    ) -> i32 {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DealDamage(
            attacker as *mut _, defender as *mut _,
            used_move as *mut _, damage_multiplier.to_bits() as c_int,
            item_id.unwrap_or(item_catalog::ITEM_NOTHING)
        ) }
    }

    /// Deals damage from a move or item used by an attacking monster on a defending monster, and 
    /// also deals recoil damage to the attacker.
    /// 
    /// Returns whether or not damage was dealt.
    pub fn deal_damage_with_recoil(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        used_move: &mut Move,
        item_id: Option<item_catalog::Type>
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DealDamageWithRecoil(
            attacker as *mut _, defender as *mut _,
            used_move as *mut _, item_id.unwrap_or(item_catalog::ITEM_NOTHING)
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictSleepStatus(
            attacker as *mut _, defender as *mut _,
            number_turns, log_failure as ffi::bool_
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictNightmareStatus(
            attacker as *mut _, defender as *mut _,
            number_turns
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictNappingStatus(
            attacker as *mut _, defender as *mut _,
            number_turns
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        number_turns: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictYawningStatus(
            attacker as *mut _, defender as *mut _,
            number_turns
        ) }
    }

    /// Inflicts the Sleepless status condition on a target monster if possible.
    ///
    /// No status is returned.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_sleepless_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictSleeplessStatus(
            attacker as *mut _, defender as *mut _
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        param3: i32,
        number_turns: i32,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictPausedStatus(
            attacker as *mut _, defender as *mut _,
            param3, number_turns, log_failure as ffi::bool_,
            check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictInfatuatedStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_,
            check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        special_effect: bool,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictBurnStatus(
            attacker as *mut _, defender as *mut _,
            special_effect as ffi::bool_, log_failure as ffi::bool_,
            check_only as ffi::bool_
        ) > 0 }
    }

    /// Inflicts the Burn status condition on all team members if possible.
    pub fn try_inflict_burn_status_whole_team(&self, _global_dungeon_struct: &mut GlobalDungeonData) {
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictPoisonedStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictBadlyPoisonedStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
    }

    /// Inflicts the Frozen status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure.
    pub fn try_inflict_frozen_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictFrozenStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_,
        ) }
    }

    /// Inflicts the Constriction status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `animation_id` - animation ID
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure.
    pub fn try_inflict_constriction_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        animation_id: i32,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictConstrictionStatus(
            attacker as *mut _, defender as *mut _,
            animation_id,
            log_failure as ffi::bool_,
        ) }
    }

    /// Inflicts the Shadow Hold (AKA Immobilized) status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure.
    pub fn try_inflict_shadow_hold_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictShadowHoldStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_,
        ) }
    }

    /// Inflicts the Shadow Hold (AKA Immobilized) status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_ingrain_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictIngrainStatus(
            attacker as *mut _, defender as *mut _
        ) }
    }

    /// Inflicts the Wrapped status condition on a target monster if possible.
    ///
    /// This also gives the user the Wrap status (Wrapped around foe).
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_wrapped_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictWrappedStatus(
            attacker as *mut _, defender as *mut _
        ) }
    }

    /// Inflicts the Petrified status condition on a target monster if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_petrified_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictPetrifiedStatus(
            attacker as *mut _, defender as *mut _
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictCringeStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictParalysisStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictConfusedStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictCoweringStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictLeechSeedStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
    }

    /// Inflicts the Destiny Bond status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_inflict_destiny_bond_status(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictDestinyBond(
            attacker as *mut _, defender as *mut _
        ) }
    }
    
    /// Lowers the specified offensive stat on the target monster.
    /// 
    /// `param_5` and `param_6` are unknown.
    pub fn lower_offensive_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16,
        param_5: ffi::undefined,
        param_6: ffi::undefined,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::LowerOffensiveStat(
            attacker as *mut _, defender as *mut _,
            stat_idx, n_stages, param_5, param_6
        ) }
    }

    /// Lowers the specified defensive stat on the target monster.
    ///
    /// `param_5` and `param_6` are unknown.
    pub fn lower_defensive_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16,
        param_5: ffi::undefined,
        param_6: ffi::undefined,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::LowerDefensiveStat(
            attacker as *mut _, defender as *mut _,
            stat_idx, n_stages, param_5, param_6
        ) }
    }

    /// Boosts the specified offensive stat on the target monster.
    pub fn boost_offensive_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostOffensiveStat(
            attacker as *mut _, defender as *mut _,
            stat_idx, n_stages
        ) }
    }

    /// Boosts the specified defensive stat on the target monster.
    pub fn boost_defensive_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        n_stages: i16
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostDefensiveStat(
            attacker as *mut _, defender as *mut _,
            stat_idx, n_stages
        ) }
    }

    /// Applies a multiplier to the specified offensive stat on the target monster.
    ///
    /// This affects struct [`ffi::monster_stat_modifiers.offensive_multipliers`], for moves like
    /// Charm and Memento.
    /// 
    /// `param_5` is unknown.
    pub fn apply_offensive_stat_multiplier(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        multiplier: i32,
        param_5: ffi::undefined
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ApplyOffensiveStatMultiplier(
            attacker as *mut _, defender as *mut _,
            stat_idx, multiplier, param_5
        ) }
    }

    /// Applies a multiplier to the specified defensive stat on the target monster.
    ///
    /// This affects struct [`ffi::monster_stat_modifiers.defensive_multipliers`], for moves like
    /// Charm and Memento.
    ///
    /// `param_5` is unknown.
    pub fn apply_defensive_stat_multiplier(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        multiplier: i32,
        param_5: ffi::undefined
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ApplyDefensiveStatMultiplier(
            attacker as *mut _, defender as *mut _,
            stat_idx, multiplier, param_5
        ) }
    }

    /// Boosts the specified hit chance stat (accuracy or evasion) on the target monster.
    pub fn boost_hit_chance_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostHitChanceStat(
            attacker as *mut _, defender as *mut _,
            stat_idx
        ) }
    }

    /// Lowers the specified hit chance stat (accuracy or evasion) on the target monster.
    pub fn lower_hit_chance_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        param_4: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::LowerHitChanceStat(
            attacker as *mut _, defender as *mut _,
            stat_idx, param_4
        ) }
    }

    /// Resets the specified hit chance stat (accuracy or evasion) back to normal on the
    /// target monster.
    pub fn reset_hit_chance_stat(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        stat_idx: i32,
        param_4: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ResetHitChanceStat(
            attacker as *mut _, defender as *mut _,
            stat_idx, param_4
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        n_stages: i32,
        n_turns: i32,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostSpeed(
            attacker as *mut _, defender as *mut _,
            n_stages, n_turns, log_failure as ffi::bool_
        ) }
    }

    /// Lowers the speed of the target monster.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `n_stages` - The number of stages to boost the speed by.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn lower_speed(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        n_stages: i32,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::LowerSpeed(
            attacker as *mut _, defender as *mut _,
            n_stages, log_failure as ffi::bool_
        ) }
    }

    /// Randomly boosts or lowers the speed of the target monster by one stage with equal
    /// probability.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn boost_or_lower_speed(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::BoostOrLowerSpeed(
            attacker as *mut _, defender as *mut _
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TrySealMove(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_
        ) > 0 }
    }

    /// Activate the Quick Feet ability on the defender, if the monster has it and it's active.
    ///
    /// Returns Whether or not the ability was activated.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn try_activate_quick_feet(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryActivateQuickFeet(
            attacker as *mut _, defender as *mut _
        ) > 0 }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        hp_to_restore: i32,
        max_hp_boost: i32,
        log_failure: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryIncreaseHp(
            attacker as *mut _, defender as *mut _,
            hp_to_restore, max_hp_boost, log_failure as ffi::bool_
        ) > 0 }
    }

    /// Restores the PP of all the target's moves by the specified amount.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `pp_to_restore` - The amount of PP to restore.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    pub fn restore_move_pp(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        pp_to_restore: i32,
        log_failure: bool
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::RestoreMovePP(
            attacker as *mut _, defender as *mut _,
            pp_to_restore, (!log_failure) as ffi::bool_
        ) }
    }

    /// Apply an item effect.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `item` - The item that was used / thrown.
    pub fn apply_item_effect(
        &self,
        param_1: ffi::undefined4,
        param_2: ffi::undefined4,
        param_3: ffi::undefined4,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        item: &mut DungeonItem,
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ApplyItemEffect(
            param_1, param_2, param_3,
            attacker as *mut _, defender as *mut _,
            item as *mut _
        ) }
    }

    /// Applies the Violent Seed boost to an entity.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    pub fn violent_seed_boost(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ViolentSeedBoost(
            attacker as *mut _, defender as *mut _
        ) }
    }

    /// Applies the IQ and possible stat boosts from eating a Gummi to the target monster.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `gummi_type` - Type of the Gummi that was eaten.
    /// * `stat_boost` - The amount of stat boost to apply; if a random stat boost occurs.
    pub fn apply_gummi_boosts(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        gummi_type: type_catalog::Type,
        stat_boost: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::ApplyGummiBoosts(
            attacker as *mut _, defender as *mut _,
            gummi_type, stat_boost
        ) }
    }

    /// Applies the IQ and possible stat boosts from eating a Gummi to the target monster.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `direction` - The direction
    pub fn try_pounce(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        direction: Direction
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryPounce(
            attacker as *mut _, defender as *mut _,
            direction as ffi::direction_id::Type
        ) }
    }

    /// Blows away the target monster in a given direction if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `direction` - The direction
    pub fn try_blow_away(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        direction: Direction
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryBlowAway(
            attacker as *mut _, defender as *mut _,
            direction as ffi::direction_id::Type
        ) }
    }

    /// Makes the target monster warp if possible.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `warp_type` - The type of warp to apply.
    /// * `position` - The position to warp to (if warp type is position-based).
    pub fn try_warp(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        warp_type: WarpType,
        position: ffi::position
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryWarp(
            attacker as *mut _, defender as *mut _,
            warp_type as ffi::warp_type::Type,
            position
        ) }
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
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        base_exp: i32
    ) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::AddExpSpecial(
            attacker as *mut _, defender as *mut _,
            base_exp
        ) }
    }
}
