//! Traits, structs and functions related to ground mode.

#[allow(unused_imports)] // for easier reference in the docs of script_special_process_call
use crate::api::gameplay;
#[allow(unused_imports)] // for easier reference in the docs of script_special_process_call
use crate::api::objects::special_process_catalog::*;
use crate::api::objects::*;
use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::ffi;

/// Misc. and general ground mode functions, guarded by this struct.
pub struct GroundModeContext(OverlayLoadLease<11>);

impl CreatableWithLease<11> for GroundModeContext {
    fn _create(lease: OverlayLoadLease<11>) -> Self {
        Self(lease)
    }

    fn lease(&self) -> &OverlayLoadLease<11> {
        &self.0
    }
}

impl GroundModeContext {
    /// Processes calls to the [`script_opcode_catalog::OPCODE_PROCESS_SPECIAL`] script opcode.
    ///
    /// Returns result value of the special process if it has one, otherwise 0.
    ///
    /// Some built-in special processes include:
    ///
    /// - [`SPECIAL_PROC_KEY_WAIT_INIT`] : [`gameplay::key_wait_init`]
    /// - [`SPECIAL_PROC_IS_BAG_FULL`] : [`gameplay::is_bag_full`]
    /// - [`SPECIAL_PROC_COUNT_ITEM_TYPE_IN_BAG`] : [`gameplay::count_item_type_in_bag`]
    /// - [`SPECIAL_PROC_ADD_ITEM_TO_BAG`] : [`gameplay::add_item_to_bag`]
    /// - [`SPECIAL_PROC_0x39`] : [`gameplay::is_storage_full`]
    /// - [`SPECIAL_PROC_COUNT_ITEM_TYPE_IN_STORAGE`] : [`gameplay::count_item_type_in_storage`]
    /// - [`SPECIAL_PROC_0x2A`] : [`gameplay::remove_items_type_in_storage`]
    /// - [`SPECIAL_PROC_ADD_ITEM_TO_STORAGE`] : [`gameplay::add_item_to_storage`]
    /// - [`SPECIAL_PROC_INIT_MAIN_TEAM_AFTER_QUIZ`] : [`gameplay::init_main_team_after_quiz`]
    /// - [`SPECIAL_PROC_0x3`] : [`gameplay::script_special_process_3`]
    /// - [`SPECIAL_PROC_0x4`] : [`gameplay::script_special_process_4`]
    /// - [`SPECIAL_PROC_0x3A`] : [`gameplay::AdventureLog::increment_number_dungeons_cleared`]
    /// - [`SPECIAL_PROC_0x3B`] : [`gameplay::AdventureLog::increment_number_of_big_treasure_wins`]
    /// - [`SPECIAL_PROC_SEND_SKY_GIFT_TO_GUILDMASTER`] : [`gameplay::AdventureLog::increment_number_of_gifts_sent`]
    /// - [`SPECIAL_PROC_SET_TEAM_SETUP_HERO_AND_PARTNER_ONLY`] : [`gameplay::set_team_setup`]
    /// - [`SPECIAL_PROC_SET_TEAM_SETUP_HERO_ONLY`] : [`gameplay::set_team_setup`]
    /// - [`SPECIAL_PROC_IS_TEAM_SETUP_SOLO`] : [`gameplay::count_party_members`]
    /// - [`SPECIAL_PROC_GET_SOS_MAIL_COUNT`] : [`gameplay::get_sos_mail_count`]
    /// - [`SPECIAL_PROC_DUNGEON_HAD_REQUEST_DONE`] : [`gameplay::dungeon_had_request_done`]
    /// - [`SPECIAL_PROC_0x3D`] : [`gameplay::script_special_process_x3d`]
    /// - [`SPECIAL_PROC_0x3E`] : [`gameplay::script_special_process_x3e`]
    /// - [`SPECIAL_PROC_0x17`] : [`gameplay::script_special_process_x17`]
    /// - [`SPECIAL_PROC_COUNT_TABLE_ITEM_TYPE_IN_BAG`] : [`gameplay::item_at_table_idx`]
    /// - [`SPECIAL_PROC_PREPARE_MENU_ACCEPT_TEAM_MEMBER`] : [`Self::prepare_menu_accept_team_member`]
    /// - [`SPECIAL_PROC_INIT_RANDOM_NPC_JOBS`] : [`Self::init_random_npc_jobs`]
    /// - [`SPECIAL_PROC_GET_RANDOM_NPC_JOB_TYPE`] : [`Self::get_random_npc_job_type`]
    /// - [`SPECIAL_PROC_GET_RANDOM_NPC_JOB_SUBTYPE`] : [`Self::get_random_npc_job_subtype`]
    /// - [`SPECIAL_PROC_GET_RANDOM_NPC_JOB_STILL_AVAILABLE`] : [`Self::is_random_npc_job_still_available`]
    /// - [`SPECIAL_PROC_ACCEPT_RANDOM_NPC_JOB`] : [`Self::accept_random_npc_job`]
    /// - [`SPECIAL_PROC_RETURN_DUNGEON`] : [`Self::return_dungeon`]
    /// - [`SPECIAL_PROC_NEXT_DAY`] : [`Self::next_day`]
    /// - [`SPECIAL_PROC_JUMP_TO_TITLE_SCREEN`] : [`Self::jump_to_title_screen`]
    /// - [`SPECIAL_PROC_0x1A`] : [`Self::jump_to_title_screen`]
    /// - [`SPECIAL_PROC_RETURN_TO_TITLE_SCREEN`] : [`Self::return_to_title_screen`]
    /// - [`SPECIAL_PROC_0x16`] : [`Self::script_special_process_x16`]
    /// - [`SPECIAL_PROC_STATUS_UPDATE`] : [`Self::status_update`]
    ///
    /// # Arguments
    /// * `param_1` - some struct containing a callback of some sort, only used for special process ID 18
    /// * `id`      - special process ID
    /// * `arg1`    - first argument, if relevant? Probably corresponds to the second parameter of
    ///               [`script_opcode_catalog::OPCODE_PROCESS_SPECIAL`]
    /// * `arg2`    - second argument, if relevant? Probably corresponds to the third parameter of
    ///               [`script_opcode_catalog::OPCODE_PROCESS_SPECIAL`]
    pub fn script_special_process_call(
        &mut self,
        param_1: &mut ffi::undefined4,
        id: special_process_catalog::Type,
        arg1: i32,
        arg2: i32,
    ) -> i32 {
        unsafe { ffi::ScriptSpecialProcessCall(param_1, id, arg1, arg2) }
    }

    /// Returns an entry from RECRUITMENT_TABLE_SPECIES.
    ///
    /// # Safety
    /// This indexes without doing bounds checking.
    pub unsafe fn get_special_recruitment_species(&self, index: i32) -> monster_catalog::Type {
        ffi::GetSpecialRecruitmentSpecies(index)
    }

    /// Implements SPECIAL_PROC_PREPARE_MENU_ACCEPT_TEAM_MEMBER.
    ///
    /// `idx` is an index into the RECRUITMENT_TABLE_SPECIES.
    pub fn prepare_menu_accept_team_member(&mut self, idx: i32) {
        unsafe { ffi::PrepareMenuAcceptTeamMember(idx) }
    }

    /// Implements SPECIAL_PROC_INIT_RANDOM_NPC_JOBS.
    ///
    /// # Arguments
    /// * `job_type` - job type? 0 is a random NPC job, 1 is a bottle mission
    /// * `param_2`  - ???
    pub fn init_random_npc_jobs(&mut self, job_type: i32, param_2: ffi::undefined2) {
        unsafe { ffi::InitRandomNpcJobs(job_type, param_2) }
    }

    /// Implements SPECIAL_PROC_GET_RANDOM_NPC_JOB_TYPE.
    pub fn get_random_npc_job_type(&self) -> i32 {
        unsafe { ffi::GetRandomNpcJobType() }
    }

    /// Implements SPECIAL_PROC_GET_RANDOM_NPC_JOB_SUBTYPE.
    pub fn get_random_npc_job_subtype(&self) -> i32 {
        unsafe { ffi::GetRandomNpcJobSubtype() }
    }

    /// Implements SPECIAL_PROC_GET_RANDOM_NPC_JOB_STILL_AVAILABLE.
    pub fn is_random_npc_job_still_available(&self) -> bool {
        unsafe { ffi::GetRandomNpcJobStillAvailable() > 0 }
    }

    /// Implements SPECIAL_PROC_ACCEPT_RANDOM_NPC_JOB.
    pub fn accept_random_npc_job(&mut self) -> bool {
        unsafe { ffi::AcceptRandomNpcJob() > 0 }
    }

    /// Implements SPECIAL_PROC_RETURN_DUNGEON.
    pub fn return_dungeon(&mut self) {
        unsafe { ffi::GroundMainReturnDungeon() }
    }

    /// Implements SPECIAL_PROC_NEXT_DAY.
    pub fn next_day(&mut self) {
        unsafe { ffi::GroundMainNextDay() }
    }

    /// Fades the screen out and throws the player to the title screen.
    ///
    /// Implements SPECIAL_PROC_JUMP_TO_TITLE_SCREEN and SPECIAL_PROC_0x1A.
    ///
    /// `arg` is argument value for SPECIAL_PROC_JUMP_TO_TITLE_SCREEN and -1
    /// for SPECIAL_PROC_0x1A. It is probably the screen fade out time...?
    pub fn jump_to_title_screen(&mut self, arg: i32) -> bool {
        unsafe { ffi::JumpToTitleScreen(arg) > 0 }
    }

    /// Fades the screen out and throws the player to the title screen.
    ///
    /// Implements SPECIAL_PROC_RETURN_TO_TITLE_SCREEN.
    pub fn return_to_title_screen(&mut self, fade_duration: u32) -> bool {
        unsafe { ffi::ReturnToTitleScreen(fade_duration) > 0 }
    }

    /// Implements SPECIAL_PROC_0x16.
    pub fn script_special_process_x16(&mut self, param_1: bool) {
        unsafe { ffi::ScriptSpecialProcess0x16(param_1 as ffi::bool_) }
    }

    /// Implements SPECIAL_PROC_STATUS_UPDATE.
    pub fn status_update(&mut self) {
        unsafe { ffi::StatusUpdate() }
    }

    /// Returns the memory allocation arena for ground mode.
    ///
    /// You can use this with [`crate::allocation::EoSCustomAllocator`].
    ///
    /// # Parameters
    /// * `size` - initial memory arena pointer, or null.
    /// * `flags` - `MemAlloc` flags.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_alloc_arena_ground(
        &self,
        arena: *mut ffi::mem_arena,
        flags: u32,
    ) -> *mut ffi::mem_arena {
        unsafe { ffi::GetAllocArenaGround(arena, flags) }
    }

    /// Returns the memory freeing arena for ground mode.
    ///
    /// You can use this with [`crate::allocation::EoSCustomAllocator`].
    ///
    /// # Parameters
    /// * `size` - initial memory arena pointer, or null.
    /// * `flags` - `MemAlloc` flags.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_free_arena_ground(
        &self,
        arena: *mut ffi::mem_arena,
        flags: u32,
    ) -> *mut ffi::mem_arena {
        unsafe { ffi::GetFreeArenaGround(arena, flags) }
    }
}
