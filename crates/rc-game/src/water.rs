//! The water surfaces: the game's water draws by kind, each ported once and shared by every class and level that
//! runs the same code (G-REN-009).
//!
//! | Kind | Classes (levels) | Code | Port |
//! |---|---|---|---|
//! | The ocean about the camera (R&C1's oceans: 1111's `0x30a908`, Pokitaru's) | the swamp 0x0a17 (L01), the sea 0x12b5 (L09, L20), the sheet 0x093e (L06, L08, L11, L13, L16, L18) | L01 `0x3bcdd8` / `0x3bc3c0`; L09 `0x3819a0` / `0x380fe8` (= L20 `0x39c9d0`); L06 `0x404680` / `0x403ac0` (the decomp's `func_L06_00403130` / `func_L06_00402570`, on six levels) | [`ocean`] |
//! | The liquid lids: squares of layered, scrolling quads | 0x11fe (L08), 0x12af / 0x12b4 (L09), 0x12b4 (L19) | L08 `0x430568` / `0x42fce0` (the decomp's `func_L08_0042EF00` / `func_L08_0042E678`; L09 `0x423d20` / `0x423498`, L19 `0x422c70` / `0x4223e8`) | [`lid`] |
//! | The liquid grid: 16 × 16 rippling patches the Thermanator freezes, with their own unit | 0x0960 (L04), 0x0fb2 / 0x0fce / 0x0fcf / 0x0fd6 / 0x0fe4 (L06), 0x0908 (L08), 0x0f59 (L18), 0x124d (L20) | L04 `0x3a2e68` / `0x3a3088` / `0x3a3390` / `0x3a33d0`, draw `0x3a35e8` (the decomp's `func_L04_003A0FC8` / `func_L04_003A15C0`, on levels 4, 6, 8, 18, 20) | [`grid`] |
//!
//! Not ported yet (each its own kind):
//! * Todano's (L12) mesh water 0x1113 (`0x421fe0`, the decomp's `func_L12_00420B78`: 294 overlay vertices (v1.01
//!   `D_L12_002A11B0`) waved in eight bands, drawn by `func_L12_00420950` unless Ratchet is in its cuboid `+0x18`; it
//!   also sets the level's water plane: v1.01 `D_L12_001B363C` on, `D_L12_001B3640` Ratchet's x, y, 59.5 + 0.25·sin,
//!   radius 48) and 0x1350 (`0x38ae28`, the decomp's `func_L12_00389BC0`).
//!
//! The heights: the ocean keeps its level and its waved height in crate::game::GameState::water_base /
//! `water_z` (what Oozla's floating objects read, `0x3bc3b8`). Ratchet's liquid heights (`0x18a150` / `0x18a154`,
//! crate::hero::surface) and the underwater test (crate::fog) take the collision faces' heights through `0x31d7a0`:
//! the liquid grid's rippled height where one of its drawn patches holds the point ([`grid::Grid::height`], the
//! engine's ripple query `0x37dc10`), else the face's own (no ocean feeds them in the original either); the grid's
//! movers also set `0x18a150` with Ratchet in their cuboid.

pub mod grid;
pub mod lid;
pub mod ocean;
