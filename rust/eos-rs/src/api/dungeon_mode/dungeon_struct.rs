use crate::api::dungeon_mode::*;
use crate::api::objects::*;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;
use alloc::vec::Vec;

/// The Rust-safe wrapped master struct that contains the state of the dungeon.
/// Can be owned or mutably borrowed from a low-level [`ffi::dungeon`].
///
/// To get a reference to the global dungeon struct, use [`GlobalDungeonData::get`] and then
/// [`GlobalDungeonData::inner`].
///
/// To access the raw struct, use [`Self::inner`].
pub struct Dungeon<T: AsRef<ffi::dungeon> + AsMut<ffi::dungeon>>(T);

/// Create new dungeon structs.
///
/// To get a reference to the global dungeon struct instead, use [`GlobalDungeonData::get`]
/// and then [`GlobalDungeonData::inner`].
impl Dungeon<ffi::dungeon> {
    /// Create a new empty dungeon struct.
    /// Note that this struct is mostly zeroed and since we don't know all the field values,
    /// it could be (=probably is) invalid.
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Create a new wrapper for an owned dungeon state.
    pub fn from_owned(dungeon: ffi::dungeon) -> Self {
        Self(dungeon)
    }
}

/// Manipulate the dungeon.
impl<T: AsRef<ffi::dungeon> + AsMut<ffi::dungeon>> Dungeon<T> {
    /// Dungeon ID.
    pub fn id(&self) -> dungeon_catalog::Type {
        self.0.as_ref().id.val()
    }

    /// Set Dungeon ID.
    pub fn set_id(&mut self, id: dungeon_catalog::Type) {
        self.0.as_mut().id.set_val(id);
    }

    /// Dungeon group ID.
    pub fn group_id(&self) -> dungeon_group_catalog::Type {
        self.0.as_ref().group_id.val()
    }

    /// Set Dungeon group ID.
    pub fn set_group_id(&mut self, id: dungeon_group_catalog::Type) {
        self.0.as_mut().group_id.set_val(id);
    }

    /// Floor number.
    pub fn floor(&self) -> u8 {
        self.0.as_ref().floor
    }

    /// Set Floor number.
    pub fn set_floor(&mut self, floor: u8) {
        self.0.as_mut().floor = floor;
    }

    /// Rescue floor number.
    pub fn rescue_floor(&self) -> u8 {
        self.0.as_ref().rescue_floor
    }

    /// Set Rescue floor number.
    pub fn set_rescue_floor(&mut self, rescue_floor: u8) {
        self.0.as_mut().rescue_floor = rescue_floor;
    }

    /// Whether or not IQ is disabled.
    pub fn is_iq_disabled(&self) -> bool {
        self.0.as_ref().iq_disabled > 0
    }

    /// Set whether or not IQ is disabled.
    pub fn set_iq_disabled(&mut self, value: bool) {
        self.0.as_mut().iq_disabled = value as ffi::bool_
    }

    /// Whether or not recruiting is enabled.
    pub fn is_recruiting_enabled(&self) -> bool {
        self.0.as_ref().recruiting_enabled > 0
    }

    /// Set whether or not recruiting is enabled.
    pub fn set_recruiting_enabled(&mut self, value: bool) {
        self.0.as_mut().recruiting_enabled = value as ffi::bool_
    }

    /// Whether or not the current mission is a story mission.
    ///
    /// If not, allows leader changing and continuing without the partner
    pub fn is_story_mission(&self) -> bool {
        self.0.as_ref().nonstory_flag == 0
    }

    /// Set whether or not the current mission is a story mission.
    ///
    /// If not, allows leader changing and continuing without the partner
    pub fn set_story_mission(&mut self, value: bool) {
        self.0.as_mut().nonstory_flag = !value as ffi::bool_
    }

    /// Whether or not sending home is disabled.
    pub fn is_send_home_disabled(&self) -> bool {
        self.0.as_ref().send_home_disabled > 0
    }

    /// Set whether or not sending home is disabled.
    pub fn set_send_home_disabled(&mut self, value: bool) {
        self.0.as_mut().send_home_disabled = value as ffi::bool_
    }

    /// Hidden Land flag.
    ///
    /// Disables sending home/leader changing, lose if partner faints. Set for dungeons
    /// between DUNGEON_HIDDEN_LAND and DUNGEON_TEMPORAL_PINNACLE.
    pub fn is_hidden_land(&self) -> bool {
        self.0.as_ref().send_home_disabled > 0
    }

    /// Set whether or not sending home is disabled.
    ///
    /// Disables sending home/leader changing, lose if partner faints. Set for dungeons
    /// between DUNGEON_HIDDEN_LAND and DUNGEON_TEMPORAL_PINNACLE.
    pub fn set_hidden_land(&mut self, value: bool) {
        self.0.as_mut().send_home_disabled = value as ffi::bool_
    }

    /// Info about the next mission destination floor, if applicable
    pub fn get_mission_destination(&self) -> &ffi::mission_destination_info {
        &self.0.as_ref().mission_destination
    }

    /// Info about the next mission destination floor, if applicable
    pub fn get_mission_destination_mut(&mut self) -> &mut ffi::mission_destination_info {
        &mut self.0.as_mut().mission_destination
    }

    /// Gets the fractional turn speed.
    ///
    /// Controls when a monster at a certain speed stage is able to act.
    ///
    /// At normal speed, this will tick up by 4 each turn (can act when x % 4 == 3)
    /// At +1 speed, ticks up by 2 each turn (can act when x % 2 == 1)
    /// At +2 speed, ticks up by 1 or 2 each turn (can act when x % 4 != 0)
    /// At +3 speed, ticks up by 1 each turn (an act every tick)
    pub fn get_fractional_turn(&self) -> u16 {
        self.0.as_ref().fractional_turn
    }

    /// Sets the fractional turn speed.
    ///
    /// Controls when a monster at a certain speed stage is able to act.
    ///
    /// At normal speed, this will tick up by 4 each turn (can act when x % 4 == 3)
    /// At +1 speed, ticks up by 2 each turn (can act when x % 2 == 1)
    /// At +2 speed, ticks up by 1 or 2 each turn (can act when x % 4 != 0)
    /// At +3 speed, ticks up by 1 each turn (an act every tick)
    pub fn set_fractional_turn(&mut self, value: u16) {
        self.0.as_mut().fractional_turn = value
    }

    /// Enemy spawn counter.
    ///
    /// Counts from 0-35, spawns happen at 0.
    pub fn get_enemy_spawn_counter(&self) -> u16 {
        self.0.as_ref().enemy_spawn_counter
    }

    /// Sets the enemy spawn counter.
    ///
    /// Counts from 0-35, spawns happen at 0.
    pub fn set_enemy_spawn_counter(&mut self, value: u16) {
        self.0.as_mut().enemy_spawn_counter = value
    }

    /// Countdown to the wind blowing you out of the dungeon.
    pub fn get_wind_turns(&self) -> i16 {
        self.0.as_ref().wind_turns
    }

    /// Sets the wind countdown.
    pub fn set_wind_turns(&mut self, value: i16) {
        self.0.as_mut().wind_turns = value
    }

    /// Enemy density. 0, prevents the enemy_spawn_counter for increasing
    pub fn get_enemy_density(&self) -> u16 {
        self.0.as_ref().enemy_density
    }

    /// Sets the enemy density. 0, prevents the enemy_spawn_counter for increasing
    pub fn set_enemy_density(&mut self, value: u16) {
        self.0.as_mut().enemy_density = value
    }

    /// If you've stolen from Kecleon (actual dungeon state)
    pub fn is_thief_alert(&self) -> bool {
        self.0.as_ref().thief_alert as ffi::bool_ > 0
    }

    /// Sets the thief alert state.
    pub fn set_thief_alert(&mut self, value: bool) {
        self.0.as_mut().thief_alert = value as ffi::bool_
    }

    /// If you've stolen from Kecleon (triggers music and other events?)
    pub fn is_thief_alert_event(&self) -> bool {
        self.0.as_ref().thief_alert_event as ffi::bool_ > 0
    }

    /// Sets the thief alert event state.
    pub fn set_thief_alert_event(&mut self, value: bool) {
        self.0.as_mut().thief_alert_event = value as ffi::bool_
    }

    /// You Entered a Monster House (actual dungeon state)
    pub fn is_monster_house_triggered(&self) -> bool {
        self.0.as_ref().monster_house_triggered as ffi::bool_ > 0
    }

    /// Sets the monster house triggered state.
    pub fn set_monster_house_triggered(&mut self, value: bool) {
        self.0.as_mut().monster_house_triggered = value as ffi::bool_
    }

    /// You Entered a Monster House (triggers music and other events?)
    pub fn is_monster_house_triggered_event(&self) -> bool {
        self.0.as_ref().monster_house_triggered_event as ffi::bool_ > 0
    }

    /// Sets the monster house triggered event state.
    pub fn set_monster_house_triggered_event(&mut self, value: bool) {
        self.0.as_mut().monster_house_triggered_event = value as ffi::bool_
    }

    /// Objective of the current dungeon. Returns None if the objective is invalid.
    pub fn get_dungeon_objective(&self) -> Option<DungeonObjective> {
        self.0.as_ref().dungeon_objective.val().try_into().ok()
    }

    /// Set objective of the current dungeon
    pub fn set_dungeon_objective(&mut self, objective: DungeonObjective) {
        self.0
            .as_mut()
            .dungeon_objective
            .set_val(objective as ffi::dungeon_objective::Type)
    }

    /// Gets the number of times the player can still be rescued in this dungeon.
    pub fn get_rescue_attempts_left(&self) -> u8 {
        self.0.as_ref().rescue_attempts_left
    }

    /// Sets the number of times the player can still be rescued in this dungeon.
    pub fn set_rescue_attempts_left(&mut self, value: u8) {
        self.0.as_mut().rescue_attempts_left = value
    }

    /// Dungeon generation info.
    pub fn get_dungeon_generation_info(&self) -> &ffi::dungeon_generation_info {
        &self.0.as_ref().gen_info
    }

    /// Dungeon generation info.
    pub fn get_dungeon_generation_info_mut(&mut self) -> &mut ffi::dungeon_generation_info {
        &mut self.0.as_mut().gen_info
    }

    /// Get the current weather. Returns None if the weather is invalid.
    pub fn get_weather(&self) -> Option<Weather> {
        self.0.as_ref().weather.val().try_into().ok()
    }

    /// Sets the current weather
    pub fn set_weather(&mut self, weather: Weather) {
        self.0
            .as_mut()
            .weather
            .set_val(weather as ffi::weather_id::Type)
    }

    /// Get the natural weather of this floor. Returns None if the weather is invalid.
    pub fn get_natural_weather(&self) -> Option<Weather> {
        self.0.as_ref().natural_weather.val().try_into().ok()
    }

    /// sets the natural weather of this floor.
    pub fn set_natural_weather(&mut self, weather: Weather) {
        self.0
            .as_mut()
            .weather
            .set_val(weather as ffi::weather_id::Type)
    }

    /// Turns left for each weather type in enum weather_id (except [`Weather::Random`]). If
    /// multiple of these are nonzero, the one with the highest number of turns left is chosen.
    /// Ties are broken in enum order
    pub fn get_weather_turns(&self) -> &[u16; 8] {
        &self.0.as_ref().weather_turns
    }

    /// Turns left for each weather type in enum weather_id (except [`Weather::Random`]). If
    /// multiple of these are nonzero, the one with the highest number of turns left is chosen.
    /// Ties are broken in enum order
    pub fn get_weather_turns_mut(&mut self) -> &mut [u16; 8] {
        &mut self.0.as_mut().weather_turns
    }

    /// Turns left for artificial permaweather from weather-setting abilities like Drought,
    /// Sand Stream, Drizzle, and Snow Warning; one counter for each weather type in enum
    /// weather_id (except WEATHER_RANDOM). Any nonzero value triggers that weather condition
    /// (it's usually set  to 1 or 0). If the weather's source is removed, this value becomes the
    /// normal number of turns  left for that weather condition. Priority in the event of multiple
    /// nonzero counters is the same as with weather_turns.
    pub fn get_artificial_permaweather_turns(&self) -> &[u16; 8] {
        &self.0.as_ref().artificial_permaweather_turns
    }

    /// Turns left for artificial permaweather from weather-setting abilities like Drought,
    /// Sand Stream, Drizzle, and Snow Warning; one counter for each weather type in enum
    /// weather_id (except WEATHER_RANDOM). Any nonzero value triggers that weather condition
    /// (it's usually set  to 1 or 0). If the weather's source is removed, this value becomes the
    /// normal number of turns  left for that weather condition. Priority in the event of multiple
    /// nonzero counters is the same as with weather_turns.
    pub fn set_artificial_permaweather_turns(&mut self) -> &mut [u16; 8] {
        &mut self.0.as_mut().artificial_permaweather_turns
    }

    /// For damaging weather conditions like sandstorm. Counts down from 9-0, damage on 9.
    pub fn get_weather_damage_counter(&self) -> u8 {
        self.0.as_ref().weather_damage_counter
    }

    /// Sets the weather damage counter.
    pub fn set_weather_damage_counter(&mut self, counter: u8) {
        self.0.as_mut().weather_damage_counter = counter
    }

    /// Number of turns left for the Mud Sport condition.
    pub fn get_mud_sport_turns(&self) -> u8 {
        self.0.as_ref().mud_sport_turns
    }

    /// Sets the number of turns left for the Mud Sport condition.
    pub fn set_mud_sport_turns(&mut self, counter: u8) {
        self.0.as_mut().mud_sport_turns = counter
    }

    /// Number of turns left for the Water Sport condition.
    pub fn get_water_sport_turns(&self) -> u8 {
        self.0.as_ref().water_sport_turns
    }

    /// Sets the number of turns left for the Water Sport condition.
    pub fn set_water_sport_turns(&mut self, counter: u8) {
        self.0.as_mut().water_sport_turns = counter
    }

    /// Whether or not current weather is nullified by Cloud Nine or Air Lock.
    pub fn is_weather_nullified(&self) -> bool {
        self.0.as_ref().nullify_weather > 0
    }

    /// Sets whether or not current weather is nullified by Cloud Nine or Air Lock.
    pub fn set_weather_nullified(&mut self, nullified: bool) {
        self.0.as_mut().nullify_weather = nullified as ffi::bool_
    }

    /// Whether or not Gravity is in effect.
    pub fn is_gravity_in_effect(&self) -> bool {
        self.0.as_ref().gravity > 0
    }

    /// Sets whether or not Gravity is in effect.
    pub fn set_gravity_in_effect(&mut self, gravity: bool) {
        self.0.as_mut().gravity = gravity as ffi::bool_
    }

    /// Entity that is taking its turn at this moment.
    pub fn get_current_active_entity(&self) -> Option<&DungeonEntity> {
        let ptr = self.0.as_ref().current_active_entity;
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }

    /// Entity that is taking its turn at this moment.
    pub fn get_current_active_entity_mut(&self) -> Option<&mut DungeonEntity> {
        let ptr = self.0.as_ref().current_active_entity;
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *ptr })
        }
    }

    /// Gets a reference to the entity that is currently leading the team, or None if none of
    /// the first 4 entities is a valid monster with its is_team_leader flag set.
    // Needs a mut reference, since:
    // It also sets LEADER_PTR to the result before returning it.
    pub fn get_leader(&mut self) -> Option<&DungeonEntity> {
        unsafe {
            let ptr = ffi::GetLeader();
            if ptr.is_null() {
                None
            } else {
                Some(&*ptr)
            }
        }
    }

    /// Gets a mutable reference to the entity that is currently leading the team, or None if none
    /// of the first 4 entities is a valid monster with its is_team_leader flag set.
    pub fn get_leader_mut(&mut self) -> Option<&mut DungeonEntity> {
        unsafe {
            let ptr = ffi::GetLeader();
            if ptr.is_null() {
                None
            } else {
                Some(&mut *ptr)
            }
        }
    }

    /// Monster that will become the leader of the team after changing leaders.
    pub fn get_new_leader(&self) -> Option<&DungeonEntity> {
        let ptr = self.0.as_ref().new_leader;
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }

    /// Monster that will become the leader of the team after changing leaders.
    pub fn get_new_leader_mut(&self) -> Option<&mut DungeonEntity> {
        let ptr = self.0.as_ref().new_leader;
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *ptr })
        }
    }

    /// Get whether the floor will be advanced at the end of the turn (unless the leader fainted).
    pub fn get_end_floor_flag(&self) -> bool {
        self.0.as_ref().end_floor_flag > 0
    }

    /// Set whether the floor will be advanced at the end of the turn (unless the leader fainted).
    pub fn set_end_floor_flag(&mut self, flag: bool) {
        self.0.as_mut().end_floor_flag = flag as ffi::bool_
    }

    /// Get whether the floor will be advanced at the end of the turn (even if the leader fainted).
    pub fn get_end_floor_flag_force(&self) -> bool {
        self.0.as_ref().end_floor_no_death_check_flag > 0
    }

    /// Set whether the floor will be advanced at the end of the turn (even if the leader fainted).
    pub fn set_end_floor_flag_force(&mut self, flag: bool) {
        self.0.as_mut().end_floor_no_death_check_flag = flag as ffi::bool_
    }

    /// False if the leader isn't doing anything right now. True if it's currently performing
    /// an action (such as walking or attacking)
    pub fn get_leader_action_flag(&self) -> bool {
        self.0.as_ref().no_action_in_progress == 0
    }

    /// Gets the table of all entities.
    pub fn get_entities(&self) -> EntityTableRef {
        EntityTableRef(&self.0.as_ref().entity_table)
    }

    /// Gets the table of all entities, mutably.
    pub fn get_entities_mut(&mut self) -> EntityTableMut {
        EntityTableMut(&mut self.0.as_mut().entity_table)
    }

    /// Gets the tile grid.
    pub fn get_tiles(&self) -> DungeonTileGridRef<56, 32> {
        DungeonTileGridRef(&self.0.as_ref().tile_ptrs)
    }

    /// Gets the tile grid, mutably.
    pub fn get_tiles_mut(&mut self) -> DungeonTileGridMut<56, 32> {
        DungeonTileGridMut(&mut self.0.as_mut().tile_ptrs)
    }

    /// Dungeon floor properties.
    pub fn get_floor_properties(&self) -> &ffi::floor_properties {
        &self.0.as_ref().floor_properties
    }

    /// Dungeon floor properties.
    pub fn get_floor_properties_mut(&mut self) -> &mut ffi::floor_properties {
        &mut self.0.as_mut().floor_properties
    }

    /// Color table. Used to apply a tint to the colors shown on screen.
    /// Changes depending on the current weather.
    pub fn get_color_table(&self) -> &[ffi::rgb; 256] {
        &self.0.as_ref().color_table
    }

    /// Color table, mutably. Used to apply a tint to the colors shown on screen.
    /// Changes depending on the current weather.
    pub fn get_color_table_mut(&mut self) -> &mut [ffi::rgb; 256] {
        &mut self.0.as_mut().color_table
    }

    /// Whether the current floor should continue or end and why. Returns None for invalid values.
    pub fn get_floor_loop_status(&self) -> Option<FloorLoopStatus> {
        self.0.as_ref().floor_loop_status.val().try_into().ok()
    }

    /// Whether the current floor should continue or end and why.
    pub fn set_floor_loop_status(&mut self, loop_status: FloorLoopStatus) {
        self.0
            .as_mut()
            .floor_loop_status
            .set_val(loop_status as ffi::floor_loop_status::Type);
    }

    /// If true, the message log won't be shown and the yellow beam animation won't
    /// appear over team members after the leader faints
    pub fn is_skip_faint_animation_flag_set(&self) -> bool {
        self.0.as_ref().skip_faint_animation_flag > 0
    }

    /// If true, the message log won't be shown and the yellow beam animation won't
    /// appear over team members after the leader faints
    pub fn set_skip_faint_animation_flag(&mut self, flag: bool) {
        self.0.as_mut().skip_faint_animation_flag = flag as ffi::bool_
    }

    /// True if the leader is running. Causes the leader's action for the next turn
    /// to be set to [`ffi::action::ACTION_WALK`] until it hits an obstacle.
    pub fn is_leader_running(&self) -> bool {
        self.0.as_ref().leader_running > 0
    }

    /// True if the leader is running. Causes the leader's action for the next turn
    /// to be set to [`ffi::action::ACTION_WALK`] until it hits an obstacle.
    pub fn set_leader_running(&mut self, running: bool) {
        self.0.as_mut().leader_running = running as ffi::bool_
    }

    /// List of spawn entries for this floor.
    pub fn get_spawn_entries(&self) -> &[ffi::monster_spawn_entry; 16] {
        &self.0.as_ref().spawn_entries
    }

    /// List of spawn entries for this floor, mutably.
    pub fn get_spawn_entries_mut(&mut self) -> &mut [ffi::monster_spawn_entry; 16] {
        &mut self.0.as_mut().spawn_entries
    }

    /// List of the indices in the complete monster spawn table for this floor that were
    /// spawned in this floor.
    pub fn get_spawn_table_entries_chosen(&self) -> &[u16; 16] {
        &self.0.as_ref().spawn_table_entries_chosen
    }

    /// List of the indices in the complete monster spawn table for this floor that were
    /// spawned in this floor.
    pub fn get_spawn_table_entries_chosen_mut(&mut self) -> &mut [u16; 16] {
        &mut self.0.as_mut().spawn_table_entries_chosen
    }

    /// Highest level among all the enemies that spawn on this floor
    pub fn get_highest_enemy_level(&self) -> u16 {
        self.0.as_ref().highest_enemy_level
    }

    /// Highest level among all the enemies that spawn on this floor
    pub fn set_highest_enemy_level(&mut self, level: u16) {
        self.0.as_mut().highest_enemy_level = level
    }

    /// Returns true if the player should get kicked out of the dungeon if an important team member
    /// (like the partner or certain story allies) faints.
    pub fn should_game_over_on_important_team_member_faint(&self) -> bool {
        unsafe { ffi::ShouldGameOverOnImportantTeamMemberFaint() > 0 }
    }

    /// Returns true if the specified monster is included in the floor's monster spawn list
    /// (the modified list after a maximum of 14 different species were chosen).
    pub fn is_on_monster_spawn_list(monster_id: monster_catalog::Type) -> bool {
        unsafe { ffi::IsOnMonsterSpawnList(monster_id) > 0 }
    }
}

/// Equivalent to [`Dungeon::new`].
impl Default for Dungeon<ffi::dungeon> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsRef<ffi::dungeon> + AsMut<ffi::dungeon>> Dungeon<T> {
    /// Access the raw struct.
    pub fn inner(&self) -> &ffi::dungeon {
        self.0.as_ref()
    }
    /// Access the raw struct mutably.
    pub fn inner_mut(&mut self) -> &mut ffi::dungeon {
        self.0.as_mut()
    }
}

impl AsRef<ffi::dungeon> for ffi::dungeon {
    fn as_ref(&self) -> &ffi::dungeon {
        self
    }
}

impl AsMut<ffi::dungeon> for ffi::dungeon {
    fn as_mut(&mut self) -> &mut ffi::dungeon {
        self
    }
}

/// Note that this struct is mostly zeroed and since we don't know all the field values,
/// it could be (=probably is) invalid.
impl Default for ffi::dungeon {
    // This is the default implementation of bindgen.
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

/// Helper struct for dealing with the current floor, the global dungeon and the mission state.
///
/// This is essentially an extension and wrapper of [`Dungeon`] that works on the
/// global dungeon struct only.
///
/// To generate dungeons and manipulate floors layouts, you can get the game's
/// builtin generator with [`Self::get_builtin_dungeon_generator`], or configure the global
/// state of the current floor correctly and use [`Self::generate_floor`].
pub struct GlobalDungeonData<'a>(&'a OverlayLoadLease<29>, Dungeon<&'a mut ffi::dungeon>);

impl<'a> GlobalDungeonData<'a> {
    /// Checks if the global dungeon pointer is null.
    pub fn is_global_dungeon_ptr_null(_ov29: &OverlayLoadLease<29>) -> bool {
        // SAFETY: We have a lease and know OV29 is loaded.
        unsafe { ffi::GetDungeonPtrMaster() }.is_null()
    }

    /// Returns a mutable reference to the global dungeon struct wrapped in this struct.
    ///
    /// # Safety
    /// This is unsafe, since it borrows the global dungeon struct (essentially a `static mut`)
    /// mutably.
    ///
    /// You need to make sure no other borrows over the global dungeon struct
    /// (or any of it's values) exist.
    pub unsafe fn get(ov29: &'a OverlayLoadLease<29>) -> Self {
        let ptr = ffi::GetDungeonPtrMaster();
        assert!(!ptr.is_null(), "Global dungeon pointer is null!");
        Self(ov29, Dungeon(&mut *ffi::GetDungeonPtrMaster()))
    }

    /// This will allocate a new dungeon struct and update the global dungeon pointer to it.
    ///
    /// # Safety
    /// This is unsafe, since it borrows the global dungeon struct (essentially a `static mut`)
    /// mutably. It also invalidates any previously borrowed global dungeon struct.
    ///
    /// You need to make sure no other borrows over the global dungeon struct
    /// (or any of it's values) exist.
    pub unsafe fn alloc(ov29: &'a OverlayLoadLease<29>) -> Self {
        Self(ov29, Dungeon(&mut *ffi::DungeonAlloc()))
    }

    /// Zeros out the struct pointed to by the global dungeon pointer.
    ///
    /// # Safety
    /// This is unsafe, since it updates the global dungeon struct (essentially a `static mut`).
    pub unsafe fn z_init(&mut self) {
        ffi::DungeonZInit()
    }

    /// Frees the dungeons struct pointer to by the master dungeon pointer,
    /// and nullifies the pointer.
    ///
    /// # Safety
    /// This is unsafe, since it completely invalidates borrows of the old global dungeon struct
    /// and then invalidates the global dungeon pointer.
    pub unsafe fn free(self) {
        ffi::DungeonFree()
    }

    /// Returns a reference to the inner dungeon struct.
    pub fn inner(&self) -> &Dungeon<&'a mut ffi::dungeon> {
        &self.1
    }

    /// Returns a mutable reference to the inner dungeon struct.
    pub fn inner_mut(&mut self) -> &mut Dungeon<&'a mut ffi::dungeon> {
        &mut self.1
    }

    /// Seems to initialize the dungeon struct from specified dungeon data.
    ///
    /// The signature will be updated once we know more about this function.
    pub fn initialize_dungeon(
        &mut self,
        dungeon_data: &mut ffi::undefined,
        dungeon: &mut ffi::dungeon,
    ) -> i32 {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::InitializeDungeon(dungeon_data as *mut _, dungeon as *mut _) }
    }

    /// Returns an abstraction over the game's builtin dungeon generator. This
    /// struct implements [`dungeon_generator::DungeonFloorGeneration`], which is the
    /// recommended way to work with it, see the documentation of [`dungeon_generator`].
    ///
    /// # Note
    /// Note that the builtin dungeon generator works on the global dungeon struct directly.
    pub fn get_builtin_dungeon_generator(
        &'a mut self,
    ) -> dungeon_generator::game_builtin::GlobalDungeonStructureGenerator<'a> {
        dungeon_generator::game_builtin::GlobalDungeonStructureGenerator(self.0.clone(), self)
    }

    /// Generates a dungeon floor.
    ///
    /// If not changed by a patch, this function will use the game's default built in generator
    /// and generate the floor based on the current global configuration for the floor.
    ///
    /// For more granular control over the floor generation, you can get an abstraction over
    /// the builtin generator with [`Self::get_builtin_dungeon_generator`].
    ///
    pub fn generate_floor(&'a mut self) {
        self.get_builtin_dungeon_generator()
            .generate_floor_internal();
    }

    /// Gets the floor type. Returns None if the global dungeon struct contains invalid data.
    pub fn get_floor_type(&self) -> Option<FloorType> {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetFloorType() }.try_into().ok()
    }

    /// Checks if the current fixed floor is the "substitute room" (Fixed Room ID 0x6E).
    pub fn is_substitute_room(&self) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::FixedRoomIsSubstituteRoom() > 0 }
    }

    /// Checks if the current dungeon floor number is even.
    /// This is probably, among other things(?), used to determine whether male or female monsters
    /// should be spawned.
    /// Has a special check to return false for Labyrinth Cave B10F (the Gabite boss fight).
    pub fn is_even_floor(&self) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::FloorNumberIsEven() > 0 }
    }

    /// Returns the tile at the given coordinates.
    ///
    /// If the coordinates are out-of-bounds, this seems to return some kind of default tile.
    pub fn get_tile(&self, x: i32, y: i32) -> &DungeonTile {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { &*ffi::GetTile(x, y) }
    }

    /// Returns the tile at the given coordinates.
    ///
    /// If the coordinates are out-of-bounds, this seems to return some kind of default tile.
    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut DungeonTile {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { &mut *ffi::GetTile(x, y) }
    }

    /// Checks if gravity is active on the floor.
    pub fn is_gravity_active(&self) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GravityIsActive() > 0 }
    }

    /// Checks if the current floor is the Secret Bazaar.
    pub fn is_secret_bazaar(&self) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsSecretBazaar() > 0 }
    }

    /// Checks if the current floor is the Secret Room fixed floor (from hidden stairs).
    pub fn is_secret_room(&self) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
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
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsNormalFloor() > 0 }
    }

    /// Checks if a fixed room ID corresponds to a fixed, full-floor layout.
    pub fn is_full_floor_fixed_rooms(&self, fixed_room_id: fixed_room_catalog::Type) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsNotFullFloorFixedRoom(fixed_room_id) == 0 }
    }
    /// Checks if a position (x, y) is out of bounds on the map:
    /// !((0 <= x <= 55) && (0 <= y <= 31)).
    pub fn is_pos_out_of_bounds(&self, x: i32, y: i32) -> bool {
        // SAFETY:We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::PosIsOutOfBounds(x, y) > 0 }
    }

    /// Checks if the current floor is either the Secret Bazaar or a Secret Room.
    pub fn is_hidden_stairs_floor(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsHiddenStairsFloor() > 0 }
    }

    /// Checks if the current floor is an active mission destination of type
    /// [`MissionType::TakeItemFromOutlaw`], [`MissionType::ArrestOutlaw`] or
    /// [`MissionType::ChallengeRequest`].
    pub fn is_outlaw_or_challenge_request_floor(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsOutlawOrChallengeRequestFloor() > 0 }
    }

    /// Checks if the current floor is an active mission destination of a given type group.
    pub fn is_current_mission_type(&self, mission_type_group: MissionTypeGroup) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsCurrentMissionType(mission_type_group as ffi::mission_type::Type) > 0 }
    }

    /// Checks if the current floor is an active mission destination of a given type.
    pub fn is_current_mission_type_exact(&self, mission_type: MissionType) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe {
            ffi::IsCurrentMissionTypeExact(
                mission_type.group() as ffi::mission_type::Type,
                mission_type.c_subtype(),
            ) > 0
        }
    }

    /// Checks if the current floor is a mission destination for a Monster House outlaw mission.
    pub fn is_outlaw_monster_house_floor(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsOutlawMonsterHouseFloor() > 0 }
    }

    /// Checks if the current floor is a Golden Chamber floor.
    pub fn is_golden_chamber(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsGoldenChamber() > 0 }
    }

    /// Checks if the current floor is a boss floor for a Legendary Challenge Letter mission.
    pub fn is_legendary_challenge_floor(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsLegendaryChallengeFloor() > 0 }
    }

    /// Checks if the current floor is the boss floor in Star Cave Pit for Jirachi's
    /// Challenge Letter mission.
    pub fn is_jirachi_challenge_floor(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsJirachiChallengeFloor() > 0 }
    }

    /// Checks if the current floor is a mission destination floor with a special monster.
    ///
    /// See [`Self::floor_has_mission_monster`] for details.
    pub fn is_destination_floor_with_monster(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsDestinationFloorWithMonster() > 0 }
    }

    /// Checks if a given floor is a mission destination with a special monster, either a target to rescue or an enemy to defeat.
    ///
    /// Mission types with a monster on the destination floor:
    /// - Rescue client
    /// - Rescue target
    /// - Escort to target
    /// - Deliver item
    /// - Search for target
    /// - Take item from outlaw
    /// - Arrest outlaw
    /// - Challenge Request
    pub fn floor_has_mission_monster(&self) -> bool {
        Self::floor_has_mission_monster_static(&self.1.inner().mission_destination, self.0)
    }

    /// Static version of [`Self::floor_has_mission_monster`]. See it for details.
    pub fn floor_has_mission_monster_static(
        mission_destination: &ffi::mission_destination_info,
        _ov29: &OverlayLoadLease<29>,
    ) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::FloorHasMissionMonster(force_mut_ptr!(mission_destination)) > 0 }
    }

    /// Checks if the target enemy of the mission on the current floor has been defeated.
    pub fn is_mission_target_enemy_defeated(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::MissionTargetEnemyIsDefeated() > 0 }
    }

    /// Set the flag for whether or not the target enemy of the current mission has been defeated.
    pub fn set_mission_target_enemy_defeated(&mut self, flag: bool) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::SetMissionTargetEnemyDefeated(flag as ffi::bool_) }
    }

    /// Checks if the current floor is a mission destination floor with a fixed room.
    ///
    /// The entire floor can be a fixed room layout, or it can just contain a Sealed Chamber.
    pub fn is_destination_floor_with_fixed_room(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsDestinationFloorWithFixedRoom() > 0 }
    }

    /// Get the ID of the item that needs to be retrieve on the current floor for a mission,
    /// if one exists.
    pub fn get_item_to_retrieve(&self) -> item_catalog::Type {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetItemToRetrieve() }
    }

    /// Get the ID of the item that needs to be delivered to a mission client on the current floor,
    /// if one exists.
    pub fn get_item_to_deliver(&self) -> item_catalog::Type {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetItemToDeliver() }
    }

    /// Get the ID of the special target item for a Sealed Chamber or Treasure Memo mission on
    /// the current floor.
    pub fn get_special_target_item(&self) -> item_catalog::Type {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetSpecialTargetItem() }
    }

    /// Checks if the current floor is a mission destination floor with a special item.
    ///
    /// This excludes missions involving taking an item from an outlaw.
    pub fn is_destination_floor_with_item(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsDestinationFloorWithItem() > 0 }
    }

    /// Checks if the current floor is a mission destination floor with a "hidden outlaw" that
    /// behaves like a normal enemy.
    pub fn is_destination_floor_with_hidden_outlaw(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsDestinationFloorWithHiddenOutlaw() > 0 }
    }

    /// Checks if the current floor is a mission destination floor with a "fleeing outlaw" that
    /// runs away.
    pub fn is_destination_floor_with_fleeing_outlaw(&self) -> bool {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::IsDestinationFloorWithFleeingOutlaw() > 0 }
    }

    /// Get the monster ID of the target enemy to be defeated on the current floor for a mission,
    /// if one exists.
    pub fn get_mission_target_enemy(&self) -> monster_catalog::Type {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetMissionTargetEnemy() }
    }

    /// Get the monster ID of the specified minion group on the current floor for a mission,
    /// if it exists.
    ///
    /// Note that a single minion group can correspond to multiple actual minions of the same
    /// species. There can be up to 2 minion groups.
    pub fn get_mission_enemy_minion_group(&self, minion_group_index: i32) -> monster_catalog::Type {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetMissionEnemyMinionGroup(minion_group_index) }
    }

    /// Loads fixed room data from BALANCE/fixed.bin into the buffer pointed to by
    /// `FIXED_ROOM_DATA_PTR` (see symbol table).
    ///
    /// # Safety
    /// This modifies a global buffer. `FIXED_ROOM_DATA_PTR` needs to be valid, no references
    /// to it's content must exist.
    pub unsafe fn load_fixed_room_data(&mut self) {
        ffi::LoadFixedRoomData()
    }

    /// Sets the forced loss reason to a given value.
    pub fn set_forced_loss_reason(&mut self, reason: ForcedLossReason) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::SetForcedLossReason(reason as ffi::forced_loss_reason::Type) }
    }

    /// Gets the forced loss reason, returns None if it's invalid.
    pub fn get_forced_loss_reason(&self) -> Option<ForcedLossReason> {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetForcedLossReason() }.try_into().ok()
    }

    /// Gets the sprite index of the specified monster on this floor
    pub fn get_monster_sprite_index(&self, monster_idx: monster_catalog::Type) -> u16 {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetSpriteIndex(monster_idx) }
    }

    /// Handles a fainted monster (reviving does not count as fainting).
    ///
    /// # Arguments
    /// * `fainted_entity`: Fainted entity
    /// * `faint_reason`: Faint reason (move ID or greater than the max move id for other causes)
    /// * `killer`: Entity responsible of the fainting
    pub fn handle_faint(
        &mut self,
        fainted_entity: &mut DungeonEntity,
        faint_reason: i32,
        killer: &mut DungeonEntity,
    ) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::HandleFaint(fainted_entity, faint_reason, killer) }
    }

    /// Returns a reference to the minimap_display_data struct in the dungeon struct, if it
    /// exists.
    pub fn get_minimap_data(&self) -> Option<&ffi::minimap_display_data> {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        let ptr = unsafe { ffi::GetMinimapData() };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }

    /// Returns a mutable reference to the minimap_display_data struct in the dungeon struct, if it
    /// exists.
    pub fn get_minimap_data_mut(&mut self) -> Option<&mut ffi::minimap_display_data> {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        let ptr = unsafe { ffi::GetMinimapData() };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &mut *ptr })
        }
    }

    /// Sets [`ffi::minimap_display_data::field_0xE447`] to the specified value
    pub fn set_minimap_data_e447(&mut self, value: u8) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::SetMinimapDataE447(value) }
    }

    /// Gets the value of [`ffi::minimap_display_data::field_0xE447`].
    pub fn get_minimap_data_e447(&mut self) -> u8 {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::GetMinimapDataE447() }
    }

    /// Sets [`ffi::minimap_display_data::field_0xE448`] to the specified value
    pub fn set_minimap_data_e448(&mut self, value: u8) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::SetMinimapDataE448(value) }
    }

    /// Opens the message log window.
    pub fn open_message_log(&mut self, param_1: ffi::undefined4, param_2: ffi::undefined4) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::OpenMessageLog(param_1, param_2) }
    }

    /// Checks if a given dungeon tip was not displayed yet and if so, displays it.
    ///
    /// If `log` is true, the tip will also be written to the message log.
    pub fn display_dungeon_tip(&mut self, tip: &mut ffi::message_tip, log: bool) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::DisplayDungeonTip(tip, log as ffi::bool_) }
    }

    /// Displays a message in a dialogue box that optionally waits for player input before closing.
    pub fn display_message(
        &mut self,
        param_1: ffi::undefined4,
        message_id: i32,
        wait_for_input: bool,
    ) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::DisplayMessage(param_1, message_id, wait_for_input as ffi::bool_) }
    }

    /// Displays a message in a dialogue box that optionally waits for player input before closing.
    pub fn display_message2(
        &mut self,
        param_1: ffi::undefined4,
        message_id: i32,
        wait_for_input: bool,
    ) {
        // SAFETY: We hold a valid mutable reference to the global dungeon struct.
        unsafe { ffi::DisplayMessage2(param_1, message_id, wait_for_input as ffi::bool_) }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "eu")))]
    #[cfg(feature = "eu")]
    /// This function is exclusive to the EU ROM. Seems to perform a check to see if the monster
    /// who just fainted was a team member who should cause the minimap to be updated (or something
    /// like that, maybe related to the Map Surveyor IQ skill) and if it passes, updates the
    /// minimap.
    ///
    /// The function ends by calling another 2 functions. In US ROMs, calls to this are
    /// replaced by calls to those two functions. This seems to indicate that this function fixes
    /// some edge case glitch that can happen when a team member faints.
    ///
    /// # Arguments
    /// * `non_team_member_fainted` - False if the fainted entity was a team member.
    /// * `set_unk_byte` - True to set an unknown byte in the RAM to 1.
    pub fn faint_check(&mut self, non_team_member_fainted: bool, set_unk_byte: bool) {
        unsafe {
            ffi::EuFaintCheck(
                non_team_member_fainted as ffi::bool_,
                set_unk_byte as ffi::bool_,
            )
        }
    }
}

/// Functions for reading data from an entity table.
pub trait EntityTableRead {
    /// All monsters, whether they're used or not.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_monsters(&self) -> Vec<&DungeonEntity>;

    /// Actually used monsters.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_active_monsters(&self) -> Vec<&DungeonEntity>;

    /// All items.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_items(&self) -> Vec<&DungeonEntity>;

    /// All traps.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_traps(&self) -> Vec<&DungeonEntity>;

    /// Hidden stairs entity.
    ///
    /// Returns None if null. Note that it still may be invalid, check the validity flag!
    fn get_hidden_stairs(&self) -> Option<&DungeonEntity>;
}

/// Functions for writing data into an entity table.
pub trait EntityTableWrite: EntityTableRead {
    /// All monsters, whether they're used or not.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_monsters_mut(&mut self) -> Vec<&mut DungeonEntity>;

    /// Actually used monsters.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_active_monsters_mut(&mut self) -> Vec<&mut DungeonEntity>;

    /// All items.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_items_mut(&mut self) -> Vec<&mut DungeonEntity>;

    /// All traps.
    ///
    /// Null entries are not included, and reading is stopped at them.
    /// Note that some may be invalid, check the validity flag!
    fn get_traps_mut(&mut self) -> Vec<&mut DungeonEntity>;

    /// Hidden stairs entity.
    ///
    /// Returns None if null. Note that it still may be invalid, check the validity flag!
    fn get_hidden_stairs_mut(&mut self) -> Option<&mut DungeonEntity>;
}

/// See [`EntityTableRead`].
pub struct EntityTableRef<'a>(&'a ffi::entity_table);
/// See [`EntityTableRead`] and [`EntityTableWrite`].
pub struct EntityTableMut<'a>(&'a mut ffi::entity_table);

impl<'a> EntityTableRead for EntityTableRef<'a> {
    fn get_monsters(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.monster_slot_ptrs)
    }

    fn get_active_monsters(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.active_monster_ptrs)
    }

    fn get_items(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.item_ptrs)
    }

    fn get_traps(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.trap_ptrs)
    }

    fn get_hidden_stairs(&self) -> Option<&DungeonEntity> {
        let ptr = self.0.header.hidden_stairs_ptr;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: We checked the pointer.
            Some(unsafe { &*ptr })
        }
    }
}

impl<'a> EntityTableRead for EntityTableMut<'a> {
    fn get_monsters(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.monster_slot_ptrs)
    }

    fn get_active_monsters(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.active_monster_ptrs)
    }

    fn get_items(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.item_ptrs)
    }

    fn get_traps(&self) -> Vec<&DungeonEntity> {
        check_and_return(&self.0.header.trap_ptrs)
    }

    fn get_hidden_stairs(&self) -> Option<&DungeonEntity> {
        let ptr = self.0.header.hidden_stairs_ptr;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: We checked the pointer.
            Some(unsafe { &*ptr })
        }
    }
}

impl<'a> EntityTableWrite for EntityTableMut<'a> {
    fn get_monsters_mut(&mut self) -> Vec<&mut DungeonEntity> {
        check_and_return_mut(&mut self.0.header.monster_slot_ptrs)
    }

    fn get_active_monsters_mut(&mut self) -> Vec<&mut DungeonEntity> {
        check_and_return_mut(&mut self.0.header.active_monster_ptrs)
    }

    fn get_items_mut(&mut self) -> Vec<&mut DungeonEntity> {
        check_and_return_mut(&mut self.0.header.item_ptrs)
    }

    fn get_traps_mut(&mut self) -> Vec<&mut DungeonEntity> {
        check_and_return_mut(&mut self.0.header.trap_ptrs)
    }

    fn get_hidden_stairs_mut(&mut self) -> Option<&mut DungeonEntity> {
        let ptr = self.0.header.hidden_stairs_ptr;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: We checked the pointer.
            Some(unsafe { &mut *ptr })
        }
    }
}

fn check_and_return(ent: &[*mut ffi::entity]) -> Vec<&DungeonEntity> {
    let mut res: Vec<&DungeonEntity> = Vec::with_capacity(ent.len());
    for e in ent {
        if e.is_null() {
            break;
        }
        // SAFETY: We checked the pointer.
        res.push(unsafe { &**e });
    }
    res
}

fn check_and_return_mut(ent: &mut [*mut ffi::entity]) -> Vec<&mut DungeonEntity> {
    let mut res: Vec<&mut DungeonEntity> = Vec::with_capacity(ent.len());
    for e in ent {
        if e.is_null() {
            break;
        }
        // SAFETY: We checked the pointer.
        res.push(unsafe { &mut **e });
    }
    res
}
