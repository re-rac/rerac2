//! The arc beams: a level's table of beam slots, the manager moby that steps it once a tick and draws every live
//! slot, and the classes that claim a slot and report their beam's ends each tick. Tabora (L08) and Dobbo (L09) each
//! have their own copy of the code, the same shape with its own sizes; the port keeps one table ([`Beams`], in
//! crate::moby::MobyWorld::beams) and one step, with the two levels' shapes and draws as [`Kind`]s.
//!
//! | | Tabora ([`Kind::Tabora`]) | Dobbo ([`Kind::Dobbo`]) |
//! |---|---|---|
//! | Manager | 0x1006, `0x42a750` (`func_L08_004290E8`) | 0x0d12, `0x3fc960` (`func_L09_003FB4D0`) |
//! | Slots, points | 3, 20 | 16, 6 |
//! | Claim | `0x42a8a0(a, lit, follow)` | `0x3fcae0(a, b, lit, follow)` |
//! | Report | `0x42ab08(a, basis, target)` | `0x3fcd98(a or b, basis)` |
//! | Release | `0x42a9b8(a)` | `0x3fcc18(a or b)` |
//! | Free | `0x42aa70` (`func_L08_00429408`) | `0x3fccf0` (`func_L09_003FB860`) |
//! | Shape | `0x42abc8` ([`shape_tabora`]) | `0x3fce88` ([`shape_dobbo`]) |
//! | Draw | `0x42b550` ([`draw_tabora`]): `0x42bfe8` ends, `0x42b718` strips | `0x3fd428` ([`draw_dobbo`]): `0x3fdee8` ends, `0x3fd678` strips |
//! | Users | 0x1002 (crate::moby::classes::level08::arcer) | 0x0d09 from 0x0cea (crate::moby::classes::level09::towers) |
//!
//! **A slot** (`state`): 0 free; 1 claimed (its moby `a`, on Dobbo also `b`, its light bits `lit`, its follow word,
//! its three phases, its scroll and lights cleared); a report this tick makes 1 → 2 (first) or 4 → 3 and keeps the
//! reporter's basis (rows and origin) and the beam's far end; the manager's step shapes a slot in 2 or 3 and makes it
//! 4, frees one still in 4 (no report since its last shape) and clears a claimed one whose moby is gone. So a beam
//! lives while its owner reports it each tick.
//!
//! **The manager** ([`step`]): state 0 clears the slots (the first `a` / `b` and states) and makes 1, its update
//! distance 0xff; state 1 steps them and, with any shaped, registers its draw (the second list, `0x2f36e0`).
//!
//! **The shape**, the beam's two strips of points in its own frame (x along the beam from the origin to the far end,
//! y across, z up: rows `unit(end − origin)`, `−unit(row₀ × down)`, `row₁ × row₀`), only while the game plays
//! (`0x1a8f00` = 0); on a slot's first, both strips are laid straight along x. Its phases step by 10°, −46° and 3°.
//! * Tabora (20 points `spacing` = 1/20 of the length apart): the main strip hugs the ground — at each point after the
//!   first the height over the ground under it (`0x32a160` = L00 `0x31ce20` from 0.5 up, plus 0.6) taken off, at most
//!   half a step more than the last, else eased halfway to it — plus a wave of `(i/20)·0.3 + 0.1` × `sin(phase₀ + i·36°)`;
//!   its 19th point is the beam's tip. The second strip: the first's height plus `sin(phase₂ + i·12°)·0.5 ·
//!   sin(phase₁ + i·57°) + randf_signed(0, 0.2)`, across `cos(phase₁ + i·57°)·0.3`. Sparks ([`crate::particles::arc`],
//!   alpha 127): one at the origin moving `(1, ±1, ±1)` (random) at `randf(0.03, 0.12)` through the reporter's rows;
//!   one time in three a still one 0.3 out along its row 0 (life 4, size 120000, white); one at the tip moving a random
//!   way at `randf(0.01, 0.05)` (a `randf` drawn and dropped before it); one time in three a still one there. Lights
//!   (crate::point_lights: radius 5, intensity 0, colour (2, 1, 2)): `lit & 1` 0.25 under the origin, `lit & 2` 0.5
//!   over the tip.
//! * Dobbo (6 points 1/5 of the length apart): the main strip `(0.1 − 0.09·sin(i/5·π))·sin(phase₀ + i·120°)` up; the
//!   second its height plus `sin(phase₂ + i·12°)·0.2·sin(phase₁ + i·57°) + randf_signed(0, 0.2)`, across
//!   `cos(phase₁ + i·57°)·0.1`. Each end (`a`, then `b`), one time in three, a still spark at it jittered 0.1 (alpha
//!   `a`'s alpha · 127 / 100; life 4, size 120000, white). No lights.
//!
//! **The draw** (the manager's callback): additive strips of quads through the points (each segment's quad across the
//! view: `unit((camera − p) × along)`), the main strip with the manager's texture `pvars[1] + 63` and its glow twice as
//! wide with `pvars[0] + 63`, then the second strip at lower alphas with a spark ([`crate::particles::arc`], made at the
//! manager's next step: the draw has no pool) at a random segment. The scroll steps −0.2 a frame (wrapping at −8)
//! while the game plays; s runs scroll..scroll + 1 on each quad.
//! * Tabora (`0x42b718`): 0.6 wide (0.3 for an owner of class 0x1c) easing to 0.05 at the tip (`2t − t²`), colours
//!   `α₀ << 24 | 0x7f407f` and `α₁ << 24 | 0xff10ff` (128 / 64, then 48 / 16), the first quad faded in at the origin;
//!   the spark segment `rand_range(1, 20)`: two sparks at random corners of that segment's quad moving
//!   `randf(0.03, 0.15)` at ±20°..40° through the reporter's rows, one 0x7f207f (alpha 0x40, size 150000), one white
//!   (0x7f, 30000), both with the twinkle's frames (type 0x35). Ends (`0x42bfe8`): at the origin a camera-facing white
//!   square of FX 0xb (ALPHA 0x44) `randf(0.3, 0.325)` across, jittered `randf_signed(0, 0.05)` on each axis, and an
//!   additive one three times as wide `0x60ff10ff`; at the tip the same at `randf(0.2, 0.225)`.
//! * Dobbo (`0x3fd678`): each slot only when the sphere about its two ends (radius 0.526 of their distance) is in view
//!   within 64 (`0x2f0080`); the alphas × `a`'s alpha / 100; 0.4 wide narrowing to 0.1 in the middle
//!   (`sin(i/6·π)`), colours `0x7f3f40` and `0xffaf10`, the glow faded at both ends; the spark segment
//!   `rand_range(1, 6)`: two sparks moving `randf(0.01, 0.08)` a random way (its pitch a quarter) through the
//!   reporter's rows, at most 1/30 a tick: 0x7f3f20 (alpha × 64 / 100, 150000) and white (× 127 / 100, 30000). Ends
//!   (`0x3fdee8`): both `randf(0.3, 0.325)`, white at `a`'s alpha · 255 / 100 and `0xffaf10` at · 96 / 100.
//!
//! Not ported: the paused draws' fixed sizes are kept (the random draws still made), as the game does; the GS state the
//! draws set (`0x2fc560(0x47, 0x51001)`: no Z writes) is the fx draw's own. Dobbo's reporter leaves its basis's row 1
//! unset (stack garbage, `0x3f8590`): the port takes world up there, so only the sparks' directions differ.

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, cross, dist, len, scale, set_len, sub, sub_rot, V3};
use crate::moby::{Ctx, MobyWorld};
use crate::particles::arc;

/// The most slots a level has (Dobbo's).
pub const SLOTS: usize = 16;
/// The most points a strip has (Tabora's).
pub const POINTS: usize = 20;
/// `0x18a0e0`: down.
const DOWN: V3 = [0.0, 0.0, -1.0];
/// The square of the end glows (`$gp+0x2ca0` on L08, `+0x2110` on L09).
const SQUARE: [V3; 4] = [[0.0, -1.0, 1.0], [0.0, -1.0, -1.0], [0.0, 1.0, 1.0], [0.0, 1.0, -1.0]];
const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
/// The twinkle (type 0x35), whose first frame the strips' sparks take.
const TWINKLE: u8 = 0x35;

/// Which level's code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Tabora,
    Dobbo,
}

impl Kind {
    fn slots(self) -> usize { match self { Kind::Tabora => 3, Kind::Dobbo => SLOTS } }
}

/// Rows 0..2 and the origin (the game's 4 × 4 with its translation row).
pub type Basis = [V3; 4];

/// One slot (module docs; Tabora's tables `0x1b39a0..`, Dobbo's `0x1b35e0..`).
#[derive(Clone, Debug, Default)]
pub struct Slot {
    pub state: i32,
    pub a: Option<usize>,
    pub b: Option<usize>,
    pub phase: [f32; 3],
    pub lights: [Option<usize>; 2],
    pub lit: i32,
    pub follow: i32,
    pub scroll: f32,
    /// The reporter's basis (`0x2603e0` / `0x255ec0`).
    pub basis: Basis,
    /// The far end: Tabora's target (`0x1b3a40`), Dobbo's `b` (`0x2562c0`).
    pub end: V3,
    /// The tip: Tabora's 19th point in the world (`0x1b3a70`), Dobbo's `b` (`0x2563c0`).
    pub tip: V3,
    pub strips: [[[f32; 4]; POINTS]; 2],
}

/// A spark a strip's draw throws, made at the manager's next step (module docs).
#[derive(Clone, Copy, Debug)]
pub struct Spark {
    pub pos: V3,
    pub vel: V3,
    pub alpha: i32,
    pub rolls: arc::Rolls,
    pub size: f32,
    pub colour: u32,
}

/// The table (module docs).
#[derive(Clone, Debug, Default)]
pub struct Beams {
    pub slots: Vec<Slot>,
    pub sparks: Vec<Spark>,
}

impl Beams {
    fn ensure(&mut self) { if self.slots.len() < SLOTS { self.slots.resize(SLOTS, Slot::default()); } }

    /// The claim (module docs): true when `a` already holds a slot or one was free.
    pub fn claim(&mut self, kind: Kind, a: usize, b: Option<usize>, lit: i32, follow: i32) -> bool {
        self.ensure();
        let n = kind.slots();
        if self.slots[..n].iter().any(|s| s.a == Some(a) || (kind == Kind::Dobbo && s.b == Some(a))) { return true; }
        let Some(s) = self.slots[..n].iter_mut().find(|s| s.a.is_none()) else { return false };
        (s.a, s.b, s.phase, s.lights, s.lit, s.follow, s.scroll, s.state) = (Some(a), b, [0.0; 3], [None; 2], lit, follow, 0.0, 1);
        true
    }

    fn holding(&self, kind: Kind, key: usize) -> Option<usize> {
        let n = kind.slots().min(self.slots.len());
        self.slots[..n].iter().position(|s| s.a == Some(key) || (kind == Kind::Dobbo && s.b == Some(key)))
    }

    /// Tabora's report `0x42ab08(a, basis, target)`.
    pub fn report_tabora(&mut self, a: usize, basis: Basis, target: V3) -> bool {
        let Some(i) = self.holding(Kind::Tabora, a) else { return false };
        let s = &mut self.slots[i];
        s.state = match s.state { 1 => 2, 4 => 3, x => x };
        s.basis = basis;
        s.end = target;
        true
    }

    /// Dobbo's report `0x3fcd98(key, basis)`: the origin and both ends taken from the slot's mobys.
    pub fn report_dobbo(w: &mut MobyWorld, key: usize, basis: Basis) -> bool {
        let Some(i) = w.beams.holding(Kind::Dobbo, key) else { return false };
        let pos = |k: Option<usize>| k.and_then(|k| w.mobys.get(k)).map_or([0.0; 3], |m| m.pos);
        let (pa, pb) = (pos(w.beams.slots[i].a), pos(w.beams.slots[i].b));
        let s = &mut w.beams.slots[i];
        s.state = match s.state { 1 => 2, 4 => 3, x => x };
        s.basis = basis;
        s.basis[3] = pa;
        s.end = pb;
        true
    }

    /// The release (module docs): the slot holding `key` freed, its lights given back.
    pub fn release(w: &mut MobyWorld, kind: Kind, key: usize) -> bool {
        let Some(i) = w.beams.holding(kind, key) else { return false };
        free(w, i);
        true
    }
}

/// The free (module docs).
fn free(w: &mut MobyWorld, i: usize) {
    let s = &mut w.beams.slots[i];
    (s.state, s.a, s.b) = (0, None, None);
    for l in s.lights.iter_mut() {
        if let Some(k) = l.take() { w.point_lights.free(k); }
    }
}

fn gone(w: &MobyWorld, k: Option<usize>) -> bool { k.and_then(|k| w.mobys.get(k)).is_none_or(|m| 0x80 <= m.state) }

/// The manager's update (module docs): Tabora's 0x1006 and Dobbo's 0x0d12.
pub fn step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, kind: Kind) {
    w.beams.ensure();
    match w.mobys[id].state {
        0 => {
            for s in w.beams.slots.iter_mut().take(kind.slots()) { (s.state, s.a, s.b) = (0, None, None); }
            let m = &mut w.mobys[id];
            m.state = 1;
            m.update_dist = 0xff;
        }
        1 => {
            // The sparks the last draw threw.
            let sparks = std::mem::take(&mut w.beams.sparks);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                for s in sparks {
                    if let Some(i) = arc::put(sys, s.pos, [s.vel[0], s.vel[1], s.vel[2], 0.0], s.alpha, s.rolls) {
                        arc::tweak(sys, i, None, s.size, s.colour, Some(TWINKLE));
                    }
                }
            }
            let mut shaped = 0;
            for i in 0..kind.slots() {
                match w.beams.slots[i].state {
                    0 => {}
                    1 => {
                        if gone(w, w.beams.slots[i].a) {
                            let s = &mut w.beams.slots[i];
                            (s.state, s.a) = (0, None);
                            if kind == Kind::Dobbo { s.b = None; }
                        }
                    }
                    4 => free(w, i),
                    _ => {
                        shaped += 1;
                        match kind {
                            Kind::Tabora => shape_tabora(w, ctx, i),
                            Kind::Dobbo => shape_dobbo(w, ctx, i),
                        }
                        w.beams.slots[i].state = 4;
                    }
                }
            }
            if shaped != 0 { w.register_draw(id); }
        }
        _ => {}
    }
}

/// `xform` (`0x2fb030`-style 4 × 4): rows · v + origin.
fn xform(m: &Basis, v: [f32; 4]) -> V3 { add(add(add(scale(m[0], v[0]), scale(m[1], v[1])), scale(m[2], v[2])), m[3]) }
fn xform3(m: &Basis, v: V3) -> V3 { xform(m, [v[0], v[1], v[2], 1.0]) }
/// Through the rows only (`0x2fb018`).
fn turn(m: &Basis, v: V3) -> V3 { add(add(scale(m[0], v[0]), scale(m[1], v[1])), scale(m[2], v[2])) }

/// A frame along `from → to`: `unit(to − from)`, `−unit(row₀ × down)`, `row₁ × row₀`, at `at`.
fn along(from: V3, to: V3, at: V3) -> Basis {
    let b0 = set_len(sub(to, from), 1.0);
    let b1 = set_len(cross(b0, DOWN), -1.0);
    [b0, b1, cross(b1, b0), at]
}

/// A segment's frame: along `from → to`, its row 2 `unit((cam − eye) × row₀)`, row 1 `row₀ × row₂`, at `at`.
fn segment(from: V3, to: V3, eye: V3, cam: V3, at: V3) -> Basis {
    let f0 = set_len(sub(to, from), 1.0);
    let f2 = set_len(cross(sub(cam, eye), f0), 1.0);
    [f0, cross(f0, f2), f2, at]
}

/// `0x2ef1d0(max, v)`: `v` at most `max` long.
pub fn at_most(v: V3, max: f32) -> V3 { if max != 0.0 && max < len(v) { set_len(v, max) } else { v } }

fn playing(mode: i32) -> bool { mode == crate::scene::PLAY }

/// `0x320440`: an angle wrapped to [−π, π).
fn wrap(a: f32) -> f32 { use std::f32::consts::PI; ((a + PI) * 0.159_154_94).rem_euclid(1.0) * std::f32::consts::TAU - PI }

fn phases(s: &mut Slot) {
    s.phase[0] = add_rot(s.phase[0], 0.174_532_95);
    s.phase[1] = sub_rot(s.phase[1], 0.802_851_56);
    s.phase[2] = add_rot(s.phase[2], 0.052_36);
}

/// A still white spark at `p` (life 4, size 120000), following its owner by `follow` when asked (not ported).
fn still(ctx: &mut Ctx, p: V3, alpha: i32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    if let Some(i) = arc::spawn(sys, &mut ctx.game.rng, p, [0.0; 4], alpha) { arc::tweak(sys, i, Some(4), 120_000.0, 0x7f_7f7f, None); }
}

fn moving(ctx: &mut Ctx, p: V3, v: V3, w4: f32, alpha: i32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    arc::spawn(sys, &mut ctx.game.rng, p, [v[0], v[1], v[2], w4], alpha);
}

/// `0x31bb48(a, b)`: a vector `randf(a, b)` long a random way (yaw, then pitch).
fn shell(g: &mut crate::rng::Rng, a: f32, b: f32) -> V3 {
    let yaw = g.rand_angle();
    let pitch = g.rand_angle();
    let l = g.randf(a, b);
    crate::moby::classes::shot::polar(l, yaw, pitch)
}

/// `0x42abc8` (module docs).
pub fn shape_tabora(w: &mut MobyWorld, ctx: &mut Ctx, i: usize) {
    if !playing(ctx.game.scenes.mode) { return; }
    let mut s = w.beams.slots[i].clone();
    let origin = s.basis[3];
    let delta = sub(s.end, origin);
    let spacing = len(delta) * 0.05;
    if s.state == 2 {
        s.state = 3;
        for j in 0..POINTS {
            let p = [spacing * j as f32, 0.0, 0.0, 1.0];
            s.strips[0][j] = p;
            s.strips[1][j] = p;
        }
    }
    phases(&mut s);
    let mut point = origin;
    let step = set_len(delta, spacing);
    let (mut previous, mut advance) = (0.0f32, 0.0f32);
    let angle_step = (360.0 / (20.0 * 0.5)) * 0.017_453_294;
    for k in 0..19 {
        let half = (advance + spacing) * 0.5;
        let mut height = 0.0;
        if 0 < k {
            // Ratchet's vehicle's height when it is the moby `0x1bff58` (not ported: the port has no such vehicle).
            let ground = crate::moby::ai::ground_z(ctx, 0.5, point, 0) + 0.6;
            height = point[2] - ground;
            if previous + half < height { height = previous + half } else if 0.0 < height { height = (height + previous) * 0.5 }
            advance = height - previous;
            previous = height;
        }
        point = add(point, step);
        let t = k as f32;
        let weight = (t / 20.0) * 0.3 + 0.1;
        let weight = weight * wrap(s.phase[0] + t * angle_step).sin();
        s.strips[0][k] = [spacing * t, 0.0, weight - height, 1.0];
    }
    let frame = along(origin, s.end, origin);
    let tip = xform(&frame, s.strips[0][18]);
    s.tip = tip;
    for k in 0..19 {
        let t = k as f32;
        let twist = wrap(s.phase[1] + t * 0.994_837_76);
        let sway = wrap(s.phase[2] + t * 0.209_44);
        let b = sway.sin() * 0.5;
        let c = twist.sin();
        let r = ctx.game.rng.randf_signed(0.0, 0.2);
        s.strips[1][k] = [spacing * t, twist.cos() * 0.3, s.strips[0][k][2] + b * c + r, 1.0];
    }
    // The sparks.
    let g = &mut ctx.game.rng;
    let r = g.randf(0.03, 0.12);
    let y = g.randf(-1.0, 1.0);
    let z = g.randf(-1.0, 1.0);
    let v = turn(&s.basis, set_len([1.0, y, z], r));
    moving(ctx, origin, v, 1.0, 127);
    if ctx.game.rng.randi(3) == 0 {
        let p = add(scale(s.basis[0], 0.3), origin);
        still(ctx, p, 127);
    }
    let _ = ctx.game.rng.randf(0.01, 0.05);
    let v = shell(&mut ctx.game.rng, 0.01, 0.05);
    moving(ctx, tip, v, 0.0, 127);
    if ctx.game.rng.randi(3) == 0 { still(ctx, tip, 127); }
    // The lights.
    let light = |w: &mut MobyWorld, slot: &mut Option<usize>, pos: V3| {
        let l = crate::point_lights::PointLight { color: [2.0, 1.0, 2.0], intensity: 0.0, pos, radius: 5.0 };
        if slot.is_none() { *slot = w.point_lights.alloc(l); }
        if let Some(k) = *slot { w.point_lights.set(k, l); }
    };
    if s.lit & 1 != 0 {
        let mut p = origin;
        p[2] -= 0.25;
        light(w, &mut s.lights[0], p);
    }
    if s.lit & 2 != 0 {
        let mut p = tip;
        p[2] += 0.5;
        light(w, &mut s.lights[1], p);
    }
    w.beams.slots[i] = s;
}

/// `0x3fce88` (module docs).
pub fn shape_dobbo(w: &mut MobyWorld, ctx: &mut Ctx, i: usize) {
    if !playing(ctx.game.scenes.mode) { return; }
    let mut s = w.beams.slots[i].clone();
    let alpha = s.a.and_then(|k| w.mobys.get(k)).map_or(0, |m| m.alpha as i32);
    let origin = s.basis[3];
    let step = len(sub(s.end, origin)) / 5.0;
    if s.state == 2 {
        s.state = 3;
        for j in 0..6 {
            let p = [step * j as f32, 0.0, 0.0, 1.0];
            s.strips[0][j] = p;
            s.strips[1][j] = p;
        }
    }
    phases(&mut s);
    let angle_step = (360.0 / (6.0 * 0.5)) * 0.017_453_294;
    for k in 0..6 {
        let t = k as f32;
        let y = (t / 5.0 * std::f32::consts::PI).sin() * -0.09 + 0.1;
        let y = y * wrap(s.phase[0] + t * angle_step).sin();
        s.strips[0][k] = [step * t, 0.0, y, 1.0];
    }
    s.tip = s.b.and_then(|k| w.mobys.get(k)).map_or(s.end, |m| m.pos);
    for k in 0..6 {
        let t = k as f32;
        let x = wrap(s.phase[1] + t * 0.994_837_76);
        let z = wrap(s.phase[2] + t * 0.209_439_99);
        let y = z.sin() * 0.2;
        let zz = x.sin();
        let noise = ctx.game.rng.randf_signed(0.0, 0.2);
        s.strips[1][k] = [step * t, x.cos() * 0.1, (s.strips[0][k][2] + y * zz) + noise, 1.0];
    }
    for e in [s.a, s.b] {
        if ctx.game.rng.randi(3) != 0 { continue; }
        let mut p = e.and_then(|k| w.mobys.get(k)).map_or([0.0; 3], |m| m.pos);
        crate::moby::classes::level08::tabora_ai::jitter(&mut ctx.game.rng, &mut p, 0.1);
        still(ctx, p, alpha * 127 / 100);
    }
    s.lights = [None; 2];
    w.beams.slots[i] = s;
}

fn quad(out: &mut Vec<FxQuad>, fx: usize, additive: bool, corners: [V3; 4], st: [[f32; 2]; 4], rgba: [u32; 4]) {
    out.push(FxQuad { fx, additive, corners, st, rgba });
}

/// The end glows' squares at `pos`: white (ALPHA 0x44) `size` across jittered, then an additive one `3·size` across
/// in `glow`.
fn ends_at(out: &mut Vec<FxQuad>, pos: V3, cam: V3, size: f32, jitter: V3, white: u32, glow: u32) {
    let b = {
        let b0 = set_len(sub(cam, pos), 1.0);
        let b1 = set_len(cross(b0, DOWN), -1.0);
        [b0, b1, cross(b1, b0), pos]
    };
    let c = SQUARE.map(|k| add(xform3(&b, scale(k, size)), jitter));
    quad(out, 0xb, false, c, ST, [white; 4]);
    let c = SQUARE.map(|k| xform3(&b, scale(k, size * 3.0)));
    quad(out, 0xb, true, c, ST, [glow; 4]);
}

fn end_rolls(g: &mut crate::rng::Rng, play: bool, lo: f32, hi: f32, paused: f32) -> f32 {
    let s = g.randf(lo, hi);
    if play { s } else { paused }
}

fn end_jitter(g: &mut crate::rng::Rng, play: bool) -> V3 {
    let j = [g.randf_signed(0.0, 0.05), g.randf_signed(0.0, 0.05), g.randf_signed(0.0, 0.05)];
    if play { j } else { [0.025; 3] }
}

/// The manager's textures: `pvars[0] + 63` (the glow) and `pvars[1] + 63` (the strip).
fn textures(w: &MobyWorld, id: usize) -> (usize, usize) {
    let m = &w.mobys[id];
    ((m.pv_i(0) + 63).max(0) as usize, (m.pv_i(4) + 63).max(0) as usize)
}

/// Tabora's draw `0x42b550` (module docs).
pub fn draw_tabora(w: &mut MobyWorld, id: usize, game: &mut crate::game::GameState, cam: V3, out: &mut Vec<FxQuad>) {
    if w.beams.slots.is_empty() { return; }
    let (glow, main) = textures(w, id);
    let play = playing(game.scenes.mode);
    for i in 0..Kind::Tabora.slots() {
        if w.beams.slots[i].a.is_none() || w.beams.slots[i].state < 2 { continue; }
        if play {
            let s = &mut w.beams.slots[i].scroll;
            *s -= 0.2;
            if *s <= -8.0 { *s += 8.0; }
        }
        let s = w.beams.slots[i].clone();
        // `0x42bfe8`: the ends.
        let g = &mut game.rng;
        let size = end_rolls(g, play, 0.3, 0.325, 0.3125);
        let jitter = end_jitter(g, play);
        ends_at(out, s.basis[3], cam, size, jitter, 0xffff_ffff, 0x60ff_10ff);
        let size = end_rolls(g, play, 0.2, 0.225, 0.2225);
        ends_at(out, s.tip, cam, size, jitter, 0xffff_ffff, 0x60ff_10ff);
        let narrow = s.a.and_then(|k| w.mobys.get(k)).is_some_and(|m| m.o_class == 0x1c);
        strip_tabora(w, game, &s, [main, glow], &s.strips[0], [128, 64], narrow, -1, cam, out);
        let spark = game.rng.rand_range(1, 20);
        strip_tabora(w, game, &s, [main, glow], &s.strips[1], [48, 16], narrow, spark, cam, out);
    }
}

/// `0x42b718(tex₀, tex₁, points, α₀, α₁, slot, spark)` (module docs).
#[allow(clippy::too_many_arguments)]
fn strip_tabora(w: &mut MobyWorld, game: &mut crate::game::GameState, s: &Slot, fx: [usize; 2], points: &[[f32; 4]; POINTS], alpha: [u32; 2], narrow: bool, spark: i32, cam: V3, out: &mut Vec<FxQuad>) {
    let c0 = alpha[0] << 24 | 0x7f_407f;
    let c1 = alpha[1] << 24 | 0xff_10ff;
    let u = s.scroll;
    let st = [[u, 0.0], [u, 1.0], [u + 1.0, 0.0], [u + 1.0, 1.0]];
    let gst = [[1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0]];
    let w0 = if narrow { 0.3 } else { 0.6 };
    let mut width = w0;
    let basis = along(s.basis[3], s.end, s.basis[3]);
    let edge = |p: [f32; 4], q: [f32; 4], width: f32| {
        let (e0, e1) = (xform(&basis, p), xform(&basis, q));
        let f = segment(e0, e1, e0, cam, e0);
        ([xform3(&f, [0.0, 0.0, width]), xform3(&f, [0.0, 0.0, -width])], [xform3(&f, [0.0, 0.0, 2.0 * width]), xform3(&f, [0.0, 0.0, -2.0 * width])])
    };
    let (mut a, mut ga) = edge(points[0], points[1], width);
    let play = playing(game.scenes.mode);
    for i in 1..19 {
        let (b, gb) = edge(points[i], points[i + 1], width);
        let corners = [a[0], a[1], b[0], b[1]];
        let gcorners = [ga[0], ga[1], gb[0], gb[1]];
        if i == 1 {
            quad(out, fx[0], true, corners, st, [0xff_ffff, 0xff_ffff, c0, c0]);
            quad(out, fx[1], true, gcorners, gst, [0xff_10ff, 0xff_10ff, c1, c1]);
        } else {
            quad(out, fx[0], true, corners, st, [c0; 4]);
            quad(out, fx[1], true, gcorners, gst, [c1; 4]);
        }
        (a, ga) = (b, gb);
        if i as i32 == spark && play {
            let g = &mut game.rng;
            let r = g.randf(0.03, 0.15);
            let angle = g.randf_signed(0.349_065_9, 0.698_131_8);
            let v = turn(&s.basis, [r * angle.cos(), r * angle.sin(), 0.0]);
            // The quad's corners as the game holds them here: the next segment's start twice.
            let held = [b[0], b[1], b[0], b[1]];
            for (alpha, size, colour) in [(0x40, 150_000.0, 0x7f_207f), (0x7f, 30_000.0, 0xff_ffff)] {
                let corner = held[(g.rand() & 3) as usize];
                let rolls = arc::rolls(g);
                w.beams.sparks.push(Spark { pos: corner, vel: v, alpha, rolls, size, colour });
            }
        }
        let t = i as f32 / 20.0;
        let t = 2.0 * t - t * t;
        width = w0 + (0.05 - w0) * t;
    }
}

/// Dobbo's draw `0x3fd428` (module docs).
pub fn draw_dobbo(w: &mut MobyWorld, id: usize, game: &mut crate::game::GameState, view: &crate::particles::BSphereView, cam: V3, out: &mut Vec<FxQuad>) {
    if w.beams.slots.is_empty() { return; }
    let (glow, main) = textures(w, id);
    let play = playing(game.scenes.mode);
    for i in 0..Kind::Dobbo.slots() {
        let s = &w.beams.slots[i];
        let (Some(a), Some(b)) = (s.a.and_then(|k| w.mobys.get(k)), s.b.and_then(|k| w.mobys.get(k))) else { continue };
        if s.state < 2 { continue; }
        let mid = scale(add(a.pos, b.pos), 0.5);
        let r = dist(a.pos, b.pos) * 0.526_315_8;
        if view.culled(64.0, [mid[0], mid[1], mid[2], r]) { continue; }
        let fade = a.alpha as u32;
        if play {
            let s = &mut w.beams.slots[i].scroll;
            *s -= 0.2;
            if *s <= -8.0 { *s += 8.0; }
        }
        let s = w.beams.slots[i].clone();
        // `0x3fdee8`: the ends.
        let white = (fade * 255 / 100) << 24 | 0xff_ffff;
        let g2 = (fade * 96 / 100) << 24 | 0xff_af10;
        let g = &mut game.rng;
        let size = end_rolls(g, play, 0.3, 0.325, 0.3125);
        let jitter = end_jitter(g, play);
        ends_at(out, s.basis[3], cam, size, jitter, white, g2);
        let size = end_rolls(g, play, 0.3, 0.325, 0.3125);
        ends_at(out, s.tip, cam, size, jitter, white, g2);
        strip_dobbo(w, game, &s, [main, glow], &s.strips[0], [128, 64], fade, -1, cam, out);
        let spark = game.rng.rand_range(1, 6);
        strip_dobbo(w, game, &s, [main, glow], &s.strips[1], [48, 16], fade, spark, cam, out);
    }
}

/// `0x3fd678(tex₀, tex₁, points, inner, outer, alpha, slot, spark)` (module docs).
#[allow(clippy::too_many_arguments)]
fn strip_dobbo(w: &mut MobyWorld, game: &mut crate::game::GameState, s: &Slot, fx: [usize; 2], points: &[[f32; 4]; POINTS], alpha: [u32; 2], fade: u32, spark: i32, cam: V3, out: &mut Vec<FxQuad>) {
    const SIZE: f32 = 0.4;
    const MIDDLE: f32 = 0.1;
    const GLOW: f32 = 2.0;
    let c0 = (alpha[0] * fade / 100) << 24 | 0x7f_3f40;
    let c1 = (alpha[1] * fade / 100) << 24 | 0xff_af10;
    let u = s.scroll;
    let st = [[u, 0.0], [u, 1.0], [u + 1.0, 0.0], [u + 1.0, 1.0]];
    let gst = [[1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [0.0, 1.0]];
    let axis = along(s.basis[3], s.end, s.basis[3]);
    let mut size = SIZE;
    let edge = |from: [f32; 4], to: [f32; 4], at_to: bool, size: f32| {
        let (b, a) = (xform(&axis, from), xform(&axis, to));
        let f = segment(b, a, a, cam, if at_to { a } else { b });
        ([xform3(&f, [0.0, 0.0, size]), xform3(&f, [0.0, 0.0, -size])], [xform3(&f, [0.0, 0.0, size * GLOW]), xform3(&f, [0.0, 0.0, -size * GLOW])])
    };
    let (mut a, mut ga) = edge(points[0], points[1], false, size);
    let play = playing(game.scenes.mode);
    for i in 1..6 {
        let (b, gb) = edge(points[i - 1], points[i], true, size);
        let corners = [a[0], a[1], b[0], b[1]];
        let gcorners = [ga[0], ga[1], gb[0], gb[1]];
        let gc = match i {
            1 => [0xff_af10, 0xff_af10, c1, c1],
            5 => [c1, c1, 0xff_af10, 0xff_af10],
            _ => [c1; 4],
        };
        quad(out, fx[0], true, corners, st, [c0; 4]);
        quad(out, fx[1], true, gcorners, gst, gc);
        (a, ga) = (b, gb);
        if i as i32 == spark && play {
            let g = &mut game.rng;
            let r0 = g.randf(0.01, 0.08);
            let r1 = g.rand_angle();
            let r2 = g.rand_angle();
            let v = crate::moby::classes::shot::polar(r0, r1, r2 * 0.25);
            let v = at_most(turn(&s.basis, v), 1.0 / 30.0);
            let held = [b[0], b[1], b[0], b[1]];
            for (al, sz, colour) in [(fade as i32 * 64 / 100, 150_000.0, 0x7f_3f20), (fade as i32 * 127 / 100, 30_000.0, 0xff_ffff)] {
                let corner = held[(g.rand() & 3) as usize];
                let rolls = arc::rolls(g);
                w.beams.sparks.push(Spark { pos: corner, vel: v, alpha: al, rolls, size: sz, colour });
            }
        }
        let fraction = i as f32 / 6.0;
        size = SIZE + (MIDDLE - SIZE) * (fraction * std::f32::consts::PI).sin();
    }
}
