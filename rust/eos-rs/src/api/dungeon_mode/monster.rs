use crate::api::abilities::AbilityId;
use crate::api::dungeon_mode::*;
use crate::api::iq::IqSkillId;
use crate::api::items::{ExclusiveItemEffectId, ItemId};
use crate::api::monsters::MonsterSpeciesId;
use crate::api::moves::*;
use crate::api::types::MonsterTypeId;
use core::ops::{Deref, DerefMut};
use fixed::types::I24F8;

/// Reference type for the info struct for [`DungeonEntity`] objects that are monsters.
///
/// For methods, see the [`DungeonMonsterRead`] trait.
pub struct DungeonMonsterRef<'a>(pub &'a ffi::monster, pub &'a DungeonEntity);

/// Mutable reference type for the info struct for [`DungeonEntity`] objects that are monsters.
///
/// For methods, see the [`DungeonMonsterRead`] and [`DungeonMonsterWrite`] traits.
pub struct DungeonMonsterMut<'a>(pub &'a mut ffi::monster, pub &'a mut DungeonEntity);

impl<'a> Deref for DungeonMonsterRef<'a> {
    type Target = ffi::monster;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> Deref for DungeonMonsterMut<'a> {
    type Target = ffi::monster;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> DerefMut for DungeonMonsterMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// Trait for [`DungeonMonsterRef`] and [`DungeonMonsterMut`] (read operations).
///
/// # Important safety note
/// Please see the safety note of [`DungeonEntity`]. It also applies to this trait.
pub trait DungeonMonsterRead: private::Sealed {
    #[doc(hidden)]
    fn entity(&self) -> &DungeonEntity;
    #[doc(hidden)]
    fn monster(&self) -> &ffi::monster;

    /// Checks if the monster is a special story ally.
    /// This is a hard-coded check that looks at the monster's "Joined At" field.
    /// If the value is in the range [
    /// [`crate::api::dungeons::DungeonId::DUNGEON_JOINED_AT_BIDOOF`],
    /// [`crate::api::dungeons::DungeonId::DUNGEON_DUMMY_0xE3`]
    /// ], this check will return true.
    fn is_special_story_ally(&self) -> bool {
        unsafe { ffi::IsSpecialStoryAlly(force_mut_ptr!(self.monster())) > 0 }
    }

    /// Checks if the monster does not gain experience.
    /// This basically just inverts IsSpecialStoryAlly, with the exception of also checking for
    /// the "Joined At" field being [`crate::api::dungeons::DungeonId::DUNGEON_CLIENT`].
    fn is_experience_locked(&self) -> bool {
        unsafe { ffi::IsExperienceLocked(force_mut_ptr!(self.monster())) > 0 }
    }

    /// Checks if the monster is holding a certain item that isn't disabled by Klutz.
    fn is_holding_item(&self, item_id: ItemId) -> bool {
        unsafe { ffi::ItemIsActive(force_mut_ptr!(self.entity()), item_id) > 0 }
    }

    /// Checks if the monster is at low health (below 25% rounded down).
    fn has_low_health(&self) -> bool {
        unsafe { ffi::HasLowHealth(force_mut_ptr!(self.entity())) > 0 }
    }

    /// Checks if the monster has the Gastro Acid status.
    fn gastro_acid_status(&self) -> bool {
        unsafe { ffi::NoGastroAcidStatus(force_mut_ptr!(self.monster())) == 0 }
    }

    // Checks if the monster has a certain ability that isn't disabled by Gastro Acid.
    fn is_ability_active(&self, ability_id: AbilityId) -> bool {
        unsafe { ffi::AbilityIsActive(force_mut_ptr!(self.entity()), ability_id) > 0 }
    }

    /// Checks if the monster has a given type.
    fn has_type(&self, type_id: MonsterTypeId) -> bool {
        unsafe { ffi::MonsterIsType(force_mut_ptr!(self.entity()), type_id) > 0 }
    }

    /// Checks if the monster has a certain IQ skill enabled.
    fn is_iq_skill_enabled(&self, iq_skill_id: IqSkillId) -> bool {
        unsafe { ffi::IqSkillIsEnabled(force_mut_ptr!(self.entity()), iq_skill_id) > 0 }
    }

    /// Checks if a defender has an active ability that isn't disabled by an attacker's (self)
    /// Mold Breaker.
    fn is_defender_ability_active(
        &self,
        defender: &DungeonEntity,
        defender_ability_id: AbilityId,
        own_ability_is_active: bool,
    ) -> bool {
        unsafe {
            ffi::DefenderAbilityIsActive(
                force_mut_ptr!(self.entity()),
                force_mut_ptr!(defender),
                defender_ability_id,
                own_ability_is_active as ffi::bool_,
            ) > 0
        }
    }

    /// Checks if a certain exclusive item effect is active for the monster.
    fn is_exclusive_item_effect_active(&self, effect_id: ExclusiveItemEffectId) -> bool {
        unsafe { ffi::ExclusiveItemEffectIsActive(force_mut_ptr!(self.entity()), effect_id) > 0 }
    }

    /// Gets the type matchup for a given combat interaction. Attacker is self.
    /// Note that the actual monster's types on the attacker and defender are not used;
    /// the entities are only used to check conditions. The actual type matchup table lookup is
    /// done solely using the attack and target type parameters.
    ///
    /// This factors in some conditional effects like exclusive items, statuses, etc.
    fn get_type_matchup(
        &self,
        defender: &DungeonEntity,
        target_type_index: TargetTypeIndex,
        attack_type: MonsterTypeId,
    ) -> Option<DungeonTypeMatchup> {
        unsafe {
            ffi::GetTypeMatchup(
                force_mut_ptr!(self.entity()),
                force_mut_ptr!(defender),
                target_type_index as i32,
                attack_type,
            )
                .try_into()
                .ok()
        }
    }

    /// Probably the damage calculation function.
    /// The result seems to get written to the unknown struct behind the pointer provided by
    /// damage_out, param_9 is also unknown.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_damage(
        &self,
        defender: &DungeonEntity,
        attack_type: MonsterTypeId,
        attack_power: i32,
        crit_chance: i32,
        damage_out: &mut ffi::damage_data,
        damage_multiplier: I24F8,
        move_id: MoveId,
        param_9: i32,
    ) {
        unsafe {
            ffi::CalcDamage(
                force_mut_ptr!(self.entity()),
                force_mut_ptr!(defender),
                attack_type,
                attack_power,
                crit_chance,
                damage_out,
                damage_multiplier.to_bits() as c_int,
                move_id,
                param_9,
            )
        }
    }

    /// Appears to calculate recoil damage to the monster.
    /// This function wraps [`Self::calc_damage_fixed`] using the monster as both the attacker and
    /// the defender, after doing some basic checks (like if the monster is already at 0 HP)
    /// and applying a boost from the Reckless ability if applicable.
    /// Some parameters are unknown.
    /// The result seems to get written to the unknown struct behind the pointer provided by
    /// damage_out, some other parameters are also unknown.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    unsafe fn calc_recoil_damage_fixed(
        &self,
        fixed_damage: i32,
        param_3: ffi::undefined4,
        damage_out: &mut ffi::damage_data,
        move_id: MoveId,
        attack_type: MonsterTypeId,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        ffi::CalcRecoilDamageFixed(
            force_mut_ptr!(self.entity()),
            fixed_damage,
            param_3,
            damage_out,
            move_id,
            attack_type,
            param_7,
            message_type,
            param_9,
            param_10,
        )
    }

    /// Appears to calculate damage from a fixed-damage effect.
    /// The result seems to get written to the unknown struct behind the pointer provided by
    /// damage_out, some other parameters are also unknown.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    unsafe fn calc_damage_fixed(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::damage_data,
        attack_type: MonsterTypeId,
        move_category: MoveCategory,
        param_8: i16,
        message_type: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        ffi::CalcDamageFixed(
            force_mut_ptr!(self.entity()),
            force_mut_ptr!(defender),
            fixed_damage,
            param_4,
            damage_out,
            attack_type,
            move_category as ffi::move_category::Type,
            param_8,
            message_type,
            param_10,
            param_11,
        )
    }

    /// A wrapper around [`Self::calc_damage_fixed`] with the move category
    /// set to [`MoveCategory::None`].
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    unsafe fn calc_damage_fixed_no_category(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::damage_data,
        attack_type: MonsterTypeId,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        ffi::CalcDamageFixedNoCategory(
            force_mut_ptr!(self.entity()),
            force_mut_ptr!(defender),
            fixed_damage,
            param_4,
            damage_out,
            attack_type,
            param_7,
            message_type,
            param_9,
            param_10,
        )
    }

    /// A wrapper (with potential side effects...?) around [`Self::calc_damage_fixed`].
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    unsafe fn calc_damage_fixed_wrapper(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::damage_data,
        attack_type: MonsterTypeId,
        move_category: MoveCategory,
        param_8: i16,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        ffi::CalcDamageFixedWrapper(
            force_mut_ptr!(self.entity()),
            force_mut_ptr!(defender),
            fixed_damage,
            param_4,
            damage_out,
            attack_type,
            move_category as ffi::move_category::Type,
            param_8,
            param_9,
            param_10,
            param_11,
        )
    }

    /// Appears to calculate damage from a variable-damage projectile.
    ///
    /// One of `param_5` or `param_6` is probably the output struct.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    fn calc_damage_projectile(
        &self,
        defender: &DungeonEntity,
        used_move: &Move,
        move_power: i32,
        param_5: ffi::undefined4,
        param_6: ffi::undefined4,
    ) {
        ffi::CalcDamageProjectile(
            force_mut_ptr!(self.entity()),
            force_mut_ptr!(defender),
            force_mut_ptr!(used_move),
            move_power,
            param_5,
            param_6,
        )
    }

    /// Checks if a monster is holding an aura bow that isn't disabled by Klutz.
    fn is_aura_bow_active(&self) -> bool {
        unsafe { ffi::AuraBowIsActive(force_mut_ptr!(self.entity())) > 0 }
    }

    /// Gets the exclusive item boost for attack/special attack for a monster.
    /// Panics if the move category is not physical or special.
    fn get_exclusive_item_offense_boost(&self, move_category: MoveCategory) -> i32 {
        if move_category == MoveCategory::Special || move_category == MoveCategory::None {
            panic!("get_exclusive_item_offense_boost called with invalid move category");
        }
        unsafe {
            ffi::ExclusiveItemOffenseBoost(force_mut_ptr!(self.entity()), move_category as c_int)
        }
    }

    /// Gets the exclusive item boost for defense/special defense for a monster.
    /// Panics if the move category is not physical or special.
    fn get_exclusive_item_defense_boost(&self, move_category: MoveCategory) -> i32 {
        if move_category == MoveCategory::Special || move_category == MoveCategory::None {
            panic!("get_exclusive_item_offense_boost called with invalid move category");
        }
        unsafe {
            ffi::ExclusiveItemDefenseBoost(force_mut_ptr!(self.entity()), move_category as c_int)
        }
    }

    /// Checks if a monster is currently immune to Ground-type moves for reasons other than typing and ability.
    ///
    /// This includes checks for Gravity and Magnet Rise.
    fn has_conditional_ground_immunity(&self) -> bool {
        unsafe { ffi::HasConditionalGroundImmunity(force_mut_ptr!(self.entity())) > 0 }
    }

    /// Gets the move target-and-range field when used by a given entity.
    ///
    /// The fourth field in the returned tuple seems unused.
    /// The values in the returned tuple are None, if they are invalid (or we don't know them yet).
    ///
    /// See [`Move::get_target_and_range`] for more information.
    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> MoveTargetAndRange {
        unsafe {
            ffi::GetEntityMoveTargetAndRange(
                force_mut_ptr!(self.entity()),
                force_mut_ptr!(the_move),
                is_ai as ffi::bool_,
            )
        }
            .into()
    }

    /// Get the weather, as experienced by the monster.
    fn get_apparent_weather(&self) -> Option<Weather> {
        unsafe { ffi::GetApparentWeather(force_mut_ptr!(self.entity())) }
            .try_into()
            .ok()
    }

    /// Checks if the monster has a certain held item.
    fn has_held_item(&self, item_id: ItemId) -> bool {
        unsafe { ffi::HasHeldItem(force_mut_ptr!(self.entity()), item_id) > 0 }
    }

    /// Gets the power of a move, factoring in Ginseng/Space Globe boosts.
    fn get_move_power(&self, the_move: &Move) -> i32 {
        unsafe { ffi::GetMovePower(force_mut_ptr!(self.entity()), force_mut_ptr!(the_move)) }
    }

    /// Seems to calculate the duration of a volatile status on a monster.
    ///
    /// Returns the number of turns for the status condition.
    ///
    /// # Arguments
    /// * `turn_range` - lower & higher ends of the turn range
    /// * `effects` - flag for whether or not to factor in the Self Curer IQ skill and the
    ///               Natural Cure ability
    fn calc_status_duration(&self, turn_range: &[u16; 2], effects: bool) -> i32 {
        unsafe {
            ffi::CalcStatusDuration(
                force_mut_ptr!(self.entity()),
                force_mut_ptr!(turn_range.as_ptr()),
                effects as ffi::bool_,
            )
        }
    }

    /// Returns the number of attacks that a monster can do in one turn (1 or 2).
    ///
    /// Checks for the abilities Swift Swim, Chlorophyll, Unburden, and for exclusive items.
    fn get_number_of_attacks(&self) -> i32 {
        unsafe { ffi::GetNumberOfAttacks(force_mut_ptr!(self.entity())) }
    }

    /// Checks if a monster is levitating (has the effect of Levitate and Gravity is not active)
    fn is_levitating(&self) -> bool {
        unsafe { ffi::LevitateIsActive(force_mut_ptr!(self.entity())) > 0 }
    }

    /// Checks if the monster is under the effect of Conversion 2 (its type was changed). Returns
    /// `None` if the value is invalid.
    fn is_conversion2_active(&self) -> Option<Conversion2Status> {
        unsafe { ffi::Conversion2IsActive(force_mut_ptr!(self.entity())) }
            .try_into()
            .ok()
    }

    /// Check the type of a move when used by a certain monster. Accounts for special cases
    /// such as Hidden Power, Weather Ball, the regular attack...
    fn get_move_type_if_used_by_self(&self, the_move: &Move) -> MonsterTypeId {
        unsafe {
            ffi::GetMoveTypeForMonster(force_mut_ptr!(self.entity()), force_mut_ptr!(the_move))
        }
    }

    /// Returns the animation id to be applied to a monster that has the sleep, napping,
    /// nightmare or bide status.
    fn get_sleep_animation_id(&self) -> u8 {
        unsafe { ffi::GetSleepAnimationId(force_mut_ptr!(self.entity())) }
    }

    /// Returns true if the monster has the blinded status, or if it is not the leader and is
    /// holding Y-Ray Specs.
    fn is_blinded(&self, check_held_item: bool) -> bool {
        unsafe { ffi::IsBlinded(force_mut_ptr!(self.entity()), check_held_item as ffi::bool_) > 0 }
    }
}

/// Trait for [`DungeonMonsterMut`] (write operations).
///
/// You may find more operations in [`DungeonEffectsEmitter`].
///
/// # Important safety note
/// Please see the safety note of [`DungeonEntity`]. It also applies to this trait.
trait DungeonMonsterWrite: private::Sealed {
    #[doc(hidden)]
    fn entity_mut(&mut self) -> &mut DungeonEntity;
    #[doc(hidden)]
    fn monster_mut(&mut self) -> &mut ffi::monster;

    /// Updates the PP of any moves that were used the a monster, if PP should be consumed.
    fn update_move_pp(&mut self, should_consume_pp: bool) {
        unsafe { ffi::UpdateMovePp(self.entity_mut(), should_consume_pp as ffi::bool_) }
    }

    /// Checks if the monster has the ability Truant, and if so tries to apply the pause status
    /// to it.
    fn try_activate_truant(&mut self) {
        unsafe { ffi::TryActivateTruant(self.entity_mut()) }
    }

    /// Tries to change a monster into one of its weather-related alternative forms.
    ///
    /// Applies to Castform and Cherrim, and checks for their unique abilities.
    fn try_weather_form_change(&mut self) {
        unsafe { ffi::TryWeatherFormChange(self.entity_mut()) }
    }

    /// Restores PP for all moves, clears flags [`Move::f_consume_2_pp`],
    /// [`Move::flags2_unk5`] and [`Move::flags2_unk7`], and sets flag
    /// [`Move::f_consume_pp`].
    ///
    /// Called when a monster is revived.
    fn restore_pp_for_all_moves_set_flags(&mut self) {
        unsafe { ffi::RestorePpAllMovesSetFlags(self.entity_mut()) }
    }

    /// Checks if the specified enemy should evolve because it just defeated an ally, and if so,
    /// attempts to evolve it.
    fn evolve_this_enemy_if_should(&mut self) {
        unsafe { ffi::EnemyEvolution(force_mut_ptr!(self.entity_mut())) }
    }

    /// Makes the specified monster evolve into the specified species.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    unsafe fn evolve(&mut self, param_2: &mut ffi::undefined4, new_monster_idx: MonsterSpeciesId) {
        ffi::EvolveMonster(force_mut_ptr!(self.entity_mut()), param_2, new_monster_idx)
    }

    /// Calculates the speed stage of a monster from its speed up/down counters. The second
    /// parameter is the weight of each counter (how many stages it will add/remove), but appears
    /// to be always 1.
    ///
    /// Takes modifiers into account (paralysis, snowy weather, Time Tripper). Deoxys-speed,
    /// Shaymin-sky and enemy Kecleon during a thief alert get a flat +1 always.
    ///
    /// The calculated speed stage is both returned and saved in the monster's statuses struct.
    fn calc_speed_stage(&mut self, counter_weight: i32) -> i32 {
        unsafe { ffi::CalcSpeedStage(self.entity_mut(), counter_weight) }
    }

    /// Sets the monster's reflect damage countdown to 4.
    fn set_reflect_damage_countdown_to_four(&mut self) {
        unsafe { ffi::SetReflectDamageCountdownTo4(self.entity_mut()) }
    }

    /// Executes the set action for the specified monster.
    ///
    /// Used for both AI actions and player-inputted actions. If the action is not ACTION_NOTHING,
    /// ACTION_PASS_TURN, ACTION_WALK or ACTION_UNK_4, the monster's already_acted field is set to
    /// true.
    fn execute_action(&mut self) {
        unsafe { ffi::ExecuteMonsterAction(self.entity_mut()) }
    }
}

impl<'a> DungeonMonsterRead for DungeonMonsterRef<'a> {
    fn entity(&self) -> &DungeonEntity {
        self.1
    }

    fn monster(&self) -> &ffi::monster {
        self.0
    }
}

impl<'a> DungeonMonsterRead for DungeonMonsterMut<'a> {
    fn entity(&self) -> &DungeonEntity {
        self.1
    }

    fn monster(&self) -> &ffi::monster {
        self.0
    }
}

impl<'a> DungeonMonsterWrite for DungeonMonsterMut<'a> {
    fn entity_mut(&mut self) -> &mut DungeonEntity {
        self.1
    }

    fn monster_mut(&mut self) -> &mut ffi::monster {
        self.0
    }
}

mod private {
    use super::{DungeonMonsterMut, DungeonMonsterRef};

    pub trait Sealed {}

    impl<'a> Sealed for DungeonMonsterRef<'a> {}
    impl<'a> Sealed for DungeonMonsterMut<'a> {}
}
