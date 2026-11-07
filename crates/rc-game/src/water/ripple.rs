//! Barlow's ripple water (L04; the decomp's `func_L04_00407000` / `func_L04_00406C00` and `func_L04_00407F80` /
//! `func_L04_00407B80`): two triangle strips of the level's own (their vertices and UVs in the overlay's data) about
//! the moby, each vertex lifted by a ring wave on its distance from the moby, drawn twice with the UVs scrolled two
//! ways. The same draw twice, on two meshes (its tables at `$gp+0x1640` and `$gp+0x16e0`, one layout):
//!
//! | Class | Update / draw | Tables | Strips |
//! |---|---|---|---|
//! | **0x0b37** (1 placed) | `0x4084c0` / `0x4080c0` (first list) | `0x1b0630` | 148 and 118 vertices |
//! | **0x0b54** (1 placed) | `0x409440` / `0x409040` (second list) | `0x1b06d0` | 148 and 28 vertices |
//!
//! The tables (`+0x00` the two counts, `+0x48` / `+0x50` the strips' vertices (x, y, z) and UVs, `+0x30..+0x44` the
//! out copies the draw fills, `+0x64` the FX texture 0x46, `+0x68` the alpha 0x30, `+0x6c` the wavelength 2, `+0x70`
//! the period 1.14, `+0x74` the height 0.025, `+0x78` the UV speed 0.1, `+0x80` the two passes' UV rates (1, −π/5)
//! and (1, π/5), `+0x90` the ambient (0x40, 0x80, 0xff); `+0x60` ALPHA's FIX 0x80, unread by its blend).
//!
//! **The updates**: every tick the moby's ambient from the table's three words (`0x334650` = L00 `0x313300`).
//! * 0x0b37: **0** → 1; **1** its draw on the first list (`0x307e98` = L00 `0x2e7718`), mode | 1, not drawn
//!   (`+0x32` = 0).
//! * 0x0b54: **0** its update distance 0xc4, mode | 1, not drawn, 1 (no draw that tick); **1** with no cuboid
//!   (`+0x68`[8] = −1) or the camera (`0x1b96c0`) in it (`0x343448` = L00 `0x321858`), its draw on the second list
//!   (`0x3080a0` = L00 `0x2e7920`); else neither the ambient nor the draw.
//!
//! **The draw** ([`draw`]): the time `frame / 60 · speed` (the level frame `0x1b2008`); the phase (`+0x68`[4]; its
//! last in `[0]`) on by `360 / period · 0.000290888` (`0x310b50` = L00 `0x2efe60`, wrapped); the colour the ambient
//! (`0x334680` = L00 `0x313330`) at the table's alpha. Each vertex at the moby's place plus its own, its z plus
//! `height · sin(wrap(fmod(r, wavelength) · 2π / wavelength) + phase)` (r its distance from the moby's z axis;
//! `0x30f9b0`, `0x310cb8` = L00 `0x2effc8`, `0x310be0` = L00 `0x2efef0`, `0x3100c8` = L00 `0x2ef460`, the VU0 sine).
//! Then twice, for pass p: every UV its own plus `fmod(time · rate[p], 1)` on each axis and both strips drawn
//! (`0x30cd98` = L00 `0x2ec618`, after `0x30b818`): FX 0x46, ALPHA 0x44 (`(Cs − Cd)·As + Cd`).

use crate::fx::FxQuad;
use crate::hero::physics::V3;
use crate::moby::{Ctx, MobyWorld};

pub const OUTER: i16 = 0x0b37;
pub const GATED: i16 = 0x0b54;
pub const CLASSES: [i16; 2] = [OUTER, GATED];

/// The level these are on.
pub const LEVEL: i32 = 4;

/// The classes' tables (`$gp+0x1640`, `$gp+0x16e0`).
const SITES: [(i16, u32); 2] = [(OUTER, 0x1b0630), (GATED, 0x1b06d0)];

/// The cuboid 0x0b54's draw waits for (`+0x68`[8]).
const CUBOID: usize = 8;
const LAST: usize = 0;
const PHASE: usize = 4;

/// One mesh's tables (module docs).
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub class: i16,
    pub verts: [Vec<V3>; 2],
    pub uvs: [Vec<[f32; 2]>; 2],
    pub fx: i32,
    pub alpha: u32,
    pub wavelength: f32,
    pub period: f32,
    pub height: f32,
    pub speed: f32,
    pub rates: [[f32; 2]; 2],
    pub ambient: [u8; 3],
}

/// The level's meshes, read from its overlay at the start (the engine sets [`MobyWorld::water_ripple`]).
#[derive(Clone, Debug, Default)]
pub struct Tables {
    pub meshes: Vec<Mesh>,
}

impl Tables {
    fn get(&self, class: i16) -> Option<&Mesh> { self.meshes.iter().find(|m| m.class == class) }
}

/// L04's meshes from its overlay (`read(vaddr, len)`); None elsewhere or when the data is not there.
pub fn read_tables(level: i32, read: &dyn Fn(u32, usize) -> Option<Vec<u8>>) -> Option<Tables> {
    if level != LEVEL { return None; }
    let meshes = SITES
        .iter()
        .map(|&(class, base)| {
            let t = read(base, 0xa0)?;
            let w = |o: usize| u32::from_le_bytes(t[o..o + 4].try_into().unwrap());
            let f = |o: usize| f32::from_bits(w(o));
            let fl = |b: &[u8], o: usize| f32::from_le_bytes(b[o..o + 4].try_into().unwrap());
            let mut verts: [Vec<V3>; 2] = Default::default();
            let mut uvs: [Vec<[f32; 2]>; 2] = Default::default();
            for g in 0..2 {
                let n = (w(4 * g) as usize).min(0x400);
                let v = read(w(0x48 + 4 * g), 12 * n)?;
                verts[g] = (0..n).map(|i| [fl(&v, 12 * i), fl(&v, 12 * i + 4), fl(&v, 12 * i + 8)]).collect();
                let u = read(w(0x50 + 4 * g), 8 * n)?;
                uvs[g] = (0..n).map(|i| [fl(&u, 8 * i), fl(&u, 8 * i + 4)]).collect();
            }
            Some(Mesh {
                class,
                verts,
                uvs,
                fx: w(0x64) as i32,
                alpha: w(0x68),
                wavelength: f(0x6c),
                period: f(0x70),
                height: f(0x74),
                speed: f(0x78),
                rates: [[f(0x80), f(0x84)], [f(0x88), f(0x8c)]],
                ambient: [w(0x90) as u8, w(0x94) as u8, w(0x98) as u8],
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(Tables { meshes })
}

/// The classes' updates.
pub fn update_for(o_class: i16) -> Option<crate::moby::UpdateFn> {
    match o_class {
        OUTER => Some(update_0b37),
        GATED => Some(update_0b54),
        _ => None,
    }
}

fn ambient(w: &MobyWorld, class: i16) -> [u8; 3] { w.water_ripple.as_ref().and_then(|t| t.get(class)).map_or([0; 3], |m| m.ambient) }

/// `0x4084c0` (module docs).
pub fn update_0b37(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let a = ambient(w, OUTER);
    let m = &mut w.mobys[id];
    m.ambient = a;
    if m.state == 0 {
        m.state = 1;
        return;
    }
    w.register_draw_first(id);
    let m = &mut w.mobys[id];
    m.mode |= 1;
    m.draw_dist = 0;
}

/// `0x409440` (module docs).
pub fn update_0b54(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let a = ambient(w, GATED);
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.ambient = a;
        m.update_dist = 0xc4;
        m.state = 1;
        m.mode |= 1;
        m.draw_dist = 0;
        return;
    }
    let c = m.pv_i(CUBOID);
    if c == -1 || ctx.level.in_cuboid(c, ctx.host.camera_pos()) {
        w.mobys[id].ambient = a;
        w.register_draw(id);
    }
}

fn sin(a: f32) -> f32 { f32::from_bits(rc_formats::moby_light::vu0_sin_cos(a.to_bits()).0) }

/// `v − m·trunc(v / m)` (`0x2effc8`).
fn fmod(v: f32, m: f32) -> f32 { v - m * (v / m).trunc() }

/// `0x4080c0` / `0x409040` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, frame: i32, out: &mut Vec<FxQuad>) {
    let Some(t) = w.water_ripple.clone() else { return };
    let m = &mut w.mobys[id];
    let Some(mesh) = t.get(m.o_class) else { return };
    let time = frame as f32 * f32::from_bits(0x3c88_8889) * mesh.speed;
    let phase = m.pv_f(PHASE);
    m.set_pv_f(LAST, phase);
    let phase = crate::hero::physics::add_rot(phase, 360.0 / mesh.period * f32::from_bits(0x3998_825c));
    m.set_pv_f(PHASE, phase);
    let rgba = mesh.alpha << 24 | (m.ambient[2] as u32) << 16 | (m.ambient[1] as u32) << 8 | m.ambient[0] as u32;
    let p = m.pos;
    let wl = mesh.wavelength;
    let verts: [Vec<V3>; 2] = std::array::from_fn(|g| {
        mesh.verts[g]
            .iter()
            .map(|v| {
                let r = (v[0] * v[0] + v[1] * v[1]).sqrt();
                let a = crate::hero::physics::wrap(fmod(r, wl) * f32::from_bits(0x40c9_0fdc) / wl);
                let h = mesh.height * sin(crate::hero::physics::add_rot(a, phase));
                [v[0] + p[0], v[1] + p[1], v[2] + p[2] + h]
            })
            .collect()
    });
    for rate in mesh.rates {
        let d = [fmod(time * rate[0], 1.0), fmod(time * rate[1], 1.0)];
        for g in 0..2 {
            let pts: Vec<(V3, [f32; 2], u32)> = verts[g].iter().zip(&mesh.uvs[g]).map(|(&v, u)| (v, [u[0] + d[0], u[1] + d[1]], rgba)).collect();
            crate::moby::classes::nanotech::strip(out, mesh.fx.max(0) as usize, false, &pts);
        }
    }
}
