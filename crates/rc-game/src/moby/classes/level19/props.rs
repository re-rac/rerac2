//! Grelbin's props (L19 addresses).
//!
//! * **0x0629** ([`update_0629`], `0x381ac8`, 18 placed; the decomp's `func_L19_00380918`): a moby that stays while
//!   at least `+0x00` of its eight watched mobys (`+0x10..`) live as the classes they began as; fewer, and it goes.
//!   With a count of 0 it stops updating (mode 2) at once.
//! * **0x11c7** ([`update_11c7`], `0x421720`, 68 placed; the decomp's `func_L19_004203F0`): an ice formation.
//!   * It is gone if its uid is counted in the save (with a mission, `+0xb0` ≠ 0xff).
//!   * **Set-up**: it sits 0.1 into the ground under it (`0x2a89a8`), drawn, its rows kept (mode 0x100), state 1.
//!   * **1**: hits with the flags 0x230000 take their damage off its health (`+0x20`). Each hit flashes it white
//!     (`+0x50`) and cuts its sequence 0 in at the frame of the damage taken so far (at most 7). While it lives, a
//!     hit also plays one of its sounds 0..2.
//!   * At no health it shatters:
//!     * pieces 0..3 fly (`0x31dd30`: up 0.15 a tick, 3..6 / 3..6);
//!     * a 0x1266 is left in its place (alpha 0x50, mode 0x800, 0.65 of its class's scale, linked back at `+4`);
//!     * its sound 3 plays;
//!     * it is hidden without collision (2).
//!
//! Not ported for 0x11c7: its glow word (`$gp`-relative `0x1b1de0` in v1.01); its rows turned to the ground's
//! normal (`0x2b0408` with the line's normal `0x1bfec0`); its draw while drawn (`0x423270`); the level's
//! state table `0x21ea30` as a second gone test.

use crate::moby::{ai, shards, Ctx, MobyWorld};

pub const WATCHER: i16 = 0x0629;
pub const ICE: i16 = 0x11c7;
const ICE_LEFT: i16 = 0x1266;

pub fn update_0629(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let alive_as = |w: &MobyWorld, i: i32, c: i32| usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).is_some_and(|o| o.state < 0xfd && o.o_class as i32 == c);
    match w.mobys[id].state {
        0 => {
            for k in 0..8 {
                let i = w.mobys[id].pv_i(0x10 + 4 * k);
                let c = usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).filter(|o| o.state < 0xfd).map_or(-1, |o| o.o_class as i32);
                w.mobys[id].set_pv_i(0x30 + 4 * k, c);
            }
            let m = &mut w.mobys[id];
            m.state = 1;
            if m.pv_i(0) == 0 {
                m.state = 2;
                m.mode |= 2;
            }
        }
        1 => {
            let mut n = 0;
            for k in 0..8 {
                let (i, c) = (w.mobys[id].pv_i(0x10 + 4 * k), w.mobys[id].pv_i(0x30 + 4 * k));
                if i != -1 && c != -1 && alive_as(w, i, c) {
                    n += 1;
                } else {
                    w.mobys[id].set_pv_i(0x30 + 4 * k, -1);
                }
            }
            if n < w.mobys[id].pv_i(0) { w.delete(id); }
        }
        _ => {}
    }
}

pub fn update_11c7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.mission != 0xff && ctx.game.uid_count(m.uid) != 0 { return w.delete(id); }
    if w.mobys[id].state == 0 {
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0) - 0.1;
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        m.drawn = 1;
        m.mode |= 0x100;
        m.set_state(1, 0);
    }
    if w.mobys[id].state != 1 { return; }
    // The crack and the piece left are 0x096a's too (crate::moby::classes::level08::crystal).
    let (dmg, _) = crate::moby::classes::level08::crystal::crack(w, id, ctx, 0, 7);
    if 0.0 < dmg {
        let s = ctx.game.rng.rand_range(0, 2);
        if 0.0 < w.mobys[id].pv_f(0x20) { ctx.host.play_sound(&w.mobys[id], s, 0); }
    }
    w.clear_hit(id);
    ai::flash(&mut w.mobys[id], 0x50);
    if 0.0 < w.mobys[id].pv_f(0x20) { return; }
    shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 15, [0.0, 0.0, 0.15], 0xff);
    crate::moby::classes::level08::crystal::leave(w, id, ctx, ICE_LEFT);
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    let m = &mut w.mobys[id];
    m.set_state(2, -1);
    m.mode = (m.mode | 0x41) & 0xefff;
    m.has_collision = false;
}

pub const TELEPORTER: i16 = 0x0edc;

/// 0x0edc (`0x4131e0`, 8 placed; the decomp's v2.00 `func_US2_L19_004131DC`): a pad that takes Ratchet to the
/// level's other chunk.
/// * **1**: Ratchet standing on it (`0x322d28`) gets the prompt (owner 8): `+8` picks 0x134c, 0x134e or (anything
///   else) 0x134b. △ with that prompt showing starts the trip: 2.
/// * **2**: the screen fades out over 30 ticks. After 32 ticks:
///   * the chunk switches to `+0x00`, with music `+0x10` (`0x303530`);
///   * Ratchet is put at cuboid `+4`'s place and angles;
///   * then 3.
/// * **3**: the screen fades back in; then 1.
///
/// Not ported: Ratchet held still during the trip (his state 2), the Hacker's state test (`0x1b9a90 + 0x86` ≠ 5),
/// and the loop sound `+0x18` stopped at the end.
pub fn update_0edc(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.set_state(1, -1);
        m.set_pv_i(0x18, -1);
    }
    match w.mobys[id].state {
        1 => {
            if ctx.hero.ground_moby != Some(id) { return; }
            let text = match w.mobys[id].pv_i(8) { 0 => 0x134c, 1 => 0x134e, _ => 0x134b };
            let lease = ctx.game.prompt.request(8, text, 1);
            if lease != 0 && ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.hero.state == 0 {
                w.mobys[id].state_ticks = 0;
                ctx.game.scenes.fade_out(30.0, 0x20);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if 0x20 <= t {
                let (chunk, track, at) = (m.pv_i(0), m.pv_i(0x10), m.pv_i(4));
                ctx.switch_chunk(chunk, track);
                if let Some((pos, rot)) = ctx.level.cuboid_place(at) { ctx.hero.teleport(pos, rot); }
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => if ctx.game.scenes.fade <= 0.0 { w.mobys[id].set_state(1, -1) },
        _ => {}
    }
}

pub const DROPPER: i16 = 0x11ff;

/// 0x11ff (`0x4231b0`, 10 placed; the decomp's `func_L19_00421E80`): a platform that drops into place.
/// * **0**: its tuning: ease 0.55 a tick at most, the fall's push 0.01 and stop 0.05, the wait `+0x5c` + 60 ticks,
///   the rest 1 above its floor `+8`. Its dust's settings are kept for the tail. Then 1.
/// * **1**: once Ratchet is in cuboid `+0x00`, 2.
/// * **2**: its sound 0 on the first tick. The rumble `+0x24` grows as the wait runs out; then 3.
/// * **3**: it falls toward 20 below its floor, faster each tick; past the floor, 4.
/// * **4**: on the first tick, its sound 1 and the camera's shake (slot 0: 1 for 20 ticks). It brakes to rest 1
///   above its floor; within 0.03 of that, it rests (5).
///
/// Not ported: its tail `0x422238` (the dust and the rumble's shake by `+0x24`).
pub fn update_11ff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::ease;
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.set_pv_f(0xc, 0.0);
            m.set_pv_f(0x24, 0.0);
            m.set_pv_f(0x10, 0.55);
            m.set_pv_f(0x14, 0.01);
            m.set_pv_f(0x18, 0.05);
            m.set_pv_i(0x1c, m.pv_i(0x5c) + 60);
            m.set_pv_i(0x20, 60);
            m.set_pv_f(0x28, 1.0);
            m.set_state(1, -1);
        }
        1 => if ctx.level.in_cuboid(m.pv_i(0), ctx.hero.pos) { m.set_state(2, -1) },
        2 => {
            if m.first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0x1c);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(0x1c, t);
            if out { m.set_state(3, -1); }
            m.set_pv_f(0x24, m.pv_f(0x24) + (60 - t) as f32 * 0.25);
        }
        3 => {
            let floor = m.pv_f(8);
            if m.pos[2] < floor { m.set_state(4, -1); }
            m.set_pv_f(0x24, m.pv_f(0x24) + 1.5);
            let (mut z, mut v) = (m.pos[2], m.pv_f(0xc));
            ease(floor - 20.0, m.pv_f(0x14), 99_999.0, m.pv_f(0x10), &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(0xc, v);
        }
        4 => {
            if m.first() {
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                ctx.game.shakes.ask(0, 1.0, 20);
            }
            let m = &mut w.mobys[id];
            let (floor, rest) = (m.pv_f(8), m.pv_f(0x28));
            if (m.pos[2] - floor + rest).abs() < 0.03 { m.set_state(5, -1); }
            let (mut z, mut v) = (m.pos[2], m.pv_f(0xc));
            ease(floor - rest, 99_999.0, m.pv_f(0x18), m.pv_f(0x10), &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(0xc, v);
        }
        _ => {}
    }
}

pub const TURN_LIFT: i16 = 0x114d;

/// 0x114d's `$gp+0x2d0c..+0x2d18`: how far below its place it waits, the rise's and the turn's ticks, and how far
/// its rider may go before it goes back.
const LIFT_DROP: f32 = 4.8;
const LIFT_RISE: f32 = 30.0;
const LIFT_TURN: f32 = 20.0;
const LIFT_LEAVE: f32 = 1.6;

/// 0x114d (`0x41f8e0`, 8 placed; the decomp's `func_L19_0041E5B0`): a lift that comes up and turns round for a
/// rider (`+0x00`, set by its rider's class: the port's link).
/// * **0**: it keeps its yaw and height (`+4`, `+8`), turned half round and waiting [`LIFT_DROP`] below; then 1.
/// * **1**: once it has a rider, 2.
/// * **2**: its sound 0; it rises over [`LIFT_RISE`] ticks (cosine eased); up, its timer byte 1 and 3.
/// * **3**: it turns back to its yaw over [`LIFT_TURN`] ticks; its timer byte 2 and 4.
/// * **4**: when its rider is gone or more than [`LIFT_LEAVE`] off it (xy), its timer byte 3 and 5.
/// * **5**: its sound 1; it turns half round again; then 6.
/// * **6**: it sinks back down; its timer byte 0 and 1.
pub fn update_114d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add_rot, dist2};
    use std::f32::consts::PI;
    let ease = |t: f32| (((t * PI).cos() + 1.0) * 0.5).clamp(0.0, 1.0);
    let rider = ai::link(&w.mobys[id], 0).filter(|&r| w.mobys.get(r).is_some_and(|o| o.state < 0xfd));
    let rider_pos = rider.map(|r| w.mobys[r].pos);
    let m = &mut w.mobys[id];
    let (yaw, height, t) = (m.pv_f(4), m.pv_f(8), m.pv_f(0xc));
    match m.state {
        0 => {
            ai::set_link(m, 0, None);
            let (y, z) = (m.rot[2], m.pos[2]);
            m.set_pv_f(4, y);
            m.set_pv_f(8, z);
            m.rot[2] = add_rot(y, PI);
            m.pos[2] = z - LIFT_DROP;
            m.set_state(1, -1);
        }
        1 => if rider.is_some() {
            m.set_pv_f(0xc, 0.0);
            m.set_state(2, -1);
        },
        2 => {
            if m.first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let m = &mut w.mobys[id];
            let t = t + 1.0 / LIFT_RISE;
            if 1.0 < t {
                m.set_pv_f(0xc, 0.0);
                m.pos[2] = height;
                m.timer = 1;
                m.set_state(3, -1);
            } else {
                m.set_pv_f(0xc, t);
                m.pos[2] = height - ease(t) * LIFT_DROP;
            }
        }
        3 => {
            let t = t + 1.0 / LIFT_TURN;
            if 1.0 < t {
                m.set_pv_f(0xc, 0.0);
                m.rot[2] = yaw;
                m.timer = 2;
                m.set_state(4, -1);
            } else {
                m.set_pv_f(0xc, t);
                m.rot[2] = add_rot(yaw, ease(t) * PI);
            }
        }
        4 => if rider_pos.is_none_or(|p| LIFT_LEAVE < dist2(m.pos, p)) {
            m.timer = 3;
            ai::set_link(m, 0, None);
            m.set_pv_f(0xc, 1.0);
            m.set_state(5, -1);
        },
        5 => {
            if m.first() { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            let m = &mut w.mobys[id];
            let t = t - 1.0 / LIFT_TURN;
            if t <= 0.0 {
                m.set_pv_f(0xc, 1.0);
                m.rot[2] = add_rot(yaw, PI);
                m.set_state(6, -1);
            } else {
                m.set_pv_f(0xc, t);
                m.rot[2] = add_rot(yaw, ease(t) * PI);
            }
        }
        6 => {
            let t = t - 1.0 / LIFT_RISE;
            if t <= 0.0 {
                m.set_pv_f(0xc, 0.0);
                m.timer = 0;
                m.pos[2] = height - LIFT_DROP;
                m.set_state(1, -1);
            } else {
                m.set_pv_f(0xc, t);
                m.pos[2] = height - ease(t) * LIFT_DROP;
            }
        }
        _ => {}
    }
}

pub const REVEALED: i16 = 0x1270;

/// 0x1270 (`0x424ff0`, 5 placed; the decomp's `func_L19_00423CC0`): a moby hidden until Ratchet first enters its
/// cuboid `+0x00`, then shown for good (its uid counted, so it is shown from the start once its uid is counted or
/// gone for good). Without pvars it is deleted (`0x319058`).
/// * **0**: no collision (`+0x98` 0), hidden and not a target (mode | 0x41, & ~0x1000), its update distance 0xff, 1.
/// * **1**: Ratchet in the cuboid (`0x2acbc8`): its uid counted (crate::game::GameState::count_uid), shown — then, as
///   with its uid already gone for good (crate::game::GameState::uid_dead) or counted: mode & ~0x41 | 0x1000, its
///   class's collision (`+0x98` = the class's `+0x10`), 2.
/// * **2**: nothing.
pub fn update_1270(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::mode;
    if w.mobys[id].pvars.is_empty() { w.delete(id); return; }
    let show = |w: &mut MobyWorld, id: usize| {
        let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[id];
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = solid;
        m.set_state(2, -1);
    };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & !mode::TARGET;
            m.set_state(1, -1);
            m.update_dist = 0xff;
        }
        1 => {
            let uid = w.mobys[id].uid;
            if ctx.level.in_cuboid(w.mobys[id].pv_i(0), ctx.hero.pos) {
                ctx.game.count_uid(uid);
                show(w, id);
            }
            let gone = ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&v| v != 0);
            if !gone && ctx.game.uid_count(uid) == 0 { return; }
            show(w, id);
        }
        _ => {}
    }
}

pub const LASER: i16 = 0x11f0;

/// 0x11f0 (`0x421b48`, 6 placed; the decomp's `func_L19_00420818`, its draw `func_L19_00420B48` = `0x421e78`): a laser
/// emitter that, once set off, slides to and fro along its forward row with its beam up its up row. Its pvars:
/// `+0x00` the far end's rise, `+0x04` the slide's length, `+0x08` its speed (s), `+0x0c` the beam's length, `+0x14`
/// the spring's speed, `+0x1c` a moby that sets it off (−1 none), `+0x20` its home, `+0x30` the far end, `+0x40` its
/// hum.
/// * **0**: the far end its place + its forward row `+0x04` long (`0x2ec748` = L00 `0x2ef138`), raised `+0x00`; its
///   home its place; no hum; then as 1.
/// * **1**: its mission done, or the moby `+0x1c` past its state 0: 2.
/// * **2** / **3**: its hum (its class sound 0 looped, `0x35e400` = L00 `0x3605f0(0, 4)`) kept playing; it springs
///   (`0x31c8b8` = L00 `0x31ec88`, crate::moby::ai::spring_to: s/3600, s/3600, s/60) toward the far end (2) or its
///   home (3), within s/60 of it (`0x2ec608` = L00 `0x2ef018`) the other state.
/// * **From 2 on, every tick**: its draw ([`draw_11f0`]) on the first list (`0x2e4fd0` = L00 `0x2e7718`); a hit of
///   4 (`0x31ac40` = L00 `0x31d158`, flags 1, its class in the kind word) along its beam to the first moby met
///   (`0x2dc1b8` = L00 `0x2de878`, flags 0, itself skipped).
pub fn update_11f0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, dist, set_len};
    let hum = |w: &mut MobyWorld, ctx: &mut Ctx| {
        let h = w.mobys[id].pv_i(0x40);
        if !(h != -1 && ctx.host.sound_playing(h)) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x40, h);
        }
    };
    let state = w.mobys[id].state;
    if state == 0 {
        let m = &mut w.mobys[id];
        let mut far = add(m.pos, set_len(m.rows[0], m.pv_f(4)));
        far[2] += m.pv_f(0);
        m.set_pv_v(0x30, far);
        m.set_pv_v(0x20, m.pos);
        m.set_pv_i(0x40, -1);
    }
    match state {
        0 | 1 => {
            let m = &w.mobys[id];
            let done = m.mission != 0xff && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + m.mission as usize).is_some_and(|&b| b & 0x80 != 0);
            let woke = usize::try_from(m.pv_i(0x1c)).ok().and_then(|k| w.mobys.get(k)).is_some_and(|o| o.state != 0);
            if done || woke { w.mobys[id].set_state(2, -1); }
        }
        s @ (2 | 3) => {
            if s == 3 { hum(w, ctx); }
            let m = &mut w.mobys[id];
            let speed = m.pv_f(8);
            let goal = m.pv_v(if s == 2 { 0x30 } else { 0x20 });
            let mut v = m.pv_f(0x14);
            crate::moby::ai::spring_to(&mut m.pos, goal, speed * 0.000_277_777_8, speed * 0.000_277_777_8, speed * 0.016_666_668, &mut v);
            m.set_pv_f(0x14, v);
            if s == 2 { hum(w, ctx); }
            let m = &mut w.mobys[id];
            if dist(m.pos, goal) < speed * 0.016_666_668 { m.set_state(5 - s, -1); }
        }
        _ => {}
    }
    if w.mobys[id].state < 2 { return; }
    w.register_draw_first(id);
    let m = &w.mobys[id];
    let (from, to) = (m.pos, add(m.pos, set_len(m.rows[2], m.pv_f(0xc))));
    let mut t = crate::moby::hits::HitTemplate::new(4.0, Some(id), 1, [0.0; 4]);
    t.kind = (m.o_class as u16 as u32) << 16;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id)));
    if let Some(o) = hit {
        if let Some(target) = o.moby { w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    }
}

/// `0x421e78`: its beam up its up row `+0x0c` long (crate::moby::classes::level16::laser::beam, the same code).
pub fn draw_11f0(w: &MobyWorld, id: usize, cam: crate::hero::physics::V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::{add, set_len};
    let m = &w.mobys[id];
    super::super::level16::laser::beam(m.pos, add(m.pos, set_len(m.rows[2], m.pv_f(0xc))), cam, out);
}

pub const SCENE_HIDDEN: i16 = 0x1331;

/// 0x1331 (`0x381c48`, 1 placed; the decomp's `func_L19_00380A98`): hidden without collision nor a target during a
/// scene (`0x1a8f00` = 2: mode | 0x41, & ~0x1000, `+0x98` 0); else shown, a target, with its class's collision.
pub fn update_1331(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::mode;
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    if ctx.game.scenes.mode == crate::scene::SCENE {
        m.has_collision = false;
        m.mode = (m.mode | 0x41) & !mode::TARGET;
    } else {
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = solid;
    }
}
