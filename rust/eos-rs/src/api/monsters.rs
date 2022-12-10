//! Structs and functions to interact with the data of monsters in a general context.

use crate::api::enums::MonsterGender;
use crate::ffi;
use crate::ffi::GetLowKickMultiplier;
use alloc::vec::Vec;
use fixed::types::I24F8;

/// A monster species ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
pub type MonsterSpeciesId = ffi::monster_id;
impl Copy for MonsterSpeciesId {}

/// This impl provides general metadata about monster species in the game.
impl MonsterSpeciesId {
    /// Returns the ID struct for the monster species with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing monster species),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this monster.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Checks if the specified monster ID corresponds to any of the pokémon that have multiple
    /// forms and returns the ID of the base form if so. If it doesn't, the same ID is returned.
    ///
    /// Some of the monsters included in the check are Unown, Cherrim and Deoxys.
    pub fn base_form(&self) -> MonsterSpeciesId {
        unsafe { ffi::GetBaseForm(*self) }
    }

    /// Returns the ID of the first form of the specified monster if the specified ID corresponds
    /// to a secondary form with female gender and the first form has male gender.
    ///
    /// If those conditions don't meet, returns the same ID unchanged.
    pub fn base_gender_form(&self) -> MonsterSpeciesId {
        unsafe { ffi::FemaleToMaleForm(*self) }
    }

    /// Returns the gender field of the monster.
    pub fn gender(&self) -> MonsterGender {
        // TODO: Enum.
        unsafe { ffi::GetMonsterGender(*self).into() }
    }

    /// Returns the sprite size of the monster. If the size is between 1 and 6,
    /// 6 will be returned.
    pub fn sprite_size(&self) -> u8 {
        // TODO: Enum.
        unsafe { ffi::GetSpriteSize(*self) }
    }

    /// Returns the sprite file size of the monster.
    pub fn sprite_file_size(&self) -> u8 {
        unsafe { ffi::GetSpriteFileSize(*self) }
    }

    /// Returns the pre-evolution id of a monster given its ID.
    pub fn pre_evolution(&self) -> Self {
        unsafe { ffi::GetMonsterPreEvolution(*self) }
    }

    /// Returns a list of all the possible evolutions.
    ///
    /// This will panic if the monster has more than 32 evolutions.
    ///
    /// # Arguments
    /// * `ignore_sprite_size` - True to skip the check that prevents returning monsters with a
    ///                          different sprite size than the current one.
    /// * `include_shedinja`   - True to skip the check that prevents Shedinja from being counted
    ///                          as a potential evolution.
    pub fn evolutions(&self, ignore_sprite_size: bool, include_shedinja: bool) -> Vec<Self> {
        const MAX_EVOLUTIONS: i32 = 32;
        let mut output_list = [ffi::monster_id(0); MAX_EVOLUTIONS as usize];
        let count = unsafe {
            ffi::GetEvolutions(
                *self,
                output_list.as_mut_ptr(),
                ignore_sprite_size as ffi::bool_,
                include_shedinja as ffi::bool_,
            )
        };
        if count > MAX_EVOLUTIONS {
            // uh oh. Memory is corrupted now, so time to bail.
            // THIS PANIC IS NOT UNWIND SAFE (not that it matters).
            panic!("Monster has more than {} evolutions.", MAX_EVOLUTIONS);
        }
        output_list.into_iter().take(count as usize).collect()
    }

    /// Checks if this is an Unown.
    pub fn is_unown(&self) -> bool {
        unsafe { ffi::IsUnown(*self) > 0 }
    }

    /// Checks if this is a Shaymin.
    pub fn is_shaymin(&self) -> bool {
        unsafe { ffi::IsShaymin(*self) > 0 }
    }

    /// Checks if this is a Castform.
    pub fn is_castform(&self) -> bool {
        unsafe { ffi::IsCastform(*self) > 0 }
    }

    /// Checks if this is a Cherrim.
    pub fn is_cherrim(&self) -> bool {
        unsafe { ffi::IsCherrim(*self) > 0 }
    }

    /// Checks if this is a Deoxys.
    pub fn is_deoxys(&self) -> bool {
        unsafe { ffi::IsDeoxys(*self) > 0 }
    }

    /// Returns the flag that determines if a monster can move in dungeons.
    pub fn get_can_move_flag(&self) -> bool {
        unsafe { ffi::GetCanMoveFlag(*self) > 0 }
    }

    /// Checks if this monster is contained in the [`ffi::MISSION_BANNED_MONSTERS`] array.
    /// The function converts the ID by calling [`Self::base_form`] and
    /// [`Self::base_gender_form`] first.
    pub fn is_mission_allowed(&self) -> bool {
        unsafe { ffi::IsMonsterMissionAllowed(*self) > 0 }
    }

    /// Checks if the specified monster should be allowed to be part of a mission (probably as the
    /// client or the target), accounting for the progress on the story.
    ///
    /// If `PERFORMANCE_PROGRESS_FLAG[9]` is true, the function returns true.
    /// If it isn't, the function checks if the specified monster is contained in the
    /// [`ffi::MISSION_BANNED_STORY_MONSTERS`] array, or if it corresponds to the ID of the player
    /// or the  partner.
    ///
    /// The function converts the ID by calling [`Self::base_form`] and [`Self::base_gender_form`]
    /// first.
    pub fn is_mission_allowed_story(&self) -> bool {
        unsafe { ffi::IsMonsterMissionAllowedStory(*self) > 0 }
    }

    /// Returns whether this monster can be used (probably as the client or as the target) when
    /// generating a mission.
    ///
    /// Excluded monsters include those that haven't been fought in dungeons yet, the second form
    /// of certain monsters and, if `PERFORMANCE_PROGRESS_FLAG[9]` is 0, monsters in
    /// [`ffi::MISSION_BANNED_MONSTERS`, the species of the player and the species of the partner.
    ///
    pub fn can_be_used_for_mission(
        &self,
        exclude_monsters_in_mission_banned_monsters: bool,
    ) -> bool {
        unsafe {
            ffi::CanMonsterBeUsedForMission(
                *self,
                exclude_monsters_in_mission_banned_monsters as ffi::bool_,
            ) > 0
        }
    }

    /// Gets the Low Kick (and Grass Knot) damage multiplier for the given species.
    pub fn get_low_kick_multiplier(&self) -> I24F8 {
        unsafe { I24F8::from_num(GetLowKickMultiplier(*self)) }
    }
}

impl From<MonsterSpeciesId> for u32 {
    fn from(v: MonsterSpeciesId) -> Self {
        v.0
    }
}
