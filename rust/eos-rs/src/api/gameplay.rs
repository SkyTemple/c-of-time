//! General gameplay related functions that are always available.

use core::ptr;
use crate::api::objects::item_catalog;
use crate::ffi;
use crate::ffi::{exclusive_item_effect_id, item_id_16};
use crate::util::OwnedSlice;

/// Describes an active team setup
#[non_exhaustive]
pub enum TeamSetup {
    HeroOnly,
    HeroAndPartnerOnly
}

/// Initializes the key wait process.
///
/// Implements (most of?) SPECIAL_PROC_KEY_WAIT_INIT (see ScriptSpecialProcessCall).
pub fn key_wait_init() {
    unsafe { ffi::KeyWaitInit(); }
}

/// Checks if an item is one of the aura bows received at the start of the game.
pub fn is_aura_bow(item_id: item_catalog::Type) -> bool {
    unsafe { ffi::IsAuraBow(item_id) > 0 }
}

/// Sets the amount of money the player is carrying, clamping the value to the range
/// [0, MAX_MONEY_CARRIED].
pub fn set_money_carried(money: i32) {
    unsafe { ffi::SetMoneyCarried(money) }
}

/// Sets the amount of money the player has stored in the Duskull Bank, clamping the value to the
/// range [0, MAX_MONEY_STORED].
pub fn set_money_stored(money: i32) {
    unsafe { ffi::SetMoneyStored(money) }
}

/// Checks if the player's bag is full.
pub fn is_bag_full() -> bool {
    unsafe { ffi::IsBagFull() > 0 }
}

/// Count the amount of the specified item in the player's bag.
pub fn count_item_type_in_bag(item_id: item_catalog::Type) -> i32 {
    unsafe { ffi::CountItemTypeInBag(item_id) }
}

/// Adds the specified amount of an item to the player's bag. Returns whether or not any
/// items could be added.
pub fn add_item_to_bag(item_id: item_catalog::Type, amount: u16) -> bool {
    unsafe { ffi::AddItemToBag(
        &mut ffi::owned_item { id: item_id_16 { _bitfield_align_1: [], _bitfield_1: item_id_16::new_bitfield_1(item_id) }, amount }
    ) > 0 }
}

/// Special process 0x39.
///
/// This is *probably* is_storage_full: checks if the player's storage is full.
pub fn is_storage_full() -> bool {
    unsafe { ffi::ScriptSpecialProcess0x39() > 0 }
}

/// Count the amount of the specified item in the player's storage.
pub fn count_item_type_in_storage(item_id: item_catalog::Type) -> i32 {
    unsafe { ffi::CountItemTypeInStorage(
        &mut ffi::owned_item { id: item_id_16 { _bitfield_align_1: [], _bitfield_1: item_id_16::new_bitfield_1(item_id) }, amount: 0 }
    ) }
}

/// Removes (the specified amount...?) of the given item type from the storage.
pub fn remove_items_type_in_storage(item_id: item_catalog::Type, amount: u16) -> bool {
    unsafe { ffi::RemoveItemsTypeInStorage(
        &mut ffi::owned_item { id: item_id_16 { _bitfield_align_1: [], _bitfield_1: item_id_16::new_bitfield_1(item_id) }, amount }
    ) > 0 }
}

/// Adds (the specified amount...?) of the given item type to the storage. Returns whether or not
/// any items could be added.
pub fn add_item_to_storage(item_id: item_catalog::Type, amount: u16) -> bool {
    unsafe { ffi::AddItemToStorage(
        &mut ffi::owned_item { id: item_id_16 { _bitfield_align_1: [], _bitfield_1: item_id_16::new_bitfield_1(item_id) }, amount }
    ) > 0 }
}

/// Gets the exclusive item offset, which is the item ID relative to that of the first exclusive
/// item, the Prism Ruff.
pub fn get_exclusive_item_offset(item_id: item_catalog::Type) -> i32 {
    unsafe { ffi::GetExclusiveItemOffset(item_id) }
}

/// Applies stat boosts from an exclusive item.
pub fn apply_exclusive_item_stat_boosts(item_id: item_catalog::Type, atk_to_modify: &mut u8, sp_atk_to_modify: &mut u8, def_to_modify: &mut u8, sp_def_to_modify: &mut u8) {
    unsafe { ffi::ApplyExclusiveItemStatBoosts(item_id, atk_to_modify, sp_atk_to_modify, def_to_modify, sp_def_to_modify) }
}

/// Sets the bit for an exclusive item effect.
pub fn set_exclusive_item_effect(effect_flags: &mut u32, effect_id: exclusive_item_effect_id::Type) {
    unsafe { ffi::SetExclusiveItemEffect(effect_flags, effect_id) }
}

/// Tests the exclusive item bitvector for a specific exclusive item effect.
pub fn test_exclusive_item_effect_flag(effect_flags: &mut u32, effect_id: exclusive_item_effect_id::Type) -> bool {
    unsafe { ffi::ExclusiveItemEffectFlagTest(effect_flags, effect_id) > 0 }
}

/// Gets the language type.
///
/// This is the value backing the special LANGUAGE_TYPE script variable.
pub fn get_language_type() -> i32 {
    unsafe { ffi::GetLanguageType() }
}

/// Gets the single-byte language ID of the current program.
///
/// The language ID appears to be used to index some global tables.
///
/// It is probably the firmware language ID...?
pub fn get_language() -> i32 {
    unsafe { ffi::GetLanguage() }
}

/// Returns the current value of the NOTIFY_NOTE script variable.
pub fn get_notify_note() -> bool {
    unsafe { ffi::GetNotifyNote() > 0 }
}

/// Sets the current value of the NOTIFY_NOTE script variable.
pub fn set_notify_note(value: bool) {
    unsafe { ffi::SetNotifyNote(value as ffi::bool_) }
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

/// Gets the value of the GAME_MODE script variable.
pub fn get_game_mode() -> i32 {
    unsafe { ffi::GetGameMode() }
}

//     - name: GetSpecialEpisodeType
//       address:
//         NA: 0x204C8EC
//         EU: 0x204CC24
//       description: |-
//         Gets the special episode type from the SPECIAL_EPISODE_TYPE script variable.
//
//         return: special episode type
//     - name: ScenarioFlagBackup
//       address:
//         NA: 0x204CCB8
//         EU: 0x204CFF0
//       description: |-
//         Saves scenario flag script variables (SCENARIO_SELECT, SCENARIO_MAIN_BIT_FLAG) to their respective BACKUP script variables, but only in certain game modes.
//
//         This function prints the debug string "ScenarioFlag BackupGameMode %d" with the game mode.
//
//         No params.
//    - name: SetAdventureLogStructLocation
//       address:
//         NA: 0x204FA24
//         EU: 0x204FD5C
//         JP: 0x204FD70
//       description: |-
//         Sets the location of the adventure log struct in memory.
//
//         Sets it in a static memory location (At 0x22AB69C [US], 0x22ABFDC [EU], 0x22ACE58 [JP])
//
//         No params.
//     - name: SetAdventureLogDungeonFloor
//       address:
//         NA: 0x204FA3C
//         EU: 0x204FD74
//         JP: 0x204FD88
//       description: |-
//         Sets the current dungeon floor pair.
//
//         r0: struct dungeon_floor_pair
//     - name: GetAdventureLogDungeonFloor
//       address:
//         NA: 0x204FA5C
//         EU: 0x204FD94
//         JP: 0x204FDA8
//       description: |-
//         Gets the current dungeon floor pair.
//
//         return: struct dungeon_floor_pair
//     - name: ClearAdventureLogStruct
//       address:
//         NA: 0x204FA70
//         EU: 0x204FDA8
//         JP: 0x204FDBC
//       description: |-
//         Clears the adventure log structure.
//
//         No params.
//     - name: SetAdventureLogCompleted
//       address:
//         NA: 0x204FB9C
//         EU: 0x204FED4
//         JP: 0x204FEE8
//       description: |-
//         Marks one of the adventure log entry as completed.
//
//         r0: entry ID
//     - name: IsAdventureLogNotEmpty
//       address:
//         NA: 0x204FBC4
//         EU: 0x204FEFC
//         JP: 0x204FF10
//       description: |-
//         Checks if at least one of the adventure log entry is completed.
//
//         return: bool
//     - name: GetAdventureLogCompleted
//       address:
//         NA: 0x204FBFC
//         EU: 0x204FF34
//         JP: 0x204FF48
//       description: |-
//         Checks if one adventure log entry is completed.
//
//         r0: entry ID
//         return: bool

/// Increments by 1 the number of dungeons cleared.
pub fn increment_number_dungeons_cleared() {
    unsafe { ffi::IncrementNbDungeonsCleared() };
}

//    - name: GetNbDungeonsCleared
//       address:
//         NA: 0x204FC6C
//         EU: 0x204FFA4
//         JP: 0x204FFB8
//       description: |-
//         Gets the number of dungeons cleared.
//
//         return: the number of dungeons cleared
//     - name: IncrementNbFriendRescues
//       address:
//         NA: 0x204FC80
//         EU: 0x204FFB8
//         JP: 0x204FFCC
//       description: |-
//         Increments by 1 the number of successful friend rescues.
//
//         No params.
//     - name: GetNbFriendRescues
//       address:
//         NA: 0x204FCC8
//         EU: 0x2050000
//         JP: 0x2050014
//       description: |-
//         Gets the number of successful friend rescues.
//
//         return: the number of successful friend rescues
//     - name: IncrementNbEvolutions
//       address:
//         NA: 0x204FCDC
//         EU: 0x2050014
//         JP: 0x2050028
//       description: |-
//         Increments by 1 the number of evolutions.
//
//         No params.
//     - name: GetNbEvolutions
//       address:
//         NA: 0x204FD24
//         EU: 0x205005C
//         JP: 0x2050070
//       description: |-
//         Gets the number of evolutions.
//
//         return: the number of evolutions
//     - name: IncrementNbSteals
//       address:
//         NA: 0x204FD38
//         EU: 0x2050070
//         JP: 0x2050084
//       description: |-
//         Leftover from Time & Darkness. Does not do anything.
//
//         Calls to this matches the ones for incrementing the number of successful steals in Time & Darkness.
//
//         No params.
//     - name: IncrementNbEggsHatched
//       address:
//         NA: 0x204FD3C
//         EU: 0x2050074
//         JP: 0x2050088
//       description: |-
//         Increments by 1 the number of eggs hatched.
//
//         No params.
//     - name: GetNbEggsHatched
//       address:
//         NA: 0x204FD78
//         EU: 0x20500B0
//         JP: 0x20500C4
//       description: |-
//         Gets the number of eggs hatched.
//
//         return: the number of eggs hatched
//     - name: GetNbPokemonJoined
//       address:
//         NA: 0x204FD8C
//         EU: 0x20500C4
//         JP: 0x20500D8
//       description: |-
//         Gets the number of different pokémon that joined.
//
//         return: the number of different pokémon that joined
//     - name: GetNbMovesLearned
//       address:
//         NA: 0x204FDA0
//         EU: 0x20500D8
//         JP: 0x20500EC
//       description: |-
//         Gets the number of different moves learned.
//
//         return: the number of different moves learned
//     - name: SetVictoriesOnOneFloor
//       address:
//         NA: 0x204FDB4
//         EU: 0x20500EC
//         JP: 0x2050100
//       description: |-
//         Sets the record of victories on one floor.
//
//         r0: the new record of victories
//     - name: GetVictoriesOnOneFloor
//       address:
//         NA: 0x204FDE8
//         EU: 0x2050120
//         JP: 0x2050134
//       description: |-
//         Gets the record of victories on one floor.
//
//         return: the record of victories
//     - name: SetPokemonJoined
//       address:
//         NA: 0x204FDFC
//         EU: 0x2050134
//         JP: 0x2050148
//       description: |-
//         Marks one pokémon as joined.
//
//         r0: monster ID
//     - name: SetPokemonBattled
//       address:
//         NA: 0x204FE58
//         EU: 0x2050190
//         JP: 0x20501A4
//       description: |-
//         Marks one pokémon as battled.
//
//         r0: monster ID
//     - name: GetNbPokemonBattled
//       address:
//         NA: 0x204FEB4
//         EU: 0x20501EC
//         JP: 0x2050200
//       description: |-
//         Gets the number of different pokémon that battled against you.
//
//         return: the number of different pokémon that battled against you

/// Increments by 1 the number of big treasure wins.
pub fn increment_number_of_big_treasure_wins() {
    unsafe { ffi::IncrementNbBigTreasureWins() };
}

//    - name: SetNbBigTreasureWins
//       address:
//         NA: 0x204FEE8
//         EU: 0x2050220
//         JP: 0x2050234
//       description: |-
//         Sets the number of big treasure wins.
//
//         r0: the new number of big treasure wins
//     - name: GetNbBigTreasureWins
//       address:
//         NA: 0x204FF20
//         EU: 0x2050258
//         JP: 0x205026C
//       description: |-
//         Gets the number of big treasure wins.
//
//         return: the number of big treasure wins
//     - name: SetNbRecycled
//       address:
//         NA: 0x204FF34
//         EU: 0x205026C
//         JP: 0x2050280
//       description: |-
//         Sets the number of items recycled.
//
//         r0: the new number of items recycled
//     - name: GetNbRecycled
//       address:
//         NA: 0x204FF6C
//         EU: 0x20502A4
//         JP: 0x20502B8
//       description: |-
//         Gets the number of items recycled.
//
//         return: the number of items recycled

/// Increments by 1 the number of sky gifts sent.
pub fn increment_number_of_gifts_sent() {
    unsafe { ffi::IncrementNbSkyGiftsSent() };
}

//    - name: SetNbSkyGiftsSent
//       address:
//         NA: 0x204FFA0
//         EU: 0x20502D8
//         JP: 0x20502EC
//       description: |-
//         Sets the number of Sky Gifts sent.
//
//         return: the number of Sky Gifts sent
//     - name: GetNbSkyGiftsSent
//       address:
//         NA: 0x204FFD8
//         EU: 0x2050310
//         JP: 0x2050324
//       description: |-
//         Gets the number of Sky Gifts sent.
//
//         return: the number of Sky Gifts sent
//     - name: ComputeSpecialCounters
//       address:
//         NA: 0x204FFEC
//         EU: 0x2050324
//         JP: 0x2050338
//       description: |-
//         Computes the counters from the bit fields in the adventure log, as they are not updated automatically when bit fields are altered.
//
//         Affects GetNbPokemonJoined, GetNbMovesLearned, GetNbPokemonBattled and GetNbItemAcquired.
//
//         No params.
//     - name: RecruitSpecialPokemonLog
//       address:
//         NA: 0x2050244
//         EU: 0x205057C
//         JP: 0x2050590
//       description: |-
//         Marks a specified special pokémon as recruited in the adventure log.
//
//         r0: monster ID
//     - name: IncrementNbFainted
//       address:
//         NA: 0x20502B0
//         EU: 0x20505E8
//         JP: 0x20505FC
//       description: |-
//         Increments by 1 the number of times you fainted.
//
//         No params.
//     - name: GetNbFainted
//       address:
//         NA: 0x20502EC
//         EU: 0x2050624
//         JP: 0x2050638
//       description: |-
//         Gets the number of times you fainted.
//
//         return: the number of times you fainted
//     - name: SetItemAcquired
//       address:
//         NA: 0x2050300
//         EU: 0x2050638
//         JP: 0x205064C
//       description: |-
//         Marks one specific item as acquired.
//
//         r0: item ID
//     - name: GetNbItemAcquired
//       address:
//         NA: 0x20503CC
//         EU: 0x2050704
//         JP: 0x2050718
//       description: |-
//         Gets the number of items acquired.
//
//         return: the number of items acquired
//     - name: SetChallengeLetterCleared
//       address:
//         NA: 0x2050420
//         EU: 0x2050758
//         JP: 0x205076C
//       description: |-
//         Sets a challenge letter as cleared.
//
//         r0: challenge ID
//     - name: GetSentryDutyGamePoints
//       address:
//         NA: 0x20504A4
//         EU: 0x20507DC
//         JP: 0x20507F0
//       description: |-
//         Gets the points for the associated rank in the footprints minigame.
//
//         r0: the rank (range 0-4, 1st to 5th)
//         return: points
//     - name: SetSentryDutyGamePoints
//       address:
//         NA: 0x20504BC
//         EU: 0x20507F4
//         JP: 0x2050808
//       description: |-
//         Sets a new record in the footprints minigame.
//
//         r0: points
//         return: the rank (range 0-4, 1st to 5th; -1 if out of ranking)

//     - name: DungeonGoesUp
//       address:
//         NA: 0x2051288
//         EU: 0x20515C0
//       description: |-
//         Returns whether the specified dungeon is considered as going upward or not
//
//         r0: dungeon id
//         return: bool
//     - name: IsUnown
//       address:
//         NA: 0x2054A88
//         EU: 0x2054E04
//       description: |-
//         Checks if a monster ID is an Unown.
//
//         r0: monster ID
//         return: bool
//     - name: IsShaymin
//       address:
//         NA: 0x2054AA4
//         EU: 0x2054E20
//       description: |-
//         Checks if a monster ID is a Shaymin form.
//
//         r0: monster ID
//         return: bool
//     - name: IsCastform
//       address:
//         NA: 0x2054AD4
//         EU: 0x2054E50
//       description: |-
//         Checks if a monster ID is a Castform form.
//
//         r0: monster ID
//         return: bool
//     - name: IsCherrim
//       address:
//         NA: 0x2054B2C
//         EU: 0x2054EA8
//       description: |-
//         Checks if a monster ID is a Cherrim form.
//
//         r0: monster ID
//         return: bool
//     - name: IsDeoxys
//       address:
//         NA: 0x2054B74
//         EU: 0x2054EF0
//       description: |-
//         Checks if a monster ID is a Deoxys form.
//
//         r0: monster ID
//         return: bool
//     - name: IsMonsterOnTeam
//       address:
//         NA: 0x2055148
//         EU: 0x20554C4
//       description: |-
//         Checks if a given monster is on the exploration team (not necessarily the active party)?
//
//         r0: monster ID
//         r1: ?
//         return: bool

pub fn set_team_setup(team_setup: TeamSetup) {
    match team_setup {
        TeamSetup::HeroOnly => unsafe { ffi::SetTeamSetupHeroOnly() }
        TeamSetup::HeroAndPartnerOnly => unsafe { ffi::SetTeamSetupHeroAndPartnerOnly() }
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

//     - name: IqSkillFlagTest
//       address:
//         NA: 0x2058F04
//         EU: 0x2059280
//       description: |-
//         Tests whether an IQ skill with a given ID is active.
//
//         r0: IQ skill bitvector to test
//         r1: IQ skill ID
//         return: bool

/// Returns the number of SOS mails.
pub fn get_sos_mail_count(param_1: i32, param_2: bool) -> i32 {
    unsafe { ffi::GetSosMailCount(param_1, param_2 as ffi::bool_) }
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
pub fn item_at_table_idx(table_idx: i32) -> ffi::owned_item {
    let mut out = ffi::owned_item {
        id: item_id_16 { _bitfield_align_1: [], _bitfield_1: Default::default() },
        amount: 0
    };
    unsafe { ffi::ItemAtTableIdx(table_idx, &mut out) }
    out
}
