use crate::ffi;
use crate::ffi::type_matchup::Type;

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
/// Move index of a monster, used by some functions.
pub enum TargetTypeIndex {
    FirstType = 0,
    SecondType = 1,
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Describes the effectiveness of a move's type against type(s).
pub enum DungeonTypeMatchup {
    Immune = ffi::type_matchup::MATCHUP_IMMUNE,
    NotVeryEffective = ffi::type_matchup::MATCHUP_NOT_VERY_EFFECTIVE,
    Neutral = ffi::type_matchup::MATCHUP_NEUTRAL,
    SuperEffective = ffi::type_matchup::MATCHUP_SUPER_EFFECTIVE,
}

impl TryFrom<ffi::type_matchup::Type> for DungeonTypeMatchup {
    type Error = ();

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            ffi::type_matchup::MATCHUP_IMMUNE => Ok(DungeonTypeMatchup::Immune),
            ffi::type_matchup::MATCHUP_NOT_VERY_EFFECTIVE => {
                Ok(DungeonTypeMatchup::NotVeryEffective)
            }
            ffi::type_matchup::MATCHUP_NEUTRAL => Ok(DungeonTypeMatchup::Neutral),
            ffi::type_matchup::MATCHUP_SUPER_EFFECTIVE => Ok(DungeonTypeMatchup::SuperEffective),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of an entity in a dungeon.
pub enum DungeonEntityType {
    Nothing = ffi::entity_type::ENTITY_NOTHING,
    Monster = ffi::entity_type::ENTITY_MONSTER,
    Trap = ffi::entity_type::ENTITY_TRAP,
    Item = ffi::entity_type::ENTITY_ITEM,
    HiddenStairs = ffi::entity_type::ENTITY_HIDDEN_STAIRS,
}

impl TryFrom<ffi::entity_type::Type> for DungeonEntityType {
    type Error = ();

    fn try_from(value: ffi::entity_type::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::entity_type::ENTITY_NOTHING => Ok(DungeonEntityType::Nothing),
            ffi::entity_type::ENTITY_MONSTER => Ok(DungeonEntityType::Monster),
            ffi::entity_type::ENTITY_TRAP => Ok(DungeonEntityType::Trap),
            ffi::entity_type::ENTITY_ITEM => Ok(DungeonEntityType::Item),
            ffi::entity_type::ENTITY_HIDDEN_STAIRS => Ok(DungeonEntityType::HiddenStairs),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The category of a move.
pub enum MoveCategory {
    None = ffi::move_category::CATEGORY_NONE,
    Physical = ffi::move_category::CATEGORY_PHYSICAL,
    Special = ffi::move_category::CATEGORY_SPECIAL,
    Status = ffi::move_category::CATEGORY_STATUS,
}

impl TryFrom<ffi::move_category::Type> for MoveCategory {
    type Error = ();

    fn try_from(value: ffi::move_category::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::move_category::CATEGORY_NONE => Ok(MoveCategory::None),
            ffi::move_category::CATEGORY_PHYSICAL => Ok(MoveCategory::Physical),
            ffi::move_category::CATEGORY_SPECIAL => Ok(MoveCategory::Special),
            ffi::move_category::CATEGORY_STATUS => Ok(MoveCategory::Status),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of a floor.
pub enum FloorType {
    /// The floor is neither a fixed floor nor does it contain a rescue point.
    Normal = ffi::floor_type::FLOOR_TYPE_NORMAL,
    /// The floor is a fixed floor.
    Fixed = ffi::floor_type::FLOOR_TYPE_FIXED,
    /// The floor has a rescue point.
    Rescue = ffi::floor_type::FLOOR_TYPE_RESCUE,
}

impl TryFrom<ffi::floor_type::Type> for FloorType {
    type Error = ();

    fn try_from(value: ffi::floor_type::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::floor_type::FLOOR_TYPE_NORMAL => Ok(FloorType::Normal),
            ffi::floor_type::FLOOR_TYPE_FIXED => Ok(FloorType::Fixed),
            ffi::floor_type::FLOOR_TYPE_RESCUE => Ok(FloorType::Rescue),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of terrain of a tile.
pub enum TerrainType {
    Wall = ffi::terrain_type::TERRAIN_WALL,
    Normal = ffi::terrain_type::TERRAIN_NORMAL,
    Secondary = ffi::terrain_type::TERRAIN_SECONDARY,
    Chasm = ffi::terrain_type::TERRAIN_CHASM,
}

impl TryFrom<ffi::terrain_type::Type> for TerrainType {
    type Error = ();

    fn try_from(value: ffi::terrain_type::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::terrain_type::TERRAIN_WALL => Ok(TerrainType::Wall),
            ffi::terrain_type::TERRAIN_NORMAL => Ok(TerrainType::Normal),
            ffi::terrain_type::TERRAIN_SECONDARY => Ok(TerrainType::Secondary),
            ffi::terrain_type::TERRAIN_CHASM => Ok(TerrainType::Chasm),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// The type of secondary terrain of a tile.
pub enum SecondaryTerrainType {
    Water = ffi::secondary_terrain_type::SECONDARY_TERRAIN_WATER,
    Lava = ffi::secondary_terrain_type::SECONDARY_TERRAIN_LAVA,
    Chasm = ffi::secondary_terrain_type::SECONDARY_TERRAIN_CHASM,
}

impl TryFrom<ffi::secondary_terrain_type::Type> for SecondaryTerrainType {
    type Error = ();

    fn try_from(value: ffi::secondary_terrain_type::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::secondary_terrain_type::SECONDARY_TERRAIN_WATER => Ok(SecondaryTerrainType::Water),
            ffi::secondary_terrain_type::SECONDARY_TERRAIN_LAVA => Ok(SecondaryTerrainType::Lava),
            ffi::secondary_terrain_type::SECONDARY_TERRAIN_CHASM => Ok(SecondaryTerrainType::Chasm),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Types of weather.
pub enum Weather {
    Clear = ffi::weather_id::WEATHER_CLEAR,
    Sunny = ffi::weather_id::WEATHER_SUNNY,
    Sandstorm = ffi::weather_id::WEATHER_SANDSTORM,
    Cloudy = ffi::weather_id::WEATHER_CLOUDY,
    Rain = ffi::weather_id::WEATHER_RAIN,
    Hail = ffi::weather_id::WEATHER_HAIL,
    Fog = ffi::weather_id::WEATHER_FOG,
    Snow = ffi::weather_id::WEATHER_SNOW,
    Random = ffi::weather_id::WEATHER_RANDOM,
}

impl TryFrom<ffi::weather_id::Type> for Weather {
    type Error = ();

    fn try_from(value: ffi::weather_id::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::weather_id::WEATHER_CLEAR => Ok(Weather::Clear),
            ffi::weather_id::WEATHER_SUNNY => Ok(Weather::Sunny),
            ffi::weather_id::WEATHER_SANDSTORM => Ok(Weather::Sandstorm),
            ffi::weather_id::WEATHER_CLOUDY => Ok(Weather::Cloudy),
            ffi::weather_id::WEATHER_RAIN => Ok(Weather::Rain),
            ffi::weather_id::WEATHER_HAIL => Ok(Weather::Hail),
            ffi::weather_id::WEATHER_FOG => Ok(Weather::Fog),
            ffi::weather_id::WEATHER_SNOW => Ok(Weather::Snow),
            ffi::weather_id::WEATHER_RANDOM => Ok(Weather::Random),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Types of weather.
pub enum DungeonObjective {
    Story = ffi::dungeon_objective::OBJECTIVE_STORY,
    Rescue = ffi::dungeon_objective::OBJECTIVE_RESCUE,
    Normal = ffi::dungeon_objective::OBJECTIVE_NORMAL,
}

impl TryFrom<ffi::dungeon_objective::Type> for DungeonObjective {
    type Error = ();

    fn try_from(value: ffi::dungeon_objective::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::dungeon_objective::OBJECTIVE_STORY => Ok(DungeonObjective::Story),
            ffi::dungeon_objective::OBJECTIVE_RESCUE => Ok(DungeonObjective::Rescue),
            ffi::dungeon_objective::OBJECTIVE_NORMAL => Ok(DungeonObjective::Normal),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Direction on the dungeon grid
pub enum Direction {
    Down = ffi::direction_id::DIR_DOWN,
    DownRight = ffi::direction_id::DIR_DOWN_RIGHT,
    Right = ffi::direction_id::DIR_RIGHT,
    UpRight = ffi::direction_id::DIR_UP_RIGHT,
    Up = ffi::direction_id::DIR_UP,
    UpLeft = ffi::direction_id::DIR_UP_LEFT,
    Left = ffi::direction_id::DIR_LEFT,
    DownLeft = ffi::direction_id::DIR_DOWN_LEFT,
    Current = ffi::direction_id::DIR_CURRENT,
}

impl TryFrom<ffi::direction_id::Type> for Direction {
    type Error = ();

    fn try_from(value: ffi::direction_id::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::direction_id::DIR_DOWN => Ok(Direction::Down),
            ffi::direction_id::DIR_DOWN_RIGHT => Ok(Direction::DownRight),
            ffi::direction_id::DIR_RIGHT => Ok(Direction::Right),
            ffi::direction_id::DIR_UP_RIGHT => Ok(Direction::UpRight),
            ffi::direction_id::DIR_UP => Ok(Direction::Up),
            ffi::direction_id::DIR_UP_LEFT => Ok(Direction::UpLeft),
            ffi::direction_id::DIR_LEFT => Ok(Direction::Left),
            ffi::direction_id::DIR_DOWN_LEFT => Ok(Direction::DownLeft),
            ffi::direction_id::DIR_CURRENT => Ok(Direction::Current),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Different types of warp effects
pub enum WarpType {
    /// Warp to a random position
    Random = ffi::warp_type::WARP_RANDOM,
    /// Warp within 2 tiles of the stairs
    Stairs2 = ffi::warp_type::WARP_STAIRS_2,
    /// Warp within 2 tiles of a specified position
    PositionFuzzy = ffi::warp_type::WARP_POSITION_FUZZY,
    /// Warp to an exact position
    PositionExact = ffi::warp_type::WARP_POSITION_EXACT,
    /// Warp within 3 tiles of the stairs
    Stairs3 = ffi::warp_type::WARP_STAIRS_3,
    /// Warp within 2 tiles of the leader
    Leader = ffi::warp_type::WARP_LEADER,
}

impl TryFrom<ffi::warp_type::Type> for WarpType {
    type Error = ();

    fn try_from(value: ffi::warp_type::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::warp_type::WARP_RANDOM => Ok(WarpType::Random),
            ffi::warp_type::WARP_STAIRS_2 => Ok(WarpType::Stairs2),
            ffi::warp_type::WARP_POSITION_FUZZY => Ok(WarpType::PositionFuzzy),
            ffi::warp_type::WARP_POSITION_EXACT => Ok(WarpType::PositionExact),
            ffi::warp_type::WARP_STAIRS_3 => Ok(WarpType::Stairs3),
            ffi::warp_type::WARP_LEADER => Ok(WarpType::Leader),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
/// The status of a monster's Conversion 2 state.
pub enum Conversion2Status {
    /// The monster is not under the effect of Conversion 2.
    None = 0,
    /// The monster is under the effect of Conversion 2 from a status.
    FromStatus = 1,
    /// The monster is under the effect of Conversion 2 from an exclusive item.
    FromExclusiveItem = 2,
}

impl TryFrom<i32> for Conversion2Status {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Conversion2Status::None),
            1 => Ok(Conversion2Status::FromStatus),
            2 => Ok(Conversion2Status::FromExclusiveItem),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Group of mission type on a dungeon floor.
pub enum MissionTypeGroup {
    RescueClient = ffi::mission_type::MISSION_RESCUE_CLIENT,
    RescueTarget = ffi::mission_type::MISSION_RESCUE_TARGET,
    EscortToTarget = ffi::mission_type::MISSION_ESCORT_TO_TARGET,
    ExploreWithClient = ffi::mission_type::MISSION_EXPLORE_WITH_CLIENT,
    ProspectWithClient = ffi::mission_type::MISSION_PROSPECT_WITH_CLIENT,
    GuideClient = ffi::mission_type::MISSION_GUIDE_CLIENT,
    FindItem = ffi::mission_type::MISSION_FIND_ITEM,
    DeliverItem = ffi::mission_type::MISSION_DELIVER_ITEM,
    SearchForTarget = ffi::mission_type::MISSION_SEARCH_FOR_TARGET,
    TakeItemFromOutlaw = ffi::mission_type::MISSION_TAKE_ITEM_FROM_OUTLAW,
    ArrestOutlaw = ffi::mission_type::MISSION_ARREST_OUTLAW,
    ChallengeRequest = ffi::mission_type::MISSION_CHALLENGE_REQUEST,
    TreasureMemo = ffi::mission_type::MISSION_TREASURE_MEMO,
}

impl TryFrom<ffi::mission_type::Type> for MissionTypeGroup {
    type Error = ();

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            ffi::mission_type::MISSION_RESCUE_CLIENT => Ok(MissionTypeGroup::RescueClient),
            ffi::mission_type::MISSION_RESCUE_TARGET => Ok(MissionTypeGroup::RescueTarget),
            ffi::mission_type::MISSION_ESCORT_TO_TARGET => Ok(MissionTypeGroup::EscortToTarget),
            ffi::mission_type::MISSION_EXPLORE_WITH_CLIENT => {
                Ok(MissionTypeGroup::ExploreWithClient)
            }
            ffi::mission_type::MISSION_PROSPECT_WITH_CLIENT => {
                Ok(MissionTypeGroup::ProspectWithClient)
            }
            ffi::mission_type::MISSION_GUIDE_CLIENT => Ok(MissionTypeGroup::GuideClient),
            ffi::mission_type::MISSION_FIND_ITEM => Ok(MissionTypeGroup::FindItem),
            ffi::mission_type::MISSION_DELIVER_ITEM => Ok(MissionTypeGroup::DeliverItem),
            ffi::mission_type::MISSION_SEARCH_FOR_TARGET => Ok(MissionTypeGroup::SearchForTarget),
            ffi::mission_type::MISSION_TAKE_ITEM_FROM_OUTLAW => {
                Ok(MissionTypeGroup::TakeItemFromOutlaw)
            }
            ffi::mission_type::MISSION_ARREST_OUTLAW => Ok(MissionTypeGroup::ArrestOutlaw),
            ffi::mission_type::MISSION_CHALLENGE_REQUEST => Ok(MissionTypeGroup::ChallengeRequest),
            ffi::mission_type::MISSION_TREASURE_MEMO => Ok(MissionTypeGroup::TreasureMemo),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
/// Specific mission type on a dungeon floor.
pub enum MissionType {
    RescueClient,
    RescueTarget,
    EscortToTarget,
    ExploreWithClient(MissionSubtypeExplore),
    ProspectWithClient,
    GuideClient,
    FindItem,
    DeliverItem,
    SearchForTarget,
    TakeItemFromOutlaw(MissionSubtypeTakeItem),
    ArrestOutlaw(MissionSubtypeOutlaw),
    ChallengeRequest(MissionSubtypeChallenge),
    TreasureMemo,
}

impl MissionType {
    /// Group for this mission type.
    pub fn group(&self) -> MissionTypeGroup {
        match self {
            MissionType::RescueClient => MissionTypeGroup::RescueClient,
            MissionType::RescueTarget => MissionTypeGroup::RescueTarget,
            MissionType::EscortToTarget => MissionTypeGroup::EscortToTarget,
            MissionType::ExploreWithClient(_) => MissionTypeGroup::ExploreWithClient,
            MissionType::ProspectWithClient => MissionTypeGroup::ProspectWithClient,
            MissionType::GuideClient => MissionTypeGroup::GuideClient,
            MissionType::FindItem => MissionTypeGroup::FindItem,
            MissionType::DeliverItem => MissionTypeGroup::DeliverItem,
            MissionType::SearchForTarget => MissionTypeGroup::SearchForTarget,
            MissionType::TakeItemFromOutlaw(_) => MissionTypeGroup::TakeItemFromOutlaw,
            MissionType::ArrestOutlaw(_) => MissionTypeGroup::ArrestOutlaw,
            MissionType::ChallengeRequest(_) => MissionTypeGroup::ChallengeRequest,
            MissionType::TreasureMemo => MissionTypeGroup::TreasureMemo,
        }
    }

    /// Subtype for this mission type as a c-union, if any.
    pub fn c_subtype(&self) -> ffi::mission_subtype {
        let mut mission_subtype: ffi::mission_subtype;
        unsafe {
            let mut s = ::core::mem::MaybeUninit::uninit();
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            mission_subtype = s.assume_init();
            match self {
                MissionType::ExploreWithClient(v) => {
                    *mission_subtype.explore.as_mut() = (*v) as ffi::mission_subtype_explore::Type;
                }
                MissionType::TakeItemFromOutlaw(v) => {
                    *mission_subtype.take_item.as_mut() =
                        (*v) as ffi::mission_subtype_take_item::Type;
                }
                MissionType::ArrestOutlaw(v) => {
                    *mission_subtype.outlaw.as_mut() = (*v) as ffi::mission_subtype_outlaw::Type;
                }
                MissionType::ChallengeRequest(v) => {
                    *mission_subtype.challenge.as_mut() =
                        (*v) as ffi::mission_subtype_challenge::Type;
                }
                _ => *mission_subtype.none.as_mut() = 0,
            }
        }
        mission_subtype
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Mission subtype for [`MissionType::ExploreWithClient`].
pub enum MissionSubtypeExplore {
    Normal = ffi::mission_subtype_explore::MISSION_EXPLORE_NORMAL,
    SealedChamber = ffi::mission_subtype_explore::MISSION_EXPLORE_SEALED_CHAMBER,
    GoldenChamber = ffi::mission_subtype_explore::MISSION_EXPLORE_GOLDEN_CHAMBER,
}

impl TryFrom<ffi::mission_subtype_explore::Type> for MissionSubtypeExplore {
    type Error = ();

    fn try_from(value: ffi::mission_subtype_explore::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::mission_subtype_explore::MISSION_EXPLORE_NORMAL => {
                Ok(MissionSubtypeExplore::Normal)
            }
            ffi::mission_subtype_explore::MISSION_EXPLORE_SEALED_CHAMBER => {
                Ok(MissionSubtypeExplore::SealedChamber)
            }
            ffi::mission_subtype_explore::MISSION_EXPLORE_GOLDEN_CHAMBER => {
                Ok(MissionSubtypeExplore::GoldenChamber)
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<ffi::mission_subtype> for MissionSubtypeExplore {
    type Error = ();

    fn try_from(value: ffi::mission_subtype) -> Result<Self, Self::Error> {
        // SAFETY: We just copy the value and we check if it's a valid enum value.
        Self::try_from(*unsafe { value.explore.as_ref() })
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Mission subtype for [`MissionType::TakeItemFromOutlaw`].
pub enum MissionSubtypeTakeItem {
    NormalOutlaw = ffi::mission_subtype_take_item::MISSION_TAKE_ITEM_NORMAL_OUTLAW,
    HiddenOutlaw = ffi::mission_subtype_take_item::MISSION_TAKE_ITEM_HIDDEN_OUTLAW,
    FleeingOutlaw = ffi::mission_subtype_take_item::MISSION_TAKE_ITEM_FLEEING_OUTLAW,
}

impl TryFrom<ffi::mission_subtype_take_item::Type> for MissionSubtypeTakeItem {
    type Error = ();

    fn try_from(value: ffi::mission_subtype_take_item::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::mission_subtype_take_item::MISSION_TAKE_ITEM_NORMAL_OUTLAW => {
                Ok(MissionSubtypeTakeItem::NormalOutlaw)
            }
            ffi::mission_subtype_take_item::MISSION_TAKE_ITEM_HIDDEN_OUTLAW => {
                Ok(MissionSubtypeTakeItem::HiddenOutlaw)
            }
            ffi::mission_subtype_take_item::MISSION_TAKE_ITEM_FLEEING_OUTLAW => {
                Ok(MissionSubtypeTakeItem::FleeingOutlaw)
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<ffi::mission_subtype> for MissionSubtypeTakeItem {
    type Error = ();

    fn try_from(value: ffi::mission_subtype) -> Result<Self, Self::Error> {
        // SAFETY: We just copy the value and we check if it's a valid enum value.
        Self::try_from(*unsafe { value.take_item.as_ref() })
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Mission subtype for [`MissionType::ArrestOutlaw`].
pub enum MissionSubtypeOutlaw {
    Normal0 = ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_0,
    Normal1 = ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_1,
    Normal2 = ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_2,
    Normal3 = ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_3,
    Escort = ffi::mission_subtype_outlaw::MISSION_OUTLAW_ESCORT,
    Fleeing = ffi::mission_subtype_outlaw::MISSION_OUTLAW_FLEEING,
    Hideout = ffi::mission_subtype_outlaw::MISSION_OUTLAW_HIDEOUT,
    MonsterHouse = ffi::mission_subtype_outlaw::MISSION_OUTLAW_MONSTER_HOUSE,
}

impl TryFrom<ffi::mission_subtype_take_item::Type> for MissionSubtypeOutlaw {
    type Error = ();

    fn try_from(value: ffi::mission_subtype_outlaw::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_0 => {
                Ok(MissionSubtypeOutlaw::Normal0)
            }
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_1 => {
                Ok(MissionSubtypeOutlaw::Normal1)
            }
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_2 => {
                Ok(MissionSubtypeOutlaw::Normal2)
            }
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_NORMAL_3 => {
                Ok(MissionSubtypeOutlaw::Normal3)
            }
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_ESCORT => Ok(MissionSubtypeOutlaw::Escort),
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_FLEEING => {
                Ok(MissionSubtypeOutlaw::Fleeing)
            }
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_HIDEOUT => {
                Ok(MissionSubtypeOutlaw::Hideout)
            }
            ffi::mission_subtype_outlaw::MISSION_OUTLAW_MONSTER_HOUSE => {
                Ok(MissionSubtypeOutlaw::MonsterHouse)
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<ffi::mission_subtype> for MissionSubtypeOutlaw {
    type Error = ();

    fn try_from(value: ffi::mission_subtype) -> Result<Self, Self::Error> {
        // SAFETY: We just copy the value and we check if it's a valid enum value.
        Self::try_from(*unsafe { value.outlaw.as_ref() })
    }
}

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
/// Mission subtype for [`MissionType::ChallengeRequest`].
pub enum MissionSubtypeChallenge {
    Normal = ffi::mission_subtype_challenge::MISSION_CHALLENGE_NORMAL,
    Mewtwo = ffi::mission_subtype_challenge::MISSION_CHALLENGE_MEWTWO,
    Entei = ffi::mission_subtype_challenge::MISSION_CHALLENGE_ENTEI,
    Raikou = ffi::mission_subtype_challenge::MISSION_CHALLENGE_RAIKOU,
    Suicune = ffi::mission_subtype_challenge::MISSION_CHALLENGE_SUICUNE,
    Jirachi = ffi::mission_subtype_challenge::MISSION_CHALLENGE_JIRACHI,
}

impl TryFrom<ffi::mission_subtype_take_item::Type> for MissionSubtypeChallenge {
    type Error = ();

    fn try_from(value: ffi::mission_subtype_challenge::Type) -> Result<Self, Self::Error> {
        match value {
            ffi::mission_subtype_challenge::MISSION_CHALLENGE_NORMAL => {
                Ok(MissionSubtypeChallenge::Normal)
            }
            ffi::mission_subtype_challenge::MISSION_CHALLENGE_MEWTWO => {
                Ok(MissionSubtypeChallenge::Mewtwo)
            }
            ffi::mission_subtype_challenge::MISSION_CHALLENGE_ENTEI => {
                Ok(MissionSubtypeChallenge::Entei)
            }
            ffi::mission_subtype_challenge::MISSION_CHALLENGE_RAIKOU => {
                Ok(MissionSubtypeChallenge::Raikou)
            }
            ffi::mission_subtype_challenge::MISSION_CHALLENGE_SUICUNE => {
                Ok(MissionSubtypeChallenge::Suicune)
            }
            ffi::mission_subtype_challenge::MISSION_CHALLENGE_JIRACHI => {
                Ok(MissionSubtypeChallenge::Jirachi)
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<ffi::mission_subtype> for MissionSubtypeChallenge {
    type Error = ();

    fn try_from(value: ffi::mission_subtype) -> Result<Self, Self::Error> {
        // SAFETY: We just copy the value and we check if it's a valid enum value.
        Self::try_from(*unsafe { value.challenge.as_ref() })
    }
}
