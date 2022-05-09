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

pub use self::grid::DungeonGridMutator;

use crate::api::dungeon_mode::dungeon_generator::{DungeonEntityGeneration, DungeonFloorGeneration, DungeonPiecesGeneration};
use crate::api::objects::fixed_room_catalog;
use crate::api::overlay::OverlayLoadLease;
use crate::ffi;
use crate::ffi::floor_properties;

//-----------------------------------------------------------------------------------------------//

/// The structure and layout generator for the global dungeon.
///
/// To create instances of this struct use
/// [`crate::api::dungeon_mode::GlobalDungeonData::get_builtin_dungeon_generator`].
///
/// All methods on implementations of the dungeon generation traits are unsafe,
/// construction of this struct is therefore guarded by unsafe functions (see above).
pub struct GlobalDungeonStructureGenerator(pub(crate) OverlayLoadLease<29>);

impl GlobalDungeonStructureGenerator {
    #[doc(hidden)]
    pub(crate) fn generate_floor_internal(&mut self) {
        unsafe { ffi::GenerateFloor() }
    }

    /// Handles fixed room generation if the floor contains a fixed room.
    pub unsafe fn generate_fixed_room(&mut self, fixed_room_id: fixed_room_catalog::Type, properties: &ffi::floor_properties) -> bool {
        ffi::GenerateFixedRoom(fixed_room_id, force_mut_ptr!(properties)) > 0
    }

    /// Sets the junction flag (bit 3 of the terrain flags) on any hallway junction tiles in
    /// some range [x0, x1), [y0, y1). This leaves tiles within rooms untouched.
    pub unsafe fn flag_hallway_junctions(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        ffi::FlagHallwayJunctions(x0, y0, x1, y1)
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
    pub unsafe fn create_hallway(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, is_vertical: bool, middle_x: i32, middle_y: i32) {
        ffi::CreateHallway(
            start_x, start_y, end_x, end_y, is_vertical as ffi::bool_,
            middle_x, middle_y
        )
    }

//     - name: SetTerrainObstacleChecked
//       address:
//         NA: 0x233F900
//         EU: 0x23404E4
//       description: |-
//         Set the terrain of a specific tile to be an obstacle (wall or secondary terrain).
//
//         Secondary terrain (water/lava) can only be placed in the specified room. If the tile room index does not match, a wall will be placed instead.
//
//         r0: tile pointer
//         r1: use secondary terrain flag (true for water/lava, false for wall)
//         r2: room index
//     - name: FinalizeJunctions
//       address:
//         NA: 0x233F93C
//         EU: 0x2340520
//       description: |-
//         Finalizes junction tiles by setting the junction flag (bit 3 of the terrain flags) and ensuring open terrain.
//
//         Note that this implementation is slightly buggy. This function scans tiles left-to-right, top-to-bottom, and identifies junctions as any open, non-hallway tile (room_index != 0xFF) adjacent to an open, hallway tile (room_index == 0xFF). This interacts poorly with hallway anchors (room_index == 0xFE). This function sets the room index of any hallway anchors to 0xFF within the same loop, so a hallway anchor may or may not be identified as a junction depending on the orientation of connected hallways.
//
//         For example, in the following configuration, the "o" tile would be marked as a junction because the neighboring hallway tile to its left comes earlier in iteration, while the "o" tile still has the room index 0xFE, causing the algorithm to mistake it for a room tile:
//           xxxxx
//           ---ox
//           xxx|x
//           xxx|x
//         However, in the following configuration, the "o" tile would NOT be marked as a junction because it comes earlier in iteration than any of its neighboring hallway tiles, so its room index is set to 0xFF before it can be marked as a junction. This is actually the ONLY possible configuration where a hallway anchor will not be marked as a junction.
//           xxxxx
//           xo---
//           x|xxx
//           x|xxx
//
//         No params.
//     - name: GenerateMazeLine
//       address:
//         NA: 0x23406D4
//         EU: 0x23412B8
//       description: |-
//         Generate a "maze line" from a given starting point, within the given bounds.
//
//         A "maze line" is a random walk starting from (x0, y0). The random walk proceeds with a stride of 2 in a random direction, laying down obstacles as it goes. The random walk terminates when it gets trapped and there are no more neighboring tiles that are open and in-bounds.
//
//         r0: x0
//         r1: y0
//         r2: xmin
//         r3: ymin
//         stack[0]: xmax
//         stack[1]: ymax
//         stack[2]: use secondary terrain flag (true for water/lava, false for walls)
//         stack[3]: room index
//     - name: IsNextToHallway
//       address:
//         NA: 0x23408D0
//         EU: 0x23414B4
//       description: |-
//         Checks if a tile position is either in a hallway or next to one.
//
//         r0: x
//         r1: y
//         return: bool
//     - name: ResolveInvalidSpawns
//       address:
//         NA: 0x2340974
//         EU: 0x2341558
//       description: |-
//         Resolve invalid spawn flags on tiles.
//
//         Spawn flags can be invalid due to terrain. For example, traps can't spawn on obstacles. Spawn flags can also be invalid due to multiple being set on a single tile, in which case one will take precedence. For example, stair spawns trump trap spawns.
//
//         No params.
//     - name: ConvertSecondaryTerrainToChasms
//       address:
//         NA: 0x2340A0C
//         EU: 0x23415F0
//       description: |-
//         Converts all secondary terrain tiles (water/lava) to chasms.
//
//         No params.
//     - name: EnsureImpassableTilesAreWalls
//       address:
//         NA: 0x2340A78
//         EU: 0x234165C
//       description: |-
//         Ensures all tiles with the impassable flag are walls.
//
//         No params.
//     - name: InitializeTile
//       address:
//         NA: 0x2340AD4
//         EU: 0x23416B8
//       description: |-
//         Initialize a tile struct.
//
//         r0: tile pointer
//     - name: ResetFloor
//       address:
//         NA: 0x2340B0C
//         EU: 0x23416F0
//       description: |-
//         Resets the floor in preparation for a floor generation attempt.
//
//         Resets all tiles, resets the border to be impassable, and clears entity spawns.
//
//         No params.

//     - name: SetSecondaryTerrainOnWall
//       address:
//         NA: 0x234176C
//         EU: 0x2342350
//       description: |-
//         Set a specific tile to have secondary terrain (water/lava), but only if it's a passable wall.
//
//         r0: tile pointer
//     - name: GenerateSecondaryTerrainFormations
//       address:
//         NA: 0x23417AC
//         EU: 0x2342390
//       description: |-
//         Generate secondary terrain (water/lava) formations.
//
//         This includes "rivers" that flow from top-to-bottom (or bottom-to-top), as well as "lakes" both standalone and after rivers. Water/lava formations will never cut through rooms, but they can pass through rooms to the opposite side.
//
//         Rivers are generated by a top-down or bottom-up random walk that ends when existing secondary terrain is reached or the walk goes out of bounds. Some rivers also end prematurely in a lake. Lakes are a large collection of secondary terrain generated around a central point.
//
//         r0: bit index to test in the floor properties room flag bitvector (formations are only generated if the bit is set)
//         r1: floor properties
//     - name: StairsAlwaysReachable
//       address:
//         NA: 0x2341E6C
//         EU: 0x2342A50
//       description: |-
//         Checks that the stairs are reachable from every walkable tile on the floor.
//
//         This runs a graph traversal algorithm that is very similar to breadth-first search (the order in which nodes are visited is slightly different), starting from the stairs. If any tile is walkable but wasn't reached by the traversal algorithm, then the stairs must not be reachable from that tile.
//
//         r0: x coordinate of the stairs
//         r1: y coordinate of the stairs
//         r2: flag to always return true, but set a special bit on all walkable tiles that aren't reachable from the stairs
//         return: bool
//     - name: ConvertWallsToChasms
//       address:
//         NA: 0x2342548
//         EU: 0x234312C
//       description: |-
//         Converts all wall tiles to chasms.
//
//         No params.
//     - name: ResetInnerBoundaryTileRows
//       address:
//         NA: 0x2342B7C
//         EU: 0x2343760
//       description: |-
//         Reset the inner boundary tile rows (y == 1 and y == 30) to their initial state of all wall tiles, with impassable walls at the edges (x == 0 and x == 55).
//
//         No params.
//     - name: SpawnStairs
//       address:
//         NA: 0x2342C8C
//         EU: 0x2343870
//       description: |-
//         Spawn stairs at the given location.
//
//         If the hidden stairs flag is set, hidden stairs will be spawned instead of normal stairs.
//
//         If spawning normal stairs and the current floor is a rescue floor, the room containing the stairs will be converted into a Monster House.
//
//         r0: position (two-byte array for {x, y})
//         r1: dungeon generation info pointer (a field on the dungeon struct)
//         r2: hidden stairs flag
}

//-----------------------------------------------------------------------------------------------//

/// The entity generator for the global dungeon.
pub struct GlobalDungeonEntityGenerator(OverlayLoadLease<29>);

impl GlobalDungeonEntityGenerator {
    //     - name: ShuffleSpawnPositions
    //       address:
    //         NA: 0x2340CE4
    //         EU: 0x23418C8
    //       description: |-
    //         Randomly shuffle an array of spawn positions.
    //
    //         r0: spawn position array containing bytes {x1, y1, x2, y2, ...}
    //         r1: number of (x, y) pairs in the spawn position array
    //     - name: SpawnNonEnemies
    //       address:
    //         NA: 0x2340D4C
    //         EU: 0x2341930
    //       description: |-
    //         Spawn all non-enemy entities, which includes stairs, items, traps, and the player.
    //
    //         Most entities are spawned randomly on a subset of permissible tiles.
    //
    //         Stairs are spawned if they don't already exist on the floor, and hidden stairs of the specified type are also spawned if configured as long as there are at least 2 floors left in the dungeon. Stairs can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop, doesn't already have an enemy spawn, isn't a hallway junction, and isn't a special tile like a Key door.
    //
    //         Items are spawned both normally in rooms, as well as in walls and Monster Houses. Normal items can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop or Monster House, isn't a hallway junction, and isn't a special tile like a Key door. Buried items can spawn on any wall tile. Monster House items can spawn on any Monster House tile that isn't in a Kecleon shop and isn't a hallway junction.
    //
    //         Traps are similarly spawned both normally in rooms, as well as in Monster Houses. Normal traps can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop, doesn't already have an item or enemy spawn, and isn't a special tile like a Key door. Monster House traps follow the same conditions as Monster House items.
    //
    //         The player can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop, isn't a hallway junction, doesn't already have an item, enemy, or trap spawn, and isn't a special tile like a Key door.
    //
    //         r0: floor properties
    //         r1: empty Monster House flag. An empty Monster House is one with no items or traps, and only a small number of enemies.
    //     - name: SpawnEnemies
    //       address:
    //         NA: 0x2341470
    //         EU: 0x2342054
    //       description: |-
    //         Spawn all enemies, which includes normal enemies and those in Monster Houses.
    //
    //         Normal enemies can spawn on any tile that has open terrain, isn't in a Kecleon shop, doesn't already have another entity spawn, and isn't a special tile like a Key door.
    //
    //         Monster House enemies can spawn on any Monster House tile that isn't in a Kecleon shop, isn't where the player spawns, and isn't a special tile like a Key door.
    //
    //         r0: floor properties
    //         r1: empty Monster House flag. An empty Monster House is one with no items or traps, and only a small number of enemies.
}

//-----------------------------------------------------------------------------------------------//

/// The game's builtin dungeon generator.
///
/// **All methods on this implementation are actually unsafe. They directly update the global
/// dungeon state.** Because of this this struct can only be retrieved from methods marked unsafe,
/// see the note at [`GlobalDungeonStructureGenerator`].
impl DungeonFloorGeneration for GlobalDungeonStructureGenerator {
    type EntityGenerator = GlobalDungeonEntityGenerator;
    type PiecesGenerator = GlobalDungeonStructureGenerator;
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
    // TODO: Is there a function one level higher in the game that does actually take
    //       these three parameters? Can we maybe just set up the global struct?
    fn generate_floor(&mut self, _width: usize, _height: usize, _properties: &floor_properties) -> &mut Self {
        self.generate_floor_internal();
        self
    }

    /// Generate a standard floor with the given parameters.
    ///
    /// Broadly speaking, a standard floor is generated as follows:
    ///
    /// 1. Generating the grid
    /// 2. Creating a room or hallway anchor in each grid cell
    /// 3. Creating hallways between grid cells
    /// 4. Generating special features (maze room, Kecleon shop, Monster House, extra hallways,
    ///    room imperfections, secondary structures)
    fn generate_standard_floor(&mut self, width: usize, height: usize, properties: &floor_properties) -> &mut Self {
        assert!(width <= (i32::MAX as usize) && height <= (i32::MAX as usize));
        unsafe {
            ffi::GenerateStandardFloor(
                width as i32, height as i32, force_mut_ptr!(properties)
            )
        }
        self
    }

    /// ***Not implemented.***
    ///
    /// No function that generates a whole fixed floor is known at the time.
    fn generate_fixed_floor(&mut self, _fixed_room_id: fixed_room_catalog::Type, _properties: &floor_properties) -> &mut Self {
        unimplemented!("No function that generates a whole fixed floor is known at the time.")
    }

    ///
    fn generate_layout(&mut self, width: usize, height: usize, layout: &mut Self::LayoutGenerator, properties: &ffi::floor_properties) -> &mut Self {
        unsafe { match layout {
            BuiltinDungeonLayoutGenerators::OuterRing => ffi::GenerateOuterRingFloor(force_mut_ptr!(properties)),
            BuiltinDungeonLayoutGenerators::Crossroads => ffi::GenerateCrossroadsFloor(force_mut_ptr!(properties)),
            BuiltinDungeonLayoutGenerators::Line => ffi::GenerateLineFloor(force_mut_ptr!(properties)),
            BuiltinDungeonLayoutGenerators::Cross => ffi::GenerateCrossFloor(force_mut_ptr!(properties)),
            BuiltinDungeonLayoutGenerators::Beetle => ffi::GenerateBeetleFloor(force_mut_ptr!(properties)),
            BuiltinDungeonLayoutGenerators::OuterRooms { grid_size_x, grid_size_y } =>
                ffi::GenerateOuterRoomsFloor(*grid_size_x, *grid_size_y, force_mut_ptr!(properties)),
            BuiltinDungeonLayoutGenerators::TwoRoomsMonsterHouse => ffi::GenerateTwoRoomsWithMonsterHouseFloor(),
            BuiltinDungeonLayoutGenerators::OneRoomMonsterHouse => ffi::GenerateOneRoomMonsterHouseFloor()
        } }
        self
    }

    fn pieces<F: FnOnce(&mut Self::PiecesGenerator)>(&mut self, cb: F) -> &mut Self {
        cb(self);
        self
    }

    fn entities<F: FnOnce(&mut Self::EntityGenerator)>(&mut self, cb: F) -> &mut Self {
        cb(&mut GlobalDungeonEntityGenerator(self.0.clone()));
        self
    }

    /// This does nothing, this implementation will always update the global struct directly.
    unsafe fn generate(self) {}
}

/// Bits and pieces implemented by the game's builtin dungeon generator.
impl DungeonPiecesGeneration for GlobalDungeonStructureGenerator {

}

/// Builtin generator for entities on a dungeon floor.
impl DungeonEntityGeneration for GlobalDungeonEntityGenerator {

}

//-----------------------------------------------------------------------------------------------//

/// Builtin layout generators.
#[derive(Copy, Clone, Default)]
pub enum BuiltinDungeonLayoutGenerators {
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
    /// `grid_size_x < 4`.
    OuterRooms { grid_size_x: i32, grid_size_y: i32 },
    /// Floor layout with two rooms (left and right), one of which is a Monster House.
    TwoRoomsMonsterHouse,
    /// Floor layout with just a large, one-room Monster House.
    ///
    /// This is the default layout if dungeon generation fails.
    #[default]
    OneRoomMonsterHouse,
}
