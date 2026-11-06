//! 0x0878 / 0x08ad / 0x08d8 (`0x3be7d0`, 30, 27 and 4 placed; the decomp's `func_L17_003BD310`): a chain of hazards riding a
//! path (`+0x00`). Each link names the next one (`+0x04`, a moby index), which links back to it (`+0x1c`). The
//! link with no next one is the leader: it drives the chain round the path, and the links trail it.
//!
//! * Every tick its tuning is set by class:
//!
//!   | Class | Follow gap `+0x20` | Spacing `+0x24` | Lead `+0x28` | Speed `+0x10` (a second) |
//!   |---|---|---|---|---|
//!   | 0x08ad | 4 | 3.5 | 10 | 20 |
//!   | Others | 0 | 18 | 20 | 10 |
//!
//!   (`$gp+0xc90..+0xcac`). Its glow word flashes white 15 ticks in 60, by its uid (else grey). 0x08ad leading
//!   within 80 of Ratchet keeps its loop sound 0 (mode 4, `+0x2c`) playing.
//! * **0**: with no path of 2 points it goes. Otherwise it is drawn to 0x180 and updated from any distance. Its
//!   distance along the path starts at 4 per link behind it (`+0x0c`). A link with a next one hooks it back and is a
//!   follower; then 1.
//! * **1**: the chain is laid out along the path's start ([`lay_out`], `0x3bcc30`); the leader goes to 2, the others
//!   to 3.
//! * **2** (the leader): the chain runs ([`run`], `0x3bcec0`, while `$gp+0xcb0` is set); a hit with 0x4010000
//!   (`0x3140c0(m, …, 1)`) moves it to 5.
//! * **3** (a follower): a hit moves it to 5, and so does its next one being gone (through 4: 5 ticks later).
//! * **5**: it goes up. 0x08ad: a blast (size 10, sprites 10 each, 10 puffs, 20 streaks, its flash ball), pieces 0, 1
//!   and 3 (`0x31dd30`: 3..6 / 3..6), and its piece 2 left tumbling (spin −10..10, gravity 20..30). The others: a
//!   blast of size 3 and pieces 0..4. Then the level's bolts, and it goes.
//!
//! Not ported: the path's length written into the path's record (`0x2ef018` → path `+0x1c`, unused here).

use crate::hero::physics::{add, add_rot, atan2, dist2, euler_rows, len, set_len, sub, turn_ease, V3};
use crate::moby::{blast, bolts, countdown, shards, Ctx, MobyWorld};

pub const CHAIN: i16 = 0x0878;
pub const CHAIN_B: i16 = 0x08ad;
pub const LOUD: i16 = 0x08d8;

const RUNNING: bool = true;
const QUARTER: f32 = std::f32::consts::FRAC_PI_2;

fn tune(o_class: i16) -> (f32, f32, f32, f32) {
    if o_class == CHAIN_B { (4.0, 3.5, 10.0, 20.0) } else { (0.0, 18.0, 20.0, 10.0) }
}

fn link(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| w.mobys.get(i).is_some_and(|m| m.state < 0xfd)) }

/// The point `t` along path `p` (`0x343e88` = boot `0x2ca618`): segments of the path's first point's w, the point
/// that far along its segment's way.
fn along(path: &[[f32; 4]], t: f32) -> V3 {
    let p = |k: usize| [path[k][0], path[k][1], path[k][2]];
    let step = path[0][3];
    let n = path.len();
    let i = if step != 0.0 { (t / step) as i32 } else { 0 };
    if i < 0 { return p(0); }
    let i = i as usize;
    if n - 1 <= i { return p(n - 1); }
    let frac = t - i as f32 * step;
    add(p(i), set_len(sub(p(i + 1), p(i)), frac))
}

pub fn update_0878(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (gap, spacing, lead, speed) = tune(w.mobys[id].o_class);
    let frame = ctx.game.frame;
    let m = &mut w.mobys[id];
    m.set_pv_f(0x20, gap);
    m.set_pv_f(0x24, spacing);
    m.set_pv_f(0x28, lead);
    m.set_pv_f(0x10, speed);
    m.glow = if (frame + m.uid as i32).rem_euclid(60) < 15 { 0x80ff_ffff } else { 0x8040_4040 };
    if m.o_class == LOUD || m.o_class == CHAIN_B {
        let h = m.pv_i(0x2c);
        if m.state == 2 && dist2(m.pos, ctx.hero.pos) < 80.0 {
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x2c, h);
            }
        } else {
            if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            w.mobys[id].set_pv_i(0x2c, -1);
        }
    }
    match w.mobys[id].state {
        0 => {
            let path_ok = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|p| ctx.level.paths.get(p)).is_some_and(|p| 2 <= p.len());
            if !path_ok { return w.delete(id); }
            let mut n = 0;
            let mut k = w.mobys[id].pv_i(4);
            while let Some(o) = link(w, k) {
                n += 1;
                k = w.mobys[o].pv_i(4);
                if 64 < n { break; }
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(0x2c, -1);
            m.set_pv_i(0x34, 0);
            m.draw_dist = 0x180;
            m.update_dist = 0xff;
            m.set_pv_f(8, n as f32);
            m.set_pv_f(0xc, n as f32 * 4.0);
            m.set_pv_i(0x14, 0);
            m.set_pv_i(0x18, 0);
            if let Some(next) = link(w, w.mobys[id].pv_i(4)) {
                crate::moby::ai::set_link(&mut w.mobys[next], 0x1c, Some(id));
                w.mobys[id].set_pv_i(0x34, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            lay_out(w, id, ctx);
            let leader = RUNNING && w.mobys[id].pv_i(0x34) == 0;
            w.mobys[id].set_state(if leader { 2 } else { 3 }, -1);
        }
        2 => {
            if RUNNING { run(w, id, ctx); }
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) { w.mobys[id].set_state(5, -1); }
            w.clear_hit(id);
        }
        3 => {
            if link(w, w.mobys[id].pv_i(4)).is_none() && w.mobys[id].pv_i(4) != -1 { w.mobys[id].set_state(4, -1); }
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) { w.mobys[id].set_state(5, -1); }
            w.clear_hit(id);
        }
        4 => {
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_i(0x30, 5); }
            let mut t = m.pv_i(0x30);
            let out = countdown(&mut t);
            m.set_pv_i(0x30, t);
            if out { m.set_state(5, -1); }
        }
        5 => {
            let pos = w.mobys[id].pos;
            if w.mobys[id].o_class == CHAIN_B {
                blast::burst(w, ctx, pos, &blast::Blast { size: 10.0, bursts: [10, 10, 10], puffs: 10, streaks: 20, light: true, shake: false });
                shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 11, [0.0; 3], 0xff);
                let g = &mut ctx.game.rng;
                let (spin, gravity) = (g.randf(-10.0, 10.0), g.randf(20.0, 30.0));
                shards::spawn_piece(w, id, ctx, spin, gravity, 2, [0.0; 3], 0);
            } else {
                blast::burst(w, ctx, pos, &blast::Blast { size: 3.0, bursts: [10, 10, 10], puffs: 10, streaks: 20, light: true, shake: false });
                shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 31, [0.0; 3], 0xff);
            }
            bolts::level_drop(w, id, 0.0, 1, ctx);
            w.delete(id);
        }
        _ => {}
    }
}

/// The links behind `m`, nearest first (by `+0x1c`).
fn trailing(w: &MobyWorld, id: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut k = crate::moby::ai::link(&w.mobys[id], 0x1c);
    while let Some(o) = k.filter(|&o| o < w.mobys.len() && w.mobys[o].state < 0xfd) {
        if out.contains(&o) || 64 < out.len() { break; }
        out.push(o);
        k = crate::moby::ai::link(&w.mobys[o], 0x1c);
    }
    out
}

fn flash(w: &mut MobyWorld, o: usize, frame: i32) {
    let m = &mut w.mobys[o];
    m.glow = if (frame + m.uid as i32).rem_euclid(60) < 15 { 0x80ff_ffff } else { 0x8040_4040 };
}

/// `0x3bcc30`: the leader at the path's start facing its second point; each link behind it in a line, a gap of
/// `+0x20` past the hitch, its spacing back.
fn lay_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(path) = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|p| ctx.level.paths.get(p)) else { return };
    let (p0, p1) = ([path[0][0], path[0][1], path[0][2]], [path[1][0], path[1][1], path[1][2]]);
    let a = atan2(p1[0] - p0[0], p1[1] - p0[1]);
    let dir = [a.cos(), a.sin(), 0.0];
    let (gap, half) = (w.mobys[id].pv_f(0x20), w.mobys[id].pv_f(0x24));
    let m = &mut w.mobys[id];
    m.pos = p0;
    m.rot = [0.0, 0.0, add_rot(a, QUARTER)];
    m.set_pv_f(0x18, 0.0);
    m.set_pv_f(0x14, 0.0);
    let mut rear = add(p0, crate::hero::physics::scale(dir, -half));
    for o in trailing(w, id) {
        flash(w, o, ctx.game.frame);
        let hitch = add(rear, crate::hero::physics::scale(dir, -gap));
        let lh = w.mobys[o].pv_f(0x24);
        let mut p = add(hitch, crate::hero::physics::scale(dir, -lh));
        p[2] = p0[2];
        let q = &mut w.mobys[o];
        q.pos = p;
        q.rot = [q.rot[0], 0.0, add_rot(atan2(hitch[0] - p[0], hitch[1] - p[1]), QUARTER)];
        q.rows = euler_rows(q.rot);
        q.set_pv_i(0x34, 1);
        q.mode |= 4;
        rear = add(q.pos, q.rows[1].map(|x| x * half));
    }
    w.changed = true;
}

/// `0x3bcec0`: the leader on along the path by its speed (wrapping), facing the point its lead ahead (heading and
/// pitch eased); each link drawn to the hitch behind the one ahead.
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(path) = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|p| ctx.level.paths.get(p)) else { return };
    let length = path.len() as f32 * path[0][3];
    let m = &mut w.mobys[id];
    let mut t = m.pv_f(0xc) + m.pv_f(0x10) / 60.0;
    if length < t { t = length - t; }
    m.set_pv_f(0xc, t);
    m.pos = along(path, t);
    let mut ahead = t + m.pv_f(0x28);
    if length < ahead { ahead -= length; }
    let target = along(path, ahead);
    let (gap, spacing) = (m.pv_f(0x20), m.pv_f(0x24));
    steer(m, target, (0.000_436_332_4, 0.000_872_664_7, 0.052_359_887));
    m.rows = euler_rows(m.rot);
    let mut anchor = add(m.pos, m.rows[1].map(|x| x * spacing));
    for o in trailing(w, id) {
        flash(w, o, ctx.game.frame);
        let q = &mut w.mobys[o];
        let front = add(q.pos, q.rows[1].map(|x| x * -spacing));
        let d = sub(front, anchor);
        let mut want = if gap == 0.0 || len(d) == 0.0 { anchor } else { add(set_len(d, gap), anchor) };
        want[2] = anchor[2];
        let delta = sub(want, q.pos);
        let l = len(delta);
        if 0.0 < l { q.pos = add(q.pos, set_len(delta, l - spacing)); }
        steer(q, want, (0.000_872_664_7, 0.001_745_329_5, 0.104_719_77));
        q.rows = euler_rows(q.rot);
        q.set_pv_i(0x34, 1);
        q.mode |= 4;
        anchor = add(q.pos, q.rows[1].map(|x| x * spacing));
    }
    w.changed = true;
}

/// The heading (a quarter turn past the way to `to`) and pitch eased toward `to` (`0x31f230`).
fn steer(m: &mut crate::moby::Moby, to: V3, (a, b, top): (f32, f32, f32)) {
    let yaw = add_rot(atan2(to[0] - m.pos[0], to[1] - m.pos[1]), QUARTER);
    let (mut h, mut hv) = (m.rot[2], m.pv_f(0x18));
    turn_ease(yaw, a, b, top, &mut h, &mut hv, 0);
    let pitch = atan2(dist2(m.pos, to), to[2] - m.pos[2]);
    let (mut p, mut pv) = (m.rot[1], m.pv_f(0x14));
    turn_ease(pitch, a, b, top, &mut p, &mut pv, 0);
    m.rot[2] = h;
    m.rot[1] = p;
    m.set_pv_f(0x18, hv);
    m.set_pv_f(0x14, pv);
}
