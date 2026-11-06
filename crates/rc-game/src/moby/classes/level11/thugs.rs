//! Joba's thug **0x0cf5** (`0x422488`, 34 placed; its senses `0x426260`, tail `0x427268`; L11 addresses), the
//! level's own copy of the riders' code (Barlow's rider, crate::moby::classes::level04::rider, is the same family) on
//! the framework ([`crate::moby::ai`]): health block `+0x20`, the colour flash `+0x50`, reaction `+0x60` (a hurt goes
//! to state 0x24, sub 4), the wander block `+0x110`, movement `+0x140` (its joint turns asked on the aim record
//! `+0x2a0`). The decomp has its v1.01 body as `func_L11_00420C88` / `func_L11_00424A60` (the v2.00 ones checked
//! against them: the same but for the hit-source list and the count's guard below).
//!
//! Its words: the near and far paths `+0x250` / `+0x254` and ranges `+0x258` / `+0x25c` of the range test (`+0x264`,
//! [`range_test`], `0x33c970`; who it found `+0x268`), the timer `+0x260` (s16), alarmed `+0x266` (s16); riding a
//! hover `+0x26c` (the moby `+0x270`, kept at `+0x274`; the saddle 0x0ff9 `+0x278`, the seat target 0x0911 `+0x280`),
//! the turn speed `+0x27c`, the run path `+0x284`, its fence `+0x288`, its damage `+0x28c`, the attack flag `+0x290`
//! (the damage intake's flag), the charge `+0x294`, its yaw offset `+0x298` (degrees, then radians), the weapon part
//! 0x09ac `+0x29c`; its path network `+0x320` (16 records, crate::path_network, the count `+0x520`, from the path
//! `+0x524`, used when `+0x528` ≠ 0, the records it and its goal are in `+0x530` / `+0x534`, the walk speed `+0x538`,
//! stuck steps `+0x53c`); the chat `+0x540` (1 / 2 / 3) with the moby `+0x544` (kept at `+0x55c`), the turret
//! `+0x548` (−1 none), the patrol path `+0x54c`, the turret's turn and climb springs `+0x550` / `+0x554`, the alarm
//! path `+0x558`, the alarms counted `+0x560`, the turret's path `+0x564` and its point `+0x568`, the path home
//! `+0x56c`, `+0x570` a swing first, the hiding path `+0x574` (while Ratchet is in state 0x31), `+0x578` its idle ends
//! early, the reaction path `+0x57c`, a moby to run to `+0x580` (hooked once it is there), the turret kills `+0x584`,
//! `+0x588` hidden once, `+0x58c` the slow turret.
//!
//! **The set-up** (state 0, [`senses`]): update distance 0x80, the level's thug count (`$gp−0x338`,
//! [`super::Globals::g338`]) up unless `0x1a9050` is set; on the ground; with the network it builds it (`0x33cac0` =
//! crate::path_network::build); its yaw offset (none: ±15..25°) in radians; mirrored at random (mode 0x8000 and the
//! once bit 2); the reaction (state 0x24 sub 4, sequences 0x16 / 0x13 / 0x14, speeds 0.2 / 0.125, gravities 1/180 /
//! 7/720 / 1/240, keys, the hurt share 0.18, its path `+0x57c`), movement (radius / height 0.75, steps 0.6, slope 89°,
//! gravity 1/720, the turn before walking 85°, stop 0.25, its fence `+0x288`), the calm limits, its damage. On foot:
//! idle (4, sub 0, sequence 0x24), or chatting (5: sequences 0x1c / 0x1e from frame 5 / 0x1e), or along its patrol
//! (0xe, sequence 0x10; 0xf with `+0x578`); its weapon on joint 3. On a hover (sub 5): its place and angles, no
//! collision, the hover's rider word `+0x27c` itself, the seat 0.75 above it (not drawn), the saddle on its joint 1
//! (no hover: deleted). The wander's home its place (radius 1, range 8, 10 from Ratchet).
//!
//! **Each tick** ([`senses`]): its shadow within 24 of the camera; `+0x2e` = 2: its parts and itself deleted (off a
//! turret it rode: the turret back to state 4, sequence 0); its weapon on joint 3; at 1 health or less it drops (a
//! smoke burst); the frame's damage (flag `+0x290`) — any: a hit by anything but a thug's own kind (0x0ed8, 0x0047,
//! 0x0dd9, 0x0d1b, 0x0ce7, a hover 0x0d07) clears the enemies' skill point flag `0x1a904c` (crate::moby::classes::
//! level11::props::update_12c7), off its turret unless a graze, playback 1, with the attack flag eight pairs of sparks
//! at joint 3 (crate::moby::classes::level04::rider::sparks, the same function), a knock-down's spin and the weapon
//! gone, the flash 0xd2 red, its group alarmed ([`alarm`], not in sub 2), a kill or knock-down gives the charge to its
//! group's first thug without it (two alive or more); the saddle on joint 1; the range test; off sub 4 and state 0x25
//! it falls 0.313 a tick to the ground, and more than 20 above the ground (off a hover) it falls (0x25, sub 9,
//! sequence 0x23); under z 75 (after the sixth frame) its bolts, the charge handed on, the weapon gone, the count down.
//!
//! **Subs** (`+0x95`, [`update`]) and **states** ([`states`]) follow the original's two switches; see the comments
//! there. The tail (`0x427268`): the flash, the timers `+0x260` / `+0x26` down, its ticks, the aim record sprung
//! (crate::moby::joint_aim, list 4, 0.03 / 0.3) and the head aim (3.5); a height change under 0.005 undone.
//!
//! Not ported here: the turret 0x0c20 and the hover 0x0d07 it rides (their own updates, crate::moby::classes::level11's
//! list); the thug only writes the words they share (the turret's state byte, sequence, playback speed,
//! angles and height; the hover's `+0x27c` rider link, crate::moby::ai::link, and `+0x2c8`). The turret's shots 0x0dd9
//! ([`shot`], `0x451d60`) are made only when the level has the class. The hit's source is the frame's hit record's
//! (the original reads its health block's `+0x38`); the debug prints (`0x2bdc70`) are left out.

use crate::hero::physics::{add, add_rot, approach, atan2, dist2, ease, euler_rows, rot_dist, set_len, spring, sub, sub_rot, turn_ease, V3};
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use super::super::props::{group_alive, seq, set_seq, wrapped};
use super::super::leaper::seq_a;

/// The class, its cousin 0x0cf6 (alarmed with it), the hover it rides.
pub const CLASS: i16 = 0x0cf5;
const COUSIN: i16 = 0x0cf6;
const HOVER: i16 = 0x0d07;
const SADDLE_CLASS: i16 = 0x0ff9;
const SEAT_CLASS: i16 = 0x0911;
const WEAPON_CLASS: i16 = 0x09ac;
const SHOT_CLASS: i16 = 0x0dd9;

const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
const WANDER: usize = 0x110;
const MV: usize = 0x140;
const PATH_NEAR: usize = 0x250;
const PATH_FAR: usize = 0x254;
const RANGE_NEAR: usize = 0x258;
const RANGE_FAR: usize = 0x25c;
const T260: usize = 0x260;
const RANGE: usize = 0x264;
const ALARMED: usize = 0x266;
const WHO: usize = 0x268;
const RIDES: usize = 0x26c;
const HOVER_I: usize = 0x270;
const HOVER_LINK: usize = 0x274;
const SADDLE: usize = 0x278;
const TURN_V: usize = 0x27c;
const SEAT: usize = 0x280;
const APATH: usize = 0x284;
const FENCE: usize = 0x288;
const DAMAGE: usize = 0x28c;
const ATTACK: usize = 0x290;
const CHARGE: usize = 0x294;
const OFFSET: usize = 0x298;
const WEAPON: usize = 0x29c;
const AIM: usize = 0x2a0;
/// The aim record's asked turn about z (`+0x60` of the record).
const HEAD_TURN: usize = AIM + 0x68;
const NODES: usize = 0x320;
const NODES_MAX: usize = 16;
const N: usize = 0x520;
const START: usize = 0x524;
const NET: usize = 0x528;
const AT: usize = 0x530;
const GOAL: usize = 0x534;
const SPEED: usize = 0x538;
const STUCK: usize = 0x53c;
const CHAT: usize = 0x540;
const PARTNER_I: usize = 0x544;
const TURRET: usize = 0x548;
const PATROL: usize = 0x54c;
const SPRING_V: usize = 0x550;
const CLIMB: usize = 0x554;
const ALARM_PATH: usize = 0x558;
const PARTNER: usize = 0x55c;
const ALARMS: usize = 0x560;
const T_PATH: usize = 0x564;
const T_POINT: usize = 0x568;
const HOME_PATH: usize = 0x56c;
const SWING_FIRST: usize = 0x570;
const HIDE_PATH: usize = 0x574;
const EARLY: usize = 0x578;
const REACT_PATH: usize = 0x57c;
const RUN_TO: usize = 0x580;
const T_KILLS: usize = 0x584;
const HIDDEN: usize = 0x588;
const SLOW: usize = 0x58c;
const PVARS: usize = 0x590;

/// The framework's limits (`0x340930(walk accel, walk most, turn accel, turn most, m)`).
const CALM: [u32; 4] = [0x3ac8_3fb8, 0x3d3b_bbbc, 0x3a98_825c, 0x3dd6_7752];
const RUN: [u32; 4] = [0x3b7e_dcbb, 0x3dee_eef0, 0x3a98_825c, 0x3dd6_7752];
const RUN_TURN: [u32; 4] = [0x3b7e_dcbb, 0x3dee_eef0, 0x3b5e_68c6, 0x3e86_0a93];
const RUN_PATH: [u32; 4] = [0x3b7e_dcbb, 0x3dee_eef0, 0x3b77_d3d5, 0x3e97_e9da];
const WALK: [u32; 4] = [0x3a11_a2b4, 0x3c88_8889, 0x3a98_825c, 0x3dd6_7752];
const PATROL_L: [u32; 4] = [0x3ac8_3fb8, 0x3d3b_bbbc, 0x3b77_d3d5, 0x3e97_e9da];
const PATROL_FAST: [u32; 4] = [0x3ac8_3fb8, 0x3d3b_bbbc, 0x3bf7_d3d5, 0x3f17_e9da];
const TURRET_L: [u32; 4] = [0x3b7e_dcbb, 0x3dee_eef0, 0x3afe_2e99, 0x3bbe_a2f4];

const DEG: f32 = 0.017_453_294;
const HALF_PI: f32 = 1.570_796_5;

fn limits(w: &mut MobyWorld, id: usize, l: [u32; 4]) {
    let [a, b, c, d] = l.map(f32::from_bits);
    ai::senses(&mut w.mobys[id], a, b, c, d);
}

/// `0x33d580`: the first tick since the last sub-state change.
fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: u8) { set_seq(w, id, ctx, s, frame, blend); }
/// [`anim`] with its blend drawn from `lo..=hi` only when the sequence is to change.
fn anim_r(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, lo: i32, hi: i32) {
    if seq(w, id, ctx) != s {
        let b = ctx.game.rng.rand_range(lo, hi);
        ctx.host.set_anim(&w.mobys[id], s, frame, b.clamp(0, 255) as u8);
    }
}
fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }
fn speed(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: f32) { ctx.host.set_anim_speed(&w.mobys[id], s); }

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }
/// Across to Ratchet (`0x3013f0` with `0x189ea0`).
fn hdist(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { dist2(w.mobys[id].pos, ctx.hero.pos) }
fn faces_hero(w: &MobyWorld, id: usize, ctx: &Ctx, within: f32) -> bool { rot_dist(w.mobys[id].rot[2], yaw_to(w, id, ctx.hero.pos)) < within }
fn in_range(w: &MobyWorld, id: usize) -> bool { (1..=2).contains(&w.mobys[id].pv_s(RANGE)) }

/// `0x33f8e0(yaw, m, pos)`: the frame's move in place, facing `p`.
fn face(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) {
    let y = yaw_to(w, id, p);
    let pos = w.mobys[id].pos;
    ai::move_step(w, id, ctx, y, pos);
}
/// `0x33f6c0(m, goal)`: the move toward `goal`.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    let y = yaw_to(w, id, goal);
    ai::move_step(w, id, ctx, y, goal);
}
/// `0x32e5d0(0.001, 0.01, 0.1, m, Ratchet's moby, +0x27c)`: the facing springs toward Ratchet.
fn turn_hero(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let y = yaw_to(w, id, ctx.hero.pos);
    ai::turn_to(&mut w.mobys[id], y, 0.001, 0.01, 0.1, TURN_V);
}

/// The level path `i`'s points.
fn path(ctx: &Ctx, i: i32) -> Option<Vec<V3>> {
    usize::try_from(i).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()).map(|p| p.iter().map(|q| [q[0], q[1], q[2]]).collect())
}

/// `0x340aa8(m, path, last, first)`: a walk along the path from point `first` to `last` (−1: its last point).
fn path_start(w: &mut MobyWorld, id: usize, ctx: &Ctx, i: i32, last: i32, first: i32) {
    let Some(p) = path(ctx, i) else { return };
    let m = &mut w.mobys[id];
    let o = ai::mv(m);
    m.set_pv_i(o + 0xc4, i + 1);
    m.set_pv_s(o + 0xca, if last == -1 { p.len() as i16 - 1 } else { last as i16 });
    m.set_pv_s(o + 0xc8, first as i16);
}

/// The walk along path `off` from its nearer end to the other.
fn from_nearer_end(w: &mut MobyWorld, id: usize, ctx: &Ctx, off: usize) {
    let i = w.mobys[id].pv_i(off);
    let Some(p) = path(ctx, i) else { return };
    let pos = w.mobys[id].pos;
    let last = p.len() as i32 - 1;
    if dist2(pos, p[p.len() - 1]) < dist2(pos, p[0]) { path_start(w, id, ctx, i, 0, last) } else { path_start(w, id, ctx, i, last, 0) }
}

fn follow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) -> bool {
    let p = w.mobys[id].pv_i(off);
    ai::follow_path(w, id, ctx, p)
}

fn index(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&i| i < w.mobys.len()) }
fn turret(w: &MobyWorld, id: usize) -> Option<usize> { index(w, id, TURRET) }
/// The turret is not in its states 5 / 6 (taken).
fn turret_free(w: &MobyWorld, t: usize) -> bool { 1 < w.mobys[t].state.wrapping_sub(5) }

/// The climb onto the turret (state 0x1d, sub 2; the turret's state byte 6, its timer byte `+0xbc` 1).
fn to_turret(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: usize) {
    anim(w, id, ctx, 0xf, 0, 10);
    st(w, id, 0x1d, 2);
    w.mobys[t].state = 6;
    w.mobys[id].timer = 1;
}

/// Off the turret it rides (`+0xbc` 1): the turret back to state 4, sequence 0, both at playback 1.
fn leave_turret(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = turret(w, id) else { return };
    if w.mobys[t].state == 5 { return; }
    w.mobys[t].state = 4;
    ctx.host.set_anim(&w.mobys[t], 0, 0, 5);
    speed(w, t, ctx, 1.0);
    speed(w, id, ctx, 1.0);
    w.mobys[id].timer = 0;
}

/// `0x33c970(near, far, m, &who, near path, far path, 0)`: Ratchet (the moby kept at `who`) in the near range (2; in
/// the near path's polygon when it has one) or the far one (1), outside play neither.
fn range_test(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> i16 {
    let m = &mut w.mobys[id];
    ai::set_link(m, WHO, ctx.hero_moby);
    if ctx.game.scenes.mode != crate::scene::PLAY { return 0; }
    let hero = ctx.hero.pos;
    let d = dist2(m.pos, hero);
    let (np, fp) = (m.pv_i(PATH_NEAR), m.pv_i(PATH_FAR));
    if if np == -1 { d < m.pv_f(RANGE_NEAR) } else { ctx.level.in_polygon(np, hero) } { return 2; }
    if if fp == -1 { d < m.pv_f(RANGE_FAR) } else { ctx.level.in_polygon(fp, hero) } { return 1; }
    0
}

/// A part: its update distance, draw distance, light and ambient its owner's, drawn, rebuilt; at its owner's `joint`
/// (`0x3374c0`, crate::moby::classes::enemies::place_part).
fn make_part(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16, joint: usize, solid: bool) -> Option<usize> {
    w.class(class)?;
    let (ud, dd, light, amb) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.ambient) };
    let p = w.create(class);
    let pm = &mut w.mobys[p];
    pm.update_dist = ud;
    pm.draw_dist = dd;
    pm.drawn = 1;
    pm.light = light;
    pm.ambient = amb;
    w.rebuild(p, &*ctx.host);
    if !solid { w.mobys[p].has_collision = false; }
    super::super::enemies::place_part(w, id, p, joint, ctx);
    Some(p)
}

/// The seat target 0x0911 0.75 above it (its update distance, not drawn).
fn make_seat(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> Option<usize> {
    w.class(SEAT_CLASS)?;
    let (ud, pos) = (w.mobys[id].update_dist, w.mobys[id].pos);
    let s = w.create(SEAT_CLASS);
    w.mobys[s].update_dist = ud;
    w.mobys[s].drawn = 0;
    w.rebuild(s, &*ctx.host);
    w.mobys[s].pos = add(pos, [0.0, 0.0, 0.75]);
    Some(s)
}

fn part(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&p| w.mobys.get(p).is_some_and(|m| m.state < 0x80)) }

/// The part at `off` deleted (`0x323a00`), with a smoke burst at it first (`0x333678(0.5, 3, m, part, −1)`).
fn drop_part(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize, smoke: bool) {
    if let Some(p) = part(w, id, off) {
        if smoke {
            let at = w.mobys[p].pos;
            crate::moby::blast::smoke_burst(w, ctx, 0.5, 3.0, Some(id), at, -1);
        }
        w.delete(p);
    }
    ai::set_link(&mut w.mobys[id], off, None);
}

/// Its bolts (`0x32d0e8(m, 0, 3)`).
fn bolts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { crate::moby::bolts::drop(w, id, 0, 3, ctx); }

fn members(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}
fn alive(w: &MobyWorld, i: usize) -> bool { w.mobys.get(i).is_some_and(|m| m.state < 0x80) }
/// Its group has two live members or more (`0x32ecd0(group, −1)`).
fn group_pair(w: &MobyWorld, id: usize) -> bool { let g = w.mobys[id].group as i32; 0 <= g && 1 < group_alive(w, g) }

/// The charge (`+0x294`) given to the first of its group without it (`thugs_only`: of its class).
fn hand_charge(w: &mut MobyWorld, id: usize, thugs_only: bool) {
    if !group_pair(w, id) { return; }
    let first = members(w, id).into_iter().find(|&i| (!thugs_only || w.mobys[i].o_class == CLASS) && w.mobys[i].pv_i(CHARGE) == 0);
    if let Some(i) = first { w.mobys[i].set_pv_i(CHARGE, 1); }
}

/// `0x427310`: its group's live thugs and 0x0cf6 alarmed (`+0x266`) and their alarms counted (`+0x560`); a thug's
/// turret kills (`+0x584`) counted too when it is dead.
fn alarm(w: &mut MobyWorld, id: usize) {
    let dead = w.mobys[id].pv_f(HEALTH) <= 0.0;
    for i in members(w, id) {
        let m = &mut w.mobys[i];
        if 0x80 <= m.state || !matches!(m.o_class, CLASS | COUSIN) { continue; }
        m.set_pv_s(ALARMED, 1);
        m.set_pv_i(ALARMS, m.pv_i(ALARMS) + 1);
        if m.o_class == CLASS && dead { m.set_pv_i(T_KILLS, m.pv_i(T_KILLS) + 1); }
    }
}

/// The first live thug of its group in the chat role 1 (`+0x540`).
fn chat_one(w: &MobyWorld, id: usize) -> Option<usize> {
    members(w, id).into_iter().find(|&i| alive(w, i) && w.mobys[i].o_class == CLASS && w.mobys[i].pv_i(CHAT) == 1)
}

/// The weapon 0x09ac on its joint 3 (no collision); false when it cannot be made.
fn arm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let Some(g) = make_part(w, id, ctx, WEAPON_CLASS, 3, false) else { return false };
    ai::set_link(&mut w.mobys[id], WEAPON, Some(g));
    true
}

/// The saddle 0x0ff9 on its joint 1, the attack flag off; false when it cannot be made.
fn saddle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let Some(p) = make_part(w, id, ctx, SADDLE_CLASS, 1, true) else { return false };
    let m = &mut w.mobys[id];
    ai::set_link(m, SADDLE, Some(p));
    m.set_pv_i(ATTACK, 0);
    true
}

/// `0x426260` (module docs). True when it is to be deleted.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.update_dist = 0x80;
        if w.level11.g9050 == 0 { w.level11.g338 += 1; }
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, MV as i32);
        let z = ai::ground_z(ctx, 0.5, m.pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        for o in [CLIMB, ALARMS, T_POINT] { m.set_pv_i(o, 0); }
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 14);
        if m.pv_i(NET) != 0 {
            let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
            let n = crate::path_network::build(ctx.level, m.pv_i(START), &mut nodes);
            crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
            m.set_pv_i(N, n as i32);
            crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
        }
        let mut off = w.mobys[id].pv_f(OFFSET);
        if off == 0.0 { off = ctx.game.rng.randf_signed(15.0, 25.0); }
        w.mobys[id].set_pv_f(OFFSET, off * DEG);
        if ctx.game.rng.randi(2) == 1 {
            let m = &mut w.mobys[id];
            m.mode |= 0x8000;
            m.once |= 4;
        }
        let m = &mut w.mobys[id];
        for (o, b) in [(0x70, 0x24u8), (0x71, 0x16), (0x72, 0x13), (0x76, 0x14), (0xcc, 5), (0xcd, 10), (0xce, 8), (0xcf, 0xf), (0xd2, 6), (0xd3, 0xb)] { m.set_pv_b(o, b); }
        m.set_pv_i(0xa8, 4);
        for (o, v) in [(0x7c, 0x3e4c_ccceu32), (0x80, 0x3e00_0000), (0x84, 0x3bb6_0b61), (0x88, 0x3c1f_49f5), (0x8c, 0x3b88_8889), (0x10c, 0x3f00_0000), (0xb8, 0x3e38_51ec), (0x90, 0x3f40_0000), (0x94, 0x3f40_0000), (0x9c, 0)] { m.set_pv_f(o, f32::from_bits(v)); }
        if m.pv_i(REACT_PATH) != -1 { m.set_pv_i(0xdc, m.pv_i(REACT_PATH)); }
        m.set_pv_i(MV + 0x48, AIM as i32);
        m.set_pv_f(MV + 0x34, 0.25);
        m.set_pv_f(MV, 0.75);
        m.set_pv_f(MV + 4, 0.75);
        m.set_pv_f(MV + 8, f32::from_bits(0x3f19_999a));
        m.set_pv_f(MV + 0xc, f32::from_bits(0x3f19_999a));
        if m.pv_i(FENCE) != -1 { m.set_pv_i(MV + 0x3c, m.pv_i(FENCE)); }
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3fbd_e450));
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc6_d3f3));
        m.set_pv_f(MV + 0x2c, f32::from_bits(0x3ab6_0b61));
        limits(w, id, CALM);
        let m = &mut w.mobys[id];
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 1);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_f(DAMAGE, m.pv_b(0x34) as f32);
        m.set_pv_b(0x2e, 1);
        if m.pv_i(RIDES) == 0 {
            st(w, id, 4, 0);
            anim(w, id, ctx, 0x24, 0, 0);
            match w.mobys[id].pv_i(CHAT) {
                c @ (1 | 2) => {
                    let p = index(w, id, PARTNER_I);
                    ai::set_link(&mut w.mobys[id], PARTNER, p);
                    st(w, id, 5, 0);
                    if c == 1 { anim(w, id, ctx, 0x1c, 0, 0) } else { anim(w, id, ctx, 0x1e, 5, 0) }
                }
                3 => {
                    if let Some(p) = chat_one(w, id) { ai::set_link(&mut w.mobys[id], PARTNER, Some(p)); }
                    st(w, id, 5, 0);
                    anim(w, id, ctx, 0x1e, 0, 0);
                }
                _ => {
                    if w.mobys[id].pv_i(PATROL) != -1 {
                        st(w, id, 0xe, 0);
                        anim(w, id, ctx, 0x10, 0, 0);
                        if w.mobys[id].pv_i(EARLY) != 0 { st(w, id, 0xf, 0); }
                    }
                }
            }
            if !arm(w, id, ctx) { return true; }
        } else {
            st(w, id, 1, 5);
            let Some(h) = index(w, id, HOVER_I) else {
                w.level11.g338 -= 1;
                w.mobys[id].set_pv_i(RIDES, 0);
                st(w, id, 4, 0);
                return true;
            };
            let (hp, hr) = (w.mobys[h].pos, w.mobys[h].rot);
            let m = &mut w.mobys[id];
            ai::set_link(m, HOVER_LINK, Some(h));
            m.pos = hp;
            m.rot = hr;
            m.has_collision = false;
            m.mode &= 0xafff;
            ai::set_link(&mut w.mobys[h], 0x27c, Some(id));
            let Some(s) = make_seat(w, id, ctx) else { return true };
            ai::set_link(&mut w.mobys[id], SEAT, Some(s));
            if !saddle(w, id, ctx) { return true; }
        }
        if w.mobys[id].pv_i(HOVER_I) != -1 && part(w, id, SADDLE).is_none() && !saddle(w, id, ctx) { return true; }
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(WANDER, pos);
        m.set_pv_f(WANDER + 0x10, 1.0);
        m.set_pv_f(WANDER + 0x1c, 8.0);
        m.set_pv_f(WANDER + 0x20, 10.0);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].pv_b(0x2e) == 2 {
        drop_part(w, id, ctx, WEAPON, false);
        drop_part(w, id, ctx, SADDLE, false);
        drop_part(w, id, ctx, SEAT, false);
        if w.mobys[id].timer == 1 { leave_turret(w, id, ctx); }
        return true;
    }
    if let Some(g) = part(w, id, WEAPON) { super::super::enemies::place_part(w, id, g, 3, ctx); }
    if w.mobys[id].pv_f(HEALTH) <= 1.0 {
        drop_part(w, id, ctx, WEAPON, true);
        w.mobys[id].set_pv_i(ATTACK, 0);
    }
    let source = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0).and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
    let flag = w.mobys[id].pv_i(ATTACK) != 0;
    let kind = ai::take_damage(w, id, ctx, flag);
    if kind != 0 {
        if source.is_some_and(|c| !matches!(c, 0x0ed8 | 0x0047 | 0x0dd9 | 0x0d1b | 0x0ce7 | HOVER | 0)) { w.level11.g904c = 0; }
        if w.mobys[id].timer == 1 && kind != 1 { leave_turret(w, id, ctx); }
        if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.speed != 1.0) { speed(w, id, ctx, 1.0); }
        if w.mobys[id].pv_i(ATTACK) != 0 {
            if let Some(at) = ctx.host.joint(&w.mobys[id], 3) { super::super::level04::rider::sparks(ctx, at, 8); }
        }
        if kind == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_f(0xf4, f32::from_bits(0x3f56_7752));
            m.set_pv_f(0xf0, 0.0);
            drop_part(w, id, ctx, WEAPON, true);
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(ATTACK, 0);
        m.set_pv_b(FLASH + 7, 0xd2);
        ai::flash_start(m, FLASH);
        if w.mobys[id].sub_state != 2 { alarm(w, id); }
        if matches!(kind, 4 | 5) { hand_charge(w, id, true); }
    }
    if w.mobys[id].pv_i(HOVER_I) != -1 {
        if let Some(p) = part(w, id, SADDLE) { super::super::enemies::place_part(w, id, p, 1, ctx); }
    }
    let r = range_test(w, id, ctx);
    w.mobys[id].set_pv_s(RANGE, r);
    if w.mobys[id].sub_state != 4 && w.mobys[id].state != 0x25 {
        let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = (m.pos[2] - f32::from_bits(0x3ea0_6d3a)).max(g);
    }
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    let z = w.mobys[id].pos[2];
    if 20.0 < z - g && g < z && w.mobys[id].sub_state != 5 {
        st(w, id, 0x25, 9);
        anim(w, id, ctx, 0x23, 0, 10);
    }
    if w.mobys[id].pos[2] < 75.0 && 6 <= ctx.game.frame {
        bolts(w, id, ctx);
        hand_charge(w, id, false);
        drop_part(w, id, ctx, WEAPON, true);
        w.level11.g338 -= 1;
        return true;
    }
    false
}

/// `0x426fa0(yaw, speed, near, offset, m, goal)`: the walk through its network toward `goal`: farther than `near`
/// across, the heading through the network (crate::path_network::steer_offset, `offset`, 0.4 narrowing) eased to
/// (0.4°, 0.2°, 12°) and, within 30° of it, the speed eased toward the distance (`speed`/1200, at most `speed`/60);
/// nearer, `yaw` eased to (0.4°, 0.2, 12°) and the speed down by `speed`/1200 — stopped and within 5° of `yaw`, done.
/// The step along its facing on walkable ground (`0x32e668(0.75, 0.75, 0.4, 30°)`); 30 steps that fail, done.
#[allow(clippy::too_many_arguments)]
fn net_walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: f32, speed: f32, near: f32, off: f32, goal: V3) -> bool {
    let mut done = false;
    let m = &w.mobys[id];
    let pos = m.pos;
    let l = dist2(goal, pos);
    let step = speed * 0.000_833_333_35;
    let nodes = crate::path_network::read_nodes(&m.pvars, NODES, (m.pv_i(N).max(0) as usize).min(NODES_MAX));
    let m = &mut w.mobys[id];
    let (mut a, mut v, mut s) = (m.rot[2], m.pv_f(TURN_V), m.pv_f(SPEED));
    if near < l {
        let (mut y, mut at, mut to) = (0.0, m.pv_i(AT), m.pv_i(GOAL));
        crate::path_network::steer_offset(ctx.level, &nodes, off, f32::from_bits(0x3ecc_cccd), pos, goal, &mut y, &mut at, &mut to);
        m.set_pv_i(AT, at);
        m.set_pv_i(GOAL, to);
        turn_ease(y, f32::from_bits(0x3be4_c38a), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3e56_7752), &mut a, &mut v, 0);
        if rot_dist(a, y) < 0.523_598_85 {
            let mut x = 0.0;
            ease(l, step, step, speed * 0.016_666_668, &mut x, &mut s);
        } else {
            s = 0.0;
        }
    } else {
        turn_ease(yaw, f32::from_bits(0x3be4_c38a), 0.2, f32::from_bits(0x3e56_7752), &mut a, &mut v, 0);
        approach(0.0, step, &mut s);
        if s == 0.0 && rot_dist(a, yaw) < 0.087_266_475 {
            v = 0.0;
            done = true;
        }
    }
    m.rot[2] = a;
    m.set_pv_f(TURN_V, v);
    m.set_pv_f(SPEED, s);
    let mut target = add(set_len(m.rows[0], s), pos);
    let mut p = pos;
    let ok = ai::step_walk(ctx, id, 0.75, 0.75, f32::from_bits(0x3ecc_cccd), f32::from_bits(0x3f06_0a93), &mut p, &mut target, 0);
    let m = &mut w.mobys[id];
    m.pos = p;
    w.changed = true;
    if !ok { m.set_pv_i(STUCK, m.pv_i(STUCK) + 1); }
    if 0x1d < m.pv_i(STUCK) {
        m.set_pv_i(STUCK, 0);
        done = true;
    }
    done
}

/// Toward Ratchet: with the network [`net_walk`] (7, 0.25), else the framework's move with its yaw offset
/// (`0x33f7f0`). True when the network walk is done.
fn chase(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let (goal, off) = (ctx.hero.pos, w.mobys[id].pv_f(OFFSET));
    if w.mobys[id].pv_i(NET) == 0 {
        ai::move_toward(w, id, ctx, off, goal);
        false
    } else {
        let y = w.mobys[id].rot[2];
        net_walk(w, id, ctx, y, 7.0, 0.25, off, goal)
    }
}

/// The attack start of subs 0 and 1: the charge (sequence 0x22 from a frame `frame`, 0x18 sub 2), the swing first
/// (0x21, 0x18 sub 3) or the plain attack (the attack flag, sequence 0xe blended `blend`, 0x18 sub 3).
#[allow(clippy::too_many_arguments)]
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, charge_seq: u8, rand_frame: bool, swing_blend: u8, blend: u8, swing: bool) {
    if w.mobys[id].pv_i(CHARGE) == 1 {
        if seq(w, id, ctx) != charge_seq {
            let f = if rand_frame { ctx.game.rng.randi(4) as u8 } else { 0 };
            let b = ctx.game.rng.rand_range(7, 20) as u8;
            ctx.host.set_anim(&w.mobys[id], charge_seq, f, b);
        }
        st(w, id, 0x18, 2);
    } else if swing && w.mobys[id].pv_i(SWING_FIRST) != 0 {
        anim(w, id, ctx, 0x21, 0, swing_blend);
        st(w, id, 0x18, 3);
    } else {
        w.mobys[id].set_pv_i(ATTACK, 1);
        anim(w, id, ctx, 0xe, 0, blend);
        st(w, id, 0x18, 3);
    }
}

/// The hiding path while Ratchet is in his state 0x31 (`+0x574`): `state` sub 8 (the run-and-turn limits for 9).
fn hide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, state: u8, limits_too: bool) {
    if ctx.hero.state != 0x31 || w.mobys[id].pv_i(HIDE_PATH) == -1 { return; }
    anim_r(w, id, ctx, 0xf, 0, 5, 15);
    st(w, id, state, 8);
    if limits_too { limits(w, id, RUN_TURN); }
}

/// 0x0cf5 (module docs): the subs (`+0x95`), then [`states`].
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 0 { return; }
    if ctx.hero.state == 0x6f && w.mobys[id].sub_state != 4 { return; }
    let z0 = w.mobys[id].pos[2];
    if senses(w, id, ctx) {
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        // Ratchet with item 0x41 near: its head turned to him (±90°); else back to calm.
        10 => {
            if hdist(w, id, ctx) < 6.0 {
                let mut a = sub_rot(yaw_to(w, id, ctx.hero.pos), w.mobys[id].rot[2]);
                if w.mobys[id].once & 4 != 0 { a = -a; }
                w.mobys[id].set_pv_f(HEAD_TURN, a.clamp(-HALF_PI, HALF_PI));
            }
            if ctx.hero.items.item != 0x41 { w.mobys[id].sub_state = 0; }
        }
        0 => calm(w, id, ctx),
        1 => {
            if first_sub(w, id) { w.mobys[id].set_pv_f(WANDER + 0x1c, 6.0); }
            if let Some(t) = turret(w, id) {
                if turret_free(w, t) {
                    clear_walk_flag(w, id);
                    to_turret(w, id, ctx, t);
                }
            }
            if !in_range(w, id) { return states_and_tail(w, id, ctx, z0); }
            clear_walk_flag(w, id);
            let ap = w.mobys[id].pv_i(APATH);
            if ap != -1 {
                anim(w, id, ctx, 0xf, 0, 10);
                st(w, id, 0x1e, 2);
                let pos = w.mobys[id].pos;
                let mut best = (100.0, 0);
                for (k, p) in path(ctx, ap).unwrap_or_default().iter().enumerate() {
                    let d = dist2(pos, *p);
                    if d < best.0 { best = (d, k as i32); }
                }
                path_start(w, id, ctx, ap, -1, best.1);
            }
            match turret(w, id) {
                Some(t) if turret_free(w, t) => {
                    clear_walk_flag(w, id);
                    to_turret(w, id, ctx, t);
                }
                Some(_) => attack(w, id, ctx, 0x19, true, 0, 40, false),
                None => attack(w, id, ctx, 0x22, true, 0, 40, false),
            }
        }
        2 => {
            if first_sub(w, id) { limits(w, id, RUN_PATH); }
            let s = w.mobys[id].state;
            if !in_range(w, id) && !matches!(s, 0x1d | 0x22 | 0x20) && ctx.game.rng.randi(4) == 1 {
                speed(w, id, ctx, 1.0);
                match ctx.game.rng.randi(4) {
                    0 => anim(w, id, ctx, 0xe, 0, 10),
                    2 => anim(w, id, ctx, 0x1e, 0, 20),
                    _ => anim(w, id, ctx, 0x18, 0, 20),
                }
                st(w, id, 0x13, 1);
                w.mobys[id].set_pv_i(ALARMS, 0);
                let t = ctx.game.rng.rand_range(580, 600);
                w.mobys[id].set_pv_s(T260, t as i16);
            }
            if let Some(t) = turret(w, id).filter(|&t| turret_free(w, t)) { to_turret(w, id, ctx, t); }
            hide(w, id, ctx, 9, true);
        }
        3 => {
            if first_sub(w, id) { limits(w, id, RUN); }
            if w.mobys[id].pv_i(CHARGE) == 1 {
                anim(w, id, ctx, 0xf, 0, 10);
                st(w, id, 0x1c, 2);
            }
            if !in_range(w, id) {
                anim(w, id, ctx, 0x1a, 0, 30);
                st(w, id, 0x13, 1);
                let t = ctx.game.rng.rand_range(580, 600);
                let m = &mut w.mobys[id];
                m.set_pv_s(T260, t as i16);
                m.set_pv_i(ALARMS, 0);
            }
            hide(w, id, ctx, 9, true);
        }
        4 => {
            if 0.0 < w.mobys[id].pv_f(HEALTH) { let h = ctx.hero.pos; face(w, id, ctx, h); }
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                w.level11.g338 -= 1;
                bolts(w, id, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, None);
                return;
            }
            if r & 2 != 0 {
                w.mobys[id].set_pv_s(HEALTH + 6, 0);
                if in_range(w, id) {
                    anim(w, id, ctx, 0xf, 0, 10);
                    st(w, id, 0x1c, 2);
                    if hdist(w, id, ctx) < 2.5 && faces_hero(w, id, ctx, 0.610_865_3) { lunge(w, id, ctx, 9, 4, 2); }
                    if w.mobys[id].pv_b(0x37) == 10 { lunge(w, id, ctx, 9, 4, 2); }
                    hide(w, id, ctx, 8, false);
                } else if w.mobys[id].pv_i(ALARM_PATH) != -1 {
                    clear_walk_flag(w, id);
                    anim_r(w, id, ctx, 0xf, 0, 5, 15);
                    st(w, id, 7, 1);
                    limits(w, id, RUN_TURN);
                } else {
                    anim(w, id, ctx, 0x18, 0, 10);
                    st(w, id, 0x13, 1);
                    let t = ctx.game.rng.rand_range(580, 600);
                    w.mobys[id].set_pv_s(T260, t as i16);
                }
            }
        }
        5 => riding(w, id, ctx),
        6 => {
            if first_sub(w, id) && part(w, id, WEAPON).is_none() {
                if !arm(w, id, ctx) {
                    w.delete(id);
                    return;
                }
                let m = &mut w.mobys[id];
                m.set_pv_i(ATTACK, 1);
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.rows = euler_rows(m.rot);
            }
        }
        7 => on_turret(w, id, ctx),
        8 => {
            if ctx.hero.state != 0x31 {
                if let Some(t) = turret(w, id) {
                    to_turret(w, id, ctx, t);
                } else if w.mobys[id].pv_i(CHARGE) != 0 {
                    anim(w, id, ctx, 0xf, 0, 10);
                    st(w, id, 0x1c, 2);
                } else {
                    anim(w, id, ctx, 0x19, 0, 10);
                    st(w, id, 0x18, 2);
                }
            }
        }
        // Falling (state 0x25): down to the ground on a spring.
        9 => {
            let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            let m = &mut w.mobys[id];
            let (mut z, mut v) = (m.pos[2], m.pv_f(SPRING_V));
            spring(g, f32::from_bits(0x3a11_a2b4), f32::from_bits(0x3991_a2b4), f32::from_bits(0x3ea0_6d3a), &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(SPRING_V, v);
        }
        _ => {}
    }
    states_and_tail(w, id, ctx, z0);
}

fn states_and_tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, z0: f32) {
    if w.mobys[id].state >= 0x80 { return; }
    states(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    tail(w, id, ctx);
    let m = &mut w.mobys[id];
    if (m.pos[2] - z0).abs() < 0.005 { m.pos[2] = z0; }
}

/// The movement block's walk flag (`+0x178` bit 0x10) off.
fn clear_walk_flag(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x10);
}

/// The lunge 0x22 (sequence 0x11 from `frame` blended `blend`) after the run limits, sub `sub`.
fn lunge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, frame: u8, blend: u8, sub: i32) {
    anim(w, id, ctx, 0x11, frame, blend);
    limits(w, id, RUN);
    st(w, id, 0x22, sub);
}

/// Sub 0, calm: Ratchet near (range 2) sends it to its run path, its turret, the moby to run to, or the attack (the
/// charge, the swing first, or the attack — then off to its hover `+0x270`, 0x1f, when it has one); alarmed by its
/// group (`+0x266`): its alarm path (6 for the first alarm, 7 after) or alert (sub 1), then the same picks in range.
fn calm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if first_sub(w, id) { limits(w, id, CALM); }
    if w.mobys[id].pv_s(RANGE) == 2 {
        if w.mobys[id].pv_i(APATH) != -1 {
            anim(w, id, ctx, 0xf, 0, 10);
            st(w, id, 0x1e, 2);
        }
        if let Some(t) = turret(w, id) {
            if turret_free(w, t) {
                to_turret(w, id, ctx, t);
            } else {
                attack(w, id, ctx, 0x22, false, 10, 5, true);
            }
            return;
        }
        if w.mobys[id].pv_i(RUN_TO) != -1 {
            anim(w, id, ctx, 0xf, 0, 10);
            st(w, id, 0x20, 2);
            return;
        }
        attack(w, id, ctx, 0x22, false, 10, 5, true);
        let Some(h) = index(w, id, HOVER_I) else { return };
        ai::set_link(&mut w.mobys[id], HOVER_LINK, Some(h));
        limits(w, id, RUN_TURN);
        anim(w, id, ctx, 0xf, 0, 10);
        st(w, id, 0x1f, 2);
        return;
    }
    if w.mobys[id].pv_s(ALARMED) == 0 { return; }
    let (ap, alarms) = (w.mobys[id].pv_i(ALARM_PATH), w.mobys[id].pv_i(ALARMS));
    if ap != -1 && alarms == 1 {
        anim_r(w, id, ctx, 0x18, 0, 5, 15);
        st(w, id, 6, -1);
    } else if ap != -1 && 1 < alarms {
        anim_r(w, id, ctx, 0xf, 0, 5, 15);
        st(w, id, 7, -1);
        limits(w, id, RUN_TURN);
    } else {
        w.mobys[id].sub_state = 1;
        anim(w, id, ctx, 0x18, 0, 20);
    }
    if in_range(w, id) {
        if w.mobys[id].pv_i(APATH) != -1 {
            anim(w, id, ctx, 0xf, 0, 10);
            st(w, id, 0x1e, 2);
        } else if w.mobys[id].pv_i(RUN_TO) != -1 {
            anim(w, id, ctx, 0xf, 0, 10);
            st(w, id, 0x20, 2);
        } else {
            attack(w, id, ctx, 0x22, false, 5, 10, true);
        }
    }
    w.mobys[id].set_pv_s(ALARMED, 0);
    if let Some(t) = turret(w, id) {
        if turret_free(w, t) { to_turret(w, id, ctx, t); }
        w.mobys[id].set_pv_s(ALARMED, 1);
    }
}

/// Sub 5, on its hover (`+0x274`): the saddle on joint 1, its place and angles the hover's, the seat 0.65 above it;
/// the hover gone, dying (states 0xfe / 0xfd) or its rider word dropped (`+0x26c` 0) — or a hit on the seat (flags
/// 0x10000, `0x422438`: its damage off the health; sequence 9) — the saddle drops (smoke), the seat goes, the hover's
/// rider word cleared, off it (2, or 3 after the hit; sub 6).
fn riding(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(p) = part(w, id, SADDLE) { super::super::enemies::place_part(w, id, p, 1, ctx); }
    let hover = ai::link(&w.mobys[id], HOVER_LINK).filter(|&h| h < w.mobys.len());
    if let Some(h) = hover {
        let (hp, hr) = (w.mobys[h].pos, w.mobys[h].rot);
        let m = &mut w.mobys[id];
        m.pos = hp;
        m.rot = hr;
        if let Some(s) = part(w, id, SEAT) {
            let sm = &mut w.mobys[s];
            sm.pos = add(hp, [0.0, 0.0, 0.65]);
            sm.rot = hr;
        }
    }
    let gone = w.mobys[id].pv_i(RIDES) == 0 || hover.is_none_or(|h| matches!(w.mobys[h].state, 0xfe | 0xfd));
    if !gone {
        let hit = part(w, id, SEAT).and_then(|s| w.hit_message(s)).filter(|h| h.tmpl.flags & 0x10000 != 0);
        let Some(h) = hit else { return };
        let m = &mut w.mobys[id];
        m.set_pv_f(HEALTH, m.pv_f(HEALTH) - h.tmpl.damage);
    }
    drop_part(w, id, ctx, SADDLE, true);
    drop_part(w, id, ctx, SEAT, false);
    if let Some(h) = hover { ai::set_link(&mut w.mobys[h], 0x27c, None); }
    if gone {
        st(w, id, 2, 6);
    } else {
        anim(w, id, ctx, 9, 0, 0);
        st(w, id, 3, 6);
    }
}

/// Sub 7, on its turret (`+0x548`): the turret and it at playback 0.65 (0.35 slow) while the turret's sequence 1
/// plays, else 1; off its path (not 0x28) facing Ratchet, the turret's gun pitched toward his aim point (±30°, its
/// aim record `+0x70`'s angle, list 2, 0.03 / 0.3); it sits 1.5 above the turret, the turret's rows from its angles.
/// Out of range (not 0x26 / 0x28): along the turret's path (0x28, sequences 0x1f / 0 on the turret); on it with
/// Ratchet near: the firing (0x27, sequences 0x20 / 1).
fn on_turret(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(ATTACK, 0);
    let Some(t) = turret(w, id) else { return };
    let k = if seq(w, t, ctx) == 1 { if w.mobys[id].pv_i(SLOW) != 0 { 0.35 } else { 0.65 } } else { 1.0 };
    speed(w, t, ctx, k);
    speed(w, id, ctx, k);
    if w.mobys[id].state != 0x28 {
        let h = ctx.hero.pos;
        face(w, id, ctx, h);
        let (p, aim) = (w.mobys[id].pos, ctx.hero.aim_point);
        let pitch = atan2(dist2(p, aim), aim[2] - p[2]).clamp(-0.523_598_85, 0.523_598_85);
        w.mobys[t].set_pv_f(0xb4, -pitch);
        crate::moby::joint_aim::aim(w, t, 0x70, t, 2, 0.03, 0.3);
    }
    seat_on_turret(w, id, t);
    let s = w.mobys[id].state;
    if !in_range(w, id) && s != 0x26 && s != 0x28 {
        anim(w, id, ctx, 0x1f, 0, 10);
        ctx.host.set_anim(&w.mobys[t], 0, 0, 10);
        st(w, id, 0x28, -1);
    }
    if w.mobys[id].state == 0x28 && w.mobys[id].pv_s(RANGE) == 2 {
        limits(w, id, RUN);
        anim(w, id, ctx, 0x20, 0, 10);
        st(w, id, 0x27, -1);
        ctx.host.set_anim(&w.mobys[t], 1, 0, 10);
    }
}

/// It sits 1.5 above the turret; the turret's rows from its angles (`0x301a78`).
fn seat_on_turret(w: &mut MobyWorld, id: usize, t: usize) {
    let tp = w.mobys[t].pos;
    let rot = w.mobys[id].rot;
    let m = &mut w.mobys[id];
    m.pos = [tp[0], tp[1], tp[2] + 1.5];
    w.mobys[t].rows = euler_rows(rot);
    w.changed = true;
}

/// `0x451d60(damage, owner, dir, at)`: the turret's shot 0x0dd9 at `at` heading `dir` (its pvars: `+0x00` the way,
/// `+0x10` the owner, `+0x14` the damage, `+0x18` a random angle, `+0x1c` its spin 12..36°, `+0x20` 0.15, `+0x24` 0),
/// update and draw distance 0xff, drawn, the owner's light, alpha 0x80, mode | 0x341, scale 0.005..0.00625; and two
/// twinkles at it for 4 ticks (`0x801060e0` 0.3·s → s, white 0.04·s → 0.4·s; s 2..4). None without the class.
fn shot(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, owner: usize, dir: V3, at: V3) {
    if w.class(SHOT_CLASS).is_none() { return; }
    let (light, amb) = (w.mobys[owner].light, w.mobys[owner].ambient);
    let s = w.create(SHOT_CLASS);
    let m = &mut w.mobys[s];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.light = light;
    m.ambient = amb;
    m.pos = at;
    m.set_pv_v(0, dir);
    m.set_pv_f(0x14, damage);
    ai::set_link(m, 0x10, Some(owner));
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let spin = rng.randf(f32::from_bits(0x3e56_7752), f32::from_bits(0x3f20_d97d));
    let m = &mut w.mobys[s];
    m.set_pv_f(0x18, a);
    m.set_pv_f(0x1c, spin);
    m.set_pv_f(0x20, f32::from_bits(0x3e19_999a));
    m.alpha = 0x80;
    m.mode |= 0x100;
    let sc = ctx.game.rng.randf(f32::from_bits(0x3ba3_d70a), f32::from_bits(0x3bcc_cccd));
    let m = &mut w.mobys[s];
    m.scale = sc;
    m.set_pv_i(0x24, 0);
    m.mode |= 0x200 | 0x41;
    w.rebuild(s, &*ctx.host);
    let f = ctx.game.rng.randf(2.0, 4.0);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    crate::particles::twinkle(sys, rng, f * f32::from_bits(0x3e99_999a), f, 0.0, at, 4, 0x8010_60e0, 0, 10, [0.0; 3], 0);
    crate::particles::twinkle(sys, rng, f * f32::from_bits(0x3d23_d70a), f * f32::from_bits(0x3ecc_cccd), 0.0, at, 4, 0xffff_ffff, 1, -10, [0.0; 3], 0);
}

/// The turret's gun at frames 0 / 6 / 12 of its sequence fires from its joint 1, at 3 / 9 / 15 from joint 0
/// (`0x3363a0`); the shot's way from `way` (the joint's place given), its damage the health block's byte `+0x36`.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: usize, way: impl Fn(&MobyWorld, &Ctx, V3) -> V3) {
    let at = |w: &MobyWorld, ctx: &Ctx, ts: [f32; 3]| ts.iter().any(|&f| ai::reached(w, t, ctx, f));
    let j = if at(&*w, &*ctx, [6.0, 12.0, 0.0]) { 1 } else if at(&*w, &*ctx, [3.0, 9.0, 15.0]) { 0 } else { return };
    let Some(p) = ctx.host.joint(&w.mobys[t], j) else { return };
    let d = way(&*w, &*ctx, p);
    let dmg = w.mobys[id].pv_b(0x36) as f32;
    shot(w, ctx, dmg, t, d, p);
}

/// From `p` toward Ratchet's aim point, 0.5 long (`0x3014f8`).
fn at_aim(_: &MobyWorld, ctx: &Ctx, p: V3) -> V3 { set_len(sub(ctx.hero.aim_point, p), 0.5) }

/// The second switch, on the state (the original's order).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = w.mobys[id].state;
    match state {
        // Thrown: at its end collision back, mode | 0x5000, the stand 0x13 (sub 1, the timer 480..600 when under 300,
        // sequence 0xd), not dead (0.1 at least); on the ground from frame 10, between frames 1.5 and 17 a step back
        // and to its left on walkable ground (`0x32e668(0.5, 0.55, 0.5, 30°)`).
        2 | 3 => {
            if wrapped(w, id, ctx) {
                let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.mode |= 0x5000;
                m.has_collision = solid;
                st(w, id, 0x13, 1);
                if w.mobys[id].pv_s(T260) < 300 {
                    let t = ctx.game.rng.rand_range(480, 600);
                    w.mobys[id].set_pv_s(T260, t as i16);
                }
                anim(w, id, ctx, 0xd, 0, 10);
                let m = &mut w.mobys[id];
                if m.pv_f(HEALTH) <= 0.0 { m.set_pv_f(HEALTH, 0.1); }
            }
            let t = ctx.host.frame_time(&w.mobys[id]);
            if 10.0 <= t {
                let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
                w.mobys[id].pos[2] = z;
            }
            if !(1.5..=17.0).contains(&t) { return; }
            let m = &w.mobys[id];
            let yaw = m.rot[2];
            let back = add_rot(yaw, std::f32::consts::PI);
            let left = add_rot(yaw, HALF_PI);
            let pos = m.pos;
            let mut goal = add(add(pos, [left.cos() * 0.05, left.sin() * 0.05, 0.0]), [back.cos() * 0.15, back.sin() * 0.15, 0.0]);
            let mut p = pos;
            ai::step_walk(ctx, id, 0.5, f32::from_bits(0x3f0c_cccd), 0.5, f32::from_bits(0x3f06_0a93), &mut p, &mut goal, 0);
            w.mobys[id].pos = p;
            w.changed = true;
        }
        5 => chat(w, id, ctx),
        6 => {
            w.mobys[id].set_pv_i(ATTACK, 1);
            if wrapped(w, id, ctx) {
                st(w, id, 7, -1);
                anim(w, id, ctx, 0x10, 0, 30);
                w.mobys[id].set_pv_i(ATTACK, 0);
            }
        }
        7 => {
            if follow(w, id, ctx, ALARM_PATH) {
                st(w, id, 0xb, -1);
                anim(w, id, ctx, 0x18, 0, 20);
                if w.mobys[id].sub_state == 1 && w.mobys[id].pv_i(HOME_PATH) != -1 { st(w, id, 0x17, -1); }
            }
        }
        8 => {
            if follow(w, id, ctx, HIDE_PATH) {
                st(w, id, 0xb, -1);
                anim(w, id, ctx, 0x18, 0, 20);
            }
        }
        9 => {
            let first = path(ctx, w.mobys[id].pv_i(HIDE_PATH)).map(|p| p[0]);
            if let Some(p) = first {
                go(w, id, ctx, p);
                if dist2(w.mobys[id].pos, p) < 0.5 {
                    anim(w, id, ctx, 0x25, 0, 10);
                    st(w, id, 10, -1);
                }
            }
            if w.mobys[id].pv_i(HIDDEN) != 0 {
                anim_r(w, id, ctx, 0xf, 0, 5, 15);
                st(w, id, 8, 8);
            }
        }
        10 => {
            if wrapped(w, id, ctx) {
                anim_r(w, id, ctx, 0xf, 0, 5, 15);
                st(w, id, 8, 8);
                w.mobys[id].set_pv_i(HIDDEN, 1);
            }
        }
        // At its path's end: two of its group alive or more, a chat with another (`0x32f018`).
        0xb => {
            if wrapped(w, id, ctx) {
                limits(w, id, CALM);
                if group_pair(w, id) {
                    let live: Vec<usize> = members(w, id).into_iter().filter(|&i| alive(w, i)).collect();
                    let other = match live.first() { Some(&f) if f == id => live.get(1).copied(), f => f.copied() };
                    ai::set_link(&mut w.mobys[id], PARTNER, other);
                    st(w, id, 0xd, -1);
                    anim(w, id, ctx, 0x1c, 0, 30);
                    w.mobys[id].set_pv_i(CHAT, 1);
                    if other.is_some_and(|o| seq_a(w, o, ctx) == 0x1c) {
                        anim(w, id, ctx, 0x1e, 0, 30);
                        w.mobys[id].set_pv_i(CHAT, 2);
                    }
                }
            }
        }
        0xd => {
            let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
            let p = w.mobys[o].pos;
            face(w, id, ctx, p);
            if rot_dist(w.mobys[id].rot[2], yaw_to(w, id, p)) < 0.087_266_475 { st(w, id, 5, -1); }
        }
        0xe => {
            limits(w, id, PATROL_L);
            if follow(w, id, ctx, PATROL) {
                st(w, id, 0x11, -1);
                anim(w, id, ctx, 0xd, 0, 20);
            }
        }
        // Along the patrol, its head turned toward Ratchet (each tick added to the turn asked, ±90°).
        0xf => {
            limits(w, id, PATROL_FAST);
            if follow(w, id, ctx, PATROL) {
                from_nearer_end(w, id, ctx, PATROL);
                if ctx.game.rng.randi(5) == 1 {
                    anim(w, id, ctx, 0x19, 0, 15);
                    st(w, id, 0x10, -1);
                }
            }
            let mut a = sub_rot(yaw_to(w, id, ctx.hero.pos), w.mobys[id].rot[2]);
            if w.mobys[id].once & 4 != 0 { a = add_rot(a, std::f32::consts::PI); }
            let a = a.clamp(-HALF_PI, HALF_PI);
            let v = add_rot(a, w.mobys[id].pv_f(HEAD_TURN));
            w.mobys[id].set_pv_f(HEAD_TURN, v.clamp(-HALF_PI, HALF_PI));
        }
        0x10 => {
            let h = ctx.hero.pos;
            face(w, id, ctx, h);
            if wrapped(w, id, ctx) {
                st(w, id, 0xf, -1);
                anim(w, id, ctx, 0x10, 0, 20);
            }
        }
        0x11 => {
            if wrapped(w, id, ctx) {
                from_nearer_end(w, id, ctx, PATROL);
                st(w, id, 0xe, -1);
                anim(w, id, ctx, 0x10, 0, 20);
            }
        }
        // The stand: turning to Ratchet; at its sequence's end the walk home (its home the home path's first point).
        0x13 => {
            turn_hero(w, id, ctx);
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 0x17, 0, 20);
                st(w, id, 0x14, -1);
                let m = &mut w.mobys[id];
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x10);
                let hp = m.pv_i(HOME_PATH);
                if let Some(p) = path(ctx, hp) { w.mobys[id].set_pv_v(WANDER, p[0]); }
            }
        }
        0x14 => {
            w.mobys[id].set_pv_i(ATTACK, 1);
            turn_hero(w, id, ctx);
            limits(w, id, WALK);
            let home = w.mobys[id].pv_v(WANDER);
            go(w, id, ctx, home);
            if dist2(w.mobys[id].pos, home) < 2.0 {
                clear_walk_flag(w, id);
                st(w, id, 0x15, -1);
                anim(w, id, ctx, 0x18, 0, 20);
                if w.mobys[id].pv_i(HOME_PATH) != -1 { st(w, id, 0x17, -1); }
            }
        }
        0x16 => {
            limits(w, id, PATROL_L);
            if follow(w, id, ctx, HOME_PATH) {
                st(w, id, 0x17, -1);
                anim(w, id, ctx, 0x18, 0, 20);
            }
        }
        0x17 => {
            if wrapped(w, id, ctx) {
                from_nearer_end(w, id, ctx, HOME_PATH);
                st(w, id, 0x16, -1);
                anim(w, id, ctx, 0x10, 0, 20);
            }
        }
        // The swing's wind-up: facing Ratchet; at its end (or at once with `+0x578`, or in sequence 0xf) the strafe
        // (within 8, 0x19) or the chase (0x1a, sub 3), the attack flag; with the charge the charge (0x1c, sub 2).
        0x18 => {
            w.mobys[id].set_pv_i(ATTACK, 0);
            let h = ctx.hero.pos;
            face(w, id, ctx, h);
            if !wrapped(w, id, ctx) && w.mobys[id].pv_i(EARLY) == 0 && seq_a(w, id, ctx) != 0xf { return; }
            strafe_or_chase(w, id, ctx);
        }
        0x19 => {
            turn_hero(w, id, ctx);
            if 12.0 < hdist(w, id, ctx) {
                anim(w, id, ctx, 0xf, 0, 10);
                st(w, id, 0x1a, -1);
            }
            if hdist(w, id, ctx) < 2.5 {
                anim(w, id, ctx, 0x11, 0, 10);
                st(w, id, 0x22, -1);
            }
            if ctx.game.rng.randi(3) == 1 && wrapped(w, id, ctx) {
                anim(w, id, ctx, 0x19, 0, 10);
                if ctx.game.rng.randi(3) == 1 { anim(w, id, ctx, 0x18, 0, 10); }
                st(w, id, 0x18, 3);
            }
        }
        0x1a => {
            if chase(w, id, ctx) && w.mobys[id].pv_i(NET) != 0 {
                anim(w, id, ctx, 0x15, 0, 10);
                st(w, id, 0x19, -1);
            }
            if hdist(w, id, ctx) < 8.0 {
                anim(w, id, ctx, 0x15, 0, 10);
                st(w, id, 0x19, -1);
            }
        }
        0x1b => {
            let d = hdist(w, id, ctx);
            if d < 2.5 && faces_hero(w, id, ctx, 0.349_065_9) {
                lunge(w, id, ctx, 0, 6, 2);
            } else if d < 2.5 {
                let h = ctx.hero.pos;
                face(w, id, ctx, h);
            } else if 5.0 < d {
                limits(w, id, RUN);
                anim(w, id, ctx, 0xf, 0, 10);
                st(w, id, 0x1c, -1);
            } else {
                limits(w, id, WALK);
                let (off, h) = (w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                ai::move_toward(w, id, ctx, off, h);
            }
        }
        // The charge: within 3.5 with over 1 health the close walk (0x1b, sequence 0x17, the attack flag); within
        // 2.5 the lunge or facing him; else toward him — at the network walk's end the charge handed to the first
        // of its group without it that is nearer him and alive (then this one's swing, 0x18 sub 3).
        0x1c => {
            let d = hdist(w, id, ctx);
            if d < 3.5 && 1.0 < w.mobys[id].pv_f(HEALTH) {
                w.mobys[id].set_pv_i(ATTACK, 1);
                limits(w, id, WALK);
                st(w, id, 0x1b, -1);
                anim(w, id, ctx, 0x17, 0, 10);
            } else if d < 2.5 && faces_hero(w, id, ctx, 0.349_065_9) {
                lunge(w, id, ctx, 0, 10, 2);
            } else if d < 2.5 {
                let h = ctx.hero.pos;
                face(w, id, ctx, h);
            } else if w.mobys[id].pv_i(NET) == 0 {
                let (off, h) = (w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                ai::move_toward(w, id, ctx, off, h);
            } else if chase(w, id, ctx) && group_pair(w, id) {
                for i in members(w, id) {
                    if w.mobys[i].pv_i(CHARGE) != 0 { continue; }
                    let mine = hdist(w, id, ctx);
                    if hdist(w, i, ctx) < mine && 0.0 < w.mobys[i].pv_f(HEALTH) {
                        w.mobys[i].set_pv_i(CHARGE, 1);
                        w.mobys[id].set_pv_i(CHARGE, 0);
                        anim(w, id, ctx, 0xe, 0, 10);
                        st(w, id, 0x18, 3);
                    }
                    break;
                }
            }
        }
        // To its turret: the turret (unless taken, 5) turned with it on a spring and raised toward its pvar `+0x5c`
        // less 1.2; within 1.5 across: onto it (the turret's state 6), 1.5 above it, the climb 0x26 (sub 7).
        0x1d => {
            let Some(t) = turret(w, id) else { return };
            if w.mobys[t].state != 5 {
                let mut r = w.mobys[t].rot;
                let target = w.mobys[id].rot;
                let mut v = w.mobys[id].pv_f(SPRING_V);
                ai::spring_to(&mut r, target, f32::from_bits(0x3a98_825c), f32::from_bits(0x3b18_825c), f32::from_bits(0x3dd6_7752), &mut v);
                w.mobys[id].set_pv_f(SPRING_V, v);
                let tm = &mut w.mobys[t];
                tm.rot = r;
                tm.rows = euler_rows(r);
                let lift = tm.pv_f(0x5c);
                tm.pos[2] += ((lift - 1.2) - tm.pos[2]) * 0.11;
            }
            let tp = w.mobys[t].pos;
            go(w, id, ctx, tp);
            if dist2(w.mobys[id].pos, tp) < 1.5 && w.mobys[t].state != 5 {
                w.mobys[t].state = 6;
                let m = &mut w.mobys[id];
                m.pos = [tp[0], tp[1], tp[2] + 1.5];
                m.set_pv_f(CLIMB, tp[2] + 0.75);
                anim(w, id, ctx, 0x1f, 0, 25);
                st(w, id, 0x26, 7);
                let m = &mut w.mobys[id];
                m.set_pv_i(T_KILLS, 0);
                m.timer = 1;
            }
        }
        // The run path: within 2.5 of Ratchet the lunge (its path gone, the walk accel 1/1200); else along it
        // (accel 1/240) and at its end the charge, or (the attack flag) a third of the time the swing, else the chase.
        0x1e => {
            if hdist(w, id, ctx) < 2.5 {
                anim(w, id, ctx, 0x11, 0, 2);
                st(w, id, 0x22, -1);
                let m = &mut w.mobys[id];
                m.set_pv_i(APATH, -1);
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3a5a_740e));
            } else {
                w.mobys[id].set_pv_f(MV + 0x24, f32::from_bits(0x3b88_8889));
                if follow(w, id, ctx, APATH) {
                    if w.mobys[id].pv_i(CHARGE) == 1 {
                        anim(w, id, ctx, 0xf, 0, 10);
                        st(w, id, 0x1c, 2);
                    } else {
                        w.mobys[id].set_pv_i(ATTACK, 1);
                        if ctx.game.rng.randi(3) == 1 {
                            anim(w, id, ctx, 0xe, 0, 10);
                            st(w, id, 0x18, 3);
                        } else {
                            anim(w, id, ctx, 0xf, 0, 10);
                            st(w, id, 0x1a, 3);
                        }
                    }
                    w.mobys[id].set_pv_i(APATH, -1);
                }
            }
        }
        0x1f => {
            let Some(h) = ai::link(&w.mobys[id], HOVER_LINK).filter(|&h| h < w.mobys.len()) else { return };
            let hp = w.mobys[h].pos;
            go(w, id, ctx, hp);
            if dist2(w.mobys[id].pos, hp) < 2.1 {
                anim(w, id, ctx, 0, 0, 20);
                st(w, id, 0x21, -1);
            }
        }
        // To the moby `+0x580`: there, the strafe or chase (as 0x18's end) and that moby hooked (`0x33d5a8(m, 5)`).
        0x20 => {
            let Some(o) = index(w, id, RUN_TO) else { return };
            let op = w.mobys[o].pos;
            go(w, id, ctx, op);
            if dist2(w.mobys[id].pos, op) < 0.25 {
                strafe_or_chase(w, id, ctx);
                w.mobys[o].once |= 4;
            }
        }
        // Onto its hover: turned to the hover's facing (`0x32e468(…, 0.003, 0.03, 6°)`), 0.1 toward it a tick
        // (`0x3014f8`); within 0.05 across: riding (1, sub 5) — its place and angles, no collision, the hover's rider word,
        // the seat 0.75 above it, `+0x26c` 1.
        0x21 => {
            let Some(h) = ai::link(&w.mobys[id], HOVER_LINK).filter(|&h| h < w.mobys.len()) else { return };
            let (hp, hr) = (w.mobys[h].pos, w.mobys[h].rot);
            let m = &mut w.mobys[id];
            ai::turn_to(m, hr[2], 0.003, 0.03, f32::from_bits(0x3dd6_7752), TURN_V);
            let d = set_len(sub(hp, m.pos), f32::from_bits(0x3dcc_ccce));
            m.pos = add(m.pos, d);
            w.changed = true;
            if dist2(w.mobys[id].pos, hp) < 0.05 {
                st(w, id, 1, 5);
                let m = &mut w.mobys[id];
                m.pos = hp;
                m.rot = hr;
                m.has_collision = false;
                m.mode &= 0xafff;
                let hm = &mut w.mobys[h];
                ai::set_link(hm, 0x27c, Some(id));
                hm.set_pv_i(0x2c8, 0);
                let s = make_seat(w, id, ctx);
                ai::set_link(&mut w.mobys[id], SEAT, s);
                w.mobys[id].set_pv_i(RIDES, 1);
            }
        }
        // The lunge: sequence 0x11's frames 10..15 two hit spheres (0.4, its damage, flags 1) 0.5 and 1.5 ahead of
        // joint 2, else playback 1; at its end the recovery 0x23 (sequence 0x22, half the time 0xe); out of sub 2 the
        // strafe (0x19, sequence 0x15).
        0x22 => {
            w.mobys[id].set_pv_i(ATTACK, 0);
            let t = ctx.host.frame_time(&w.mobys[id]);
            let joint = ctx.host.joint(&w.mobys[id], 2);
            match joint.filter(|_| seq_a(w, id, ctx) == 0x11 && (10.0..=15.0).contains(&t)) {
                Some(j) => {
                    let yaw = w.mobys[id].rot[2];
                    let dmg = w.mobys[id].pv_f(DAMAGE);
                    let mut p = add(j, [yaw.cos() * 0.5, yaw.sin() * 0.5, 0.0]);
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3ecc_cccd), dmg, 1.0, p, 1, 0, 1, 0);
                    p = add(p, [yaw.cos(), yaw.sin(), 0.0]);
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3ecc_cccd), dmg, 1.0, p, 1, 0, 1, 0);
                }
                None => speed(w, id, ctx, 1.0),
            }
            if !wrapped(w, id, ctx) { return; }
            anim(w, id, ctx, 0x22, 0, 10);
            if ctx.game.rng.randi(2) == 1 { anim(w, id, ctx, 0xe, 0, 10); }
            st(w, id, 0x23, -1);
            if w.mobys[id].sub_state != 2 {
                st(w, id, 0x19, -1);
                anim(w, id, ctx, 0x15, 0, 10);
            }
        }
        0x23 => {
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 0xf, 0, 10);
                st(w, id, 0x1c, -1);
            }
        }
        // The climb onto the turret: the turret's height on a spring toward `+0x554`, it 1.5 above; there, the firing
        // (0x27, sequence 0x20; the turret's 1); facing Ratchet within 15° the turret's sequence 1; then the gun.
        0x26 => {
            let Some(t) = turret(w, id) else { return };
            let target = w.mobys[id].pv_f(CLIMB);
            let (mut z, mut v) = (w.mobys[t].pos[2], w.mobys[id].pv_f(SPRING_V));
            spring(target, f32::from_bits(0x3991_a2b4), f32::from_bits(0x3ada_740e), f32::from_bits(0x3d08_8889), &mut z, &mut v);
            w.mobys[t].pos[2] = z;
            w.mobys[id].set_pv_f(SPRING_V, v);
            w.mobys[id].pos[2] = z + 1.5;
            if (z - target).abs() < 0.1 {
                anim(w, id, ctx, 0x20, 0, 5);
                st(w, id, 0x27, -1);
                set_seq(w, t, ctx, 1, 0, 5);
            }
            if faces_hero(w, id, ctx, 0.261_799_42) && seq(w, t, ctx) != 1 {
                ctx.host.set_anim(&w.mobys[t], 1, 0, 5);
                anim(w, id, ctx, 0x20, 0, 5);
                return;
            }
            if seq_a(w, t, ctx) != 1 { return; }
            fire(w, id, ctx, t, at_aim);
        }
        0x27 => {
            let Some(t) = turret(w, id) else { return };
            fire(w, id, ctx, t, at_aim);
        }
        // Along the turret's path (two points, `+0x564` / `+0x568`): turning toward the point (its limits), sitting
        // on the turret; facing it within 5° the next point, and after 120 ticks of sequence 0x20 the stop (0x1f,
        // the turret's 0); a turret kill counted (`+0x584`): the firing again (0x20 / 1, its ticks 0); with 0x20 on
        // the gun along its facing, pitched to Ratchet's aim point.
        0x28 => {
            let tp = w.mobys[id].pv_i(T_PATH);
            if tp == -1 { return; }
            limits(w, id, TURRET_L);
            let Some(t) = turret(w, id) else { return };
            let pts = path(ctx, tp).unwrap_or_default();
            let k = w.mobys[id].pv_i(T_POINT).max(0) as usize;
            if let Some(&node) = pts.get(k).or(pts.last()) {
                face(w, id, ctx, node);
                seat_on_turret(w, id, t);
                if rot_dist(w.mobys[id].rot[2], yaw_to(w, id, node)) <= 0.087_266_475 {
                    let m = &mut w.mobys[id];
                    let n = m.pv_i(T_POINT) + 1;
                    m.set_pv_i(T_POINT, if 1 < n { 0 } else { n });
                    if seq_a(w, id, ctx) == 0x20 && 120 < w.mobys[id].state_ticks as u16 {
                        anim(w, id, ctx, 0x1f, 0, 10);
                        ctx.host.set_anim(&w.mobys[t], 0, 0, 10);
                    }
                }
            }
            if w.mobys[id].pv_i(T_KILLS) != 0 {
                anim(w, id, ctx, 0x20, 0, 5);
                ctx.host.set_anim(&w.mobys[t], 1, 0, 5);
                let m = &mut w.mobys[id];
                m.set_pv_i(T_KILLS, 0);
                m.state_ticks = 0;
            }
            if seq_a(w, id, ctx) != 0x20 { return; }
            let yaw = w.mobys[id].rot[2];
            fire(w, id, ctx, t, move |_, ctx, p| {
                let aim = ctx.hero.aim_point;
                let pitch = atan2(dist2(p, aim), aim[2] - p[2]);
                [yaw.cos() * 0.5 * pitch.cos(), yaw.sin() * 0.5 * pitch.cos(), pitch.sin() * 0.5]
            });
        }
        _ => {}
    }
}

/// Within 8 of Ratchet the strafe (0x19, sequence 0x15), else the chase (0x1a sub 3, sequence 0xf), the attack
/// flag either way; with the charge the charge (0x1c, sub 2).
fn strafe_or_chase(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(ATTACK, 1);
    if hdist(w, id, ctx) < 8.0 {
        anim(w, id, ctx, 0x15, 0, 10);
        st(w, id, 0x19, -1);
    } else {
        anim(w, id, ctx, 0xf, 0, 10);
        st(w, id, 0x1a, 3);
    }
    if w.mobys[id].pv_i(CHARGE) != 0 {
        anim(w, id, ctx, 0xf, 0, 10);
        st(w, id, 0x1c, 2);
    }
}

/// The chat (state 5): in role 3 its head turned toward its partner (±90°, not in sub 10); role 1 in its sequence
/// 0x1c now and then 0x1b (a third of those 0x1a); the sequences 0x1a / 0x1b over: 0x1d, and with a thug partner the
/// roles passed (it 1 → 2); 0x1d over: 0x1e (role 1: 0x1c; role 3: a new partner, the first role-1 thug of its group);
/// role 1 in 0x1e: 0x1d; role 3 with its partner in 0x1d: 0x1d.
fn chat(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
    if w.mobys[id].pv_i(CHAT) == 3 && w.mobys[id].sub_state != 10 {
        let mut a = sub_rot(yaw_to(w, id, w.mobys[o].pos), w.mobys[id].rot[2]);
        if w.mobys[id].once & 4 != 0 { a = -a; }
        w.mobys[id].set_pv_f(HEAD_TURN, a.clamp(-HALF_PI, HALF_PI));
    }
    if w.mobys[id].pv_i(CHAT) == 1 && seq_a(w, id, ctx) == 0x1c && ctx.game.rng.randi(200) == 1 {
        anim(w, id, ctx, 0x1b, 0, 30);
        if ctx.game.rng.randi(3) == 1 { anim(w, id, ctx, 0x1a, 0, 30); }
    }
    if (0x1a..=0x1b).contains(&seq_a(w, id, ctx)) && wrapped(w, id, ctx) {
        anim(w, id, ctx, 0x1d, 0, 30);
        if w.mobys[o].o_class == CLASS {
            w.mobys[o].set_pv_i(CHAT, 1);
            w.mobys[id].set_pv_i(CHAT, 2);
        }
    }
    if seq_a(w, id, ctx) == 0x1d && wrapped(w, id, ctx) {
        anim(w, id, ctx, 0x1e, 0, 30);
        if w.mobys[id].pv_i(CHAT) == 1 { anim(w, id, ctx, 0x1c, 0, 30); }
        if w.mobys[id].pv_i(CHAT) == 3 {
            if let Some(p) = chat_one(w, id) { ai::set_link(&mut w.mobys[id], PARTNER, Some(p)); }
        }
    }
    if w.mobys[id].pv_i(CHAT) == 1 && seq_a(w, id, ctx) == 0x1e { anim(w, id, ctx, 0x1d, 0, 20); }
    let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
    if w.mobys[id].pv_i(CHAT) == 3 && seq_a(w, o, ctx) == 0x1d { anim(w, id, ctx, 0x1d, 0, 35); }
}

/// `0x427268` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    for off in [T260, HEALTH + 6] {
        let mut t = m.pv_s(off) as i32;
        countdown(&mut t);
        m.set_pv_s(off, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks += 1; }
    crate::moby::joint_aim::aim(w, id, AIM, id, 4, 0.03, 0.3);
    crate::moby::joint_aim::head(w, ctx, id, AIM, 4, 3.5);
}


/// The group trigger.
pub const TRIGGER: i16 = 0x0b6d;
/// The classes it hides and wakes.
const SQUAD: [i16; 3] = [CLASS, COUSIN, HOVER];

/// The trigger's group `k` (its pvars `+0x04` / `+0x08`, 255 none): its members.
fn squad(w: &MobyWorld, id: usize, k: usize) -> Option<Vec<usize>> {
    let g = w.mobys[id].pv_i(4 * k);
    if g == 255 { return None; }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned()
}

/// A member woken: its sub-state 1 (alert), shown (mode without 0x41 / 2, with TARGET), its collision back.
fn wake(w: &mut MobyWorld, i: usize) {
    let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[i];
    m.sub_state = 1;
    m.mode = ((m.mode & 0xffbe) | crate::moby::mode::TARGET) & 0xfffd;
    m.has_collision = solid;
    w.changed = true;
}

/// The squad classes of group `k` woken; `alive` the stricter test (not dying, its health over 0). False when the
/// group is missing (the original leaves the update there).
fn wake_group(w: &mut MobyWorld, id: usize, k: usize, alive: bool) -> bool {
    let Some(list) = squad(w, id, k) else { return false };
    for i in list {
        let Some(m) = w.mobys.get(i) else { continue };
        let ok = if alive { !matches!(m.state, 0xfe | 0xfd) && 0.0 < m.pv_f(ai::health(m)) } else { m.state < 0x80 };
        if ok && SQUAD.contains(&m.o_class) { wake(w, i); }
    }
    true
}

/// 0x0b6d (`0x47bdd0`, 7 placed): an enemy squad's trigger, on its parent moby (`+0x00`, the moby index). Its pvars:
/// `+0x04` / `+0x08` its two groups (255 none), `+0x14` its looped sound's handle.
/// * **0**: 1, mode | 0x800, the parent hidden (mode | 0x41); the thugs, 0x0cf6 and hovers 0x0d07 of both groups
///   hidden (mode | 0x43 without TARGET, no collision).
/// * **1**: Ratchet on the parent (crate::moby::classes::level17::floaters::hero_on, `0x325398`), the parent hooked or
///   its mission done: 2 (its ticks 0), its sound 1 and the loop 2 (unless already playing), both groups' squads woken
///   (sub-state 1, shown, solid). Hit by a turret shot 0x0dd9 (flags 0x10000): its sound 0, both squads woken, a smoke
///   burst (1, 13), its pieces thrown 0.42 along its facing (`0x2b0a68`, crate::moby::shards::break_into, 3..6), gone;
///   any other hit let go.
/// * **2**: up 0.15 a tick; after 60 ticks the loop stopped, 3, no collision, hidden (mode | 0x41 without TARGET).
pub fn update_trigger(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let parent = index(w, id, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.mode |= 0x800;
        if let Some(p) = parent { w.mobys[p].mode |= 0x41; }
        for k in [1, 2] {
            let Some(list) = squad(w, id, k) else { return };
            for i in list {
                let Some(m) = w.mobys.get_mut(i) else { continue };
                if m.state < 0x80 && SQUAD.contains(&m.o_class) {
                    m.mode = (m.mode & 0xefff) | 0x43;
                    m.has_collision = false;
                }
            }
        }
        w.changed = true;
    }
    if w.mobys[id].state == 1 {
        let mission = w.mobys[id].mission as i32;
        let set_off = parent.is_some_and(|p| super::super::level17::floaters::hero_on(ctx, p) || w.mobys[p].once & 4 != 0) || super::mission_done(ctx, mission);
        if set_off {
            let m = &mut w.mobys[id];
            m.state_ticks = 0;
            m.state = 2;
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            let h = w.mobys[id].pv_i(0x14);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 2, 4);
                w.mobys[id].set_pv_i(0x14, h);
            }
            if !wake_group(w, id, 1, true) || !wake_group(w, id, 2, false) { return; }
        }
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
        if let Some(h) = hit {
            let shot = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == SHOT_CLASS);
            if shot {
                let v = set_len(w.mobys[id].rows[0], f32::from_bits(0x3ed5_5556));
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                let m = &mut w.mobys[id];
                m.state_ticks = 0;
                w.clear_hit(id);
                let woke = wake_group(w, id, 1, false) && wake_group(w, id, 2, false);
                if woke {
                    let pos = w.mobys[id].pos;
                    crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), pos, -1);
                    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xff, v, 0xff);
                } else {
                    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xff, v, 0xff);
                    let pos = w.mobys[id].pos;
                    crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), pos, -1);
                }
                w.delete(id);
                return;
            }
        }
        w.clear_hit(id);
    }
    if w.mobys[id].state == 2 {
        let m = &mut w.mobys[id];
        m.pos[2] += 0.15;
        m.state_ticks += 1;
        w.changed = true;
        if 60 < w.mobys[id].state_ticks {
            let h = w.mobys[id].pv_i(0x14);
            if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            let m = &mut w.mobys[id];
            m.set_pv_i(0x14, -1);
            m.state = 3;
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & 0xefff;
        }
    }
}
