//! The water plane: a flat water height about a place, the second answer of the water's height query `0x31d7a0`
//! (after the liquid grid's patches, crate::water::grid::Grid::surface). Its globals sit beside the grid unit's (L00
//! `0x1b25bc..0x1b25cc`, L12 `0x1b35bc..0x1b35cc`: on, then the place and the radius as one vector whose z is the
//! height); the level start clears them. Its one writer is Todano's mesh water (crate::water::mesh), every tick.
//!
//! **The query** (`0x31d7a0`'s second part): with the plane on, a point within 0.5 of its height (`0x2eedb0`, the
//! absolute value) and within its radius of its place (`0x2ef050`, the distance in x, y and z) has the plane's height.

use crate::hero::physics::{len, sub, V3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Plane {
    /// `0x1b35bc`.
    pub on: bool,
    /// `0x1b35c0..0x1b35c8`: the place, its z the height.
    pub at: V3,
    /// `0x1b35cc`.
    pub radius: f32,
}

impl Plane {
    /// The query's plane part (module docs); None outside it.
    pub fn height(&self, p: V3) -> Option<f32> {
        if !self.on || 0.5 <= (p[2] - self.at[2]).abs() || self.radius <= len(sub(p, self.at)) { return None; }
        Some(self.at[2])
    }
}
