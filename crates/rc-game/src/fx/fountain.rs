//! The quad fountains 0x02f8 (the decomp's `func_L04_003C3BA0` / `func_L04_003C4050`: L04 `0x3c4f80` / `0x3c5430`,
//! L19 `0x39d190` / `0x39d640`, the same code on both levels with their own tables) and their clock 0x0329 (the
//! decomp's `func_L04_003C6FE8`: L04 `0x3c83c8`, L17 `0x3a5b10`, L19 `0x3a05d8`): a cuboid of the level (`+0x68`[4])
//! filled with camera-facing quads that scroll their texture up as they rise and fade in and out, each re-rolled when
//! it has faded, with a pool of drops in it, a cap over it and glow puffs off its top.
//!
//! **The pool** (the level's arrays of 200 slots, `0x23be10..`; L19 `0x23c110..`): per slot its place in the cuboid
//! (x, y, z in −1..1, w 1), height `speed` and width `size` factors, texture scroll `age` and its rate, which of the
//! two camera frames it faces by (`flip`), a drop (`drop`), and the fade-in, fade-out and wait timers (s16). A
//! fountain takes `+0x02` quads and `+0x40` drops from the pool's next (`0x3c84a8`: `0x1b2e84`, the index, moved on),
//! from `+0x00`.
//!
//! **The clock** 0x0329 ([`update_clock`]): **0** the pool's next 0, the wait step (`0x1b2e70`) 60, the fade-in
//! (`0x1b2e74`) 60 and its 1/60 (`0x1b2e78`), the fade-out (`0x1b2e7c`) 60 and 1/60 (`0x1b2e80`), its update distance
//! 0xff, 1. **1**: the caps' scroll (`$gp+0x518`) −0.0025 a tick (−8 and under: + 8). The fountains alone read them.
//!
//! **The fountain** ([`update`]; `+0x68` the pvars below). **0**: 1, its update distance 0xff; no cuboid: gone
//! (`0x3329d8` = L00 `0x311688`). Else its radius `+0x48` the longer of the cuboid's first two rows, the drops' speed
//! `+0x44` that radius with drops (3 without), its half-height `+0x50` half its third row; its slots taken; each quad
//! [`roll`]ed with a random frame and its fade-in 0 (so its wait first), each drop [`roll_drop`]ed with a random frame;
//! its colour `+0x38` from the bytes `+0x3c..+0x3e`. **1**: nothing while a 10-wide sphere 9 over the cuboid's place is
//! out of view at the table's far (`0x310d70` = L00 `0x2f0080`, crate::particles::BSphereView::culled); with puffs on
//! (`+0x42`) and the camera within 128 of it, the table's count of glow puffs (crate::particles::puff: at a random
//! point of the cuboid's top 0.5 over it, drifting ±0.25 a second across and rising `+0x20` a second, size
//! `randf(+0x24, +0x28)`, alpha `+0x2c..+0x2e`, the table's colour, jitter, growth and spins; the life `+0x30..+0x34`,
//! words `+0x24` = 3, `+0x2a` / `+0x2b` the alpha and the life's low byte, and half of them blended, 0x44); then its
//! draw on the second list (`0x3080a0` = L00 `0x2e7920`).
//!
//! **The draw** ([`draw`], `0x3c5430`): two frames at the moby facing the camera (`f = unit(camera − place)`, `r =
//! unit(up × f)`, `u = f × r`, and the same from `−f`), each slot's age on by its rate (−8 and under: + 8) and its
//! frame placed at its point in the cuboid. A drop: the table's quads 2 and 3 of its drop points (y × speed, z ×
//! size), u its UV's second plus the age, v its first, the colour `+0x5c`'s alpha × the corner's byte with the rgb
//! `+0x5d..+0x5f`, FX 0x3f + `+0x54`, blended (0x44). A quad: its fade (a fade-in running: `(1 − t·k)·127`; then a
//! fade-out running: `t·k·127`, re-rolled with a new frame at its end, alpha 0 that frame; else the wait, the fade-out
//! started at its end, 0x7f), the table's four corners (y × speed, z × size), u its UV's first less the age, the
//! colour its alpha × the corner's byte with `+0x38`, additive (0x48). Then with puffs off, the cap: the six quads of
//! the cap points (y × `+0x48`, z × `+0x4c`) at the cuboid's place `+0x50` down, in the first frame, u its UV's second
//! plus the scroll, the corner's byte at 0xff with `+0x5d..`, FX 0x3f + `+0x58`, blended.
//!
//! The pvars: `+0x00` the first slot, `+0x02` the quads, `+0x04` the cuboid, `+0x08..+0x0c` the quads' scroll rate,
//! `+0x10..+0x14` their speed, `+0x18..+0x1c` their size, `+0x20` the puffs' rise, `+0x24..+0x28` their size,
//! `+0x2c..+0x2e` their alpha, `+0x30..+0x34` their life, `+0x38` the quads' colour, `+0x40` the drops, `+0x42`
//! puffs on, `+0x44` the drops' speed base, `+0x4c` their size base and the cap's depth, `+0x54` / `+0x58` the
//! textures, `+0x5c..+0x5f` the drops' and cap's colour.

use crate::fx::FxQuad;
use crate::hero::physics::{cross, dist, scale, sub, unit, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x02f8;
pub const CLOCK: i16 = 0x0329;

/// The pool's slots (`0x23be10..`, 200 of each).
const SLOTS: usize = 200;

/// The levels' tables: the small one at `$gp+0x2f0` and the meshes'.
const SITES: [(i32, u32, u32); 2] = [(4, 0x1af2e0, 0x2ab7e0), (19, 0x1af2d0, 0x295690)];

mod pv {
    pub const FIRST: usize = 0x00;
    pub const QUADS: usize = 0x02;
    pub const CUBOID: usize = 0x04;
    pub const RATE: usize = 0x08;
    pub const SPEED: usize = 0x10;
    pub const SIZE: usize = 0x18;
    pub const RISE: usize = 0x20;
    pub const PUFF_SIZE: usize = 0x24;
    pub const PUFF_ALPHA: usize = 0x2c;
    pub const PUFF_LIFE: usize = 0x30;
    pub const RGB: usize = 0x38;
    pub const RGB_BYTES: usize = 0x3c;
    pub const DROPS: usize = 0x40;
    pub const PUFFS: usize = 0x42;
    pub const DROP_SPEED: usize = 0x44;
    pub const RADIUS: usize = 0x48;
    pub const DEPTH: usize = 0x4c;
    pub const HALF: usize = 0x50;
    pub const FX: usize = 0x54;
    pub const CAP_FX: usize = 0x58;
    pub const ALPHA: usize = 0x5c;
}

/// A level's tables (module docs).
#[derive(Clone, Debug, Default)]
pub struct Tables {
    /// `$gp+0x2f0`: a waiting quad's alpha; `+0x2f4` the fades' scale.
    pub full: i32,
    pub fade_scale: f32,
    /// `$gp+0x300`: a quad's corners; `+0x340` their alpha bytes; `+0x348` their UVs.
    pub corners: [V3; 4],
    pub corner_alpha: [u8; 4],
    pub corner_uv: [[f32; 2]; 4],
    /// `$gp+0x368`: the drop and cap points' alpha bytes.
    pub alphas: [u8; 12],
    /// `$gp+0x374..+0x388`: the puffs' jitter, growth, count, spins and colour; `+0x3a0` the view test's far.
    pub jitter: f32,
    pub grow: f32,
    pub count: i32,
    pub spin: [i32; 2],
    pub rgb: u32,
    pub far: f32,
    /// `$gp+0x518`'s value at the start.
    pub scroll: f32,
    /// The cap points, the drop points, the six quads (point indices), the cap's and the drops' UVs.
    pub cap: [V3; 12],
    pub drop: [V3; 12],
    pub quads: [[usize; 4]; 6],
    pub cap_uv: [[f32; 2]; 12],
    pub drop_uv: [[f32; 2]; 12],
}

/// The tables of a level with the fountain, from its overlay (`read(vaddr, len)`); None elsewhere.
pub fn read_tables(level: i32, read: &dyn Fn(u32, usize) -> Option<Vec<u8>>) -> Option<Tables> {
    let &(_, gp, tb) = SITES.iter().find(|s| s.0 == level)?;
    let g = read(gp, 0x22c)?;
    let t = read(tb, 0x2a0)?;
    let w = |b: &[u8], o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let f = |b: &[u8], o: usize| f32::from_bits(w(b, o));
    let v3 = |b: &[u8], o: usize| [f(b, o), f(b, o + 4), f(b, o + 8)];
    Some(Tables {
        full: w(&g, 0) as i32,
        fade_scale: f(&g, 4),
        corners: std::array::from_fn(|k| v3(&g, 0x10 + 0x10 * k)),
        corner_alpha: std::array::from_fn(|k| g[0x50 + k]),
        corner_uv: std::array::from_fn(|k| [f(&g, 0x58 + 8 * k), f(&g, 0x5c + 8 * k)]),
        alphas: std::array::from_fn(|k| g[0x78 + k]),
        jitter: f(&g, 0x84),
        grow: f(&g, 0x88),
        count: w(&g, 0x8c) as i32,
        spin: [w(&g, 0x90) as i32, w(&g, 0x94) as i32],
        rgb: w(&g, 0x98),
        far: f(&g, 0xb0),
        scroll: f(&g, 0x228),
        cap: std::array::from_fn(|k| v3(&t, 0x10 * k)),
        drop: std::array::from_fn(|k| v3(&t, 0xc0 + 0x10 * k)),
        quads: std::array::from_fn(|j| std::array::from_fn(|k| (w(&t, 0x180 + 0x10 * j + 4 * k) as usize).min(11))),
        cap_uv: std::array::from_fn(|k| [f(&t, 0x1e0 + 8 * k), f(&t, 0x1e4 + 8 * k)]),
        drop_uv: std::array::from_fn(|k| [f(&t, 0x240 + 8 * k), f(&t, 0x244 + 8 * k)]),
    })
}

/// One pool slot (module docs).
#[derive(Clone, Copy, Debug, Default)]
pub struct Slot {
    pub at: [f32; 4],
    pub speed: f32,
    pub size: f32,
    pub age: f32,
    pub rate: f32,
    pub flip: bool,
    pub drop: bool,
    pub fade_in: i16,
    pub fade_out: i16,
    pub wait: i16,
}

/// The fountains' shared state: the tables, the clock's words and the pool.
#[derive(Clone, Debug, Default)]
pub struct Fountains {
    pub tables: Option<std::sync::Arc<Tables>>,
    /// `0x1b2e70` the wait step, `0x1b2e74` / `0x1b2e78` the fade-in and its step, `0x1b2e7c` / `0x1b2e80` the
    /// fade-out and its step, `0x1b2e84` the pool's next, `$gp+0x518` the caps' scroll.
    pub wait: i32,
    pub fade_in: u16,
    pub fade_in_k: f32,
    pub fade_out: u16,
    pub fade_out_k: f32,
    pub next: i32,
    pub scroll: f32,
    pub slots: Vec<Slot>,
    /// Each fountain's cuboid (its matrix rows), as its state 0 found it (the draw reads the level's table).
    pub cuboids: std::collections::HashMap<usize, [[f32; 4]; 4]>,
}

impl Fountains {
    /// The fountains of a level whose tables are `t` (the scroll from its data).
    pub fn new(t: Option<Tables>) -> Fountains {
        Fountains { scroll: t.as_ref().map_or(0.0, |t| t.scroll), tables: t.map(std::sync::Arc::new), slots: vec![Slot::default(); SLOTS], ..Default::default() }
    }
}

/// The classes' updates.
pub fn update_for(o_class: i16) -> Option<crate::moby::UpdateFn> {
    match o_class {
        CLASS => Some(update),
        CLOCK => Some(update_clock),
        _ => None,
    }
}

/// `0x3c83c8` (module docs).
pub fn update_clock(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let f = &mut w.fountain;
    match w.mobys[id].state {
        0 => {
            f.next = 0;
            w.mobys[id].state = 1;
            w.mobys[id].update_dist = 0xff;
            f.wait = 0x3c;
            f.fade_in = 0x3c;
            f.fade_in_k = 1.0 / 60.0;
            f.fade_out = 0x3c;
            f.fade_out_k = f32::from_bits(0x3c88_8889);
        }
        1 => {
            let t = f.scroll - f32::from_bits(0x3b23_d70a);
            f.scroll = if t <= -8.0 { t + 8.0 } else { t };
        }
        _ => {}
    }
}

/// `0x30ffa8` (L00 `0x2ef340`): `v` (with its w) through the four rows.
fn through(v: [f32; 4], m: &[[f32; 4]; 4]) -> V3 { std::array::from_fn(|k| v[0] * m[0][k] + v[1] * m[1][k] + v[2] * m[2][k] + v[3] * m[3][k]) }

fn len4(r: &[f32; 4]) -> f32 { (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt() }

/// `0x3c5c38(m, i)`: slot `i` a quad again: a random point, speed `+0x10..`, size `+0x18..`, age 0, rate
/// `+0x08..`, the fade-in from the clock, no fade-out, not a drop, its wait `(i − first + 1)` wait steps.
fn roll(f: &mut Fountains, m: &crate::moby::Moby, rng: &mut crate::rng::Rng, i: usize) {
    let (wait, fade_in) = (f.wait, f.fade_in);
    let Some(s) = f.slots.get_mut(i) else { return };
    for k in 0..3 { s.at[k] = rng.randf_signed(0.0, 1.0); }
    s.speed = rng.randf(m.pv_f(pv::SPEED), m.pv_f(pv::SPEED + 4));
    s.size = rng.randf(m.pv_f(pv::SIZE), m.pv_f(pv::SIZE + 4));
    s.age = 0.0;
    s.rate = rng.randf(m.pv_f(pv::RATE), m.pv_f(pv::RATE + 4));
    s.fade_in = fade_in as i16;
    s.fade_out = 0;
    s.drop = false;
    s.wait = ((i as i32 - m.pv_s(pv::FIRST) as i32 + 1).wrapping_mul(wait)) as i16;
}

/// `0x3c5da0(m, i)`: slot `i` a drop: a random point, speed `+0x44..+0x44 + 2`, size `+0x4c..+0x4c + 4`, age 0, rate
/// 0.003..0.005.
fn roll_drop(f: &mut Fountains, m: &crate::moby::Moby, rng: &mut crate::rng::Rng, i: usize) {
    let Some(s) = f.slots.get_mut(i) else { return };
    for k in 0..3 { s.at[k] = rng.randf_signed(0.0, 1.0); }
    let (a, b) = (m.pv_f(pv::DROP_SPEED), m.pv_f(pv::DEPTH));
    s.speed = rng.randf(a, a + 2.0);
    s.size = rng.randf(b, b + 4.0);
    s.age = 0.0;
    s.rate = rng.randf(f32::from_bits(0x3b44_9ba6), f32::from_bits(0x3ba3_d70a));
    s.drop = true;
}

/// `0x3c4f80` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.update_dist = 0xff;
            let c = m.pv_i(pv::CUBOID);
            if c == -1 {
                w.delete(id);
                return;
            }
            let Some(cub) = usize::try_from(c).ok().and_then(|c| ctx.level.cuboids.get(c)) else { return };
            let m = &mut w.mobys[id];
            let mut x = cub.matrix;
            x[3][3] = 1.0;
            let radius = len4(&x[1]).max(len4(&x[0]));
            m.set_pv_f(pv::RADIUS, radius);
            m.set_pv_f(pv::DROP_SPEED, if m.pv_s(pv::DROPS) != 0 { radius } else { 3.0 });
            m.set_pv_f(pv::HALF, len4(&x[2]) * 0.5);
            let (quads, drops) = (m.pv_s(pv::QUADS) as i32, m.pv_s(pv::DROPS) as i32);
            let f = &mut w.fountain;
            if f.slots.len() < SLOTS { f.slots.resize(SLOTS, Slot::default()); }
            let first = f.next;
            f.next = first.wrapping_add(quads + drops);
            let m = &mut w.mobys[id];
            m.set_pv_s(pv::FIRST, first as i16);
            let first = first as i16 as i32;
            let rng = &mut ctx.game.rng;
            for i in first..first + quads {
                let flip = rng.randi(2) != 0;
                let Ok(i) = usize::try_from(i) else { continue };
                if let Some(s) = f.slots.get_mut(i) { s.at[3] = 1.0; s.flip = flip; }
                roll(f, m, rng, i);
                if let Some(s) = f.slots.get_mut(i) { s.fade_in = 0; }
            }
            for i in first + quads..first + quads + drops {
                let flip = rng.randi(2) != 0;
                let Ok(i) = usize::try_from(i) else { continue };
                if let Some(s) = f.slots.get_mut(i) { s.at[3] = 1.0; s.flip = flip; }
                roll_drop(f, m, rng, i);
            }
            let rgb = (m.pv_b(pv::RGB_BYTES + 2) as i32) << 16 | (m.pv_b(pv::RGB_BYTES + 1) as i32) << 8 | m.pv_b(pv::RGB_BYTES) as i32;
            m.set_pv_i(pv::RGB, rgb);
            f.cuboids.insert(id, x);
        }
        1 => {
            let Some(x) = w.fountain.cuboids.get(&id).copied() else { return };
            let Some(t) = w.fountain.tables.clone() else { return };
            let p = x[3];
            if ctx.view.is_some_and(|v| v.culled(t.far, [p[0], p[1], p[2] + 9.0, 10.0])) { return; }
            let m = &w.mobys[id];
            if m.pv_s(pv::PUFFS) != 0 && dist(ctx.host.camera_pos(), m.pos) < 128.0 {
                for _ in 0..t.count {
                    let rng = &mut ctx.game.rng;
                    let v = [rng.randf_signed(0.0, 1.0), rng.randf_signed(0.0, 1.0), 1.0, 1.0];
                    let mut at = through(v, &x);
                    at[2] += 0.5;
                    let drift = f32::from_bits(0x3b88_8889);
                    let vel = [rng.randf_signed(0.0, drift), rng.randf_signed(0.0, drift), m.pv_f(pv::RISE) * f32::from_bits(0x3c88_8889)];
                    let size = rng.randf(m.pv_f(pv::PUFF_SIZE), m.pv_f(pv::PUFF_SIZE + 4));
                    let rgba = (rng.rand_range(m.pv_s(pv::PUFF_ALPHA) as i32, m.pv_s(pv::PUFF_ALPHA + 2) as i32) as u32) << 24 | t.rgb;
                    let spin = rng.rand_range(t.spin[0], t.spin[1]);
                    let Some(s) = ctx.parts.as_deref_mut() else { continue };
                    let Some(i) = crate::particles::puff(s, rng, t.jitter, 1.0, t.grow, size, at, spin, vel, rgba, 0) else { continue };
                    let life = rng.rand_range(m.pv_i(pv::PUFF_LIFE), m.pv_i(pv::PUFF_LIFE + 4)) as i16;
                    let blend = rng.randi(2) != 0;
                    let r = &mut s.pool.recs[i];
                    r[0xa..0xc].copy_from_slice(&life.to_le_bytes());
                    r[0x24..0x28].copy_from_slice(&3u32.to_le_bytes());
                    r[0x2a] = (rgba >> 24) as u8;
                    r[0x2b] = r[0xa];
                    if blend { r[3] = 0x44; }
                }
            }
            w.register_draw(id);
        }
        _ => {}
    }
}

/// `0x30f718` (L00 `0x2eeae0`) on an s16: true when it was 0 already or has just run out.
fn countdown(x: &mut i16) -> bool {
    if *x == 0 { return true; }
    *x = x.wrapping_sub(1);
    *x <= 0
}

/// `0x3c5430` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let Some(t) = w.fountain.tables.clone() else { return };
    let Some(x) = w.fountain.cuboids.get(&id).copied() else { return };
    let m = w.mobys[id].clone();
    let alpha = (m.pv_b(pv::ALPHA) as u32) << 24;
    let rgb = (m.pv_b(pv::ALPHA + 3) as u32) << 16 | (m.pv_b(pv::ALPHA + 2) as u32) << 8 | m.pv_b(pv::ALPHA + 1) as u32;
    let first = m.pv_s(pv::FIRST) as i32;
    let end = first + m.pv_s(pv::QUADS) as i32 + m.pv_s(pv::DROPS) as i32;
    let up = [0.0, 0.0, 1.0];
    // `0x30fbf0(out, a, b)` (L00 `0x2eef88`) is b × a.
    let frame = |f: V3| -> [[f32; 4]; 4] {
        let r = unit(cross(up, f));
        let u = cross(f, r);
        let row = |v: V3| [v[0], v[1], v[2], 0.0];
        [row(f), row(r), row(u), [m.pos[0], m.pos[1], m.pos[2], 1.0]]
    };
    let f0 = unit(sub(cam, m.pos));
    let mut frames = [frame(f0), frame(scale(f0, -1.0))];
    let (fx, cap_fx) = ((0x3f + m.pv_i(pv::FX)).max(0) as usize, (0x3f + m.pv_i(pv::CAP_FX)).max(0) as usize);
    let quad_rgb = m.pv_i(pv::RGB) as u32;
    for i in first..end {
        let Ok(i) = usize::try_from(i) else { continue };
        let Some(&s0) = w.fountain.slots.get(i) else { continue };
        let mut s = s0;
        let age = s.age - s.rate;
        s.age = if age <= -8.0 { age + 8.0 } else { age };
        let k_ = s.flip as usize;
        let at = through(s.at, &x);
        frames[k_][3] = [at[0], at[1], at[2], 1.0];
        let rows = frames[k_];
        // The speed and size as they were (a quad re-rolled below keeps them for this frame).
        let (speed, size) = (s.speed, s.size);
        let corner = |p: V3| through([p[0], p[1] * speed, p[2] * size, 1.0], &rows);
        if s.drop {
            for q in &t.quads[2..4] {
                out.push(FxQuad {
                    fx,
                    additive: false,
                    corners: q.map(|n| corner(t.drop[n])),
                    st: q.map(|n| [t.drop_uv[n][1] + s.age, t.drop_uv[n][0]]),
                    rgba: q.map(|n| alpha.wrapping_mul(t.alphas[n] as u32) | rgb),
                });
            }
        } else {
            let opacity = if s.fade_in != 0 {
                countdown(&mut s.fade_in);
                ((1.0 - s.fade_in as f32 * w.fountain.fade_in_k) * t.fade_scale) as i32
            } else if s.fade_out != 0 {
                if countdown(&mut s.fade_out) {
                    w.fountain.slots[i] = s;
                    roll(&mut w.fountain, &m, rng, i);
                    s = w.fountain.slots[i];
                    s.flip = rng.randi(2) != 0;
                    0
                } else {
                    (s.fade_out as f32 * w.fountain.fade_out_k * t.fade_scale) as i32
                }
            } else {
                if countdown(&mut s.wait) { s.fade_out = w.fountain.fade_out as i16; }
                t.full
            };
            let a = (opacity as u32) << 24;
            out.push(FxQuad {
                fx,
                additive: true,
                corners: std::array::from_fn(|k| corner(t.corners[k])),
                st: std::array::from_fn(|k| [t.corner_uv[k][0] - s.age, t.corner_uv[k][1]]),
                rgba: std::array::from_fn(|k| a.wrapping_mul(t.corner_alpha[k] as u32) | quad_rgb),
            });
        }
        w.fountain.slots[i] = s;
    }
    if m.pv_s(pv::PUFFS) == 0 {
        let mut rows = frames[0];
        rows[3] = [x[3][0], x[3][1], x[3][2] - m.pv_f(pv::HALF), 1.0];
        let (sy, sz) = (m.pv_f(pv::RADIUS), m.pv_f(pv::DEPTH));
        for q in &t.quads {
            out.push(FxQuad {
                fx: cap_fx,
                additive: false,
                corners: q.map(|n| through([t.cap[n][0], t.cap[n][1] * sy, t.cap[n][2] * sz, 1.0], &rows)),
                st: q.map(|n| [t.cap_uv[n][1] + w.fountain.scroll, t.cap_uv[n][0]]),
                rgba: q.map(|n| (t.alphas[n] as u32).wrapping_mul(0xff00_0000) | rgb),
            });
        }
    }
}
