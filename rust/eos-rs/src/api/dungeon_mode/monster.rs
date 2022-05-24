use crate::api::dungeon_mode::*;
use crate::api::moves::*;
use crate::api::objects::*;
use fixed::types::I24F8;

/// Extension trait for [`DungeonMonsterRef`] (read operations).
///
/// # Important safety note
/// Please see the safety note of [`DungeonEntityExt`]. It also applies to this trait.
pub trait DungeonMonsterExtRead {
    /// Checks if the monster is a special story ally.
    /// This is a hard-coded check that looks at the monster's "Joined At" field.
    /// If the value is in the range [ [`dungeon_catalog::DUNGEON_JOINED_AT_BIDOOF`], [`dungeon_catalog::DUNGEON_DUMMY_0xE3`] ],
    /// this check will return true.
    fn is_special_story_ally(&self) -> bool;

    /// Checks if the monster does not gain experience.
    /// This basically just inverts IsSpecialStoryAlly, with the exception of also checking for
    /// the "Joined At" field being [`dungeon_catalog::DUNGEON_CLIENT`].
    fn is_experience_locked(&self) -> bool;

    /// Checks if the monster is holding a certain item that isn't disabled by Klutz.
    fn is_holding_item(&self, item_id: item_catalog::Type) -> bool;

    /// Checks if the monster is at low health (below 25% rounded down).
    fn has_low_health(&self) -> bool;

    /// Checks if the monster has the Gastro Acid status.
    fn gastro_acid_status(&self) -> bool;

    // Checks if the monster has a certain ability that isn't disabled by Gastro Acid.
    fn is_ability_active(&self, ability_id: ability_catalog::Type) -> bool;

    /// Checks if the monster has a given type.
    fn has_type(&self, type_id: type_catalog::Type) -> bool;

    /// Checks if the monster has a certain IQ skill enabled.
    fn is_iq_skill_enabled(&self, iq_skill_id: iq_skill_catalog::Type) -> bool;

    /// Checks if a defender has an active ability that isn't disabled by an attacker's (self)
    /// Mold Breaker.
    fn is_defender_ability_active(
        &self,
        defender: &DungeonEntity,
        defender_ability_id: ability_catalog::Type,
        own_ability_is_active: bool,
    ) -> bool;

    /// Checks if a certain exclusive item effect is active for the monster.
    fn is_exclusive_item_effect_active(&self, item_id: exclusive_item_effect_catalog::Type)
        -> bool;

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
        attack_type: type_catalog::Type,
    ) -> Option<DungeonTypeMatchup>;

    /// Probably the damage calculation function.
    /// The result seems to get written to the unknown struct behind the pointer provided by
    /// damage_out, param_9 is also unknown.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_damage(
        &self,
        defender: &DungeonEntity,
        attack_type: type_catalog::Type,
        attack_power: i32,
        crit_chance: i32,
        damage_out: &mut ffi::undefined4,
        damage_multiplier: I24F8,
        move_id: move_catalog::Type,
        param_9: i32,
    );

    /// Appears to calculate recoil damage to the monster.
    /// This function wraps [`Self::calc_damage_fixed`] using the monster as both the attacker and
    /// the defender, after doing some basic checks (like if the monster is already at 0 HP)
    /// and applying a boost from the Reckless ability if applicable.
    /// Some parameters are unknown.
    /// The result seems to get written to the unknown struct behind the pointer provided by
    /// damage_out, some other parameters are also unknown.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_recoil_damage_fixed(
        &self,
        fixed_damage: i32,
        param_3: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        move_id: move_catalog::Type,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    );

    /// Appears to calculate damage from a fixed-damage effect.
    /// The result seems to get written to the unknown struct behind the pointer provided by
    /// damage_out, some other parameters are also unknown.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_damage_fixed(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        message_type: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    );

    /// A wrapper around [`Self::calc_damage_fixed`] with the move category
    /// set to [`MoveCategory::None`].
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_damage_fixed_no_category(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    );

    /// A wrapper (with potential side effects...?) around [`Self::calc_damage_fixed`].
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_damage_fixed_wrapper(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    );

    /// Appears to calculate damage from a variable-damage projectile.
    ///
    /// One of `param_5` or `param_6` is probably the output struct.
    ///
    /// The signature of this method WILL change once we figure out what the parameters are.
    fn calc_damage_projectile(
        &self,
        defender: &DungeonEntity,
        used_move: &Move,
        move_power: i32,
        param_5: ffi::undefined4,
        param_6: ffi::undefined4,
    );

    /// Checks if a monster is holding an aura bow that isn't disabled by Klutz.
    fn is_aura_bow_active(&self) -> bool;

    /// Gets the exclusive item boost for attack/special attack for a monster.
    /// Panics if the move category is not physical or special.
    fn get_exclusive_item_offense_boost(&self, move_category: MoveCategory) -> i32;

    /// Gets the exclusive item boost for defense/special defense for a monster.
    /// Panics if the move category is not physical or special.
    fn get_exclusive_item_defense_boost(&self, move_category: MoveCategory) -> i32;

    /// Checks if a monster is currently immune to Ground-type moves for reasons other than typing and ability.
    ///
    /// This includes checks for Gravity and Magnet Rise.
    fn has_conditional_ground_immunity(&self) -> bool;

    /// Gets the move target-and-range field when used by a given entity.
    ///
    /// The fourth field in the returned tuple seems unused.
    /// The values in the returned tuple are None, if they are invalid (or we don't know them yet).
    ///
    /// See [`Move::get_target_and_range`] for more information.
    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> MoveTargetAndRange;

    /// Get the weather, as experienced by the monster.
    fn get_apparent_weather(&self) -> Option<Weather>;

    /// Checks if the monster has a certain held item.
    fn has_held_item(&self, item_id: item_catalog::Type) -> bool;

    /// Gets the power of a move, factoring in Ginseng/Space Globe boosts.
    fn get_move_power(&self, the_move: &Move) -> i32;

    /// Seems to calculate the duration of a volatile status on a monster.
    ///
    /// Returns the number of turns for the status condition.
    ///
    /// # Arguments
    /// * `turn_range` - lower & higher ends of the turn range
    /// * `effects` - flag for whether or not to factor in the Self Curer IQ skill and the
    ///               Natural Cure ability
    fn calc_status_duration(&self, turn_range: &[u16; 2], effects: bool) -> i32;

    /// Returns the number of attacks that a monster can do in one turn (1 or 2).
    ///
    /// Checks for the abilities Swift Swim, Chlorophyll, Unburden, and for exclusive items.
    fn get_number_of_attacks(&self) -> i32;

    /// Checks if a monster is levitating (has the effect of Levitate and Gravity is not active).
    fn is_levitating(&self) -> bool;

    /// Checks if the monster is under the effect of Conversion 2 (its type was changed). Returns
    /// `None` if the value is invalid.
    fn is_conversion2_active(&self) -> Option<Conversion2Status>;
}

/// Extension trait for [`DungeonMonsterMut`] (write operations).
///
/// You may find more operations in [`DungeonEffectsEmitter`].
///
/// # Important safety note
/// Please see the safety note of [`DungeonEntityExt`]. It also applies to this trait.
pub trait DungeonMonsterExtWrite {
    /// Updates the PP of any moves that were used the a monster, if PP should be consumed.
    fn update_move_pp(&mut self, should_consume_pp: bool);

    /// Checks if the monster has the ability Truant, and if so tries to apply the pause status
    /// to it.
    fn try_activate_truant(&mut self);

    /// Tries to change a monster into one of its weather-related alternative forms.
    ///
    /// Applies to Castform and Cherrim, and checks for their unique abilities.
    fn try_weather_form_change(&mut self);
}

impl<'a> DungeonMonsterExtRead for DungeonMonsterRef<'a> {
    fn is_special_story_ally(&self) -> bool {
        unsafe { ffi::IsSpecialStoryAlly(force_mut_ptr!(self.0)) > 0 }
    }

    fn is_experience_locked(&self) -> bool {
        unsafe { ffi::IsExperienceLocked(force_mut_ptr!(self.0)) > 0 }
    }

    fn is_holding_item(&self, item_id: u32) -> bool {
        unsafe { ffi::ItemIsActive(force_mut_ptr!(self.1), item_id) > 0 }
    }

    fn has_low_health(&self) -> bool {
        unsafe { ffi::HasLowHealth(force_mut_ptr!(self.1)) > 0 }
    }

    fn gastro_acid_status(&self) -> bool {
        unsafe { ffi::NoGastroAcidStatus(force_mut_ptr!(self.0)) == 0 }
    }

    fn is_ability_active(&self, ability_id: ability_catalog::Type) -> bool {
        unsafe { ffi::AbilityIsActive(force_mut_ptr!(self.1), ability_id) > 0 }
    }

    fn has_type(&self, type_id: type_catalog::Type) -> bool {
        unsafe { ffi::MonsterIsType(force_mut_ptr!(self.1), type_id) > 0 }
    }

    fn is_iq_skill_enabled(&self, iq_skill_id: iq_skill_catalog::Type) -> bool {
        unsafe { ffi::IqSkillIsEnabled(force_mut_ptr!(self.1), iq_skill_id) > 0 }
    }

    fn is_defender_ability_active(
        &self,
        defender: &DungeonEntity,
        defender_ability_id: ability_catalog::Type,
        own_ability_is_active: bool,
    ) -> bool {
        unsafe {
            ffi::DefenderAbilityIsActive(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                defender_ability_id,
                own_ability_is_active as ffi::bool_,
            ) > 0
        }
    }

    fn is_exclusive_item_effect_active(
        &self,
        effect_id: exclusive_item_effect_catalog::Type,
    ) -> bool {
        unsafe { ffi::ExclusiveItemEffectIsActive(force_mut_ptr!(self.1), effect_id) > 0 }
    }

    fn get_type_matchup(
        &self,
        defender: &DungeonEntity,
        target_type_index: TargetTypeIndex,
        attack_type: type_catalog::Type,
    ) -> Option<DungeonTypeMatchup> {
        unsafe {
            ffi::GetTypeMatchup(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                target_type_index as i32,
                attack_type,
            )
            .try_into()
            .ok()
        }
    }

    fn calc_damage(
        &self,
        defender: &DungeonEntity,
        attack_type: type_catalog::Type,
        attack_power: i32,
        crit_chance: i32,
        damage_out: &mut ffi::undefined4,
        damage_multiplier: I24F8,
        move_id: move_catalog::Type,
        param_9: i32,
    ) {
        unsafe {
            ffi::CalcDamage(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                attack_type,
                attack_power,
                crit_chance,
                damage_out as *mut _,
                damage_multiplier.to_bits() as c_int,
                move_id,
                param_9,
            )
        }
    }

    fn calc_recoil_damage_fixed(
        &self,
        fixed_damage: i32,
        param_3: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        move_id: move_catalog::Type,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        unsafe {
            ffi::CalcRecoilDamageFixed(
                force_mut_ptr!(self.1),
                fixed_damage,
                param_3,
                damage_out as *mut _,
                move_id,
                attack_type,
                param_7,
                message_type,
                param_9,
                param_10,
            )
        }
    }

    fn calc_damage_fixed(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        message_type: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        unsafe {
            ffi::CalcDamageFixed(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                fixed_damage,
                param_4,
                damage_out as *mut _,
                attack_type,
                move_category as move_catalog::Type,
                param_8,
                message_type,
                param_10,
                param_11,
            )
        }
    }

    fn calc_damage_fixed_no_category(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        unsafe {
            ffi::CalcDamageFixedNoCategory(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                fixed_damage,
                param_4,
                damage_out as *mut _,
                attack_type,
                param_7,
                message_type,
                param_9,
                param_10,
            )
        }
    }

    fn calc_damage_fixed_wrapper(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        unsafe {
            ffi::CalcDamageFixedWrapper(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                fixed_damage,
                param_4,
                damage_out as *mut _,
                attack_type,
                move_category as move_catalog::Type,
                param_8,
                param_9,
                param_10,
                param_11,
            )
        }
    }

    fn calc_damage_projectile(
        &self,
        defender: &DungeonEntity,
        used_move: &Move,
        move_power: i32,
        param_5: ffi::undefined4,
        param_6: ffi::undefined4,
    ) {
        unsafe {
            ffi::CalcDamageProjectile(
                force_mut_ptr!(self.1),
                force_mut_ptr!(defender),
                force_mut_ptr!(used_move),
                move_power,
                param_5,
                param_6,
            )
        }
    }

    fn is_aura_bow_active(&self) -> bool {
        unsafe { ffi::AuraBowIsActive(force_mut_ptr!(self.1)) > 0 }
    }

    fn get_exclusive_item_offense_boost(&self, move_category: MoveCategory) -> i32 {
        if move_category == MoveCategory::Special || move_category == MoveCategory::None {
            panic!("get_exclusive_item_offense_boost called with invalid move category");
        }
        unsafe { ffi::ExclusiveItemOffenseBoost(force_mut_ptr!(self.1), move_category as c_int) }
    }

    fn get_exclusive_item_defense_boost(&self, move_category: MoveCategory) -> i32 {
        if move_category == MoveCategory::Special || move_category == MoveCategory::None {
            panic!("get_exclusive_item_offense_boost called with invalid move category");
        }
        unsafe { ffi::ExclusiveItemDefenseBoost(force_mut_ptr!(self.1), move_category as c_int) }
    }

    fn has_conditional_ground_immunity(&self) -> bool {
        unsafe { ffi::HasConditionalGroundImmunity(force_mut_ptr!(self.1)) > 0 }
    }

    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> MoveTargetAndRange {
        unsafe {
            ffi::GetEntityMoveTargetAndRange(
                force_mut_ptr!(self.1),
                force_mut_ptr!(the_move),
                is_ai as ffi::bool_,
            )
        }
        .into()
    }

    fn get_apparent_weather(&self) -> Option<Weather> {
        unsafe { ffi::GetApparentWeather(force_mut_ptr!(self.1)) }
            .try_into()
            .ok()
    }

    fn has_held_item(&self, item_id: item_catalog::Type) -> bool {
        unsafe { ffi::HasHeldItem(force_mut_ptr!(self.1), item_id) > 0 }
    }

    fn get_move_power(&self, the_move: &Move) -> i32 {
        unsafe { ffi::GetMovePower(force_mut_ptr!(self.1), force_mut_ptr!(the_move)) }
    }

    fn calc_status_duration(&self, turn_range: &[u16; 2], effects: bool) -> i32 {
        unsafe {
            ffi::CalcStatusDuration(
                force_mut_ptr!(self.1),
                force_mut_ptr!(turn_range.as_ptr()),
                effects as ffi::bool_,
            )
        }
    }

    fn get_number_of_attacks(&self) -> i32 {
        unsafe { ffi::GetNumberOfAttacks(force_mut_ptr!(self.1)) }
    }

    fn is_levitating(&self) -> bool {
        unsafe { ffi::LevitateIsActive(force_mut_ptr!(self.1)) > 0 }
    }

    fn is_conversion2_active(&self) -> Option<Conversion2Status> {
        unsafe { ffi::Conversion2IsActive(force_mut_ptr!(self.1)) }
            .try_into()
            .ok()
    }
}

impl<'a> DungeonMonsterExtRead for DungeonMonsterMut<'a> {
    fn is_special_story_ally(&self) -> bool {
        self.as_ref().is_special_story_ally()
    }

    fn is_experience_locked(&self) -> bool {
        self.as_ref().is_experience_locked()
    }

    fn is_holding_item(&self, item_id: u32) -> bool {
        self.as_ref().is_holding_item(item_id)
    }

    fn has_low_health(&self) -> bool {
        self.as_ref().has_low_health()
    }

    fn gastro_acid_status(&self) -> bool {
        self.as_ref().gastro_acid_status()
    }

    fn is_ability_active(&self, ability_id: ability_catalog::Type) -> bool {
        self.as_ref().is_ability_active(ability_id)
    }

    fn has_type(&self, type_id: type_catalog::Type) -> bool {
        self.as_ref().has_type(type_id)
    }

    fn is_iq_skill_enabled(&self, iq_skill_id: iq_skill_catalog::Type) -> bool {
        self.as_ref().is_iq_skill_enabled(iq_skill_id)
    }

    fn is_defender_ability_active(
        &self,
        defender: &DungeonEntity,
        defender_ability_id: ability_catalog::Type,
        own_ability_is_active: bool,
    ) -> bool {
        self.as_ref().is_defender_ability_active(
            defender,
            defender_ability_id,
            own_ability_is_active,
        )
    }

    fn is_exclusive_item_effect_active(
        &self,
        item_id: exclusive_item_effect_catalog::Type,
    ) -> bool {
        self.as_ref().is_exclusive_item_effect_active(item_id)
    }

    fn get_type_matchup(
        &self,
        defender: &DungeonEntity,
        target_type_index: TargetTypeIndex,
        attack_type: type_catalog::Type,
    ) -> Option<DungeonTypeMatchup> {
        self.as_ref()
            .get_type_matchup(defender, target_type_index, attack_type)
    }

    fn calc_damage(
        &self,
        defender: &DungeonEntity,
        attack_type: type_catalog::Type,
        attack_power: i32,
        crit_chance: i32,
        damage_out: &mut ffi::undefined4,
        damage_multiplier: I24F8,
        move_id: move_catalog::Type,
        param_9: i32,
    ) {
        self.as_ref().calc_damage(
            defender,
            attack_type,
            attack_power,
            crit_chance,
            damage_out,
            damage_multiplier,
            move_id,
            param_9,
        )
    }

    fn calc_recoil_damage_fixed(
        &self,
        fixed_damage: i32,
        param_3: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        move_id: move_catalog::Type,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        self.as_ref().calc_recoil_damage_fixed(
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

    fn calc_damage_fixed(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        message_type: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        self.as_ref().calc_damage_fixed(
            defender,
            fixed_damage,
            param_4,
            damage_out,
            attack_type,
            move_category,
            param_8,
            message_type,
            param_10,
            param_11,
        )
    }

    fn calc_damage_fixed_no_category(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        self.as_ref().calc_damage_fixed_no_category(
            defender,
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

    fn calc_damage_fixed_wrapper(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: &mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        self.as_ref().calc_damage_fixed_wrapper(
            defender,
            fixed_damage,
            param_4,
            damage_out,
            attack_type,
            move_category,
            param_8,
            param_9,
            param_10,
            param_11,
        )
    }

    fn calc_damage_projectile(
        &self,
        defender: &DungeonEntity,
        used_move: &Move,
        move_power: i32,
        param_5: ffi::undefined4,
        param_6: ffi::undefined4,
    ) {
        self.as_ref()
            .calc_damage_projectile(defender, used_move, move_power, param_5, param_6)
    }

    fn is_aura_bow_active(&self) -> bool {
        self.as_ref().is_aura_bow_active()
    }

    fn get_exclusive_item_offense_boost(&self, move_category: MoveCategory) -> i32 {
        self.as_ref()
            .get_exclusive_item_offense_boost(move_category)
    }

    fn get_exclusive_item_defense_boost(&self, move_category: MoveCategory) -> i32 {
        self.as_ref()
            .get_exclusive_item_defense_boost(move_category)
    }

    fn has_conditional_ground_immunity(&self) -> bool {
        self.as_ref().has_conditional_ground_immunity()
    }

    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> MoveTargetAndRange {
        self.as_ref().get_move_target_and_range(the_move, is_ai)
    }

    fn get_apparent_weather(&self) -> Option<Weather> {
        self.as_ref().get_apparent_weather()
    }

    fn has_held_item(&self, item_id: item_catalog::Type) -> bool {
        self.as_ref().has_held_item(item_id)
    }

    fn get_move_power(&self, the_move: &Move) -> i32 {
        self.as_ref().get_move_power(the_move)
    }

    fn calc_status_duration(&self, turn_range: &[u16; 2], effects: bool) -> i32 {
        self.as_ref().calc_status_duration(turn_range, effects)
    }

    fn get_number_of_attacks(&self) -> i32 {
        self.as_ref().get_number_of_attacks()
    }

    fn is_levitating(&self) -> bool {
        self.as_ref().is_levitating()
    }

    fn is_conversion2_active(&self) -> Option<Conversion2Status> {
        self.as_ref().is_conversion2_active()
    }
}

impl<'a> DungeonMonsterExtWrite for DungeonMonsterMut<'a> {
    fn update_move_pp(&mut self, should_consume_pp: bool) {
        unsafe { ffi::UpdateMovePp(self.1 as *mut _, should_consume_pp as ffi::bool_) }
    }

    fn try_activate_truant(&mut self) {
        unsafe { ffi::TryActivateTruant(self.1 as *mut _) }
    }

    fn try_weather_form_change(&mut self) {
        unsafe { ffi::TryWeatherFormChange(self.1 as *mut _) }
    }
}
