//! The liquid grid: a rippling liquid surface made of 16 × 16 patches that the Thermanator freezes, with its own unit
//! (the level-shared code of levels 4, 6, 8, 18 and 20, L04 addresses below; the decomp's `func_L04_0039FEF0`,
//! `func_L04_003A0FC8`, `func_L04_003A15C0`, …; R&C1's ripple module at level01 `0x2b7a48..0x2b96c0` is the same
//! design, randcre's `rc_game::water`). Ported once; each level's class wrapper is one [`Site`]:
//!
//! | Level | Classes | Wrapper | Records (count) | Unit |
//! |---|---|---|---|---|
//! | 4 Barlow | 0x0960 (9 placed) | `0x3e9320` | `0x2af7c0` (10) | `0x3a2e68` / `0x3a3088` / `0x3a3390` / `0x3a33d0`, draw `0x3a35e8` |
//! | 6 Notak | 0x0fb2 / 0x0fce / 0x0fcf / 0x0fd6 / 0x0fe4 (15) | `0x4581c8` (the decomp's `func_L06_00456BB0`) | `0x2aa580` (48) | `0x3c0980` / `0x3c0ba0` / `0x3c0ea8` / `0x3c0ee8`, draw `0x3c1100` |
//! | 8 Tabora | 0x0908 (3) | `0x3d12c0`, draw callback `0x3d1440` | `0x2a0d80` (7) | `0x38dae0` / `0x38dd00` / `0x38e008` / `0x38e048`, draw `0x38e260` |
//! | 18 Damosel | 0x0f59 (22) | `0x437000` (`func_L18_00435B10`) | `0x2a9c00` (18) | `0x39b5a8` / `0x39b7c8` / `0x39bad0` / `0x39bb10`, draw `0x39bd28` |
//! | 20 Yeedil | 0x124d (6) | `0x448c90` (`func_L20_00447748`) | `0x2ab980` (6) | `0x3952c0` / `0x3954e0` / `0x3957e8` / `0x395828`, draw `0x395a40` |
//!
//! **The records** (0x1190 bytes each, the level's data; [`Tables`], read at load): `+0x00` the patch's centre, `+0x0c`
//! eight neighbour links, `+0x14` / `+0x18` its two textures (FX 0x3f / 0x40 on every level), `+0x1c` / `+0x1d` two
//! blend factors (0x60 / 0x40), `+0x1e` the drawn 4 × 4 sub-blocks (a bit each), `+0x20..+0x44` the water texture's
//! scroll and wobble, `+0x48` frozen, `+0x4c` the thaw / freeze blend, then three height buffers of 16 × 16 cells and
//! their halos (`+0x50 + k·0x5c0`): the newest (`gp−0x17ec`, [`Grid::cur`]), the one before (`gp−0x17e8`) and the drawn
//! one (`gp−0x17e4`: buffer 2 between steps).
//!
//! **The moby** (pvars: `+0x00` / `+0x04` its high and low heights, `+0x08` the phase (degrees, made radians), `+0x0c`
//! the rate (degrees a second, made radians a tick), `+0x10` its first record (−1 none), `+0x14` frozen, `+0x15` the
//! hits to turn (0x19), `+0x16` the turn's timer, `+0x18` its group (`groups[+0x18]` records from its first; −1: one,
//! and no ripples run), `+0x20` the cuboid that holds Ratchet's water height). Its state (`+0x20`):
//! * **0** (`0x3a2e68`): the timer `+0xbc` 0, mode | 1, its class's type 0x12 (a Thermanator target), `+0x15` 0x19, the
//!   phase and rate made radians; its collision faces made water (surface 0) or ice (7) by `+0x14` (`0x3a29b0`: two
//!   copies of the class's collision, their faces' type bits `0x3a28a8(0)` / `(7)`, swapped by `0x3a2968`). The moby
//!   of record 0 (`+0x10` = 0): its update distance 0xff and the unit's init ([`Grid::init`]), with two drops on every
//!   record at `randf(±6)` (radius 2, 0.2) unless frozen. Its draw distance 0 (the model is not drawn); 1, and on to 1.
//! * **1** (`0x3a3088`): with `+0xbc` (the Thermanator's hits: its cold shot's `0x41e180` = L00 `0x3e3400` on water,
//!   its hot shot's `0x3e1c18` on ice, crate::moby::classes::thermanator) at `+0x15` or more: 2, the
//!   hits 0, the timer 60, frozen flipped (its faces with it); thawed, a drop on each of its records (radius 1,
//!   −0.05, added). Frozen: nothing more. Else Ratchet in cuboid `+0x20`: his water height `0x18a150` its height; its
//!   height `low + h·cos(phase) + h` (`h` half of high less low), the phase moved by the rate unless Ratchet's group is
//!   20 / 25 or his state 0x39 / 0x78 / 0x79; one time in 200 a drop on each of its records.
//! * **2** (`0x3a3390`): the timer down (`0x30f718`); at 0: 1.
//! * **Every tick** (`0x3a33d0`, with `+0x10` ≥ 0 and `+0x18` ≠ −1): each of its records at its height, drawn (its
//!   `masks` entry) when its sphere (the centre + 8, radius 22.6) is in view within 71.5 and within 71.5 of the camera,
//!   its blend `1 − timer/60`, frozen as the moby; the moby of record 0 then runs the clock ([`Grid::clock`],
//!   `0x3a13f8`) and registers the draw ([`draw`]): on the second list with the camera under water (`0x1b9980` = L00
//!   `0x1b9000`), else on the first.
//!
//! The wrappers first return while Ratchet steers a class 0x45 (`0x18b650`, its state not 0xfe / 0xfd) in his state
//! 0x31. Per level besides:
//! * 18: a moby not of record 0, out of state 0, with area `+0x28` set and the camera (`0x1b9740`) not in it: nothing.
//! * 20: in state 2, the mobys of `+0x30..+0x3c` (−1 none) hooked (`0x33faf8` = L00 `0x32a818`) while frozen, else not.
//! * 6: thawed and grouped, its loop (the level sound 20, flags 4: `0x3a1e90` = L00 `0x3607e8`, the slot `+0x38`) kept
//!   playing, else stopped (`0x3a1998` = L00 `0x3602f0`); in state 0 with cuboid `+0x28` its update distance 0xff; in
//!   state 2, grouped and within 2 of `+0x2c`, its height eased there (`0x35efa8` = L00 `0x31ed78(+0x2c, 1/600,
//!   1/600, 0.1)`, the speed `+0x30`); last, with cuboid `+0x28` not holding the camera (`0x1b9840`) — nor, under
//!   water, the camera's x, y at its high height — its records not drawn.
//!
//! **The unit.**
//! * The init (`0x3a0df8` + `0x3a2c58` + `0x3a2dd8` + `0x3a2630` + `0x3a2e38`): every record's heights 0, scroll 0,
//!   speeds −0.0016, wobble angles ±π/2 turning by 0.02 / −0.031, radius 0.03; the constants (16, 16) the patch,
//!   (−8, −8) its corner, (1, 1) the cell, 0.9 the reflection, 0.1 the clock's step (0.16 the damping threshold, the
//!   data's); the light `(cos a, sin a, −1)·0.57735` with `a` the level's first light set's A direction's angle; each
//!   record's bounds (`0x3a1018`: its corner, `z − 1`, in 1/1024, 16 wide); the links (`0x3a2630`: the records 16 away
//!   in x, y or both, within 0.1); the masks.
//! * The clock (`0x3a13f8`): with `f ≥ 1 − 0.1` the step (`0x3a12e0`: f 0.1, the drawn buffer the one before), else
//!   buffer 2 `= before·(1 − f) + newest·f` (`0x330530`); then `f += 0.1` (on the PS2's truncating adds: a step every
//!   nine ticks). The step: for every drawn record, `n = 0.25·Σ8 − before` over the newest (`0x3303c8`, `0x33081c`),
//!   damped `n − n·k` when frozen (k 0.35) or when `|n| ≥ 0.16` (k 0.00625), into the one before with its edges copied
//!   into the neighbours'; then the two swap.
//! * A drop (`0x3a16b0`, the engine's `0x37d5a0`; `0x3a1690` its plain mode): [`Grid::disturb`]. The height query
//!   (`0x3a1d20` = L00 `0x37dc10`, through `0x31d7a0` while the unit is on, `0x1b2c78`): [`Grid::height`], which
//!   crate::hero::physics's ground probe and crate::fog's underwater test read.
//!
//! **The draw** (`0x3a35e8`, [`draw`]): every drawn record, its blend `b = 1 − (1 − x)²` of `x` its blend (frozen) or
//! `1 − blend`: the wobble angles × (1 − b), the speeds and turns (1 − b) × (−0.0016, 0.02, −0.031); the colour
//! `0x8f7f2f` → `0xbfbfbf` by `b` (`0x310cf0`); then, in view (`0x3a35e8` clips its four corners: here a sphere of
//! 11.32 about it), the drawn buffer's vertices (`0x3305b0`: the normal from the eight around, the colour × `clamp((n·L
//! − 0.573)·3.25 + 1.073, 0, 1)`, the reflection's sphere-map UV `r·0.45 + 0.5`), the scroll and wobble turned
//! (`0x3a14d8`) and each drawn sub-block (`0x3a20d8` → `0x330b84`) sent twice: its water (FX `+0x18`, UV `(c, r)/4 +
//! scroll + R·(cos, sin)` of the wobble angle of `(r + c) & 1`, ALPHA_1 `(Cs − Cd)·FIX + Cd`, FIX `+0x1d` → 0x60 by
//! `b`), then its sheen (FX `+0x14`, the sphere-map UV, ALPHA_2 `(Cs − Cd)·FIX + Cd` with FIX `(1 − 2b)·70` below
//! the half way, else `(Cs·FIX) + Cd` with `(2b − 1)·170` (170.5 freezing)). Both test Z without writing it (TEST
//! 0x53001). Here the FIX is the quads' alpha (the effects' blend, as crate::water::lid's).
//!
//! Not ported: the target list `0x3a7020` (`0x231e40`: the moby and its `+0x28` factor; for the Thermanator), the edge pins `+0x44` (0 on every
//! record: nothing sets them), the camera +0x2c0's second cuboid test on Notak reads the under-water flag.

use crate::fx::FxQuad;
use crate::hero::physics::V3;
use crate::moby::{Ctx, MobyWorld};
use crate::ps2v::Pf;

/// Every level's grid classes.
pub const CLASSES: [i16; 9] = [0x0960, 0x0fb2, 0x0fce, 0x0fcf, 0x0fd6, 0x0fe4, 0x0908, 0x0f59, 0x124d];

/// A level's copy: its class wrapper's records, their count, the groups' sizes and the masks.
#[derive(Clone, Copy, Debug)]
pub struct Site {
    pub level: i32,
    pub records: u32,
    pub count: usize,
    pub groups: u32,
    pub masks: u32,
}

pub const SITES: [Site; 5] = [
    Site { level: 4, records: 0x2af7c0, count: 10, groups: 0x1aff50, masks: 0x1aff70 },
    Site { level: 6, records: 0x2aa580, count: 48, groups: 0x1b1808, masks: 0x2df080 },
    Site { level: 8, records: 0x2a0d80, count: 7, groups: 0x1b0168, masks: 0x1b0178 },
    Site { level: 18, records: 0x2a9c00, count: 18, groups: 0x1b1638, masks: 0x1b1650 },
    Site { level: 20, records: 0x2ab980, count: 6, groups: 0x1b22e0, masks: 0x1b22e8 },
];

/// The groups' sizes read (the pvars' `+0x18` index them; 0..10 on the disc).
const GROUPS: usize = 16;

/// A record's placed part (`+0x00..+0x1d`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Record {
    pub centre: [f32; 3],
    /// `+0x14` / `+0x18`.
    pub tex: [i32; 2],
    /// `+0x1c` / `+0x1d`.
    pub fix: [u8; 2],
}

/// The level's tables (crate::level::LevelData::liquid).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tables {
    pub records: Vec<Record>,
    pub groups: Vec<i32>,
    pub masks: Vec<u16>,
    /// The level's first light set's A direction (x, y: `0x1c7440 + 0x10`).
    pub light: [f32; 2],
}

/// Level `level`'s tables from its overlay (`read(vaddr, len)`), `light` its first light set's A direction (x, y).
pub fn read_tables(level: i32, read: &dyn Fn(u32, usize) -> Option<Vec<u8>>, light: [f32; 2]) -> Option<Tables> {
    let c = SITES.iter().find(|c| c.level == level)?;
    let word = |b: &[u8], o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let records = (0..c.count)
        .map(|i| {
            let b = read(c.records + i as u32 * 0x1190, 0x20)?;
            Some(Record {
                centre: [0, 4, 8].map(|o| f32::from_bits(word(&b, o))),
                tex: [word(&b, 0x14) as i32, word(&b, 0x18) as i32],
                fix: [b[0x1c], b[0x1d]],
            })
        })
        .collect::<Option<Vec<_>>>()?;
    let g = read(c.groups, 4 * GROUPS)?;
    let m = read(c.masks, 2 * c.count)?;
    Some(Tables {
        records,
        groups: (0..GROUPS).map(|k| word(&g, 4 * k) as i32).collect(),
        masks: (0..c.count).map(|k| u16::from_le_bytes([m[2 * k], m[2 * k + 1]])).collect(),
        light,
    })
}

// ---------------------------------------------------------------------------------------------------------------------
// The unit.

/// Logical grid side: rows / columns −1..=17 (the 16 × 16 cells and their halos).
const G: usize = 19;
const fn gi(r: i32, c: i32) -> usize { ((r + 1) as usize) * G + (c + 1) as usize }
type Heights = [Pf; G * G];

const ONE: Pf = Pf::b(0x3f80_0000);
/// `0x3a2c58`'s constants: the patch (16), its corner (−8), the cell (1), the reflection (0.9), the clock's step
/// (0.1); the damping threshold 0.16 (`0x1ad83c`, the data's).
const SIZE: Pf = Pf::b(0x4180_0000);
const ORIGIN: Pf = Pf::b(0xc100_0000);
const CELL: Pf = ONE;
const REFL: f32 = 0.9;
const STEP: Pf = Pf::b(0x3dcc_cccd);
const DAMP: u32 = 0x3e23_d70a;
/// The texture's speed and the wobble's turns (`0x3a0df8`, the draw's).
const SPEED: u32 = 0xbad1_b717;
const TURN: [u32; 2] = [0x3ca3_d70a, 0xbcfd_f3b6];
/// The colours by the blend (`gp−0x15e0` / `gp−0x15dc`) and ALPHA_1's FIX frozen (`gp−0x15c8`).
const WATER: u32 = 0x008f_7f2f;
const ICE: u32 = 0x00bf_bfbf;
const FIX_ICE: u8 = 0x60;

/// Neighbour slots a written cell feeds: (link, the neighbour's row, column).
fn mirrors(r: i32, c: i32, mut f: impl FnMut(usize, i32, i32)) {
    if r == 0 { f(0, 16, c); }
    if r == 1 { f(0, 17, c); }
    if c == 0 { f(1, r, 16); }
    if c == 1 { f(1, r, 17); }
    if r == 15 { f(2, -1, c); }
    if c == 15 { f(3, r, -1); }
    match (r, c) {
        (0, 0) => f(4, 16, 16),
        (0, 1) => f(4, 16, 17),
        (1, 0) => f(4, 17, 16),
        _ => {}
    }
    if (r, c) == (0, 15) { f(5, 16, -1); }
    if (r, c) == (15, 0) { f(6, -1, 16); }
    if (r, c) == (15, 15) { f(7, -1, -1); }
}

/// One record at run time.
#[derive(Clone, Debug)]
pub struct Patch {
    pub centre: [Pf; 3],
    /// `+0x0c..+0x13` (0xff none).
    pub links: [u8; 8],
    pub tex: [i32; 2],
    pub fix: [u8; 2],
    /// `+0x1e`.
    pub mask: u16,
    /// `+0x20` / `+0x24` the scroll, `+0x28` / `+0x2c` its speeds, `+0x30` / `+0x34` the wobble's angles, `+0x38` /
    /// `+0x3c` their turns, `+0x40` its radius.
    scroll: [Pf; 2],
    speed: [Pf; 2],
    theta: [f32; 2],
    dtheta: [f32; 2],
    radius: f32,
    /// `+0x48` / `+0x4c`.
    pub frozen: bool,
    pub blend: f32,
    buf: Vec<Heights>,
    /// `0x3a1018`'s bounds: the corner and `z − 1` in 1/1024, the width.
    bounds: [i32; 3],
    extent: i32,
}

/// The unit's state (records `0x1b2c70`, count `0x1b2c78`; on L04).
#[derive(Clone, Debug, Default)]
pub struct Grid {
    pub patches: Vec<Patch>,
    /// The buffers: the newest, the one before, the drawn (`gp−0x17ec` / `−0x17e8` / `−0x17e4`).
    pub cur: usize,
    pub prev: usize,
    pub render: usize,
    /// `gp−0x1790`.
    pub clock: Pf,
    /// `0x1ad850`.
    light: [f32; 3],
    /// The water plane beside the unit's globals (`0x1b2c7c..`; crate::water::plane), kept by the init.
    pub plane: super::plane::Plane,
}

impl Grid {
    /// The unit is on (`0x1b2c78` ≠ 0: the height query asks it).
    pub fn on(&self) -> bool { !self.patches.is_empty() }

    /// The water's height query `0x31d7a0` (L12 `0x3211a0`): with the unit on, a drawn patch's rippled height there
    /// ([`Grid::height`]); else the water plane's (crate::water::plane::Plane::height); None: the point's own.
    pub fn surface(&self, p: V3) -> Option<f32> {
        if self.on() {
            if let Some(h) = self.height(p) { return Some(h); }
        }
        self.plane.height(p)
    }

    /// The init (module docs).
    pub fn init(&mut self, t: &Tables) {
        let k1024 = Pf::b(0x4480_0000);
        let mut patches: Vec<Patch> = t
            .records
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let centre = r.centre.map(Pf::f);
                Patch {
                    centre,
                    links: [0xff; 8],
                    tex: r.tex,
                    fix: r.fix,
                    mask: t.masks.get(i).copied().unwrap_or(0),
                    scroll: [Pf::ZERO; 2],
                    speed: [Pf::b(SPEED); 2],
                    theta: [std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2],
                    dtheta: TURN.map(f32::from_bits),
                    radius: f32::from_bits(0x3cf5_c28f),
                    frozen: false,
                    blend: 0.0,
                    buf: vec![[Pf::ZERO; G * G]; 3],
                    bounds: [
                        ((centre[0] + ORIGIN) * k1024).to_i32(),
                        ((centre[1] + ORIGIN) * k1024).to_i32(),
                        ((centre[2] - ONE) * k1024).to_i32(),
                    ],
                    extent: (SIZE * k1024).to_i32() as i16 as i32,
                }
            })
            .collect();
        // `0x3a2630`: the links.
        let n = patches.len();
        for i in 0..n {
            for j in (0..n).filter(|&j| j != i) {
                let dx = patches[i].centre[0].to_f32() - patches[j].centre[0].to_f32();
                let dy = patches[i].centre[1].to_f32() - patches[j].centre[1].to_f32();
                let near = |v: f32| v.abs() < 0.1;
                let l = &mut patches[i].links;
                if near(dx) {
                    if near(dy - 16.0) { l[0] = j as u8; }
                    if near(dy + 16.0) { l[2] = j as u8; }
                } else if near(dy) {
                    if near(dx - 16.0) { l[1] = j as u8; }
                    if near(dx + 16.0) { l[3] = j as u8; }
                } else if near(dx + 16.0) {
                    if near(dy - 16.0) { l[5] = j as u8; }
                    if near(dy + 16.0) { l[7] = j as u8; }
                } else if near(dx - 16.0) {
                    if near(dy - 16.0) { l[4] = j as u8; }
                    if near(dy + 16.0) { l[6] = j as u8; }
                }
            }
        }
        let a = t.light[1].atan2(t.light[0]);
        let k = f32::from_bits(0x3f13_cd36);
        *self = Grid { patches, cur: 0, prev: 1, render: 2, clock: Pf::b(0x3f8c_cccd), light: [a.cos() * k, a.sin() * k, -k], plane: self.plane };
    }

    /// Writes `v` to cell (r, c) of buffer `b` of patch `p` and the halos it feeds, the links counted from `base`.
    fn set_mirrored(&mut self, base: usize, p: usize, b: usize, r: i32, c: i32, v: Pf) {
        self.patches[p].buf[b][gi(r, c)] = v;
        let links = self.patches[p].links;
        mirrors(r, c, |slot, dr, dc| {
            if links[slot] == 0xff { return; }
            if let Some(q) = self.patches.get_mut(base + links[slot] as usize) { q.buf[b][gi(dr, dc)] = v; }
        });
    }

    /// `0x3a16b0(patches, n, added, x, y, r, amp)` (the engine's `0x37d5a0`): on every patch of `range`, each cell
    /// within ±r (clamped to the patch) of (x, y) gets `amp` (`amp / √d²` beyond 1). Added: plus the drawn buffer's;
    /// else kept when the newest's is larger. Into the newest buffer and the halos it feeds — of the patches counted
    /// from the first of `range` (the original's base pointer: a one-patch call feeds patch `i + link`).
    pub fn disturb(&mut self, x: f32, y: f32, r: f32, amp: f32, range: std::ops::Range<usize>, added: bool) {
        let (x, y, r, amp) = (Pf::f(x), Pf::f(y), Pf::f(r), Pf::f(amp));
        let (b, rb, base) = (self.cur, self.render, range.start);
        for p in range {
            if p >= self.patches.len() { break; }
            let [px, py, _] = self.patches[p].centre;
            let oy = py + ORIGIN;
            let fy = (y - oy) / CELL;
            let fx = (x - (px + ORIGIN)) / CELL;
            let r0 = (fy - r).to_i32().max(0);
            let r1 = (fy + r).to_i32().min(15);
            let c1 = (fx + r).to_i32().min(15);
            let c0 = (fx - r).to_i32().max(0);
            let mut yc = oy + CELL * Pf::from_i32(r0);
            for row in r0..=r1 {
                let mut xc = (px + ORIGIN) + CELL * Pf::from_i32(c0);
                for col in c0..=c1 {
                    let dy = y - yc;
                    let dx = x - xc;
                    let d2 = dx * dx + dy * dy;
                    let mut v = if d2 < ONE { amp } else { amp / d2.sqrt() };
                    let h = self.patches[p].buf[b][gi(row, col)];
                    if added {
                        v += self.patches[p].buf[rb][gi(row, col)];
                    } else if v.abs() < h.abs() {
                        v = h;
                    }
                    self.set_mirrored(base, p, b, row, col, v);
                    xc += CELL;
                }
                yc += CELL;
            }
        }
    }

    /// `0x3a12e0` (module docs).
    fn step(&mut self) {
        let (cb, pb) = (self.cur, self.prev);
        let quarter = Pf::b(0x3e80_0000);
        for p in 0..self.patches.len() {
            if self.patches[p].mask == 0 { continue; }
            let k = Pf::b(if self.patches[p].frozen { 0x3eb3_3333 } else { 0x3bcc_cccd });
            let frozen = self.patches[p].frozen;
            let spr = self.patches[p].buf[cb];
            for r in 0..16 {
                for c in 0..16 {
                    let h = |dr: i32, dc: i32| spr[gi(r + dr, c + dc)];
                    let mut acc = h(-1, -1) * quarter;
                    for (dr, dc) in [(-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] { acc += h(dr, dc) * quarter; }
                    let old = self.patches[p].buf[pb][gi(r, c)];
                    let n = acc - old * ONE;
                    let small = ((n.0 & 0x7fff_ffff) as i32).wrapping_sub(DAMP as i32) < 0;
                    self.patches[p].buf[pb][gi(r, c)] = if !frozen && small { n } else { n - n * k };
                }
            }
            let links = self.patches[p].links;
            let src = self.patches[p].buf[pb];
            for (slot, &l) in links.iter().enumerate() {
                let Some(q) = (l != 0xff).then_some(l as usize).filter(|&q| q < self.patches.len()) else { continue };
                let dst = &mut self.patches[q].buf[pb];
                for r in 0..16 {
                    for c in 0..16 {
                        mirrors(r, c, |s, dr, dc| if s == slot { dst[gi(dr, dc)] = src[gi(r, c)]; });
                    }
                }
            }
        }
        std::mem::swap(&mut self.cur, &mut self.prev);
    }

    /// `0x3a13f8` (module docs).
    pub fn clock(&mut self) {
        if ONE - STEP <= self.clock {
            self.clock = STEP;
            self.step();
            self.render = self.prev;
        } else {
            self.render = 2;
            let f = self.clock;
            let w = ONE - f;
            let (pb, cb) = (self.prev, self.cur);
            for p in self.patches.iter_mut().filter(|p| p.mask != 0) {
                for i in 0..G * G { p.buf[2][i] = p.buf[pb][i] * w + p.buf[cb][i] * f; }
            }
        }
        self.clock += STEP;
    }

    /// `0x3a10c8` (L00 `0x37d3b0`): the first patch whose bounds hold the point (z within [z − 1, z + 1)) and whose
    /// mask has the point's sub-block.
    fn find(&self, x: Pf, y: Pf, z: Pf) -> Option<usize> {
        let k = Pf::b(0x4480_0000);
        let (xi, yi, zi) = ((x * k).to_i32(), (y * k).to_i32(), (z * k).to_i32());
        self.patches.iter().position(|p| {
            let [bx, by, bz] = p.bounds;
            if !(bx <= xi && by <= yi && bz <= zi && xi < bx + p.extent && yi < by + p.extent && zi < bz + 0x800) { return false; }
            let col = ((x - (p.centre[0] + ORIGIN)) / CELL).to_i32();
            let row = ((y - (p.centre[1] + ORIGIN)) / CELL).to_i32();
            p.mask as u32 & 1 << (((col >> 2) & 3) as u32 | (row as u32 & 0xc)) != 0
        })
    }

    /// `0x3a1d20` (L00 `0x37dc10`): the drawn buffer's height at the point's (x, y), bilinear, plus the patch's
    /// height; None outside every drawn patch (or with the unit off).
    pub fn height(&self, p: V3) -> Option<f32> {
        let (x, y) = (Pf::f(p[0]), Pf::f(p[1]));
        let i = self.find(x, y, Pf::f(p[2]))?;
        let pa = &self.patches[i];
        let dx = x - (pa.centre[0] + ORIGIN);
        let dy = y - (pa.centre[1] + ORIGIN);
        let col = (dx / CELL).to_i32();
        let row = (dy / CELL).to_i32();
        let fx = (dx - Pf::from_i32(col) * CELL) / CELL;
        let fy = (dy - Pf::from_i32(row) * CELL) / CELL;
        let h = &pa.buf[self.render];
        let (h00, h01, h10, h11) = (h[gi(row, col)], h[gi(row, col + 1)], h[gi(row + 1, col)], h[gi(row + 1, col + 1)]);
        let a = h00 + (h01 - h00) * fx;
        let b = h10 + (h11 - h10) * fx;
        Some((a + (b - a) * fy + pa.centre[2]).to_f32())
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// The moby.

const HIGH: usize = 0x00;
const LOW: usize = 0x04;
const PHASE: usize = 0x08;
const RATE: usize = 0x0c;
const FIRST: usize = 0x10;
const FROZEN: usize = 0x14;
const TURN_AT: usize = 0x15;
const TIMER: usize = 0x16;
const GROUP: usize = 0x18;
const HERO_CUBOID: usize = 0x20;
/// Notak's: the camera's cuboid, the eased height and its speed, the loop's slot.
const N_CUBOID: usize = 0x28;
const N_TARGET: usize = 0x2c;
const N_SPEED: usize = 0x30;
const N_SLOT: usize = 0x38;
/// Damosel's area, Yeedil's hooked mobys.
const D_AREA: usize = 0x28;
const Y_HOOKS: usize = 0x30;

/// The moby's records `first..end` (`+0x10`, `+0x18`'s group).
fn records(m: &crate::moby::Moby, t: &Tables) -> (i32, i32) {
    let first = m.pv_i(FIRST);
    let g = m.pv_i(GROUP);
    let n = if g == -1 { 1 } else { usize::try_from(g).ok().and_then(|g| t.groups.get(g)).copied().unwrap_or(0) };
    (first, n + first)
}

/// Its collision faces: water (0) thawed, ice (7) frozen (`0x3a28a8` / `0x3a2968`).
fn faces(m: &mut crate::moby::Moby) { m.coll_type = Some(if m.pv_b(FROZEN) != 0 { 7 } else { 0 }); }

/// One drop on each of its records `first..end` at `randf(±4)` about the centre (radius 1, −0.05, added).
fn drops(w: &mut MobyWorld, ctx: &mut Ctx, first: i32, end: i32) {
    for i in first.max(0)..end {
        let i = i as usize;
        let Some(c) = w.liquid.patches.get(i).map(|p| p.centre) else { break };
        let x = (c[0] + Pf::f(ctx.game.rng.randf_exact(-4.0, 4.0))).to_f32();
        let y = (c[1] + Pf::f(ctx.game.rng.randf_exact(-4.0, 4.0))).to_f32();
        w.liquid.disturb(x, y, 1.0, f32::from_bits(0xbd4c_cccd), i..i + 1, true);
    }
}

/// The grid classes' update (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    let lv: &crate::level::LevelData = ctx.level;
    let Some(t) = lv.liquid.as_ref() else { return };
    if !SITES.iter().any(|c| c.level == level) { return; }
    let steering = ctx.hero.vehicle.moby.and_then(|v| w.mobys.get(v)).is_some_and(|v| v.o_class == 0x45 && v.state != 0xfe && v.state != 0xfd);
    if steering && ctx.hero.state == 0x31 { return; }
    if w.mobys[id].pvars.is_empty() { return; }
    let cam = ctx.host.camera_pos();
    let (first, end) = records(&w.mobys[id], t);
    let m = &w.mobys[id];
    if level == 18 && first != 0 && m.state != 0 && m.pv_i(D_AREA) != -1 && !ctx.level.in_area(m.pv_i(D_AREA), cam) { return; }
    if level == 6 { notak_loop(w, id, ctx); }
    let state = w.mobys[id].state;
    if state == 0 {
        start(w, id, ctx, t);
        if level == 6 && w.mobys[id].pv_i(N_CUBOID) != -1 { w.mobys[id].update_dist = 0xff; }
    }
    match w.mobys[id].state {
        1 => run(w, id, ctx, first, end),
        2 => {
            let m = &mut w.mobys[id];
            let mut timer = m.pv_s(TIMER);
            // `0x30f718`: at 0 already, or down to 0.
            let done = timer == 0 || {
                timer -= 1;
                m.set_pv_s(TIMER, timer);
                timer <= 0
            };
            if done { m.state = 1; }
            if level == 20 {
                let frozen = w.mobys[id].pv_b(FROZEN) != 0;
                for k in 0..4 {
                    let Some(h) = usize::try_from(w.mobys[id].pv_i(Y_HOOKS + 4 * k)).ok().filter(|&h| h < w.mobys.len()) else { continue };
                    if frozen { w.mobys[h].once |= 4 } else { w.mobys[h].once &= !4 }
                }
            }
            if level == 6 {
                let m = &mut w.mobys[id];
                let target = m.pv_f(N_TARGET);
                if (m.pos[2] - target).abs() <= 2.0 && m.pv_i(GROUP) != -1 {
                    let (mut z, mut v) = (m.pos[2], m.pv_f(N_SPEED));
                    crate::travel::approach(target, f32::from_bits(0x3ada_740e), f32::from_bits(0x3dcc_ccce), &mut z, &mut v);
                    m.pos[2] = z;
                    m.set_pv_f(N_SPEED, v);
                }
            }
        }
        _ => {}
    }
    every(w, id, ctx, t, first, end);
    if level == 6 { notak_hide(w, id, ctx, cam, first, end); }
}

/// State 0 (`0x3a2e68`).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: &Tables) {
    let o_class = w.mobys[id].o_class;
    if let Some(c) = w.classes.iter_mut().find(|c| c.o_class == o_class) { c.kind = 0x12; }
    let m = &mut w.mobys[id];
    m.timer = 0;
    m.mode |= 1;
    m.set_pv_b(TURN_AT, 0x19);
    m.set_pv_f(PHASE, m.pv_f(PHASE) * f32::from_bits(0x3c8e_fa36));
    m.set_pv_f(RATE, m.pv_f(RATE) * f32::from_bits(0x3998_825c));
    faces(m);
    if m.pv_i(FIRST) == 0 {
        m.update_dist = 0xff;
        let thawed = m.pv_b(FROZEN) == 0;
        w.liquid.init(t);
        if thawed {
            let n = w.liquid.patches.len();
            for i in 0..n {
                for _ in 0..2 {
                    let c = w.liquid.patches[i].centre;
                    let x = (c[0] + Pf::f(ctx.game.rng.randf_exact(-6.0, 6.0))).to_f32();
                    let y = (c[1] + Pf::f(ctx.game.rng.randf_exact(-6.0, 6.0))).to_f32();
                    w.liquid.disturb(x, y, 2.0, f32::from_bits(0x3e4c_cccd), 0..n, false);
                }
            }
        }
    }
    let m = &mut w.mobys[id];
    m.draw_dist = 0;
    m.state = 1;
}

/// State 1 (`0x3a3088`).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, first: i32, end: i32) {
    let m = &mut w.mobys[id];
    if m.pv_b(TURN_AT) <= m.timer {
        m.state = 2;
        m.timer = 0;
        m.set_pv_s(TIMER, 60);
        m.set_pv_b(FROZEN, m.pv_b(FROZEN) ^ 1);
        faces(m);
        if first >= 0 && m.pv_b(FROZEN) == 0 { drops(w, ctx, first, end); }
    }
    let m = &mut w.mobys[id];
    if m.pv_b(FROZEN) != 0 { return; }
    if m.pv_i(HERO_CUBOID) != -1 && ctx.level.in_cuboid(m.pv_i(HERO_CUBOID), ctx.hero.pos) { ctx.hero.surf.water_z = m.pos[2]; }
    let h = (m.pv_f(HIGH) - m.pv_f(LOW)) * 0.5;
    m.pos[2] = m.pv_f(LOW) + (m.pv_f(PHASE).cos() * h + h);
    if !matches!(ctx.hero.group, 20 | 25) && !matches!(ctx.hero.state, 0x39 | 0x78 | 0x79) {
        m.set_pv_f(PHASE, crate::hero::physics::add_rot(m.pv_f(PHASE), m.pv_f(RATE)));
    }
    if first >= 0 && ctx.game.rng.randi(200) == 0 { drops(w, ctx, first, end); }
}

/// Every tick (`0x3a33d0`).
fn every(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: &Tables, first: i32, end: i32) {
    let m = &w.mobys[id];
    if first < 0 || m.pv_i(GROUP) == -1 { return; }
    let (z, timer, frozen) = (m.pos[2], m.pv_s(TIMER), m.pv_b(FROZEN) != 0);
    let cam = ctx.host.camera_pos();
    for i in first as usize..end.max(0) as usize {
        let Some(p) = w.liquid.patches.get_mut(i) else { break };
        p.centre[2] = Pf::f(z);
        let c = [p.centre[0].to_f32() + 8.0, p.centre[1].to_f32() + 8.0, z];
        let sphere = [c[0], c[1], c[2], f32::from_bits(0x41b4_cccd)];
        let seen = !ctx.view.is_some_and(|v| v.culled(71.5, sphere)) && crate::hero::physics::len(crate::hero::physics::sub(c, cam)) <= 71.5;
        p.blend = 1.0 - timer as f32 / 60.0;
        p.mask = if seen { t.masks.get(i).copied().unwrap_or(0) } else { 0 };
        p.frozen = frozen;
        if i == 0 {
            w.liquid.clock();
            if ctx.game.underwater { w.register_draw(id) } else { w.register_draw_first(id) }
        }
    }
}

/// Notak's loop (`0x4581c8`'s first part): the level sound 20 while thawed and grouped.
fn notak_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let slot = m.pv_i(N_SLOT);
    if m.pv_b(FROZEN) == 0 && m.pv_i(GROUP) != -1 {
        if !ctx.host.sound_playing(slot) {
            let s = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 20, 4);
            w.mobys[id].set_pv_i(N_SLOT, s);
        }
    } else {
        if slot != -1 && ctx.host.sound_playing(slot) { ctx.host.stop_sound(slot); }
        w.mobys[id].set_pv_i(N_SLOT, -1);
    }
}

/// Notak's last part: with cuboid `+0x28` not holding the camera (nor, under water, its x, y at the high height), its
/// records not drawn.
fn notak_hide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, cam: V3, first: i32, end: i32) {
    let m = &w.mobys[id];
    let c = m.pv_i(N_CUBOID);
    if c == -1 || ctx.level.in_cuboid(c, cam) { return; }
    if ctx.game.underwater && ctx.level.in_cuboid(c, [cam[0], cam[1], m.pv_f(HIGH)]) { return; }
    for i in first.max(0)..end {
        if let Some(p) = w.liquid.patches.get_mut(i as usize) { p.mask = 0; }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// The draw.

/// `0x3a35e8(records, n)` (module docs), from the moby of record 0's registration.
pub fn draw(w: &mut MobyWorld, view: &crate::particles::BSphereView, cam: V3, out: &mut Vec<FxQuad>) {
    let g = &mut w.liquid;
    let (light, render) = (g.light, g.render);
    for p in g.patches.iter_mut() {
        if p.mask == 0 { continue; }
        let x = if p.frozen { p.blend } else { 1.0 - p.blend };
        let b = 1.0 - (1.0 - x) * (1.0 - x);
        let fade = 1.0 - b;
        let speed = Pf::f(fade * f32::from_bits(SPEED));
        p.speed = [speed; 2];
        p.theta = p.theta.map(|a| a * fade);
        p.dtheta = TURN.map(|d| fade * f32::from_bits(d));
        let colour = crate::hud::lerp_rgba(b, WATER, ICE);
        let [cx, cy, cz] = p.centre.map(Pf::to_f32);
        if view.culled(1.0e6, [cx, cy, cz, 11.32]) { continue; }
        // `0x3a14d8`: the scroll and the wobble turned.
        for k in 0..2 {
            let mut s = p.scroll[k] + p.speed[k];
            if ONE < s { s -= ONE; }
            if s < Pf::ZERO { s += ONE; }
            p.scroll[k] = s;
            p.theta[k] = crate::hero::physics::add_rot(p.theta[k], p.dtheta[k]);
        }
        let (su, sv) = (p.scroll[0].to_f32(), p.scroll[1].to_f32());
        let wobble = p.theta.map(|a| [su + a.cos() * p.radius, sv + a.sin() * p.radius]);
        // `0x3305b0` / `0x3a2030`: the 17 × 17 vertices.
        let h = &p.buf[render];
        let hf = |r: i32, c: i32| h[gi(r, c)].to_f32();
        let (x0, y0) = (cx - 8.0, cy - 8.0);
        let mut pos = [[0.0f32; 3]; 17 * 17];
        let mut rgb = [0u32; 17 * 17];
        let mut env = [[0.0f32; 2]; 17 * 17];
        for r in 0..17i32 {
            for c in 0..17i32 {
                let hc = hf(r, c);
                let (hd, hr, hu, hl) = (hf(r + 1, c), hf(r, c + 1), hf(r - 1, c), hf(r, c - 1));
                let e10 = [0.0, -1.0, hc - hd];
                let e11 = [-1.0, 1.0, hd - hr];
                let e12 = [0.0, 1.0, hc - hu];
                let e13 = [1.0, -1.0, hu - hl];
                use crate::hero::physics::{add, cross, dot, scale, set_len, sub};
                let n = scale(set_len(add(cross(e11, e10), cross(e13, e12)), 1.0), -1.0);
                let v = [x0 + c as f32, y0 + r as f32, hc + cz];
                let e = set_len(sub(cam, v), 1.0);
                let t = ((dot(light, n) - 0.573) * 3.25 + 0.573 + 0.5).clamp(0.0, 1.0);
                let byte = |k: u32| (((colour >> (8 * k)) & 0xff) as f32 * t) as u32 & 0xff;
                let v5 = scale(e, -1.0);
                let d = dot(v5, n);
                let refl = if d < 0.0 { e } else { add(v5, scale(sub(scale(n, d), v5), 2.0)) };
                let k = (r * 17 + c) as usize;
                pos[k] = v;
                rgb[k] = byte(0) | byte(1) << 8 | byte(2) << 16;
                env[k] = [refl[0] * REFL * 0.5 + 0.5, refl[1] * REFL * 0.5 + 0.5];
            }
        }
        let fix1 = (p.fix[1] as f32 + (FIX_ICE as f32 - p.fix[1] as f32) * b) as i32 as u32 & 0xff;
        let twice = b + b;
        let (added, fix2) = if p.frozen {
            if twice <= 1.0 { (false, (1.0 - twice) * 70.0) } else { (true, (twice - 1.0) * 170.5) }
        } else if 1.0 <= twice {
            (true, (twice - 1.0) * 170.0)
        } else {
            (false, (1.0 - twice) * 70.0)
        };
        let fix2 = fix2 as i32 as u32 & 0xff;
        // Each drawn sub-block's two passes (`0x330b84`): its 4 × 4 cells, GS strip order (r, c), (r + 1, c), (r, c + 1),
        // (r + 1, c + 1).
        for s in (0..16).filter(|s| p.mask & 1 << s != 0) {
            let (r0, c0) = ((s >> 2) * 4, (s & 3) * 4);
            let corners = |r: i32, c: i32| [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)].map(|(r, c)| (r as usize, c as usize));
            for r in r0..r0 + 4 {
                for c in c0..c0 + 4 {
                    let v = corners(r, c);
                    let k = v.map(|(r, c)| r * 17 + c);
                    let uv = v.map(|(r, c)| {
                        let (lr, lc) = ((r as i32 - r0) as f32, (c as i32 - c0) as f32);
                        let o = wobble[(r + c) & 1];
                        [lc * 0.25 + o[0], lr * 0.25 + o[1]]
                    });
                    out.push(FxQuad { fx: p.tex[1].max(0) as usize, additive: false, corners: k.map(|k| pos[k]), st: uv, rgba: k.map(|k| rgb[k] | fix1 << 24) });
                }
            }
            for r in r0..r0 + 4 {
                for c in c0..c0 + 4 {
                    let k = corners(r, c).map(|(r, c)| r * 17 + c);
                    out.push(FxQuad { fx: p.tex[0].max(0) as usize, additive: added, corners: k.map(|k| pos[k]), st: k.map(|k| env[k]), rgba: k.map(|k| rgb[k] | fix2 << 24) });
                }
            }
        }
    }
}
