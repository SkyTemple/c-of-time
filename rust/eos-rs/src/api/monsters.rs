//! Structs and functions to interact with the data of monsters in a general context.

use crate::ffi;
use alloc::vec::Vec;

/// A monster species ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::get`] method to get instances of this.
pub type MonsterSpeciesId = ffi::monster_id;
impl Copy for MonsterSpeciesId {}

/// This impl provides general metadata about monster species in the game.
impl MonsterSpeciesId {
    /// Returns the ID struct for the monster species with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing monster species),
    /// otherwise this is UB.
    pub const unsafe fn get(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this monster.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Returns the gender field of the monster.
    pub fn gender(&self) -> u8 {
        // TODO: Enum.
        unsafe { ffi::GetMonsterGender(*self) }
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
}
