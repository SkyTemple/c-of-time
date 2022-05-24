//! General gameplay related functions that are always available.

use crate::api::objects::*;
use crate::ffi;
use crate::ffi::{exclusive_item_effect_id, item_id_16};
use crate::util::OwnedSlice;
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
    unsafe {
        ffi::AddItemToBag(&mut ffi::owned_item {
            id: item_id_16 {
                _bitfield_align_1: [],
                _bitfield_1: item_id_16::new_bitfield_1(item_id),
            },
            amount,
        }) > 0
    }
}

/// Special process 0x39.
///
/// This is *probably* is_storage_full: checks if the player's storage is full.
pub fn is_storage_full() -> bool {
    unsafe { ffi::ScriptSpecialProcess0x39() > 0 }
}

/// Count the amount of the specified item in the player's storage.
pub fn count_item_type_in_storage(item_id: item_catalog::Type) -> i32 {
    unsafe {
        ffi::CountItemTypeInStorage(&mut ffi::owned_item {
            id: item_id_16 {
                _bitfield_align_1: [],
                _bitfield_1: item_id_16::new_bitfield_1(item_id),
            },
            amount: 0,
        })
    }
}

/// Removes (the specified amount...?) of the given item type from the storage.
pub fn remove_items_type_in_storage(item_id: item_catalog::Type, amount: u16) -> bool {
    unsafe {
        ffi::RemoveItemsTypeInStorage(&mut ffi::owned_item {
            id: item_id_16 {
                _bitfield_align_1: [],
                _bitfield_1: item_id_16::new_bitfield_1(item_id),
            },
            amount,
        }) > 0
    }
}

/// Adds (the specified amount...?) of the given item type to the storage. Returns whether or not
/// any items could be added.
pub fn add_item_to_storage(item_id: item_catalog::Type, amount: u16) -> bool {
    unsafe {
        ffi::AddItemToStorage(&mut ffi::owned_item {
            id: item_id_16 {
                _bitfield_align_1: [],
                _bitfield_1: item_id_16::new_bitfield_1(item_id),
            },
            amount,
        }) > 0
    }
}

/// Gets the exclusive item offset, which is the item ID relative to that of the first exclusive
/// item, the Prism Ruff.
pub fn get_exclusive_item_offset(item_id: item_catalog::Type) -> i32 {
    unsafe { ffi::GetExclusiveItemOffset(item_id) }
}

/// Applies stat boosts from an exclusive item.
pub fn apply_exclusive_item_stat_boosts(
    item_id: item_catalog::Type,
    atk_to_modify: &mut u8,
    sp_atk_to_modify: &mut u8,
    def_to_modify: &mut u8,
    sp_def_to_modify: &mut u8,
) {
    unsafe {
        ffi::ApplyExclusiveItemStatBoosts(
            item_id,
            atk_to_modify,
            sp_atk_to_modify,
            def_to_modify,
            sp_def_to_modify,
        )
    }
}

/// Sets the bit for an exclusive item effect.
pub fn set_exclusive_item_effect(
    effect_flags: &mut u32,
    effect_id: exclusive_item_effect_id::Type,
) {
    unsafe { ffi::SetExclusiveItemEffect(effect_flags, effect_id) }
}

/// Tests the exclusive item bitvector for a specific exclusive item effect.
pub fn test_exclusive_item_effect_flag(
    effect_flags: &mut u32,
    effect_id: exclusive_item_effect_id::Type,
) -> bool {
    unsafe { ffi::ExclusiveItemEffectFlagTest(effect_flags, effect_id) > 0 }
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
pub struct AdventureLog;

impl AdventureLog {
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
    pub fn set_monster_battled(&mut self, monster_id: u32) {
        unsafe { ffi::SetPokemonBattled(monster_id) };
    }

    /// Marks one monster as joined.
    pub fn set_monster_joined(&mut self, monster_id: u32) {
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
    pub fn set_special_monster_recruited(&mut self, monster_id: u32) {
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
    pub fn set_item_acquired(&mut self, item_id: u32) {
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

/// Returns whether the specified dungeon is considered as going upward or not
pub fn dungeon_goes_up(dungeon_id: dungeon_catalog::Type) -> bool {
    unsafe { ffi::DungeonGoesUp(dungeon_id) > 0 }
}

/// Checks if a monster ID is an Unown.
pub fn is_unown(monster_id: monster_catalog::Type) -> bool {
    unsafe { ffi::IsUnown(monster_id) > 0 }
}

/// Checks if a monster ID is a Shaymin.
pub fn is_shaymin(monster_id: monster_catalog::Type) -> bool {
    unsafe { ffi::IsShaymin(monster_id) > 0 }
}

/// Checks if a monster ID is a Castform.
pub fn is_castform(monster_id: monster_catalog::Type) -> bool {
    unsafe { ffi::IsCastform(monster_id) > 0 }
}

/// Checks if a monster ID is a Cherrim.
pub fn is_cherrim(monster_id: monster_catalog::Type) -> bool {
    unsafe { ffi::IsCherrim(monster_id) > 0 }
}

/// Checks if a monster ID is a Deoxys.
pub fn is_deoxys(monster_id: monster_catalog::Type) -> bool {
    unsafe { ffi::IsDeoxys(monster_id) > 0 }
}

/// Checks if a given monster is on the exploration team (not necessarily the active party)?
pub fn is_monster_on_team(monster_id: monster_catalog::Type, param_2: i32) -> bool {
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
pub fn iq_skill_flag_test(iq_skill_flags: &mut u32, iq_id: iq_skill_catalog::Type) -> bool {
    unsafe { ffi::IqSkillFlagTest(iq_skill_flags, iq_id) > 0 }
}

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
        id: item_id_16 {
            _bitfield_align_1: [],
            _bitfield_1: Default::default(),
        },
        amount: 0,
    };
    unsafe { ffi::ItemAtTableIdx(table_idx, &mut out) }
    out
}
