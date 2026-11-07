//! 0x07e5, Damosel orbit's walkers (L23 `0x3bfe40`, the decomp's `func_US2_L23_003BFE40`; its frame `0x3c0df0` and
//! helpers `0x3c1278` … `0x3c32c8`, `func_L23_003BF9B0` … `func_L23_003C1E88`; 25 placed): two-legged walkers, each
//! carrying a gun turret 0x07e6 (its rider, `+0x2a0`, made at its set-up), that the opponent 0x0f7f
//! (crate::moby::classes::level23::boss) launches at Giant Clank from its joints. They walk the planetoid's paths
//! (crate::moby::classes::level23::paths, the block [`paths::DRONE`]), fire missiles 0x1237 at him from afar and one
//! of them at a time (`0x1b3630`, the opponent picks it) charges and fires its beam close up.
//!
//! **The frame** ([`frame`], `0x3c0df0`): **0**, the set-up: its words (speed 9.5, the sound −1, …), mode | 0x100,
//! its reaction block (`+0x140`: hurt state 5, the shares), its mover (`+0x50`: radius 2.5, height 0.75, steps 2.75,
//! slope 2, the turn left π/4, options | 0x81), its health the fight's (`0x1b3670`, the opponent's `+0x31c`), its
//! rider made, hidden ([`hide`]). After it, while the opponent's relaunch word is set (`$gp+0x2610`, set at its
//! set-up) and the intro over: fewer than 10 out, it comes back ([`respawn`]); else the word cleared. Out of 0xd: its
//! senses, its scale × 1.4 (× 0.7 in 2) and the rider's, the rider's distances and glow its own; a hit by a special
//! 0x1190 kills it; the damage (crate::moby::ai::take_damage); the down at it and its rows stood up on it.
//!
//! **Each tick** ([`update_07e5`]): with the opponent dying it kills itself (a hit of its health). Not fought, its
//! beams gone in states out of 9..0xb. Out of 0xd: its height, facing Giant Clank (45°), his distance `+0x27c`, the
//! walk's clear test to him within 50 (`+0x324`), its flat place `+0x300`. **The knock-back** (sub 3,
//! crate::moby::ai::flight): in its sequence 8 a hit ahead (5 at 4 along its way, 100, 0x90000); dead: the class
//! sound 5, a blast at the rider and, killed by Giant Clank, a pickup at his feet by the opponent's choice
//! (`0x41f918`: health when his is 5 short, else the gun or specials he lacks), then it is sent under the planetoid;
//! over: [`hide`] when dead, else 5. **The states**: **1** shown → 3 after 6 ticks; **2** launched: shown, no
//! collision, its sequences 9 / 1, on the ground; **3** falling (sequence 1) → 5 on the ground, or 4 with the ground
//! within 4 under it; **4** landing (sequence 2) → 5 at its end or past frame 12; **5** → 7 after 8 ticks (sequence
//! 3); **6** comes back ([`respawn`]); **7** the walk to him for 200 ticks (speed 9.5..14.25, 32 unseen): seen, faced
//! and within 45 → 8; **8** the stand-off: the rider's sequence 4 (its turn `0x1b3688`); lost or past 60 → 5; past
//! 20 to him, else still; the rider's missiles at his chest at its frames 28 and 14 ([`missile`]); **9** the charge
//! (the fought one): its sequences 6 / 8, speed 25.5, the class sound 4 held; past 25 ticks to him, within 8 → 0xa;
//! **0xa** the beam: lost or past 10 → 5; after 15 ticks the middle of the rider's joints 0 and 1 within 14 of his
//! middle: his hit (`0x1b3678`, flags 0x10007, its way the rider's (row 0 + ¼ row 2) · 15), a blast; → 0xb; **0xb**
//! after 20 ticks no longer fought → 5 (the level sound 29 at 15).
//!
//! **After the states**: its timers `+0x250` / `+0x254` and ticks; out of 0xd the rider at its joint 1 ([`rider`]:
//! in sub 2 free and turned toward him) and its spin.
//!
//! **The walk** ([`walk`], `0x3c1708`): toward its target (`+0x320`: Giant Clank, or the dodge point `+0x2d0`) by the
//! slots unless it is clear to it (the test cached for him); stepping round a walker 8 ahead of it (a random side 12
//! across, or a pause) or a wall ahead ([`dodge`], `0x3c2860`), for 22..67 ticks; at its speed `+0x2a8`/60, facing
//! the point, by the body's move; then put on the ground ([`ground`]).
//!
//! **The respawn** ([`respawn`], `0x3c2f98`): at the slots' point nearest a point 80 below Giant Clank and 50 off him
//! along one of eight ways of his frame; shown ([`show`]), 5, the sequence 3; the opponent's next launch and the count
//! out counted on.
//!
//! Not ported: the beams 0x11ac (draws), the rider's shadow and its own (`0x2a93b0`, `+0x6f`), the draw `0x3c32c8`,
//! the missiles' trails, Ratchet's voice 0x2b.

use super::{boss, paths, pickups, shots};
use crate::hero::physics::{add, dist, scale, set_len, sub, V3};
use crate::moby::frames::{self, upright};
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x07e5;
pub const RIDER: i16 = 0x07e6;

/// `0x1afa08..`: the walk's speeds (base, the charge's, unseen), its reach and the beam's.
const SPEED: f32 = 9.5;
const SPEED_CHARGE: f32 = 25.5;
const SPEED_UNSEEN: f32 = 32.0;
const SIGHT: f32 = 50.0;
const ENGAGE: f32 = 45.0;
const LOSE: f32 = 60.0;
const CLOSE: f32 = 20.0;
const BEAM_RANGE: f32 = 8.0;
const BEAM_ABORT: f32 = 10.0;
const BEAM_REACH: f32 = 14.0;
/// `0x1afa54`: the dodge's ticks (±half).
const DODGE: i32 = 45;
/// `0x1afa64`: the rider's turn back.
const TURN_BACK: i16 = 30;

fn rider_of(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], 0x2a0).filter(|&r| w.mobys.get(r).is_some_and(|m| m.o_class == RIDER && m.state < 0xfd)) }
fn seq(w: &MobyWorld, ctx: &Ctx, id: usize) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_b) }

/// `0x2b3890`: the first time since the last state change (its own flag).
fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

/// `0x3c1400(m, seq, 0, blend, which)`: the sequence on itself (`which` & 1) and its rider (& 2).
fn anim(w: &MobyWorld, ctx: &mut Ctx, id: usize, s: u8, blend: u8, which: u8) {
    if which & 1 != 0 { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
    if which & 2 != 0 {
        if let Some(r) = rider_of(w, id) { ctx.host.set_anim(&w.mobys[r], s, 0, blend); }
    }
}

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pv_i(0x3b4) != -1 {
        ctx.host.stop_sound(m.pv_i(0x3b4));
        m.set_pv_i(0x3b4, -1);
    }
}

/// `0x3c2da0`: no longer fought (`0x1b3630` cleared), its beams gone.
fn unselect(w: &mut MobyWorld) { w.level23.selected = None; }

/// `0x3c2e20(m, &d)`: it may fight Giant Clank — in 8 with none fought, drawn, seeing him and within 26° of his
/// facing from him (a dot of 0.8976); its distance.
pub fn may_fight(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<f32> {
    let m = &w.mobys[id];
    if m.state != 8 || w.level23.selected.is_some() || m.drawn == 0 || m.pv_s(0x324) == 0 { return None; }
    let rows = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows, |h| h.rows);
    let d = set_len(sub(m.pos, ctx.hero.pos), 1.0);
    (crate::hero::physics::dot(d, rows[0]) <= f32::from_bits(0x3f65_c8fb)).then(|| m.pv_f(0x27c))
}

/// `0x3c2ef0(m)`: it is the one fought, into 9 (sub 2).
pub fn select(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    w.mobys[id].set_state(9, 2);
    w.level23.selected = Some(id);
}

/// `0x3c1468` (module docs: hidden): its beams gone, its rider and itself hidden without collision, state 0xd.
fn hide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_s(0x178, 0);
    for k in [rider_of(w, id), Some(id)].into_iter().flatten() {
        let m = &mut w.mobys[k];
        m.mode = (m.mode | 0x41) & 0xefff;
        m.has_collision = false;
    }
    w.mobys[id].set_state(13, 4);
    w.mobys[id].set_pv_i(0x25c, 0);
    stop_sound(w, id, ctx);
}

/// `0x3c1510` (module docs: shown): itself and its rider shown with collision, in the target list; its walk 0; its
/// health back (`0x1b3670`), its waits; the sequence 1 on both.
fn show(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_s(0x178, 0);
    for k in [Some(id), rider_of(w, id)].into_iter().flatten() {
        let m = &mut w.mobys[k];
        m.mode = (m.mode & 0xffbe) | 0x1000;
        m.has_collision = true;
    }
    let hp = w.level23.drone_health;
    let m = &mut w.mobys[id];
    m.set_pv_s(paths::DRONE.mode, 0);
    m.set_pv_i(0x25c, 0);
    m.set_pv_s(0x26, 2);
    m.set_pv_s(0x24, hp as i16);
    m.set_pv_f(0x20, hp as f32);
    m.set_pv_s(0x2a4, 0);
    m.set_pv_f(0x19c, 150.0 / hp as f32);
    m.set_pv_f(0x198, 40.0 / hp as f32);
    anim(w, ctx, id, 1, 0, 3);
    stop_sound(w, id, ctx);
}

/// `0x3c1638(m, 1)` (module docs: on the ground): its sequence 5 unless in 5 / 6 or launched; a line from 2 below
/// its place to 0.1 above it puts it on what it meets.
fn ground(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = seq(w, ctx, id);
    if !(5..=6).contains(&s) && w.mobys[id].state != 2 { anim(w, ctx, id, 5, 6, 1); }
    let r = w.level23.radius;
    let p = w.mobys[id].pos;
    let (a, b) = (paths::surface(ctx, r, p, -2.0), paths::surface(ctx, r, p, 0.1));
    if let Some(h) = w.coll_line(ctx, a, b, 0x12, None) { w.mobys[id].pos = h.point; }
}

/// `0x3c2f98` (module docs).
fn respawn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let r = w.level23.radius;
    let flat = paths::surface(ctx, r, ctx.hero.pos, 0.0);
    let rows = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows, |h| h.rows);
    let below = scale(rows[2], -80.0);
    let half = |a: V3, b: V3| scale(add(a, b), 0.5);
    let off = match ctx.game.rng.randi(8) {
        0 => scale(rows[0], 50.0),
        1 => scale(rows[0], -50.0),
        2 => scale(rows[1], 50.0),
        3 => scale(rows[1], -50.0),
        4 => half(scale(rows[1], 50.0), scale(rows[0], 50.0)),
        5 => half(scale(rows[0], 50.0), scale(rows[1], -50.0)),
        6 => half(scale(rows[1], -50.0), scale(rows[0], -50.0)),
        _ => half(scale(rows[0], -50.0), scale(rows[1], 50.0)),
    };
    let at = add(add(below, off), flat);
    if let Some(p) = paths::nearest_point(w, at, true) { w.mobys[id].pos = p; }
    let p = paths::surface(ctx, r, w.mobys[id].pos, 0.0);
    w.mobys[id].pos = p;
    w.mobys[id].set_pv_v(0x290, p);
    show(w, id, ctx);
    w.mobys[id].set_state(5, 0);
    anim(w, ctx, id, 3, 20, 3);
    w.level23.next_drone = (w.level23.next_drone + 1) % 25;
    w.level23.drones_out += 1;
}

/// `0x3c0df0` (module docs).
fn frame(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_f(0x2a8, SPEED);
        m.set_pv_i(0x3b4, -1);
        for off in [0x3be, 0x26c, 0x26e, 0x2a6] { m.set_pv_s(off, 0); }
        m.mode |= 0x100;
        for (off, v) in [(0x1b7, 1u8), (0x150, 12), (0x151, 8), (0x152, 8), (0x156, 8), (0x157, 8), (0x1ac, 0xff), (0x1ad, 0xff), (0x1ae, 0xff), (0x1af, 0xff), (0x1b2, 0xff), (0x1b3, 0xff), (0x138, 5), (0x139, 6)] {
            m.set_pv_b(off, v);
        }
        let hp = w.level23.drone_health.max(1) as f32;
        let m = &mut w.mobys[id];
        for (off, v) in [(0x15c, f32::from_bits(0x3f15_5556)), (0x174, 4.0), (0x160, 0.066_666_67), (0x164, 0.041_666_668), (0x168, 0.083_333_336), (0x16c, 0.016_666_668), (0x170, 3.0), (0x17c, 0.4), (0x19c, 150.0 / hp), (0x198, 40.0 / hp), (0x84, 4.0), (0x70, 0.004_444_444_6), (0x74, 0.004_444_444_6), (0x78, 0.200_000_02), (0x50, 2.5), (0x54, 0.75), (0x60, 2.0), (0x58, 2.75), (0x5c, 2.75), (0x80, std::f32::consts::FRAC_PI_4)] {
            m.set_pv_f(off, v);
        }
        m.set_pv_i(0x188, 3);
        m.set_pv_f(0x20, hp);
        m.set_pv_s(0x24, hp as i16);
        m.set_pv_s(0x28, 3);
        m.set_pv_s(0x26, 60);
        m.set_pv_i(0x88, m.pv_i(0x88) | 0x81);
        (m.update_dist, m.draw_dist) = (0xff, 0xff);
        let rider = w.create(RIDER);
        let rm = &mut w.mobys[rider];
        rm.rows = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        rm.pvars.resize(rm.pvars.len().max(0x80), 0);
        ai::set_link(&mut w.mobys[id], 0x2a0, Some(rider));
        w.mobys[id].set_pv_i(0x258, 0);
        hide(w, id, ctx);
        let m = &mut w.mobys[id];
        m.set_pv_s(0x48, 12);
        m.set_pv_s(0x4a, 14);
    } else if w.level23.relaunch && w.level23.intro == 0 {
        if w.level23.drones_out < 10 { respawn(w, id, ctx); } else { w.level23.relaunch = false; }
    }
    if w.mobys[id].state == 13 { return; }
    ai::senses(&mut w.mobys[id], 0.004_444_444_6, 0.133_333_34, 0.001_745_329_5, 0.209_439_55);
    let k = if w.mobys[id].state == 2 { 0.7 } else { 1.4 };
    let sc = w.class(CLASS).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = sc * k;
    if let Some(r) = rider_of(w, id) {
        let rs = w.class(RIDER).map_or(1.0, |c| c.scale);
        let (ud, dd, light, glow) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.glow) };
        let rm = &mut w.mobys[r];
        (rm.update_dist, rm.draw_dist, rm.light, rm.glow, rm.drawn) = (ud, dd, light, glow, 1);
        rm.scale = rs * k;
    }
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x30000 != 0 && h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == super::special::CLASS)) {
        w.mobys[id].set_pv_f(0x20, 0.0);
    }
    if w.mobys[id].state == 2 { w.mobys[id].set_pv_s(0x178, 0); }
    ai::take_damage(w, id, ctx, false);
    let down = scale(ctx.hero.gravity.up_at(w.mobys[id].pos), -1.0);
    let m = &mut w.mobys[id];
    m.set_pv_v(0x280, down);
    m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
}

/// `0x3c12d0(m, speed, at, joint)` (module docs: a missile): from the rider's joint's side (its row 1, negated for
/// joint 1) toward Giant Clank's chest (his row 2 · 3) at `0x1b368c`/60, its damage `speed` (twice `0x1b3674`).
fn missile(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, damage: f32, joint: usize) {
    let Some(r) = rider_of(w, id) else { return };
    let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
    let at = add(ctx.hero.pos, scale(up, 3.0));
    let Some((jr, jt)) = ctx.host.joint_matrix(&w.mobys[r], joint) else { return };
    let side = if joint != 0 { -1.0 } else { 1.0 };
    let c = add(scale(jr[1], side), jt);
    let v = set_len(sub(at, c), w.level23.drone_missile_speed * 0.016_666_668);
    shots::drone_missile(w, ctx, id, v, c, damage);
    ctx.host.play_sound_of(&w.mobys[id], CLASS, 3, 0);
}

/// `0x3c2860` (module docs: the dodge): a line from its lift up its row 2 to (4, −1.5..1.5, lift) in its frame; a
/// hit puts the point `+0x2d0` 30 across the face's normal from it (on the surface) and its dodge on.
fn dodge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let h = paths::lift(w, id);
    let r = ctx.game.rng.randi(8) as f32 * f32::from_bits(0x3edb_6db7) - 1.5;
    let m = &w.mobys[id];
    let (pos, rows) = (m.pos, m.rows);
    let start = add(scale(rows[2], h), pos);
    let end = add(crate::gravity::to_world([4.0, r, h], &rows), pos);
    let Some(hit) = w.coll_line(ctx, start, end, 0x12, None) else { return false };
    let axis = set_len(add(hit.normal, [0.0; 3]), 1.0);
    let d = sub(start, hit.point);
    let off = set_len(sub(d, scale(axis, crate::hero::physics::dot(d, axis))), -30.0);
    let p = paths::surface(ctx, w.level23.radius, add(pos, off), 0.0);
    let t = ctx.game.rng.rand_range(-DODGE / 2, DODGE / 2) + DODGE;
    let m = &mut w.mobys[id];
    m.set_pv_v(0x2d0, p);
    m.set_pv_s(0x26c, 1);
    m.set_pv_s(0x26e, t as i16);
    true
}

/// `0x3c1708` (module docs).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let b = paths::DRONE;
    let r = w.level23.radius;
    let to_hero = w.mobys[id].pv_i(0x320) == 0;
    let target = if to_hero { ctx.hero.pos } else { w.mobys[id].pv_v(0x2d0) };
    let mut dest = target;
    let goal = paths::surface(ctx, r, target, 0.0);
    let arrived = if to_hero { w.mobys[id].pv_s(0x324) != 0 } else { paths::clear(w, ctx, id, w.mobys[id].pos, target, true) };
    let m = &mut w.mobys[id];
    if !arrived && m.pv_s(b.mode) == 0 && m.pv_s(0x26c) == 0 { m.set_pv_s(b.mode, 1); }
    if m.pv_s(0x26c) != 0 {
        let mut t = m.pv_s(0x26e) as i32;
        if t != 0 { countdown(&mut t); }
        m.set_pv_s(0x26e, t as i16);
        if t == 0 { m.set_pv_s(0x26c, 0); }
        if m.pv_s(0x26c) == -1 { return; }
        dest = m.pv_v(0x2d0);
    } else {
        let mode = m.pv_s(b.mode);
        let allow = match mode {
            2 => false,
            4 => !arrived,
            _ if to_hero && m.pv_s(0x324) != 0 => false,
            0 => !arrived,
            3 => false,
            _ => true,
        };
        if allow {
            if dodge(w, id, ctx) {
                let t = ctx.game.rng.rand_range(-DODGE / 2, DODGE / 2) + DODGE;
                let m = &mut w.mobys[id];
                dest = m.pv_v(0x2d0);
                let p = m.pos;
                m.set_pv_v(0x2e0, p);
                m.set_pv_s(0x26e, t as i16);
            }
        } else {
            let m = &w.mobys[id];
            let ahead = add(scale(m.rows[0], 8.0), m.pos);
            let hit = w.coll_line(ctx, m.pos, ahead, 0x11, Some(id));
            if hit.and_then(|h| h.moby).is_some_and(|o| w.mobys[o].o_class == CLASS) {
                let side = match ctx.game.rng.randi(3) {
                    0 => 1.0,
                    1 => -1.0,
                    _ => 0.0,
                };
                let m = &mut w.mobys[id];
                if side == 0.0 {
                    m.set_pv_s(0x26c, -1);
                    anim(w, ctx, id, 3, 4, 1);
                } else {
                    m.set_pv_s(0x26c, 1);
                    let d = scale(add(scale(m.rows[1], side), m.rows[0]), 12.0);
                    let p = paths::surface(ctx, r, add(m.pos, d), 0.0);
                    m.set_pv_v(0x2d0, p);
                }
                let t = ctx.game.rng.rand_range(-DODGE / 2, DODGE / 2) + DODGE;
                let m = &mut w.mobys[id];
                m.set_pv_s(b.mode, 0);
                m.set_pv_s(0x26e, t as i16);
            }
        }
    }
    {
        let flat = (paths::surface(ctx, r, ctx.hero.pos, 0.0), w.mobys[id].pv_v(0x300));
        match w.mobys[id].pv_s(b.mode) {
            1 => {
                paths::start_walk(w, ctx, id, b, goal, 0.0);
            }
            2 => {
                let way = w.mobys[id].pv_v(b.way);
                if dist(w.mobys[id].pos, way) < 2.0 { w.mobys[id].set_pv_s(b.mode, 3); } else { dest = way; }
            }
            3 => {
                if dist(w.mobys[id].pv_v(0x300), goal) < 20.0 && arrived {
                    w.mobys[id].set_pv_s(b.mode, 4);
                } else {
                    if dist(w.mobys[id].pos, w.mobys[id].pv_v(b.way)) < 2.0 { paths::next_point(w, ctx, id, b, goal, 0.0, flat); }
                    dest = w.mobys[id].pv_v(b.way);
                }
            }
            4 => w.mobys[id].set_pv_s(b.mode, 0),
            _ => {}
        }
    }
    let v = set_len(sub(dest, w.mobys[id].pos), w.mobys[id].pv_f(0x2a8) * 0.016_666_668);
    frames::face_point(w, id, dest);
    frames::move_by(w, id, ctx, v);
    ground(w, id, ctx);
}

/// `0x3c2b18` (module docs: the rider): at its joint 1 (free in sub 2 or while turning back, else placed there); in
/// sub 2 turned about its own z by a velocity eased toward Giant Clank's yaw in its frame (`0x2aaa00`, the mover's
/// turn words `+0x64` / `+0x68`·2 / `+0x6c`), then for 30 ticks back toward the walker's joint 1's x (`+0x64`·2 /
/// `+0x68`·4); stood up on the sphere.
fn rider(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(r) = rider_of(w, id) else { return };
    let sub2 = w.mobys[id].sub_state == 2;
    let back = w.mobys[id].pv_s(0x2a6);
    if sub2 || back != 0 {
        if let Some(p) = ctx.host.joint(&w.mobys[id], 1) { w.mobys[r].pos = p; }
    } else {
        super::super::enemies::place_part(w, id, r, 1, ctx);
        return;
    }
    let m = &w.mobys[id];
    let (a, b, c, down) = (m.pv_f(0x64), m.pv_f(0x68), m.pv_f(0x6c), m.pv_v(0x280));
    let to = if sub2 {
        if back == 0 { w.mobys[id].set_pv_f(0x2ac, 0.0); }
        Some((sub(ctx.hero.pos, w.mobys[r].pos), a, b + b))
    } else {
        if back == TURN_BACK { w.mobys[id].set_pv_f(0x2ac, 0.0); }
        ctx.host.joint_matrix(&w.mobys[id], 1).map(|(jr, _)| (jr[0], a + a, b * 4.0))
    };
    if let Some((v, step, brake)) = to {
        let yaw = {
            let l = crate::gravity::to_local(v, &w.mobys[r].rows);
            crate::hero::physics::atan2(l[0], l[1])
        };
        let mut x = 0.0;
        let mut vel = w.mobys[id].pv_f(0x2ac);
        boss::glide(&mut x, &mut vel, yaw, step, brake, c);
        w.mobys[id].set_pv_f(0x2ac, vel);
        let rm = &mut w.mobys[r];
        rm.rows = crate::moby::attach::turn(&crate::moby::attach::rows_of([0.0, 0.0, vel]), &rm.rows);
        rm.mode |= 0x106;
        rm.rows = turn_rows(&rm.rows, upright(&rm.rows, scale(down, -1.0)));
    }
    if sub2 {
        w.mobys[id].set_pv_s(0x2a6, TURN_BACK);
    } else {
        let mut t = back as i32;
        countdown(&mut t);
        w.mobys[id].set_pv_s(0x2a6, t as i16);
    }
}

/// `0x41f918`: the pickup the opponent gives for a walker Giant Clank kills — health when he is more than 5 short;
/// else (or when only half short) the gun or the specials he lacks; one of them at random, none (−1) when full.
fn reward(w: &MobyWorld, ctx: &mut Ctx) -> i32 {
    let Some(b) = w.level23.boss else { return -1 };
    let m = &w.mobys[b];
    let g = &ctx.hero.giant;
    let most = m.pv_s(0x2f4) as i32;
    let mut table = [0i32; 3];
    let mut n = 0;
    if g.hp < most - 5 {
        table[0] = 0;
        n = 1;
    }
    if n == 0 || most / 2 <= g.hp {
        if g.counts[2] < m.pv_s(0x2fa) as i32 {
            table[n] = 1;
            n += 1;
        }
        if g.counts[3] < m.pv_s(0x2fe) as i32 {
            table[n] = 2;
            n += 1;
        }
    }
    if n == 0 { return -1; }
    table[ctx.game.rng.randi(n as i32).min(2) as usize]
}

/// 0x07e5 (`0x3bfe40`, module docs).
pub fn update_07e5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x3c0 { w.mobys[id].pvars.resize(0x3c0, 0); }
    frame(w, id, ctx);
    if 200 < w.level23.intro { return; }
    if w.level23.boss.is_some_and(|b| w.mobys[b].state == 4) {
        let (p, hp) = (w.mobys[id].pos, w.level23.drone_health as f32);
        let t = crate::moby::hits::HitTemplate::new(hp, Some(id), 0x1_0000, [0.0, 0.0, 0.0, 1.0]);
        w.deliver_hit_any(id, &t, [p[0], p[1], p[2], 1.0]);
    }
    let selected = w.level23.selected == Some(id);
    w.mobys[id].set_pv_s(0x326, 0);
    if selected && !(9..=11).contains(&w.mobys[id].state) { unselect(w); }
    let mut facing = false;
    let hero = ctx.hero.pos;
    if w.mobys[id].state != 13 {
        let h = ctx.hero.gravity.height(w.mobys[id].pos);
        let m = &w.mobys[id];
        facing = super::super::level22::planet::turn_to(&m.rows, sub(hero, m.pos)) <= std::f32::consts::FRAC_PI_4;
        let d = dist(hero, m.pos);
        let clear = d < SIGHT && paths::clear(w, ctx, id, w.mobys[id].pos, hero, true);
        let flat = paths::surface(ctx, w.level23.radius, w.mobys[id].pos, 0.0);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x270, h);
        m.set_pv_f(0x27c, d);
        m.set_pv_s(0x324, clear as i16);
        m.set_pv_v(0x300, flat);
    }
    if w.mobys[id].sub_state == 3 {
        let f = ai::flight(w, id, ctx);
        stop_sound(w, id, ctx);
        let s = seq(w, ctx, id);
        let way = w.mobys[id].pv_v(0x1c0);
        if s == 8 && 0.0 < crate::hero::physics::len(way) {
            let r = w.level23.radius;
            if w.mobys[id].pv_f(0x270) < r {
                let p = paths::surface(ctx, r, w.mobys[id].pos, 0.0);
                w.mobys[id].pos = p;
            }
            let m = &w.mobys[id];
            let at = add(add(crate::gravity::to_world([0.0, 0.0, m.pv_f(0x174)], &m.rows), m.pos), set_len(way, 4.0));
            ai::attack_sphere(w, id, ctx, 5.0, 100.0, 0.0, at, 0x9_0000, 1, 0, 0);
        }
        if w.mobys[id].pv_s(0x178) & 1 != 0 && w.mobys[id].pv_s(0x2a4) == 0 {
            ctx.host.play_sound_of(&w.mobys[id], CLASS, 5, 0);
            if let Some(r) = rider_of(w, id) {
                let p = w.mobys[r].pos;
                crate::moby::blast::burst(w, ctx, p, &crate::moby::blast::Blast { size: 2.5, bursts: [35, 20, 20], puffs: 20, streaks: 45, light: true, shake: true });
            }
            w.mobys[id].set_pv_s(0x2a4, 1);
            let killer = usize::try_from(w.mobys[id].pv_s(0x140 + 0x70) as i32).ok();
            if killer.is_some() && killer == ctx.hero_moby {
                let k = reward(w, ctx);
                if k != -1 {
                    let p = ctx.hero.pos;
                    pickups::drop_at(w, ctx, p, k as usize);
                }
            }
            let r = w.level23.radius;
            w.mobys[id].pos = paths::surface(ctx, r, ctx.hero.pos, -r * 2.0);
        }
        if f & 1 != 0 {
            hide(w, id, ctx);
        } else if w.mobys[id].first() && s == 8 {
            ctx.host.play_level_sound_after(None, 31, 0);
            anim(w, ctx, id, 8, 4, 2);
            w.mobys[id].set_pv_s(0x26, 25);
            w.mobys[id].set_pv_s(0x2a6, 0);
            if let Some(r) = rider_of(w, id) {
                let p = w.mobys[r].pos;
                crate::moby::blast::burst(w, ctx, p, &crate::moby::blast::Blast { size: 1.0, bursts: [10, 15, 15], puffs: 15, streaks: 15, light: false, shake: false });
            }
        } else if f & 2 != 0 {
            w.mobys[id].set_state(5, 0);
        }
    }
    let st = w.mobys[id].state;
    let t = w.mobys[id].state_ticks as u16 as i32;
    let rider_seq = rider_of(w, id).map(|r| seq(w, ctx, r));
    match st {
        5 => {
            if first_sub(w, id) {
                anim(w, ctx, id, 3, 4, 3);
                stop_sound(w, id, ctx);
            }
            if 8 < t { w.mobys[id].set_state(7, 1); }
        }
        1 => {
            if first_sub(w, id) { show(w, id, ctx); } else if 6 <= t { w.mobys[id].set_state(3, 0); }
        }
        2 => {
            w.mobys[id].has_collision = false;
            if first_sub(w, id) {
                show(w, id, ctx);
                anim(w, ctx, id, 9, 0, 1);
                anim(w, ctx, id, 1, 0, 2);
                ground(w, id, ctx);
                let m = &mut w.mobys[id];
                m.set_pv_s(0x3bc, 1);
                m.set_pv_f(0x3b8, 32.0);
            }
        }
        6 => respawn(w, id, ctx),
        3 => {
            if first_sub(w, id) && seq(w, ctx, id) != 1 { anim(w, ctx, id, 1, 4, 3); }
            let p = w.mobys[id].pos;
            frames::move_to(w, id, ctx, p);
            if w.mobys[id].pv_s(0x110) != 0 {
                w.mobys[id].set_state(5, 0);
                anim(w, ctx, id, 3, 10, 3);
            } else {
                let m = &w.mobys[id];
                let below = add(scale(m.rows[2], -3.0), m.pos);
                let above = add(scale(m.rows[2], 0.2), m.pos);
                if w.coll_line(ctx, above, below, 0x12, None).is_some_and(|h| dist(h.point, m.pos) < 4.0) { w.mobys[id].set_state(4, 0); }
            }
        }
        4 => {
            if first_sub(w, id) && seq(w, ctx, id) != 2 {
                anim(w, ctx, id, 2, 6, 3);
            } else if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0) || 12.0 < ctx.host.frame_time(&w.mobys[id]) {
                w.mobys[id].set_state(5, 0);
                anim(w, ctx, id, 3, 20, 3);
            }
            let p = w.mobys[id].pos;
            frames::move_to(w, id, ctx, p);
        }
        7 => {
            if first_sub(w, id) {
                let s = SPEED + ctx.game.rng.randf(0.0, SPEED * 0.5);
                let m = &mut w.mobys[id];
                m.set_pv_i(0x250, 200);
                m.set_pv_f(0x2a8, s);
                m.set_pv_i(0x320, 0);
            }
            if w.mobys[id].drawn == 0 {
                w.mobys[id].set_pv_f(0x2a8, SPEED_UNSEEN);
            } else if w.mobys[id].pv_f(0x2a8) == SPEED_UNSEEN {
                let s = SPEED + ctx.game.rng.randf(0.0, SPEED * 0.5);
                w.mobys[id].set_pv_f(0x2a8, s);
            }
            let m = &w.mobys[id];
            if m.pv_i(0x250) == 0 {
                w.mobys[id].set_state(5, 0);
            } else if m.pv_s(0x324) != 0 && facing && m.pv_f(0x27c) < ENGAGE {
                w.mobys[id].set_state(8, 2);
            } else {
                walk(w, id, ctx);
            }
        }
        8 => {
            if w.mobys[id].first() {
                let s = SPEED + ctx.game.rng.randf(0.0, SPEED * 0.5);
                w.mobys[id].set_pv_f(0x2a8, s);
            }
            if rider_seq != Some(4) {
                anim(w, ctx, id, 4, 10, 2);
                if let Some(r) = rider_of(w, id) { ctx.host.set_anim_speed(&w.mobys[r], w.level23.rider_turn); }
            }
            let (clear, d, s) = (w.mobys[id].pv_s(0x324) != 0, w.mobys[id].pv_f(0x27c), seq(w, ctx, id));
            if !clear || LOSE < d {
                w.mobys[id].set_state(5, 0);
                w.mobys[id].set_pv_i(0x250, 0);
            } else if CLOSE < d || ((5..=6).contains(&s) && CLOSE - 2.0 < d) {
                w.mobys[id].set_pv_i(0x320, 0);
                walk(w, id, ctx);
            } else {
                anim(w, ctx, id, 3, 10, 1);
                let p = w.mobys[id].pos;
                frames::move_to(w, id, ctx, p);
            }
            if let Some(r) = rider_of(w, id) {
                let dmg = w.level23.drone_missile * 2.0;
                if ai::reached(w, r, ctx, 28.0) { missile(w, id, ctx, dmg, 0); } else if ai::reached(w, r, ctx, 14.0) { missile(w, id, ctx, dmg, 1); }
            }
        }
        9 => {
            if w.mobys[id].first() {
                anim(w, ctx, id, 6, 12, 1);
                anim(w, ctx, id, 8, 12, 2);
                w.mobys[id].set_pv_f(0x2a8, SPEED_CHARGE);
                let h = ctx.host.play_sound_of(&w.mobys[id], CLASS, 4, 4);
                w.mobys[id].set_pv_i(0x3b4, h);
            }
            if w.mobys[id].pv_s(0x324) == 0 || LOSE < w.mobys[id].pv_f(0x27c) {
                w.mobys[id].set_state(5, 0);
                w.mobys[id].set_pv_i(0x250, 0);
            } else {
                rider_joints(w, id, ctx);
                if 25 < t {
                    w.mobys[id].set_pv_i(0x320, 0);
                    if w.mobys[id].pv_f(0x27c) < BEAM_RANGE { w.mobys[id].set_state(10, 2); }
                    walk(w, id, ctx);
                }
            }
        }
        10 => {
            if w.mobys[id].pv_s(0x324) == 0 || BEAM_ABORT < w.mobys[id].pv_f(0x27c) {
                w.mobys[id].set_state(5, 0);
            } else {
                rider_joints(w, id, ctx);
                if !w.mobys[id].first() && 15 < t {
                    let m = &w.mobys[id];
                    let mid = scale(add(m.pv_v(0x330), m.pv_v(0x340)), 0.5);
                    if dist(mid, w.level23.mid) < BEAM_REACH {
                        if let Some(r) = rider_of(w, id) {
                            let rr = w.mobys[r].rows;
                            let p = scale(add(scale(rr[2], 0.25), rr[0]), 15.0);
                            let at = w.mobys[id].pos;
                            let dmg = w.level23.drone_beam;
                            boss::hit_hero(w, ctx, Some(id), 0x1_0007, dmg, at, Some([p[0], p[1], p[2], f32::from_bits(0x45af_df66)]));
                        }
                        let mid = w.level23.mid;
                        crate::moby::blast::burst(w, ctx, mid, &crate::moby::blast::Blast { size: 1.2, bursts: [10, 10, 10], puffs: 10, streaks: 45, light: false, shake: true });
                        w.mobys[id].set_pv_s(0x3be, 1);
                    }
                    w.mobys[id].set_state(11, 2);
                }
            }
        }
        11 => {
            if w.mobys[id].first() {
                stop_sound(w, id, ctx);
            } else if 20 < t {
                unselect(w);
                w.mobys[id].set_state(5, 0);
            } else if 15 <= t && w.mobys[id].pv_s(0x3be) != 0 {
                ctx.host.play_level_sound_after(None, 29, 0);
                w.mobys[id].set_pv_s(0x3be, 0);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    for off in [0x250, 0x254] {
        let mut k = m.pv_i(off);
        countdown(&mut k);
        m.set_pv_i(off, k);
    }
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    if w.mobys[id].state != 13 {
        rider(w, id, ctx);
        spin(w, id);
    }
}

/// The rider's joints 1 and 0 (`+0x330` / `+0x340`), the beam's ends.
fn rider_joints(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(r) = rider_of(w, id) else { return };
    let (a, b) = (ctx.host.joint(&w.mobys[r], 1), ctx.host.joint(&w.mobys[r], 0));
    let m = &mut w.mobys[id];
    if let Some(a) = a { m.set_pv_v(0x330, a); }
    if let Some(b) = b { m.set_pv_v(0x340, b); }
}

/// `0x3c2f28` (module docs: the spin): the rider's joint 2 turned 320′ a tick about its z.
fn spin(w: &mut MobyWorld, id: usize) {
    let Some(r) = rider_of(w, id) else { return };
    let a = w.mobys[id].pv_f(0x3b0);
    let rm = &mut w.mobys[r];
    rm.manips.retain(|x| x.0 != 2);
    rm.manips.push((2, crate::quat::axis_angle(a, [0.0, 0.0, 1.0])));
    w.mobys[id].set_pv_f(0x3b0, crate::hero::physics::add_rot(a, 320.0 * 0.000_290_888_25));
}
