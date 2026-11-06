//! The sinking pit **0x135f** (L14 `0x3907c0`, 12 placed; L19 `0x383478`, 2 placed; L00 `0x386740`; the decomp's
//! `func_L00_00385370`): an 8 × 8 grid of points over a quadrilateral, waving, that rises and falls and draws itself as
//! 7 × 7 quads (`0x390d60`); Ratchet inside it sinks. Its helpers are by their L14 addresses (L00: `0x386e80`
//! set-up, `0x387868` the waves, `0x387738` the heights, `0x386a10` the sink, `0x387d98` the motes, `0x386ce0` the
//! draw).
//!
//! Its pvars: `+0x00` the acceleration, `+0x04` its height now, `+0x08` its speed, `+0x0c` / `+0x10` the ticks held
//! up / down, `+0x14` the top speed, `+0x18` the rise, `+0x1c` the ticks of a move, `+0x20` the timer, `+0x24` the
//! shade's period, `+0x28` the texture's tiling, `+0x2c` its wobble, `+0x30` the waves' period (then its inverse),
//! `+0x34` their height, `+0x38` / `+0x3c` their phase across x / y, `+0x40` its surface (each pit its own grid),
//! `+0x44` its cuboid (−1 none), `+0x48` its texture (an FX index − 0x3f), `+0x4c` its flags (1 / 2: the x / y
//! edges held; 4 no heights nor draw; 8 no motes; 0x10 / 0x20 / 0x40 no waves / texture wobble / shading), `+0x50..`
//! / `+0x60..` its four corners' x / y, `+0x70` its base height, `+0x74..+0x88` the motes' spin, size, alpha, life,
//! wait and gravity, `+0x8c` the motes' timer, `+0x90` / `+0x94` the cell Ratchet sinks in, `+0x98` his ticks sinking
//! (−1 none).
//!
//! * **0**: the ticks of a move 20 when none; height 0; the top speed `2·rise / ticks`, the acceleration
//!   `4·rise / ticks²`; its grid set up ([`setup`], `0x390f00`); the waves' period inverted; the motes' timer its
//!   wait; not sinking. Surface 0 alone: alpha 0xff and the shared samples' rates (64 × `randf(0.5, 2)` and
//!   `randf(0.002, 0.006)`). 1 (and nothing else this tick).
//! * **1** (rising): on its first tick speed 0, the timer the move's ticks; out: 2. Its height eased toward the rise
//!   (`0x3267b0` = L00 `0x31ed78`, crate::camera::approach with no step: the top speed only).
//! * **2** (up): on its first tick the timer the ticks held up and its height the rise; out: 3.
//! * **3** (falling): as 1, toward 0; out: 4.
//! * **4** (down): on its first tick its height 0, the timer the ticks held down; out: 1.
//! * **5** (Ratchet sinking): nothing.
//! * **Every tick after**: surface 0 steps the shared samples ([`waves`], `0x3918e8`); its heights ([`heights`],
//!   `0x3917b8`); the sink ([`sink`], `0x390a90`); the motes ([`motes`], `0x391e18`); its draw ([`draw`]) on the
//!   world's list (`0x2ef160` = L00 `0x2e77e0`, run by `0x2e7830` after the tfrags and ties, before the shrubs: the
//!   port's first list stands in).

use crate::fx::FxQuad;
use crate::hero::physics::V3;
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x135f;
/// The corners' class (its pvar `+0x14` the corner 0..3; corner 0's height the base).
pub const CORNER: i16 = 0x1130;

const ACCEL: usize = 0x00;
const HEIGHT: usize = 0x04;
const SPEED: usize = 0x08;
const HOLD_UP: usize = 0x0c;
const HOLD_DOWN: usize = 0x10;
const TOP: usize = 0x14;
const RISE: usize = 0x18;
const TICKS: usize = 0x1c;
const TIMER: usize = 0x20;
const SHADE_PERIOD: usize = 0x24;
const TILE: usize = 0x28;
const WOBBLE: usize = 0x2c;
const WAVE_PERIOD: usize = 0x30;
const WAVE_HEIGHT: usize = 0x34;
const PHASE_X: usize = 0x38;
const PHASE_Y: usize = 0x3c;
const SURFACE: usize = 0x40;
const CUBOID: usize = 0x44;
const TEX: usize = 0x48;
const FLAGS: usize = 0x4c;
const CORNER_X: usize = 0x50;
const CORNER_Y: usize = 0x60;
const BASE: usize = 0x70;
const MOTE_SPIN: usize = 0x74;
const MOTE_SIZE: usize = 0x78;
const MOTE_ALPHA: usize = 0x7c;
const MOTE_LIFE: usize = 0x80;
const MOTE_WAIT: usize = 0x84;
const MOTE_GRAVITY: usize = 0x88;
const MOTE_TIMER: usize = 0x8c;
const CELL_X: usize = 0x90;
const CELL_Y: usize = 0x94;
const SINKING: usize = 0x98;

/// One of the 64 shared samples (`0x233180` in L14, 0x20 bytes each).
#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    /// `+0x00`: the wave, `+0x04` the shade, `+0x08` / `+0x0c` their rates, `+0x10` the texture's wobble.
    pub wave: f32,
    pub shade: f32,
    pub shade_rate: f32,
    pub wobble_rate: f32,
    pub wobble: f32,
}

/// The pits' shared state (crate::moby::MobyWorld::pit): the samples every pit reads (stepped by surface 0) and each
/// surface's 64 points (`0x233980 + surface·0x400`, 16 bytes each).
#[derive(Clone, Debug)]
pub struct Globals {
    pub samples: [Sample; 64],
    pub grids: Vec<[V3; 64]>,
}

impl Default for Globals {
    fn default() -> Self { Globals { samples: [Sample::default(); 64], grids: Vec::new() } }
}

impl Globals {
    fn grid(&mut self, surface: i32) -> &mut [V3; 64] {
        let k = surface.max(0) as usize;
        if self.grids.len() <= k { self.grids.resize(k + 1, [[0.0; 3]; 64]); }
        &mut self.grids[k]
    }

    fn point(&self, surface: i32, k: usize) -> V3 {
        self.grids.get(surface.max(0) as usize).map_or([0.0; 3], |g| g[k.min(63)])
    }
}

fn timer_out(m: &mut crate::moby::Moby, off: usize) -> bool {
    let mut t = m.pv_i(off);
    let out = countdown(&mut t);
    m.set_pv_i(off, t);
    out
}

/// `0x3267b0(&height, &speed, target, 0, 0, top)`.
fn ease(m: &mut crate::moby::Moby, target: f32) {
    let (mut x, mut v) = (m.pv_f(HEIGHT), m.pv_f(SPEED));
    crate::camera::approach(target, m.pv_f(ACCEL), m.pv_f(ACCEL), m.pv_f(TOP), &mut x, &mut v);
    m.set_pv_f(HEIGHT, x);
    m.set_pv_f(SPEED, v);
}

/// `0x3907c0` (module docs).
pub fn update_135f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            if m.pv_i(TICKS) <= 0 { m.set_pv_i(TICKS, 20); }
            let t = m.pv_i(TICKS) as f32;
            m.set_pv_f(HEIGHT, 0.0);
            m.set_pv_f(TOP, (m.pv_f(RISE) + m.pv_f(RISE)) / t);
            m.set_pv_f(ACCEL, m.pv_f(RISE) * 4.0 / (m.pv_i(TICKS) * m.pv_i(TICKS)) as f32);
            setup(w, id);
            let m = &mut w.mobys[id];
            m.set_pv_f(WAVE_PERIOD, 1.0 / m.pv_f(WAVE_PERIOD));
            m.set_pv_i(MOTE_TIMER, m.pv_i(MOTE_WAIT));
            m.set_pv_i(SINKING, -1);
            if m.pv_i(SURFACE) == 0 {
                m.alpha = 0xff;
                for s in w.pit.samples.iter_mut() {
                    s.shade_rate = ctx.game.rng.randf(0.5, 2.0);
                    s.wobble_rate = ctx.game.rng.randf(0.002, 0.006);
                }
            }
            w.mobys[id].set_state(1, -1);
            return;
        }
        s @ (1 | 3) => {
            if m.first() {
                m.set_pv_f(SPEED, 0.0);
                m.set_pv_i(TIMER, m.pv_i(TICKS));
            }
            if timer_out(m, TIMER) { m.set_state(s + 1, -1); }
            let target = if s == 1 { m.pv_f(RISE) } else { 0.0 };
            ease(m, target);
        }
        2 => {
            if m.first() {
                m.set_pv_i(TIMER, m.pv_i(HOLD_UP));
                m.set_pv_f(HEIGHT, m.pv_f(RISE));
            }
            if timer_out(m, TIMER) { m.set_state(3, -1); }
        }
        4 => {
            if m.first() {
                m.set_pv_f(HEIGHT, 0.0);
                m.set_pv_i(TIMER, m.pv_i(HOLD_DOWN));
            }
            if timer_out(m, TIMER) { m.set_state(1, -1); }
        }
        _ => {}
    }
    if w.mobys[id].pv_i(SURFACE) == 0 { waves(w, id, ctx.game.frame); }
    let shown = heights(w, id, ctx);
    sink(w, id, ctx);
    motes(w, id, ctx);
    // The draw (`0x390d60`) tests the cuboid as `0x3917b8` does, with nothing moved between.
    if shown { w.register_draw_first(id); }
}

/// `0x390f00`: with its group (`+0x21`) holding 5 alive (`0x324260`), each 0x1130 of it gives its corner (`+0x14`,
/// 0 past 3) its x and y, corner 0 the base height; else the corners and base 0. The points: column c and row r of
/// 7 each `l + (r' − l)·r/7`, `l = c₀ + (c₁ − c₀)·c/7` and `r' = c₃ + (c₂ − c₃)·c/7`, at the base height.
fn setup(w: &mut MobyWorld, id: usize) {
    let group = w.mobys[id].group as i32;
    let mut corners = ([0.0f32; 4], [0.0f32; 4], 0.0f32);
    if crate::moby::classes::props::group_alive(w, group) == 5 {
        let members: Vec<usize> = w.groups.get(group as usize).cloned().unwrap_or_default();
        for k in members {
            let Some(c) = w.mobys.get_mut(k) else { continue };
            if c.o_class != CORNER { continue; }
            if 3 < c.pv_i(0x14) as u32 { c.set_pv_i(0x14, 0); }
            let n = c.pv_i(0x14) as usize;
            corners.0[n] = c.pos[0];
            corners.1[n] = c.pos[1];
            if n == 0 { corners.2 = c.pos[2]; }
        }
    }
    let m = &mut w.mobys[id];
    for k in 0..4 {
        m.set_pv_f(CORNER_X + 4 * k, corners.0[k]);
        m.set_pv_f(CORNER_Y + 4 * k, corners.1[k]);
    }
    m.set_pv_f(BASE, corners.2);
    let (x, y, z) = corners;
    let surface = m.pv_i(SURFACE);
    let grid = w.pit.grid(surface);
    for r in 0..8 {
        for c in 0..8 {
            let fc = c as f32 * 0.142_857_15;
            let l = [x[0] + (x[1] - x[0]) * fc, y[0] + (y[1] - y[0]) * fc];
            let rr = [x[3] + (x[2] - x[3]) * fc, y[3] + (y[2] - y[3]) * fc];
            let fr = r as f32 * 0.142_857_15;
            grid[r * 8 + c] = [l[0] + (rr[0] - l[0]) * fr, l[1] + (rr[1] - l[1]) * fr, z];
        }
    }
}

/// `0x2f7b88` (L00 `0x2eff80`): the fraction.
fn fract(x: f32) -> f32 { x - x.trunc() }

/// `0x3918e8` (surface 0): sample k (column `k mod 8`, row `k / 8`) — its wave `sin(2π·fract(px·c + py·r + frame /
/// period))` unless flag 0x10, its shade `cos(2π·fract(frame·rate / shade period))` unless 0x40, its wobble
/// `sin(2π·fract(frame·rate))` unless 0x20.
fn waves(w: &mut MobyWorld, id: usize, frame: i32) {
    let m = &w.mobys[id];
    let (flags, f) = (m.pv_i(FLAGS), frame as f32);
    let (px, py, inv, period) = (m.pv_f(PHASE_X), m.pv_f(PHASE_Y), m.pv_f(WAVE_PERIOD), m.pv_f(SHADE_PERIOD));
    const TAU: f32 = 6.283_185_5;
    for (k, s) in w.pit.samples.iter_mut().enumerate() {
        let (c, r) = ((k % 8) as f32, (k / 8) as f32);
        if flags & 0x10 == 0 { s.wave = (fract(px * c + py * r + f * inv) * TAU).sin(); }
        if flags & 0x40 == 0 { s.shade = (fract(f * s.shade_rate / period) * TAU).cos(); }
        if flags & 0x20 == 0 { s.wobble = (fract(f * s.wobble_rate) * TAU).sin(); }
    }
}

/// `0x3917b8`: with a cuboid and Ratchet in it (`0x329290` = L00 `0x321858`), flag 4 clear, each point's height the
/// base plus its height now, plus its sample's wave × the waves' height unless flag 0x10 or the point is on an edge
/// held (flag 1: columns 0 and 7; flag 2: rows 0 and 7). Returns whether it ran (the draw's own test).
fn heights(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let (cuboid, flags) = (m.pv_i(CUBOID), m.pv_i(FLAGS));
    if cuboid == -1 || !ctx.level.in_cuboid(cuboid, ctx.hero.pos) || flags & 4 != 0 { return false; }
    let (z, amp, surface) = (m.pv_f(BASE) + m.pv_f(HEIGHT), m.pv_f(WAVE_HEIGHT), m.pv_i(SURFACE));
    let waves: Vec<f32> = w.pit.samples.iter().map(|s| s.wave).collect();
    let grid = w.pit.grid(surface);
    for (k, p) in grid.iter_mut().enumerate() {
        p[2] = z;
        if flags & 0x10 == 0 {
            let (c, r) = (k % 8, k / 8);
            if (c != 0 && c != 7 || flags & 1 == 0) && (r != 0 && r != 7 || flags & 2 == 0) { p[2] += waves[k] * amp; }
        }
    }
    true
}

/// `0x391b10(pit, p)`: with a cuboid holding `p`, `p` within the waves' height above the pit's height now and 1.2 more
/// below, inside its four corners (`0x324848`, none of the edges with `p` on its outer side), the first cell (row by
/// row) whose corners hold `p` and whose mean height is above `p` by 1.2 at most.
fn cell_of(w: &MobyWorld, id: usize, ctx: &Ctx, p: V3) -> Option<(i32, i32)> {
    let m = &w.mobys[id];
    let cuboid = m.pv_i(CUBOID);
    if cuboid == -1 || !ctx.level.in_cuboid(cuboid, p) { return None; }
    let top = m.pv_f(BASE) + m.pv_f(HEIGHT);
    let amp = m.pv_f(WAVE_HEIGHT);
    if top + amp < p[2] || p[2] < top - amp - 1.2 { return None; }
    let outside = |c: &[[f32; 2]; 4]| (0..4).any(|i| {
        let (a, b) = (c[i], c[(i + 1) % 4]);
        (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]) > 0.0
    });
    let quad: [[f32; 2]; 4] = std::array::from_fn(|k| [m.pv_f(CORNER_X + 4 * k), m.pv_f(CORNER_Y + 4 * k)]);
    if outside(&quad) { return None; }
    let surface = m.pv_i(SURFACE);
    let pt = |k: usize| w.pit.point(surface, k);
    for j in 0..7 {
        for i in 0..7 {
            let (a, b) = (j * 8 + i, (j + 1) * 8 + i);
            let ks = [a, a + 1, b + 1, b];
            let c = ks.map(|k| [pt(k)[0], pt(k)[1]]);
            if outside(&c) { continue; }
            let h = ks.iter().map(|&k| pt(k)[2]).sum::<f32>() * 0.25;
            if p[2] <= h && h - 1.2 <= p[2] { return Some((i as i32, j as i32)); }
        }
    }
    None
}

/// `0x390a90`: not sinking yet, Ratchet's place (`0x189ea0`) in a cell ([`cell_of`]): sinking from 0, the pit to 5.
/// Sinking: with the camera's height within the pit's height now ± (the waves' height + 1), the camera's shift
/// (`0x1b9708` = L00 `0x1b8fc8`, crate::camera::CamTweaks::shift) lifts it to the top of that; Ratchet, unless in
/// 0x38 / 0x79 / 0x7a or in 0x39 with no sequence 0x6a, put into 0x7a (another body, `0x18c0d4`), 0x79 (his class
/// has sequence 0x6a) or 0x39 (`0x2cbee0` = L00 `0x2c4560`); his height lerped (`0x326270`) toward his cell's first
/// point's less ticks / 60 by ticks · 0.05 (at most 1); ticks + 1.
///
/// Not ported: Ratchet in 0x31 holding a moby (`0x189e20 + 0x1830`): its blast (`0x327f70` = L00 `0x320538`), the
/// sound `0x2d9e88(0x88)`, the hold let go, state 0 and `0x2ba4d8`, then after 5 ticks of sinking `0x2bab70`.
fn sink(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(SINKING) == -1 {
        if let Some((c, r)) = cell_of(w, id, ctx, ctx.hero.pos) {
            let m = &mut w.mobys[id];
            m.set_pv_i(CELL_X, c);
            m.set_pv_i(CELL_Y, r);
            m.set_pv_i(SINKING, 0);
            m.set_state(5, -1);
        }
    }
    let m = &w.mobys[id];
    let ticks = m.pv_i(SINKING);
    if ticks < 0 { return; }
    let centre = m.pv_f(BASE) + m.pv_f(HEIGHT);
    let amp = m.pv_f(WAVE_HEIGHT);
    let ceiling = centre + amp + 1.0;
    let cam_z = ctx.host.camera_pos()[2];
    if cam_z <= ceiling && centre - amp - 1.0 <= cam_z { ctx.game.cam_tweaks.shift[2] = ceiling - cam_z; }
    let has_seq = ctx.hero_moby.and_then(|h| w.mobys.get(h)).and_then(|hm| ctx.host.seq_frames(hm, 0x6a)).is_some_and(|n| n != 0);
    let s = ctx.hero.state;
    if s != 0x38 && s != 0x79 && (s != 0x39 || has_seq) && s != 0x7a {
        let next = if ctx.hero.mode != 0 { 0x7a } else if has_seq { 0x79 } else { 0x39 };
        ctx.hero.request_state(next, true);
    }
    let blend = (ticks as f32 * 0.05).min(1.0);
    let k = (m.pv_i(CELL_Y) * 8 + m.pv_i(CELL_X)).clamp(0, 63) as usize;
    let goal = w.pit.point(m.pv_i(SURFACE), k)[2] - ticks as f32 * 0.016_666_668;
    let z = ctx.hero.pos[2];
    ctx.hero.pos[2] = z + (goal - z) * blend;
    w.mobys[id].set_pv_i(SINKING, ticks + 1);
}

/// `0x391258`: a random place in the grid — `x, y = randf(0, 6.99)` each, from cell (⌊x⌋, ⌊y⌋)'s first point along
/// its next point by fract(x) and toward the point below by fract(y), at that first point's height.
fn place(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng) -> V3 {
    let x = rng.randf(0.0, 6.99);
    let y = rng.randf(0.0, 6.99);
    let (ix, iy) = (x as usize, y as usize);
    let surface = w.mobys[id].pv_i(SURFACE);
    let pt = |k: usize| w.pit.point(surface, k);
    let k = iy * 8 + ix;
    let (p, next, below) = (pt(k), pt(k + 1), pt((iy + 1) * 8 + ix));
    let (fx, fy) = (fract(x), fract(y));
    [p[0] + (next[0] - p[0]) * fx + (below[0] - p[0]) * fy, p[1] + (next[1] - p[1]) * fx + (below[1] - p[1]) * fy, p[2]]
}

/// `0x391e18`: unless flag 8, the motes' timer out: a place ([`place`]); with nothing within 1 above it (`0x391a98`: a
/// line 4 up, flags 0x12; no hit 9999) a mote (crate::particles::mote, `0x34ae20`) there moving up at the pit's speed,
/// grey 0x808080 at the motes' alpha → 0, size `+0x78 · 210000` → 0, its life, spin and gravity, def entry 3; the
/// timer its wait again.
fn motes(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(FLAGS) & 8 != 0 { return; }
    if !timer_out(&mut w.mobys[id], MOTE_TIMER) { return; }
    let p = place(w, id, &mut ctx.game.rng);
    let top = [p[0], p[1], p[2] + 4.0];
    let d = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p, top, crate::collision::QueryFlags(0x12), None)).map_or(9999.0, |o| crate::hero::physics::dist(p, o.point));
    let m = &w.mobys[id];
    if 1.0 < d {
        let vel = [0.0, 0.0, m.pv_f(SPEED)];
        let (size, life, spin, alpha, gravity) = (m.pv_f(MOTE_SIZE) * 210_000.0, m.pv_i(MOTE_LIFE), m.pv_i(MOTE_SPIN), m.pv_i(MOTE_ALPHA), m.pv_f(MOTE_GRAVITY));
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::mote::mote(sys, &mut ctx.game.rng, size, 0.0, gravity, p, vel, life, 0x80_8080, spin, alpha, 0, 3);
        }
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(MOTE_TIMER, m.pv_i(MOTE_WAIT));
}

/// A sample's wobble by its index the C way (`(r mod 8)·8 + c mod 8`, the remainders keeping their sign): the
/// entries before the table (column or row −1) read as 0 here.
fn wobble(s: &[Sample; 64], c: i32, r: i32) -> f32 {
    let k = (r % 8) * 8 + c % 8;
    usize::try_from(k).ok().and_then(|k| s.get(k)).map_or(0.0, |s| s.wobble)
}

/// `0x390d60` (module docs): the 7 × 7 cells as quads (`0x391428` per corner: column c, row r), FX `+0x48` + 0x3f,
/// ALPHA 0x88 (`Cs·As`); each corner at its point, coloured `(0x80, g, g, 0x80)` with `g = shade·48 + 80` (unless
/// flag 0x40), its `s` `c·tiling + wobble·(n(c, r) + n(c+1, r) + n(c−1, r) + n(c, r+1) + n(c, r−1))` and `t` the
/// same about (c+4, r+4) on rows (unless 0x20). Here the normal blend at alpha 0x80.
pub fn draw(w: &MobyWorld, id: usize, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let (flags, surface) = (m.pv_i(FLAGS), m.pv_i(SURFACE));
    let (tile, wob) = (m.pv_f(TILE), m.pv_f(WOBBLE));
    let s = &w.pit.samples;
    let n5 = |c: i32, r: i32| wobble(s, c, r) + wobble(s, c + 1, r) + wobble(s, c - 1, r) + wobble(s, c, r + 1) + wobble(s, c, r - 1);
    let fx = (m.pv_i(TEX) + 0x3f).max(0) as usize;
    let mut rgba = [0x8080_8080u32; 4];
    let mut st = [[0.0f32; 2]; 4];
    for r in 0..7 {
        for c in 0..7 {
            let cells = [(c, r), (c + 1, r), (c, r + 1), (c + 1, r + 1)];
            let corners = cells.map(|(c, r)| w.pit.point(surface, (r * 8 + c) as usize));
            for (v, &(c, r)) in cells.iter().enumerate() {
                let k = (r * 8 + c) as usize;
                if flags & 0x40 == 0 {
                    let g = (s[k].shade * 48.0 + 80.0) as i32 as u32;
                    rgba[v] = (g << 16) | (g << 8) | 0x8000_0000 | 0x80;
                }
                if flags & 0x20 == 0 {
                    st[v] = [c as f32 * tile + wob * n5(c, r), r as f32 * tile + wob * n5(c + 4, r + 4)];
                }
            }
            out.push(FxQuad { fx, additive: false, corners, st, rgba });
        }
    }
}
