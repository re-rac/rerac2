//! The level's height map (core header `+0xa4`; Going Commando's level load `0x304660` hands it to `0x2fe0c8`, which
//! copies the 0x40-byte header to `0x1b1ca0` and keeps the cells' address at `0x1b1ce0`; 0 when the word is 0). It
//! is a 256 × 256 grid of byte heights over the level's ground plan: the rain stops on its top (`0x3269e0`).

use crate::buf::{Buf, Result};
use crate::level::LevelCore;

/// Cells per side.
pub const SIDE: usize = 256;
/// The header's bytes (`0x1b1ca0..0x1b1ce0`).
const HEADER: usize = 0x40;

/// A level's height map.
#[derive(Clone, Debug)]
pub struct HeightMap {
    /// `+0x00`: the grid's corner (x, y) and the height of a cell of 0 (z).
    pub origin: [f32; 3],
    /// `+0x30` / `+0x34`: a cell's width along x and y.
    pub cell: [f32; 2],
    /// `+0x38`: the height of one step of a cell.
    pub step: f32,
    /// The cells, row (y) by row; 0xff: no height (the rain falls through).
    pub cells: Vec<u8>,
}

impl HeightMap {
    /// `0x3269e0` without its variant test: the height at `p`'s place, or None outside the grid or over an empty
    /// cell (the game's 1023).
    pub fn height(&self, p: [f32; 3]) -> Option<f32> {
        let x = (p[0] - self.origin[0]) / self.cell[0];
        let y = (p[1] - self.origin[1]) / self.cell[1];
        if !(0.0..=256.0).contains(&x) || !(0.0..=256.0).contains(&y) { return None; }
        // `cvt.w.s` truncates; 256 itself reads the next row's first cell, as the game does (and past the end, none).
        let c = *self.cells.get((y as usize) * SIDE + x as usize)?;
        (c != 0xff).then(|| c as f32 * self.step + self.origin[2])
    }
}

/// The level's height map, or None when core header `+0xa4` is 0.
pub fn parse_height_map(core: &LevelCore, core_data: &[u8]) -> Result<Option<HeightMap>> {
    let at = core.header.heightmap_offset;
    if at <= 0 { return Ok(None); }
    let b = Buf(core_data).sub(at as usize, HEADER + SIDE * SIDE, "height map")?;
    Ok(Some(HeightMap {
        origin: [b.f32(0)?, b.f32(4)?, b.f32(8)?],
        cell: [b.f32(0x30)?, b.f32(0x34)?],
        step: b.f32(0x38)?,
        cells: b.bytes()[HEADER..].to_vec(),
    }))
}
