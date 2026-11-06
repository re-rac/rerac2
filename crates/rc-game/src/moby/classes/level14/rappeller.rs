//! The prison's rope enemy **0x09b4** (L14 `0x3d0610`, 26 placed; L00 `0x3bec28`; the boot ELF's v2 `0x319190`; the
//! decomp's `func_US2_L00_003BEC28`, its helpers the decomp's `func_00319780`.. `func_0031AB18`, here by their L00
//! places): it hangs on a rope above the floor and comes down it to fight (or walks in along a path), shooting
//! fireballs (crate::moby::classes::level07::gunner's 0x0b7f, made by L00 `0x3e1118`). On the framework
//! (crate::moby::ai): target block `+0x20` (`+0x30` the moby, `+0x34` found), health `+0x40` (`+0x46` its wait, its
//! damage byte `+0x54`), the flash `+0x70`, reaction `+0x80` (a hurt to state 17, sub 4; radius and centre 0.5,
//! bounce 0.4), movement `+0x130` (radius 0.8, height 0.5, steps 0.4 / 0.4, options 5).
//!
//! **Its pvars:** `+0x280` / `+0x282` timers counted down (s16), `+0x286` alarmed by its group ([`alarm`]),
//! `+0x28c` start in state 3 (sub 1), `+0x290` the height's speed, `+0x29c` / `+0x2a0` the turn's and the tilt's
//! speeds, `+0x2a4` engaged, `+0x2a8` / `+0x2ac` the sway's phases, `+0x2c0` ticks on the rope, `+0x2c4` the rope's
//! looped sound, `+0x2c8` the kick's window (read by nothing here), `+0x2d0` the wake countdown, `+0x2dc` the
//! hang height (−1 none), `+0x2e0` wait to be hooked, `+0x2f0` the rope's top (20 over joint 0 at the set-up, or the
//! ceiling the line meets), `+0x300` / `+0x310` / `+0x320` joints 0..2 and `+0x330` 0.25 along joint 3's first row,
//! `+0x350` the lead (one of its group's at a time), `+0x354` the rounds shot hanging, `+0x35c` a cuboid that sets
//! off its walk in, `+0x360` / `+0x368` where a hang began and its goal height, `+0x370` the path in (−1 none),
//! `+0x374` / `+0x378` / `+0x37c` its place on it, speed and started, `+0x400` its trigger area (the target).
//!
//! **Each tick** (`0x3bec28`): nothing as Clank (`0x18c0d4` = 1). The sub-state, then the state, then the rope's
//! sound (10, looped, in states 12, 13, 14 and 16; stopped otherwise), the head ([`head`], `0x3c0018`), the tail
//! (`0x3c0758`: its flash, `0x3201e8`, crate::moby::ai::flash; its two timers; its state ticks; its head aim,
//! `0x327308` on `+0x380`, list 5, 2.0: crate::moby::joint_aim::head) and its draw
//! ([`draw`], `0x3c0850`).
//!
//! **Sub-states** (`+0x95`; the limits A = `0x32dcf0(1/450, 1/15, 0.0667°, 6°)` and B = `(1/300, 0.1, 0.0667°, 6°)`):
//! **0** waiting (A; with `+0x2e0` only once hooked): found and engaged, once `+0x2d0` is out, to the climb (9, sub
//! 3, sequence 1); alarmed, alert (8, sub 2, `+0x280` 480..600 when under 300). **1** on the rope (A): found and
//! engaged once `+0x2d0` is out, or alarmed while engaged, down the rope (13, sub 3, sequence 0x19). **2** alert (A,
//! speed 1): found and engaged, the climb; `+0x280` out, idle (2, sub 0, sequence 2). **3** fighting (B, speed 1):
//! not found and not on the rope or the path (13, 12, 15), alert (sequence 6, `+0x280` 580..600), or with `+0x28c`
//! state 3 (sub 1, sequence 0xb). **4** hurt: the framework's flight (`0x328b80`, its wait 60 on the first tick);
//! dead (or no health), a smoke burst (`0x3210a8`, 1, 13), its bolts (`0x31adc0`, limit 3) and deleted; over, found
//! the turn (7, sub 3, sequence 7) else alert (8, sub 2, sequence 6, `+0x280` 580..600), engaged.
//!
//! **States** (`+0x20`): **3** the sway (`0x3244e8`: 4°, 3°, 0.75° a tick). **4** at its sequence's end 5 (0xd).
//! **5** facing its target, the tilt sprung level and the rows rebuilt; down toward 3 under the ground (eased: 1/40,
//! 0.163, ¼); within 0.05 of it, 6 (0xe). **6** at its end the ground and 10 (sub 3, 0x1e). **7** turning (its rows,
//! [`face`]) until aimed within 10° ([`off_aim`]), then 10 (0x1e). **9** at its end B and 7. **10** the stand: at
//! its sequence's end not aimed 7, else hurt by it (`0x18c2bc`) or 1 in 6 the kick (11, 0x10); at frame 17 a
//! fireball from joint 4 at 1/3 a tick toward Ratchet's aim point (another target's middle), or not aimed along its
//! facing turned 10° toward it (`0x321c00`). **11** the kick: at its end (Ratchet more than 4 across: B, else A) 10;
//! `+0x2c8` set in frames ≤ 5 and 12..15 of sequence 0x10. **12** down the rope: facing its target; at or under the
//! hang height `+0x2dc` (not −1) with a group the lead taken ([`claim`]) and 14, without 16 (0x1d); else down toward 3
//! under the ground (1/60, 0.163, 1/6); within 0.05 of it, 6. **13** the drop onto the rope: facing its target
//! (`0x31c1e0`: 0.001, 0.01, 0.1), the tilt sprung level, its height down by 1/4000 of the ground's height a tick
//! (the original's sum) while more than 0.05 over it, else 6; at its sequence's end 12 (0x1a). **14** a hang move: its
//! goal `+0x2dc` plus 4..7 (the lead) or −3..3; there at once 16 (the original compares with where it began on its
//! first tick, so this lasts a tick); its height eased (1/240, 1/240, 1/6). **15** the walk in: once Ratchet is in the
//! cuboid `+0x35c`, along the path `+0x370` (its place eased to the end: 1/72, 1/720, 1/3; the pose `0x343fb8`,
//! crate::paths::pose), at its end 10 (sub 3). **16** hanging: at its sequence's end with a group, one round more;
//! the lead after 3 passes the lead on ([`pass_lead`]) and moves (14, 0x1a), the others move each time; at frame 16
//! a fireball at its target (sound 0xb), else with sequence 0x1e a smoke puff at joint 4 under frame 17.
//!
//! **The head** ([`head`], `0x3c0018`): the set-up (`0x3c03d0`) in state 0; the shadow (`0x31da58` within 24,
//! crate::shadow_volumes::probe_near); the joints; the frame's damage (`0x327e38`, crate::moby::ai::take_damage);
//! high over the ground (none, or more than 20), mid-flight it tumbles (crate::moby::attach::pivot_turn 0.05, 0.05
//! about 0.5 up; the rows given back to the angles first); else on a surface 1 or 11 under it (a line 1 down), its
//! bolts and deleted. Hurt: kind 5 `+0x110` 0 / `+0x114` 48°, its group alarmed, the lead passed on, and dead or kind
//! 5 with two of its group alive the first of the group's 0x09b4s not engaged is. The target (`0x32ad38` with no
//! range: the area `+0x400`); the lead off the rope with Ratchet its target takes the nearest of its group that is
//! no 0x09b4 and nearer than him ([`retarget`], `0x3c0fb0`).
//!
//! The engine functions, by their L00 places: `0x31b8d0` (crate::moby::Moby::set_state), `0x31b5f0` (set_anim,
//! crate::moby::classes::props::set_seq), `0x32a7c8` / `0x32a7f0` / `0x32a838` (the first tick of the state / sub,
//! hooked), `0x32dcf0` (crate::moby::ai::senses), `0x328b80` (crate::moby::ai::flight), `0x323bf8`
//! (crate::moby::ai::reached), `0x311880` (the frame time), `0x3127c8` / `0x312728` (a joint's place / frame),
//! `0x31ce20` (crate::moby::ai::ground_z), `0x2de878` / `0x2e23e0` (the line and its surface), `0x31ebd8` / `0x31ed78`
//! (crate::hero::physics::spring / ease), `0x31f230` (turn_ease), `0x31c1e0` (crate::moby::ai::turn_to toward a moby),
//! `0x321858` (in_cuboid), `0x321c00` (crate::camera::rot), `0x3244e8` (crate::moby::classes::props::sway),
//! `0x31c8e0` (crate::moby::classes::props::group_alive), `0x2eeab0` / `0x2eeae0` (crate::moby::countdown),
//! `0x3602f0` / `0x3605f0` (the sounds), `0x33ef78` (crate::particles::sprite), `0x2e7920` (the draw), `0x311688`
//! (delete), crate::rng. `0x2a48f0(0x1afd70, ticks)` at the end of the drop is an empty debug print.
//!
//! The port keeps `+0x2c4` −1 at the set-up where the original writes 1: the original's stop tests that the
//! channel is its own, which the port's handles do not carry.

use std::f32::consts::{FRAC_PI_2, PI};

use crate::hero::physics::{add, atan2, dist, dist2, dot, ease, scale, set_len, spring, sub, turn_ease, V3};
use crate::moby::attach::{angles_of, pivot_turn, rows_of, turn};
use crate::moby::classes::level07::gunner;
use crate::moby::classes::props::{group_alive, set_seq, sway, wrapped};
use crate::moby::{ai, bolts, countdown, mode, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x09b4;

const T_MOBY: usize = 0x30;
const T_FOUND: usize = 0x34;
const HEALTH: usize = 0x40;
const WAIT: usize = 0x46;
const DAMAGE: usize = 0x54;
const FLASH: usize = 0x70;
const REACT: usize = 0x80;
const MV: usize = 0x130;
const TIMER: usize = 0x280;
const TIMER2: usize = 0x282;
const ALARM: usize = 0x286;
const IDLE: usize = 0x28c;
const SPEED_Z: usize = 0x290;
const TURN_V: usize = 0x29c;
const TILT_V: usize = 0x2a0;
const ENGAGED: usize = 0x2a4;
const SWAY_A: usize = 0x2a8;
const SWAY_B: usize = 0x2ac;
const TICKS: usize = 0x2c0;
const SOUND: usize = 0x2c4;
const KICK: usize = 0x2c8;
const WAKE: usize = 0x2d0;
const HANG: usize = 0x2dc;
const HOOK: usize = 0x2e0;
const TOP: usize = 0x2f0;
const J0: usize = 0x300;
const J1: usize = 0x310;
const J2: usize = 0x320;
const J3: usize = 0x330;
const LEAD: usize = 0x350;
const ROUNDS: usize = 0x354;
const CUBOID: usize = 0x35c;
const HANG_AT: usize = 0x360;
const HANG_Z: usize = 0x368;
const PATH: usize = 0x370;
const PATH_T: usize = 0x374;
const PATH_V: usize = 0x378;
const PATH_ON: usize = 0x37c;
/// The head aim (list 5, 2.0 with the enemies' big heads: crate::moby::joint_aim::head).
const HEAD_AIM: usize = 0x380;
const AREA: usize = 0x400;

/// The limits A and B (module docs).
const SLOW: [u32; 4] = [0x3b11_a2b4, 0x3d88_8889, 0x3a98_825c, 0x3dd6_7752];
const FAST: [u32; 4] = [0x3b5a_740e, 0x3dcc_ccce, 0x3a98_825c, 0x3dd6_7752];
/// 1/60, 1/15: the tilt's spring.
const TILT_K: f32 = f32::from_bits(0x3c88_8889);
const TILT_MAX: f32 = f32::from_bits(0x3d88_8889);
/// 0.05: "there".
const NEAR: f32 = f32::from_bits(0x3d4c_cccd);
/// 0.163: the descents' brake.
const DOWN_BRAKE: f32 = f32::from_bits(0x3e27_40db);
/// 1/3 a tick: the fireball's speed.
const SHOT_SPEED: f32 = f32::from_bits(0x3eaa_aaab);

fn senses(m: &mut Moby, s: [u32; 4]) {
    ai::senses(m, f32::from_bits(s[0]), f32::from_bits(s[1]), f32::from_bits(s[2]), f32::from_bits(s[3]));
}

/// `0x32a7f0`: the first time since the last sub-state change.
fn first_sub(m: &mut Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

/// `0x2eeab0` on an int pvar.
fn count_i(m: &mut Moby, o: usize) -> bool {
    let mut t = m.pv_i(o);
    let d = countdown(&mut t);
    m.set_pv_i(o, t);
    d
}

/// `0x2eeae0` on a short pvar.
fn count_s(m: &mut Moby, o: usize) {
    let mut t = m.pv_s(o) as i32;
    countdown(&mut t);
    m.set_pv_s(o, t as i16);
}

fn found(m: &Moby) -> bool { m.pv_i(T_FOUND) == 1 }

/// Its group's members (none for group −1).
fn members(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

fn has_group(w: &MobyWorld, id: usize) -> bool { w.mobys[id].group != -1 }

/// Its target moby's place (Ratchet's feet for him).
fn target_place(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> {
    let k = ai::link(&w.mobys[id], T_MOBY)?;
    if Some(k) == ctx.hero_moby { return Some(ctx.hero.pos); }
    w.mobys.get(k).map(|t| t.pos)
}

/// Where a shot goes: Ratchet's aim point (`0x189ef0`) when he is the target, else the target's middle.
fn aim_point(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> {
    let k = ai::link(&w.mobys[id], T_MOBY)?;
    if Some(k) == ctx.hero_moby { return Some(ctx.hero.aim_point); }
    w.mobys.get(k).map(|t| [t.pos[0], t.pos[1], t.pos[2] + t.bsphere[3] * 0.000_976_562_5])
}

/// 0x09b4 (module docs).
pub fn update_09b4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.mode == 1 { return; }
    if subs(w, id, ctx) { return; }
    states(w, id, ctx);
    rope_sound(w, id, ctx);
    if head(w, id, ctx) { return; }
    // `0x3c0758`.
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    count_s(m, TIMER);
    count_s(m, TIMER2);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 5, 2.0);
    w.changed = true;
    w.register_draw(id);
}

/// The sub-state (module docs): true when it was deleted.
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(HOOK) != 0 && m.once & 4 == 0 { return false; }
            if first_sub(m) { senses(m, SLOW); }
            if found(m) && m.pv_i(ENGAGED) != 0 {
                if count_i(m, WAKE) { climb(w, id, ctx); }
            } else if m.pv_s(ALARM) != 0 {
                m.set_state(8, 2);
                m.set_pv_s(ALARM, 0);
                if m.pv_s(TIMER) < 300 {
                    let t = ctx.game.rng.rand_range(0x1e0, 0x258);
                    w.mobys[id].set_pv_s(TIMER, t as i16);
                }
            }
        }
        2 => {
            if first_sub(&mut w.mobys[id]) {
                senses(&mut w.mobys[id], SLOW);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            let m = &w.mobys[id];
            if found(m) && m.pv_i(ENGAGED) != 0 {
                climb(w, id, ctx);
            } else if m.pv_s(TIMER) == 0 {
                w.mobys[id].set_state(2, 0);
                set_seq(w, id, ctx, 2, 0, 10);
            }
        }
        1 => {
            let m = &mut w.mobys[id];
            if first_sub(m) { senses(m, SLOW); }
            let go = if found(m) && m.pv_i(ENGAGED) != 0 {
                count_i(m, WAKE)
            } else if m.pv_s(ALARM) != 0 && m.pv_i(ENGAGED) != 0 {
                m.set_pv_s(ALARM, 0);
                true
            } else {
                false
            };
            if go {
                w.mobys[id].set_state(13, 3);
                set_seq(w, id, ctx, 0x19, 0, 4);
            }
        }
        3 => {
            if first_sub(&mut w.mobys[id]) {
                senses(&mut w.mobys[id], FAST);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            let m = &w.mobys[id];
            if !found(m) && !matches!(m.state, 13 | 12 | 15) {
                alert(w, id, ctx);
                if w.mobys[id].pv_i(IDLE) != 0 {
                    w.mobys[id].set_state(3, 1);
                    set_seq(w, id, ctx, 0xb, 0, 0x14);
                }
            }
        }
        4 => {
            if w.mobys[id].first() { w.mobys[id].set_pv_s(WAIT, 60); }
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 || w.mobys[id].pv_f(HEALTH) <= 0.0 {
                let at = w.mobys[id].pos;
                crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), at, -1);
                bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
                return true;
            }
            if f & 2 != 0 {
                if found(&w.mobys[id]) {
                    w.mobys[id].set_state(7, 3);
                    set_seq(w, id, ctx, 7, 0, 6);
                } else {
                    alert(w, id, ctx);
                }
                w.mobys[id].set_pv_i(ENGAGED, 1);
            }
        }
        _ => {}
    }
    false
}

/// The climb (9, sub 3, sequence 1).
fn climb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(9, 3);
    set_seq(w, id, ctx, 1, 0, 4);
}

/// Alert (8, sub 2, sequence 6, `+0x280` 580..600).
fn alert(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(8, 2);
    set_seq(w, id, ctx, 6, 0, 10);
    let t = ctx.game.rng.rand_range(0x244, 0x258);
    w.mobys[id].set_pv_s(TIMER, t as i16);
}

/// The stand (10, sequence 0x1e).
fn stand(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, sub: i32) {
    w.mobys[id].set_state(10, sub);
    set_seq(w, id, ctx, 0x1e, 0, 6);
}

/// 6 (sequence 0xe).
fn land(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(6, -1);
    set_seq(w, id, ctx, 0xe, 0, 3);
}

/// The tilt sprung level (`0x31ebd8(0, 1/60, 0, 1/15)` on one angle, the speed `+0x2a0`).
fn level_out(m: &mut Moby, axis: usize) {
    let (mut a, mut v) = (m.rot[axis], m.pv_f(TILT_V));
    spring(0.0, TILT_K, 0.0, TILT_MAX, &mut a, &mut v);
    m.rot[axis] = a;
    m.set_pv_f(TILT_V, v);
}

/// `0x31c1e0(m, target, &+0x29c, 0.001, 0.01, 0.1)`: the facing turned toward its target moby.
fn turn_to_target(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(t) = target_place(w, id, ctx) else { return };
    let m = &mut w.mobys[id];
    let yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
    ai::turn_to(m, yaw, f32::from_bits(0x3a83_126f), f32::from_bits(0x3c23_d70a), f32::from_bits(0x3dcc_cccd), TURN_V);
}

/// `0x31ed78` on its height with the speed `+0x290`.
fn ease_z(m: &mut Moby, t: f32, a: f32, b: f32, max: f32) {
    let (mut z, mut v) = (m.pos[2], m.pv_f(SPEED_Z));
    ease(t, a, b, max, &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(SPEED_Z, v);
}

/// The state (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        7 => {
            if off_aim(w, id, ctx, 10.0) {
                face_target(w, id, ctx);
            } else {
                w.mobys[id].set_state(10, -1);
                set_seq(w, id, ctx, 0x1e, 0, 6);
            }
        }
        3 => sway(&mut w.mobys[id], f32::from_bits(0x3d8e_fa36), f32::from_bits(0x3d56_7752), f32::from_bits(0x3c56_7752), SWAY_A, SWAY_B),
        13 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(TICKS, m.pv_i(TICKS) + 1);
            if ai::link(m, T_MOBY).is_none() { return; }
            turn_to_target(w, id, ctx);
            let m = &mut w.mobys[id];
            level_out(m, 1);
            level_out(m, 0);
            let g = ai::ground_z(ctx, 0.5, m.pos, 0);
            let m = &mut w.mobys[id];
            if NEAR < m.pos[2] - g {
                m.pos[2] -= g * f32::from_bits(0x3983_126f);
            } else {
                land(w, id, ctx);
            }
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(12, -1);
                set_seq(w, id, ctx, 0x1a, 0, 0xf);
            }
        }
        12 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(TICKS, m.pv_i(TICKS) + 1);
            face_target(w, id, ctx);
            let m = &w.mobys[id];
            let g = ai::ground_z(ctx, 0.5, m.pos, 0);
            let (z, floor) = (m.pos[2], m.pv_f(HANG));
            if floor != -1.0 && z <= floor {
                if has_group(w, id) {
                    w.mobys[id].set_pv_i(ROUNDS, 0);
                    claim(w, id);
                    w.mobys[id].set_state(14, -1);
                } else {
                    w.mobys[id].set_state(16, -1);
                    set_seq(w, id, ctx, 0x1d, 0, 3);
                }
            } else if NEAR < z - g {
                ease_z(&mut w.mobys[id], g - 3.0, TILT_K, DOWN_BRAKE, f32::from_bits(0x3e2a_aaab));
            } else {
                land(w, id, ctx);
            }
        }
        14 => {
            if w.mobys[id].first() {
                let lead = w.mobys[id].pv_i(LEAD) != 0;
                let d = if lead { ctx.game.rng.randf(4.0, 7.0) } else { ctx.game.rng.randf(-3.0, 3.0) };
                let m = &mut w.mobys[id];
                let p = m.pos;
                m.set_pv_v(HANG_AT, p);
                m.set_pv_f(HANG_Z, m.pv_f(HANG) + d);
            }
            let m = &mut w.mobys[id];
            if dist(m.pos, m.pv_v(HANG_AT)) < NEAR {
                m.set_pv_i(ROUNDS, 0);
                m.set_state(16, -1);
            }
            m.set_pv_i(TICKS, m.pv_i(TICKS) + 1);
            face_target(w, id, ctx);
            let m = &mut w.mobys[id];
            let r = f32::from_bits(0x3b88_8889);
            let t = m.pv_f(HANG_Z);
            ease_z(m, t, r, r, f32::from_bits(0x3e2a_aaab));
        }
        4 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(5, -1);
                set_seq(w, id, ctx, 0xd, 0, 0xf);
            }
        }
        5 => {
            if ai::link(&w.mobys[id], T_MOBY).is_none() { return; }
            let m = &mut w.mobys[id];
            level_out(m, 1);
            m.rows = rows_of(m.rot);
            turn_to_target(w, id, ctx);
            let m = &w.mobys[id];
            let g = ai::ground_z(ctx, 0.5, m.pos, 0);
            if NEAR < m.pos[2] - g {
                ease_z(&mut w.mobys[id], g - 3.0, f32::from_bits(0x3ccc_ccce), DOWN_BRAKE, 0.25);
            } else {
                land(w, id, ctx);
            }
        }
        6 => {
            if wrapped(w, id, ctx) {
                stand(w, id, ctx, 3);
                let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
                w.mobys[id].pos[2] = z;
            }
        }
        9 => {
            if wrapped(w, id, ctx) {
                senses(&mut w.mobys[id], FAST);
                w.mobys[id].set_state(7, -1);
                set_seq(w, id, ctx, 7, 0, 6);
            }
        }
        10 => {
            if wrapped(w, id, ctx) {
                if off_aim(w, id, ctx, 10.0) {
                    w.mobys[id].set_state(7, -1);
                    set_seq(w, id, ctx, 7, 0, 6);
                } else if ctx.hero.damage.hit_by == Some(id) || ctx.game.rng.randi(6) == 1 {
                    w.mobys[id].set_state(11, -1);
                    set_seq(w, id, ctx, 0x10, 0, 7);
                }
            }
            if ai::reached(w, id, ctx, 17.0) {
                let Some(muzzle) = ctx.host.joint(&w.mobys[id], 4) else { return };
                let Some(t) = aim_point(w, id, ctx) else { return };
                let v = if off_aim(w, id, ctx, 10.0) {
                    let to = sub(t, muzzle);
                    let fwd = w.mobys[id].rows[0];
                    // `0x2eef88(axis, fwd, to)` = to × fwd.
                    let axis = crate::hero::physics::cross(to, fwd);
                    let a = if 0.0 < dot(fwd, to) { -f32::from_bits(0x3e32_b8c3) } else { f32::from_bits(0x3e32_b8c3) };
                    crate::camera::rot(fwd, a, axis)
                } else {
                    sub(t, muzzle)
                };
                fire(w, id, ctx, v, muzzle);
            }
        }
        11 => {
            if wrapped(w, id, ctx) {
                let s = if 4.0 < dist2(w.mobys[id].pos, ctx.hero.pos) { FAST } else { SLOW };
                senses(&mut w.mobys[id], s);
                w.mobys[id].set_state(10, -1);
                set_seq(w, id, ctx, 0x1e, 0, 6);
            }
            let f = ctx.host.frame_time(&w.mobys[id]);
            let kick = ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0x10) && (f <= 5.0 || (12.0 <= f && f <= 15.0));
            w.mobys[id].set_pv_i(KICK, kick as i32);
        }
        15 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_f(PATH_T, 0.0);
                w.mobys[id].set_pv_f(PATH_V, 0.0);
            }
            let m = &w.mobys[id];
            if m.pv_i(PATH_ON) == 0 {
                if ctx.level.in_cuboid(m.pv_i(CUBOID), ctx.hero.pos) { w.mobys[id].set_pv_i(PATH_ON, 1); }
                return;
            }
            let Some(points) = usize::try_from(m.pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| 1 < p.len()) else { return };
            let end = (points.len() - 1) as f32;
            let m = &mut w.mobys[id];
            let (mut t, mut v) = (m.pv_f(PATH_T), m.pv_f(PATH_V));
            ease(end, f32::from_bits(0x3c63_8e39), f32::from_bits(0x3ab6_0b61), SHOT_SPEED, &mut t, &mut v);
            m.set_pv_f(PATH_T, t);
            m.set_pv_f(PATH_V, v);
            let (p, pitch, yaw, _) = crate::paths::pose(points, t, false);
            m.pos = p;
            m.rot[1] = pitch;
            m.rot[2] = yaw;
            m.rows = rows_of(m.rot);
            if end <= t { m.set_state(10, 3); }
        }
        16 => {
            if wrapped(w, id, ctx) && has_group(w, id) {
                let m = &mut w.mobys[id];
                let n = m.pv_i(ROUNDS) + 1;
                m.set_pv_i(ROUNDS, n);
                let go = if m.pv_i(LEAD) != 0 { 3 <= n && pass_lead(w, id) } else { true };
                if go {
                    w.mobys[id].set_state(14, -1);
                    set_seq(w, id, ctx, 0x1a, 0, 0xf);
                }
            }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if ai::reached(w, id, ctx, 16.0) {
                let Some(muzzle) = ctx.host.joint(&w.mobys[id], 4) else { return };
                let Some(t) = aim_point(w, id, ctx) else { return };
                ctx.host.play_sound(&w.mobys[id], 0xb, 0);
                fire(w, id, ctx, sub(t, muzzle), muzzle);
            } else if f < 17.0 && crate::moby::classes::props::seq(w, id, ctx) == 0x1e {
                let Some(muzzle) = ctx.host.joint(&w.mobys[id], 4) else { return };
                let Some(sys) = ctx.parts.as_deref_mut() else { return };
                let rng = &mut ctx.game.rng;
                let s0 = rng.randf(1.0, 1.5) * 210_000.0;
                let s1 = rng.randf(f32::from_bits(0x3e4c_cccd), f32::from_bits(0x3ecc_cccd)) * 210_000.0;
                let fall = rng.randf_signed(0.0, f32::from_bits(0x3dcc_cccd));
                crate::particles::sprite(sys, rng, s0, s1, fall, muzzle, [0.0; 3], 4, 0x80_8080, 9, 0x4f, 0, 0);
            }
        }
        _ => {}
    }
}

/// A fireball (`0x3e1118(m, vel, at, 0, 0, damage)`) along `v` at 1/3 a tick, its damage byte `+0x54`.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, v: V3, at: V3) {
    let damage = w.mobys[id].pv_b(DAMAGE) as f32;
    gunner::shot(w, id, ctx, damage, set_len(v, SHOT_SPEED), at);
}

/// The rope's sound (module docs).
fn rope_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    let on = h != -1 && ctx.host.sound_playing(h);
    if matches!(w.mobys[id].state, 12 | 13 | 14 | 16) {
        if !on {
            let h = ctx.host.play_sound(&w.mobys[id], 10, 4);
            w.mobys[id].set_pv_i(SOUND, h);
        }
    } else {
        if on { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
    }
}

/// `0x3c0018` (module docs): true when it was deleted.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 { setup(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    for (j, o) in [(0, J0), (1, J1), (2, J2)] {
        if let Some(p) = ctx.host.joint(&w.mobys[id], j) { w.mobys[id].set_pv_v(o, p); }
    }
    if let Some((r, t)) = ctx.host.joint_matrix(&w.mobys[id], 3) { w.mobys[id].set_pv_v(J3, add(t, scale(r[0], 0.25))); }
    let kind = ai::take_damage(w, id, ctx, false);
    let pos = w.mobys[id].pos;
    let g = ai::ground_z(ctx, 0.5, pos, 0);
    if g == 0.0 || 20.0 < pos[2] - g {
        let m = &mut w.mobys[id];
        if m.pv_b(REACT + 0x1a) == 1 {
            if m.mode & mode::KEEP_ROWS != 0 {
                m.rot = angles_of(&m.rows);
                m.mode &= !mode::KEEP_ROWS;
            }
            pivot_turn(m, [NEAR, NEAR, 0.0], [0.0, 0.0, 0.5]);
        }
    } else {
        let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, pos, sub(pos, [0.0, 0.0, 1.0]), crate::collision::QueryFlags(0), Some(id)));
        if hit.is_some_and(|h| matches!(h.surface_id(), 1 | 11)) {
            bolts::drop(w, id, 0, 3, ctx);
            w.delete(id);
            return true;
        }
    }
    if kind != 0 {
        if kind == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_i(REACT + 0x90, 0);
            m.set_pv_f(REACT + 0x94, f32::from_bits(0x3f56_7752));
        }
        alarm(w, id);
        pass_lead(w, id);
        let g = w.mobys[id].group;
        if (kind == 4 || kind == 5) && g != -1 && 2 <= group_alive(w, g as i32) {
            for k in members(w, id) {
                let Some(o) = w.mobys.get_mut(k) else { break };
                if o.o_class == CLASS && o.pv_i(ENGAGED) == 0 {
                    o.set_pv_i(ENGAGED, 1);
                    break;
                }
            }
        }
    }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    let m = &w.mobys[id];
    if !matches!(m.state, 3 | 12 | 13 | 14 | 16) && m.pv_i(LEAD) != 0 && ctx.hero_moby.is_some() && ai::link(m, T_MOBY) == ctx.hero_moby {
        retarget(w, id, ctx);
    }
    false
}

/// `0x3c03d0`: the set-up.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let wake = ctx.game.rng.rand_range(0x78, 0x2bc);
    let m = &mut w.mobys[id];
    // `0x2effb8`: 15 and 12 as bytes.
    m.set_pv_b(HEALTH + 0x28, 15);
    m.set_pv_b(HEALTH + 0x2a, 12);
    m.scale *= f32::from_bits(0x3f8c_cccd);
    m.mode |= 0x8100;
    m.set_pv_i(0x18, MV as i32);
    m.set_pv_b(0x48, 3);
    m.set_pv_b(REACT + 0x10, 0x11);
    m.set_pv_i(REACT + 0x48, 4);
    m.set_pv_i(REACT + 0x7c, m.pv_i(0x270));
    m.set_pv_b(REACT + 0x15, 0xd);
    m.set_pv_f(REACT + 0x5c, f32::from_bits(0x3e99_999a));
    m.set_pv_f(REACT + 0x58, NEAR);
    m.set_pv_f(REACT + 0x20, TILT_MAX);
    m.set_pv_f(REACT + 0x24, f32::from_bits(0x3b88_8889));
    m.set_pv_f(REACT + 0x28, f32::from_bits(0x3c08_8889));
    m.set_pv_f(REACT + 0x30, 0.5);
    m.set_pv_f(REACT + 0x34, 0.5);
    m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
    m.set_pv_f(REACT + 0x1c, f32::from_bits(0x3dcc_ccce));
    let most = m.pv_i(0x2e8) as f32;
    if 0.0 < most { m.set_pv_f(REACT + 0x40, most); }
    m.set_pv_f(MV + 0x2c, 0.0);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x4149_0fdc));
    m.set_pv_f(MV, f32::from_bits(0x3f4c_cccd));
    m.set_pv_f(MV + 4, 0.5);
    m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 5);
    m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
    senses(m, FAST);
    let p = m.pos;
    m.set_pv_v(0x220, p);
    m.set_pv_f(0x23c, 3.0);
    m.set_pv_f(0x240, 10.0);
    m.set_pv_f(0x230, 1.0);
    m.set_pv_i(0x2d8, wake);
    m.set_pv_f(0x294, m.pv_f(0x294) * f32::from_bits(0x3c8e_fa36));
    m.set_pv_b(0x250, 0xb0);
    m.set_pv_b(0x251, 0x80);
    m.set_pv_b(0x252, 0x40);
    m.set_pv_i(SOUND, -1);
    m.set_pv_f(0x2bc, 0.0);
    m.set_pv_i(TICKS, 0);
    m.set_pv_v(0x340, p);
    m.set_pv_f(SPEED_Z, 0.0);
    m.set_pv_i(KICK, 0);
    m.set_pv_i(0x2cc, 0);
    m.set_pv_i(PATH_ON, 0);
    // The rope's top: 20 over joint 0, or the ceiling between.
    if let Some(j) = ctx.host.joint(&w.mobys[id], 0) {
        let top = add(j, [0.0, 0.0, 20.0]);
        let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, j, top, crate::collision::QueryFlags(0), Some(id)));
        let m = &mut w.mobys[id];
        m.set_pv_v(J0, j);
        m.set_pv_v(TOP, hit.map_or(top, |h| [top[0], top[1], h.point[2]]));
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 4);
    m.mode |= mode::KEEP_ROWS;
    m.set_pv_f(MV + 0x10, f32::from_bits(0x4149_0fdc));
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 1);
    if m.pv_i(IDLE) == 0 {
        if m.pv_i(PATH) != -1 {
            m.set_state(15, 3);
            set_seq(w, id, ctx, 7, 0, 0);
        } else {
            m.set_state(1, 0);
            set_seq(w, id, ctx, 0, 0, 0);
        }
    } else {
        m.set_state(3, 1);
        set_seq(w, id, ctx, 0xb, 0, 0);
    }
}

/// `0x3c07d0`: its group's living 0x09b4s alarmed (`+0x286`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for k in members(w, id) {
        let Some(o) = w.mobys.get_mut(k) else { break };
        if o.state < 0x80 && o.o_class == CLASS { o.set_pv_s(ALARM, 1); }
    }
}

/// `0x3c0e18`: the lead taken when none of its group's living 0x09b4s has it; whether it has it.
fn claim(w: &mut MobyWorld, id: usize) -> bool {
    if !has_group(w, id) { return false; }
    let taken = members(w, id).into_iter().any(|k| w.mobys.get(k).is_some_and(|o| o.state < 0x80 && o.o_class == CLASS && o.pv_i(LEAD) != 0));
    if !taken { w.mobys[id].set_pv_i(LEAD, 1); }
    w.mobys[id].pv_i(LEAD) != 0
}

/// `0x3c0ec0`: holding the lead, it goes to the group's living 0x09b4 longest in its state (not hurt, not itself);
/// whether it was passed.
fn pass_lead(w: &mut MobyWorld, id: usize) -> bool {
    if !has_group(w, id) || w.mobys[id].pv_i(LEAD) == 0 { return false; }
    let mut best: Option<(usize, i32)> = None;
    for k in members(w, id) {
        let Some(o) = w.mobys.get(k) else { continue };
        if o.sub_state == 4 || 0x80 <= o.state || o.o_class != CLASS { continue; }
        let t = o.state_ticks as u16 as i32;
        if best.is_none_or(|(_, b)| b < t) && k != id { best = Some((k, t)); }
    }
    let Some((k, _)) = best else { return false };
    w.mobys[k].set_pv_i(LEAD, 1);
    w.mobys[id].set_pv_i(LEAD, 0);
    true
}

/// `0x3c0fb0`: the nearest living member of its group that is no 0x09b4 and nearer than Ratchet as its target.
fn retarget(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    if !has_group(w, id) { return; }
    let p = w.mobys[id].pos;
    let mut best = dist(p, ctx.hero.pos);
    for k in members(w, id) {
        let Some(o) = w.mobys.get(k) else { continue };
        if 0x80 <= o.state || o.o_class == CLASS { continue; }
        let d = dist(p, o.pos);
        if d < best {
            ai::set_link(&mut w.mobys[id], T_MOBY, Some(k));
            best = d;
        }
    }
}

/// `0x3c1230(m, &n, keep)`: the normal of what is under it along its up (from 0.1 over to 3 under), else down its
/// up; with `keep` 0 the other way.
fn under_normal(w: &MobyWorld, id: usize, ctx: &Ctx, keep: bool) -> V3 {
    let m = &w.mobys[id];
    let up = m.rows[2];
    let (a, b) = (add(m.pos, set_len(up, 0.1)), add(m.pos, set_len(up, -3.0)));
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(2), Some(id)));
    let n = hit.map_or(set_len(up, -1.0), |h| set_len(h.normal, 1.0));
    if keep { n } else { scale(n, -1.0) }
}

/// `0x3c10e0(m, deg)`: its target (none: true) more than `deg` off its facing (its first row), both across the
/// surface under it.
fn off_aim(w: &MobyWorld, id: usize, ctx: &Ctx, deg: f32) -> bool {
    let Some(t) = target_place(w, id, ctx) else { return true };
    let n = under_normal(w, id, ctx, true);
    let m = &w.mobys[id];
    let fwd = m.rows[0];
    let d = set_len(sub(t, m.pos), 1.0);
    let d = set_len(sub(d, set_len(n, dot(n, d))), 1.0);
    let off = (FRAC_PI_2 - dot(d, fwd).clamp(-1.0, 1.0).asin()).abs();
    deg * f32::from_bits(0x3c8e_fa36) < off
}

/// `0x3c1348`: its rows turned toward its target ([`face`]).
fn face_target(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(t) = target_place(w, id, ctx) else { return };
    let v = sub(t, w.mobys[id].pos);
    face(&mut w.mobys[id], v);
}

/// `0x3c13b0(m, block, v, 0)`: the way `v` in its own frame (`0x3269a0`), the turn speed (`+0xb4`) eased toward it
/// (`0x31f230` at the movement block's limits, at most π), its rows turned by it about their up (`0x2efa00`).
fn face(m: &mut Moby, v: V3) {
    let r = m.rows;
    let yaw = atan2(dot(r[0], v), dot(r[1], v));
    let (a, b) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18));
    let (mut x, mut s) = (0.0, m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, PI, &mut x, &mut s, 0);
    m.set_pv_f(MV + 0xb4, s);
    m.rows = turn(&m.rows, &rows_of([0.0, 0.0, s]));
}

/// `0x3c0850`: in states 12, 13, 14 and 16 or sub 1 its rope (`0x3c09f0`: effect texture 14, additive, 0.05 to each
/// side across the view, `0x1f7f7f7f`, from the top `+0x2f0` to joint 0), then three glows (`0x3c0bd8`: effect
/// texture 0xb, pulled 0.5 toward the camera, a random roll, the template's ±2 square × the size, then half of it):
/// 0.125 at joint 1 in `0x36f6` (alphas 0x20 / 0x20), 0.15 at joint 2 in `0x1050b0` (0x30 / 0x20) and 0.15 at
/// `+0x330` (0x10 / 0x30).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if matches!(m.state, 12 | 13 | 14 | 16) || m.sub_state == 1 {
        let (p0, p1) = (m.pv_v(TOP), m.pv_v(J0));
        let a = set_len(sub(p1, p0), 1.0);
        let c = set_len(crate::hero::physics::cross(a, sub(cam, p0)), f32::from_bits(0x3d4c_cccd));
        out.push(crate::fx::FxQuad {
            fx: 14,
            additive: true,
            corners: [add(p0, c), sub(p0, c), add(p1, c), sub(p1, c)],
            st: [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            rgba: [0x1f7f_7f7f; 4],
        });
    }
    crate::fx::glow(out, rng, 0.125, m.pv_v(J1), 0x36f6, 0x20, 0x20, cam);
    crate::fx::glow(out, rng, 0.15, m.pv_v(J2), 0x10_50b0, 0x30, 0x20, cam);
    crate::fx::glow(out, rng, 0.15, m.pv_v(J3), 0x10_50b0, 0x10, 0x30, cam);
}
