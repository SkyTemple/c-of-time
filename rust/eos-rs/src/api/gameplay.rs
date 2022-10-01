//! General gameplay related functions that are always available.

use crate::api::dungeons::DungeonId;
use crate::api::enums::{MissionGenerationResult, MissionType};
use crate::api::iq::IqSkillId;
use crate::api::items::ItemId;
use crate::api::monsters::MonsterSpeciesId;
use crate::ctypes::c_int;
use crate::ffi;
use crate::util::OwnedSlice;
use core::marker::PhantomData;
use core::ptr;

/// Describes an active team setup
#[non_exhaustive]
pub enum TeamSetup {
    HeroOnly,
    HeroAndPartnerOnly,
}

#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// Move index of a monster, used by some functions.
pub enum TargetTypeIndex {
    FirstType = 0,
    SecondType = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
/// The rank a player scored in the sentry duty (footprint) minigame.
pub enum SentryGameRank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
}

impl TryFrom<i32> for SentryGameRank {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SentryGameRank::First),
            1 => Ok(SentryGameRank::Second),
            2 => Ok(SentryGameRank::Third),
            3 => Ok(SentryGameRank::Fourth),
            4 => Ok(SentryGameRank::Fifth),
            _ => Err(()),
        }
    }
}

/// Initializes the key wait process.
///
/// Implements (most of?) SPECIAL_PROC_KEY_WAIT_INIT (see ScriptSpecialProcessCall).
pub fn key_wait_init() {
    unsafe {
        ffi::KeyWaitInit();
    }
}

/// Gets the single-byte language ID of the current program.
///
/// The language ID appears to be used to index some global tables.
///
/// It is probably the firmware language ID...?
pub fn get_language() -> i32 {
    unsafe { ffi::GetLanguage() }
}

/// Initializes the main team. If the personality quest was just passed, the data will be taken
/// from there, otherwise the default fallback team will be set.
pub fn init_main_team_after_quiz() {
    unsafe { ffi::InitMainTeamAfterQuiz() }
}

/// Implements SPECIAL_PROC_0x3.
pub fn script_special_process_3() {
    unsafe { ffi::ScriptSpecialProcess0x3() }
}

/// Implements SPECIAL_PROC_0x4.
pub fn script_special_process_4() {
    unsafe { ffi::ScriptSpecialProcess0x4() }
}

/// Probably related to saving or quicksaving?
///
/// This function prints the debug message "NoteSave Base %d %d" with some values. It's also the
/// only place where GetRngSeed is called.
pub fn note_save_base(param_1: i32) -> i32 {
    unsafe { ffi::NoteSaveBase(param_1) }
}

/// Probably related to saving or quicksaving?
///
/// This function prints the debug message "NoteLoad Base %d" with some value. It's also the
/// only place where SetRngSeed is called.
pub fn note_load_base() -> i32 {
    unsafe { ffi::NoteLoadBase() }
}

/// Adventure log helper
pub struct AdventureLog(PhantomData<()>);

impl AdventureLog {
    /// Returns an internal reference to the adventure log. Note that this isn't a reference
    /// to the actual struct in memory (yet).
    ///
    /// # Safety
    /// This is unsafe, since it essentially borrows a global variable mutably, see
    /// safety rules for `static mut`s.
    pub unsafe fn get() -> Self {
        Self(PhantomData)
    }

    /// Sets the location of the adventure log struct in memory.
    ///
    /// Sets it in a static memory location.
    pub fn set_struct_location(&mut self) {
        unsafe { ffi::SetAdventureLogStructLocation() }
    }

    /// Clears the adventure log structure.
    pub fn clear_struct(&mut self) {
        unsafe { ffi::ClearAdventureLogStruct() }
    }

    /// Returns the current dungeon floor pair.
    pub fn get_dungeon_floor(&self) -> ffi::dungeon_floor_pair {
        unsafe { ffi::GetAdventureLogDungeonFloor() }
    }

    /// Sets the current dungeon floor pair.
    pub fn set_dungeon_floor(&mut self, dungeon_floor: ffi::dungeon_floor_pair) {
        unsafe { ffi::SetAdventureLogDungeonFloor(dungeon_floor) }
    }

    /// Checks if one adventure log entry is completed.
    pub fn is_entry_completed(&self, entry_id: u32) -> bool {
        unsafe { ffi::GetAdventureLogCompleted(entry_id) > 0 }
    }

    /// Marks one of the adventure log entry as completed.
    pub fn mark_entry_completed(&mut self, entry_id: u32) {
        unsafe { ffi::SetAdventureLogCompleted(entry_id) }
    }

    /// Checks if none of of the adventure log entry is completed.
    pub fn is_empty(&self) -> bool {
        unsafe { ffi::IsAdventureLogNotEmpty() == 0 }
    }

    /// Gets the number of dungeons cleared.
    pub fn get_number_dungeons_cleared(&self) -> u32 {
        unsafe { ffi::GetNbDungeonsCleared() }
    }

    /// Increments by 1 the number of dungeons cleared.
    pub fn increment_number_dungeons_cleared() {
        unsafe { ffi::IncrementNbDungeonsCleared() };
    }

    /// Gets the number of successful friend rescues.
    pub fn get_number_friend_rescues(&self) -> u32 {
        unsafe { ffi::GetNbFriendRescues() }
    }

    /// Increments by 1 the number of successful friend rescues.
    pub fn increment_number_friend_rescues(&mut self) {
        unsafe { ffi::IncrementNbFriendRescues() };
    }

    /// Gets the number of evolutions.
    pub fn get_number_evolutions(&self) -> u32 {
        unsafe { ffi::GetNbEvolutions() }
    }

    /// Increments by 1 the number of evolutions.
    pub fn increment_number_evolutions(&mut self) {
        unsafe { ffi::IncrementNbEvolutions() };
    }

    /// Leftover from Time & Darkness. Does not do anything.
    ///
    /// Calls to this matches the ones for incrementing the number of successful steals in Time & Darkness.
    pub fn increment_number_steals(&mut self) {
        unsafe { ffi::IncrementNbSteals() };
    }

    /// Gets the number of eggs hatched.
    pub fn get_number_eggs_hatched(&self) -> u32 {
        unsafe { ffi::GetNbEggsHatched() }
    }

    /// Increments by 1 the number of eggs hatched.
    pub fn increment_number_eggs_hatched(&mut self) {
        unsafe { ffi::IncrementNbEggsHatched() };
    }

    /// Gets the number of different monsters that joined.
    pub fn get_number_monsters_joined(&self) -> u32 {
        unsafe { ffi::GetNbPokemonJoined() }
    }

    /// Gets the number of different moves learned.
    pub fn get_number_moves_learned(&self) -> u32 {
        unsafe { ffi::GetNbMovesLearned() }
    }

    /// Gets the record of victories on one floor.
    pub fn get_victories_on_one_floor(&self) -> u32 {
        unsafe { ffi::GetVictoriesOnOneFloor() }
    }

    /// Sets the record of victories on one floor.
    pub fn set_victories_on_one_floor(&mut self, victories: u32) {
        unsafe { ffi::SetVictoriesOnOneFloor(victories) };
    }

    /// Gets the number of different monsters that battled against you.
    pub fn get_number_monsters_battled(&self) -> u32 {
        unsafe { ffi::GetNbPokemonBattled() }
    }

    /// Marks one monster as battled.
    pub fn set_monster_battled(&mut self, monster_id: MonsterSpeciesId) {
        unsafe { ffi::SetPokemonBattled(monster_id) };
    }

    /// Marks one monster as joined.
    pub fn set_monster_joined(&mut self, monster_id: MonsterSpeciesId) {
        unsafe { ffi::SetPokemonJoined(monster_id) };
    }

    /// Gets the number of big treasure wins.
    pub fn get_number_big_treasure_wins(&self) -> u32 {
        unsafe { ffi::GetNbBigTreasureWins() }
    }

    /// Increments by 1 the number of big treasure wins.
    pub fn increment_number_of_big_treasure_wins() {
        unsafe { ffi::IncrementNbBigTreasureWins() };
    }

    /// Sets the number of big treasure wins.
    pub fn set_number_big_treasure_wins(&mut self, number: u32) {
        unsafe { ffi::SetNbBigTreasureWins(number) };
    }

    /// Gets the number of items recycled.
    pub fn get_number_recycled(&mut self) -> u32 {
        unsafe { ffi::GetNbRecycled() }
    }

    /// Sets the number of items recycled.
    pub fn set_number_recycled(&mut self, number: u32) {
        unsafe { ffi::SetNbRecycled(number) };
    }

    /// Gets the number of Sky Gifts sent.
    pub fn get_number_sky_gifts_sent(&self) -> u32 {
        unsafe { ffi::GetNbSkyGiftsSent() }
    }

    /// Increments by 1 the number of sky gifts sent.
    pub fn increment_number_of_gifts_sent() {
        unsafe { ffi::IncrementNbSkyGiftsSent() };
    }

    /// Sets the number of Sky Gifts sent.
    pub fn set_number_sky_gifts_sent(&mut self, number: u32) {
        unsafe { ffi::SetNbSkyGiftsSent(number) };
    }

    /// Computes the counters from the bit fields in the adventure log, as they are not updated
    /// automatically when bit fields are altered.
    ///
    /// Affects [`Self::get_number_monsters_joined`], [`Self::get_number_moves_learned`],
    /// [`Self::get_number_monsters_battled`] and [`Self::get_number_items_acquired`].
    pub fn compute_special_counters(&mut self) {
        unsafe { ffi::ComputeSpecialCounters() };
    }

    /// Marks a specified special monster as recruited in the adventure log.
    pub fn set_special_monster_recruited(&mut self, monster_id: MonsterSpeciesId) {
        unsafe { ffi::RecruitSpecialPokemonLog(monster_id) };
    }

    /// Gets the number of times the player fainted.
    pub fn get_number_fainted(&self) -> u32 {
        unsafe { ffi::GetNbFainted() }
    }

    /// Increments by 1 the number of times the player fainted.
    pub fn increment_number_of_fainted() {
        unsafe { ffi::IncrementNbFainted() };
    }

    /// Gets the number of items acquired.
    pub fn get_number_items_acquired(&self) -> u32 {
        unsafe { ffi::GetNbItemAcquired() }
    }

    /// Marks one specific item as acquired.
    pub fn set_item_acquired(&mut self, item_id: ItemId) {
        unsafe { ffi::SetItemAcquired(item_id) };
    }

    /// Sets a challenge letter as cleared.
    pub fn set_challenge_letter_cleared(&mut self, challenge_letter: u32) {
        unsafe { ffi::SetChallengeLetterCleared(challenge_letter) };
    }

    /// Gets the points for the associated rank in the footprints minigame.
    pub fn get_sentry_duty_game_points(&self, rank: SentryGameRank) -> u32 {
        unsafe { ffi::GetSentryDutyGamePoints(rank as i32) }
    }

    /// Sets the points for the associated rank in the footprints minigame.
    pub fn set_sentry_duty_game_points(&mut self, points: u32) -> Option<SentryGameRank> {
        unsafe { ffi::SetSentryDutyGamePoints(points) }
            .try_into()
            .ok()
    }
}

/// Checks if a given monster is on the exploration team (not necessarily the active party)?
pub fn is_monster_on_team(monster_id: MonsterSpeciesId, param_2: i32) -> bool {
    unsafe { ffi::IsMonsterOnTeam(monster_id, param_2) > 0 }
}

/// Sets the team setup of the currently active party.
pub fn set_team_setup(team_setup: TeamSetup) {
    match team_setup {
        TeamSetup::HeroOnly => unsafe { ffi::SetTeamSetupHeroOnly() },
        TeamSetup::HeroAndPartnerOnly => unsafe { ffi::SetTeamSetupHeroAndPartnerOnly() },
    }
}

/// Appears to get the team's active party members.
///
/// Output is a slice-like of 2-byte values (they seem to be indexes of some sort) describing each
/// party member.
pub fn get_party_members() -> impl AsRef<[u16]> {
    unsafe {
        let mut party_members: [u16; 4] = [0; 4];
        let nb = ffi::GetPartyMembers(party_members.as_mut_ptr());
        OwnedSlice::new(party_members, 0, nb as usize)
    }
}

/// Counts the number of monsters in the active team.
pub fn count_party_members() -> i32 {
    unsafe { ffi::GetPartyMembers(ptr::null_mut()) }
}

/// Tests whether an IQ skill with a given ID is active.
pub fn iq_skill_flag_test(iq_skill_flags: &mut u32, iq_id: IqSkillId) -> bool {
    unsafe { ffi::IqSkillFlagTest(iq_skill_flags, iq_id) > 0 }
}

/// Returns the number of SOS mails.
pub fn get_sos_mail_count(param_1: i32, param_2: bool) -> i32 {
    unsafe { ffi::GetSosMailCount(param_1, param_2 as ffi::bool_) }
}

/// Attempts to generate a random mission.
///
/// Returns the result, `None` is returned if the game returns an invalid result internally.
///
/// # Safety
/// The caller must make sure the undefined params are valid for this function.
pub fn generate_mission(
    unknown: &mut ffi::undefined,
    mission_data: &mut ffi::mission,
) -> Option<MissionGenerationResult> {
    unsafe { ffi::GenerateMission(unknown, mission_data).try_into().ok() }
}

/// Generates the missions displayed on the Job Bulletin Board and the Outlaw Notice Board.
pub fn generate_daily_missions() {
    unsafe { ffi::GenerateDailyMissions() }
}

enum _DoMissionCheckType {
    General(usize),
    Accepted,
}

enum _DoMissionCheckResult {
    General(Option<usize>),
    Accepted(bool),
}

fn _do_mission_check(
    check_type: _DoMissionCheckType,
    mission_type: MissionType,
    dungeon_id: DungeonId,
) -> _DoMissionCheckResult {
    let mission_type_group = mission_type.group() as ffi::mission_type::Type;
    let mission_subtype = mission_type.c_subtype();

    // TODO: This may not actually be safe if the game does anything else with this ominous struct.
    #[repr(C)]
    struct MissionSubtypeStruct {
        subtype: ffi::mission_subtype,
        // just to reduce the chance this ends badly, we add some padidng bytes
        _pad: [u8; 127],
    }
    let mut mission_subtype_struct = MissionSubtypeStruct {
        subtype: mission_subtype,
        _pad: [0; 127],
    };

    unsafe {
        match check_type {
            _DoMissionCheckType::General(start_index) => {
                let result = ffi::GetMissionByTypeAndDungeon(
                    start_index as c_int,
                    mission_type_group,
                    &mut mission_subtype_struct as *mut MissionSubtypeStruct as *mut ffi::undefined,
                    dungeon_id,
                );

                _DoMissionCheckResult::General(if result < 0 {
                    None
                } else {
                    Some(result as usize)
                })
            }

            _DoMissionCheckType::Accepted => _DoMissionCheckResult::Accepted(
                ffi::CheckAcceptedMissionByTypeAndDungeon(
                    mission_type_group,
                    &mut mission_subtype_struct as *mut MissionSubtypeStruct as *mut ffi::undefined,
                    dungeon_id,
                ) > 0,
            ),
        }
    }
}

/// Returns the position on the mission list of the first mission of the specified type that takes
/// place in the specified dungeon.
///
/// If the type of the mission has a subtype, the subtype of the checked mission must match
/// too.
pub fn get_mission_by_type_and_dungeon(
    start_index: usize,
    mission_type: MissionType,
    dungeon_id: DungeonId,
) -> Option<usize> {
    match _do_mission_check(
        _DoMissionCheckType::General(start_index),
        mission_type,
        dungeon_id,
    ) {
        _DoMissionCheckResult::General(r) => r,
        _DoMissionCheckResult::Accepted(_) => unreachable!(),
    }
}

/// Returns true if there are any accepted missions on the mission list that are of the specified
/// type and take place in the specified dungeon.
///
/// If the type of the mission has a subtype, the subtype of the checked mission must match
/// too.
pub fn check_accepted_mission_by_type_and_dungeon(
    mission_type: MissionType,
    dungeon_id: DungeonId,
) -> bool {
    match _do_mission_check(_DoMissionCheckType::Accepted, mission_type, dungeon_id) {
        _DoMissionCheckResult::General(_) => unreachable!(),
        _DoMissionCheckResult::Accepted(r) => r,
    }
}

/// Given a mission struct, clears some of it fields.
///
/// In particular, [`ffi::mission::status`] is set to
/// [`ffi::mission_status::MISSION_STATUS_INVALID`], [`ffi::mission::dungeon_id`] is set to -1,
/// [`ffi::mission::floor`] is set to 0 and [`ffi::mission::reward_type`] is set
/// to [`ffi::mission_reward_type::MISSION_REWARD_MONEY`].
pub fn clear_mission_data(mission: &mut ffi::mission) {
    unsafe { ffi::ClearMissionData(mission) }
}

/// Returns the number of missions completed.
pub fn dungeon_had_request_done(param_1: i32, param_2: bool) -> i32 {
    unsafe { ffi::GetSosMailCount(param_1, param_2 as ffi::bool_) }
}

/// Implements SPECIAL_PROC_0x3D.
pub fn script_special_process_x3d() {
    unsafe { ffi::ScriptSpecialProcess0x3D() }
}

/// Implements SPECIAL_PROC_0x3E.
pub fn script_special_process_x3e() {
    unsafe { ffi::ScriptSpecialProcess0x3E() }
}

/// Implements SPECIAL_PROC_0x17.
pub fn script_special_process_x17() {
    unsafe { ffi::ScriptSpecialProcess0x17() }
}

/// Gets info about the item at a given item table (not sure what this table is...) index.
///
/// Once we find out more about the item table, this function will probably move to a wrapper
/// struct.
pub fn item_at_table_idx(table_idx: i32) -> ffi::bulk_item {
    let mut out = ffi::bulk_item {
        id: ffi::item_id_16 {
            _bitfield_align_1: [],
            _bitfield_1: Default::default(),
        },
        quantity: 0,
    };
    unsafe { ffi::ItemAtTableIdx(table_idx, &mut out) }
    out
}

/// Checks whether the specified monster has been attacked by the player at some point in their
/// adventure during an exploration.
///
/// The check is performed using the result of passing the ID to
/// [`MonsterSpeciesId::base_gender_form`].
pub fn has_monster_been_attacked_in_dungeons(monster_id: MonsterSpeciesId) -> bool {
    unsafe { ffi::HasMonsterBeenAttackedInDungeons(monster_id) > 0 }
}

/// Marks a dungeon tip as already shown to the player
pub fn set_dungeon_tip_shown(tip_id: i32) {
    unsafe { ffi::SetDungeonTipShown(tip_id) }
}

/// Checks if a dungeon tip has already been shown before or not.
pub fn was_dungeon_tip_shown(tip_id: i32) -> bool {
    unsafe { ffi::GetDungeonTipShown(tip_id) > 0 }
}

/// Returns the monster ID of the specified monster spawn entry.
pub fn get_monster_id_from_spawn_entry(spawn_entry: &ffi::monster_spawn_entry) -> MonsterSpeciesId {
    unsafe { ffi::GetMonsterIdFromSpawnEntry(force_mut_ptr!(spawn_entry)) }
}

/// Returns the level of the monster defined in the specified monster spawn entry.
pub fn get_monster_level_from_spawn_entry(spawn_entry: &ffi::monster_spawn_entry) -> u8 {
    unsafe { ffi::GetMonsterLevelFromSpawnEntry(force_mut_ptr!(spawn_entry)) }
}

/// Applies the IQ boosts from eating a Gummi to the target monster.
///
/// You should not use this in dungeon mode.
/// Use [`crate::api::dungeon_mode::DungeonEffectsEmitter::apply_gummi_boosts`] instead.
///
/// # Safety
/// The caller must make sure the undefined params and buffer are valid for this function.
pub unsafe fn apply_gummi_boosts(
    param_1: *mut ffi::undefined2,
    param_2: *mut ffi::undefined2,
    param_3: *mut ffi::undefined,
    param_4: *mut ffi::undefined,
    param_5: ffi::undefined2,
    param_6: ffi::undefined,
    buffer: *mut crate::ctypes::c_void,
) {
    ffi::ApplyGummiBoostsGroundMode(param_1, param_2, param_3, param_4, param_5, param_6, buffer)
}

/// Returns the data of the player monster (first slot in Chimecho Assembly).
pub fn get_hero_data<'a>() -> Option<&'a ffi::ground_monster> {
    let ptr = unsafe { ffi::GetHeroData() };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &*ptr })
    }
}

/// Returns the data of the player monster (first slot in Chimecho Assembly), mutably.
pub fn get_hero_data_mut<'a>() -> Option<&'a mut ffi::ground_monster> {
    let ptr = unsafe { ffi::GetHeroData() };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &mut *ptr })
    }
}

/// Returns the data of the partner monster (second slot in Chimecho Assembly).
pub fn get_partner_data<'a>() -> Option<&'a ffi::ground_monster> {
    let ptr = unsafe { ffi::GetPartnerData() };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &*ptr })
    }
}

/// Returns the data of the partner monster (second slot in Chimecho Assembly), mutably.
pub fn get_partner_data_mut<'a>() -> Option<&'a mut ffi::ground_monster> {
    let ptr = unsafe { ffi::GetPartnerData() };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &mut *ptr })
    }
}

/// Returns a struct containing information about a team member.
pub fn get_team_member_data<'a>(member_id: u8) -> Option<&'a ffi::team_member> {
    let ptr = unsafe { ffi::GetTeamMemberData(member_id) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &*ptr })
    }
}

/// Returns a struct containing information about a team member.
pub fn get_team_member_data_mut<'a>(member_id: u8) -> Option<&'a mut ffi::team_member> {
    let ptr = unsafe { ffi::GetTeamMemberData(member_id) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &mut *ptr })
    }
}

/// Returns the data of a monster sent into the Explorer Dojo using the "exchange teams" option.
///
/// `entry_number` must be a value between \[0,3\].
pub fn get_explorer_dojo_monster_data<'a>(entry_number: u8) -> Option<&'a ffi::ground_monster> {
    let ptr = unsafe { ffi::GetExplorerMazeMonster(entry_number) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &*ptr })
    }
}

/// Returns the data of a monster sent into the Explorer Dojo using the "exchange teams" option.
///
/// `entry_number` must be a value between \[0,3\].
pub fn get_explorer_dojo_monster_data_mut<'a>(
    entry_number: u8,
) -> Option<&'a mut ffi::ground_monster> {
    let ptr = unsafe { ffi::GetExplorerMazeMonster(entry_number) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &mut *ptr })
    }
}
