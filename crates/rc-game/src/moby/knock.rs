//! The knock-and-fall block (`func_L06_0035EA68` and its starts: L06 `0x35fde0` = L08 `0x32d388` = L17 `0x320fc8`,
//! 2112 bytes; the starts L08 `0x32d250` / `0x32b648` = L17 `0x320e90` / `0x31f288`): the knock a hit gives a moby
//! that is not on the L00 framework's reaction (crate::moby::ai::flight), and its fall to the ground — level 6 / 8 /
//! 17's mortar 0x0c1a (crate::moby::classes::level17::mortar) hurt and dying. The decomp adapts it from R&C1's
//! `mine_fall`.
//!
//! **The block** (`b`, in the pvars): `+0x00` the velocity (w 1), `+0x10` the gravity (a tick²), `+0x14` the brake of
//! the speed across, `+0x18` / `+0x1c` the knock's speeds across and up (the up one also the fastest rise), `+0x20`
//! the body's radius (an int, 1/1024ths), `+0x24` options (2 the start's sequence call `0x31c210`, 4 no turn, 0x10 a
//! ledge stops it, 0x20 set when the start found it without health), `+0x28` its centre's height, `+0x2c` (s16) a
//! life of 300, `+0x2e` / `+0x2f` the smoke trail and its points' mask, `+0x30` the bounce, `+0x3c` active, `+0x3e`
//! (s16) the phase (0 rising, 1 falling, 2 on the ground), `+0x40` the heading to face (from where the knock came),
//! `+0x44` / `+0x48` the turn's speed and its most, `+0x4c` the speed across the flight keeps to, `+0x50` / `+0x54`
//! the sequence's keys to reach rising and falling (−1 none: the speed left alone), `+0x5c` the water count.
//!
//! **The start** [`aim`] + [`start`]: the knock's heading the hit's way across (`0x2f0370`); a hit marked 5627.925
//! in its w (the no-limit knocks) scales the speeds across and up by its length across and its z. Then the velocity
//! `(cos, sin)·across, up`, the heading back the other way (`0x2f0d48`(·, π)), the turn, phase and age 0, the life,
//! active; no health left: `+0x24` | 0x20 and no collision (`+0x98` 0); its sequence (`0x31c0a0` = L00 `0x31b5f0`).
//!
//! **Each tick** [`fall`] (the flags it returns: 1 it landed, 2 it touched something, 0x40 its sequence wrapped,
//! 0x100 out of the world): the gravity off the rise; the speed across eased (by the brake) to `+0x4c` (none on the
//! ground); the rise at most `+0x1c`. The ground under its centre (`0x31e048` = L00 `0x31ce20`): over water
//! (`0x2e3518` = L00 `0x2e23e0` 0) its first fall through the surface starts the water count (and a splash,
//! `0x3a2c20` = L00 `0x39f938`, whose class no level has), the ground then the one under its feet without water
//! (`0x31e0c0` = L00 `0x31ce98`); else with option 0x10 or on the ground, a step whose ground falls more than 0.25
//! stops it across. Wet: the speed × 0.7 for 3 ticks, the gravity × 0.75 for 6. Its sequence's speed (with both
//! keys): rising, to reach `+0x50` in the ticks the rise has left; falling, `+0x54` in the ticks to the ground
//! (`0x31df40` = L00 `0x31cd18`, the larger root under 60), 0 once reached; on the ground 1. It turns to its heading
//! (`0x31cbf0` = L00 `0x31c078`: 0.02, 0.3, at most `+0x48`) unless option 4. It moves; the crates in a sphere of
//! 1.1 × its radius ahead of its centre (`0x2e25e8` = L00 `0x2e14b0`, flags 0x10; `0x322148` = L00 `0x3204e8`) take
//! 1 damage with flags 0x10000 its way (`0x31e6f0` = L00 `0x31d4c8`). Faster than its radius, a line from its old
//! centre to the new (flags 0x24, `0x2df9b0` = L00 `0x2de878`) stops the centre where it meets; then a sphere of its
//! radius there (`0x2e0a10` = L00 `0x2df8d8`): a touch puts it out at the pushed centre; ground (normal z ≥ 0.707) it
//! moves into takes that part of the velocity off — the world or a moby face lands it (1, phase 2), and on the
//! ground the speed eases to 0 by the gravity —, a wall bounces it (`+0x30`). Rising, a fall turns the phase to 1.
//!
//! Not ported: the smoke trail (`+0x2e` = 1: `0x320d68` = L00 `0x31fa80`, trails from `+0x2f` random points through
//! it along its velocity; the mortar's is 0), the start's other sequence call for option 2 (`0x31c210` = L00
//! `0x31b760`; the mortar has none: the plain call is made).

use crate::collision::{coll_line_m, coll_sphere_m, QueryFlags};
use crate::hero::physics::{add, add_rot, approach, atan2, dot, len, len2, scale, set_len2, sub, unit, V3};
use crate::moby::{Ctx, MobyWorld};

const G: usize = 0x10;
const BRAKE: usize = 0x14;
const ACROSS: usize = 0x18;
const UP: usize = 0x1c;
const RADIUS: usize = 0x20;
const OPTIONS: usize = 0x24;
const LIFT: usize = 0x28;
const LIFE: usize = 0x2c;
const BOUNCE: usize = 0x30;
const ACTIVE: usize = 0x3c;
const PHASE: usize = 0x3e;
const HEADING: usize = 0x40;
const TURN_V: usize = 0x44;
const TURN_MOST: usize = 0x48;
const KEEP: usize = 0x4c;
const KEY_UP: usize = 0x50;
const KEY_DOWN: usize = 0x54;
const WET: usize = 0x5c;

/// The w that marks a hit's direction as a knock of its own (`0x45afdf66`).
const NO_LIMIT: f32 = 5627.925;

/// `0x31f288(dir, &angle, &across, &up)`: the knock's heading from the hit's direction; a no-limit one scales
/// `across` by its length across and `up` by its z.
pub fn aim(dir: [f32; 4], across: &mut f32, up: &mut f32) -> f32 {
    let a = atan2(dir[0], dir[1]);
    if dir[3] == NO_LIMIT {
        *across *= len2([dir[0], dir[1], dir[2]]);
        *up *= dir[2];
    }
    a
}

/// `0x320e90(m, b, seq, blend, frame, angle)` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, b: usize, seq: u8, blend: u8, frame: u8, angle: f32) {
    let m = &mut w.mobys[id];
    let (across, up) = (m.pv_f(b + ACROSS), m.pv_f(b + UP));
    m.set_pv_v(b, [angle.cos() * across, angle.sin() * across, up]);
    m.set_pv_f(b + 0xc, 1.0);
    m.set_pv_i(b + TURN_V, 0);
    m.set_pv_s(b + PHASE, 0);
    m.set_pv_f(b + HEADING, add_rot(angle, f32::from_bits(0x4049_0fdc)));
    m.set_pv_s(b + LIFE, 300);
    m.set_pv_b(b + ACTIVE, 1);
    // `0x320d08` = L00 `0x31fa20`: the health block.
    if m.pv_i(0) != 0 && m.pv_f(crate::moby::ai::health(m)) == 0.0 {
        m.set_pv_i(b + OPTIONS, m.pv_i(b + OPTIONS) | 0x20);
        m.has_collision = false;
    }
    ctx.host.set_anim(&w.mobys[id], seq, frame, blend);
}

/// The ground under `p` (`0x31e048(p, flags, 0.5)`) and whether the line met water (`0x2e3518` = 0: surface 0).
fn ground(ctx: &Ctx, p: V3, flags: u32) -> (f32, bool) {
    let a = [p[0], p[1], p[2] + 0.5];
    let b = [p[0], p[1], 0.01];
    let o = ctx.coll.and_then(|c| coll_line_m(c, ctx.scene, a, b, QueryFlags(flags | 2), None));
    (o.as_ref().map_or(0.0, |o| o.point[2]), o.as_ref().is_some_and(|o| o.surface_id() == 0))
}

/// `0x31df40(a, b, c)` (= L00 `0x31cd18`): the roots of `a·t² + b·t + c`, the larger first, and how many.
fn roots(a: f32, b: f32, c: f32) -> (i32, f32) {
    let disc = b * b - 4.0 * a * c;
    if disc == 0.0 { return (1, -b / (a + a)); }
    let q = disc.abs().sqrt();
    let (r1, r2) = ((-b + q) / (a + a), (-b - q) / (a + a));
    (if 0.0 < disc { 2 } else { 0 }, r1.max(r2))
}

/// `0x320fc8(m, b)` (module docs): the flags.
pub fn fall(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, b: usize) -> u32 {
    let mut flags = 0u32;
    let m = &mut w.mobys[id];
    let mut v = m.pv_v(b);
    v[2] -= m.pv_f(b + G);
    let phase = m.pv_s(b + PHASE);
    let mut s = len2(v);
    approach(if phase == 2 { 0.0 } else { m.pv_f(b + KEEP) }, m.pv_f(b + BRAKE), &mut s);
    v = set_len2(v, s);
    if m.pv_f(b + UP) < v[2] { v[2] = m.pv_f(b + UP); }
    m.set_pv_v(b, v);
    let lift = m.pv_f(b + LIFT);
    let c = add(m.pos, [0.0, 0.0, lift]);
    let (mut gz, water) = ground(ctx, c, 0);
    let m = &mut w.mobys[id];
    if water {
        if m.pv_f(b + WET) == 0.0 && gz <= c[2] && c[2] + v[2] < gz {
            // The splash at the surface (`0x3a2c20(c at gz, 3)`) makes nothing.
            m.set_pv_f(b + WET, 1.0);
        }
        gz = ground(ctx, m.pos, 0x20).0;
    } else if m.pv_i(b + OPTIONS) & 0x10 != 0 || phase == 2 {
        let next = add(m.pos, v);
        if ground(ctx, next, 0x20).0 < gz - 0.25 { m.set_pv_v(b, set_len2(v, 0.0)); }
    }
    let m = &mut w.mobys[id];
    let wet = m.pv_f(b + WET);
    if wet != 0.0 {
        if wet < 4.0 { m.set_pv_v(b, scale(m.pv_v(b), 0.7)); }
        if wet < 7.0 {
            m.set_pv_f(b + WET, wet + 1.0);
            m.set_pv_f(b + G, m.pv_f(b + G) * 0.75);
        }
    }
    if m.pos.iter().any(|&x| x < 2.0 || 1021.0 < x) { flags = 0x100; }
    if m.pv_f(b + KEY_DOWN) != -1.0 && m.pv_f(b + KEY_UP) != -1.0 {
        let now = ctx.host.frame_time(m);
        let mut speed = ctx.host.anim(m).map_or(1.0, |a| a.speed);
        match m.pv_s(b + PHASE) {
            0 => {
                let k = (m.pv_f(b + 8) / m.pv_f(b + G)).abs();
                if 2.0 < k {
                    let want = (m.pv_f(b + KEY_UP) - now) / k;
                    if 0.0 < want {
                        approach(want, 0.1, &mut speed);
                        ctx.host.set_anim_speed(m, speed);
                    }
                }
            }
            1 => {
                if m.pv_f(b + KEY_DOWN) <= now {
                    ctx.host.set_anim_speed(m, 0.0);
                } else {
                    let h = m.pv_f(b + G) * -0.5;
                    let mut t = 60.0;
                    let (n, r) = roots(h, m.pv_f(b + 8) - h, m.pos[2] - gz);
                    if 0 < n && 0.0 < r && r < t { t = r; }
                    if t < 0.0001 { t = 0.0001; }
                    let want = ((m.pv_f(b + KEY_DOWN) - now) / t).clamp(0.0, 2.0);
                    approach(want, 0.1, &mut speed);
                    ctx.host.set_anim_speed(m, speed);
                }
            }
            2 => ctx.host.set_anim_speed(m, 1.0),
            _ => {}
        }
    }
    if crate::moby::classes::props::wrapped(w, id, ctx) { flags |= 0x40; }
    let m = &mut w.mobys[id];
    if m.pv_i(b + OPTIONS) & 4 == 0 {
        let (yaw, most) = (m.pv_f(b + HEADING), m.pv_f(b + TURN_MOST));
        crate::moby::ai::turn_to(m, yaw, 0.02, 0.3, most, b + TURN_V);
    }
    let v = m.pv_v(b);
    let from = add(m.pos, [0.0, 0.0, lift]);
    m.pos = add(m.pos, v);
    let mut to = add(m.pos, [0.0, 0.0, lift]);
    let ahead = add(to, v);
    let r_int = m.pv_i(b + RADIUS) as f32;
    let dir = [v[0], v[1], v[2], m.pv_f(b + 0xc)];
    let near = ctx.scene.map(|sc| crate::collision::mobys::coll_sphere_mobys(sc, ahead, r_int * 0.001_074_218_8, QueryFlags(0x10), Some(id))).unwrap_or_default();
    for k in near {
        if !crate::moby::aim::is_crate(w, k) { continue; }
        let p = w.mobys[id].pos;
        let t = crate::moby::hits::HitTemplate::new(1.0, Some(id), 0x10000, dir);
        w.deliver_hit_any(k, &t, [p[0], p[1], p[2], 1.0]);
    }
    let q = QueryFlags(0x24);
    let r = r_int * (1.0 / 1024.0);
    if r < len(v) {
        if let Some(o) = ctx.coll.and_then(|c| coll_line_m(c, ctx.scene, from, to, q, Some(id))) { to = o.point; }
    }
    if let Some(o) = ctx.coll.and_then(|c| coll_sphere_m(c, ctx.scene, to, r, q, Some(id))) {
        flags |= 2;
        let m = &mut w.mobys[id];
        m.pos = sub(o.pushed_centre.unwrap_or(to), [0.0, 0.0, lift]);
        let n = unit(o.normal);
        let v = m.pv_v(b);
        let e = dot(v, n);
        if 0.707 <= n[2] {
            if e < 0.0 {
                if o.moby.is_none() || 0 < o.kind {
                    flags |= 1;
                    m.set_pv_s(b + PHASE, 2);
                }
                let mut v = sub(v, scale(n, e));
                if m.pv_s(b + PHASE) == 2 {
                    let mut s = len(v);
                    approach(0.0, m.pv_f(b + G), &mut s);
                    v = crate::hero::physics::set_len(v, s);
                }
                m.set_pv_v(b, v);
            }
        } else if e < 0.0 {
            let k = e * -(m.pv_f(b + BOUNCE) + 1.0);
            m.set_pv_v(b, add(v, scale(n, k)));
        }
    }
    let m = &mut w.mobys[id];
    if m.pv_s(b + PHASE) == 0 && m.pv_f(b + 8) < 0.0 { m.set_pv_s(b + PHASE, 1); }
    w.changed = true;
    flags
}
