//! Yeedil's fighter **0x12f3** (`0x39e320`, 8 placed; the decomp's `func_L20_0039CF58`): a trooper on the player's
//! side in the boss 0x11e0's fight (crate::moby::classes::level20::boss). Its set-up makes it class 0x0d0f (the Dobbo
//! trooper's model, crate::moby::classes::trooper, its update its own), with that trooper's turret 0x0d0e
//! (`+0x254`) and two guns 0x0d66 (`+0x258` / `+0x25c`) made as it comes in. It waits in state 6 until the boss
//! brings it in ([`bring_in`]), then walks near its target and fires level 12's shots
//! (crate::moby::classes::blaster_shot) from its guns. On the framework (crate::moby::ai): health `+0x20` (10 by
//! default; its status `+0x2e`, its shot's damage byte `+0x34`), movement `+0x50` (radius 0.825, height 0.2, steps
//! 0.4 / 0.4, slope 90°, the turn left 30°, stop 0.1), reaction `+0x140` (a hurt to 5, sub 9/4/7 …: its bytes as set
//! below), target `+0x1f0` (`+0x200` the moby; the search's area `+0x230`).
//!
//! **Its pvars:** `+0x260` its goal, `+0x270..+0x27c` its parts' break thresholds (0x40..0xfe each), `+0x27c` among
//! the two of its group nearest Ratchet (then it takes the area's occupant as its target), `+0x280` its target (a
//! link), `+0x284` its yaw last tick, `+0x288` / `+0x28c` the turret's lag behind its turns and its speed.
//!
//! **Each tick** ([`update_12f3`]): its turret's collision off for the tick; the head ([`head`], `0x39e708`): the
//! set-up in state 0, a hit from a 0x0d0f does nothing, the frame's damage (`0x33d250` = L00 `0x327e38`,
//! crate::moby::ai::take_damage), its status 2 puts it away ([`stow`]); the target: `+0x280` (half a unit up), else
//! the search (128, its area). In a scene nothing more. In sub 1 the hurt's flight (`0x33df98` = L00 `0x328b80`,
//! ai::flight): dead, a blast half a unit up (size 0.5), its bolts and put away; knocked out of it, 3 (sub 0) with
//! sequence 0. **States**: **6** in the pool (nothing more). **2** arriving: at the sequence's end 3. **3** on its first
//! tick its target ([`retarget`], `0x39f520`), its goal ([`goal`], `0x39f5b0`) and sequence 11; within the stop
//! distance of the goal it turns to its target (3°) → 4, else walks to the goal. **4** sequence 10, the turret's aim
//! at the target's point; at ticks 0 and 10 of every 20 a shot from gun 1 / gun 0's joint 0 a quarter of the way to
//! the target's point a tick (its class sound 1 on the turret); after 120 ticks or its target hidden, 3. Then the
//! tail ([`tail`], `0x39eb78`): its parts placed ([`parts`]); without health its parts gone with a blast a unit up;
//! on its frame slice (one in 8) whether it is among the two of its group nearest Ratchet; the turret's lag eased
//! (0.556° a tick², 33.3° a tick); its ticks. Its turret's collision back.
//!
//! The engine functions with the port's: `0x330820` (set_state), `0x330540` / `0x330468` (set_anim, cut), `0x33faa8`
//! (first), `0x342e38` = L00 `0x32dcf0` (ai::senses), `0x342dd0` (L01 `0x323560`, the facing eased), `0x341b68` (L01
//! `0x322450`, ai::move_step), `0x340018` = L00 `0x32ad38` (ai::find_target), `0x327768` = L00 `0x3127c8` (a joint),
//! `0x339aa0` = L00 `0x3253f0` (crate::moby::classes::enemies::place_part: gun 1 mirrored, rows 1 and 2 turned),
//! `0x33fa28` = L00 `0x32a748` (a part's distances, owner, place and light), `0x339450` = L00 `0x3241d8` (its colour
//! on its parts), `0x41b550` = L00 `0x3f8348` (crate::moby::blast::burst), `0x32fd10` = L00 `0x31adc0`
//! (crate::moby::bolts::drop), `0x3c0048` = L12 `0x3a7fb8` (shot::fire), `0x445df0` (boss::pick_for).
//!
//! Not ported: in a scene the deletion of it and its parts once the loaded part (`0x1bdc70`) is 5 or more; its
//! parts' break pieces by its health (`0x39ec48`: the turret's pieces 0..9 but 4, 5 and 7 under their thresholds,
//! `0x2ef7b0`, with its sound 7); the arrival's effect (`0x3007d0`); the moby's draw as class 0x0d0f (its placed
//! instance keeps drawing its own model in the port).

use super::boss;
use crate::hero::physics::{add_rot, atan2, dist2, len, rot_dist, scale, set_len2, sub, sub_rot, turn_ease, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::enemies::place_part;
use crate::moby::classes::blaster_shot as shot;
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, bolts, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12f3;
/// What its set-up makes it, and its parts.
pub const AS: i16 = 0x0d0f;
pub const TURRET: i16 = 0x0d0e;
pub const GUN: i16 = 0x0d66;

const HP: usize = 0x20;
const MAX: usize = 0x24;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const T_POINT: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const AREA: usize = 0x230;
const TURRET_L: usize = 0x254;
const GUNS: [usize; 2] = [0x258, 0x25c];
const GOAL: usize = 0x260;
const THRESHOLDS: usize = 0x270;
const FEW: usize = 0x27c;
const TARGET: usize = 0x280;
const LAST_YAW: usize = 0x284;
const LAG: usize = 0x288;
const LAG_V: usize = 0x28c;

/// `$gp−0x348..−0x33c`: its limits (/60, /3600, °/s, °/s²).
const LIMITS: (f32, f32, f32, f32) = (6.0, 30.0, 720.0, 2000.0);
const DEG3: f32 = f32::from_bits(0x3d56_7750);
/// The blast of its end (`0x3f8348(0.5, 20, at, 0, 25, 20, 20, 20, 45, 0, 1, …)`).
const END: Blast = Blast { size: 0.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false };

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&k| k < w.mobys.len()) }
fn class_coll(w: &MobyWorld, k: usize) -> bool { w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision) }

/// `0x39ef18(m, seq, blend)`: the sequence on it unless it plays, and on its turret always.
fn seq_both(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
    if let Some(t) = link(w, id, TURRET_L) { ctx.host.set_anim(&w.mobys[t], s, 0, blend); }
}

/// `0x39f250`: put away — state 6 (sub 2), hidden, no collision; its parts gone.
pub fn stow(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_state(6, 2);
    m.mode = (m.mode | 0x41) & 0xefff;
    m.has_collision = false;
    for off in [TURRET_L, GUNS[0], GUNS[1]] {
        if let Some(k) = link(w, id, off) { w.delete(k); }
        ai::set_link(&mut w.mobys[id], off, None);
    }
    w.changed = true;
}

/// `0x39f2e0(m, make)`: with `make` its turret (`+0x7c` 0x3ff) and guns made when missing (false when one can not
/// be); then the turret at its joint 0 turned by the lag `+0x288`, rebuilt, its colour, its sequence followed (at its
/// frame, blend 10); the guns on the turret's joints 0 / 1 (gun 1 mirrored), its colour.
fn parts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, make: bool) -> bool {
    if make {
        if link(w, id, TURRET_L).is_none() {
            if w.class(TURRET).is_none() { return false; }
            let t = w.create(TURRET);
            w.adopt(t, Some(id), &*ctx.host);
            w.mobys[t].bangles = 0x3ff;
            ai::set_link(&mut w.mobys[id], TURRET_L, Some(t));
        }
        for off in GUNS {
            if link(w, id, off).is_some() { continue; }
            if w.class(GUN).is_none() { return false; }
            let g = w.create(GUN);
            w.adopt(g, Some(id), &*ctx.host);
            ai::set_link(&mut w.mobys[id], off, Some(g));
        }
    }
    let amb = w.mobys[id].ambient;
    let Some(t) = link(w, id, TURRET_L) else { return true };
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let yaw = add_rot(w.mobys[id].rot[2], w.mobys[id].pv_f(LAG));
    let tm = &mut w.mobys[t];
    tm.pos = at;
    tm.rot[2] = yaw;
    w.rebuild(t, &*ctx.host);
    w.mobys[t].ambient = amb;
    let a = ctx.host.anim(&w.mobys[id]);
    if let Some(a) = a {
        if seq(w, t, ctx) != a.seq_b { ctx.host.set_anim(&w.mobys[t], a.seq_b, a.frame_b, 10); }
    }
    for (k, off) in GUNS.into_iter().enumerate() {
        let Some(g) = link(w, id, off) else { continue };
        place_part(w, t, g, k, ctx);
        let gm = &mut w.mobys[g];
        if k == 1 { gm.rows[1] = scale(gm.rows[1], -1.0); gm.rows[2] = scale(gm.rows[2], -1.0); }
        gm.ambient = amb;
    }
    true
}

/// `0x39ef90(boss, g, pos)` with `0x39f0c0`: the first of group `g` in the pool brought in at `pos` facing Ratchet —
/// the boss its owner; shown with its collision, 2 (sub 0) with sequence 2 cut in, no drops, playing at 1; its
/// parts' thresholds drawn, not among the nearest; `+0x7c` 0xf; its parts made (none: put away again); the
/// reaction's options cleared, full health, the turret's sequence 2 cut in.
pub fn bring_in(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx, g: i32, pos: V3) -> Option<usize> {
    let id = boss::members(w, g).into_iter().find(|&k| w.mobys[k].state == 6)?;
    w.mobys[id].parent = Some(boss);
    let h = ctx.hero.pos;
    let rot = [0.0, 0.0, atan2(h[0] - pos[0], h[1] - pos[1])];
    let c = class_coll(w, id);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = c;
    m.set_state(2, 0);
    ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
    w.mobys[id].drops = 0;
    for k in 0..0xc {
        let b = ctx.game.rng.rand_range(0x40, 0xfe) as u8;
        w.mobys[id].set_pv_b(THRESHOLDS + k, b);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(FEW, 0);
    m.pos = pos;
    m.rot = rot;
    m.bangles = 0xf;
    w.rebuild(id, &*ctx.host);
    if !parts(w, id, ctx, true) {
        stow(w, id);
        return None;
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(REACT + 0x38, 0);
    m.set_pv_f(HP, m.pv_s(MAX) as f32);
    if let Some(t) = link(w, id, TURRET_L) { ctx.host.set_anim(&w.mobys[t], 2, 0, 0); }
    w.changed = true;
    Some(id)
}

/// `0x39e708`'s set-up (module docs): its reaction's bytes and rates, its movement block, `+0x7c` 0xf, its limits,
/// health 10 when none, class 0x0d0f, state 6 (sub 2), sequence 0, mode 0x4000 and not 4.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for (o, b) in [(0x151, 0x12u8), (0x156, 0x14), (0x1ac, 5), (0x1ad, 9), (0x1ae, 4), (0x1af, 7), (0x1b2, 10), (0x1b3, 0x11), (0x150, 5), (0x152, 0x11), (0x1b6, 0)] {
        m.set_pv_b(o, b);
    }
    m.set_pv_i(0x188, 1);
    for (o, f) in [
        (0x174, 0x3f00_0000u32), (0x17c, 0x3e19_999a), (0x15c, 0x3eaa_aaab), (0x160, 0x3dee_eef0), (0x164, 0x3b88_8889), (0x16c, 0x3c08_8889),
        (0x54, 0x3e4c_cccd), (0x50, 0x3f53_3333), (0x5c, 0x3ecc_cccd), (0x60, 0x3fc9_0fdc), (0x80, 0x3f06_0a93), (0x84, 0x3dcc_cccd),
        (0x170, 0x3f00_0000), (0x168, 0x3c08_8889), (0x58, 0x3ecc_cccd),
    ] {
        m.set_pv_f(o, f32::from_bits(f));
    }
    m.bangles = 0xf;
    ai::senses(m, LIMITS.1 * 0.000_277_777_8, LIMITS.0 * 0.016_666_668, LIMITS.3 * 4.848_137_5e-6, LIMITS.2 * 0.000_290_888_25);
    if m.pv_s(MAX) == 0 {
        m.set_pv_s(MAX, 10);
        m.set_pv_f(HP, 10.0);
    }
    // `0x331cd8(m, 0x0d0f)`: its class, slot, scale, collision and glow made the trooper's.
    if let Some(c) = w.class(AS).cloned() {
        let m = &mut w.mobys[id];
        m.o_class = AS;
        m.slot = c.slot;
        m.scale = c.scale;
        m.has_collision = c.has_collision;
        if c.glow != 0 {
            m.mode |= mode::GLOW;
            m.glow = c.glow;
        }
    }
    let m = &mut w.mobys[id];
    m.set_state(6, 2);
    if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xfffb) | 0x4000;
}

/// `0x39e708` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { setup(w, id, ctx); }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    if hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == AS) { w.clear_hit(id); }
    ai::take_damage(w, id, ctx, false);
    if w.mobys[id].pv_b(STATUS) == 2 { stow(w, id); }
    let m = &mut w.mobys[id];
    m.set_pv_b(STATUS, 1);
    match link(w, id, TARGET) {
        Some(t) => {
            let p = w.mobys[t].pos;
            let m = &mut w.mobys[id];
            ai::set_link(m, T_MOBY, Some(t));
            m.set_pv_v(T_POINT, [p[0], p[1], p[2] + 0.5]);
        }
        None => {
            let area = w.mobys[id].pv_i(AREA);
            ai::find_target(w, id, ctx, 128.0, 0.0, area, None, false);
        }
    }
}

/// `0x39f520`: among the nearest two, no target of its own and the area's occupant searched for; else the boss's
/// pick ([`boss::pick_for`]) or the boss itself, its place the target's point.
fn retarget(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(FEW) != 0 {
        ai::set_link(&mut w.mobys[id], TARGET, None);
        let area = w.mobys[id].pv_i(AREA);
        ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
        return;
    }
    let parent = w.mobys[id].parent.filter(|&p| p < w.mobys.len());
    let t = parent.and_then(|p| boss::pick_for(w, p, id)).or(parent);
    let p = t.map_or([0.0; 3], |t| w.mobys[t].pos);
    let m = &mut w.mobys[id];
    ai::set_link(m, TARGET, t);
    ai::set_link(m, T_MOBY, t);
    m.set_pv_v(T_POINT, p);
}

/// `0x39f5b0`: its goal 8..12 away — with Ratchet its target, about him at the camera's yaw ±45°; else on the line
/// from its target to it, at most that far across; the goal's distance across.
fn goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let d = ctx.game.rng.randf(8.0, 12.0);
    let t = link(w, id, T_MOBY);
    let g = if t.is_some() && t == ctx.hero_moby {
        let rows = ctx.host.camera_rows();
        let a = add_rot(ctx.game.rng.randf(-45.0, 45.0) * 0.017_453_294, rows[0][1].atan2(rows[0][0]));
        let h = ctx.hero.pos;
        [a.cos() * d + h[0], a.sin() * d + h[1], h[2]]
    } else {
        let tp = t.map_or([0.0; 3], |t| w.mobys[t].pos);
        let mut v = sub(w.mobys[id].pos, tp);
        // `0x3032f8`: across only, its height kept.
        if d < len(v) { v = set_len2(v, d); }
        [v[0] + tp[0], v[1] + tp[1], v[2] + tp[2]]
    };
    let m = &mut w.mobys[id];
    m.set_pv_v(GOAL, g);
    m.set_pv_f(MV + 0xcc, dist2(m.pos, g));
}

/// `0x39eb78` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    parts(w, id, ctx, false);
    // `0x39ec48`'s end: without health its parts gone with a blast a unit up.
    if w.mobys[id].pv_f(HP) <= 0.0 {
        for off in GUNS {
            if let Some(g) = link(w, id, off) { w.delete(g); }
            ai::set_link(&mut w.mobys[id], off, None);
        }
        if let Some(t) = link(w, id, TURRET_L) {
            w.delete(t);
            ai::set_link(&mut w.mobys[id], TURRET_L, None);
            let p = w.mobys[id].pos;
            blast::burst(w, ctx, [p[0], p[1], p[2] + 1.0], &END);
        }
    }
    if ctx.game.frame & 7 == (id & 7) as i32 {
        // `0x39ea48`: fewer than two others of its group (out of the pool) nearer Ratchet.
        let (pos, h) = (w.mobys[id].pos, ctx.hero.pos);
        let limit = dist2(pos, h);
        let n = boss::members(w, w.mobys[id].group as i32).into_iter().filter(|&k| k != id && w.mobys[k].state != 6 && dist2(w.mobys[k].pos, h) < limit).count();
        w.mobys[id].set_pv_i(FEW, (n < 2) as i32);
    }
    let m = &mut w.mobys[id];
    let t = sub_rot(m.pv_f(LAST_YAW), m.rot[2]);
    let (mut a, mut v) = (m.pv_f(LAG), m.pv_f(LAG_V));
    turn_ease(t, f32::from_bits(0x3c1e_dd20), f32::from_bits(0x3c1e_dd20), f32::from_bits(0x3f14_ef4e), &mut a, &mut v, 0);
    m.set_pv_f(LAG, a);
    m.set_pv_f(LAG_V, v);
    m.set_pv_f(LAST_YAW, m.rot[2]);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    w.changed = true;
}

/// 0x12f3 (module docs).
pub fn update_12f3(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(t) = link(w, id, TURRET_L) { w.mobys[t].has_collision = false; }
    head(w, id, ctx);
    if ctx.game.scenes.mode == crate::scene::SCENE { return; }
    if w.mobys[id].sub_state == 1 {
        let f = ai::flight(w, id, ctx);
        if f & 1 != 0 {
            let p = w.mobys[id].pos;
            blast::burst(w, ctx, [p[0], p[1], p[2] + 0.5], &END);
            bolts::drop(w, id, 0, 3, ctx);
            stow(w, id);
        } else if f & 2 != 0 {
            w.mobys[id].set_state(3, 0);
            seq_both(w, id, ctx, 0, 20);
        }
    }
    let ticks = w.mobys[id].state_ticks as u16;
    match w.mobys[id].state {
        6 => return,
        2 => if wrapped(w, id, ctx) { w.mobys[id].set_state(3, -1) },
        3 => {
            if w.mobys[id].first() {
                retarget(w, id, ctx);
                goal(w, id, ctx);
                seq_both(w, id, ctx, 11, 20);
            }
            let m = &w.mobys[id];
            if m.pv_f(MV + 0xcc) <= m.pv_f(MV + 0x34) {
                let tp = link(w, id, T_MOBY).map_or(ctx.hero.pos, |t| w.mobys[t].pos);
                let p = w.mobys[id].pos;
                let yaw = atan2(tp[0] - p[0], tp[1] - p[1]);
                crate::moby::ai::face_at(&mut w.mobys[id], MV, yaw);
                if rot_dist(w.mobys[id].rot[2], yaw) < DEG3 { w.mobys[id].set_state(4, -1); }
            } else {
                let (p, g) = (w.mobys[id].pos, w.mobys[id].pv_v(GOAL));
                ai::move_step(w, id, ctx, atan2(g[0] - p[0], g[1] - p[1]), g);
            }
        }
        4 => {
            if w.mobys[id].first() { seq_both(w, id, ctx, 10, 20); }
            let (p, tp) = (w.mobys[id].pos, w.mobys[id].pv_v(T_POINT));
            w.mobys[id].set_pv_f(LAST_YAW, atan2(tp[0] - p[0], tp[1] - p[1]));
            let t = ticks % 20;
            if t == 0 || t == 10 {
                let gun = link(w, id, if t == 0 { GUNS[1] } else { GUNS[0] });
                let at = gun.and_then(|g| ctx.host.joint(&w.mobys[g], 0)).unwrap_or(p);
                let vel = scale(sub(tp, at), 0.25);
                if let Some(tm) = link(w, id, TURRET_L) { ctx.host.play_sound(&w.mobys[tm], 1, 0); }
                let dmg = w.mobys[id].pv_b(DAMAGE) as f32;
                shot::fire(w, ctx, dmg, id, vel, at);
            }
            if 120 < ticks || link(w, id, TARGET).is_some_and(|t| w.mobys[t].mode & 1 != 0) { w.mobys[id].set_state(3, -1); }
        }
        _ => {}
    }
    tail(w, id, ctx);
    if let Some(t) = link(w, id, TURRET_L) { w.mobys[t].has_collision = class_coll(w, t); }
}
