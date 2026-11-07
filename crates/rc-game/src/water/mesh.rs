//! Todano's mesh water (L12; the decomp's `func_L12_00420B78` / `func_L12_00420950` and `func_L12_00389BC0` /
//! `func_L12_00389988`): a mesh of the level's own (its vertices, quads and UVs in the overlay's data) waved in
//! place, sent as two layers of FX quads, and the water plane (crate::water::plane) set about Ratchet each tick.
//! The same code twice, on two meshes:
//!
//! | Class | Update / draw | Vertices | Quads | Waves |
//! |---|---|---|---|---|
//! | **0x1113** (1 placed) | `0x421fe0` / `0x421db8` | 294 at `0x2a11b0` | 223 at `0x2a2410` (UVs `0x2a38f8`) | eight bands of 21 (vertices 0..167): band `k` by the phase `k & 3`, × 0.001 |
//! | **0x1350** (1 placed) | `0x38ae28` / `0x38abf0` | 4 at `gp−0x200` | 1 at `gp−0x1c0` (UVs `gp−0x1a8`) | vertex `k` by the phase `k`, × 0.01 / 0.025 / 0.01 / 0.025 |
//!
//! Its pvars: `+0x00..+0x0c` the four phases, `+0x10` the texture's scroll, `+0x14` its speed, `+0x18` (0x1113) the
//! area that hides it, `+0x20..+0x2c` four more phases (counted, never read).
//! * **0**: the phases 0, 0.25, 0.5, 0.75 (and 1, 1.25, 1.5, 1.75), the update distance 0xff, 1; the vertices
//!   × 0.01 when the first one's x is above 1000 (the data's are in hundredths: so once a load).
//! * **1**: (0x1113: nothing with Ratchet in its area `+0x18`, `0x32eb50` = L00 `0x32a848`) its draw on the first
//!   list (`0x2eac38` = L00 `0x2e7718`); the plane on, at Ratchet's place (`0x189ea0`), radius 48, its height
//!   `59.5 + 0.25·sin((frame % 360)·0.0174444 − 3.14)` (the level frame `0x1b2948`; `0x2f2b48` = L00 `0x2ef460`,
//!   the VU0 sine); the scroll on by its speed (8 and over: less 8); each wave's vertices' z plus `sin(phase·3.14159)`
//!   × its size; every phase plus 0.01 (the first four over 2: 0); the speed `randf` of the level's pair (0.001,
//!   0.002: `gp+0x2f04` / `gp−0xf4`).
//! * **2** (nothing sets it): within 64 of Ratchet its draw on the second list (`0x2eae40` = L00 `0x2e7920`).
//!
//! **The draw** ([`draw`]): every quad (its corners' vertices and UVs, the scroll added to u) twice: FX `fx` (`gp+0x2f00` /
//! `gp−0xf8`: 0x14) in the first colour (0x20404010) with ALPHA row 2 of `0x2635e0` (`Cs·FIX`, FIX 0x80: the
//! colour as it is), then FX `fx + 1` in the second (0x20808080) with row 0 (`(Cs − Cd)·As + Cd`). Here the first
//! layer is the normal blend at alpha 0x80 (the texture's alpha still masks it).
//!
//! Not ported: Ratchet's no-dive word `0x18a01e` (5 here; crate::hero::water leaves the dive always allowed).

use crate::fx::FxQuad;
use crate::hero::physics::{dist, V3};
use crate::moby::{Ctx, MobyWorld};

/// The two classes.
pub const LARGE: i16 = 0x1113;
pub const SMALL: i16 = 0x1350;
pub const CLASSES: [i16; 2] = [LARGE, SMALL];

const SCROLL: usize = 0x10;
const SPEED: usize = 0x14;
const AREA: usize = 0x18;
/// `0x40490fd0`: the waves' π.
const PI_W: u32 = 0x4049_0fd0;

/// How a mesh waves (module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Waves {
    /// `n` bands of `size` vertices, band `k` by the phase `k & 3`, × `amp`.
    Bands { n: usize, size: usize, amp: u32 },
    /// Vertex `k` by the phase `k`, × `amp[k]`.
    Corners { amp: [u32; 4] },
}

/// Where a mesh's data sits on L12 and how it moves.
struct Site {
    class: i16,
    verts: u32,
    n_verts: usize,
    quads: u32,
    n_quads: usize,
    uvs: u32,
    /// The words of the FX index, the two colours and the speed's range.
    fx: u32,
    rgba: [u32; 2],
    speed: [u32; 2],
    waves: Waves,
    /// The area that hides it (`+0x18`).
    area: bool,
}

const SITES: [Site; 2] = [
    Site { class: LARGE, verts: 0x2a11b0, n_verts: 294, quads: 0x2a2410, n_quads: 223, uvs: 0x2a38f8, fx: 0x1b1ef0, rgba: [0x1b1ee4, 0x1b1ee8], speed: [0x1b1ef4, 0x1b1ef8], waves: Waves::Bands { n: 8, size: 21, amp: 0x3a83_126f }, area: true },
    Site { class: SMALL, verts: 0x1aedf0, n_verts: 4, quads: 0x1aee30, n_quads: 1, uvs: 0x1aee48, fx: 0x1aeef8, rgba: [0x1aede4, 0x1aede8], speed: [0x1aeefc, 0x1aef00], waves: Waves::Corners { amp: [0x3c23_d70a, 0x3ccc_cccd, 0x3c23_d70a, 0x3ccc_cccd] }, area: false },
];

/// The level this water is on.
pub const LEVEL: i32 = 12;

/// One mesh: its vertices, its quads' corners (vertex, UV), its UVs, its FX, colours and the speed's range.
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub class: i16,
    pub verts: Vec<V3>,
    pub quads: Vec<[(u16, u16); 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub fx: i32,
    pub rgba: [u32; 2],
    pub speed: [f32; 2],
}

/// The level's meshes as its overlay has them (crate::level::LevelData::mesh).
#[derive(Clone, Debug, Default)]
pub struct Tables {
    pub meshes: Vec<Mesh>,
}

/// L12's meshes from its overlay (`read(vaddr, len)`); None elsewhere or when the data is not there.
pub fn read_tables(level: i32, read: &dyn Fn(u32, usize) -> Option<Vec<u8>>) -> Option<Tables> {
    if level != LEVEL { return None; }
    let word = |a: u32| read(a, 4).map(|b| u32::from_le_bytes(b[..4].try_into().unwrap()));
    let meshes = SITES
        .iter()
        .map(|s| {
            let v = read(s.verts, 16 * s.n_verts)?;
            let f = |b: &[u8], o: usize| f32::from_le_bytes(b[o..o + 4].try_into().unwrap());
            let verts = (0..s.n_verts).map(|i| [f(&v, 16 * i), f(&v, 16 * i + 4), f(&v, 16 * i + 8)]).collect();
            let q = read(s.quads, 0x18 * s.n_quads)?;
            let h = |o: usize| u16::from_le_bytes([q[o], q[o + 1]]);
            let quads: Vec<[(u16, u16); 4]> = (0..s.n_quads).map(|i| std::array::from_fn(|c| (h(0x18 * i + 6 * c), h(0x18 * i + 6 * c + 2)))).collect();
            let n_uv = quads.iter().flatten().map(|&(_, u)| u as usize + 1).max().unwrap_or(0);
            let u = read(s.uvs, 8 * n_uv)?;
            let uvs = (0..n_uv).map(|i| [f(&u, 8 * i), f(&u, 8 * i + 4)]).collect();
            Some(Mesh {
                class: s.class,
                verts,
                quads,
                uvs,
                fx: word(s.fx)? as i32,
                rgba: [word(s.rgba[0])?, word(s.rgba[1])?],
                speed: [f32::from_bits(word(s.speed[0])?), f32::from_bits(word(s.speed[1])?)],
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(Tables { meshes })
}

/// The meshes at run time (the game waves its tables in place): copied from the level's at a mover's state 0.
#[derive(Clone, Debug, Default)]
pub struct Meshes {
    pub meshes: Vec<Mesh>,
}

impl Meshes {
    fn get(&self, class: i16) -> Option<&Mesh> { self.meshes.iter().find(|m| m.class == class) }
    fn get_mut(&mut self, class: i16) -> Option<&mut Mesh> { self.meshes.iter_mut().find(|m| m.class == class) }
}

/// The classes' updates (one each: two copies of the code).
pub fn update_for(o_class: i16) -> Option<crate::moby::UpdateFn> {
    match o_class {
        LARGE => Some(update_1113),
        SMALL => Some(update_1350),
        _ => None,
    }
}

/// `0x421fe0` (module docs).
pub fn update_1113(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { run(w, id, ctx, &SITES[0]); }

/// `0x38ae28` (module docs).
pub fn update_1350(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { run(w, id, ctx, &SITES[1]); }

fn sin(a: f32) -> f32 { f32::from_bits(rc_formats::moby_light::vu0_sin_cos(a.to_bits()).0) }

fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: &Site) {
    if w.mobys[id].pvars.len() < 0x30 { w.mobys[id].pvars.resize(0x30, 0); }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            for (k, v) in [0.0, 0.25, 0.5, 0.75].into_iter().enumerate() { m.set_pv_f(4 * k, v); }
            for (k, v) in [1.0, 1.25, 1.5, 1.75].into_iter().enumerate() { m.set_pv_f(0x20 + 4 * k, v); }
            m.state = 1;
            m.update_dist = 0xff;
            if w.water_mesh.get(s.class).is_none() {
                let Some(t) = ctx.level.mesh.as_ref().and_then(|t| t.meshes.iter().find(|m| m.class == s.class)) else { return };
                w.water_mesh.meshes.push(t.clone());
            }
            let Some(mesh) = w.water_mesh.get_mut(s.class) else { return };
            if mesh.verts.first().is_some_and(|v| 1000.0 < v[0]) {
                let k = f32::from_bits(0x3c23_d70a);
                for v in &mut mesh.verts { *v = v.map(|x| x * k); }
            }
        }
        1 => {
            if s.area && ctx.level.in_area(w.mobys[id].pv_i(AREA), ctx.hero.pos) { return; }
            w.register_draw_first(id);
            let angle = (ctx.game.frame % 360) as f32 * f32::from_bits(0x3c8e_e7a7) - f32::from_bits(0x4048_f5c3);
            let p = ctx.hero.pos;
            w.liquid.plane = super::plane::Plane { on: true, at: [p[0], p[1], sin(angle) * 0.25 + 59.5], radius: 48.0 };
            let m = &mut w.mobys[id];
            let mut scroll = m.pv_f(SCROLL) + m.pv_f(SPEED);
            if 8.0 <= scroll { scroll -= 8.0; }
            m.set_pv_f(SCROLL, scroll);
            let phase: [f32; 4] = std::array::from_fn(|k| m.pv_f(4 * k));
            if let Some(mesh) = w.water_mesh.get_mut(s.class) {
                match s.waves {
                    Waves::Bands { n, size, amp } => {
                        let d: [f32; 4] = std::array::from_fn(|k| sin(phase[k] * f32::from_bits(PI_W)) * f32::from_bits(amp));
                        for band in 0..n {
                            for v in mesh.verts.iter_mut().skip(band * size).take(size) { v[2] += d[band & 3]; }
                        }
                    }
                    Waves::Corners { amp } => {
                        for (k, v) in mesh.verts.iter_mut().take(4).enumerate() { v[2] += sin(phase[k] * f32::from_bits(PI_W)) * f32::from_bits(amp[k]); }
                    }
                }
            }
            let m = &mut w.mobys[id];
            let step = f32::from_bits(0x3c23_d70a);
            for k in 0..4 {
                let v = m.pv_f(4 * k) + step;
                m.set_pv_f(4 * k, if 2.0 < v { 0.0 } else { v });
            }
            if matches!(s.waves, Waves::Bands { .. }) {
                for k in 0..4 { m.set_pv_f(0x20 + 4 * k, m.pv_f(0x20 + 4 * k) + step); }
            }
            let speed = w.water_mesh.get(s.class).map_or([0.0; 2], |m| m.speed);
            let v = ctx.game.rng.randf(speed[0], speed[1]);
            w.mobys[id].set_pv_f(SPEED, v);
        }
        2 => {
            if dist(w.mobys[id].pos, ctx.hero.pos) < 64.0 { w.register_draw(id); }
        }
        _ => {}
    }
}

/// `0x421db8` / `0x38abf0` (module docs).
pub fn draw(w: &MobyWorld, id: usize, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let Some(mesh) = w.water_mesh.get(m.o_class) else { return };
    let scroll = m.pv_f(SCROLL);
    for (fx, rgba) in [(mesh.fx, mesh.rgba[0] & 0x00ff_ffff | 0x80 << 24), (mesh.fx + 1, mesh.rgba[1])] {
        for q in &mesh.quads {
            let corner = |c: usize| mesh.verts.get(q[c].0 as usize).copied().unwrap_or([0.0; 3]);
            let st = |c: usize| mesh.uvs.get(q[c].1 as usize).map_or([0.0; 2], |u| [u[0] + scroll, u[1]]);
            out.push(FxQuad { fx: fx.max(0) as usize, additive: false, corners: std::array::from_fn(corner), st: std::array::from_fn(st), rgba: [rgba; 4] });
        }
    }
}
