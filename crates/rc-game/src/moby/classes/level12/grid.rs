//! Level 12's laser grids **0x0f9f** (`0x419100`, 3 placed; the decomp's `func_L12_00417CA8`; L12 addresses): a row
//! of laser lines across a corridor (its cuboid `[0]`, the gameplay's table `0x1b2f0c`, the lines along its `r₁`, the
//! corridor along its `r₂`) that burns Ratchet in it, but for the shadow of the Tractor Beam's shuttle box 0x0fc4
//! (crate::moby::classes::level12::tractor) riding across it in its own cuboid (`+0x48`): Ratchet walks through behind
//! the box. Through to its far end (or into the cuboid `+0x58`), its uid is counted: next time it is off.
//!
//! Its pvars: `+0x00` the corridor's cuboid, `+0x04` the lines' phase, `+0x34` where the box is along the corridor
//! (−1..1 of `r₂`), `+0x48` the box's cuboid, `+0x4c` the box seen this tick, `+0x50` done, `+0x54` the shadow's kind
//! (−1: the lines in it are not drawn; else they stop at the box), `+0x58` the far cuboid (−1 none), `+0x5c` 1: no
//! shadow.
//!
//! * **0**: its uid marked (the save's `0x21f204` or its count in `0x19b4a8`): 2 (off); else 1.
//! * **1**: the corridor's length `|r₂|`; its turn `R` (`0x2f2da0`, the cuboid's angles `+0x70`; places in it by its
//!   transpose, `0x2f3048` / `0x2f2a28`); `+0x34` 1. A line across the box's cuboid 2.5 up (its `r₀` by
//!   `1 + 1.5·inverse₀` of it each way, `$gp+0x2b90`, `0x2f2a00`) that meets something (`0x2e1e20`, flags 0x12): the box
//!   seen, `+0x34` where along the corridor it met (`(R·(hit − centre)).z / |r₂|`); a box 0x0fc4 met gives its move this
//!   tick (`0x325a70` = L00 `0x322058`, crate::moby::platform::ride: its ride of this moby's place, less the place,
//!   flat; `$gp+0x2b84` 1 of it).
//!   - Ratchet 0.2 over his feet in the corridor (`$gp+0x2b80`, `0x325270` = L00 `0x321858`): burnt, unless the box is
//!     seen, `+0x5c` is not 1 and his feet are from 0.75 before the box (`$gp+0x2b70` 0.5 · 1.5) to 2.5 past it
//!     (`$gp+0x2b74` 5 · 0.5) along the corridor: a hit of half his health limit (`0x1a7a08`, flags 0x10001,
//!     `0x320b58` / `0x320d78` = L00 `0x31d158` / `0x31d378`) toward `(0, 0, −3)` less the box's move.
//!   - The same point at the corridor's height in it at its far end (within 0.01 of −1), or in the far cuboid: done
//!     (once), its uid counted.
//!   - Its draw (`0x2eac38` = L00 `0x2e7718`, the frame's first list: [`draw`], `0x419878`).
//!
//! Not ported: the Clank check (`0x3df858` / `0x3df898`, Clank's moby `0x1b08bc`: in the corridor, his height snapped to
//! its centre's within 2, `0x3e0d10(1, −1)`); the port has no Clank mode. The draw's lines stopped at the box
//! (`+0x54` not −1) take their line queries (`0x2e1e20`, flags 2) in the update ([`clips`]), the draw having no
//! collision; the box may still move after it this tick.

use crate::fx::FxQuad;
use crate::hero::physics::{add, cross, dot, len, scale, set_len, sub, V3};
use crate::moby::attach::rows_of;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f9f;
/// The shuttle box (crate::moby::classes::level12::tractor::update_0fc4).
const BOX: i16 = 0x0fc4;

const CUBOID: usize = 0x00;
const PHASE: usize = 0x04;
const AT: usize = 0x34;
const BOX_CUBOID: usize = 0x48;
const SEEN: usize = 0x4c;
const DONE: usize = 0x50;
const SHADOW: usize = 0x54;
const FAR: usize = 0x58;
const BARE: usize = 0x5c;
/// The port's, past the class's pvars: the corridor's rows and centre for the draw (which has no level data), then
/// one record a line (0x10: where it stops, its w 1 when it does).
const ROWS: usize = 0x200;
const CLIPS: usize = 0x240;

/// `$gp+0x2b70` / `+0x2b74`: the shadow before and past the box; `$gp+0x2ba0` / `+0x2ba4` the draw's with `+0x54` −1.
const BEFORE: f32 = 0.5;
const PAST: f32 = 5.0;
/// `$gp+0x2b80`: how far over his feet Ratchet is looked for; `$gp+0x2b84`: the share of the box's move he takes.
const LIFT: f32 = 0.2;
const CARRY: f32 = 1.0;

/// The cuboid's rows 0..2, its centre and its angles.
fn cuboid(ctx: &Ctx, i: i32) -> Option<([V3; 3], V3, V3, [V3; 3])> {
    let c = ctx.level.cuboids.get(usize::try_from(i).ok()?)?;
    let v = |r: [f32; 4]| [r[0], r[1], r[2]];
    Some(([v(c.matrix[0]), v(c.matrix[1]), v(c.matrix[2])], v(c.matrix[3]), v(c.extra), [v(c.inverse[0]), v(c.inverse[1]), v(c.inverse[2])]))
}

/// `R·d` (`0x2f2a28` with the transpose).
fn local(r: &[V3; 3], d: V3) -> V3 { [dot(d, r[0]), dot(d, r[1]), dot(d, r[2])] }

/// `v` through the rows (`0x2f2a00`).
fn through(r: &[V3; 3], v: V3) -> V3 { add(add(scale(r[0], v[0]), scale(r[1], v[1])), scale(r[2], v[2])) }

/// Gone with its uid marked (`0x21f204[uid]` ≠ 0, or its count in `0x19b4a8`).
fn uid_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let uid = w.mobys[id].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// Done once: its uid counted.
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pv_i(DONE) != 0 { return; }
    m.set_pv_i(DONE, 1);
    let uid = m.uid;
    ctx.game.count_uid(uid);
}

/// 0x0f9f (module docs).
pub fn update_0f9f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let s = if uid_done(w, id, ctx) { 2 } else { 1 };
            w.mobys[id].set_state(s, -1);
        }
        1 => tick(w, id, ctx),
        _ => {}
    }
}

/// State 1 (module docs).
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let ci = m.pv_i(CUBOID);
    let Some((rows, o, angles, _)) = cuboid(ctx, ci) else { return };
    let Some((brows, bo, _, binv)) = cuboid(ctx, m.pv_i(BOX_CUBOID)) else { return };
    let length = len(rows[2]);
    let r = rows_of(angles);
    w.mobys[id].set_pv_f(AT, 1.0);
    let k = through(&binv, [1.5, 0.0, 0.0]);
    let mut a = through(&brows, add([1.0, 0.0, 0.0], k));
    let mut b = sub(bo, a);
    a = add(bo, a);
    a[2] += 2.5;
    b[2] += 2.5;
    w.mobys[id].set_pv_i(SEEN, 0);
    let mut carry = [0.0; 3];
    if let Some(hit) = w.coll_line(ctx, a, b, 0x12, None) {
        let m = &mut w.mobys[id];
        m.set_pv_i(SEEN, 1);
        let v = local(&r, sub(hit.point, o));
        m.set_pv_f(AT, -v[2] / -length);
        if let Some(bx) = hit.moby.filter(|&k| w.mobys.get(k).is_some_and(|x| x.o_class == BOX)) {
            let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
            let p = crate::moby::platform::ride(w, bx, pos, rot).map_or(pos, |(p, _)| p);
            let mut d = sub(p, pos);
            d[2] = 0.0;
            carry = scale(d, CARRY);
        }
    }
    let feet = ctx.hero.pos;
    let mut point = feet;
    point[2] += LIFT;
    if ctx.level.in_cuboid(ci, point) {
        let m = &w.mobys[id];
        let z = -local(&r, sub(feet, o))[2] / -length;
        let at = m.pv_f(AT);
        let burnt = z < at - BEFORE / length * 1.5 || at + PAST / length * 0.5 <= z || m.pv_i(SEEN) == 0 || m.pv_i(BARE) == 1;
        if burnt {
            if let Some(h) = ctx.hero_moby {
                let dir = sub([0.0, 0.0, -3.0], carry);
                let dmg = (ctx.game.max_health / 2) as f32;
                let t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, [dir[0], dir[1], dir[2], 0.0]);
                w.deliver_hit_any(h, &t, [0.0, 0.0, 0.0, 1.0]);
            }
        }
    }
    point[2] = o[2];
    if ctx.level.in_cuboid(ci, point) {
        let z = -local(&r, sub(point, o))[2] / -length;
        if (z + 1.0).abs() < 0.01 { finish(w, id, ctx); }
    }
    let far = w.mobys[id].pv_i(FAR);
    if far != -1 && ctx.level.in_cuboid(far, point) { finish(w, id, ctx); }
    let m = &mut w.mobys[id];
    for (k, row) in rows.iter().enumerate() { m.set_pv_v(ROWS + 0xc * k, *row); }
    m.set_pv_v(ROWS + 0x24, o);
    clips(w, id, ctx);
    w.register_draw_first(id);
}

/// The draw's lines, `t` from −1 to 1 by 0.05 along the corridor (the game's sum of steps).
fn lines() -> Vec<f32> {
    let mut v = Vec::new();
    let mut t = -1.0f32;
    while t <= 1.0 {
        v.push(t);
        t += f32::from_bits(0x3d4c_cccd);
    }
    v
}

/// The shadow's reach before and past the box, by `+0x54`.
fn shadow(kind: i32) -> (f32, f32) { if kind != -1 { (f32::from_bits(0x4013_3333), 7.0) } else { (BEFORE, PAST) } }

/// Whether line `t` is in the box's shadow (the draw's test, with `+0x34` held to −1..1 as the draw holds it).
fn shaded(w: &MobyWorld, id: usize, length: f32, t: f32) -> bool {
    let m = &w.mobys[id];
    let at = m.pv_f(AT).clamp(-1.0, 1.0);
    let (before, past) = shadow(m.pv_i(SHADOW));
    m.pv_i(SEEN) != 0 && at - before / length * 1.5 <= t && t < at + past / length * 0.5
}

/// A line's ends: `−r₁` and `r₁` at `t·r₂` from the centre.
fn ends(rows: &[V3; 3], o: V3, t: f32) -> (V3, V3) {
    let a = add(through(rows, [0.0, -1.0, t]), o);
    let b = add(through(rows, [0.0, 1.0, t]), o);
    (a, b)
}

/// The port's: the update's line queries for the draw's lines stopped at the box (`0x2e1e20(p₀, p₂, 2)`, `+0x54`
/// not −1): where each meets something.
fn clips(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let rows = [m.pv_v(ROWS), m.pv_v(ROWS + 0xc), m.pv_v(ROWS + 0x18)];
    let o = m.pv_v(ROWS + 0x24);
    let length = len(rows[2]);
    let stop = m.pv_i(SHADOW) != -1;
    for (k, t) in lines().into_iter().enumerate() {
        let mut rec = [0.0f32; 4];
        if stop && shaded(w, id, length, t) {
            let (a, b) = ends(&rows, o, t);
            if let Some(hit) = w.coll_line(ctx, a, b, 2, None) { rec = [hit.point[0], hit.point[1], hit.point[2], 1.0]; }
        }
        let m = &mut w.mobys[id];
        for (j, x) in rec.iter().enumerate() { m.set_pv_f(CLIPS + 0x10 * k + 4 * j, *x); }
    }
}

/// `$gp+0x2ae0`: the FX texture; ALPHA `$gp+0x2ae4` / `+0x2ae8` (1, 1): 0x44, blended.
const FX: usize = 6;
/// `$gp+0x2af0` / `0x1b1ae4`: the corners' ST.
const ST: [[f32; 2]; 4] = [[0.5, 0.0], [0.5, 1.0], [0.5, 0.0], [0.5, 1.0]];
/// `$gp+0x2b1c` / `+0x2b20`: the lines' colour and their sparks'.
const LINE: u32 = 0x2000_00ff;
const SPARK: u32 = 0x7000_80ff;
/// `$gp+0x2b7c`: the lines' half width; `$gp+0x2b78`: their sparks' length (of the line).
const HALF: f32 = 0.15;
const SPARK_LEN: f32 = 0.01;

/// `0x419878` (the frame's first list): `+0x34` held to −1..1, the phase on by 1/120 (wrapped at 1); for each line
/// (`t` from −1 to 1 by 0.05) but those in the box's shadow when the box is seen (from `1.5·before/|r₂|` before it to
/// `0.5·past/|r₂|` past it: 0.5 / 5, or 2.3 / 7 with `+0x54` not −1, which draws them stopped at what they meet): a
/// ribbon from `−r₁` to `r₁` (`0x2eee18` = L00 `0x2eb998`; FX 6, ALPHA 0x44) 0.15 to either side across
/// `(p₂ − p₀) × (camera − p₀)`, in 0x200000ff; then its spark, the stretch from the phase to the phase + 0.01 of it,
/// in 0x700080ff (of a stopped line, only when it starts short of the stop).
///
/// Not ported: the phase stepped by `0.5 / r₂.x` a line (`$f22`, read by nothing).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &mut w.mobys[id];
    let at = m.pv_f(AT);
    if 1.0 < at { m.set_pv_f(AT, 1.0) } else if at < -1.0 { m.set_pv_f(AT, -1.0) }
    let mut ph = m.pv_f(PHASE) + f32::from_bits(0x3c08_8889);
    if 1.0 < ph { ph -= 1.0; }
    m.set_pv_f(PHASE, ph);
    let rows = [m.pv_v(ROWS), m.pv_v(ROWS + 0xc), m.pv_v(ROWS + 0x18)];
    let o = m.pv_v(ROWS + 0x24);
    let length = len(rows[2]);
    let lerp = |a: V3, b: V3, t: f32| add(a, scale(sub(b, a), t));
    for (k, t) in lines().into_iter().enumerate() {
        let mut stopped = false;
        if shaded(w, id, length, t) {
            if w.mobys[id].pv_i(SHADOW) == -1 { continue; }
            stopped = true;
        }
        let m = &w.mobys[id];
        let (p0, full) = ends(&rows, o, t);
        let mut p2 = full;
        if stopped && m.pv_f(CLIPS + 0x10 * k + 0xc) != 0.0 { p2 = m.pv_v(CLIPS + 0x10 * k); }
        let side = set_len(cross(sub(p2, p0), sub(cam, p0)), 1.0);
        let (u, d) = (scale(side, HALF), scale(side, -HALF));
        let mut q = [add(p0, u), add(p0, d), add(p2, u), add(p2, d)];
        out.push(FxQuad { fx: FX, additive: false, corners: q, st: ST, rgba: [LINE; 4] });
        let mut short = 0.0;
        if stopped {
            short = len(sub(q[2], q[0]));
            q = [add(p0, u), add(p0, d), add(full, u), add(full, d)];
        }
        let s = [lerp(q[0], q[2], ph), lerp(q[1], q[3], ph), lerp(q[0], q[2], ph + SPARK_LEN), lerp(q[1], q[3], ph + SPARK_LEN)];
        if stopped && short <= len(sub(s[0], q[0])) { continue; }
        out.push(FxQuad { fx: FX, additive: false, corners: s, st: ST, rgba: [SPARK; 4] });
    }
}
