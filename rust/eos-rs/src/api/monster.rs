//! Structs and functions to interact with the data of monsters in a general context.

use crate::api::objects::monster_catalog;
use crate::ffi;
use alloc::vec::Vec;

/// Metadata of a monster species.
///
/// This struct provides general metadata about monster species in the game.
pub struct MonsterSpeciesInfo(monster_catalog::Type);

impl MonsterSpeciesInfo {
    /// Returns the info struct for the monster species with the given ID.
    ///
    /// The caller should make sure, the ID is valid (refers to an existing monster species),
    /// otherwise the data returned by methods of this struct will be invalid and undefined.
    pub fn get(monster_idx: monster_catalog::Type) -> Self {
        Self(monster_idx)
    }

    /// Returns the ID of this monster.
    pub fn id(&self) -> monster_catalog::Type {
        self.0
    }

    /// Returns the gender field of the monster.
    pub fn gender(&self) -> u8 {
        // TODO: Enum.
        unsafe { ffi::GetMonsterGender(self.0) }
    }

    /// Returns the sprite size of the monster. If the size is between 1 and 6,
    /// 6 will be returned.
    pub fn sprite_size(&self) -> u8 {
        // TODO: Enum.
        unsafe { ffi::GetSpriteSize(self.0) }
    }

    /// Returns the sprite file size of the monster.
    pub fn sprite_file_size(&self) -> u8 {
        unsafe { ffi::GetSpriteFileSize(self.0) }
    }

    /// Returns the pre-evolution id of a monster given its ID.
    pub fn pre_evolution(&self) -> MonsterSpeciesInfo {
        unsafe { Self::get(ffi::GetMonsterPreEvolution(self.0)) }
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
    pub fn evolutions(
        &self,
        ignore_sprite_size: bool,
        include_shedinja: bool,
    ) -> Vec<MonsterSpeciesInfo> {
        unsafe {
            const MAX_EVOLUTIONS: i32 = 32;
            let mut output_list = [0; MAX_EVOLUTIONS as usize];
            let count = ffi::GetEvolutions(
                self.0,
                output_list.as_mut_ptr(),
                ignore_sprite_size as ffi::bool_,
                include_shedinja as ffi::bool_,
            );
            if count >= MAX_EVOLUTIONS {
                // uh oh. Memory is corrupted now, so time to bail.
                // THIS PANIC IS NOT UNWIND SAFE (not that it matters).
                panic!("Monster has more than {} evolutions.", MAX_EVOLUTIONS);
            }
            output_list
                .into_iter()
                .take(count as usize)
                .map(MonsterSpeciesInfo)
                .collect()
        }
    }
}
