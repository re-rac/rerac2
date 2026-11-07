//! Smolg's path lifts (L17 addresses): platforms that ride a path once a moby they watch reaches a state.
//!
//! Both ease along their path by a timer: the share `(1 − cos(timer·π / (time·60)))/2` of the path's points (its
//! segments' count times it, the whole part the segment, the rest the way along it; the last point itself at the end:
//! `0x3c5070` / `0x3c8a20`, [`along`]). A moby they watch is "set" when it is gone (state 0xfe / 0xfd), past the table
//! or in their state word ([`set`]). Each tick their move goes into their platform block `+0x50`
//! (crate::moby::platform, `0x324660` = L00 `0x322668`), turns unchanged.
//!
//! * **0x0935** ([`update_0935`], `0x3c8c08`, 2 placed; the decomp's `func_L17_003C7680`): the carriers' dock
//!   (crate::moby::classes::level17::carrier). Its glow pulses [`pulse`]. Its pvars: `+0x90` the state its watched
//!   mobys must reach, `+0x9c` the timer (ticks, a float), `+0xa0` / `+0xa4` the first leg's moby and path, `+0xac` its
//!   state, `+0xb0` / `+0xb4` the second leg's moby and path, `+0xbc` its state, `+0xc0` a moby that must be in its
//!   state 3 for the second leg (−1 none), `+0xc4` the drop's speed, `+0xc8` the wait (ticks, a float), `+0xcc` a
//!   cuboid (−1 none).
//!   * **0** (once): 1, at the first path's start (or the second's).
//!   * **The first leg** (both its words set): **0** its moby set: the timer 0, 1. **1** the timer on to 8 s
//!     (`$gp+0x1360`), along its path; at the end the wait 0, 2. **2** while the second leg is still (0): the wait on;
//!     at 5 s (`$gp+0x1368`) 3, its moby's timer byte 0, the timer 8 s. **3** back down its path to 0, then 0. With
//!     Ratchet in the cuboid `+0xcc` while the first leg is out (1), no more than 0.05 above it and within 5
//!     (`$gp+0x1370`) across of the cuboid's middle, the timer is held (one taken off) instead of the move
//!     (`0x3c8a20`; the cuboid test `0x3234b8` = L00 `0x321858`, crate::level::LevelData::in_cuboid).
//!   * **The second leg** (both its words set): **0** with no first leg or the first at its end (2), and the moby
//!     `+0xc0` (if any) in its state 3, its moby set: the timer 0, 1. **1** the timer on to 4 s (`$gp+0x1364`), along
//!     its path; at the end the wait 0, 2. **2** the wait on; past 5 s, with Ratchet out of the cuboid `+0xcc`: 3, its
//!     sound 1 of class 0x093b (`0x363d28` = L00 `0x360698`), its moby's timer byte 0, the speed 0; Ratchet in it: the
//!     speed one less each tick. **3** it drops: the line from 0.5 above it to 20.01 below (`0x2df9b0` = L00
//!     `0x2de878`, flags 2, itself) gives the floor (0 with no hit); above it the speed on by 9.7/3600 (`$gp+0x136c`)
//!     and down by it; else on the floor, the speed 0, 0 and its sound 0 (`0x363c80` = L00 `0x3605f0`).
//! * **0x08ac / 0x093d** ([`update_08ac`], `0x3c51d0`, one each; the decomp's `func_L17_003C3C50`): a lift on two
//!   paths. Its pvars: `+0x3e` its flags (| 9 at the start), `+0x90` / `+0x94` the two legs' mobys, `+0x98` the state
//!   they must reach, `+0xa0` / `+0xa4` the two legs' paths (−1 none), `+0xa8` the timer (ticks, a float), `+0xac` the
//!   wait (ticks, a float), `+0xb0` / `+0xb4` the legs are one-way, `+0xb8` held, `+0xbc` the hold's countdown (ticks),
//!   `+0xc0` its last height change, `+0xc4` the leg (0 / 1).
//!   * **0**: the change, the hold and its countdown 0, the flags | 9, at its first path's start (or the second's); 1.
//!   * **1**: with the first leg's moby and path set, its moby set: the timer 0, leg 0, 2; else with no first path and
//!     the second leg's moby and path set, its moby set: leg 1, 2.
//!   * **2**: unless held the timer on to 8 s (`$gp+0x1200`), along the leg's path (`0x3c5070`, which keeps the height
//!     change); at the end a one-way leg loses its path (1); else the wait 0 and 4.
//!   * **3**: unless held the timer back to 0, along the leg's path; at 0, 1.
//!   * **4**: the wait on; at 2 s (`$gp+0x1204`) 3 and the leg's moby's timer byte 0.
//!   * **The hold**: the countdown on; while it runs it is held. Else coming down (its change below 0) with Ratchet
//!     under it (`0x3c4f80`: within 4 / 3 across its first / second row and 0.9..4 below it), the countdown 30 and
//!     held; else Ratchet not under it lets it go.
//! * **0x08a7** ([`update_08a7`], `0x3c48b0`, 2 placed): a lift between its path's ends that Ratchet sends with △
//!   (its own docs).
//!
//! The lifts carry Ratchet by their move (crate::moby::carry_hero_with), standing in for his read of the block.

use crate::hero::physics::{add, dist2, scale, sub, V3};
use crate::moby::{platform, Ctx, MobyWorld};

pub const DOCK: i16 = 0x0935;
pub const LIFT_A: i16 = 0x08ac;
pub const LIFT_B: i16 = 0x093d;

const PI: f32 = f32::from_bits(0x4049_0fdc);

/// `0x3c97d0` / the start of `0x3c8c08`: the glow from `0x80464646` to `0x80828282` and back every 150 ticks, by the
/// sine of the frame's share of them as a turn (`0x2f0ee8` = L00 `0x2f0000`, crate::hud::lerp_rgba).
pub fn pulse(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let a = (ctx.game.frame.rem_euclid(150) as f32 * f32::from_bits(0x3bda_740e) * 2.0) * f32::from_bits(0x4049_0fdb);
    let a = if f32::from_bits(0x4049_0fdb) < a { a - f32::from_bits(0x40c9_0fdb) } else { a };
    w.mobys[id].glow = crate::hud::lerp_rgba((a.sin() + 1.0) * 0.5, 0x8046_4646, 0x8082_8282);
}

/// The place `timer` ticks into a `time`-second ride along `pts` (module docs).
pub fn along(pts: &[[f32; 4]], timer: f32, time: f32) -> Option<V3> {
    let n = pts.len() as i32;
    if n == 0 { return None; }
    let share = (1.0 - (timer * PI / (time * 60.0)).cos()) * 0.5;
    let segs = (n - 1) as f32;
    let i = (segs * share) as i32;
    let part = segs * share - i as f32;
    let p = |k: i32| { let q = pts[k.clamp(0, n - 1) as usize]; [q[0], q[1], q[2]] };
    Some(if i == n - 1 { p(i) } else { add(scale(sub(p(i + 1), p(i)), part), p(i)) })
}

/// The moby `k` is set: past the table, gone, or in the state `want`.
fn set(w: &MobyWorld, k: i32, want: i32) -> bool {
    let Some(m) = usize::try_from(k).ok().and_then(|k| w.mobys.get(k)) else { return true };
    m.state == 0xfe || m.state == 0xfd || m.state as i32 == want
}

fn path<'a>(ctx: &'a Ctx, k: i32) -> Option<&'a [[f32; 4]]> {
    usize::try_from(k).ok().and_then(|k| ctx.level.paths.get(k)).map(|p| p.as_slice())
}

/// The moby `k`'s timer byte 0.
fn clear_timer(w: &mut MobyWorld, k: i32) {
    if let Some(m) = usize::try_from(k).ok().and_then(|k| w.mobys.get_mut(k)) { m.timer = 0; }
}

/// The path's start.
fn start(ctx: &Ctx, k: i32) -> Option<V3> { path(ctx, k).and_then(|p| p.first()).map(|q| [q[0], q[1], q[2]]) }

/// Its move into its platform block `+0x50`, the turns unchanged, and Ratchet carried (crate::moby::carry_hero_with,
/// standing in for his read of the block).
fn ride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, old: V3) {
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    platform::write(m, 0x50, sub(pos, old), rot, rot);
    let flags = m.pv_i(0x50 + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, rot[2], pos, rot[2]);
}

const DOCK_OUT: f32 = 8.0;
const DOCK_DOWN: f32 = 4.0;
const DOCK_WAIT: f32 = 5.0;
const DOCK_FALL: f32 = 9.7;
const DOCK_NEAR: f32 = 5.0;

/// `0x3c8a20(m, path, time)`: [`along`], or with Ratchet holding it the timer one back.
fn dock_along(w: &mut MobyWorld, id: usize, ctx: &Ctx, k: i32, time: f32) {
    let m = &w.mobys[id];
    let cub = m.pv_i(0xcc);
    if cub != -1 && m.pv_i(0xac) == 1 && ctx.level.in_cuboid(cub, ctx.hero.pos) && ctx.hero.pos[2] <= m.pos[2] + 0.05 {
        let mid = ctx.level.cuboids.get(cub as usize).map(|c| [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]]);
        if mid.is_some_and(|c| dist2(m.pos, c) <= DOCK_NEAR) {
            let t = m.pv_f(0x9c);
            w.mobys[id].set_pv_f(0x9c, t - 1.0);
            return;
        }
    }
    let t = w.mobys[id].pv_f(0x9c);
    if let Some(p) = path(ctx, k).and_then(|p| along(p, t, time)) { w.mobys[id].pos = p; }
}

/// 0x0935 (module docs).
pub fn update_0935(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pulse(w, id, ctx);
    let old = w.mobys[id].pos;
    if w.mobys[id].state == 0 {
        w.mobys[id].state = 1;
        let m = &w.mobys[id];
        let (p1, p2) = (m.pv_i(0xa4), m.pv_i(0xb4));
        let at = if p1 != -1 { start(ctx, p1) } else if p2 != -1 { start(ctx, p2) } else { None };
        if let Some(p) = at { w.mobys[id].pos = p; }
    }
    let (want, t1, p1) = { let m = &w.mobys[id]; (m.pv_i(0x90), m.pv_i(0xa0), m.pv_i(0xa4)) };
    if t1 != -1 && p1 != -1 {
        match w.mobys[id].pv_i(0xac) {
            0 => if set(w, t1, want) {
                let m = &mut w.mobys[id];
                m.set_pv_f(0x9c, 0.0);
                m.set_pv_i(0xac, 1);
            },
            1 => {
                let m = &mut w.mobys[id];
                m.set_pv_f(0x9c, (m.pv_f(0x9c) + 1.0).min(DOCK_OUT * 60.0));
                dock_along(w, id, ctx, p1, DOCK_OUT);
                let m = &mut w.mobys[id];
                if DOCK_OUT * 60.0 <= m.pv_f(0x9c) {
                    m.set_pv_f(0xc8, 0.0);
                    m.set_pv_i(0xac, 2);
                }
            }
            2 => if w.mobys[id].pv_i(0xbc) == 0 {
                let m = &mut w.mobys[id];
                let k = m.pv_f(0xc8) + 1.0;
                m.set_pv_f(0xc8, k);
                if DOCK_WAIT * 60.0 <= k {
                    m.set_pv_i(0xac, 3);
                    m.set_pv_f(0x9c, DOCK_OUT * 60.0);
                    clear_timer(w, t1);
                }
            },
            3 => {
                let m = &mut w.mobys[id];
                m.set_pv_f(0x9c, (m.pv_f(0x9c) - 1.0).max(0.0));
                dock_along(w, id, ctx, p1, DOCK_OUT);
                if w.mobys[id].pv_f(0x9c) <= 0.0 { w.mobys[id].set_pv_i(0xac, 0); }
            }
            _ => {}
        }
    }
    let (t2, p2) = { let m = &w.mobys[id]; (m.pv_i(0xb0), m.pv_i(0xb4)) };
    if t2 != -1 && p2 != -1 {
        match w.mobys[id].pv_i(0xbc) {
            0 => {
                let m = &w.mobys[id];
                let mut go = t1 == -1 || m.pv_i(0xac) == 2;
                let gate = m.pv_i(0xc0);
                if go && gate != -1 && usize::try_from(gate).ok().and_then(|g| w.mobys.get(g)).is_none_or(|g| g.state != 3) { go = false; }
                if go && set(w, t2, want) {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(0x9c, 0.0);
                    m.set_pv_i(0xbc, 1);
                }
            }
            1 => {
                let m = &mut w.mobys[id];
                m.set_pv_f(0x9c, (m.pv_f(0x9c) + 1.0).min(DOCK_DOWN * 60.0));
                dock_along(w, id, ctx, p2, DOCK_DOWN);
                let m = &mut w.mobys[id];
                if DOCK_DOWN * 60.0 <= m.pv_f(0x9c) {
                    m.set_pv_f(0xc8, 0.0);
                    m.set_pv_i(0xbc, 2);
                }
            }
            2 => {
                let m = &mut w.mobys[id];
                let k = m.pv_f(0xc8) + 1.0;
                m.set_pv_f(0xc8, k);
                if DOCK_WAIT * 60.0 <= k {
                    if !ctx.level.in_cuboid(m.pv_i(0xcc), ctx.hero.pos) {
                        m.set_pv_i(0xbc, 3);
                        ctx.host.play_sound_of(&w.mobys[id], 0x093b, 1, 0);
                        clear_timer(w, t2);
                        w.mobys[id].set_pv_f(0xc4, 0.0);
                    } else {
                        m.set_pv_f(0xc4, m.pv_f(0xc4) - 1.0);
                    }
                }
            }
            3 => {
                let pos = w.mobys[id].pos;
                let below = [pos[0], pos[1], pos[2] - f32::from_bits(0x41a0_147b)];
                let above = [pos[0], pos[1], pos[2] + 0.5];
                let floor = w.coll_line(ctx, above, below, 2, Some(id)).map_or(0.0, |h| h.point[2]);
                let m = &mut w.mobys[id];
                if floor < m.pos[2] {
                    let s = m.pv_f(0xc4) + DOCK_FALL * f32::from_bits(0x3991_a2b4);
                    m.set_pv_f(0xc4, s);
                    m.pos[2] -= s;
                } else {
                    m.set_pv_f(0xc4, 0.0);
                    m.pos[2] = floor;
                    m.set_pv_i(0xbc, 0);
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                }
            }
            _ => {}
        }
    }
    ride(w, id, ctx, old);
}

const LIFT_TIME: f32 = 8.0;
const LIFT_WAIT: f32 = 2.0;
const HOLD: i32 = 30;

/// `0x3c4f80(m, p)`: `p` under it — within 4 / 3 across its first / second row and 0.9..4 below it.
fn under(m: &crate::moby::Moby, p: V3) -> bool {
    let d = sub(p, m.pos);
    let l: V3 = std::array::from_fn(|i| crate::hero::physics::dot(m.rows[i], d));
    l[0].abs() < 4.0 && l[1].abs() < 3.0 && (p[2] - m.pos[2]).abs() < 4.0 && f32::from_bits(0x3f66_6666) < m.pos[2] - p[2]
}

/// `0x3c5070(m, path, old)`: [`along`] by the timer `+0xa8`, the height change kept at `+0xc0`.
fn lift_along(w: &mut MobyWorld, id: usize, ctx: &Ctx, old: V3) {
    let m = &w.mobys[id];
    let k = if m.pv_i(0xc4) != 0 { m.pv_i(0xa4) } else { m.pv_i(0xa0) };
    let t = m.pv_f(0xa8);
    if let Some(p) = path(ctx, k).and_then(|p| along(p, t, LIFT_TIME)) { w.mobys[id].pos = p; }
    let m = &mut w.mobys[id];
    m.set_pv_f(0xc0, m.pos[2] - old[2]);
}

/// 0x08ac / 0x093d (module docs).
pub fn update_08ac(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let old = w.mobys[id].pos;
    {
        let m = &mut w.mobys[id];
        let mut c = m.pv_i(0xbc);
        crate::moby::countdown(&mut c);
        m.set_pv_i(0xbc, c);
    }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(0xc0, 0.0);
            m.set_pv_i(0xb8, 0);
            m.set_pv_i(0xbc, 0);
            m.state = 1;
            m.set_pv_s(0x3e, m.pv_s(0x3e) | 9);
            let (a, b) = (m.pv_i(0xa0), m.pv_i(0xa4));
            let at = if a != -1 { start(ctx, a) } else if b != -1 { start(ctx, b) } else { None };
            if let Some(p) = at { w.mobys[id].pos = p; }
        }
        1 => {
            let m = &w.mobys[id];
            let (s0, s1, want, d0, d1) = (m.pv_i(0x90), m.pv_i(0x94), m.pv_i(0x98), m.pv_i(0xa0), m.pv_i(0xa4));
            let leg = if s0 != -1 && d0 != -1 && set(w, s0, want) {
                Some(0)
            } else if d0 == -1 && s1 != -1 && d1 != -1 && set(w, s1, want) {
                Some(1)
            } else {
                None
            };
            if let Some(leg) = leg {
                let m = &mut w.mobys[id];
                m.set_pv_f(0xa8, 0.0);
                m.set_pv_i(0xc4, leg);
                m.state = 2;
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_f(0xa8);
            if m.pv_i(0xb8) == 0 { t += 1.0; }
            m.set_pv_f(0xa8, t.min(LIFT_TIME * 60.0));
            lift_along(w, id, ctx, old);
            let m = &mut w.mobys[id];
            if LIFT_TIME * 60.0 <= m.pv_f(0xa8) {
                let leg = m.pv_i(0xc4);
                if m.pv_i(0xb0) != 0 && leg == 0 {
                    m.set_pv_i(0xa0, -1);
                    m.state = 1;
                } else if m.pv_i(0xb4) != 0 && leg == 1 {
                    m.set_pv_i(0xa4, -1);
                    m.state = 1;
                } else {
                    m.state = 4;
                    m.set_pv_f(0xac, 0.0);
                }
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_f(0xa8);
            if m.pv_i(0xb8) == 0 { t -= 1.0; }
            m.set_pv_f(0xa8, t.max(0.0));
            lift_along(w, id, ctx, old);
            if w.mobys[id].pv_f(0xa8) <= 0.0 { w.mobys[id].state = 1; }
        }
        4 => {
            let m = &mut w.mobys[id];
            let k = m.pv_f(0xac) + 1.0;
            m.set_pv_f(0xac, k);
            if LIFT_WAIT * 60.0 <= k {
                m.state = 3;
                let leg = m.pv_i(0xc4);
                let who = match leg { 0 => Some(m.pv_i(0x90)), 1 => Some(m.pv_i(0x94)), _ => None };
                if let Some(k) = who { clear_timer(w, k); }
            }
        }
        _ => {}
    }
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    if m.pv_i(0xbc) != 0 {
        m.set_pv_i(0xb8, 1);
    } else if m.pv_f(0xc0) < 0.0 && under(m, hero) {
        if m.pv_i(0xbc) == 0 { m.set_pv_i(0xbc, HOLD); }
        m.set_pv_i(0xb8, 1);
    } else if !under(m, hero) {
        m.set_pv_i(0xb8, 0);
    }
    ride(w, id, ctx, old);
}

pub const ELEVATOR: i16 = 0x08a7;

/// 0x08a7's speed along its path (`$gp+0x11e0`, a second) and its camera tweak's distance, pivot height and pitch
/// (`$gp+0x11e8` / `+0x11ec` / `+0x11f4`).
const ELEV_SPEED: f32 = 4.0;
const ELEV_CAM: [f32; 3] = [f32::from_bits(0x3f59_999a), 0.0, f32::from_bits(0x3e99_999a)];

/// The scripted walk under way (`0x2d5510` = L00 `0x2d4490` 1: his state 0x62..0x63, or asked for this tick, which
/// the original's `0x2d54c0` sets at once).
fn walking(ctx: &Ctx) -> bool {
    (0x62..=0x63).contains(&ctx.hero.state) || ctx.hero.state_req.is_some_and(|r| r.id == 0x62)
}

/// Its sound `+0xb4` stopped (when playing) and none.
fn elev_stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(0xb4);
    if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(0xb4, -1);
}

/// Its camera tweak let go (`0x2d85d8` = L00 `0x2d7508`).
fn elev_release(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pv_i(0xd4) != 0 {
        if let Ok(s) = usize::try_from(m.pv_i(0xd8)) { ctx.game.cam_tweaks.release(s); }
        m.set_pv_i(0xd4, 0);
    }
}

/// The way from point `a` to `b` at its speed a tick, kept at `+0xe0`.
fn elev_way(w: &mut MobyWorld, id: usize, pts: &[[f32; 4]], a: usize, b: usize) {
    let (Some(p), Some(q)) = (pts.get(a), pts.get(b)) else { return };
    let v = crate::hero::physics::set_len([p[0] - q[0], p[1] - q[1], p[2] - q[2]], ELEV_SPEED * f32::from_bits(0x3c88_8889));
    w.mobys[id].set_pv_v(0xe0, v);
}

/// The ride to the height `z` (states 1 and 3): its sound 0 (mode 4) kept playing; Ratchet on it (crate::moby::ai::stands_on):
/// its camera tweak of priority 5 (`0x2d8538` = L00 `0x2d7468`) once; its height eased to `z` (`0x320060` = L00
/// `0x31ed78`, crate::hero::physics::ease: 1/516 up, 1/258 braking, at most 0.5). True within 0.2 of it.
fn elev_ride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, z: f32) -> bool {
    if w.mobys[id].first() { w.mobys[id].set_pv_f(0x98, 0.0); }
    let h = w.mobys[id].pv_i(0xb4);
    if h < 0 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(0xb4, h);
    }
    if crate::moby::ai::stands_on(ctx, id) && w.mobys[id].pv_i(0xd4) == 0 {
        if let Some(s) = ctx.game.cam_tweaks.request(5) {
            let t = &mut ctx.game.cam_tweaks.slots[s];
            (t.dist, t.pivot_h, t.pitch) = (ELEV_CAM[0], ELEV_CAM[1], ELEV_CAM[2]);
            let m = &mut w.mobys[id];
            m.set_pv_i(0xd8, s as i32);
            m.set_pv_i(0xd4, 1);
        }
    }
    let m = &mut w.mobys[id];
    let (mut y, mut v) = (m.pos[2], m.pv_f(0x98));
    crate::hero::physics::ease(z, f32::from_bits(0x3afe_dcbb), f32::from_bits(0x3b7e_dcbb), 0.5, &mut y, &mut v);
    m.pos[2] = y;
    m.set_pv_f(0x98, v);
    (m.pos[2] - z).abs() < 0.2
}

/// 0x08a7 (`0x3c48b0`, 2 placed; the decomp's `func_US2_L17_003C48B0`): a lift between the ends of its path (`+0xa4`)
/// that Ratchet sends with △. Its pvars: `+0x50` its platform block, `+0x98` its climb speed, `+0xac` at the last
/// point (a halfword; state 2 counts it down), `+0xb4` its sound, `+0xc8` the point it heads for, `+0xcc` it works
/// only hooked, `+0xd4` / `+0xd8` its camera tweak held and its slot, `+0xe0` its way a tick (kept, unused here).
/// * **0**: its sound none. Ratchet standing on it (`0x18a15c`, `0x18a160`) and (with `+0xcc`) it hooked
///   (`0x32d828` = L00 `0x32a838`): the prompt 0x12ec (`0x329d50(8, 0x12ec, 1)`, crate::prompt); △ sends it — from the
///   first point 4, its point 1; from the last 5, its point the one before the last; its way toward it. Else with
///   Ratchet more than 10 above it at the last point it goes to the first; more than 10 below it at the first, to the
///   last (0.2 above it).
/// * **4** / **5**: on its first tick he walks to its place keeping its yaw (`0x2d54c0` = L00 `0x2d4440`,
///   crate::hero::Hero::script_walk_to); once he is there (or stopped) 1 / 3.
/// * **1** / **3**: the ride to the last / first point's height; there 0, its sound stopped, its camera tweak let go,
///   and (1) at the other end / (3) at the first.
/// * **2**: its sound stopped; at the end of the countdown 5, the point before.
/// * Each tick, Ratchet off it, its camera tweak let go; its move into its platform block and Ratchet carried.
///
/// Not ported: on the ride with Ratchet on it, the words `0x18a032` / `0x18a034` (4 each) and `0x3c47d0` (its path
/// `+0xd0` of 7 points rebuilt from the table `0x296f80` through its frame, its index into `0x18c31e`), the hero
/// camera's path, which the port's camera lacks.
pub fn update_08a7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let old = w.mobys[id].pos;
    let level = ctx.level;
    let Some(pts) = usize::try_from(w.mobys[id].pv_i(0xa4)).ok().and_then(|k| level.paths.get(k)).map(|p| p.as_slice()).filter(|p| 1 < p.len()) else { return };
    let n = pts.len();
    let pt = |k: usize| { let q = pts[k]; [q[0], q[1], q[2]] };
    'states: {
        match w.mobys[id].state {
            0 => {
                w.mobys[id].set_pv_i(0xb4, -1);
                if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 {
                    if w.mobys[id].pv_i(0xcc) != 0 && w.mobys[id].once & 4 == 0 { break 'states; }
                    ctx.game.prompt.request(8, 0x12ec, 1);
                    if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
                        if w.mobys[id].pv_s(0xac) == 0 {
                            w.mobys[id].set_state(4, -1);
                            w.mobys[id].set_pv_i(0xc8, 1);
                            elev_way(w, id, pts, 1, 0);
                        } else {
                            w.mobys[id].set_state(5, -1);
                            w.mobys[id].set_pv_i(0xc8, n as i32 - 2);
                            elev_way(w, id, pts, n - 2, n - 1);
                        }
                        break 'states;
                    }
                }
                let hz = ctx.hero.pos[2];
                let m = &mut w.mobys[id];
                let at_last = m.pv_s(0xac) != 0;
                if m.pos[2] + 10.0 < hz && at_last {
                    m.pos = pt(0);
                    m.set_pv_s(0xac, 0);
                } else if hz < m.pos[2] - 10.0 && !at_last {
                    m.pos = pt(n - 1);
                    m.set_pv_s(0xac, 1);
                    m.pos[2] += f32::from_bits(0x3e4c_cccd);
                }
            }
            4 | 5 => {
                if w.mobys[id].first() {
                    let m = &w.mobys[id];
                    ctx.hero.script_walk_to(m.rot[2], m.pos, false);
                }
                if walking(ctx) { break 'states; }
                let to = if w.mobys[id].state == 4 { 1 } else { 3 };
                w.mobys[id].set_state(to, -1);
            }
            1 => if elev_ride(w, id, ctx, pts[n - 1][2]) {
                w.mobys[id].set_state(0, -1);
                elev_stop(w, id, ctx);
                elev_release(w, id, ctx);
                let m = &mut w.mobys[id];
                m.set_pv_s(0xac, (m.pv_s(0xac) == 0) as i16);
            },
            2 => {
                elev_stop(w, id, ctx);
                let m = &mut w.mobys[id];
                let mut k = m.pv_s(0xac) as i32;
                let done = crate::moby::countdown(&mut k);
                m.set_pv_s(0xac, k as i16);
                if done {
                    m.set_state(5, -1);
                    let p = m.pv_i(0xc8) - 1;
                    m.set_pv_i(0xc8, p);
                    if let Ok(p) = usize::try_from(p) { elev_way(w, id, pts, p, p + 1); }
                }
            }
            3 => if elev_ride(w, id, ctx, pts[0][2]) {
                w.mobys[id].set_pv_s(0xac, 0);
                w.mobys[id].set_state(0, -1);
                elev_release(w, id, ctx);
                elev_stop(w, id, ctx);
            },
            _ => {}
        }
    }
    if !crate::moby::ai::stands_on(ctx, id) { elev_release(w, id, ctx); }
    ride(w, id, ctx, old);
}
