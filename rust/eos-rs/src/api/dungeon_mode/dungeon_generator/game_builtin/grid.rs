//! Temporary grid cells used as building-blocks for dungeon generation.
//! These building blocks are generated and then piece-by-piece used to fill
//! the global tile data.

use alloc::vec;
use alloc::vec::Vec;
use core::iter::repeat_with;
use crate::api::dungeon_mode::dungeon_generator::DungeonGridCell;
use crate::api::overlay::OverlayLoadLease;
use crate::ctypes::c_int;
use crate::ffi;
use crate::ffi::floor_properties;

/// The capacity of the dungeon grid in both X and Y directions.
pub const GRID_CAPACITY_DIM: usize = 15;

impl Default for ffi::dungeon_grid_cell {
    fn default() -> Self {
        Self {
            start_x: Default::default(),
            start_y: Default::default(),
            end_x: Default::default(),
            end_y: Default::default(),
            is_invalid: Default::default(),
            field_0x9: Default::default(),
            is_room: Default::default(),
            is_connected: Default::default(),
            field_0xc: Default::default(),
            field_0xd: Default::default(),
            is_monster_house: Default::default(),
            field_0xf: Default::default(),
            is_maze_room: Default::default(),
            was_merged_into_other_room: Default::default(),
            is_merged_room: Default::default(),
            is_connected_to_top: Default::default(),
            is_connected_to_bottom: Default::default(),
            is_connected_to_left: Default::default(),
            is_connected_to_right: Default::default(),
            should_connect_to_top: Default::default(),
            should_connect_to_bottom: Default::default(),
            should_connect_to_left: Default::default(),
            should_connect_to_right: Default::default(),
            field_0x1b: Default::default(),
            flag_imperfect: Default::default(),
            flag_secondary_structure: Default::default(),
        }
    }
}

/// This helper struct can be used to create a grid of cells ([`DungeonGridCell`]).
///
/// All coordinates that this struct uses are "ingame" coordinates (so (x, y)), unless
/// otherwise noted.
/// 
/// It can also take ownership of existing `Vec<DungeonGridCell>` and manipulate them.
///
/// Finally you can convert this struct into it's inner `Vec<DungeonGridCell>` using
/// [`Self::into_inner()`]. See the notes on [`Self::new()`] for more information.
///
/// The cell grid is used in one phase of the dungeon generation. Note that most
/// of the generation functions here will generate tiles in the global dungeon struct.
///
/// Most of the methods of this will modify the global dungeon struct and are therefore marked
/// as `unsafe`.
pub struct DungeonGridMutator {
    cells: Vec<DungeonGridCell>,
    width: usize,
    height: usize,
    _lease: OverlayLoadLease<29>
}

impl DungeonGridMutator {
    /// Takes ownership of the given `in_cells` and returns a new `DungeonGridMutator`.
    ///
    /// The grid is a vector of grid cells stored in column-major order (!) (such that grid cells
    /// with the same x value are stored contiguously (y, x))
    ///
    /// The dimensions passed in must match the dimensions of the `Vec<DungeonGridCell>`,
    /// otherwise this will panic.
    ///
    /// Internally the grid has a fixed buffer size / capacity of 15 x 15. If the grid size in the
    /// X or Y direction is less than this, so when working with the (raw) grid buffer,
    /// you will need to take into account that each column will have 15 rows, where the last
    /// (`15-height`) in each column will be uninitialised and that after `width` rows
    /// the rest of the array will be uninitialised as well.
    ///
    /// This helper will abstract for you over this, if you query a cell via eg. [`Self::get`],
    /// however [`Self::into_inner()`] ill still return a vector with 15 x 15 cells, with all
    /// missing values initialized with defaults.
    ///
    /// Note that the game usually works with the assumption that there are exactly 15 rows.
    /// Using other row sizes may or may not lead to UB with some methods.
    ///
    /// The maximum values for `width` and `height` are `15`, otherwise this function will panic.
    pub fn new_from_vec(in_cells: Vec<DungeonGridCell>, width: usize, height: usize, ov29: OverlayLoadLease<29>) -> Self {
        assert!(width <= GRID_CAPACITY_DIM && height <= GRID_CAPACITY_DIM);
        assert!(grid.len() == grid_width as usize * grid_height as usize);
        debug_assert!(cells.len() == GRID_CAPACITY_DIM * GRID_CAPACITY_DIM);

        let mut cells_iter = in_cells.into_iter().peekable();

        let mut cells = Vec::with_capacity(GRID_CAPACITY_DIM * (width - 1) + height);

        for x in 0..GRID_CAPACITY_DIM {
            if x < width {
                for y in 0..GRID_CAPACITY_DIM {
                    cells.push(if y < height {
                        cells_iter.next().unwrap()
                    } else {
                        Default::default()
                    });
                }
            } else {
                for _ in 0..GRID_CAPACITY_DIM {
                    cells.push(Default::default())
                }
            }
            if cells_iter.peek().is_none() {
                break;
            }
        }

        Self { cells, width, height, _lease: ov29 }
    }

    /// Initialize a dungeon grid with defaults.
    ///
    /// The grid is an array of grid cells stored in column-major order (!) (such that grid cells
    /// with the same x value are stored contiguously (y, x)).
    ///
    /// Internally the grid has a fixed buffer size / capacity of 15 x 15. If the grid size in the
    /// X or Y direction is less than this, so when working with the (raw) grid buffer,
    /// you will need to take into account that each column will have 15 rows, where the last
    /// (`15-height`) in each column will be uninitialised and that after `width` rows
    /// the rest of the array will be uninitialised as well.
    ///
    /// This helper will abstract for you over this, if you query a cell via eg. [`Self::get`],
    /// however [`Self::into_inner()`] ill still return a vector with 15 x 15 cells, with all
    /// missing values initialized with defaults.
    ///
    /// `width` must be less than or equal to 15, otherwise this function will panic.
    pub fn new(&self, width: usize, ov29: OverlayLoadLease<29>) -> Self {
        let height = GRID_CAPACITY_DIM;
        assert!(width <= GRID_CAPACITY_DIM);
        // The game stops after the last column that had cells, so this is the actual capacity:
        let min_grid_size = (GRID_CAPACITY_DIM * (width - 1) + height) as usize;
        let mut cells;
        /// SAFETY: We know the grid vector will be big enough.
        ///         We also know the height is at least 15, so all cells will be initialized.
        unsafe {
            cells = repeat_with(|| Default::default())
                .take(min_grid_size).collect::<Vec<DungeonGridCell>>();
            ffi::InitDungeonGrid(raw_grid.as_mut_ptr() as *mut _, width as i32, height as i32);
        }
        Self { cells, width, height, _lease: ov29 }
    }

    /// Extract the grid from the mutator, along with it's width and height.
    ///
    /// The grid will always be a 15x15 matrix, you need to ignore other cells, see
    /// the notes for [`Self::new()`] for more information.
    pub fn into_inner(self) -> (Vec<DungeonGridCell>, usize, usize) {
        debug_assert!(cells.len() == GRID_CAPACITY_DIM * GRID_CAPACITY_DIM);
        (self.cells, self.width, self.height)
    }

    /// Get the cell at the given coordinates.
    /// Panics if the coordinates are out of bounds.
    pub fn get(&self, x: usize, y: usize) -> &DungeonGridCell {
        debug_assert!(cells.len() == GRID_CAPACITY_DIM * GRID_CAPACITY_DIM);
        let coords = Self::get_coords(x, y);
        if coords >= self.cells.len() {
            panic!("Grid cell at ({}, {}) is out of bounds", x, y);
        }
        &self.cells[coords]
    }

    /// Get the cell at the given coordinates, no extra checking is done, just normal slice
    /// indexing is done. This is UB if overflow checks are disabled and the coordinates are oob.
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &DungeonGridCell {
        &self.cells[Self::get_coords(x, y)]
    }

    /// Get the cell at the given coordinates, mutably.
    /// Panics if the coordinates are out of bounds.
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut DungeonGridCell {
        debug_assert!(cells.len() == GRID_CAPACITY_DIM * GRID_CAPACITY_DIM);
        let coords = Self::get_coords(x, y);
        if coords >= self.cells.len() {
            panic!("Grid cell at ({}, {}) is out of bounds", x, y);
        }
        &mut self.cells[coords]
    }

    /// Get the cell at the given coordinates, mutable, no extra checking is done, just normal
    /// slice  indexing is done. This is UB if overflow checks are disabled and the coordinates are
    /// oob.
    pub unsafe fn get_mut_unchecked(&self, x: usize, y: usize) -> &mut DungeonGridCell {
        &mut self.cells[Self::get_coords(x, y)]
    }

    /// Merges two vertically stacked rooms into one larger room.
    ///
    /// # Arguments
    /// * `x` - x grid coordinate of the rooms to merge
    /// * `y` - y grid coordinate of the rooms to merge
    /// * `dy` - dy, where the lower room has a y grid coordinate of y+dy
    pub unsafe fn merge_rooms_vertically<'a>(&mut self, x: i32, y: i32, dy: i32) {
        assert!(x >= 0 && y >= 0 && dy >= 0);
        assert!(x as usize <= self.width && y as usize <= self.height && (y+dy) as usize <= self.height);
        /// SAFETY: We checked the coordinates.
        unsafe { ffi::MergeRoomsVertically(x, y, dy, self.cells.as_mut_ptr()) }
    }

    /// Generate extra hallways on the floor via a series of random walks.
    ///
    /// Each random walk starts from a random tile in a random room, leaves the room in a
    /// random cardinal direction, and from there tunnels through obstacles through a series of
    /// random turns, leaving open terrain in its wake. The random walk stops when it reaches open
    /// terrain, goes out of bounds, or reaches an impassable obstruction.
    pub unsafe fn generate_extra_hallways(&mut self, number_extra_hallways: i32) {
        assert!(width > 0 && height > 0);
        assert!(grid.len() >= grid_width as usize * grid_height as usize);
        /// SAFETY: We checked the grid size.
        unsafe {
            ffi::GenerateExtraHallways(
                self.cells.as_mut_ptr(),
                self.width as i32, self.height as i32,
                number_extra_hallways
            )
        }
    }

    /// Get the grid cell positions for a given set of floor grid dimensions. Width and height
    /// must be positive.
    pub unsafe fn get_grid_positions(width: i32, height: i32, _ov29: &OverlayLoadLease<29>) -> (Vec<i32>, Vec<i32>) {
        assert!(width > 0 && height > 0);
        let mut x_positions = vec![0; width as usize];
        let mut y_positions = vec![0; height as usize];

        /// SAFETY: We made sure the positions vectors are big enough.
        unsafe {
            ffi::GetGridPositions(
                x_positions.as_mut_ptr(), y_positions.as_mut_ptr(),
                width, height
            );
        }

        (x_positions, y_positions)
    }

    /// Randomly selects a subset of grid cells to become rooms.
    ///
    /// The given number of grid cells will become rooms. If any of the selected grid cells are
    /// invalid, fewer rooms will be generated. The number of rooms assigned will always be at
    /// least 2 and never exceed 36.
    ///
    /// Cells not marked as rooms will become hallway anchors. A hallway anchor is a single tile
    /// in a non-room grid cell to which hallways will be connected later, thus "anchoring"
    /// hallway generation.
    ///
    /// `number_rooms` is the number of rooms; if positive, a random value between
    /// \[n_rooms, n_rooms+2\] will be used. If negative, |n_rooms| will be used exactly.
    pub unsafe fn assign_rooms(&mut self, number_rooms: i32) {
        /// SAFETY: We checked the grid size.
        unsafe { ffi::AssignRooms(
            self.cells.as_mut_ptr(),
            self.width as i32, self.height as i32,
            number_rooms
        ) }
    }

    /// Creates rooms and hallway anchors in each grid cell as designated by [`Self::assign_rooms`].
    ///
    /// This function creates a rectangle of open terrain for each room (with some margin relative
    /// to the grid cell border). A single open tile is created in hallway anchor cells, and a
    /// hallway anchor indicator is set for later reference.
    ///
    /// Panics if any start position is invalid.
    ///
    /// # Arguments
    /// * `start_x` - Array of the starting x coordinates of each grid column
    /// * `start_y` - Array of the starting y coordinates of each grid row
    /// * `room_flags` - Only uses bit 2 (mask: 0b100), which enables room imperfections
    pub unsafe fn create_rooms_and_anchors(&mut self, starts_x: &mut [i32], starts_y: &mut [i32], room_flags: u32) {
        Self::assert_start_positions_valid(starts_x, starts_y);
        /// SAFETY: We checked the grid size & start positions.
        unsafe {
            ffi::CreateRoomsAndAnchors(
                self.cells.as_mut_ptr(),
                self.width as i32, self.height as i32,
                starts_x.as_mut_ptr(), starts_y.as_mut_ptr(), room_flags
            )
        }
    }
    /// Try to generate secondary structures in flagged rooms.
    ///
    /// If a valid room with no special features is flagged to have a secondary structure, try to
    /// generate a random one in the room, based on the result of a dice roll:
    ///
    /// ```text
    ///   0: no secondary structure
    ///   1: maze, or a central water/lava "plus sign" as fallback, or a single water/lava tile in
    ///      the center as a second fallback
    ///   2: checkerboard pattern of water/lava
    ///   3: central pool of water/lava
    ///   4: central "island" with items and a Warp Tile, surrounded by a "moat" of water/lava
    ///   5: horizontal or vertical divider of water/lava splitting the room in two
    /// ```
    ///
    /// If the room isn't the right shape, dimension, or otherwise doesn't support the selected
    /// secondary structure, it is left untouched.
    pub unsafe fn generate_secondary_structures(&mut self, number_rooms: i32) {
        /// SAFETY: We checked the grid size.
        unsafe{ ffi::GenerateSecondaryStructures(
            self.cells.as_mut_ptr(),
            self.width as i32, self.height as i32,
        ) }
    }

    /// Randomly assigns connections between adjacent grid cells.
    ///
    /// Connections are created via a random walk with momentum, starting from the grid cell at
    /// (cursor x, cursor y). A connection is drawn in a random direction from the current cursor,
    /// and this process is repeated a certain number of times (the "floor connectivity" specified
    /// in the floor properties). The direction of the random walk has "momentum"; there's a 50%
    /// chance it will be the same as the previous step (or rotated counterclockwise if on the
    /// boundary).
    ///
    /// This helps to reduce the number of dead ends and forks in the road caused by the random
    /// walk "doubling back" on itself.
    ///
    /// If dead ends are disabled in the floor properties, there is an additional phase to remove
    /// dead end hallway anchors (only hallway anchors, not rooms) by drawing additional
    /// connections. Note that the actual implementation contains a bug: the grid cell validity
    /// checks use the wrong index, so connections may be drawn to invalid cells.
    ///
    /// Panics if the cursor positions are out of bounds.
    pub unsafe fn assign_grid_cell_connections(&mut self, cursor_x: i32, cursor_y: i32, floor_properties: &ffi::floor_properties) {
        assert!(cursor_x >= 0 && cursor_x < self.width && cursor_y >= 0 && cursor_y < self.height);
        /// SAFETY: We checked the grid size and cursor positions.
        unsafe { ffi::AssignGridCellConnections(
            self.cells.as_mut_ptr(),
            self.width as i32, self.height as i32,
            cursor_x, cursor_y, force_mut_ptr!(floor_properties)
        ) }
    }

    /// Create grid cell connections either by creating hallways or merging rooms.
    ///
    /// When creating a hallway connecting a hallway anchor, the exact anchor coordinates are used
    /// as the endpoint. When creating a hallway connecting a room, a random point on the room edge
    /// facing the hallway is used as the endpoint. The grid cell boundaries are used as the middle
    /// coordinates for kinks (see [`super::GlobalDungeonStructureGenerator::create_hallway`]).
    ///
    /// If room merging is enabled, there is a 9.75% chance that two connected rooms will be merged
    /// into a single larger room (9.75% comes from two 5% rolls, one for each of the two rooms
    /// being merged). A room can only participate in a merge once.
    pub unsafe fn create_grid_cell_connections(&mut self, starts_x: &mut [i32], starts_y: &mut [i32], enable_room_merging: bool) {
        Self::assert_start_positions_valid(starts_x, starts_y);
        /// SAFETY: We checked the grid size & start positions.
        unsafe {
            ffi::CreateGridCellConnections(
                self.cells.as_mut_ptr(),
                self.width as i32, self.height as i32,
                starts_x.as_mut_ptr(), starts_y.as_mut_ptr(),
                (!enable_room_merging) as ffi::bool_
            )
        }
    }

    /// Attempt to generate room imperfections for each room in the floor layout, if enabled.
    ///
    /// Each room has a 40% chance of having imperfections if its grid cell is flagged to allow
    /// room imperfections. Imperfections are generated by randomly growing the walls of the room
    /// inwards for a certain number of iterations, starting from the corners.
    pub unsafe fn generate_room_imperfections(&mut self) {
        /// SAFETY: We checked the grid size.
        unsafe { ffi::GenerateRoomImperfections(self.cells.as_mut_ptr(), self.width as i32, self.height as i32) }
    }

    /// Ensure the grid forms a connected graph (all valid cells are reachable) by adding hallways
    /// to unreachable grid cells.
    ///
    /// If a grid cell cannot be connected for some reason, remove it entirely.
    pub unsafe fn ensure_connected_grid(&mut self, starts_x: &mut [i32], starts_y: &mut [i32]) {
        Self::assert_start_positions_valid(starts_x, starts_y);
        /// SAFETY: We checked the grid size & start positions.
        unsafe { ffi::EnsureConnectedGrid(
            self.cells.as_mut_ptr(), self.width as i32, self.height as i32,
            starts_x.as_mut_ptr(), starts_y.as_mut_ptr()
        ) }
    }

    //// A Kecleon shop will be generated with a probability determined by the Kecleon shop spawn
    /// chance parameter (percentage from 0 to 100).
    ///
    /// A Kecleon shop will be generated in a random room that is valid, connected, has no other
    /// special features, and has dimensions of at least 5x4. Kecleon shops will occupy the entire
    /// room interior, leaving a one tile margin from the room walls.
    pub unsafe fn generate_kecleon_shop(&mut self, spawn_chance: u8) {
        assert!(spawn_chance <= 100);
        /// SAFETY: We checked the grid size.
        unsafe { ffi::GenerateKecleonShop(
            self.cells.as_mut_ptr(),
            self.width as i32, self.height as i32,
            spawn_chance as c_int
        ) }
    }

    //// Possibly generate a Monster House on the floor.
    ///
    /// A Monster House will be generated with a probability determined by the Monster House
    /// spawn chance parameter, and only if the current floor can support one (no non-Monster-House
    /// outlaw missions or special floor types). A Monster House will be generated in a random room
    /// that is valid, connected, and is not a merged or maze room.
    ///
    /// `spawn_chance` is the percentage chance that a Monster House will be generated (0-100).
    pub unsafe fn generate_monster_house(&mut self, spawn_chance: u8) {
        assert!(spawn_chance <= 100);
        /// SAFETY: We checked the grid size.
        unsafe { ffi::GenerateMonsterHouse(
            self.cells.as_mut_ptr(),
            self.width as i32, self.height as i32,
            spawn_chance as c_int
        ) }
    }

    /// Possibly generate a maze room on the floor.
    ///
    /// A maze room will be generated with a probability determined by the maze room chance
    /// parameter. A maze will be generated in a random room that is valid, connected, has odd
    /// dimensions, and has no other features.
    ///
    /// `spawn_chance` is the percentage chance that a Monster House will be generated (0-100).
    pub unsafe fn generate_maze_room(&mut self, spawn_chance: u8) {
        assert!(spawn_chance <= 100);
        /// SAFETY: We checked the grid size.
        unsafe { ffi::GenerateMazeRoom(
            self.cells.as_mut_ptr(),
            self.width as i32, self.height as i32,
            spawn_chance as c_int
        ) }
    }

    /// Generate a maze room within a given grid cell.
    /// A "maze" is generated within the room using a series of random walks to place obstacle
    /// terrain (walls or secondary terrain) in a maze-like arrangement. "Maze lines"
    /// (see [`super::GlobalDungeonStructureGenerator::generate_maze_line`]) are
    /// generated using every other tile around the room's border, as well as every other interior
    /// tile, as a starting point.
    /// This ensures that there are stripes of walkable open terrain surrounded by stripes of
    /// obstacles (the maze walls).
    pub fn generate_maze<'a>(&'a self, cell: &'a mut ffi::dungeon_grid_cell, secondary_terrain_instead_of_walls: bool) {
        unsafe { ffi::GenerateMaze(cell as *mut _, secondary_terrain_instead_of_walls as ffi::bool_) }
    }

    /// Set spawn flag 5 (0b100000 or 0x20) on all tiles in a room.
    pub fn set_spawn_flag_5<'a>(&'a self, cell: &'a mut ffi::dungeon_grid_cell) {
        unsafe { ffi::SetSpawnFlag5(cell as *mut _) }
    }
}
