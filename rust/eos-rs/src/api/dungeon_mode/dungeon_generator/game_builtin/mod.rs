//! The game's built-in dungeon generator.
//!
//! **IMPORTANT to know** for this implementation: A lot of functions will not work as expected,
//! since some low level functions just ignore some parameters passed in completely.
//!
//! The generation implementation that is built into the game works directly
//! with the global tile data based on temporary grid cells.
//! This module contains functions that either are high level wrappers or little pieces
//! of the algorithm that don't need dungeon grid cells.
//!
//! To get an instance of the generator, use
//! [`crate::api::dungeon_mode::GlobalDungeonData::get_builtin_dungeon_generator`].

mod grid;

pub use self::grid::{DungeonGridMutator, GRID_CAPACITY_DIM};
use crate::api::dungeon_mode::GlobalDungeonData;

use crate::api::dungeon_mode::dungeon_generator::{
    DungeonEntityGeneration, DungeonFloorGeneration,
};
use crate::api::objects::fixed_room_catalog;
use crate::api::overlay::OverlayLoadLease;
use crate::ctypes::c_int;
use crate::ffi;

//-----------------------------------------------------------------------------------------------//

/// The structure and layout generator for the global dungeon.
///
/// To create instances of this struct use
/// [`crate::api::dungeon_mode::GlobalDungeonData::get_builtin_dungeon_generator`].
pub struct GlobalDungeonStructureGenerator<'a>(
    pub(crate) OverlayLoadLease<29>,
    pub(crate) &'a mut GlobalDungeonData<'a>,
);

impl<'a> GlobalDungeonStructureGenerator<'a> {
    #[doc(hidden)]
    pub(crate) fn generate_floor_internal(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::GenerateFloor() }
    }

    /// Handles fixed room generation if the floor contains a fixed room.
    pub fn generate_fixed_room(
        &mut self,
        fixed_room_id: fixed_room_catalog::Type,
        properties: &ffi::floor_properties,
    ) -> bool {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::GenerateFixedRoom(fixed_room_id, force_mut_ptr!(properties)) > 0 }
    }

    /// Sets the junction flag (bit 3 of the terrain flags) on any hallway junction tiles in
    /// some range [x0, x1), [y0, y1). This leaves tiles within rooms untouched.
    pub fn flag_hallway_junctions(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::FlagHallwayJunctions(x0, y0, x1, y1) }
    }

    /// Create a hallway between two points.
    ///
    /// If the two points share no coordinates in common (meaning the line connecting them is
    /// diagonal), a "kinked" hallway is created, with the kink at a specified "middle"
    /// coordinate (in practice the grid cell boundary). For example, with a kinked horizontal
    /// hallway, there are two horizontal lines extending out from the endpoints, connected by a
    /// vertical line on the middle x coordinate.
    ///
    /// If a hallway would intersect with an existing open tile (like an existing hallway), the
    /// hallway will only be created up to the point where it intersects with the open tile.
    ///
    /// # Arguments
    ///
    /// * `start_x` - The x coordinate of the start of the hallway.
    /// * `start_y` - The y coordinate of the start of the hallway.
    /// * `end_x` - The x coordinate of the end of the hallway.
    /// * `end_y` - The y coordinate of the end of the hallway.
    /// * `is_vertical` - vertical flag (true for vertical hallway, false for horizontal).
    /// * `middle_x` - Middle x coordinate for kinked horizontal hallways.
    /// * `middle_y` - Middle y coordinate for kinked vertical hallways.
    pub fn create_hallway(
        &mut self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        is_vertical: bool,
        middle_x: i32,
        middle_y: i32,
    ) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe {
            ffi::CreateHallway(
                start_x,
                start_y,
                end_x,
                end_y,
                is_vertical as ffi::bool_,
                middle_x,
                middle_y,
            )
        }
    }

    /// Finalizes junction tiles by setting the junction flag (bit 3 of the terrain flags) and
    /// ensuring open terrain.
    ///
    /// Note that this implementation is slightly buggy. This function scans tiles left-to-right,
    /// top-to-bottom, and identifies junctions as any open, non-hallway tile (room_index != 0xFF)
    /// adjacent to an open, hallway tile (room_index == 0xFF). This interacts poorly with hallway
    /// anchors (room_index == 0xFE). This function sets the room index of any hallway anchors to
    /// 0xFF within the same loop, so a hallway anchor may or may not be identified as a junction
    /// depending on the orientation of connected hallways.
    ///
    /// For example, in the following configuration, the "o" tile would be marked as a junction
    /// because the neighboring hallway tile to its left comes earlier in iteration, while the "o"
    /// tile still has the room index 0xFE, causing the algorithm to mistake it for a room tile:
    /// ```text
    /// xxxxx
    /// ---ox
    /// xxx|x
    /// xxx|x
    /// ```
    ///
    /// However, in the following configuration, the "o" tile would NOT be marked as a junction
    /// because it comes earlier in iteration than any of its neighboring hallway tiles, so its
    /// room index is set to 0xFF before it can be marked as a junction. This is actually the ONLY
    /// possible configuration where a hallway anchor will not be marked as a junction.
    /// ```text
    /// xxxxx
    /// xo---
    /// x|xxx
    /// x|xxx
    /// ```
    pub fn finalize_junctions(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::FinalizeJunctions() }
    }

    /// Generate a "maze line" from a given starting point, within the given bounds.
    ///
    /// A "maze line" is a random walk starting from (x0, y0). The random walk proceeds with a
    /// stride of 2 in a random direction, laying down obstacles as it goes. The random walk
    /// terminates when it gets trapped and there are no more neighboring tiles that are open and
    /// in-bounds.
    pub fn generate_maze_line(
        &mut self,
        x0: i32,
        y0: i32,
        x_min: i32,
        y_min: i32,
        x_max: i32,
        y_max: i32,
        use_secondary_terrain: bool,
        room: u8,
    ) {
        unsafe {
            ffi::GenerateMazeLine(
                x0,
                y0,
                x_min,
                y_min,
                x_max,
                y_max,
                use_secondary_terrain as ffi::bool_,
                room,
            )
        }
    }

    /// Checks if a tile position is either in a hallway or next to one.
    pub fn is_next_to_hallway(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::IsNextToHallway(x, y) > 0 }
    }

    /// Resolve invalid spawn flags on tiles.
    ///
    /// Spawn flags can be invalid due to terrain. For example, traps can't spawn on obstacles.
    /// Spawn flags can also be invalid due to multiple being set on a single tile, in which case
    /// one will take precedence. For example, stair spawns trump trap spawns.
    pub fn resolve_invalid_spawns(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::ResolveInvalidSpawns() }
    }

    /// Converts all secondary terrain tiles (water/lava) to chasms.
    pub fn convert_secondary_terrain_to_chasms(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::ConvertSecondaryTerrainToChasms() }
    }

    /// Converts all wall tiles to chasms.
    pub fn convert_walls_to_chasms(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::ConvertWallsToChasms() }
    }

    /// Ensures all tiles with the impassable flag are walls.
    pub fn ensure_impassable_tiles_are_walls(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::EnsureImpassableTilesAreWalls() }
    }

    /// Resets the floor in preparation for a floor generation attempt.
    ///
    /// Resets all tiles, resets the border to be impassable, and clears entity spawns.
    pub fn reset_floor(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::ResetFloor() }
    }

    /// Generate secondary terrain (water/lava) formations.
    ///
    /// This includes "rivers" that flow from top-to-bottom (or bottom-to-top), as well as "lakes"
    /// both standalone and after rivers. Water/lava formations will never cut through rooms, but
    /// they can pass through rooms to the opposite side.
    ///
    /// Rivers are generated by a top-down or bottom-up random walk that ends when existing
    /// secondary terrain is reached or the walk goes out of bounds. Some rivers also end
    /// prematurely in a lake. Lakes are a large collection of secondary terrain generated around
    /// a central point.
    ///
    /// # Arguments
    /// * `test_flag` - bit index to test in the floor properties room flag bitvector
    ///                 (formations are only generated if the bit is set)
    /// * `floor_props` - floor properties
    pub fn generate_secondary_terrain_formations(
        &mut self,
        test_flag: u8,
        floor_props: &ffi::floor_properties,
    ) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::GenerateSecondaryTerrainFormations(test_flag, force_mut_ptr!(floor_props)) }
    }

    /// Checks that the stairs are reachable from every walkable tile on the floor.
    ///
    /// This runs a graph traversal algorithm that is very similar to breadth-first search
    /// (the order in which nodes are visited is slightly different), starting from the stairs.
    /// If any tile is walkable but wasn't reached by the traversal algorithm, then the stairs
    /// must not be reachable from that tile.
    ///
    /// If `mark_unreachable` is true, this function will instead always return true, but set a
    /// special bit on all walkable tiles that aren't reachable from the stairs.
    pub fn stairs_are_always_reachable(
        &mut self,
        x_stairs: i32,
        y_stairs: i32,
        mark_unreachable: bool,
    ) -> bool {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe {
            ffi::StairsAlwaysReachable(x_stairs, y_stairs, mark_unreachable as ffi::bool_) > 0
        }
    }

    /// Reset the inner boundary tile rows (y == 1 and y == 30) to their initial state of all wall
    /// tiles, with impassable walls at the edges (x == 0 and x == 55).
    pub fn reset_inner_boundary_tile_rows(&mut self) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe { ffi::ResetInnerBoundaryTileRows() }
    }

    /// Spawn stairs at the given location.
    ///
    /// If the hidden stairs flag is set, hidden stairs will be spawned instead of normal stairs.
    ///
    /// If spawning normal stairs and the current floor is a rescue floor, the room containing
    /// the stairs will be converted into a Monster House.
    pub fn spawn_stairs(
        &mut self,
        x: u8,
        y: u8,
        gen_info: &ffi::dungeon_generation_info,
        is_hidden_stairs: bool,
    ) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe {
            ffi::SpawnStairs(
                [x, y].as_mut_ptr(),
                force_mut_ptr!(gen_info),
                is_hidden_stairs as ffi::bool_,
            )
        }
    }
}

//-----------------------------------------------------------------------------------------------//

/// The entity generator for the global dungeon.
pub struct GlobalDungeonEntityGenerator(());

impl GlobalDungeonEntityGenerator {
    /// Creates a new generator.
    ///
    /// # Safety:
    /// The caller needs to make sure that overlay 29 is loaded and it's safe to
    /// manipulate the global dungeon.
    unsafe fn new() -> Self {
        Self(())
    }

    /// Randomly shuffle an array of spawn positions.
    pub fn shuffle_spawn_positions(&self, spawn_positions: &mut [ffi::spawn_position]) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe {
            ffi::ShuffleSpawnPositions(spawn_positions.as_mut_ptr(), spawn_positions.len() as c_int)
        }
    }
}

//-----------------------------------------------------------------------------------------------//

/// The game's builtin dungeon generator.
impl<'a> DungeonFloorGeneration for GlobalDungeonStructureGenerator<'a> {
    type EntityGenerator = GlobalDungeonEntityGenerator;
    type LayoutGenerator = BuiltinDungeonLayoutGenerators;

    /// Generate a dungeon floor.
    ///
    /// This ignores all parameters at the moment and instead reads from the global dungeon struct
    /// directly.
    ///
    /// This is the master function that generates the dungeon floor.
    ///
    /// Very loosely speaking, this function first tries to generate a valid floor layout.
    /// Then it tries to spawn entities in a valid configuration. Finally, it performs cleanup
    /// and post-processing depending on the dungeon.
    ///
    /// If a spawn configuration is invalid, the entire floor layout is scrapped and regenerated.
    /// If the generated floor layout is invalid 10 times in a row, or a valid spawn configuration
    /// isn't generated within 10 attempts, the generation algorithm aborts and the default
    /// one-room Monster House floor is generated as a fallback.
    ///
    /// If not changed by a patch, this function will use the game's default built in generator
    /// and generate the floor based on the current global configuration for the floor.
    // TODO: Set up global struct here to take param into account?
    fn generate_floor(
        &mut self,
        _width: usize,
        _height: usize,
        _properties: &ffi::floor_properties,
    ) -> &mut Self {
        self.generate_floor_internal();
        self
    }

    /// Width and height are ignored for most layouts.
    fn generate_layout(
        &mut self,
        layout: &mut Self::LayoutGenerator,
        properties: &ffi::floor_properties,
    ) -> &mut Self {
        unsafe {
            match layout {
                BuiltinDungeonLayoutGenerators::Standard { width, height } => {
                    ffi::GenerateStandardFloor(*width, *height, force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::OuterRing => {
                    ffi::GenerateOuterRingFloor(force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::Crossroads => {
                    ffi::GenerateCrossroadsFloor(force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::Line => {
                    ffi::GenerateLineFloor(force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::Cross => {
                    ffi::GenerateCrossFloor(force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::Beetle => {
                    ffi::GenerateBeetleFloor(force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::OuterRooms { width, height } => {
                    ffi::GenerateOuterRoomsFloor(*width, *height, force_mut_ptr!(properties))
                }
                BuiltinDungeonLayoutGenerators::TwoRoomsMonsterHouse => {
                    ffi::GenerateTwoRoomsWithMonsterHouseFloor()
                }
                BuiltinDungeonLayoutGenerators::OneRoomMonsterHouse => {
                    ffi::GenerateOneRoomMonsterHouseFloor()
                }
            }
        }
        self
    }

    fn entities<F: FnOnce(&mut Self::EntityGenerator)>(&mut self, cb: F) -> &mut Self {
        // SAFETY: We have a lease on the overlay and a mutable borrow on the global dungeon.
        let mut ent = unsafe { GlobalDungeonEntityGenerator::new() };
        cb(&mut ent);
        self
    }

    /// This does nothing, this implementation will always update the global struct directly.
    fn generate(self, _: &mut GlobalDungeonData) {}
}

/// Builtin generator for entities on a dungeon floor.
impl DungeonEntityGeneration for GlobalDungeonEntityGenerator {
    /// Spawn all non-enemy entities, which includes stairs, items, traps, and the player.
    ///
    /// Most entities are spawned randomly on a subset of permissible tiles.
    ///
    /// Stairs are spawned if they don't already exist on the floor, and hidden stairs of the
    /// specified type are also spawned if configured as long as there are at least 2 floors left
    /// in the dungeon. Stairs can spawn on any tile that has open terrain, is in a room, isn't in
    /// a Kecleon shop, doesn't already have an enemy spawn, isn't a hallway junction, and isn't a
    /// special tile like a Key door.
    ///
    /// Items are spawned both normally in rooms, as well as in walls and Monster Houses. Normal
    /// items can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop
    /// or Monster House, isn't a hallway junction, and isn't a special tile like a Key door.
    /// Buried items can spawn on any wall tile. Monster House items can spawn on any Monster House
    /// tile that isn't in a Kecleon shop and isn't a hallway junction.
    ///
    /// Traps are similarly spawned both normally in rooms, as well as in Monster Houses. Normal
    /// traps can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop,
    /// doesn't already have an item or enemy spawn, and isn't a special tile like a Key door.
    /// Monster House traps follow the same conditions as Monster House items.
    ///
    /// The player can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon
    /// shop, isn't a hallway junction, doesn't already have an item, enemy, or trap spawn, and
    /// isn't a special tile like a Key door.
    fn spawn_non_enemies(
        &mut self,
        floor_properties: &ffi::floor_properties,
        empty_monster_house: bool,
    ) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe {
            ffi::SpawnNonEnemies(
                force_mut_ptr!(floor_properties),
                empty_monster_house as ffi::bool_,
            )
        }
    }

    /// Spawn all enemies, which includes normal enemies and those in Monster Houses.
    ///
    /// Normal enemies can spawn on any tile that has open terrain, isn't in a Kecleon shop,
    /// doesn't already have another entity spawn, and isn't a special tile like a Key door.
    ///
    /// Monster House enemies can spawn on any Monster House tile that isn't in a Kecleon shop,
    /// isn't where the player spawns, and isn't a special tile like a Key door.
    fn spawn_enemies(
        &mut self,
        floor_properties: &ffi::floor_properties,
        empty_monster_house: bool,
    ) {
        // SAFETY: We have a mutable reference to the dungeon.
        unsafe {
            ffi::SpawnEnemies(
                force_mut_ptr!(floor_properties),
                empty_monster_house as ffi::bool_,
            )
        }
    }
}

//-----------------------------------------------------------------------------------------------//

/// Builtin layout generators.
#[derive(Copy, Clone, Default)]
pub enum BuiltinDungeonLayoutGenerators {
    /// Broadly speaking, a standard floor is generated as follows:
    ///
    /// 1. Generating the grid
    /// 2. Creating a room or hallway anchor in each grid cell
    /// 3. Creating hallways between grid cells
    /// 4. Generating special features (maze room, Kecleon shop, Monster House, extra hallways,
    ///    room imperfections, secondary structures)
    Standard { width: i32, height: i32 },
    /// Floor layout with a 4x2 grid of rooms, surrounded by an outer ring of hallways.
    OuterRing,
    /// Floor layout with a mesh of hallways on the interior 3x2 grid, surrounded by a
    /// boundary of rooms protruding from the interior like spikes, excluding the corner cells.
    Crossroads,
    /// Floor layout with 5 grid cells in a horizontal line.
    Line,
    /// Floor layout with 5 rooms arranged in a cross ("plus sign") formation.
    Cross,
    /// Floor layout in a "beetle" formation, which is created by taking a 3x3 grid
    /// of rooms, connecting the rooms within each row, and merging the central column
    /// into one big room.
    Beetle,
    /// Floor layout with a ring of rooms on the grid boundary and nothing in the
    /// interior.
    ///
    /// Note that this is bugged, and won't properly connect all the rooms together for
    /// `width < 4`.
    OuterRooms { width: i32, height: i32 },
    /// Floor layout with two rooms (left and right), one of which is a Monster House.
    TwoRoomsMonsterHouse,
    /// Floor layout with just a large, one-room Monster House.
    ///
    /// This is the default layout if dungeon generation fails.
    #[default]
    OneRoomMonsterHouse,
}
