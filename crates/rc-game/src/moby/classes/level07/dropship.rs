//! 0x0d44 (`0x3efa48`, 4 placed; the decomp's `func_L07_003EE708`): the dropship. It waits hidden until Ratchet
//! enters its cuboid, flies in to its target cuboid and drops its groups of prowlers (0x0d29) and gunners (0x0d2a)
//! one at a time, the next when the last is dead; with none left it climbs away. It is a platform (its block at
//! `+0x50`) and can be shot down.
//!
//! pvars: `+0x20` health (40 when `+0x24` is 0), `+0x50` the platform block, `+0x90` the cuboid Ratchet sets it off
//! in (−1: never), `+0x94` the cuboid it flies to, `+0x98` the one it starts from when its own place is in view (0:
//! its place), `+0x9c` up to seven groups (−1 ends), `+0xc0` its velocity, `+0xd0` the target, `+0xe0` its angles
//! (the roll eased by `+0x11c`, the yaw `+0xe8`), `+0xf0` its place before the bob, `+0x100..+0x108` the wobbles'
//! phases, `+0x110` the platform it rides (none here: the port finds no setter), `+0x114` / `+0x118` the climb's
//! speed and height, `+0x120` the group, `+0x124` the timer between groups.
//!
//! * **0**: health 40 if unset, two wobble phases random angles (`0x311d10`), `+0xf0` its place; 1, hidden without
//!   collision.
//! * **1**: Ratchet (`0x189ea0`) in cuboid `+0x90` (`0x317920`) → [`set_off`] (`0x3f0020`).
//! * **2**: [`fly`]; hooked (`0x320c98`) → 3.
//! * **3**: [`fly`]; within 3 of the target (xy): each prowler / gunner of the group not dying gets its class sound 0 and
//!   is dropped ([`drop_in`]) from `$gp+0x1c10` in its frame; 4.
//! * **4**: its first tick the timer 60; [`fly`]; the last group (or the seventh) → 5; else once the group is dead
//!   and the timer out, the next group and 3.
//! * **5**: [`fly`]; the climb's speed toward 1/6 by 1/1200 a tick, its height on by it; past 30 and off screen
//!   (`+0x31`), deleted.
//!
//! From state 2, riding nothing: a hit's damage (`0x3140c0(m, 0x10000)`, `0x314120`) taken toward 0, the hit let go;
//! at 0 its pieces (`0x31dd30`: mask 0x7f7, its block's move, 5..10 / 5..10, no sound), the big explosion facing away
//! from the camera (`0x3f0768`), its class sound 2, deleted.

use crate::hero::physics::{add, add_rot, dot, len, set_len, sub, V3};
use crate::moby::classes::level04::launcher::carry;
use crate::moby::classes::props::group_alive;
use crate::moby::{ai, countdown, platform, shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d44;

/// `$gp+0x1c10`: where it drops from, in its frame.
const DROP: V3 = [-2.5, 0.0, -2.0];
/// `$gp+0x1c20` / `+0x1c24`: the roll's and pitch's wobble, degrees; `+0x1c28` the bob.
const WOBBLE: [f32; 3] = [5.0, 1.0, 0.25];
/// `$gp+0x1c2c..+0x1c34`: the wobbles' phases, degrees a second.
const PHASE: [f32; 3] = [70.0, 120.0, 200.0];
/// `$gp+0x1c38`: the steering's change, a second² (/3600).
const ACCEL: f32 = 40.0;
/// `$gp+0x1c3c`: its top speed a second.
const TOP: f32 = 80.0;
/// `$gp+0x1c40`: the roll toward its sideways speed.
const BANK: f32 = -5.0;
/// `$gp+0x1c44` / `+0x1c48`: the roll's ease (degrees a second²) and top rate (a second).
const ROLL: (f32, f32) = (360.0, 720.0);

const HIDE: u16 = 0x41;
const RAD: f32 = std::f32::consts::PI / 180.0;

pub fn update_0d44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let r = (ctx.game.rng.rand_angle(), ctx.game.rng.rand_angle());
            let m = &mut w.mobys[id];
            if m.pv_s(0x24) == 0 {
                m.set_pv_s(0x24, 0x28);
                m.set_pv_f(0x20, 40.0);
            }
            m.set_pv_f(0x104, r.0);
            m.set_pv_f(0x108, r.1);
            let pos = m.pos;
            m.set_pv_v(0xf0, pos);
            m.set_state(1, -1);
            m.has_collision = false;
            m.mode = (m.mode | HIDE) & 0xefff;
        }
        1 => {
            let c = w.mobys[id].pv_i(0x90);
            if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) { set_off(w, id, ctx); }
        }
        2 => {
            fly(w, id);
            if w.mobys[id].once & 4 != 0 { w.mobys[id].set_state(3, -1); }
        }
        3 => {
            if fly(w, id) < 3.0 {
                let k = w.mobys[id].pv_i(0x120).clamp(0, 6) as usize;
                let group = w.mobys[id].pv_i(0x9c + 4 * k);
                if group != -1 {
                    let m = &w.mobys[id];
                    let r = m.rows;
                    let from = add(std::array::from_fn(|i| DROP[0] * r[0][i] + DROP[1] * r[1][i] + DROP[2] * r[2][i]), m.pos);
                    let support = ai::link(m, 0x110);
                    let members: Vec<usize> = w.groups.get(group as usize).map_or(Vec::new(), |g| g.iter().map(|&i| i as usize).collect());
                    for e in members {
                        let Some(o) = w.mobys.get(e) else { break };
                        if 0xfd <= o.state { continue; }
                        if o.o_class == super::prowler::CLASS || o.o_class == super::gunner::CLASS {
                            ctx.host.play_sound(&w.mobys[id], 0, 0);
                            drop_in(w, ctx, e, from, id, support);
                        }
                    }
                }
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => {
            if w.mobys[id].first() { w.mobys[id].set_pv_i(0x124, 0x3c); }
            fly(w, id);
            let k = w.mobys[id].pv_i(0x120);
            if 6 <= k || w.mobys[id].pv_i(0x9c + 4 * (k as usize + 1)) == -1 {
                w.mobys[id].set_state(5, -1);
            } else if group_alive(w, w.mobys[id].pv_i(0x9c + 4 * k as usize)) == 0 {
                let mut t = w.mobys[id].pv_i(0x124);
                let out = countdown(&mut t);
                let m = &mut w.mobys[id];
                m.set_pv_i(0x124, t);
                if out {
                    m.set_pv_i(0x120, k + 1);
                    m.set_state(3, -1);
                }
            }
        }
        5 => {
            fly(w, id);
            let m = &mut w.mobys[id];
            let mut v = m.pv_f(0x114);
            crate::hero::physics::approach(1.0 / 6.0, 1.0 / 1200.0, &mut v);
            m.set_pv_f(0x114, v);
            let h = m.pv_f(0x118) + v;
            m.set_pv_f(0x118, h);
            if 30.0 < h && m.drawn == 0 { return w.delete(id); }
        }
        _ => {}
    }
    hurt(w, id, ctx);
}

/// `0x3f0020`: shown with its class's collision, state 3, drawn and updated from any distance; from cuboid `+0x98`
/// when its own place is in view (`0x2e6140(its sphere, 128)`), facing and aiming at cuboid `+0x94`.
pub(super) fn set_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let start = w.mobys[id].pv_i(0x98);
    let m = &w.mobys[id];
    let sphere = [m.pos[0], m.pos[1], m.pos[2], m.bsphere[3]];
    let seen = start != 0 && ctx.view.is_some_and(|v| !v.culled(128.0, sphere));
    let at = |i: i32| usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i)).map(|c| [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]]);
    let (from, to) = (if seen { at(start) } else { None }, at(m.pv_i(0x94)).unwrap_or(m.pos));
    let m = &mut w.mobys[id];
    m.mode = (m.mode & !HIDE) | 0x1000;
    m.has_collision = true;
    m.set_state(3, -1);
    m.draw_dist = 0xff;
    m.update_dist = 0xff;
    if let Some(p) = from { m.pos = p; }
    m.set_pv_f(0xe8, crate::hero::physics::atan2(to[0] - m.pos[0], to[1] - m.pos[1]));
    let pos = m.pos;
    m.set_pv_v(0xf0, pos);
    m.set_pv_v(0xd0, to);
}

/// `0x3f0168`: one tick of flight toward the target (in the frame of what it rides): the velocity steered toward the
/// stopping speed √(d·a/2) (at most [`TOP`]) by at most `a`; the roll eased toward its sideways speed; the three
/// wobbles; its platform block written. Returns the xy distance left (`0x2e5110`).
fn fly(w: &mut MobyWorld, id: usize) -> f32 {
    let support = ai::link(&w.mobys[id], 0x110);
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let mut place = w.mobys[id].pv_v(0xf0);
    let target = match support.and_then(|s| w.mobys.get(s).map(|p| (s, p))) {
        Some((s, p)) => {
            if let Some(b) = platform::block(p) { place = add(place, p.pv_v(b + 0x10)); }
            let d = w.mobys[id].pv_v(0xd0);
            let r = p.rows;
            let t = add(std::array::from_fn(|i| d[0] * r[0][i] + d[1] * r[1][i] + d[2] * r[2][i]), p.pos);
            let yaw = w.mobys[s].rot[2];
            w.mobys[id].set_pv_f(0xe8, yaw);
            t
        }
        None => w.mobys[id].pv_v(0xd0),
    };
    let a = ACCEL / 3600.0;
    let diff = sub(target, place);
    let d = len(diff);
    let amount = (d * a * 0.5).sqrt().min(TOP / 60.0);
    let step = if amount < d { set_len(diff, amount) } else { diff };
    let m = &mut w.mobys[id];
    let mut vel = m.pv_v(0xc0);
    let mut fix = sub(step, vel);
    if a < len(fix) { fix = set_len(fix, a); }
    vel = add(vel, fix);
    place = add(place, vel);
    m.set_pv_v(0xc0, vel);
    m.set_pv_v(0xf0, place);
    let side = dot(m.rows[1], vel);
    let (mut roll, mut rv) = (m.pv_f(0xe0), m.pv_f(0x11c));
    let k = RAD / 3600.0;
    crate::hero::physics::ease(side * BANK, ROLL.0 * k, ROLL.0 * k, ROLL.1 * RAD / 60.0, &mut roll, &mut rv);
    m.set_pv_f(0xe0, roll);
    m.set_pv_f(0x11c, rv);
    m.pos = place;
    m.rot = [roll, m.pv_f(0xe4), m.pv_f(0xe8)];
    let ph: [f32; 3] = std::array::from_fn(|i| add_rot(m.pv_f(0x100 + 4 * i), PHASE[i] * RAD / 60.0));
    for (i, p) in ph.iter().enumerate() { m.set_pv_f(0x100 + 4 * i, *p); }
    m.rot[0] = add_rot(m.rot[0], WOBBLE[0] * (ph[0].sin() * RAD));
    m.rot[1] = add_rot(m.rot[1], WOBBLE[1] * (ph[1].sin() * RAD));
    m.pos[2] += WOBBLE[2] * ph[2].sin() + m.pv_f(0x118);
    let (new_pos, new_rot) = (m.pos, m.rot);
    if let Some(b) = platform::block(m) { platform::write(m, b, sub(new_pos, old_pos), old_rot, new_rot); }
    crate::hero::physics::dist2(place, target)
}

/// `0x3e86c0` / `0x3ecf28`: a prowler (state 10) or gunner (state 8) shown with its class's collision and carried
/// in (crate::moby::classes::level04::launcher::carry, `0x2e2c98`) from `from` to its place, lifted 0.4 of the drop
/// either end, over 45..75 ticks; the arc's start rides the dropship.
fn drop_in(w: &mut MobyWorld, ctx: &mut Ctx, e: usize, from: V3, ship: usize, support: Option<usize>) {
    let state = if w.mobys[e].o_class == super::prowler::CLASS { 10 } else { 8 };
    let has = w.class(w.mobys[e].o_class).is_some_and(|c| c.has_collision);
    let o = &mut w.mobys[e];
    o.set_state(state, -1);
    o.mode = (o.mode & !HIDE) | 0x1000;
    o.has_collision = has;
    let (to, rot) = (o.pos, o.rot);
    let ticks = ctx.game.rng.rand_range(0x2d, 0x4b);
    let up = (to[2] - from[2]) * 0.4;
    carry(w, ctx, e, from, [0.0, 0.0, up], to, [0.0, 0.0, -up], rot, ticks, Some(ship), support);
}

/// The tail: from state 2, riding nothing, the hit's damage; at no health the pieces, the explosion and gone.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if id >= w.mobys.len() || w.mobys[id].state < 2 || 0xfd <= w.mobys[id].state || ai::link(&w.mobys[id], 0x110).is_some() { return; }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    let mut health = w.mobys[id].pv_f(0x20);
    crate::hero::physics::approach(0.0, dmg, &mut health);
    w.mobys[id].set_pv_f(0x20, health);
    w.clear_hit(id);
    if health != 0.0 { return; }
    let m = &w.mobys[id];
    let mv = platform::block(m).map_or([0.0; 3], |b| m.pv_v(b + 0x10));
    shards::break_into(w, id, ctx, (5.0, 10.0), (5.0, 10.0), 0x7f7, mv, 0xff);
    let pos = w.mobys[id].pos;
    let dir = set_len(sub(pos, ctx.host.camera_pos()), 1.0);
    crate::moby::classes::explosion::start(w, ctx, 0.0, pos, dir);
    ctx.host.play_sound(&w.mobys[id], 2, 0);
    w.delete(id);
}
