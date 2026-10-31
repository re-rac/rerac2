//! Maktar's dropship, **0x0b76** (L02 `0x402d80`; its place `0x403340`, its jets `0x403480`): hidden until Ratchet
//! enters its cuboid, it flies its path and sets down the creature keepers 0x09c5 (crate::moby::classes::keeper)
//! waiting hidden in its group, one every two seconds, through the teleport-in carrier
//! (crate::moby::classes::teleport); with none left and none of them alive it climbs away. Shot down, it breaks.
//!
//! **pvars**: `+0x20` its health (40 when the data's maximum `+0x24` is 0, which it then becomes), `+0x100` its path,
//! `+0x104` the cuboid that wakes it, `+0x110` the keepers' group; `+0x11c` its climb a tick, `+0x120` its place on
//! the path, `+0x130` its angles there, `+0x140` / `+0x144` / `+0x148` its bob's and sways' phases, `+0x14c` its place
//! along the path (in points) and `+0x150` its speed (points a tick), `+0x154` / `+0x158` / `+0x15c` its accel, brake
//! and top speed (1/720, 1/360 and ½ unit a tick over its first segment's length, `+0x160` that length's inverse),
//! `+0x168` the drop timer, `+0x16c` its looped sound.
//!
//! **States**: 0 the set-up — its health, its path's start, hidden and not solid (1). 1 waiting for Ratchet in its
//! cuboid: then (a keeper waiting in its group, else it goes) its speed's first step ([`ease`], its place back to 0),
//! shown and solid (2). 2 flying its path ([`follow`]): its top speed scaled by `max(0.1, 1 − the turn)` between its
//! heading and the path's a second ahead; at its end (3). 3 over the drop point: every 120 ticks the first keeper of
//! its group still hidden (state 0x17, [`waiting_keeper`]) is set down on the ground under it ([`drop_keeper`], its
//! sound 0); none left and none alive: 4. 4 climbing, its climb easing to 1/6 a tick (1/720 a tick²); out of sight
//! it goes.
//!
//! **Each tick**: its scale 1.5 its class's; drawn (or flying, 2 / 4) it is placed ([`place`]); drawn while flying or
//! dropping (2..4) its jets smoke ([`jets`]); past 1, a hit (flags 0x10000) takes its damage
//! (crate::moby::ai::damage_of) off its health; at 0 its sound 2, its pieces thrown (crate::moby::shards::break_into,
//! mask 0x7f7, 5..10 out and up) and it goes.
//!
//! Not ported: its death's explosion `0x425300` (a moby 0x0d4e with its own update and particles, and its blasts
//! `0x425c68`; also used by `0x4029e8`, `0x42b978` and `0x3f8ff8`), the angles' w (`+0x13c`, the path points' w).

use crate::hero::physics::{add, add_rot, approach, atan2, ease, rot_dist, sub, V3};
use crate::moby::{ai, mode, Ctx, MobyWorld};
use super::flock::follow;

pub const CLASS: i16 = 0x0b76;
/// `$gp+0x2780`: its two jets in its frame.
const JETS: [V3; 2] = [[0.0, 3.15, -2.0], [0.0, -3.15, -2.0]];
/// `$gp+0x27a0` / `0x1b1794`: the jets' fall, units a second.
const JET_FALL: (f32, f32) = (5.0, 6.0);
/// The jets' puffs (crate::particles::trail: colours, base size): the first on odd frames (`$gp+0x27a8`, `0x1b179c`,
/// `$gp+0x27c0`) or even (`0x1b17a0`, `0x1b17a4`, `0x1b17b4`), then the second (`0x1b17a8`, `0x1b17ac`, `0x1b17b8`).
const PUFF_ODD: (u32, u32, i16) = (0xcf00_00ff, 0x0000_00cf, 0x6c);
const PUFF_EVEN: (u32, u32, i16) = (0x6000_ffff, 0x0000_0080, 0xd2);
const PUFF_CORE: (u32, u32, i16) = (0xefff_7f4f, 0x00ff_0000, 0x24);
/// The keeper's launcher link (crate::moby::classes::keeper, `+0x370`).
const KEEPER_GUN: usize = 0x370;

fn path<'a>(ctx: &'a Ctx, id: i32) -> Option<&'a Vec<[f32; 4]>> {
    usize::try_from(id).ok().and_then(|i| ctx.level.paths.get(i)).filter(|p| 2 <= p.len())
}

/// `0x3ceb08(group)` (the keeper's code): the first member of moby group `group` hidden for its drop (state 0x17).
pub fn waiting_keeper(w: &MobyWorld, group: i32) -> Option<usize> {
    let g = usize::try_from(group).ok().filter(|&g| g & 0xff != 0xff)?;
    w.groups.get(g)?.iter().copied().find(|&i| w.mobys.get(i).is_some_and(|m| m.state == 0x17))
}

/// `0x3cebc0(keeper, pos)` (the keeper's code): set down at `pos` — home-bound (state 0x16, sub 4), its sequence 0 at
/// once (`0x31f5e0`), its launcher (`+0x370`) at its joint 0; both shown, targets and solid as their classes; its drop
/// count `+0x7f` 0; the two brought in by the teleport-in carrier at `pos`, facing its way.
pub fn drop_keeper(w: &mut MobyWorld, k: usize, ctx: &mut Ctx, pos: V3) {
    let gun = ai::link(&w.mobys[k], KEEPER_GUN).filter(|&g| g < w.mobys.len());
    w.mobys[k].set_state(0x16, 4);
    ctx.host.set_anim(&w.mobys[k], 0, 0, 0);
    let joint = ctx.host.joint_matrix(&w.mobys[k], 0);
    for (i, at) in [(Some(k), None), (gun, joint)] {
        let Some(i) = i else { continue };
        let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[i];
        if let Some((rows, p)) = at { (m.rows, m.pos) = (rows, p); }
        m.mode = (m.mode & !0x41) | mode::TARGET;
        m.has_collision = solid;
    }
    w.mobys[k].drops = 0;
    let rot = w.mobys[k].rot;
    let linked: Vec<usize> = gun.into_iter().collect();
    super::teleport::start(w, k, pos, rot, &linked);
    w.changed = true;
}

/// 0x0b76 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = w.mobys[id].state;
    let old = w.mobys[id].pos;
    match state {
        0 => {
            let Some(p) = path(ctx, w.mobys[id].pv_i(0x100)).cloned() else {
                w.delete(id);
                return;
            };
            let m = &mut w.mobys[id];
            if m.pv_s(0x24) == 0 {
                m.set_pv_s(0x24, 0x28);
                m.set_pv_f(0x20, 40.0);
            }
            m.set_state(1, -1);
            m.set_pv_i(0x16c, -1);
            let d = crate::hero::physics::dist([p[0][0], p[0][1], p[0][2]], [p[1][0], p[1][1], p[1][2]]);
            let inv = 1.0 / d;
            m.set_pv_f(0x150, 0.0);
            m.set_pv_f(0x160, inv);
            m.set_pv_f(0x15c, inv * 0.5);
            m.set_pv_f(0x154, inv * 0.001_388_888_9);
            m.set_pv_f(0x158, inv * 0.002_777_777_8);
            m.set_pv_v(0x120, [p[0][0], p[0][1], p[0][2]]);
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 0x41;
        }
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(0x104), ctx.hero.pos) {
                if waiting_keeper(w, w.mobys[id].pv_i(0x110)).is_none() {
                    w.delete(id);
                    return;
                }
                let n = path(ctx, w.mobys[id].pv_i(0x100)).map_or(2, |p| p.len()) as f32;
                let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                let (mut t, mut v, top) = (m.pv_f(0x14c), m.pv_f(0x150), m.pv_f(0x15c));
                ease(n - 1.0, top, m.pv_f(0x158), top, &mut t, &mut v);
                m.set_pv_f(0x150, v);
                m.set_pv_f(0x14c, 0.0);
                m.set_state(2, -1);
                m.mode = (m.mode & !0x41) | mode::TARGET;
                m.has_collision = solid;
            }
        }
        2 => {
            if let Some(p) = path(ctx, w.mobys[id].pv_i(0x100)).cloned() {
                let n = p.len() as f32;
                let m = &mut w.mobys[id];
                let ahead = (m.pv_f(0x150) * 60.0 + m.pv_f(0x14c)).min(n - 2.0);
                let (i0, i1) = (ahead as usize, (ahead + 1.0) as usize);
                let way = atan2(p[i1][0] - p[i0][0], p[i1][1] - p[i0][1]);
                let k = (1.0 - rot_dist(way, m.pv_f(0x138))).max(0.1);
                let (mut t, mut v) = (m.pv_f(0x14c), m.pv_f(0x150));
                ease(n - 1.0, m.pv_f(0x154), m.pv_f(0x158), k * m.pv_f(0x15c), &mut t, &mut v);
                m.set_pv_f(0x14c, t);
                m.set_pv_f(0x150, v);
                let (mut at, mut rot) = (m.pv_v(0x120), m.pv_v(0x130));
                follow(&p, false, t, &mut at, &mut rot);
                m.set_pv_v(0x120, at);
                m.set_pv_v(0x130, rot);
                if n - 1.0 <= t { m.set_state(3, -1); }
            }
        }
        3 => drop_tick(w, id, ctx),
        4 => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_f(0x11c);
            approach(0.166_666_67, 0.001_388_888_9, &mut v);
            m.set_pv_f(0x11c, v);
            let mut at = m.pv_v(0x120);
            at[2] += v;
            m.set_pv_v(0x120, at);
            if m.drawn == 0 {
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    let scale = w.class(CLASS).map_or(1.0, |c| c.scale) * 1.5;
    w.mobys[id].scale = scale;
    let state = w.mobys[id].state;
    let drawn = w.mobys[id].drawn != 0;
    if drawn || state == 2 || state == 4 { place(w, id, ctx); }
    if drawn && (2..5).contains(&state) {
        let moved = sub(w.mobys[id].pos, old);
        jets(w, id, ctx, moved);
    }
    if 1 < state {
        // `0x322040(m, 0x10000, 0)`: the hit.
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
        let dmg = ai::damage_of(w, id, hit.as_ref(), false);
        let m = &mut w.mobys[id];
        let mut hp = m.pv_f(0x20);
        approach(0.0, dmg, &mut hp);
        m.set_pv_f(0x20, hp);
        w.clear_hit(id);
        if hp == 0.0 {
            ctx.host.play_sound(&w.mobys[id], 2, 0);
            crate::moby::shards::break_into(w, id, ctx, (5.0, 10.0), (5.0, 10.0), 0x7f7, [0.0; 3], 0xff);
            w.delete(id);
            return;
        }
    }
    w.rebuild(id, ctx.host);
    w.changed = true;
}

/// State 3: the drops (module docs).
fn drop_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let group = w.mobys[id].pv_i(0x110);
    if group == -1 { return; }
    let mut t = w.mobys[id].pv_i(0x168);
    let due = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(0x168, t);
    if !due { return; }
    let Some(k) = waiting_keeper(w, group) else {
        // `0x320a70(group, −1)`: its members alive.
        if super::props::group_alive(w, group) == 0 { w.mobys[id].set_state(4, -1); }
        return;
    };
    let mut at = w.mobys[id].pos;
    at[2] = ai::ground_z(ctx, 0.5, at, 0);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    drop_keeper(w, k, ctx, at);
    w.mobys[id].set_pv_i(0x168, 0x78);
}

/// `0x403340`: its looped sound 1 (`+0x16c`) kept playing; at its place on the path with its angles there, swaying
/// 0.05 by the sines of two phases (x ½° a tick, `+0x148`; y 0.8333° a tick, `+0x144`) and bobbing ¼ by a third
/// (7/6° a tick, `+0x140`).
fn place(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(0x16c);
    if !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
        w.mobys[id].set_pv_i(0x16c, h);
    }
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(0x120);
    m.rot = m.pv_v(0x130);
    let a = add_rot(m.pv_f(0x148), f32::from_bits(0x3c0e_fa37));
    m.set_pv_f(0x148, a);
    m.rot[0] += a.sin() * 0.05;
    let b = add_rot(m.pv_f(0x144), f32::from_bits(0x3c6e_4bb0));
    m.set_pv_f(0x144, b);
    m.rot[1] += b.sin() * 0.05;
    let c = add_rot(m.pv_f(0x140), f32::from_bits(0x3ca6_ce95));
    m.set_pv_f(0x140, c);
    m.pos[2] += c.sin() * 0.25;
}

/// `0x403480(m, moved)`: at each jet ([`JETS`] through its rows), falling `randf(5, 6)` a second plus its move this
/// tick, two additive trail puffs (crate::particles::trail, shrinking 10 a tick): one of [`PUFF_ODD`] (18..22 ticks)
/// or [`PUFF_EVEN`] (20..24) by the frame's parity (`0x1b30c8`), then one of [`PUFF_CORE`] (10..15).
fn jets(w: &MobyWorld, id: usize, ctx: &mut Ctx, moved: V3) {
    let m = &w.mobys[id];
    let (rows, pos) = (m.rows, m.pos);
    let odd = ctx.game.frame & 1 != 0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for j in JETS {
        let at = add(std::array::from_fn(|k| j[0] * rows[0][k] + j[1] * rows[1][k] + j[2] * rows[2][k]), pos);
        let fall = rng.randf(JET_FALL.0, JET_FALL.1) * 0.016_666_668;
        let vel = add([0.0, 0.0, -fall], moved);
        let (life, (c0, c1, base)) = if odd { (rng.rand_range(0x12, 0x16), PUFF_ODD) } else { (rng.rand_range(0x14, 0x18), PUFF_EVEN) };
        crate::particles::trail(sys, rng, at, vel, c0, c1, life, base, -10, true);
        let life = rng.rand_range(10, 0xf);
        let (c0, c1, base) = PUFF_CORE;
        crate::particles::trail(sys, rng, at, vel, c0, c1, life, base, -10, true);
    }
}
