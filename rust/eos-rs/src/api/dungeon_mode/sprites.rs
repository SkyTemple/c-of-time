use crate::api::monsters::MonsterSpeciesId;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Helper struct for dealing with sprite data in dungeon mode.
///
/// To get an instance of this, use [`crate::api::dungeon_mode::GlobalDungeonData::sprites`].
pub struct DungeonSpriteHandler<'a>(pub(crate) &'a OverlayLoadLease<29>);

impl<'a> DungeonSpriteHandler<'a> {
    /// Loads the sprites of monsters that appear on the current floor because of a mission,
    /// if applicable.
    ///
    /// This includes monsters to be rescued, outlaws and its minions.
    pub fn load_mission_monster_sprites(&mut self) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::LoadMissionMonsterSprites() }
    }

    /// Gets the sprite index of the specified monster on this floor
    pub fn get_monster_sprite_index(&self, monster_idx: MonsterSpeciesId) -> u16 {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetSpriteIndex(monster_idx) }
    }

    /// Loads the sprite of the specified monster to use it in a dungeon.
    ///
    /// # Safety
    /// The caller must make sure the undefined params are valid for this function.
    pub unsafe fn load_monster_sprite(
        &mut self,
        monster_id: MonsterSpeciesId,
        param_2: ffi::undefined,
    ) {
        ffi::LoadMonsterSprite(monster_id, param_2)
    }

    /// Checks Castform and Cherrim
    ///
    /// Note: unverified, ported from Irdkwia's notes
    pub fn get_total_sprite_file_size(&self, monster_id: MonsterSpeciesId) -> i32 {
        unsafe { ffi::GetTotalSpriteFileSize(monster_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn store_sprite_file_index_both_genders(
        &mut self,
        monster_id: MonsterSpeciesId,
        file_id: i32,
    ) {
        unsafe { ffi::StoreSpriteFileIndexBothGenders(monster_id, file_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn swap_monster_wan_file_index(&mut self, src_id: i32, dst_id: i32) {
        unsafe { ffi::SwapMonsterWanFileIndex(src_id, dst_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn delete_monster_sprite_file(&mut self, monster_id: MonsterSpeciesId) {
        unsafe { ffi::DeleteMonsterSpriteFile(monster_id) }
    }

    /// Note: unverified, ported from Irdkwia's notes
    pub fn delete_all_monster_sprite_files(&mut self) {
        unsafe { ffi::DeleteAllMonsterSpriteFiles() }
    }
}
