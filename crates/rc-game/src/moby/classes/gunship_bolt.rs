//! The bolt shot **0x05de** (L09 `0x3afc38` / L20 `0x3cd670` makes it, the decomp's `func_L09_003AE8C8`; its update
//! L09 `0x3afdf8` / L20 `0x3cd830`, `func_L09_003AEA88`): the gunship's shot (crate::moby::classes::gunship) and
//! L09 0x0d30's (crate::moby::classes::level09::patrol) and L20 0x12f8's (crate::moby::classes::level20::strafer).
//! It flies straight at its velocity for its range, hits what its way meets and bursts.
//!
//! **Made** ([`fire`]): at `pos` with its velocity `vel` a tick, update distance 0xff, mode 0x100 (its rows kept: the
//! velocity, `unit(vel × down)` with Ratchet's gravity `0x18a0e0`, and their cross), its life `range / |vel|` ticks
//! (`+0x48`, its inverse `+0x4c`), its class sound 0. **Its pvars:** `+0x00` the spin's arm (`pos × vel` at 0.25,
//! `0x3b0180`), `+0x10` / `+0x20` the arm's two ends about it, `+0x30` the velocity, `+0x40` its owner (a link),
//! `+0x50` its size, `+0x54` its damage, `+0x58` its range, `+0x5c` the spin's angle (a random start), `+0x60` the
//! hit's push, `+0x64` / `+0x68` the hit's kind bytes.
//!
//! **Each tick** ([`update_05de`]): in state 0 on by its velocity; the hit (its damage, from itself, flags 0x10001,
//! along its velocity at its push, the no-limit 5627.925, kind its class · 0x10000 with the two bytes) on what the
//! line from where it was meets (its owner ignored), else a 0.2 sphere at it: either, its class sound 0, the general
//! explosion (crate::moby::blast::explode: no hit, near 9, size its size, 5 / 4 / 10 × its size embers, billows and
//! flares, debris 1, flags 0x810001), a 1.0 sphere of the hit (flags 0x10) and gone. Then its life: out, the same
//! explosion with a 2.0 sphere of its damage, and gone; out of the level's box (2 .. 1020.25 on each axis), gone;
//! else the tail (`0x3b01f0`): the arm (× its size) turned about the velocity by the spin (`0x31df98`, q·v·q̄), its two
//! ends either side of it, the spin on by −18°, and the draw registered.
//!
//! **Its draw** ([`draw`], `0x3b02e8` on the frame's first list, registered by the tail each tick it gets there:
//! `0x2e36c0` = L00 `0x2e7718`; the same code is L16 `0x3aa620`, L18, L19 and L20 `0x3cdd20`):
//! * `0x3b03d0`: three strips of FX 0x10 between the arm's ends, each across the view (`axis × (camera − end A)`,
//!   axis the ends' way): `0x7f081840` additive or not by `rand() % 2` (half-width `randf(0.3, 0.4)` × its size),
//!   `0x1f030b1f` additive at twice that, `0x7f7f7f7f` additive at `randf(0.2, 0.25)` × its size (its width the
//!   first drawn), in that order;
//! * `0x3b0878`: a glow of FX 0xb (additive) at it facing the camera (yaw, pitch, a random roll; `0x3b0340`'s ±2
//!   square × `randf(0.35, 0.45)` × its size), `0x103080` at alpha `rand_range(0x20, 0x30)`;
//! * five times `0x3b0a20`: a spark somewhere along the arm (`randf(0, 1)` of the way, ±0.1 × its size about it), two
//!   facing quads of FX 0xb (additive or not by `rand() % 2`, `randf(0.05, 0.15)` × its size, `0x103080` then half
//!   that size white, at alpha `rand_range(0x40, 0x7f)`), and from it `0x3b0d58`'s streak of FX 0x17 a random way
//!   `randf(0.25, 0.65)` long: `0x7f7f7f7f` to `0x103080` (additive or not by `rand() % 2`), and `0x3f030b1f` to
//!   `0x030b1f` additive twice as wide, half-width `randf(0.1, 0.2)` × its size at the spark and 0.15 of it at the far
//!   end, t flipped when `rand()` is even.
//!
//! Not ported: the gravity frame `0x322810` asked at the update's start (its result unused).

use crate::fx::FxQuad;
use crate::hero::physics::{add, cross, len, scale, set_len, sub, sub_rot, V3};
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x05de;

const ARM: usize = 0x00;
const END_A: usize = 0x10;
const END_B: usize = 0x20;
const VEL: usize = 0x30;
const OWNER: usize = 0x40;
const LIFE: usize = 0x48;
const INV_LIFE: usize = 0x4c;
const SIZE: usize = 0x50;
const DAMAGE: usize = 0x54;
const RANGE: usize = 0x58;
const SPIN: usize = 0x5c;
const PUSH: usize = 0x60;
const KIND_A: usize = 0x64;
const KIND_B: usize = 0x68;
const PVARS: usize = 0x70;

/// `0x3afc38(pos, owner, damage, range, push, vel, a, b, size)` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, owner: usize, damage: f32, range: f32, push: f32, vel: V3, a: u8, b: u8, size: f32) -> usize {
    let k = w.create(CLASS);
    let down = ctx.hero.gravity.down;
    let m = &mut w.mobys[k];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    m.update_dist = 0xff;
    m.mode = mode::KEEP_ROWS;
    m.state = 0;
    m.set_pv_v(VEL, vel);
    m.pos = pos;
    m.set_pv_f(RANGE, range);
    // `0x2eb228(out, a, b)` is b × a.
    let r1 = set_len(cross(vel, down), 1.0);
    m.rows = [vel, r1, cross(vel, r1)];
    let ticks = (range / len(vel)) as i32;
    m.set_pv_i(LIFE, ticks);
    ai::set_link(m, OWNER, Some(owner));
    m.set_pv_f(DAMAGE, damage);
    m.set_pv_b(KIND_A, a);
    m.set_pv_b(KIND_B, b);
    m.set_pv_f(PUSH, push);
    m.set_pv_f(INV_LIFE, 1.0 / ticks as f32);
    m.set_pv_f(SIZE, size);
    // `0x3b0180`: the spin's arm and its start.
    let arm = set_len(cross(pos, vel), 0.25);
    m.set_pv_v(ARM, arm);
    let s = ctx.game.rng.rand_angle();
    w.mobys[k].set_pv_f(SPIN, s);
    ctx.host.play_sound(&w.mobys[k], 0, 0);
    w.rebuild(k, &*ctx.host);
    k
}

/// The general explosion of its end (module docs): `radius` 0 at a hit, 2 when its life is out.
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, radius: f32) {
    let m = &w.mobys[id];
    let s = m.pv_f(SIZE);
    let e = crate::moby::blast::Explosion {
        radius,
        damage: if radius == 0.0 { 0.0 } else { m.pv_f(DAMAGE) },
        flags: 0x81_0001,
        near: 9.0,
        size: s,
        embers: (s * 5.0) as i32,
        billows: (s * 4.0) as i32,
        flares: (s * 10.0) as i32,
        sound: -1,
        debris: 1,
        lod: -1,
        ..Default::default()
    };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
}

/// 0x05de (`0x3afdf8`, module docs).
pub fn update_05de(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    let from = w.mobys[id].pos;
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        let vel = m.pv_v(VEL);
        m.pos = add(m.pos, vel);
        let (pos, owner) = (m.pos, ai::link(m, OWNER));
        let d = set_len(vel, m.pv_f(PUSH));
        let mut t = crate::moby::hits::HitTemplate::new(m.pv_f(DAMAGE), Some(id), 0x1_0001, [d[0], d[1], d[2], f32::from_bits(0x45af_df66)]);
        t.kind = (m.o_class as u16 as u32) << 16 | (m.pv_b(KIND_B) as u32) << 8 | m.pv_b(KIND_A) as u32;
        let (coll, scene) = (ctx.coll, ctx.scene);
        let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, from, pos, crate::collision::QueryFlags(0), owner));
        if let Some(o) = line.as_ref() {
            if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
        }
        let met = line.is_some() || coll.and_then(|c| crate::collision::coll_sphere_m(c, scene, pos, f32::from_bits(0x3e4c_cccd), crate::collision::QueryFlags(0), owner)).is_some();
        if met {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            burst(w, id, ctx, 0.0);
            w.sphere_hit(scene, pos, 1.0, 0x10, owner, Some(&t));
            w.delete(id);
            return;
        }
    }
    let m = &mut w.mobys[id];
    let mut life = m.pv_i(LIFE);
    let out = countdown(&mut life);
    m.set_pv_i(LIFE, life);
    if out {
        burst(w, id, ctx, 2.0);
        w.delete(id);
        return;
    }
    const HI: f32 = f32::from_bits(0x447f_4000);
    if m.pos.iter().any(|&c| c < 2.0 || HI < c) {
        w.delete(id);
        return;
    }
    tail(w, id);
}

/// `0x3b01f0` (module docs).
fn tail(w: &mut MobyWorld, id: usize) {
    use crate::quat::{axis_angle, mul};
    let m = &mut w.mobys[id];
    let axis = set_len(m.pv_v(VEL), 1.0);
    let q = axis_angle(m.pv_f(SPIN), axis);
    let v = scale(m.pv_v(ARM), m.pv_f(SIZE));
    let r = mul(mul(q, [v[0], v[1], v[2], 0.0]), [-q[0], -q[1], -q[2], q[3]]);
    let a = [r[0], r[1], r[2]];
    m.set_pv_v(END_A, add(a, m.pos));
    m.set_pv_v(END_B, sub(m.pos, a));
    let s = sub_rot(m.pv_f(SPIN), f32::from_bits(0x3ea0_d97d));
    m.set_pv_f(SPIN, s);
    w.register_draw_first(id);
}

const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
/// `0x3b0340`'s template square (y, z), ±2.
const SQUARE: [[f32; 2]; 4] = [[-2.0, -2.0], [-2.0, 2.0], [2.0, -2.0], [2.0, 2.0]];
/// The sparks' and the glow's colour.
const ORANGE: u32 = 0x10_3080;

/// `rand() % 2 + 1` into the ALPHA register's bits 2..3: 0x44 (not additive) or 0x48 (additive).
fn coin(rng: &mut crate::rng::Rng) -> bool { rng.rand() % 2 == 1 }

/// A strip from `a` to `b` across `side` (a unit) at half-width `h` (`0x2eb5e0` of `(0, 0, ±h, 1)` by the rows at
/// either end).
#[allow(clippy::too_many_arguments)]
fn strip(out: &mut Vec<FxQuad>, fx: usize, additive: bool, a: V3, b: V3, side: V3, h: [f32; 2], rgba: [u32; 4], st: [[f32; 2]; 4]) {
    let (sa, sb) = (scale(side, h[0]), scale(side, h[1]));
    out.push(FxQuad { fx, additive, corners: [add(a, sa), sub(a, sa), add(b, sb), sub(b, sb)], st, rgba });
}

/// The rows facing the camera from `p` (`0x2eb7b0` / `0x2eb2f0` / `0x2eb978`: yaw, pitch, a random roll).
fn facing(rng: &mut crate::rng::Rng, p: V3, cam: V3) -> [V3; 3] {
    use crate::hero::physics::{atan2, dist2, euler_rows};
    let yaw = atan2(cam[0] - p[0], cam[1] - p[1]);
    let pitch = -atan2(dist2(p, cam), cam[2] - p[2]);
    euler_rows([rng.rand_angle(), pitch, yaw])
}

/// The template square × `k` on `rows` at `p` (`0x2e78f0` with the rows as its matrix).
fn square(out: &mut Vec<FxQuad>, fx: usize, additive: bool, rows: &[V3; 3], p: V3, k: f32, rgba: u32) {
    let corners = SQUARE.map(|[y, z]| add(p, add(scale(rows[1], y * k), scale(rows[2], z * k))));
    out.push(FxQuad { fx, additive, corners, st: ST, rgba: [rgba; 4] });
}

/// `0x3b02e8` (module docs): the strips, the glow and five sparks.
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    if m.pvars.len() < PVARS { return; }
    let (a, b, size, pos) = (m.pv_v(END_A), m.pv_v(END_B), m.pv_f(SIZE), m.pos);
    // `0x3b03d0`.
    let beam_add = coin(rng);
    let core = rng.randf(0.2, 0.25) * size;
    let beam = rng.randf(0.3, 0.4);
    let (halo, beam) = ((beam + beam) * size, beam * size);
    // `0x2eb228(out, x, y)` is y × x.
    let side = set_len(cross(set_len(sub(b, a), 1.0), sub(cam, a)), 1.0);
    strip(out, 0x10, beam_add, a, b, side, [beam; 2], [0x7f08_1840; 4], ST);
    strip(out, 0x10, true, a, b, side, [halo; 2], [0x1f03_0b1f; 4], ST);
    strip(out, 0x10, true, a, b, side, [core; 2], [0x7f7f_7f7f; 4], ST);
    // `0x3b0878`.
    let c = (rng.rand_range(0x20, 0x30) as u32) << 24;
    let rows = facing(rng, pos, cam);
    let k = rng.randf(0.35, 0.45) * size;
    square(out, 0xb, true, &rows, pos, k, c.wrapping_add(ORANGE));
    for _ in 0..5 { spark(rng, a, b, size, cam, out); }
}

/// `0x3b0a20` with `0x3b0d58` (module docs).
fn spark(rng: &mut crate::rng::Rng, a: V3, b: V3, size: f32, cam: V3, out: &mut Vec<FxQuad>) {
    let t = rng.randf(0.0, 1.0);
    let mut p = add(a, scale(sub(b, a), t));
    for c in &mut p { *c += rng.randf(-0.1, 0.1) * size; }
    let additive = coin(rng);
    let c = (rng.rand_range(0x40, 0x7f) as u32) << 24;
    let rows = facing(rng, p, cam);
    let k = rng.randf(0.05, 0.15) * size;
    square(out, 0xb, additive, &rows, p, k, c.wrapping_add(ORANGE));
    square(out, 0xb, additive, &rows, p, k * 0.5, c.wrapping_add(0x7f_7f7f));
    // `0x3b0d58`.
    let d: V3 = std::array::from_fn(|_| rng.randf(-1.0, 1.0));
    let l = rng.randf(0.25, 0.65);
    let end = add(p, set_len(d, l));
    let first_add = coin(rng);
    let h = rng.randf(0.1, 0.2) * size;
    let st = if rng.rand() & 1 != 0 { ST } else { [[0.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] };
    let side = set_len(cross(set_len(sub(end, p), 1.0), sub(cam, p)), 1.0);
    let far = h * 0.15;
    strip(out, 0x17, first_add, p, end, side, [h, far], [0x7f7f_7f7f, 0x7f7f_7f7f, ORANGE, ORANGE], st);
    strip(out, 0x17, true, p, end, side, [h + h, far + far], [0x3f03_0b1f, 0x3f03_0b1f, 0x03_0b1f, 0x03_0b1f], st);
}
