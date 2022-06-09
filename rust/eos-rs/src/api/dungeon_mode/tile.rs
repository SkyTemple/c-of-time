use crate::api::dungeon_mode::*;
use crate::api::objects::*;
use crate::ffi;
use core::slice;

/// Extension trait for [`DungeonTile`].
///
/// # Important safety note
/// Please see the safety note of [`DungeonEntityExt`]. It also applies to this trait.
pub trait DungeonTileExt {
    /// Initializes the tile struct.
    fn init(&mut self);

    /// Gets the terrain type of a tile.
    /// Returns None if the terrain type is invalid.
    fn get_terrain(&self) -> Option<TerrainType>;

    /// Set the terrain of a specific tile to be an obstacle (wall or secondary terrain).
    ///
    /// If `secondary_terrain` is true, secondary terrain will be placed, otherwise wall.
    ///
    /// Secondary terrain (water/lava) can only be placed in the specified room.
    /// If the tile room index does not match, a wall will be placed instead.
    fn set_terrain_obstacle_checked(&mut self, secondary_terrain: bool, room_index: u8);

    /// Set a specific tile to have secondary terrain (water/lava), but only if it's a passable wall.
    fn set_secondary_terrain_on_wall(&mut self);
}

impl DungeonTileExt for DungeonTile {
    fn init(&mut self) {
        unsafe { ffi::InitializeTile(self as *mut _) };
    }

    fn get_terrain(&self) -> Option<TerrainType> {
        unsafe { ffi::GetTileTerrain(force_mut_ptr!(self)) }
            .try_into()
            .ok()
    }

    fn set_terrain_obstacle_checked(&mut self, secondary_terrain: bool, room_index: u8) {
        unsafe {
            ffi::SetTerrainObstacleChecked(
                self as *mut _,
                secondary_terrain as ffi::bool_,
                room_index,
            )
        }
    }

    fn set_secondary_terrain_on_wall(&mut self) {
        unsafe { ffi::SetSecondaryTerrainOnWall(self as *mut _) }
    }
}

/// Functions for reading from a tile grid.
pub trait DungeonTileGridRead<const W: usize, const H: usize> {
    /// Returns the tile at the given position. Panics if the position is out of bounds.
    fn get(&self, x: usize, y: usize) -> Option<&DungeonTile>;

    /// Iterate over all tiles in the grid, in row-major order (from top-left to bottom-right).
    fn iter(&self) -> DungeonTileGridIter<W>;
}

/// Functions for writing into a tile grid.
pub trait DungeonTileGridWrite<const W: usize, const H: usize>: DungeonTileGridRead<W, H> {
    /// Returns the tile at the given position, mutably. Panics if the position is out of bounds.
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut DungeonTile>;
    /// Place the given tile at the given position. This overwrites the data of the tile that is
    /// pointed to at this location. Panics if the position is out of bounds.
    fn insert(&mut self, x: usize, y: usize, tile: DungeonTile);

    /// Iterate over all tiles in the grid, in row-major order (from top-left to bottom-right).
    fn iter_mut(&mut self) -> DungeonTileGridIterMut<W>;
}

/// See [`DungeonTileGridRead`].
pub struct DungeonTileGridRef<'a, const W: usize, const H: usize>(
    pub(crate) &'a [[*mut ffi::tile; W]; H],
);
/// See [`DungeonTileGridRead`] and [`DungeonTileGridWrite`].
pub struct DungeonTileGridMut<'a, const W: usize, const H: usize>(
    pub(crate) &'a mut [[*mut ffi::tile; W]; H],
);

impl<'a, const W: usize, const H: usize> DungeonTileGridRead<W, H>
    for DungeonTileGridRef<'a, W, H>
{
    fn get(&self, x: usize, y: usize) -> Option<&DungeonTile> {
        unsafe { self.0[y][x].as_ref() }
    }

    fn iter(&self) -> DungeonTileGridIter<W> {
        DungeonTileGridIter {
            tiles_iter: self.0.iter(),
            cur_row: None,
            cur_row_iter: None,
        }
    }
}

impl<'a, const W: usize, const H: usize> DungeonTileGridRead<W, H>
    for DungeonTileGridMut<'a, W, H>
{
    fn get(&self, x: usize, y: usize) -> Option<&DungeonTile> {
        unsafe { self.0[y][x].as_ref() }
    }

    fn iter(&self) -> DungeonTileGridIter<W> {
        DungeonTileGridIter {
            tiles_iter: self.0.iter(),
            cur_row: None,
            cur_row_iter: None,
        }
    }
}

impl<'a, const W: usize, const H: usize> DungeonTileGridWrite<W, H>
    for DungeonTileGridMut<'a, W, H>
{
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut DungeonTile> {
        unsafe { self.0[y][x].as_mut().map(|tile| &mut *tile) }
    }

    fn insert(&mut self, x: usize, y: usize, tile: DungeonTile) {
        let mut otile = unsafe { &mut *self.0[y][x] };
        otile._bitfield_align_1 = tile._bitfield_align_1;
        otile._bitfield_1 = tile._bitfield_1;
        otile.spawn_or_visibility_flags = tile.spawn_or_visibility_flags;
        otile.texture_id = tile.texture_id;
        otile.field_0x6 = tile.field_0x6;
        otile.room = tile.room;
        otile.walkable_neighbor_flags = tile.walkable_neighbor_flags;
        otile.monster = tile.monster;
        otile.object = tile.object;
    }

    fn iter_mut(&mut self) -> DungeonTileGridIterMut<W> {
        DungeonTileGridIterMut {
            tiles_iter: self.0.iter(),
            cur_row: None,
            cur_row_iter: None,
        }
    }
    // TODO
}

/// A iterator over the tiles in a dungeon tile grid.
pub struct DungeonTileGridIter<'a, const W: usize> {
    tiles_iter: slice::Iter<'a, [*mut ffi::tile; W]>,
    cur_row: Option<&'a [*mut ffi::tile; W]>,
    cur_row_iter: Option<slice::Iter<'a, *mut ffi::tile>>,
}

impl<'a, const W: usize> Iterator for DungeonTileGridIter<'a, W> {
    type Item = &'a DungeonTile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_row.is_none() {
            self.cur_row = self.tiles_iter.next();
            self.cur_row_iter = Some(self.cur_row?.iter());
        }

        let tile = self.cur_row_iter.as_mut().unwrap().next();
        match tile {
            Some(tile) => Some(unsafe { &**tile }),
            None => {
                // Reached end of row. Unset current row and try next row.
                self.cur_row = None;
                self.next()
            }
        }
    }
}

/// A mutable iterator over the tiles in a dungeon tile grid.
pub struct DungeonTileGridIterMut<'a, const W: usize> {
    tiles_iter: slice::Iter<'a, [*mut ffi::tile; W]>,
    cur_row: Option<&'a [*mut ffi::tile; W]>,
    cur_row_iter: Option<slice::Iter<'a, *mut ffi::tile>>,
}

impl<'a, const W: usize> Iterator for DungeonTileGridIterMut<'a, W> {
    type Item = &'a mut DungeonTile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_row.is_none() {
            self.cur_row = self.tiles_iter.next();
            self.cur_row_iter = Some(self.cur_row?.iter());
        }

        let tile = self.cur_row_iter.as_mut().unwrap().next();
        match tile {
            Some(tile) => Some(unsafe { &mut **tile }),
            None => {
                // Reached end of row. Unset current row and try next row.
                self.cur_row = None;
                self.next()
            }
        }
    }
}
