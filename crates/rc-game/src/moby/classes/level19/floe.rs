//! Grelbin's movers (L19 addresses): the floating platform 0x0f4b and the moving scenery 0x0edb / 0x1146
//! ([`update_0edb`]), which share the burst when shot ([`burst`]) and the drift to a cuboid's place ([`drift`]), and
//! the riser 0x1271 ([`update_1271`]). All keep a platform block at `+0x20` (crate::moby::platform) for their riders.
//!
//! The floating platform **0x0f4b** ([`update_0f4b`], `0x4168b8`, 16 placed; level 20's at `0x449298`; the
//! decomp's v2.00 `func_US2_L19_004168B8`): it bobs on the water and tips under Ratchet, and once set off drifts to
//! the place of its cuboid `+0xa0` over `+0xa4` ticks; some burst when shot instead.
//!
//! **Its pvars:** `+0x20` its platform block (crate::moby::platform; `+0x5c` its flags, 1 set here), `+0x60` /
//! `+0x70` the place and angles it set out from, `+0x80` / `+0x90` its tilt (x, y; z its yaw) and the tilt's speeds,
//! `+0xa0` the cuboid it goes to (−1: it stays, without its update), `+0xa4` the trip's ticks, `+0xa8` how far it has
//! come (0..1), `+0xac` a moby whose death sets it off (−1: none, set off at once), `+0xb0` the return to Ratchet at
//! the end, `+0xb1` it does not burst, `+0xb2` carrying Ratchet, `+0xb3` its uid counted for its mission (the gate),
//! `+0xb4` a moby whose timer byte it sets when it goes (and waits for at the end; −1 none), `+0xb8` / `+0xbc` its
//! bob's speed and height, `+0xc0` / `+0xc4` its sway's phases, `+0xc8` its bob's phase, `+0xd0` the checkpoint at
//! the end and its mission done when it goes.
//!
//! * **0**: its tilt its angles, the bob's phase random (`0x3194f0` = L00 `0x31bb00`); with no cuboid it stops
//!   updating (mode 2). Already there (its mission done, with `+0xd0`; or its uid counted, with the gate) it is put at
//!   the cuboid's place and angles (its uid counted once more, with a mission) and 4; else 1. Its height its own.
//! * **1**: set off by its timer byte, a hook (`0x327970` = L00 `0x32a838`) or the moby `+0xac` gone (or none): the
//!   moby `+0xb4`'s timer byte 1, its uid counted (with a mission and the gate) and its mission done (with `+0xd0`),
//!   2. Shot (`0x31b918` = L00 `0x31dce8`, mask 0x40000) unless `+0xb1`: the same, but it bursts first — the general
//!   explosion at its place 1 up (`0x31e168` = L00 `0x320538`: flashes 4 / 2 past 9, size 1, light 15, 10 embers, 3
//!   billows, 16 flares, the shake, debris, flags 0x810001) and its pieces 0..4 (`0x324a08` = L00 `0x327498`: along
//!   its right and up at 1/12, 3..6 / 3..6) — and it is hidden without collision.
//! * **2**: it drifts toward the cuboid's place (`(1 − cos πt) / 2`; `0x2eca58` = L00 `0x2ef448`), its angles each
//!   the short way (`0x31c5b0` = L00 `0x31e980`); its height the bob's. There, 3 at once.
//! * **3**: the moby `+0xb4`'s timer byte clear (or none): with `+0xd0` the checkpoint at Ratchet (`0x357930` = L00
//!   `0x359b20`), 4.
//!
//! **Each tick after:** out of 2 its bob (0.1 · sin of its phase, 1° a tick; `0x2eca70` / `0x2ed470` = L00 `0x2ef460`
//! / `0x2efe60`), 0.2 lower under Ratchet; its height a spring (0.005, × 0.98). Past 0: with Ratchet standing on it
//! (`0x18a15c`, on the ground) the first tick drops it 1/60, its tilt springs toward his offset along its forward and
//! right rows × 0.05 (`0x31ccf8` = L00 `0x31f0c8`: `$gp+0x2b90..`: 0.03, 0.3, at most 0.1) and Ratchet is pushed
//! (`0x189f20`) by the tilt's turn about it (`0x31f9a0`, L01 `0x316c48`); else the tilt springs back to 0 (0.01,
//! 0.3, 0.1). Its sway (`0x321f40` = L00 `0x3244e8(0.0201, 0.00727, 0.00902)`, crate::moby::classes::props::sway)
//! with its tilt added. Its platform block gets the tick's move and turn (`0x320098` = L00 `0x322668`).
//!
//! Not ported: the level's state table `0x21ee94` (set-off and already-there tests); at the end with `+0xb0`, the
//! return to Ratchet (`0x1b9d00` = 0, `0x2b5a90`, L18 `0x2d0050`); the checkpoint at Ratchet's moby in camera mode
//! (`0x18c0d4`); Ratchet's second platform (`0x18a398` / `0x18a3a8`) as standing on it.

use crate::hero::physics::{add, add_rot, dot, scale, set_len, sub, turn_spring, V3};
use crate::moby::attach::rows_of;
use crate::moby::{blast, shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f4b;

const BLOCK: usize = 0x20;
const FLAGS: usize = 0x5c;
const START_POS: usize = 0x60;
const START_ROT: usize = 0x70;
const TILT: usize = 0x80;
const TILT_V: usize = 0x90;
const DEST: usize = 0xa0;
const DURATION: usize = 0xa4;
const PROGRESS: usize = 0xa8;
const TRIGGER: usize = 0xac;
const NO_BURST: usize = 0xb1;
const CARRYING: usize = 0xb2;
const GATE: usize = 0xb3;
const LINKED: usize = 0xb4;
const VEL: usize = 0xb8;
const HEIGHT: usize = 0xbc;
const SWAY_A: usize = 0xc0;
const SWAY_B: usize = 0xc4;
const PHASE: usize = 0xc8;
const NOTIFY: usize = 0xd0;

/// `$gp+0x2b80`: the tilt per unit of Ratchet's offset.
const GAIN: f32 = 0.05;
/// `$gp+0x2b90..+0x2b98` / `+0x2b9c..+0x2ba4`: the tilt's spring under Ratchet and back.
const RIDDEN: [f32; 3] = [f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a), f32::from_bits(0x3dcc_cccd)];
const FREE: [f32; 3] = [f32::from_bits(0x3c23_d70a), f32::from_bits(0x3e99_999a), f32::from_bits(0x3dcc_cccd)];

fn lerp_rot(a: f32, b: f32, t: f32) -> f32 { add_rot(a, crate::hero::physics::sub_rot(b, a) * t) }

fn mission_done(ctx: &Ctx, mission: u8) -> bool {
    mission != 0xff && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0)
}

/// The gate (`ADVANCE_GATE`): its uid counted, with a mission and `+0xb3`; then with `+0xd0` its mission done.
fn gate(w: &MobyWorld, id: usize, ctx: &mut Ctx, notify: bool) {
    let m = &w.mobys[id];
    if m.mission == 0xff || m.pv_b(GATE) == 0 { return; }
    ctx.game.count_uid(m.uid);
    if notify && m.pv_i(NOTIFY) != 0 { ctx.game.mission_done(m.mission); }
}

/// The moby `+0xb4`'s timer byte 1.
fn poke_linked(w: &mut MobyWorld, id: usize) {
    let k = w.mobys[id].pv_i(LINKED);
    poke(w, k);
}

/// 0x0f4b (module docs).
pub fn update_0f4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let phase = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            let rot = m.rot;
            m.set_pv_v(TILT, rot);
            m.set_pv_v(TILT_V, [0.0; 3]);
            m.set_pv_f(PHASE, phase);
            if m.pv_i(DEST) == -1 {
                m.mode |= 2;
                return;
            }
            m.set_pv_f(PROGRESS, 0.0);
            m.set_pv_i(FLAGS, m.pv_i(FLAGS) | 1);
            let pos = m.pos;
            m.set_pv_v(START_POS, pos);
            m.set_pv_v(START_ROT, rot);
            let there = (m.pv_i(NOTIFY) != 0 && mission_done(ctx, m.mission)) || (m.pv_b(GATE) != 0 && ctx.game.uid_count(m.uid) != 0);
            if there {
                gate(w, id, ctx, false);
                if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(DEST)) { (w.mobys[id].pos, w.mobys[id].rot) = (p, r); }
                w.mobys[id].set_state(4, -1);
            } else {
                w.mobys[id].set_state(1, -1);
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(VEL, 0.0);
            m.set_pv_f(HEIGHT, m.pos[2]);
        }
        1 => {
            let m = &w.mobys[id];
            let t = m.pv_i(TRIGGER);
            let gone = usize::try_from(t).ok().and_then(|t| w.mobys.get(t)).is_some_and(|o| o.state == 0xfe || o.state == 0xfd);
            if m.timer != 0 || m.once & 4 != 0 || t == -1 || gone {
                poke_linked(w, id);
                gate(w, id, ctx, true);
                w.mobys[id].set_state(2, -1);
            }
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x4_0000 != 0);
            if w.mobys[id].pv_b(NO_BURST) == 0 && hit.is_some() {
                poke_linked(w, id);
                burst(w, id, ctx);
                gate(w, id, ctx, true);
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            let z = w.mobys[id].pos[2];
            drift(w, id, ctx, DEST, DURATION, PROGRESS, START_POS, START_ROT);
            let m = &mut w.mobys[id];
            m.set_pv_f(HEIGHT, m.pos[2]);
            m.pos[2] = z;
            if 1.0 <= m.pv_f(PROGRESS) {
                m.set_state(3, -1);
                arrive(w, id, ctx);
            }
        }
        3 => arrive(w, id, ctx),
        _ => {}
    }
    tail(w, id, ctx, old_pos, old_rot);
}

/// The burst both drifters share (module docs): the general explosion at its place 1 up and its pieces 0..4 along its
/// right and up at 1/12; hidden without collision.
fn burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let v = set_len(add(m.rows[1], m.rows[2]), f32::from_bits(0x3daa_aaab));
    let at = add(m.pos, m.rows[2]);
    let e = blast::Explosion {
        flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, light: 15.0, embers: 10, billows: 3, flares: 0x10, sound: -1, shake: true,
        debris: 1, lod: -1, flags: 0x81_0001, ..Default::default()
    };
    blast::explode(w, ctx, Some(id), Some(at), &e);
    shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 31, v, 0xff);
    let m = &mut w.mobys[id];
    m.drawn = 0;
    m.mode |= 1;
    m.has_collision = false;
}

/// The drift both drifters share (module docs): `+progress` on by 1 / `+duration` (at most 1), the place and angles
/// from those it set out from toward the cuboid `+dest`'s by `(1 − cos πt) / 2`, each angle the short way.
#[allow(clippy::too_many_arguments)]
fn drift(w: &mut MobyWorld, id: usize, ctx: &Ctx, dest: usize, duration: usize, progress: usize, start: usize, start_rot: usize) {
    let to = ctx.level.cuboid_place(w.mobys[id].pv_i(dest));
    let m = &mut w.mobys[id];
    let p = (m.pv_f(progress) + 1.0 / m.pv_f(duration)).min(1.0);
    m.set_pv_f(progress, p);
    let blend = (1.0 - (p * std::f32::consts::PI).cos()) * 0.5;
    let Some((to, to_rot)) = to else { return };
    let (from, r) = (m.pv_v(start), m.pv_v(start_rot));
    m.pos = add(from, scale(sub(to, from), blend));
    m.rot = [lerp_rot(r[0], to_rot[0], blend), lerp_rot(r[1], to_rot[1], blend), lerp_rot(r[2], to_rot[2], blend)];
}

/// State 3 (module docs).
fn arrive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = w.mobys[id].pv_i(LINKED);
    if usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_some_and(|o| o.timer != 0) { return; }
    if w.mobys[id].pv_i(NOTIFY) != 0 { ctx.game.save_checkpoint(ctx.hero.pos, ctx.hero.rot); }
    w.mobys[id].set_state(4, -1);
}

/// The bob, the tilt and the sway (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, old_pos: V3, old_rot: V3) {
    let m = &mut w.mobys[id];
    let mut height = m.pv_f(HEIGHT);
    if m.state != 2 {
        height += m.pv_f(PHASE).sin() * 0.1;
        m.set_pv_f(PHASE, add_rot(m.pv_f(PHASE), f32::from_bits(0x3c8e_fa37)));
    }
    if m.pv_b(CARRYING) != 0 { height -= 0.2; }
    let v = (m.pv_f(VEL) + (height - m.pos[2]) * 0.005) * 0.98;
    m.set_pv_f(VEL, v);
    m.pos[2] += v;
    if m.state != 0 {
        let on = ctx.hero.air_ticks == 0 && ctx.hero.ground_moby == Some(id);
        let (mut tilt, mut tv) = (m.pv_v(TILT), m.pv_v(TILT_V));
        if on {
            if m.pv_b(CARRYING) == 0 { m.set_pv_f(VEL, m.pv_f(VEL) - 1.0 / 60.0); }
            m.set_pv_b(CARRYING, 1);
            let off = sub(ctx.hero.pos, m.pos);
            let (fwd, right) = (dot(off, m.rows[0]), dot(off, m.rows[1]));
            let old = tilt;
            let [k, d, most] = RIDDEN;
            turn_spring(-GAIN * right, k, d, most, &mut tilt[0], &mut tv[0], 0);
            turn_spring(GAIN * fwd, k, d, most, &mut tilt[1], &mut tv[1], 0);
            // `0x31f9a0`: Ratchet's place turned about it from the old tilt to the new.
            let (ro, rn) = (rows_of(old), rows_of(tilt));
            let local: V3 = std::array::from_fn(|i| dot(off, ro[i]));
            let moved: V3 = std::array::from_fn(|k| local[0] * rn[0][k] + local[1] * rn[1][k] + local[2] * rn[2][k]);
            ctx.hero.push = sub(add(moved, m.pos), ctx.hero.pos);
        } else {
            m.set_pv_b(CARRYING, 0);
            let [k, d, most] = FREE;
            turn_spring(0.0, k, d, most, &mut tilt[0], &mut tv[0], 0);
            turn_spring(0.0, k, d, most, &mut tilt[1], &mut tv[1], 0);
        }
        m.set_pv_v(TILT, tilt);
        m.set_pv_v(TILT_V, tv);
        super::super::props::sway(m, f32::from_bits(0x3ca4_6c8b), f32::from_bits(0x3bee_4bb0), f32::from_bits(0x3c13_be49), SWAY_A, SWAY_B);
        m.rot[0] = add_rot(tilt[0], m.rot[0]);
        m.rot[1] = add_rot(tilt[1], m.rot[1]);
    }
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old_pos), old_rot, rot);
    w.changed = true;
}

pub const MOVER: i16 = 0x0edb;
pub const MOVER_B: i16 = 0x1146;
/// The class whose burst also plays its sound 0 (none placed here; level 18 places one, L18 `0x43ae58`, the same code).
pub const SOUNDED: i16 = 0x10ed;

/// 0x0edb / 0x1146's pvars (module docs).
mod mv {
    pub const DEST: usize = 0x80;
    pub const DURATION: usize = 0x84;
    pub const PROGRESS: usize = 0x88;
    pub const TRIGGER: usize = 0x8c;
    pub const STAYS: usize = 0x91;
    pub const CHECK_MISSION: usize = 0x92;
    pub const NOTIFY: usize = 0x93;
    pub const LINKED: usize = 0x94;
    pub const AREA: usize = 0x98;
}

/// 0x0edb / 0x1146's set-off: its mission done (with `+0x92`) or Ratchet in its area `+0x98` (`0x327980` = L00
/// `0x32a848`).
fn mover_there(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    (m.pv_b(mv::CHECK_MISSION) != 0 && mission_done(ctx, m.mission)) || (m.pv_i(mv::AREA) != -1 && ctx.level.in_area(m.pv_i(mv::AREA), ctx.hero.pos))
}

/// 0x0edb (8 placed) and 0x1146 (4) ([`update_0edb`], `0x412b80`; the decomp's v2.00 `func_US2_L19_00412B80`): a
/// piece of the scenery that moves to the place of its cuboid `+0x80` over `+0x84` ticks once set off; the breakable
/// ones (`+0x91` clear) burst when shot instead.
/// * **0**: with no cuboid it stops (mode 2, state 4). Already there (its mission done with `+0x92`, or Ratchet in
///   its area `+0x98`) it is put at the cuboid's place and angles and stops updating — a breakable one goes — else 1.
/// * **1**: set off by its timer byte, a hook, the moby `+0x8c` gone (or none), its mission done or Ratchet in its
///   area: the moby `+0x94`'s timer byte 1, its mission done (with `+0x93`), 2. Shot (mask 0x40000) and breakable:
///   the same after the burst ([`burst`]; 0x10ed's sound 0).
/// * **2**: its sound 0 on the first tick; the drift ([`drift`]); there, 3.
/// * **3**: once the moby `+0x94`'s timer byte is clear (or none), it stops updating (mode 2).
///
/// Out of 3 its platform block (`+0x20`) gets the tick's move and turn (`0x320098` = L00 `0x322668`).
///
/// Not ported: the level's state table (`0x21e7b0` in v1.01) as a set-off; at the end with `+0x90`, the return to
/// Ratchet (`0x1b9a80` = 0, `0x2b5a90`).
pub fn update_0edb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(mv::DEST) == -1 {
                m.mode |= 2;
                m.set_state(4, -1);
                return;
            }
            m.set_pv_i(mv::PROGRESS, 0);
            m.set_pv_i(FLAGS, m.pv_i(FLAGS) | 1);
            let (pos, rot) = (m.pos, m.rot);
            m.set_pv_v(START_POS, pos);
            m.set_pv_v(START_ROT, rot);
            if mover_there(w, id, ctx) {
                if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(mv::DEST)) { (w.mobys[id].pos, w.mobys[id].rot) = (p, r); }
                w.mobys[id].mode |= 2;
                if w.mobys[id].pv_b(mv::STAYS) == 0 { return w.delete(id); }
                w.mobys[id].set_state(3, -1);
            } else {
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            let m = &w.mobys[id];
            let t = m.pv_i(mv::TRIGGER);
            let gone = usize::try_from(t).ok().and_then(|t| w.mobys.get(t)).is_some_and(|o| o.state == 0xfe || o.state == 0xfd);
            if m.timer != 0 || m.once & 4 != 0 || t == -1 || gone || mover_there(w, id, ctx) {
                let k = w.mobys[id].pv_i(mv::LINKED);
                poke(w, k);
                if w.mobys[id].pv_b(mv::NOTIFY) != 0 { ctx.game.mission_done(w.mobys[id].mission); }
                w.mobys[id].set_state(2, -1);
            }
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x4_0000 != 0);
            if w.mobys[id].pv_b(mv::STAYS) == 0 && hit.is_some() {
                let k = w.mobys[id].pv_i(mv::LINKED);
                poke(w, k);
                burst(w, id, ctx);
                if w.mobys[id].o_class == SOUNDED { ctx.host.play_sound(&w.mobys[id], 0, 0); }
                if w.mobys[id].pv_b(mv::NOTIFY) != 0 { ctx.game.mission_done(w.mobys[id].mission); }
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            let c = w.mobys[id].o_class;
            if w.mobys[id].first() && (c == MOVER || c == MOVER_B) { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            drift(w, id, ctx, mv::DEST, mv::DURATION, mv::PROGRESS, START_POS, START_ROT);
            if 1.0 <= w.mobys[id].pv_f(mv::PROGRESS) { w.mobys[id].set_state(3, -1); }
        }
        3 => {
            let k = w.mobys[id].pv_i(mv::LINKED);
            if usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_some_and(|o| o.timer != 0) { return; }
            w.mobys[id].mode |= 2;
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state < 3 {
        let (pos, rot) = (m.pos, m.rot);
        crate::moby::platform::write(m, BLOCK, sub(pos, old_pos), old_rot, rot);
    }
    w.changed = true;
}

/// Moby `k`'s timer byte 1 (−1: none).
fn poke(w: &mut MobyWorld, k: i32) {
    if let Some(o) = usize::try_from(k).ok().and_then(|k| w.mobys.get_mut(k)) { o.timer = 1; }
}

pub const RISER: i16 = 0x1271;

/// 0x1271 (`0x4251f0`, 5 placed; the decomp's v1.01 `func_L19_00423EC0`): a platform that rises 6 into place once
/// the moby `+0x7c` has been killed (its uid counted in the save).
/// * **0**: its place kept (`+0x60`). With no moby, or it gone or counted already, it stays and stops updating (mode
///   2, state 4); else updated from any distance, 6 lower, 1.
/// * **1**: the moby's uid counted: the moby `+0x80`'s timer byte 1, 2.
/// * **2**: after `+0x78` ticks (`0x2ec0d0` = L00 `0x2eeab0`), 3.
/// * **3**: it rises over `+0x70` ticks (`(cos πt + 1) / 2` of the 6 left); up, 4.
/// * **4**: once the moby `+0x80`'s timer byte is clear (or none), it stops updating (mode 2).
///
/// Out of 4 its platform block (`+0x20`) gets the tick's move and turn.
///
/// Not ported: the level's state table (`0x21ea30` in v1.01) as a set-off.
pub fn update_1271(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const START: usize = 0x60;
    const DURATION: usize = 0x70;
    const PROGRESS: usize = 0x74;
    const WAIT: usize = 0x78;
    const WATCHED: usize = 0x7c;
    const LINKED: usize = 0x80;
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let watched = usize::try_from(w.mobys[id].pv_i(WATCHED)).ok().and_then(|k| w.mobys.get(k));
    let counted = watched.is_some_and(|o| ctx.game.uid_count(o.uid) != 0);
    match w.mobys[id].state {
        0 => {
            let gone = watched.is_none_or(|o| o.state == 0xfe || o.state == 0xfd);
            let m = &mut w.mobys[id];
            m.set_pv_i(FLAGS, m.pv_i(FLAGS) | 1);
            m.set_pv_f(PROGRESS, 0.0);
            let pos = m.pos;
            m.set_pv_v(START, pos);
            if gone || counted {
                m.mode |= 2;
                m.set_state(4, -1);
            } else {
                m.update_dist = 0xff;
                m.pos[2] -= 6.0;
                m.set_state(1, -1);
            }
        }
        1 => if counted {
            let k = w.mobys[id].pv_i(LINKED);
            poke(w, k);
            w.mobys[id].set_state(2, -1);
        },
        2 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(WAIT);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(WAIT, t);
            if out { m.set_state(3, -1); }
        }
        3 => {
            let m = &mut w.mobys[id];
            let p = (m.pv_f(PROGRESS) + 1.0 / m.pv_f(DURATION)).min(1.0);
            m.set_pv_f(PROGRESS, p);
            let t = ((p * std::f32::consts::PI).cos() + 1.0) * 0.5;
            m.pos = add(m.pv_v(START), [0.0, 0.0, -6.0 * t]);
            if 1.0 <= p { m.set_state(4, -1); }
        }
        4 => {
            let k = w.mobys[id].pv_i(LINKED);
            if usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_none_or(|o| o.timer == 0) { w.mobys[id].mode |= 2; }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state < 4 {
        let (pos, rot) = (m.pos, m.rot);
        crate::moby::platform::write(m, BLOCK, sub(pos, old_pos), old_rot, rot);
    }
    w.changed = true;
}
