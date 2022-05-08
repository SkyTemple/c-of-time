//! Traits, enums, structs and functions related to dungeon mode.

pub mod dungeon_generator;

use core::ffi::CStr;
use core::fmt::Debug;
use crate::api::fixed::I24F8;
use crate::api::moves::{HealingMoveType, MoveRange, MoveTarget};
use crate::api::objects::*;
use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::ctypes::*;
use crate::ffi;
use crate::string_util::str_to_cstring;

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
/// Move index of a monster, used by some functions.
pub enum TargetTypeIndex {
    FirstType = 0, SecondType = 1
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Describes the effectiveness of a move's type against type(s).
pub enum DungeonTypeMatchup {
    Immune = ffi::type_matchup::MATCHUP_IMMUNE,
    NotVeryEffective = ffi::type_matchup::MATCHUP_NOT_VERY_EFFECTIVE,
    Neutral = ffi::type_matchup::MATCHUP_NEUTRAL,
    SuperEffective = ffi::type_matchup::MATCHUP_SUPER_EFFECTIVE,
}

impl TryInto<DungeonTypeMatchup> for ffi::type_matchup::Type {
    type Error = ();

    fn try_into(self) -> Result<DungeonTypeMatchup, Self::Error> {
        match self {
            ffi::type_matchup::MATCHUP_IMMUNE => Ok(DungeonTypeMatchup::Immune),
            ffi::type_matchup::MATCHUP_NOT_VERY_EFFECTIVE => Ok(DungeonTypeMatchup::NotVeryEffective),
            ffi::type_matchup::MATCHUP_NEUTRAL => Ok(DungeonTypeMatchup::Neutral),
            ffi::type_matchup::MATCHUP_SUPER_EFFECTIVE => Ok(DungeonTypeMatchup::SuperEffective),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of an entity in a dungeon.
pub enum DungeonEntityType {
    Nothing = ffi::entity_type::ENTITY_NOTHING,
    Monster = ffi::entity_type::ENTITY_MONSTER,
    Trap = ffi::entity_type::ENTITY_TRAP,
    Item = ffi::entity_type::ENTITY_ITEM,
    HiddenStairs = ffi::entity_type::ENTITY_HIDDEN_STAIRS,
}

impl TryInto<DungeonEntityType> for ffi::entity_type::Type {
    type Error = ();

    fn try_into(self) -> Result<DungeonEntityType, Self::Error> {
        match self {
            ffi::entity_type::ENTITY_NOTHING => Ok(DungeonEntityType::Nothing),
            ffi::entity_type::ENTITY_MONSTER => Ok(DungeonEntityType::Monster),
            ffi::entity_type::ENTITY_TRAP => Ok(DungeonEntityType::Trap),
            ffi::entity_type::ENTITY_ITEM => Ok(DungeonEntityType::Item),
            ffi::entity_type::ENTITY_HIDDEN_STAIRS => Ok(DungeonEntityType::HiddenStairs),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The category of a move.
pub enum MoveCategory {
    None = ffi::move_category::CATEGORY_NONE,
    Physical = ffi::move_category::CATEGORY_PHYSICAL,
    Special = ffi::move_category::CATEGORY_SPECIAL,
    Status = ffi::move_category::CATEGORY_STATUS
}

impl TryInto<MoveCategory> for ffi::entity_type::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveCategory, Self::Error> {
        match self {
            ffi::move_category::CATEGORY_NONE => Ok(MoveCategory::None),
            ffi::move_category::CATEGORY_PHYSICAL => Ok(MoveCategory::Physical),
            ffi::move_category::CATEGORY_SPECIAL => Ok(MoveCategory::Special),
            ffi::move_category::CATEGORY_STATUS => Ok(MoveCategory::Status),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of a floor.
pub enum FloorType {
    /// The floor is neither a fixed floor nor does it contain a rescue point.
    Normal = ffi::floor_type::FLOOR_TYPE_NORMAL,
    /// The floor is a fixed floor.
    Fixed = ffi::floor_type::FLOOR_TYPE_FIXED,
    /// The floor has a rescue point.
    Rescue = ffi::floor_type::FLOOR_TYPE_RESCUE,
}

impl TryInto<FloorType> for ffi::floor_type::Type {
    type Error = ();

    fn try_into(self) -> Result<FloorType, Self::Error> {
        match self {
            ffi::floor_type::FLOOR_TYPE_NORMAL => Ok(FloorType::Normal),
            ffi::floor_type::FLOOR_TYPE_FIXED => Ok(FloorType::Fixed),
            ffi::floor_type::FLOOR_TYPE_RESCUE => Ok(FloorType::Rescue),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of terrain of a tile.
pub enum TerrainType {
    Wall = ffi::terrain_type::TERRAIN_WALL,
    Normal = ffi::terrain_type::TERRAIN_NORMAL,
    Secondary = ffi::terrain_type::TERRAIN_SECONDARY,
    Chasm = ffi::terrain_type::TERRAIN_CHASM,
}

impl TryInto<TerrainType> for ffi::terrain_type::Type {
    type Error = ();

    fn try_into(self) -> Result<TerrainType, Self::Error> {
        match self {
            ffi::terrain_type::TERRAIN_WALL => Ok(TerrainType::Wall),
            ffi::terrain_type::TERRAIN_NORMAL => Ok(TerrainType::Normal),
            ffi::terrain_type::TERRAIN_SECONDARY => Ok(TerrainType::Secondary),
            ffi::terrain_type::TERRAIN_CHASM => Ok(TerrainType::Chasm),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of secondary terrain of a tile.
pub enum SecondaryTerrainType {
    Water = ffi::secondary_terrain_type::SECONDARY_TERRAIN_WATER,
    Lava = ffi::secondary_terrain_type::SECONDARY_TERRAIN_LAVA,
    Chasm = ffi::secondary_terrain_type::SECONDARY_TERRAIN_CHASM,
}

impl TryInto<SecondaryTerrainType> for ffi::secondary_terrain_type::Type {
    type Error = ();

    fn try_into(self) -> Result<SecondaryTerrainType, Self::Error> {
        match self {
            ffi::secondary_terrain_type::SECONDARY_TERRAIN_WATER => Ok(SecondaryTerrainType::Water),
            ffi::secondary_terrain_type::SECONDARY_TERRAIN_LAVA => Ok(SecondaryTerrainType::Lava),
            ffi::secondary_terrain_type::SECONDARY_TERRAIN_CHASM => Ok(SecondaryTerrainType::Chasm),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Types of weather.
pub enum Weather {
    Clear = ffi::weather_id::WEATHER_CLEAR,
    Sunny = ffi::weather_id::WEATHER_SUNNY,
    Sandstorm = ffi::weather_id::WEATHER_SANDSTORM,
    Cloudy = ffi::weather_id::WEATHER_CLOUDY,
    Rain = ffi::weather_id::WEATHER_RAIN,
    Hail = ffi::weather_id::WEATHER_HAIL,
    Fog = ffi::weather_id::WEATHER_FOG,
    Snow = ffi::weather_id::WEATHER_SNOW,
    Random = ffi::weather_id::WEATHER_RANDOM
}

impl TryInto<Weather> for ffi::weather_id::Type {
    type Error = ();

    fn try_into(self) -> Result<Weather, Self::Error> {
        match self {
            ffi::weather_id::WEATHER_CLEAR => Ok(Weather::Clear),
            ffi::weather_id::WEATHER_SUNNY => Ok(Weather::Sunny),
            ffi::weather_id::WEATHER_SANDSTORM => Ok(Weather::Sandstorm),
            ffi::weather_id::WEATHER_CLOUDY => Ok(Weather::Cloudy),
            ffi::weather_id::WEATHER_RAIN => Ok(Weather::Rain),
            ffi::weather_id::WEATHER_HAIL => Ok(Weather::Hail),
            ffi::weather_id::WEATHER_FOG => Ok(Weather::Fog),
            ffi::weather_id::WEATHER_SNOW => Ok(Weather::Snow),
            ffi::weather_id::WEATHER_RANDOM => Ok(Weather::Random),
            _ => Err(()),
        }
    }
}

/// Extension trait for [`DungeonEntity`].
/// 
/// # Important safety note
/// Implementations of this trait can assume that overlay 29 is loaded (since this is the
/// only context dungeon entities are actually relevant). If you manually implement this trait,
/// for some reason, you NEED to make sure overlay 29 is loaded when using some functions of this
/// trait.
/// If you use the DungeonEntity structs manually outside of dungeon mode, this trait will be
/// unsafe to use.
/// The trait and its functions are marked safe for convenience, since in it's intended use case,
/// overlay 29 will always be loaded.
pub trait DungeonEntityExt {
    /// Checks if a given entity is actually valid.
    fn is_valid(slf: *mut Self) -> bool;

    /// Entity type. Invalid values will return None.
    fn entity_type(&self) -> Option<DungeonEntityType>;
    
    /// This returns the monster info struct for the entity,
    /// panics if the entity is not a monster.
    fn info_for_monster(&self) -> Option<DungeonMonsterRef>;

    /// This returns the item info struct for the entity,
    /// panics if the entity is not an item.
    fn info_for_item(&self) -> Option<&DungeonItem>;

    /// This returns the trap info struct for the entity,
    /// panics if the entity is not a trap.
    fn info_for_trap(&self) -> Option<&DungeonTrap>;

    /// This returns the monster info struct for the entity,
    /// panics if the entity is not a monster.
    fn info_for_monster_mut(&mut self) -> Option<DungeonMonsterMut>;

    /// This returns the item info struct for the entity,
    /// panics if the entity is not an item.
    fn info_for_item_mut(&mut self) -> Option<&mut DungeonItem>;

    /// This returns the trap info struct for the entity,
    /// panics if the entity is not a trap.
    fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap>;
}

impl DungeonEntityExt for DungeonEntity {
    fn is_valid(slf: *mut Self) -> bool {
        // SAFETY: The lease passed into the function promises us that the overlay is loaded.
        //         Since this function is intended to actually check if the entity is valid,
        //         it's safe to call, even if the `slf` pointer is invalid.
        unsafe { ffi::EntityIsValid(slf) > 0 }
    }

    fn entity_type(&self) -> Option<DungeonEntityType> {
        self.type_.try_into().ok()
    }

    fn info_for_monster(&self) -> Option<DungeonMonsterRef> {
        if self.entity_type() == Some(DungeonEntityType::Monster) {
            unsafe { Some(DungeonMonsterRef(&*(self.info as *const ffi::monster), self)) }
        } else {
            None
        }
    }

    fn info_for_item(&self) -> Option<&DungeonItem> {
        if self.entity_type() == Some(DungeonEntityType::Item) {
            unsafe { Some(&*(self.info as *const DungeonItem)) }
        } else {
            None
        }
    }

    fn info_for_trap(&self) -> Option<&DungeonTrap> {
        if self.entity_type() == Some(DungeonEntityType::Trap) {
            unsafe { Some(&*(self.info as *const DungeonTrap)) }
        } else {
            None
        }
    }

    fn info_for_monster_mut(&mut self) -> Option<DungeonMonsterMut> {
        if self.entity_type() == Some(DungeonEntityType::Monster) {
            unsafe { Some(DungeonMonsterMut(&mut *(self.info as *mut ffi::monster), self)) }
        } else {
            None
        }
    }

    fn info_for_item_mut(&mut self) -> Option<&mut DungeonItem> {
        if self.entity_type() == Some(DungeonEntityType::Item) {
            unsafe { Some(&mut *(self.info as *mut DungeonItem)) }
        } else {
            None
        }
    }

    fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap> {
        if self.entity_type() == Some(DungeonEntityType::Trap) {
            unsafe { Some(&mut *(self.info as *mut DungeonTrap)) }
        } else {
            None
        }
    }
}

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
    fn is_defender_ability_active(&self, defender: &DungeonEntity, defender_ability_id: ability_catalog::Type, own_ability_is_active: bool) -> bool;

    /// Checks if a certain exclusive item effect is active for the monster.
    fn is_exclusive_item_effect_active(&self, item_id: exclusive_item_effect_catalog::Type) -> bool;

    /// Checks if the monster is a team member under the effects of a certain exclusive item effect.
    fn is_exclusive_item_effect_is_active_for_team_member(&self, item_id: exclusive_item_effect_catalog::Type) -> bool;

    /// Gets the type matchup for a given combat interaction. Attacker is self.
    /// Note that the actual monster's types on the attacker and defender are not used;
    /// the entities are only used to check conditions. The actual type matchup table lookup is
    /// done solely using the attack and target type parameters.
    ///
    /// This factors in some conditional effects like exclusive items, statuses, etc.
    fn get_type_matchup(&self, defender: &DungeonEntity, target_type_index: TargetTypeIndex, attack_type: type_catalog::Type) -> Option<DungeonTypeMatchup>;

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
        damage_out: *mut ffi::undefined4,
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
        damage_out: *mut ffi::undefined4,
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
        damage_out: *mut ffi::undefined4,
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
        damage_out: *mut ffi::undefined4,
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
        damage_out: *mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4
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
        param_6: ffi::undefined4
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
    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> (Option<MoveTarget>, Option<MoveRange>, Option<HealingMoveType>, u16);

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
}

/// Extension trait for [`DungeonMonsterMut`] (write operations).
///
/// # Important safety note
/// Please see the safety note of [`DungeonEntityExt`]. It also applies to this trait.
pub trait DungeonMonsterExtWrite {
    /// Updates the PP of any moves that were used the a monster, if PP should be consumed.
    fn update_move_pp(&mut self, should_consume_pp: bool);
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

    fn is_defender_ability_active(&self, defender: &DungeonEntity, defender_ability_id: ability_catalog::Type, own_ability_is_active: bool) -> bool {
        unsafe { ffi::DefenderAbilityIsActive(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            defender_ability_id, own_ability_is_active as ffi::bool_
        ) > 0 }
    }

    fn is_exclusive_item_effect_active(&self, effect_id: exclusive_item_effect_catalog::Type) -> bool {
        unsafe { ffi::ExclusiveItemEffectIsActive(force_mut_ptr!(self.1), effect_id) > 0 }
    }

    fn is_exclusive_item_effect_is_active_for_team_member(&self, effect_id: exclusive_item_effect_catalog::Type) -> bool {
        unsafe { ffi::TeamExclusiveItemEffectIsActive(force_mut_ptr!(self.1), effect_id) > 0 }
    }

    fn get_type_matchup(&self, defender: &DungeonEntity, target_type_index: TargetTypeIndex, attack_type: type_catalog::Type) -> Option<DungeonTypeMatchup> {
        unsafe { ffi::GetTypeMatchup(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            target_type_index as i32, attack_type
        ).try_into().ok() }
    }

    fn calc_damage(
        &self,
        defender: &DungeonEntity,
        attack_type: type_catalog::Type,
        attack_power: i32,
        crit_chance: i32,
        damage_out: *mut ffi::undefined4,
        damage_multiplier: I24F8,
        move_id: move_catalog::Type,
        param_9: i32,
    ) {
        unsafe { ffi::CalcDamage(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            attack_type, attack_power, crit_chance, damage_out,
            damage_multiplier.to_bits() as c_int, move_id, param_9
        ) }
    }

    fn calc_recoil_damage_fixed(
        &self,
        fixed_damage: i32,
        param_3: ffi::undefined4,
        damage_out: *mut ffi::undefined4,
        move_id: move_catalog::Type,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        unsafe { ffi::CalcRecoilDamageFixed(
            force_mut_ptr!(self.1), fixed_damage, param_3, damage_out,
            move_id, attack_type, param_7, message_type, param_9, param_10
        ) }
    }

    fn calc_damage_fixed(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: *mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        message_type: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4,
    ) {
        unsafe { ffi::CalcDamageFixed(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            fixed_damage, param_4, damage_out,
            attack_type, move_category as move_catalog::Type, param_8, message_type,
            param_10, param_11
        ) }
    }

    fn calc_damage_fixed_no_category(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: *mut ffi::undefined4,
        attack_type: type_catalog::Type,
        param_7: i16,
        message_type: ffi::undefined4,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
    ) {
        unsafe { ffi::CalcDamageFixedNoCategory(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            fixed_damage, param_4, damage_out,
            attack_type, param_7, message_type,
            param_9, param_10
        ) }
    }

    fn calc_damage_fixed_wrapper(
        &self,
        defender: &DungeonEntity,
        fixed_damage: i32,
        param_4: ffi::undefined4,
        damage_out: *mut ffi::undefined4,
        attack_type: type_catalog::Type,
        move_category: MoveCategory,
        param_8: i16,
        param_9: ffi::undefined4,
        param_10: ffi::undefined4,
        param_11: ffi::undefined4
    ) {
        unsafe { ffi::CalcDamageFixedWrapper(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            fixed_damage, param_4, damage_out,
            attack_type, move_category as move_catalog::Type, param_8,
            param_9, param_10, param_11
        ) }
    }

    fn calc_damage_projectile(
        &self,
        defender: &DungeonEntity,
        used_move: &Move,
        move_power: i32,
        param_5: ffi::undefined4,
        param_6: ffi::undefined4
    ) {
        unsafe { ffi::CalcDamageProjectile(
            force_mut_ptr!(self.1), force_mut_ptr!(defender),
            force_mut_ptr!(used_move), move_power,
            param_5, param_6
        ) }
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

    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> (Option<MoveTarget>, Option<MoveRange>, Option<HealingMoveType>, u16) {
        unsafe { ffi::GetEntityMoveTargetAndRange(
            force_mut_ptr!(self.1), force_mut_ptr!(the_move), is_ai as ffi::bool_
        ) }.into()
    }

    fn get_apparent_weather(&self) -> Option<Weather> {
        unsafe { ffi::GetApparentWeather(force_mut_ptr!(self.1)) }.try_into().ok()
    }

    fn has_held_item(&self, item_id: item_catalog::Type) -> bool {
        unsafe { ffi::HasHeldItem(force_mut_ptr!(self.1), item_id) > 0 }
    }

    fn get_move_power(&self, the_move: &Move) -> i32 {
        unsafe { ffi::GetMovePower(force_mut_ptr!(self.1), force_mut_ptr!(the_move)) }
    }

    fn calc_status_duration(&self, turn_range: &[u16; 2], effects: bool) -> i32 {
        unsafe { ffi::CalcStatusDuration(
            force_mut_ptr!(self.1), force_mut_ptr!(turn_range.as_ptr()),
            effects as ffi::bool_
        ) }
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

    fn is_defender_ability_active(&self, defender: &DungeonEntity, defender_ability_id: ability_catalog::Type, own_ability_is_active: bool) -> bool {
        self.as_ref().is_defender_ability_active(defender, defender_ability_id, own_ability_is_active)
    }

    fn is_exclusive_item_effect_active(&self, item_id: exclusive_item_effect_catalog::Type) -> bool {
        self.as_ref().is_exclusive_item_effect_active(item_id)
    }

    fn is_exclusive_item_effect_is_active_for_team_member(&self, item_id: exclusive_item_effect_catalog::Type) -> bool {
        self.as_ref().is_exclusive_item_effect_is_active_for_team_member(item_id)
    }

    fn get_type_matchup(&self, defender: &DungeonEntity, target_type_index: TargetTypeIndex, attack_type: type_catalog::Type) -> Option<DungeonTypeMatchup> {
        self.as_ref().get_type_matchup(defender, target_type_index, attack_type)
    }

    fn calc_damage(&self, defender: &DungeonEntity, attack_type: type_catalog::Type, attack_power: i32, crit_chance: i32, damage_out: *mut ffi::undefined4, damage_multiplier: I24F8, move_id: move_catalog::Type, param_9: i32) {
        self.as_ref().calc_damage(defender, attack_type, attack_power, crit_chance, damage_out, damage_multiplier, move_id, param_9)
    }

    fn calc_recoil_damage_fixed(&self, fixed_damage: i32, param_3: ffi::undefined4, damage_out: *mut ffi::undefined4, move_id: move_catalog::Type, attack_type: type_catalog::Type, param_7: i16, message_type: ffi::undefined4, param_9: ffi::undefined4, param_10: ffi::undefined4) {
        self.as_ref().calc_recoil_damage_fixed(fixed_damage, param_3, damage_out, move_id, attack_type, param_7, message_type, param_9, param_10)
    }

    fn calc_damage_fixed(&self, defender: &DungeonEntity, fixed_damage: i32, param_4: ffi::undefined4, damage_out: *mut ffi::undefined4, attack_type: type_catalog::Type, move_category: MoveCategory, param_8: i16, message_type: ffi::undefined4, param_10: ffi::undefined4, param_11: ffi::undefined4) {
        self.as_ref().calc_damage_fixed(defender, fixed_damage, param_4, damage_out, attack_type, move_category, param_8, message_type, param_10, param_11)
    }

    fn calc_damage_fixed_no_category(&self, defender: &DungeonEntity, fixed_damage: i32, param_4: ffi::undefined4, damage_out: *mut ffi::undefined4, attack_type: type_catalog::Type, param_7: i16, message_type: ffi::undefined4, param_9: ffi::undefined4, param_10: ffi::undefined4) {
        self.as_ref().calc_damage_fixed_no_category(defender, fixed_damage, param_4, damage_out, attack_type, param_7, message_type, param_9, param_10)
    }

    fn calc_damage_fixed_wrapper(&self, defender: &DungeonEntity, fixed_damage: i32, param_4: ffi::undefined4, damage_out: *mut ffi::undefined4, attack_type: type_catalog::Type, move_category: MoveCategory, param_8: i16, param_9: ffi::undefined4, param_10: ffi::undefined4, param_11: ffi::undefined4) {
        self.as_ref().calc_damage_fixed_wrapper(defender, fixed_damage, param_4, damage_out, attack_type, move_category, param_8, param_9, param_10, param_11)
    }

    fn calc_damage_projectile(&self, defender: &DungeonEntity, used_move: &Move, move_power: i32, param_5: ffi::undefined4, param_6: ffi::undefined4) {
        self.as_ref().calc_damage_projectile(defender, used_move, move_power, param_5, param_6)
    }

    fn is_aura_bow_active(&self) -> bool {
        self.as_ref().is_aura_bow_active()
    }

    fn get_exclusive_item_offense_boost(&self, move_category: MoveCategory) -> i32 {
        self.as_ref().get_exclusive_item_offense_boost(move_category)
    }

    fn get_exclusive_item_defense_boost(&self, move_category: MoveCategory) -> i32 {
        self.as_ref().get_exclusive_item_defense_boost(move_category)
    }

    fn has_conditional_ground_immunity(&self) -> bool {
        self.as_ref().has_conditional_ground_immunity()
    }

    fn get_move_target_and_range(&self, the_move: &Move, is_ai: bool) -> (Option<MoveTarget>, Option<MoveRange>, Option<HealingMoveType>, u16) {
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
}

impl<'a> DungeonMonsterExtWrite for DungeonMonsterMut<'a> {
    fn update_move_pp(&mut self, should_consume_pp: bool) {
        unsafe { ffi::UpdateMovePp(self.1 as *mut _, should_consume_pp as ffi::bool_) }
    }
}

/// Extension trait for [`Move`] specific to dungeon mode.
pub trait DungeonMoveExt {
    /// Checks if the move isn't a physical move.
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool;

    /// Checks if the move isn't a physical move.
    fn move_is_not_physical_static(move_id: move_catalog::Type, _ov29: &OverlayLoadLease<29>) -> bool;
}

impl DungeonMoveExt for Move {
    fn move_is_not_physical(&self, _ov29: &OverlayLoadLease<29>) -> bool {
        Self::move_is_not_physical_static(self.id.val(), _ov29)
    }

    fn move_is_not_physical_static(move_id: move_catalog::Type, _ov29: &OverlayLoadLease<29>) -> bool {
        unsafe { ffi::MoveIsNotPhysical(move_id) > 0 }
    }
}

/// Extension trait for [`DungeonTile`].

pub trait DungeonTileExt {
    /// Gets the terrain type of a tile.
    /// Returns None if the terrain type is invalid.
    fn get_terrain(&self) -> Option<TerrainType>;
}

impl DungeonTileExt for DungeonTile {
    fn get_terrain(&self) -> Option<TerrainType> {
        unsafe { ffi::GetTileTerrain(force_mut_ptr!(self)) }.try_into().ok()
    }
}

/// Helper struct for dealing with the current floor and global dungeon and mission state.
///
/// To generate dungeons, see the [`dungeon_generator`] module.
///
/// # Safety
/// A lot of methods on this struct are unsafe because they work on/with the global dungeon pointer
/// and the normal reference borrowing rules can't be enforced (there may be multiple mutable
/// references to data inside the struct at the same time for example). Additionally some methods
/// may leave the global dungeon struct or pointer in an invalid/null state.
///
/// Some non-writing safe-marked functions also work with this global struct but are deemed
/// "safe enough" to use. They may return some nonsensical, but still valid data in situations
/// where the global dungeon pointer or the struct itself is invalid.
///
/// Most of these functions will additionally result in UB or abort
/// if the global dungeon pointer is null.
///
/// Methods that take `&mut self` modify the global dungeon struct/pointer.
pub struct GlobalDungeonData(OverlayLoadLease<29>);

impl CreatableWithLease<29> for GlobalDungeonData {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self(lease)
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self.0
    }
}

impl GlobalDungeonData {
    /// Checks if the global dungeon pointer is null.
    pub fn global_dungeon_ptr_null(&self) -> bool {
        unsafe { ffi::GetDungeonPtrMaster() }.is_null()
    }

    /// Returns a reference to the current global dungeon struct.
    /// Panics if the global dungeon pointer is null.
    pub unsafe fn global_dungeon_ref(&self) -> &ffi::dungeon {
        let ptr = ffi::GetDungeonPtrMaster();
        assert!(!ptr.is_null(), "Global dungeon pointer is null!");
        &*ffi::GetDungeonPtrMaster()
    }

    /// Returns a mutable reference to the current global dungeon struct.
    /// Panics if the global dungeon pointer is null.
    pub unsafe fn global_dungeon_mut(&mut self) -> &mut ffi::dungeon {
        let ptr = ffi::GetDungeonPtrMaster();
        assert!(!ptr.is_null(), "Global dungeon pointer is null!");
        &mut *ptr
    }

    /// This will allocate a new dungeon struct and update the global dungeon pointer to it.
    pub unsafe fn alloc(&mut self) -> &mut ffi::dungeon {
        &mut *ffi::DungeonAlloc()
    }

    /// Zeros out the struct pointed to by the global dungeon pointer.
    pub unsafe fn z_init(&mut self) {
        ffi::DungeonZInit()
    }

    /// Frees the dungeons struct pointer to by the master dungeon pointer,
    /// and nullifies the pointer.
    pub unsafe fn free(&mut self) {
        ffi::DungeonFree()
    }

    /// Seems to initialize the dungeon struct from specified dungeon data.
    ///
    /// The signature will be updated once we know more about this function.
    pub unsafe fn initialize_dungeon(&mut self, dungeon_data: *mut ffi::undefined, dungeon: *mut ffi::dungeon) -> i32 {
        ffi::InitializeDungeon(dungeon_data, dungeon)
    }

    /// Gets the floor type. Returns None if the global dungeon struct contains invalid data.
    pub fn get_floor_type(&self) -> Option<FloorType> {
        unsafe { ffi::GetFloorType() }.try_into().ok()
    }

    /// Checks if the current fixed floor is the "substitute room" (Fixed Room ID 0x6E).
    pub fn is_substitute_room(&self) -> bool {
        unsafe { ffi::FixedRoomIsSubstituteRoom() > 0 }
    }

    /// Checks if the current dungeon floor number is even.
    /// This is probably, among other things(?), used to determe whether male or female monsters
    /// should be spawned.
    /// Has a special check to return false for Labyrinth Cave B10F (the Gabite boss fight).
    pub fn is_even_floor(&self) -> bool {
        unsafe { ffi::FloorNumberIsEven() > 0 }
    }
    
    /// Returns the tile at the given coordinates.
    pub unsafe fn get_tile(&self, x: i32, y: i32) -> &DungeonTile {
        &*ffi::GetTile(x, y)
    }

    /// Returns the tile at the given coordinates.
    pub unsafe fn get_tile_mut(&self, x: i32, y: i32) -> &mut DungeonTile {
        &mut*ffi::GetTile(x, y)
    }
    
    /// Checks if gravity is active on the floor.
    pub fn is_gravity_active(&self) -> bool {
        unsafe { ffi::GravityIsActive() > 0 }
    }
    
    /// Checks if the current floor is the Secret Bazaar.
    pub fn is_secret_bazaar(&self) -> bool {
        unsafe { ffi::IsSecretBazaar() > 0 }
    }

    /// Checks if the current floor is the Secret Room fixed floor (from hidden stairs).
    pub fn is_secret_room(&self) -> bool {
        unsafe { ffi::IsSecretRoom() > 0 }
    }

    /// Checks if the current floor is a normal layout.
    ///
    /// "Normal" means any layout that is NOT one of the following:
    /// - Hidden stairs floors
    /// - Golden Chamber
    /// - Challenge Request floor
    /// - Outlaw hideout
    /// - Treasure Memo floor
    /// - Full-room fixed floors (ID < 0xA5) [0xA5 == Sealed Chamber]
    pub fn is_normal_floor(&self) -> bool {
        unsafe { ffi::IsNormalFloor() > 0 }
    }

    /// Checks if a fixed room ID corresponds to a fixed, full-floor layout.
    pub fn is_full_floor_fixed_rooms(&self, fixed_room_id: fixed_room_catalog::Type) -> bool {
        unsafe { ffi::IsNotFullFloorFixedRoom(fixed_room_id) == 0 }
    }

    /// This is the master function that generates the dungeon floor.
    ///
    /// Very loosely speaking, this function first tries to generate a valid floor layout.
    /// Then it tries to spawn entities in a valid configuration. Finally, it performs cleanup
    /// and post-processing depending on the dungeon.
    ///
    /// If a spawn configuration is invalid, the entire floor layout is scrapped and regenerated.
    /// If the generated floor layout is invalid 10 times in a row, or a valid spawn configuration
    /// isn't generated within 10 attempts, the generation algorithm aborts and the default
    /// one-room Monster House floor is generated as a fallback.
    pub unsafe fn generate_floor(&mut self) {
        ffi::GenerateFloor()
    }

    //     - name: PosIsOutOfBounds
    //       address:
    //         NA: 0x2340CAC
    //         EU: 0x2341890
    //       description: |-
    //         Checks if a position (x, y) is out of bounds on the map: !((0 <= x <= 55) && (0 <= y <= 31)).
    //
    //         r0: x
    //         r1: y
    //         return: bool
    //     - name: IsHiddenStairsFloor
    //       address:
    //         NA: 0x234450C
    //         EU: 0x23450F0
    //       description: |-
    //         Checks if the current floor is either the Secret Bazaar or a Secret Room.
    //
    //         return: bool
    //     - name: IsCurrentMissionType
    //       address:
    //         NA: 0x234921C
    //         EU: 0x2349E1C
    //       description: |-
    //         Checks if the current floor is an active mission destination of a given type (and any subtype).
    //
    //         r0: mission type
    //         return: bool
    //     - name: IsCurrentMissionTypeExact
    //       address:
    //         NA: 0x2349250
    //         EU: 0x2349E50
    //       description: |-
    //         Checks if the current floor is an active mission destination of a given type and subtype.
    //
    //         r0: mission type
    //         r1: mission subtype
    //         return: bool
    //     - name: IsOutlawMonsterHouseFloor
    //       address:
    //         NA: 0x234928C
    //         EU: 0x2349E8C
    //       description: |-
    //         Checks if the current floor is a mission destination for a Monster House outlaw mission.
    //
    //         return: bool
    //     - name: IsGoldenChamber
    //       address:
    //         NA: 0x23492B0
    //         EU: 0x2349EB0
    //       description: |-
    //         Checks if the current floor is a Golden Chamber floor.
    //
    //         return: bool
    //     - name: IsLegendaryChallengeFloor
    //       address:
    //         NA: 0x23492D4
    //         EU: 0x2349ED4
    //       description: |-
    //         Checks if the current floor is a boss floor for a Legendary Challenge Letter mission.
    //
    //         return: bool
    //     - name: IsJirachiChallengeFloor
    //       address:
    //         NA: 0x2349314
    //         EU: 0x2349F14
    //       description: |-
    //         Checks if the current floor is the boss floor in Star Cave Pit for Jirachi's Challenge Letter mission.
    //
    //         return: bool
    //     - name: IsDestinationFloorWithMonster
    //       address:
    //         NA: 0x234934C
    //         EU: 0x2349F4C
    //       description: |-
    //         Checks if the current floor is a mission destination floor with a special monster.
    //
    //         See FloorHasMissionMonster for details.
    //
    //         return: bool
    //     - name: MissionTargetEnemyIsDefeated
    //       address:
    //         NA: 0x2349470
    //         EU: 0x234A070
    //       description: |-
    //         Checks if the target enemy of the mission on the current floor has been defeated.
    //
    //         return: bool
    //     - name: SetMissionTargetEnemyDefeated
    //       address:
    //         NA: 0x2349490
    //         EU: 0x234A090
    //       description: |-
    //         Set the flag for whether or not the target enemy of the current mission has been defeated.
    //
    //         r0: new flag value
    //     - name: IsDestinationFloorWithFixedRoom
    //       address:
    //         NA: 0x23494A4
    //         EU: 0x234A0A4
    //       description: |-
    //         Checks if the current floor is a mission destination floor with a fixed room.
    //
    //         The entire floor can be a fixed room layout, or it can just contain a Sealed Chamber.
    //
    //         return: bool
    //     - name: GetItemToRetrieve
    //       address:
    //         NA: 0x23494CC
    //         EU: 0x234A0CC
    //       description: |-
    //         Get the ID of the item that needs to be retrieve on the current floor for a mission, if one exists.
    //
    //         return: item ID
    //     - name: GetItemToDeliver
    //       address:
    //         NA: 0x23494F0
    //         EU: 0x234A0F0
    //       description: |-
    //         Get the ID of the item that needs to be delivered to a mission client on the current floor, if one exists.
    //
    //         return: item ID
    //     - name: GetSpecialTargetItem
    //       address:
    //         NA: 0x234951C
    //         EU: 0x234A11C
    //       description: |-
    //         Get the ID of the special target item for a Sealed Chamber or Treasure Memo mission on the current floor.
    //
    //         return: item ID
    //     - name: IsDestinationFloorWithItem
    //       address:
    //         NA: 0x2349564
    //         EU: 0x234A164
    //       description: |-
    //         Checks if the current floor is a mission destination floor with a special item.
    //
    //         This excludes missions involving taking an item from an outlaw.
    //
    //         return: bool
    //     - name: IsDestinationFloorWithHiddenOutlaw
    //       address:
    //         NA: 0x23495C4
    //         EU: 0x234A1C4
    //       description: |-
    //         Checks if the current floor is a mission destination floor with a "hidden outlaw" that behaves like a normal enemy.
    //
    //         return: bool
    //     - name: IsDestinationFloorWithFleeingOutlaw
    //       address:
    //         NA: 0x23495E8
    //         EU: 0x234A1E8
    //       description: |-
    //         Checks if the current floor is a mission destination floor with a "fleeing outlaw" that runs away.
    //
    //         return: bool
    //     - name: GetMissionTargetEnemy
    //       address:
    //         NA: 0x2349620
    //         EU: 0x234A220
    //       description: |-
    //         Get the monster ID of the target enemy to be defeated on the current floor for a mission, if one exists.
    //
    //         return: monster ID
    //     - name: GetMissionEnemyMinionGroup
    //       address:
    //         NA: 0x2349638
    //         EU: 0x234A238
    //       description: |-
    //         Get the monster ID of the specified minion group on the current floor for a mission, if it exists.
    //
    //         Note that a single minion group can correspond to multiple actual minions of the same species. There can be up to 2 minion groups.
    //
    //         r0: minion group index (0-indexed)
    //         return: monster ID
    //     - name: FloorHasMissionMonster
    //       address:
    //         NA: 0x2349748
    //         EU: 0x234A348
    //       description: |-
    //         Checks if a given floor is a mission destination with a special monster, either a target to rescue or an enemy to defeat.
    //
    //         Mission types with a monster on the destination floor:
    //         - Rescue client
    //         - Rescue target
    //         - Escort to target
    //         - Deliver item
    //         - Search for target
    //         - Take item from outlaw
    //         - Arrest outlaw
    //         - Challenge Request
    //
    //         r0: mission destination info pointer
    //         return: bool
}

/// Helper struct for emitting move and item effects.
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
    /// Inflicts the Burn status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `special_effect` - Flag to apply some special effect alongside the burn?
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `log_success` - Glag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_burn(
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

    /// Inflicts the Badly Poisoned status condition on a target monster if possible.
    ///
    /// Returns true if the target monster was affected.
    ///
    /// # Arguments
    /// * `attacker` - The monster that is trying to inflict this status.
    /// * `defender` - The monster that is being inflicted with this status.
    /// * `log_failure` - Flag to log a message to the dungeon message log on failure
    /// * `log_success` - Glag to only perform the check for inflicting without actually inflicting.
    pub fn try_inflict_bad_poison(
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

    /// Deals damage from a move or item used by an attacking monster on a defending monster.
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

    //     - name: AddExpSpecial
    //       address:
    //         NA: 0x230253C
    //         EU: 0x2302F68
    //       description: |-
    //         Adds to a monster's experience points, subject to experience boosting effects.
    //
    //         This function appears to be called only under special circumstances. Possibly when granting experience from damage (e.g., Joy Ribbon)?
    //
    //         Interestingly, the parameter in r0 isn't actually used. This might be a compiler optimization to avoid shuffling registers, since this function might be called alongside lots of other functions that have both the attacker and defender as the first two arguments.
    //
    //         r0: attacker pointer
    //         r1: defender pointer
    //         r2: base experience gain, before boosts
    //     - name: InflictSleepStatusSingle
    //       address:
    //         NA: 0x2311824
    //         EU: 0x2312284
    //       description: |-
    //         This is called by TryInflictSleepStatus.
    //
    //         r0: entity pointer
    //         r1: number of turns
    //     - name: TryInflictSleepStatus
    //       address:
    //         NA: 0x23118D8
    //         EU: 0x2312338
    //       description: |-
    //         Inflicts the Sleep status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of turns
    //         r3: flag to log a message on failure
    //     - name: TryInflictNightmareStatus
    //       address:
    //         NA: 0x2311C4C
    //         EU: 0x23126AC
    //       description: |-
    //         Inflicts the Nightmare status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of turns
    //     - name: TryInflictNappingStatus
    //       address:
    //         NA: 0x2311D60
    //         EU: 0x23127C0
    //       description: |-
    //         Inflicts the Napping status condition (from Rest) on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of turns
    //     - name: TryInflictYawningStatus
    //       address:
    //         NA: 0x2311E70
    //         EU: 0x23128D0
    //       description: |-
    //         Inflicts the Yawning status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of turns
    //     - name: TryInflictSleeplessStatus
    //       address:
    //         NA: 0x2311F80
    //         EU: 0x23129E0
    //       description: |-
    //         Inflicts the Sleepless status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //     - name: TryInflictPausedStatus
    //       address:
    //         NA: 0x231206C
    //         EU: 0x2312ACC
    //       description: |-
    //         Inflicts the Paused status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: ?
    //         r3: number of turns
    //         stack[0]: flag to log a message on failure
    //         stack[1]: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictInfatuatedStatus
    //       address:
    //         NA: 0x23121AC
    //         EU: 0x2312C0C
    //       description: |-
    //         Inflicts the Infatuated status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictBurnStatus
    //       address:
    //         NA: 0x2312338
    //         EU: 0x2312D98
    //       description: |-
    //         Inflicts the Burn status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to apply some special effect alongside the burn?
    //         r3: flag to log a message on failure
    //         stack[0]: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictBurnStatusWholeTeam
    //       address:
    //         NA: 0x2312618
    //         EU: 0x2313078
    //       description: |-
    //         Inflicts the Burn status condition on all team members if possible.
    //
    //         No params.
    //     - name: TryInflictPoisonedStatus
    //       address:
    //         NA: 0x2312664
    //         EU: 0x23130C4
    //       description: |-
    //         Inflicts the Poisoned status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictBadlyPoisonedStatus
    //       address:
    //         NA: 0x231293C
    //         EU: 0x231339C
    //       description: |-
    //         Inflicts the Badly Poisoned status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictFrozenStatus
    //       address:
    //         NA: 0x2312BF8
    //         EU: 0x2313658
    //       description: |-
    //         Inflicts the Frozen status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //     - name: TryInflictConstrictionStatus
    //       address:
    //         NA: 0x2312E20
    //         EU: 0x2313880
    //       description: |-
    //         Inflicts the Constriction status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: animation ID
    //         r3: flag to log a message on failure
    //     - name: TryInflictShadowHoldStatus
    //       address:
    //         NA: 0x2312F78
    //         EU: 0x23139D8
    //       description: |-
    //         Inflicts the Shadow Hold (AKA Immobilized) status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //     - name: TryInflictIngrainStatus
    //       address:
    //         NA: 0x2313130
    //         EU: 0x2313B90
    //       description: |-
    //         Inflicts the Ingrain status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //     - name: TryInflictWrappedStatus
    //       address:
    //         NA: 0x23131F4
    //         EU: 0x2313C54
    //       description: |-
    //         Inflicts the Wrapped status condition on a target monster if possible.
    //
    //         This also gives the user the Wrap status (Wrapped around foe).
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //     - name: TryInflictPetrifiedStatus
    //       address:
    //         NA: 0x231346C
    //         EU: 0x2313ECC
    //       description: |-
    //         Inflicts the Petrified status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //     - name: LowerOffensiveStat
    //       address:
    //         NA: 0x23135FC
    //         EU: 0x231405C
    //       description: |-
    //         Lowers the specified offensive stat on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: number of stages
    //         stack[0]: ?
    //         stack[1]: ?
    //     - name: LowerDefensiveStat
    //       address:
    //         NA: 0x2313814
    //         EU: 0x2314274
    //       description: |-
    //         Lowers the specified defensive stat on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: number of stages
    //         stack[0]: ?
    //         stack[1]: ?
    //     - name: BoostOffensiveStat
    //       address:
    //         NA: 0x231399C
    //         EU: 0x23143FC
    //       description: |-
    //         Boosts the specified offensive stat on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: number of stages
    //     - name: BoostDefensiveStat
    //       address:
    //         NA: 0x2313B08
    //         EU: 0x2314568
    //       description: |-
    //         Boosts the specified defensive stat on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: number of stages
    //     - name: ApplyOffensiveStatMultiplier
    //       address:
    //         NA: 0x2313D40
    //         EU: 0x23147A0
    //       description: |-
    //         Applies a multiplier to the specified offensive stat on the target monster.
    //
    //         This affects struct monster_stat_modifiers::offensive_multipliers, for moves like Charm and Memento.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: multiplier
    //         stack[0]: ?
    //     - name: ApplyDefensiveStatMultiplier
    //       address:
    //         NA: 0x2313F64
    //         EU: 0x23149C4
    //       description: |-
    //         Applies a multiplier to the specified defensive stat on the target monster.
    //
    //         This affects struct monster_stat_modifiers::defensive_multipliers, for moves like Screech.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: multiplier
    //         stack[0]: ?
    //     - name: BoostHitChanceStat
    //       address:
    //         NA: 0x23140E4
    //         EU: 0x2314B44
    //       description: |-
    //         Boosts the specified hit chance stat (accuracy or evasion) on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //     - name: LowerHitChanceStat
    //       address:
    //         NA: 0x231422C
    //         EU: 0x2314C8C
    //       description: |-
    //         Lowers the specified hit chance stat (accuracy or evasion) on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: ?
    //     - name: TryInflictCringeStatus
    //       address:
    //         NA: 0x23143E8
    //         EU: 0x2314E48
    //       description: |-
    //         Inflicts the Cringe status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictParalysisStatus
    //       address:
    //         NA: 0x2314544
    //         EU: 0x2314FA4
    //       description: |-
    //         Inflicts the Paralysis status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: BoostSpeed
    //       address:
    //         NA: 0x2314810
    //         EU: 0x2315270
    //       description: |-
    //         Boosts the speed of the target monster.
    //
    //         If the number of turns specified is 0, a random turn count will be selected using the default SPEED_BOOST_DURATION_RANGE.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of stages
    //         r3: number of turns
    //         stack[0]: flag to log a message on failure
    //     - name: BoostSpeedOneStage
    //       address:
    //         NA: 0x231493C
    //         EU: 0x231539C
    //       description: |-
    //         A wrapper around BoostSpeed with the number of stages set to 1.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of turns
    //         r3: flag to log a message on failure
    //     - name: LowerSpeed
    //       address:
    //         NA: 0x2314954
    //         EU: 0x23153B4
    //       description: |-
    //         Lowers the speed of the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: number of stages
    //         r3: flag to log a message on failure
    //     - name: TrySealMove
    //       address:
    //         NA: 0x2314ABC
    //         EU: 0x231551C
    //       description: |-
    //         Seals one of the target monster's moves. The move to be sealed is randomly selected.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         return: Whether or not a move was sealed
    //     - name: BoostOrLowerSpeed
    //       address:
    //         NA: 0x2314C2C
    //         EU: 0x231568C
    //       description: |-
    //         Randomly boosts or lowers the speed of the target monster by one stage with equal probability.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //     - name: ResetHitChanceStat
    //       address:
    //         NA: 0x2314C8C
    //         EU: 0x23156EC
    //       description: |-
    //         Resets the specified hit chance stat (accuracy or evasion) back to normal on the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: stat index
    //         r3: ?
    //     - name: TryActivateQuickFeet
    //       address:
    //         NA: 0x2314E1C
    //         EU: 0x231587C
    //       description: |-
    //         Activate the Quick Feet ability on the defender, if the monster has it and it's active.
    //
    //         r0: attacker pointer
    //         r1: defender pointer
    //         return: bool, whether or not the ability was activated
    //     - name: TryInflictConfusedStatus
    //       address:
    //         NA: 0x2314F38
    //         EU: 0x2315998
    //       description: |-
    //         Inflicts the Confused status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictCoweringStatus
    //       address:
    //         NA: 0x231516C
    //         EU: 0x2315CCC
    //       description: |-
    //         Inflicts the Cowering status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryIncreaseHp
    //       address:
    //         NA: 0x23152E4
    //         EU: 0x2315D44
    //       description: |-
    //         Restore HP and possibly boost max HP of the target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: HP to restore
    //         r3: max HP boost
    //         stack[0]: flag to log a message on failure
    //         return: Success flag
    //     - name: TryInflictLeechSeedStatus
    //       address:
    //         NA: 0x23157EC
    //         EU: 0x231624C
    //       description: |-
    //         Inflicts the Leech Seed status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: flag to log a message on failure
    //         r3: flag to only perform the check for inflicting without actually inflicting
    //         return: Whether or not the status could be inflicted
    //     - name: TryInflictDestinyBond
    //       address:
    //         NA: 0x2315A50
    //         EU: 0x23164B0
    //       description: |-
    //         Inflicts the Destiny Bond status condition on a target monster if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //     - name: RestoreMovePP
    //       address:
    //         NA: 0x2317C20
    //         EU: 0x2318680
    //       description: |-
    //         Restores the PP of all the target's moves by the specified amount.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: PP to restore
    //         r3: flag to suppress message logging
    //     - name: ApplyItemEffect
    //       address:
    //         NA: 0x231B68C
    //         EU: 0x231C0EC
    //       description: |-
    //         Seems to apply an item's effect via a giant switch statement?
    //
    //         r3: attacker pointer
    //         stack[0]: defender pointer
    //         stack[1]: thrown item pointer
    //         others: ?
    //     - name: ViolentSeedBoost
    //       address:
    //         NA: 0x231CE1C
    //         EU: 0x231D884
    //       description: |-
    //         Applies the Violent Seed boost to an entity.
    //
    //         r0: attacker pointer
    //         r1: defender pointer
    //     - name: ApplyGummiBoosts
    //       address:
    //         NA: 0x231D0C0
    //         EU: 0x231DB28
    //       description: |-
    //         Applies the IQ and possible stat boosts from eating a Gummi to the target monster.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: Gummi type ID
    //         r3: Stat boost amount, if a random stat boost occurs
    //     - name: TryPounce
    //       address:
    //         NA: 0x231FC20
    //         EU: 0x2320688
    //       description: |-
    //         Makes the target monster execute the Pounce action in a given direction if possible.
    //
    //         If the direction ID is 8, the target will pounce in the direction it's currently facing.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: direction ID
    //     - name: TryBlowAway
    //       address:
    //         NA: 0x231FDE0
    //         EU: 0x2320848
    //       description: |-
    //         Blows away the target monster in a given direction if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: direction ID
    //     - name: TryWarp
    //       address:
    //         NA: 0x2320D08
    //         EU: 0x2321770
    //       description: |-
    //         Makes the target monster warp if possible.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: warp type
    //         r3: position (if warp type is position-based)
    //     - name: DealDamageWithRecoil
    //       address:
    //         NA: 0x2327F34
    //         EU: 0x23289A0
    //       description: |-
    //         Deals damage from a move or item used by an attacking monster on a defending monster, and also deals recoil damage to the attacker.
    //
    //         r0: attacker pointer
    //         r1: defender pointer
    //         r2: move
    //         r3: item ID
    //         return: bool, whether or not damage was dealt
    //     - name: DealDamage
    //       address:
    //         NA: 0x2332B20
    //         EU: 0x2333560
    //       description: |-
    //         Deals damage from a move or item used by an attacking monster on a defending monster.
    //
    //         r0: attacker pointer
    //         r1: defender pointer
    //         r2: move
    //         r3: damage multiplier (as a Q24.8 fixed-point float)
    //         stack[0]: item ID
    //         return: amount of damage dealt
}

/// Builder for creating dungeon message log messages.
///
/// By default message will be shown 'quiet', meaning there will be no popup
/// shown when the message is logged. You can force a popup to be shown with [`Self::popup`],
/// but please also note that with some configurations, a popup will always be displayed, even
/// if [`Self::popup`] is not called. See the implementation for more details.
pub struct LogMessageBuilder<'a> {
    _lease: OverlayLoadLease<29>,
    popup: bool,
    check_user: bool,
    target_check_fainted: Option<&'a DungeonEntity>
}

impl<'a> CreatableWithLease<29> for LogMessageBuilder<'a> {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self {
            _lease: lease,
            popup: false,
            check_user: false,
            target_check_fainted: None
        }
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self._lease
    }
}

impl<'a> LogMessageBuilder<'a> {
    /// Show a message popup when the message is displayed.
    pub fn popup(&mut self) -> &mut Self {
        self.popup = true;
        self
    }

    /// Do not show the message if the user is fainted.
    ///
    /// # Note
    /// [`Self::target_check_fainted`] will take precedence over this, both can not be active
    /// at the same time.
    pub fn check_user_fainted(&'a mut self) -> &'a mut Self {
        self.check_user = true;
        self
    }

    // Do not show the message if the target is fainted and an unknown check
    // regarding the user passes.
    pub fn target_check_fainted(&'a mut self, target: &'a DungeonEntity) -> &'a mut Self {
        self.target_check_fainted = Some(target);
        self
    }

    /// Replaces instances of a given placeholder tag by the string representation of the given entity.
    /// Concretely this means that any occurrences of `\[string:<string_id>\]` will be replaced by the
    /// name of the given entity.
    /// Example: If use pass `string_id` with 1, it will replace all occurrences of `\[string:1\]`.
    ///
    /// # Note
    /// As a performance optimization this will immediately reserve that string with the game when
    /// called. This can have weird effects if you expect to show the message built by this builder
    /// at a later time.
    pub fn string(&mut self, string_id: u16, entity: &DungeonEntity) -> &mut Self {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::SubstitutePlaceholderStringTags(
            string_id as c_int, force_mut_ptr!(entity), 0
        ) }
        self
    }

    /// Writes a log entry using the message with the given message ID.
    pub fn log_msg(&mut self, user: &DungeonEntity, message_id: i32) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            match (self.popup, self.check_user, self.target_check_fainted) {
                (false, false, None) => ffi::LogMessageByIdQuiet(force_mut_ptr!(user), message_id),
                (_,     true,  None) => ffi::LogMessageByIdWithPopupCheckUser(force_mut_ptr!(user), message_id),
                (false, _,     Some(target)) => ffi::LogMessageByIdQuietCheckUserTarget(force_mut_ptr!(user), force_mut_ptr!(target), message_id,),
                (true,  false, None) => ffi::LogMessageByIdWithPopup(force_mut_ptr!(user), message_id),
                (true,  _,     Some(target)) => ffi::LogMessageByIdWithPopupCheckUserTarget(force_mut_ptr!(user), force_mut_ptr!(target), message_id,),
            }
        }
    }

    pub fn log_str<S: AsRef<str> + Debug>(&mut self, user: &DungeonEntity, message: S) {
        self.log_cstr(user, str_to_cstring(message))
    }

    pub fn log_cstr<S: AsRef<CStr>>(&mut self, user: &DungeonEntity, message: S) {
        let message = message.as_ref().as_ptr() as *const c_char;
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            match (self.popup, self.check_user, self.target_check_fainted) {
                (false, false, None) => ffi::LogMessageQuiet(force_mut_ptr!(user), message),
                (_,     true,  None) => ffi::LogMessageWithPopupCheckUser(force_mut_ptr!(user), message),
                (true,  false, None) => ffi::LogMessageWithPopup(force_mut_ptr!(user), message),
                (_,     _,     Some(target)) => ffi::LogMessageWithPopupCheckUserTarget(force_mut_ptr!(user), force_mut_ptr!(target), message,),
            }
        }
    }
}


/// Helper struct for dungeon RNG.
pub struct DungeonRng(OverlayLoadLease<29>);

impl CreatableWithLease<29> for DungeonRng {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self(lease)
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self.0
    }
}

impl DungeonRng {
    //     - name: GenerateDungeonRngSeed
    //       address:
    //         NA: 0x22EA980
    //         EU: 0x22EB330
    //       description: |-
    //         Generates a seed with which to initialize the dungeon PRNG.
    //
    //         The seed is calculated by starting with a different seed, the "preseed" x0 (defaults to 1, but can be set by other functions). The preseed is iterated twice with the same recurrence relation used in the primary LCG to generate two pseudorandom 32-bit numbers x1 and x2. The output seed is then computed as
    //           seed = (x1 & 0xFF0000) | (x2 >> 0x10) | 1
    //         The value x1 is then saved as the new preseed.
    //
    //         This method of seeding the dungeon PRNG appears to be used only sometimes, depending on certain flags in the data for a given dungeon.
    //
    //         return: RNG seed
    //     - name: GetDungeonRngPreseed
    //       address:
    //         NA: 0x22EA9CC
    //         EU: 0x22EB37C
    //       description: |-
    //         Gets the current preseed stored in the global dungeon PRNG state. See GenerateDungeonRngSeed for more information.
    //
    //         return: current dungeon RNG preseed
    //     - name: SetDungeonRngPreseed
    //       address:
    //         NA: 0x22EA9DC
    //         EU: 0x22EB38C
    //       description: |-
    //         Sets the preseed in the global dungeon PRNG state. See GenerateDungeonRngSeed for more information.
    //
    //         r0: preseed
    //     - name: InitDungeonRng
    //       address:
    //         NA: 0x22EA9EC
    //         EU: 0x22EB39C
    //       description: |-
    //         Initialize (or reinitialize) the dungeon PRNG with a given seed. The primary LCG and the five secondary LCGs are initialized jointly, and with the same seed.
    //
    //         r0: seed
    //     - name: DungeonRand16Bit
    //       address:
    //         NA: 0x22EAA20
    //         EU: 0x22EB3D0
    //       description: |-
    //         Computes a pseudorandom 16-bit integer using the dungeon PRNG.
    //
    //         Note that the dungeon PRNG is only used in dungeon mode (as evidenced by these functions being in overlay 29). The game uses another lower-quality PRNG (see arm9.yml) for other needs.
    //
    //         Random numbers are generated with a linear congruential generator (LCG). The game actually maintains 6 separate sequences that can be used for generation: a primary LCG and 5 secondary LCGs. The generator used depends on parameters set on the global PRNG state.
    //
    //         All dungeon LCGs have a modulus of 2^32 and a multiplier of 1566083941 (see DUNGEON_PRNG_LCG_MULTIPLIER). The primary LCG uses an increment of 1, while the secondary LCGs use an increment of 2531011 (see DUNGEON_PRNG_LCG_INCREMENT_SECONDARY). So, for example, the primary LCG uses the recurrence relation:
    //           x = (1566083941*x_prev + 1) % 2^32
    //
    //         Since the dungeon LCGs generate 32-bit integers rather than 16-bit, the primary LCG yields 16-bit values by taking the upper 16 bits of the computed 32-bit value. The secondary LCGs yield 16-bit values by taking the lower 16 bits of the computed 32-bit value.
    //
    //         All of the dungeon LCGs have a hard-coded default seed of 1, but in practice the seed is set with a call to InitDungeonRng during dungeon initialization.
    //
    //         return: pseudorandom int on the interval [0, 65535]
    //     - name: DungeonRandInt
    //       address:
    //         NA: 0x22EAA98
    //         EU: 0x22EB448
    //       description: |-
    //         Compute a pseudorandom integer under a given maximum value using the dungeon PRNG.
    //
    //         r0: high
    //         return: pseudorandom integer on the interval [0, high - 1]
    //     - name: DungeonRandRange
    //       address:
    //         NA: 0x22EAAC0
    //         EU: 0x22EB470
    //       description: |-
    //         Compute a pseudorandom value between two integers using the dungeon PRNG.
    //
    //         r0: x
    //         r1: y
    //         return: pseudorandom integer on the interval [min(x, y), max(x, y) - 1]
    //     - name: DungeonRandOutcome
    //       address:
    //         NA:
    //           - 0x22EAB20
    //           - 0x22EAB50
    //       description: |-
    //         Returns the result of a possibly biased coin flip (a Bernoulli random variable) with some success probability p, using the dungeon PRNG.
    //
    //         r0: success percentage (100*p)
    //         return: true with probability p, false with probability (1-p)
    //     - name: DungeonRngUnsetSecondary
    //       address:
    //         NA: 0x22EAC34
    //         EU: 0x22EB5E4
    //       description: |-
    //         Sets the dungeon PRNG to use the primary LCG for subsequent random number generation, and also resets the secondary LCG index back to 0.
    //
    //         Similar to DungeonRngSetPrimary, but DungeonRngSetPrimary doesn't modify the secondary LCG index if it was already set to something other than 0.
    //
    //         No params.
    //     - name: DungeonRngSetSecondary
    //       address:
    //         NA: 0x22EAC4C
    //         EU: 0x22EB5FC
    //       description: |-
    //         Sets the dungeon PRNG to use one of the 5 secondary LCGs for subsequent random number generation.
    //
    //         r0: secondary LCG index
    //     - name: DungeonRngSetPrimary
    //       address:
    //         NA: 0x22EAC64
    //       description: |-
    //         Sets the dungeon PRNG to use the primary LCG for subsequent random number generation.
    //
    //         No params.
    //     - name: DungeonRandOutcomeUserTargetInteraction
    //       address:
    //         NA: 0x2324934
    //         EU: 0x232539C
    //       description: |-
    //         Like DungeonRandOutcome, but specifically for user-target interactions.
    //
    //         This modifies the underlying random process depending on factors like Serene Grace, and whether or not either entity has fainted.
    //
    //         r0: user entity pointer
    //         r1: target entity pointer
    //         r2: base success percentage (100*p). 0 is treated specially and guarantees success.
    //     - name: DungeonRandOutcomeUserAction
    //       address:
    //         NA: 0x2324A20
    //         EU: 0x2325488
    //       description: |-
    //         Like DungeonRandOutcome, but specifically for user actions.
    //
    //         This modifies the underlying random process to factor in Serene Grace (and checks whether the user is a valid entity).
    //
    //         r0: entity pointer
    //         r1: base success percentage (100*p). 0 is treated specially and guarantees success.

    /// Compute a pseudorandom integer on the interval [0, 100) using the dungeon PRNG.
    pub fn rand100(&self) -> u32 {
        unsafe { ffi::DungeonRand100() }
    }

}

// Misc dungeon functions.

/// Seems to zero some damage description struct, which is output by the damage
/// calculation function.
pub unsafe fn reset_damage_desc(damage_desc: *mut ffi::undefined4, _ov29: &OverlayLoadLease<29>) {
    ffi::ResetDamageDesc(damage_desc);
}

/// [`DungeonMonsterRef::calc_damage`] seems to use scratch space of
/// some kind, which this function zeroes.
pub unsafe fn reset_damage_calc_scratch_space(_ov29: &OverlayLoadLease<29>) {
    ffi::ResetDamageCalcScratchSpace();
}

/// This changes the palettes of windows in both screens to an appropriate value depending on
/// the playthrough.
///
/// If you're in a special episode, they turn green , otherwise, they turn blue or pink depending
/// on your character's sex
///
pub fn set_both_screens_window_color_to_default(_ov29: &OverlayLoadLease<29>) {
    unsafe { ffi::SetBothScreensWindowColorToDefault() }
}
