//! Tabora's burrower **0x0fff** (L08 `0x424f78`, 296 placed; the decomp's `func_US2_L08_00424F74`): it lies under
//! the sand until Ratchet comes near, rises, hovers, patrols, hunts and dives at him to bite. Tabora's copy of the
//! Oozla swamp flyer's code (crate::moby::classes::flyer, L01 `0x3bfe10`) with its pvars 0x80 lower, its timers
//! halves and its own target search (crate::moby::classes::level08::tabora_ai), on the framework
//! (crate::moby::ai): health `+0x20`, reaction `+0x50` (a hurt goes to state 9), target block `+0x100`.
//!
//! **Its pvars:** `+0x150` home (`+0x15c` its hover offset, 0..1), `+0x160` the goal, `+0x16c` the hover's turn,
//! `+0x170` the velocity, `+0x180` / `+0x184` / `+0x1ac` the mobys its wake-ups watch, `+0x188` / `+0x189` its update
//! distance and the wander radius, `+0x18a` how it starts (0 under the sand, 1 hovering, 2 launched along `+0xd0`),
//! `+0x18b` what wakes it (0 Ratchet in its first volumes, 1 the moby `+0x180` past its state 2, 2 a poke, 3 / 4 the
//! moby `+0x1ac` / `+0x180` gone, 5 the moby `+0x184` past its state 2 or gone; a poke with bit 1 always), `+0x18c`
//! it is one of the group's attackers (the shared word `+0x1cc`, at most 3), `+0x18d` the wake-up's range,
//! `+0x190` / `+0x194` / `+0x198` its first volumes (path, sphere, cuboid) and `+0x19c` / `+0x1a0` / `+0x1a4` the
//! hunt's, `+0x1a8` the path it keeps its goals in, `+0x1b0` the floor under it, `+0x1b4` its hover height,
//! `+0x1b8` / `+0x1ba` / `+0x1bc` / `+0x1be` its timer, alarm, wait after a bite and knock ticks (s16),
//! `+0x1c0` its speed, `+0x1c4` / `+0x1c8` the yaw's and pitch's spring speeds, `+0x1d0` the floor test's top,
//! `+0x1d4` the lowest floor, `+0x1d8` / `+0x1dc` the groups it alarms.
//!
//! **States** (`+0x20`): 0 the set-up ([`setup`], `0x4249e8`; gone at once when the moby `+0x184` is marked gone and
//! so is it). 1 under the sand until woken ([`wake`]), then 3 a wait of 0..180 and 2 rising to its hover height
//! (sub 0, then 5) or sinking back (sub 1, then 1), sand puffs about it early in its sequence (crate::moby::classes::level08::tabora_ai::puffs,
//! `0x423798`). 4 launched upward until it meets something (but a 0x0945) or stalls, then 5. 5 hovering on a weave
//! for 180..360, then 6 patrolling (new goals about home, sub 0 / 1; home and down when nothing is about, sub 2). In
//! the hunt's volumes, 1 in 9 a tick it hunts (7): goals near Ratchet; within 12 of him with fewer than 3 of its
//! group at him and its wait over, it dives (8): to him, the bite (sequence 4: frames 19.5..29, a sphere 0.5 at
//! joint 1, its damage `+0x34`), a 120-tick hold and again while he is still about; else away. 9 the knock (the
//! framework's flight; 360 ticks at most), then hunting. 10 dying: its class sound 1, its sparkles (crate::moby::classes::level08::tabora_ai::sparkles,
//! `0x423b80`), the explosion (`0x32e558` = L00 `0x320538`), its bolts (flag 0x1000 for one launched) and its uid
//! marked; deleted.
//!
//! **Each tick** ([`head`], `0x424ea8`; [`tail`], `0x424df8`): the floor ([`floor`], `0x423f08`), half its class's
//! size, the frame's damage (`0x336b18` = L00 `0x327e38`, crate::moby::ai::take_damage): hurt, it leaves the
//! attackers and alarms its groups ([`alarm`], `0x4240e0`), 330 knock ticks (or, with no health left, the
//! reaction's spin 0.838 and option 2); at the end the alarm (a poke: 300..360 once under 200; counted down while
//! nothing is targeted), its ticks, its timers, and from state 2 its shadow's slab (the floor, `0x32b038`). Out of
//! 20..1003 on any axis it is deleted. Ratchet in the state 0x6f hides it; it shows again from state 4.
//!
//! **The steering** ([`steer`], `0x424618(turn accel, brake, most, accel, slow, top, height, stop)`): the speed
//! eases toward the goal across (braking within ten ticks of it, at most `top`/60), the facing springs toward it
//! (`0x3292d8` = L00 `0x31c078`, crate::moby::ai::turn_to), the height to the floor ten ticks ahead plus `height`
//! (Ratchet's when he is within 5 and lower), the velocity along the facing at that pitch (`0x333c00` = L00
//! `0x325100`), the move against the world 0.4 up (`0x3297f0` = L00 `0x31c618`, crate::moby::slide), the pitch sprung
//! to it (`0x329170`, [`tabora_ai::spring_angle`]); blocked (flags 7), the facing turns three times as fast. The
//! flags: 8 at the goal, | the slide's.
//!
//! The engine functions with the port's: `0x339370` = L00 `0x32a7c8` (crate::moby::Moby::first), `0x3285f0` = L00
//! `0x31b5f0` (crate::moby::classes::props::set_seq), `0x2fa6e0` = L00 `0x2eeae0` (the s16 countdown),
//! `0x31e880` = L00 `0x311880` (the frame time), `0x31f7c8` = L00 `0x3127c8` (a joint), `0x32a618` = L00 `0x31d180`
//! (crate::moby::ai::attack_sphere), `0x337860` = L00 `0x328b80` (crate::moby::ai::flight), `0x327dc0` = L00
//! `0x31adc0` (crate::moby::bolts::drop), `0x32c420` = L00 `0x31ed78` (crate::hero::physics::ease), `0x2eb6d0` =
//! L00 `0x2df8d8` (the sphere test), `0x2ea670` = L00 `0x2de878` (the line test), `0x32a160` = L00 `0x31ce20`
//! (crate::moby::ai::ground_z), `0x329bb0` = L00 `0x31c970` (crate::moby::MobyWorld::poke_group).
//!
//! Not ported: the level's gate `0x1ca080` the update starts with (a state word, a mode and two heights: with a mode
//! set, a burrower above or below the heights for the state is not updated; nothing the port has sets it), the
//! enemies' big-heads cheat on its joint block `+0x1e0` (`0x335a60`), the wake-up by the moby `+0x180` when there is
//! none (the arena statistics `0x2b72f0`), the path's length (`0x351498`), the five ground marks of its death
//! (`0x423a30` → `0x348d78` = L01 `0x330218`).

use super::tabora_ai::{self, find_target, floor_at, volume_reach, Volumes};
use crate::hero::physics::{add, add_rot, atan2, dist2, ease, len2, set_len, sub, V3};
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, bolts, countdown, Ctx, MobyWorld};
use std::f32::consts::PI;

pub const CLASS: i16 = 0x0fff;

const HEALTH: usize = 0x20;
const REACT: usize = 0x50;
const T_BLOCK: usize = 0x100;
const T_AIM: usize = 0x120;
const T_RESULT: usize = 0x144;
const HOME: usize = 0x150;
const GOAL: usize = 0x160;
const TURN: usize = 0x16c;
const VEL: usize = 0x170;
const START: usize = 0x18a;
const WAKE: usize = 0x18b;
const ATTACKING: usize = 0x18c;
const FLOOR: usize = 0x1b0;
const HOVER: usize = 0x1b4;
const TIMER: usize = 0x1b8;
const ALARM: usize = 0x1ba;
const WAIT: usize = 0x1bc;
const KNOCK: usize = 0x1be;
const SPEED: usize = 0x1c0;
const ATTACKERS: usize = 0x1cc;
/// `0x1a8f20`: the zero vector.
const ZERO: V3 = [0.0; 3];

/// The steering's arguments (`0x424618`).
#[derive(Clone, Copy)]
struct Steer {
    turn_a: f32,
    turn_b: f32,
    turn_most: f32,
    accel: f32,
    slow: f32,
    top: f32,
    height: f32,
    stop: f32,
}

fn s16_down(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(off) as i32;
    let r = countdown(&mut t);
    m.set_pv_s(off, t as i16);
    r
}

fn first_vols(w: &MobyWorld, id: usize) -> Volumes {
    let m = &w.mobys[id];
    Volumes(m.pv_i(0x190), m.pv_i(0x198), m.pv_i(0x194), -1)
}

fn hunt_vols(w: &MobyWorld, id: usize) -> Volumes {
    let m = &w.mobys[id];
    Volumes(m.pv_i(0x19c), m.pv_i(0x1a4), m.pv_i(0x1a0), -1)
}

/// The hunt's search: its draw distance across, `near` the wake-up's range (states 7 and 8) or 0.
fn search(w: &mut MobyWorld, id: usize, ctx: &Ctx, near: bool) -> i32 {
    let m = &w.mobys[id];
    let (range, n) = (m.draw_dist as f32, if near { m.pv_b(0x18d) as f32 } else { 0.0 });
    let v = hunt_vols(w, id);
    find_target(w, id, ctx, T_BLOCK, range, n, v)
}

/// Ratchet away for a dive: his state 0x31 or the movement groups 0x11 / 0x12.
fn hero_away(ctx: &Ctx) -> bool { ctx.hero.state == 0x31 || ctx.hero.group == 0x11 || ctx.hero.group == 0x12 }

/// `0x423f08`: the floor under it (`+0x1b0`; from home 20 up in the set-up, else from 2 up, to 10 down) and its
/// hover height (`+0x1b4`, 1.7 over it, at least 0.1 over Ratchet).
fn floor(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let (z, up) = if m.state == 0 { (m.pv_f(HOME + 8), 20.0) } else { (m.pos[2], 2.0) };
    let f = floor_at(ctx, m.pos, z + up, z - 10.0, m.pv_f(0x1d4), m.pv_f(0x1d0)).max(m.pv_f(0x1d4));
    let m = &mut w.mobys[id];
    m.set_pv_f(FLOOR, f);
    m.set_pv_f(HOVER, (f + 1.7).max(ctx.hero.pos[2] + 0.1));
}

/// `0x424168`: it leaves the group's attackers.
fn release(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].pv_b(ATTACKING) == 0 { return; }
    let p = w.mobys[id].pv_i(ATTACKERS);
    let mut n = w.shared_i(p);
    countdown(&mut n);
    w.set_shared_i(p, n);
    w.mobys[id].set_pv_b(ATTACKING, 0);
}

/// `0x4240e0`: its group, `+0x1d8` and (the game reads `+0x1d8` again for it) `+0x1dc` poked; itself too.
fn alarm(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    let (g, a, b) = (m.group as i32, m.pv_i(0x1d8), m.pv_i(0x1dc));
    if g != -1 { w.poke_group(g, 1); }
    if a != -1 { w.poke_group(a, 1); }
    if b != -1 { w.poke_group(a, 1); }
    w.mobys[id].timer = 1;
}

/// `0x424ea8` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    floor(w, id, ctx);
    w.mobys[id].scale = w.class(CLASS).map_or(1.0, |c| c.scale) * 0.5;
    if ai::take_damage(w, id, ctx, false) == 0 { return; }
    let m = &mut w.mobys[id];
    if m.pv_f(HEALTH) <= 0.0 {
        m.set_pv_f(REACT + 0x94, f32::from_bits(0x3f56_7752));
        m.set_pv_s(REACT + 0x38, m.pv_s(REACT + 0x38) | 2);
    } else {
        m.set_pv_s(KNOCK, 0x14a);
    }
    release(w, id);
    alarm(w, id);
}

/// `0x424df8` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].timer != 0 {
        w.mobys[id].timer = 0;
        if w.mobys[id].pv_s(ALARM) < 200 {
            let t = ctx.game.rng.rand_range(300, 0x168) as i16;
            w.mobys[id].set_pv_s(ALARM, t);
        }
    }
    if w.mobys[id].pv_i(T_RESULT) == 2 { s16_down(w, id, ALARM); }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0x7ffd { m.state_ticks += 1; }
    s16_down(w, id, TIMER);
    s16_down(w, id, WAIT);
    let m = &mut w.mobys[id];
    if 2 <= m.state { m.shadow_slab = [m.pv_f(FLOOR) - m.pos[2], 0.2]; }
}

/// `0x4241a0`: the goal kept to its path: outside, a place 1.2 about Ratchet (half the time, while something is
/// targeted and it is not waiting after a bite), else home or (half the time, with one) 3 about the moby `+0x184`,
/// 1.7 up; 3..5 off either way when it would come within 3 of Ratchet while waiting. Pulled 2 back from a wall between,
/// then 1.7 over the ground under it (from 4 up).
fn pick_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let path = w.mobys[id].pv_i(0x1a8);
    let goal = w.mobys[id].pv_v(GOAL);
    let mut goal = if tabora_ai::in_volumes(ctx.level, goal, path, -1, -1, -1) != 0 {
        goal
    } else {
        let m = &w.mobys[id];
        let other = m.pv_i(0x184);
        let near_hero = ctx.game.rng.rand() & 1 == 0 && m.pv_i(T_RESULT) != 2 && m.pv_s(WAIT) == 0;
        let g = &mut ctx.game.rng;
        let mut goal = if near_hero {
            let a = g.randf(-PI, PI).cos() * 1.2;
            let b = g.randf(-PI, PI).sin() * 1.2;
            add([a, b, 0.0], ctx.hero.pos)
        } else if other == -1 || g.rand() & 1 != 0 {
            m.pv_v(HOME)
        } else {
            let a = g.randf(-PI, PI).cos() * 3.0;
            let b = g.randf(-PI, PI).sin() * 3.0;
            let at = w.mobys.get(other as usize).map_or([0.0; 3], |o| o.pos);
            add([a, b, 0.0], at)
        };
        goal[2] += 1.7;
        if w.mobys[id].pv_s(WAIT) != 0 && dist2(goal, ctx.hero.pos) < 3.0 {
            for c in goal.iter_mut().take(2) {
                let d = ctx.game.rng.randf(3.0, 5.0);
                *c += if ctx.game.rng.rand() & 1 == 0 { d } else { -d };
            }
        }
        goal
    };
    let at = w.mobys[id].pos;
    if let Some(o) = w.coll_line(ctx, at, goal, 2, Some(id)) {
        goal = add(o.point, set_len(sub(o.point, goal), 2.0));
    }
    goal[2] = ai::ground_z(ctx, 4.0, goal, 0) + 1.7;
    w.mobys[id].set_pv_v(GOAL, goal);
}

/// `0x4244e8`: a new goal about home (the wander radius `+0x189`), 8 farther from Ratchet while waiting after a
/// bite; then [`pick_goal`].
fn new_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let a = ctx.game.rng.randf(-PI, PI);
    let b = ctx.game.rng.randf(-PI, PI);
    let m = &mut w.mobys[id];
    let r = m.pv_b(0x189) as f32;
    let mut goal = add([a.cos() * r, b.sin() * r, 0.0], m.pv_v(HOME));
    goal[2] += 1.7;
    if m.pv_s(WAIT) != 0 {
        let away = sub(goal, ctx.hero.pos);
        goal = add(goal, set_len([away[0], away[1], 0.0], 8.0));
    }
    m.set_pv_v(GOAL, goal);
    pick_goal(w, id, ctx);
}

/// `0x424618` (module docs).
fn steer(w: &mut MobyWorld, id: usize, ctx: &Ctx, s: Steer) -> u32 {
    let m = &w.mobys[id];
    let goal = m.pv_v(GOAL);
    let yaw_to = atan2(goal[0] - m.pos[0], goal[1] - m.pos[1]);
    let d = (len2(sub(goal, m.pos)) - s.stop).max(0.0);
    let k = 0.000_277_777_8;
    let mut v = m.pv_f(SPEED) + (s.accel * d * k).min(s.accel * k);
    if d < v * 10.0 { v -= s.slow * v; }
    let mut flags = if d < s.stop { 8 } else { 0 };
    let most = s.top * 0.016_666_668;
    v = v.clamp(-most, most);
    ai::turn_to(&mut w.mobys[id], yaw_to, s.turn_a * 4.848_137_5e-6, s.turn_b * 4.848_137_5e-6, s.turn_most * 0.000_290_888_25, 0x1c4);
    let m = &w.mobys[id];
    let ahead = [m.pos[0] + yaw_to.cos() * v * 10.0, m.pos[1] + yaw_to.sin() * v * 10.0, m.pos[2]];
    let mut low = m.pv_f(0x1d4);
    if m.state == 8 { low = low.max(ctx.hero.pos[2]); }
    let home_z = m.pv_f(HOME + 8);
    let mut z = floor_at(ctx, ahead, home_z + 3.0, home_z - 10.0, low, m.pv_f(0x1d0));
    if dist2(m.pos, ctx.hero.pos) < 5.0 && ctx.hero.pos[2] < z { z = ctx.hero.pos[2]; }
    z += s.height;
    let pitch = atan2(dist2(m.pos, ahead), z - m.pos[2]);
    let yaw = m.rot[2];
    // `0x325100(v, yaw, pitch)`.
    let mut vel = [yaw.cos() * v * pitch.cos(), yaw.sin() * v * pitch.cos(), pitch.sin() * v];
    w.mobys[id].pos[2] += 0.4;
    flags |= crate::moby::slide::slide(w, id, ctx, 0.0, 0.75, 0.0, &mut vel, 0);
    let m = &mut w.mobys[id];
    m.pos[2] -= 0.4;
    m.set_pv_v(VEL, vel);
    let (a, pv) = tabora_ai::spring_angle(m.rot[1], pitch, 0.001_745_329_3, 0.000_872_664_74, 0.000_872_664_74, m.pv_f(0x1c8));
    m.rot[1] = a;
    m.set_pv_f(0x1c8, pv);
    if flags & 7 != 0 {
        ai::turn_to(m, yaw_to, s.turn_a * 1.454_441_2e-5, s.turn_b * 4.848_137_5e-6, s.turn_most * 0.000_872_664_74, 0x1c4);
    }
    w.mobys[id].set_pv_f(SPEED, v);
    w.changed = true;
    flags
}

/// `0x4249e8` (module docs): home and its floor, its update distance from its first volumes (and the wander radius
/// from the hunt's), the reaction block, its start.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_b(0x48, 15);
    m.set_pv_b(0x4a, 3);
    m.set_pv_b(0x29, 1);
    if ctx.game.frame == 0 { let p = m.pv_i(ATTACKERS); w.set_shared_i(p, 0); }
    let m = &mut w.mobys[id];
    let pos = m.pos;
    m.set_pv_v(HOME, pos);
    floor(w, id, ctx);
    let m = &mut w.mobys[id];
    m.pos[2] = m.pv_f(FLOOR);
    let pos = m.pos;
    m.set_pv_v(HOME, pos);
    let (a, b, c) = (m.pv_i(0x190), m.pv_i(0x198), m.pv_i(0x194));
    let r = volume_reach(ctx.level, pos, a, b, c, -1);
    if 0.0 < r { m.update_dist = (r + 2.0) as i32 as u8; }
    let ud = m.update_dist;
    m.set_pv_b(0x188, ud);
    m.set_pv_b(0x189, ud);
    let (a, b, c) = (m.pv_i(0x19c), m.pv_i(0x1a4), m.pv_i(0x1a0));
    let r = volume_reach(ctx.level, pos, a, b, c, -1);
    if 0.0 < r { m.set_pv_b(0x188, (r + 2.0) as i32 as u8); }
    // The reaction block (`+0x50`): a hurt goes to state 9.
    m.set_pv_i(0x10, REACT as i32);
    m.set_pv_b(0x60, 9);
    m.set_pv_i(0x98, -1);
    m.set_pv_b(0x62, 0xff);
    m.set_pv_b(0x67, 5);
    m.set_pv_f(0x6c, 0.25);
    m.set_pv_f(0x70, f32::from_bits(0x3e08_8889));
    m.set_pv_f(0x74, f32::from_bits(0x3b88_8889));
    m.set_pv_f(0x78, f32::from_bits(0x3c08_8889));
    m.set_pv_f(0x7c, f32::from_bits(0x3b36_0b61));
    m.set_pv_f(0x80, f32::from_bits(0x3e99_999a));
    m.set_pv_f(0x8c, f32::from_bits(0x3ecc_cccd));
    m.set_pv_b(0xc3, 3);
    m.set_pv_b(0xc6, 1);
    m.set_pv_s(0x88, 0);
    m.set_pv_b(0x6a, 0);
    m.set_pv_b(0x61, 0xff);
    m.set_pv_b(0x66, 5);
    m.set_pv_f(0x84, 0.0);
    m.set_pv_b(0xbc, 1);
    m.set_pv_b(0xbd, 3);
    m.set_pv_b(0xbe, 1);
    m.set_pv_b(0xbf, 3);
    m.set_pv_b(0xc2, 1);
    m.set_pv_b(0xc4, 0);
    m.set_pv_s(KNOCK, 0);
    m.set_pv_b(0x28, 0);
    m.set_pv_i(0x30, 0);
    m.set_pv_v(GOAL, pos);
    m.set_pv_f(SPEED, 0.0);
    match m.pv_b(START) {
        0 => {
            m.pos[2] = m.pv_f(FLOOR) - 1.7;
            let p = m.pos;
            m.set_pv_v(HOME, p);
            m.set_pv_v(VEL, ZERO);
            m.mode &= !0x1000;
            set_seq(w, id, ctx, 1, 0, 3);
            let m = &mut w.mobys[id];
            m.drawn = 0;
            m.mode |= 1;
            m.set_state(1, -1);
        }
        1 => {
            m.pos[2] = m.pv_f(FLOOR) + 2.7;
            let p = m.pos;
            m.set_pv_v(HOME, p);
            m.set_pv_v(VEL, ZERO);
            new_goal(w, id, ctx);
            w.mobys[id].mode |= 0x1000;
            set_seq(w, id, ctx, 1, 0, 3);
            w.mobys[id].set_state(6, 0);
        }
        2 => {
            let v = m.pv_v(0xd0);
            m.set_pv_v(VEL, v);
            m.pos = add(m.pos, v);
            let p = m.pos;
            m.set_pv_v(HOME, p);
            set_seq(w, id, ctx, 5, 0, 3);
            let m = &mut w.mobys[id];
            m.drawn = 1;
            m.mode = (m.mode & !1) | 0x1000;
            m.set_state(4, -1);
        }
        _ => {}
    }
    let r = ctx.game.rng.randf(0.0, 1.0);
    w.mobys[id].set_pv_f(HOME + 0xc, r);
    w.changed = true;
}

/// The moby `i` (−1 none: true): past its state 2 (`0x423fd0`), gone (`0x424000` / `0x424048`), or either
/// (`0x424090`).
fn past(w: &MobyWorld, i: i32) -> bool { i == -1 || w.mobys.get(i as usize).is_some_and(|m| 3 <= m.state) }
fn gone(w: &MobyWorld, i: i32) -> bool { i == -1 || w.mobys.get(i as usize).is_none_or(|m| matches!(m.state, 0xfe | 0xfd)) }

/// State 1: what wakes it (`+0x18b`, module docs).
fn wake(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let mut ready = match m.pv_b(WAKE) {
        0 => {
            if ctx.game.rng.randi(7) == 0 {
                let range = w.mobys[id].pv_b(0x18d) as f32;
                let v = first_vols(w, id);
                find_target(w, id, ctx, T_BLOCK, range, 0.0, v) != 2
            } else {
                false
            }
        }
        1 => {
            let i = m.pv_i(0x180);
            if i != -1 {
                past(w, i)
            } else {
                w.mobys[id].set_pv_b(WAKE, 0);
                false
            }
        }
        2 => m.timer != 0,
        3 => gone(w, m.pv_i(0x1ac)),
        4 => gone(w, m.pv_i(0x180)),
        5 => {
            let i = m.pv_i(0x184);
            past(w, i) || gone(w, i)
        }
        _ => false,
    };
    if w.mobys[id].timer & 2 != 0 { ready = true; }
    if !ready { return; }
    let t = ctx.game.rng.rand_range(0, 0xb4) as i16;
    let m = &mut w.mobys[id];
    m.update_dist = m.pv_b(0x188);
    m.set_pv_v(VEL, ZERO);
    m.set_pv_s(TIMER, t);
    m.set_state(3, -1);
}

/// 0x0fff (module docs).
pub fn update_0fff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state == 0x6f {
        let m = &mut w.mobys[id];
        m.drawn = 0;
        m.mode |= 1;
        return;
    }
    let m = &mut w.mobys[id];
    if m.mode & 1 != 0 && 4 <= m.state {
        m.mode &= !1;
        m.drawn = 1;
    }
    head(w, id, ctx);
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let other = m.pv_i(0x184);
            if other != -1 {
                if let Some(o) = w.mobys.get(other as usize).filter(|o| o.mission != 0xff) {
                    let ou = o.uid;
                    let done = ctx.game.uid_dead.get(ou.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(ou) != 0;
                    if done && ctx.game.uid_count(w.mobys[id].uid) != 0 {
                        w.delete(id);
                        return;
                    }
                }
            }
            setup(w, id, ctx);
        }
        1 => wake(w, id, ctx),
        3 => {
            if s16_down(w, id, TIMER) {
                if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); }
                let m = &mut w.mobys[id];
                m.drawn = 1;
                m.mode &= !1;
                m.set_state(2, 0);
            }
        }
        2 => rise(w, id, ctx),
        4 => launched(w, id, ctx),
        5 => hover(w, id, ctx),
        6 => patrol(w, id, ctx),
        7 => hunt(w, id, ctx),
        8 => dive(w, id, ctx),
        9 => knock(w, id, ctx),
        10 => {
            die(w, id, ctx);
            return;
        }
        _ => {}
    }
    if 0x80 <= w.mobys[id].state { return; }
    tail(w, id, ctx);
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(20.0..=1003.0).contains(&c)) { w.delete(id); }
}

/// State 2: rising to its hover height (sub 0) or sinking under the sand (sub 1); puffs early in its sequence.
fn rise(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let t = match m.sub_state {
        0 => m.pv_f(HOVER),
        1 => m.pv_f(FLOOR) - 1.7,
        _ => m.pos[2],
    };
    if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 4, 0); }
    let m = &mut w.mobys[id];
    let (mut z, mut v) = (m.pos[2], m.pv_f(VEL + 8));
    ease(t, 0.006_944_444_6, 0.004_166_667, 0.5, &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(VEL + 8, v);
    w.changed = true;
    if ctx.host.frame_time(&w.mobys[id]) < 12.0 {
        let m = &w.mobys[id];
        let at = [m.pos[0], m.pos[1], m.pv_f(FLOOR)];
        tabora_ai::puffs(w, id, ctx, at, 3, 1.0, 1.5, 0.5, 0.0, 0.1, 0.8);
    }
    if 0.2 <= (w.mobys[id].pos[2] - t).abs() { return; }
    match w.mobys[id].sub_state {
        0 => {
            w.mobys[id].mode |= 0x1000;
            set_seq(w, id, ctx, 3, 0, 3);
            let t = ctx.game.rng.rand_range(0xb4, 0x168) as i16;
            w.mobys[id].set_pv_s(TIMER, t);
            w.mobys[id].set_state(5, -1);
        }
        1 => {
            w.mobys[id].mode &= !0x1000;
            set_seq(w, id, ctx, 1, 0, 3);
            let m = &mut w.mobys[id];
            m.set_pv_s(TIMER, 0);
            m.drawn = 0;
            m.mode |= 1;
            m.set_state(1, -1);
        }
        _ => {}
    }
}

/// The hover's start: sequence 3, 180..360 ticks, state 5.
fn to_hover(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    set_seq(w, id, ctx, 3, 0, 3);
    let t = ctx.game.rng.rand_range(0xb4, 0x168) as i16;
    w.mobys[id].set_pv_s(TIMER, t);
    w.mobys[id].set_state(5, -1);
}

/// State 4: launched upward until it meets something (a sphere of 0.75, 0.75 up: not a 0x0945), stalls or 240 ticks.
fn launched(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, m.pv_v(VEL));
    m.set_pv_f(VEL + 8, m.pv_f(VEL + 8) - 0.002_722_222_4);
    m.pos[2] += 0.75;
    let at = m.pos;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, at, 0.75, crate::collision::QueryFlags(0), Some(id)));
    if let Some(o) = hit {
        if o.moby.is_none_or(|i| w.mobys[i].o_class != 0x945) { to_hover(w, id, ctx); }
    }
    let m = &w.mobys[id];
    if m.pv_f(VEL + 8) < 0.0 || 240 < m.state_ticks as u16 { to_hover(w, id, ctx); }
    let m = &mut w.mobys[id];
    m.pos[2] -= 0.75;
    m.set_pv_f(VEL + 8, m.pv_f(VEL + 8) - 0.002_722_222_4);
    if m.pos[2] < m.pv_f(FLOOR) { m.pos[2] = m.pv_f(FLOOR); }
    w.changed = true;
}

/// State 5: hovering on its weave: a goal 2 ahead turned ±0.2..0.5 of 90° (flipped each time it is drawn: on its
/// first tick and one frame in 50), the height bobbing on the frame count.
fn hover(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let mut angle = m.pv_f(TURN);
    if m.state_ticks == 0 || ctx.game.frame % 50 == m.uid as i32 % 50 {
        let k = ctx.game.rng.randf(0.2, 0.5);
        angle = if 0.0 < angle { k * -std::f32::consts::FRAC_PI_2 } else { k * std::f32::consts::FRAC_PI_2 };
    }
    let m = &mut w.mobys[id];
    let dir = add_rot(m.rot[2], angle);
    let goal = add([dir.cos() * 2.0, dir.sin() * 2.0, 0.0], m.pos);
    m.set_pv_v(GOAL, goal);
    // `0x320440(frame · 0.025)`: the bob's phase wrapped into −π..π (`0x2eff90`'s fraction).
    let phase = ((ctx.game.frame as f32 * 0.025 + PI) * 0.159_154_94).fract() * 6.283_185_5 - PI;
    pick_goal(w, id, ctx);
    let h = w.mobys[id].pv_f(HOME + 0xc) + 1.7 + 1.0 + phase.sin();
    if steer(w, id, ctx, Steer { turn_a: 360.0, turn_b: 180.0, turn_most: 560.0, accel: 20.0, slow: 0.0, top: 16.0, height: h, stop: -2.0 }) != 0 {
        angle *= 1.025;
    }
    w.mobys[id].set_pv_f(TURN, angle);
    if w.mobys[id].pv_s(TIMER) != 0 { return; }
    set_seq(w, id, ctx, 1, 0, 3);
    w.mobys[id].set_state(6, 0);
}

fn patrol_steer(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> u32 {
    let h = w.mobys[id].pv_f(HOME + 0xc) + 1.7;
    steer(w, id, ctx, Steer { turn_a: 120.0, turn_b: 360.0, turn_most: 120.0, accel: 30.0, slow: 0.0, top: 8.0, height: h, stop: 1.0 })
}

/// The pause before the next leg: `30..90` ticks, sequence 3 when it is longer than 6, then `(state, 1)`.
fn pause(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, state: u8) {
    if w.mobys[id].pv_s(TIMER) != 0 { return; }
    let t = ctx.game.rng.rand_range(0x1e, 0x5a) as i16;
    w.mobys[id].set_pv_s(TIMER, t);
    if 7 <= t { set_seq(w, id, ctx, 3, 0, 3); }
    w.mobys[id].set_state(state, 1);
}

/// The leg's end: a new goal, no pause (`0..0`), `(state, 0)`.
fn next_leg(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, state: u8) {
    new_goal(w, id, ctx);
    let t = ctx.game.rng.rand_range(0, 0) as i16;
    w.mobys[id].set_pv_s(TIMER, t);
    if 7 <= t { set_seq(w, id, ctx, 1, 0, 3); }
    w.mobys[id].set_state(state, 0);
}

/// State 6: patrolling about home; in the hunt's volumes, 1 in 9 a tick, the hunt.
fn patrol(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].sub_state {
        0 => pause(w, id, ctx, 6),
        1 => {
            patrol_steer(w, id, ctx);
            if w.mobys[id].pv_s(TIMER) == 0 { next_leg(w, id, ctx, 6); }
        }
        2 => {
            let home = w.mobys[id].pv_v(HOME);
            w.mobys[id].set_pv_v(GOAL, home);
            if patrol_steer(w, id, ctx) & 8 != 0 {
                set_seq(w, id, ctx, 2, 0, 10);
                w.mobys[id].set_state(2, 1);
            }
        }
        _ => {}
    }
    if search(w, id, ctx, false) != 2 {
        if ctx.game.rng.randi(9) == 0 {
            let t = ctx.game.rng.rand_range(300, 0x168) as i16;
            w.mobys[id].set_pv_s(ALARM, t);
            w.mobys[id].set_state(7, 0);
        }
    } else if w.mobys[id].state == 6 && w.mobys[id].pv_b(START) == 0 {
        w.mobys[id].set_state(6, 2);
    }
}

/// State 7: hunting near Ratchet; within 12 of the target, its wait over and fewer than 3 attackers, the dive.
fn hunt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].sub_state {
        0 => pause(w, id, ctx, 7),
        1 => {
            let h = w.mobys[id].pv_f(HOME + 0xc) + 1.7;
            if steer(w, id, ctx, Steer { turn_a: 250.0, turn_b: 180.0, turn_most: 250.0, accel: 32.0, slow: 0.1, top: 13.0, height: h, stop: 1.0 }) & 8 != 0 {
                w.mobys[id].set_pv_s(TIMER, 0);
            }
            if w.mobys[id].pv_s(TIMER) == 0 { next_leg(w, id, ctx, 7); }
        }
        _ => {}
    }
    if search(w, id, ctx, true) == 2 || hero_away(ctx) {
        w.mobys[id].set_pv_i(T_RESULT, 2);
        if w.mobys[id].pv_s(ALARM) == 0 { w.mobys[id].set_state(6, 0); }
        return;
    }
    let m = &w.mobys[id];
    let slot = m.pv_i(ATTACKERS);
    let n = w.shared_i(slot);
    if n < 3 && m.pv_s(WAIT) == 0 && dist2(m.pos, m.pv_v(T_AIM)) < 12.0 {
        w.set_shared_i(slot, n + 1);
        let m = &mut w.mobys[id];
        m.set_pv_b(ATTACKING, 1);
        let t = m.pv_v(T_AIM);
        m.set_pv_v(GOAL, t);
        m.set_state(8, 0);
    }
}

/// State 8: the dive and the bite.
fn dive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let target = w.mobys[id].pv_v(T_AIM);
    w.mobys[id].set_pv_v(GOAL, target);
    let s = |stop_h: f32| Steer { turn_a: 260.0, turn_b: 180.0, turn_most: 270.0, accel: 32.0, slow: 0.15, top: 8.0, height: stop_h, stop: 0.5 };
    let near = |w: &MobyWorld| dist2(w.mobys[id].pos, target) < 1.5 && (w.mobys[id].pos[2] - target[2]).abs() < 1.1;
    match w.mobys[id].sub_state {
        0 => {
            steer(w, id, ctx, s(0.4));
            if near(w) {
                set_seq(w, id, ctx, 4, 0, 3);
                w.mobys[id].set_state(8, 1);
            }
        }
        1 => {
            steer(w, id, ctx, s(0.3));
            w.mobys[id].set_state(8, 2);
        }
        2 => {
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 4) {
                let t = ctx.host.frame_time(&w.mobys[id]);
                if 19.5 < t && t < 29.0 {
                    if let Some(j) = ctx.host.joint(&w.mobys[id], 1) {
                        let damage = w.mobys[id].pv_b(0x34) as f32;
                        ai::attack_sphere(w, id, ctx, 0.5, damage, 1.0, j, 1, 0, 1, 0);
                    }
                }
                if wrapped(w, id, ctx) {
                    let wait = ctx.game.rng.rand_range(0xb4, 0x104) as i16;
                    let m = &mut w.mobys[id];
                    m.set_pv_s(TIMER, 0x78);
                    m.set_pv_s(WAIT, wait);
                    set_seq(w, id, ctx, 1, 0, 3);
                    w.mobys[id].set_state(8, 3);
                }
            }
        }
        3 => {
            steer(w, id, ctx, s(0.3));
            if s16_down(w, id, TIMER) {
                if near(w) {
                    set_seq(w, id, ctx, 4, 0, 3);
                    w.mobys[id].set_state(8, 1);
                } else {
                    w.mobys[id].set_pv_v(GOAL, target);
                    w.mobys[id].set_state(8, 0);
                }
            }
        }
        _ => {}
    }
    if search(w, id, ctx, true) != 2 && !hero_away(ctx) { return; }
    release(w, id);
    if w.mobys[id].pv_s(ALARM) != 0 {
        w.mobys[id].set_pv_s(TIMER, 0);
        set_seq(w, id, ctx, 3, 0, 3);
        w.mobys[id].set_state(6, 0);
        return;
    }
    // Away: the game's goal is `set_len(2·pos − goal, 5)`, which the path then pulls back in.
    let m = &mut w.mobys[id];
    let g = set_len(add(sub(m.pos, m.pv_v(GOAL)), m.pos), 5.0);
    m.set_pv_v(GOAL, g);
    pick_goal(w, id, ctx);
    steer(w, id, ctx, s(0.3));
    w.mobys[id].set_pv_s(TIMER, 0);
    set_seq(w, id, ctx, 3, 0, 3);
    let wait = ctx.game.rng.rand_range(0xb4, 0x104) as i16;
    w.mobys[id].set_pv_s(WAIT, wait);
    w.mobys[id].set_state(7, 0);
}

/// State 9: the knock (its first tick its class sound 0); dead, 10; over (or 360 ticks), hunting.
fn knock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_s(KNOCK, m.pv_s(KNOCK).wrapping_add(1));
    if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
    let res = ai::flight(w, id, ctx);
    if res & 1 != 0 {
        w.mobys[id].set_state(10, -1);
    } else if res & 2 != 0 || 360 < w.mobys[id].pv_s(KNOCK) {
        w.mobys[id].set_pv_b(REACT + 0x1a, 2);
        set_seq(w, id, ctx, 1, 0, 3);
        let m = &mut w.mobys[id];
        m.set_pv_s(KNOCK, 0);
        m.set_pv_v(VEL, ZERO);
        let st = if m.pv_f(HEALTH) <= 0.0 { 10 } else { 7 };
        m.set_state(st, if st == 7 { 0 } else { -1 });
    }
}

/// State 10: its death (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 1, 0); }
    tabora_ai::sparkles(w, id, ctx, ZERO);
    let e = Explosion { flash: 2.0, flash2: 1.0, near: 9.0, size: 1.0, embers: 4, sound: -1, lod: -1, colour: 1, flags: 0x81_0001, ..Default::default() };
    blast::explode(w, ctx, Some(id), None, &e);
    release(w, id);
    let flags = if w.mobys[id].pv_b(START) == 2 { 0x1000 } else { 0 };
    bolts::drop(w, id, flags, 3, ctx);
    // `0x426360..`: the uid marked again past the drop's own marking (the game's code does both).
    let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
    ctx.game.uid_gone(uid, mission);
    ctx.game.bump_uid(uid);
    w.delete(id);
}
