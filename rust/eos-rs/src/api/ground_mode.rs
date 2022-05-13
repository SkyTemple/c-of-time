//! Traits, structs and functions related to ground mode.

use crate::api::objects::monster_catalog;
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

//   functions:
//     - name: ScriptSpecialProcessCall
//       address:
//         NA: 0x22E7118
//         EU: 0x22E7A58
//       description: |-
//         Processes calls to the OPCODE_PROCESS_SPECIAL script opcode.
//
//         r0: some struct containing a callback of some sort, only used for special process ID 18
//         r1: special process ID
//         r2: first argument, if relevant? Probably corresponds to the second parameter of OPCODE_PROCESS_SPECIAL
//         r3: second argument, if relevant? Probably corresponds to the third parameter of OPCODE_PROCESS_SPECIAL
//         return: return value of the special process if it has one, otherwise 0

impl GroundModeContext {
    /// Returns an entry from RECRUITMENT_TABLE_SPECIES.
    ///
    /// # Safety
    /// This indexes without doing bounds checking.
    pub unsafe fn get_special_recruitment_species(&self, index: i32) -> monster_catalog::Type {
        ffi::GetSpecialRecruitmentSpecies(index)
    }
}

//     - name: PrepareMenuAcceptTeamMember
//       address:
//         NA: 0x22E8080
//         EU: 0x22E89C0
//       description: |-
//         Implements SPECIAL_PROC_PREPARE_MENU_ACCEPT_TEAM_MEMBER (see ScriptSpecialProcessCall).
//
//         r0: index into RECRUITMENT_TABLE_SPECIES
//     - name: InitRandomNpcJobs
//       address:
//         NA: 0x22E8124
//         EU: 0x22E8A64
//       description: |-
//         Implements SPECIAL_PROC_INIT_RANDOM_NPC_JOBS (see ScriptSpecialProcessCall).
//
//         r0: job type? 0 is a random NPC job, 1 is a bottle mission
//         r1: ?
//     - name: GetRandomNpcJobType
//       address:
//         NA: 0x22E81BC
//         EU: 0x22E8AFC
//       description: |-
//         Implements SPECIAL_PROC_GET_RANDOM_NPC_JOB_TYPE (see ScriptSpecialProcessCall).
//
//         return: job type?
//     - name: GetRandomNpcJobSubtype
//       address:
//         NA: 0x22E81D4
//         EU: 0x22E8B14
//       description: |-
//         Implements SPECIAL_PROC_GET_RANDOM_NPC_JOB_SUBTYPE (see ScriptSpecialProcessCall).
//
//         return: job subtype?
//     - name: GetRandomNpcJobStillAvailable
//       address:
//         NA: 0x22E81F0
//         EU: 0x22E8B30
//       description: |-
//         Implements SPECIAL_PROC_GET_RANDOM_NPC_JOB_STILL_AVAILABLE (see ScriptSpecialProcessCall).
//
//         return: bool
//     - name: AcceptRandomNpcJob
//       address:
//         NA: 0x22E8258
//         EU: 0x22E8B98
//       description: |-
//         Implements SPECIAL_PROC_ACCEPT_RANDOM_NPC_JOB (see ScriptSpecialProcessCall).
//
//         return: bool
//     - name: GroundMainLoop
//       address:
//         NA: 0x22E8774
//         EU: 0x22E90B4
//       description: |-
//         Appears to be the main loop for ground mode.
//
//         Based on debug print statements and general code structure, it seems contain a core loop, and dispatches to various functions in response to different events.
//
//         r0: mode, which is stored globally and used in switch statements for dispatch
//         return: return code
//     - name: GetAllocArenaGround
//       address:
//         NA: 0x22E935C
//         EU: 0x22E9C9C
//       description: |-
//         The GetAllocArena function used for ground mode. See SetMemAllocatorParams for more information.
//
//         r0: initial memory arena pointer, or null
//         r1: flags (see MemAlloc)
//         return: memory arena pointer, or null
//     - name: GetFreeArenaGround
//       address:
//         NA: 0x22E93C0
//         EU: 0x22E9D00
//       description: |-
//         The GetFreeArena function used for ground mode. See SetMemAllocatorParams for more information.
//
//         r0: initial memory arena pointer, or null
//         r1: pointer to free
//         return: memory arena pointer, or null
//     - name: GroundMainReturnDungeon
//       address:
//         NA: 0x22E9414
//         EU: 0x22E9D54
//       description: |-
//         Implements SPECIAL_PROC_RETURN_DUNGEON (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: GroundMainNextDay
//       address:
//         NA: 0x22E9438
//         EU: 0x22E9D78
//       description: |-
//         Implements SPECIAL_PROC_NEXT_DAY (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: JumpToTitleScreen
//       address:
//         NA: 0x22E95DC
//         EU: 0x22E9F1C
//       description: |-
//         Implements SPECIAL_PROC_JUMP_TO_TITLE_SCREEN and SPECIAL_PROC_0x1A (see ScriptSpecialProcessCall).
//
//         r0: int, argument value for SPECIAL_PROC_JUMP_TO_TITLE_SCREEN and -1 for SPECIAL_PROC_0x1A
//         return: bool (but note that the special process ignores this and always returns 0)
//     - name: ReturnToTitleScreen
//       address:
//         NA: 0x22E9694
//         EU: 0x22E9FD4
//       description: |-
//         Implements SPECIAL_PROC_RETURN_TO_TITLE_SCREEN (see ScriptSpecialProcessCall).
//
//         r0: fade duration
//         return: bool (but note that the special process ignores this and always returns 0)
//     - name: ScriptSpecialProcess0x16
//       address:
//         NA: 0x22E96F4
//         EU: 0x22EA034
//       description: |-
//         Implements SPECIAL_PROC_0x16 (see ScriptSpecialProcessCall).
//
//         r0: bool
//     - name: StatusUpdate
//       address:
//         NA: 0x2313A98
//         EU: 0x2314478
//       description: |-
//         Implements SPECIAL_PROC_STATUS_UPDATE (see ScriptSpecialProcessCall).
//
//         No params.
