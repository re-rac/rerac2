//! Maktar Nebula's small objects (L02 class updates; no names in any reference, so they go by o_class).
//!
//! * 0x0afc (`0x3f0618`): no update (an empty function); 0x0873 creates one in its place.
//! * 0x0aaf (`0x3eb9f8`): on its first tick sequence state 1, no collision, mode 0x43 without TARGET ([`update_0aaf`]).
//! * 0x0873 (`0x3c1d20`): on its first tick a 0x0afc in its place ([`update_0873`]).
//! * 0x0a24 (`0x3d90f0`) / 0x0a55 (`0x3dd430`): on their first tick at their path's first point ([`update_0a24`],
//!   [`update_0a55`]).
//! * 0x0a4b (`0x3dd3c8`): its shadow's slab probed while drawn within 64 of the camera, range 0x3c ([`update_0a4b`]).
//! * 0x0aab / 0x0aac (`0x3eb6f8`): sliding doors, set off by a hook ([`update_0aab`]).
//! * 0x0d0d (`0x424cf8`): a breakable target: shot, it breaks into its pieces, debris chunks and smoke ([`update_0d0d`]).
//! * 0x0adf (`0x3ecbb8`): a platform swinging along its cuboid, carrying Ratchet ([`update_0adf`]).
//! * 0x0e93 (`0x42fb20`): a breakable lamp: two glows, shot it breaks into its pieces, a ball, bolts, debris chunks and
//!   smoke ([`update_0e93`]). Not ported: the count of them `0x12cf` keeps (`$gp−0x3c0`; at 10 left, once: `0x367f58`,
//!   `0x32c310(0x1233)`).
//! * The lock puzzle: the power nodes 0x0c82 ([`update_0c82`]) a bomb 0x09c8 kills, the lock 0x0c81
//!   ([`update_0c81`]) that, with its four nodes dead, hooks its group, and the conveyor bridge 0x0ab0
//!   ([`update_0ab0`]) that unfolds when hooked and carries Ratchet on its belt surfaces 0x0aaf.
//!
//! Not ported (the lock puzzle): the bomb 0x09c8 (`0x3d1020`: dragged by the Tractor Beam, its fuse, its blast,
//! home), the catapult 0x0aa6 (`0x3e9d48`) that flings it, the walls 0x0c1b (`0x446a48`) it breaks, the nodes' and
//! the lock's draws (`0x414118`, `0x413460`).
//!
//! Not ported: 0x0adf's lift (`0x3ece68`, its update when the cuboid's z scale is 2: up and down to Ratchet's level
//! as he rides it); 0x0e96 (`0x4301b0`, the arena crowd: it waits for its controller past state 1, cheers by the
//! controller's pvars `+0x6c` and waves about (500, 600); the controller is not ported); 0x0e9c (`0x4305f8`: its tie
//! group's instances turned π/400 a tick and a texture's scroll `+0x74`; the engine draws the ties static); 0x0ccc
//! (`0x417bf0`, moves a moby another class links in); 0x000b (`0x396448`, the ship's pad: its hologram 0x477 and the
//! △ prompt that starts the travel, the main dev's progression), 0x12cf (`0x38b458`, that count), 0x0c24
//! (`0x40b4c0`, the help voice lines).

use crate::moby::{carry_hero, mode, Ctx, MobyWorld};
use crate::shadow_volumes::probe_near;

/// 0x0afc.
pub fn update_0afc(_w: &mut MobyWorld, _id: usize, _ctx: &mut Ctx) {}

/// `0x3eb9f8`: pvars `+0x5c` |= 4, state 1, the collision blob `+0x98` cleared, mode 0x43 without TARGET.
pub fn update_0aaf(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state != 0 { return; }
    m.set_pv_i(0x5c, m.pv_i(0x5c) | 4);
    m.set_state(1, -1);
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x43;
    w.changed = true;
}

/// `0x3c1d20`: pvars `+4` / `+8` 0, `+0x10` the yaw, `+0x18` = `+0x14`, update distance 0x80, state 1; a 0x0afc
/// (`0x315438`) at its position with its update and draw distances, mode |= 0x41, kept in `+0x1c` (index + 1).
pub fn update_0873(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let m = &mut w.mobys[id];
    m.set_pv_i(4, 0);
    m.update_dist = 0x80;
    m.state = 1;
    m.set_pv_i(8, 0);
    m.set_pv_f(0x10, m.rot[2]);
    m.set_pv_i(0x18, m.pv_i(0x14));
    let (pos, update_dist, draw_dist) = (m.pos, m.update_dist, m.draw_dist);
    let c = w.create(0x0afc);
    let child = &mut w.mobys[c];
    child.update_dist = update_dist;
    child.draw_dist = draw_dist;
    child.pos = pos;
    child.mode |= 0x41;
    w.rebuild(c, ctx.host);
    w.changed = true;
    w.mobys[id].set_pv_i(0x1c, c as i32 + 1);
}

/// The path table's (`0x1ff1c0`) path `i`'s first point (the spline's `+0x10`).
fn path_start(ctx: &Ctx, i: i32) -> Option<[f32; 3]> {
    let p = usize::try_from(i).ok().and_then(|i| ctx.level.paths.get(i)).and_then(|p| p.first())?;
    Some([p[0], p[1], p[2]])
}

/// 0x0a24 (`0x3d90f0`): on its first tick update distance 0x80, pvars `+0x0c` −1.0, `+0x14` 0, at the first point of
/// its path `+4`, state 1. The game also keeps the pvars of its moby `+0` at `+8` and the path's address at `+0x18`
/// (here the indices at `+0` / `+4` stand for them; no ported class reads them).
pub fn update_0a24(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let at = path_start(ctx, w.mobys[id].pv_i(4));
    let m = &mut w.mobys[id];
    m.update_dist = 0x80;
    m.set_pv_f(0xc, -1.0);
    m.set_pv_i(0x14, 0);
    if let Some(p) = at { m.pos = p; }
    m.set_state(1, -1);
    w.changed = true;
}

/// 0x0a55 (`0x3dd430`): 0x0a24's start with the path at `+0` (its address kept at `+0x10`) and `+8` 0.
pub fn update_0a55(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let at = path_start(ctx, w.mobys[id].pv_i(0));
    let m = &mut w.mobys[id];
    m.update_dist = 0x80;
    m.set_pv_f(0xc, -1.0);
    m.set_pv_i(8, 0);
    if let Some(p) = at { m.pos = p; }
    m.set_state(1, -1);
    w.changed = true;
}

/// 0x0a4b (`0x3dd3c8`): drawn and within 64 of the camera (`0x1ba500`), its shadow's slab probed (`0x321db0`) and its
/// range 0x3c.
pub fn update_0a4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { probe_near(w, id, ctx, 64.0, 0x3c); }

/// 0x0adf (`0x3ecbb8`): a platform on the cuboid `+0x60` (−1: none, state 3). At its start it takes its place in the
/// cuboid's frame (the inverse `+0x40` of its offset from the cuboid; x at `+0x74`, z at `+0x7c`), its phase from
/// `+0x64` (`p·2π − π` at `+0x68`), pvars `+0x5c` |= 0x10 (state 2); then it swings along the frame's y, `0.25·sin`
/// of the phase stepping 60°/s (`$gp+0x1d88` / `$gp+0x1d8c`), with its looped sound 0 (handle `+0x80`), carrying
/// Ratchet (`0x326a60`, the block `+0x20`). A cuboid with a z scale of 2 makes it the lift (not ported: it stays).
pub fn update_0adf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_yaw) = (w.mobys[id].pos, w.mobys[id].rot[2]);
    let c = w.mobys[id].pv_i(0x60);
    let Some(v) = usize::try_from(c).ok().and_then(|i| ctx.level.cuboids.get(i)).copied() else {
        if w.mobys[id].state == 0 { w.mobys[id].set_state(3, -1); }
        return;
    };
    match w.mobys[id].state {
        0 => {
            if (v.matrix[2][2] - 2.0).abs() < 0.01 { return; }
            let m = &mut w.mobys[id];
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 0x10);
            let d: [f32; 3] = std::array::from_fn(|k| m.pos[k] - v.matrix[3][k]);
            let local: [f32; 3] = std::array::from_fn(|k| d[0] * v.inverse[0][k] + d[1] * v.inverse[1][k] + d[2] * v.inverse[2][k]);
            m.set_pv_f(0x68, m.pv_f(0x64) * std::f32::consts::TAU - std::f32::consts::PI);
            m.set_pv_f(0x74, local[0]);
            m.set_pv_i(0x80, -1);
            m.set_pv_f(0x7c, local[2]);
            m.set_state(2, -1);
        }
        2 => {
            let m = &mut w.mobys[id];
            let a = crate::hero::physics::add_rot(m.pv_f(0x68), 60.0 * 0.000_290_888_25);
            m.set_pv_f(0x68, a);
            m.set_pv_f(0x78, 0.25 * a.sin());
            let h = w.mobys[id].pv_i(0x80);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x80, h);
            }
            let m = &mut w.mobys[id];
            let l = [m.pv_f(0x74), m.pv_f(0x78), m.pv_f(0x7c)];
            m.pos = std::array::from_fn(|k| l[0] * v.matrix[0][k] + l[1] * v.matrix[1][k] + l[2] * v.matrix[2][k] + v.matrix[3][k]);
        }
        _ => {}
    }
    let m = &w.mobys[id];
    carry_hero(ctx, id, old, old_yaw, m.pos, m.rot[2]);
}

/// 0x0aab / 0x0aac (`0x3eb6f8`): a sliding door. At its start its home (`+0`) and a pulse phase (`+0x18`); closed (1)
/// its colour `+0x78` pulses (`0x800c0c00` | `(4·sin − 3)·208` within 0x50..0xd0, the phase 6° a tick) and, past 100
/// with Ratchet on a 0x0a61, it glows (two glows 315000 across at (0, −1.25, −2), frames 0 and 1, colour 0x20000012,
/// 15 ticks); hooked (`0x32a838`) it plays its sound 0 and opens (2): coloured 0xff0cff0c, it slides along its facing
/// (0x0aac forward, 0x0aab back) by `+0x10`, its speed `+0x14` gaining (2 + 5·the distance)/3600 a tick; at 2.5 it
/// bounces back at −0.5 of its speed until under 1/60 (state 3).
pub fn update_0aab(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let a = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0x18, a);
            let pos = m.pos;
            m.set_pv_v(0, pos);
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            let a = crate::hero::physics::add_rot(m.pv_f(0x18), f32::from_bits(0x3d56_7752));
            m.set_pv_f(0x18, a);
            let v = (((a.sin() * 4.0 - 3.0) * 208.0) as i32).clamp(0x50, 0xd0);
            let on_pad = ctx.hero.ground_moby.and_then(|g| w.mobys.get(g)).is_some_and(|g| g.o_class == 0x0a61);
            if on_pad && 100 < v {
                let (pos, far) = (w.mobys[id].pos, w.mobys[id].draw_dist);
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    for def in [0, 1] {
                        crate::particles::glow_def(sys, &mut ctx.game.rng, 315_000.0, 0.0, [0.0, -1.25, -2.0], id, pos, far, false, 0x2000_0012, 0xf, def);
                    }
                }
            }
            let m = &mut w.mobys[id];
            m.glow = v as u32 | 0x800c_0c00;
            if m.once & 4 == 0 { return; }
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            let m = &mut w.mobys[id];
            m.glow = 0xff0c_ff0c;
            let v = m.pv_f(0x14) + 2.0 * 0.000_277_777_78 + m.pv_f(0x10) * 0.000_277_777_78 * 5.0;
            m.set_pv_f(0x14, v);
            m.set_pv_f(0x10, m.pv_f(0x10) + v);
            if 2.5 < m.pv_f(0x10) {
                m.set_pv_f(0x10, 2.5);
                if 0.016_666_668 < m.pv_f(0x14) {
                    m.set_pv_f(0x14, m.pv_f(0x14) * -0.5);
                } else {
                    m.set_pv_f(0x14, 0.0);
                    m.set_state(3, -1);
                }
            }
            let d = if m.o_class == 0x0aac { m.pv_f(0x10) } else { -m.pv_f(0x10) };
            let along = crate::hero::physics::set_len(m.rows[0], d);
            m.pos = crate::hero::physics::add(along, m.pv_v(0));
            w.changed = true;
        }
        _ => {}
    }
}

/// 0x0d0d (`0x424cf8`): a breakable. At its start a target with all its pieces (`0x315c10`); a hit with the flags
/// 0x4010000 breaks it (state 2): no pieces drawn (`+0x7c` = 0; here hidden), `+0` 120, no collision, the level's bolts
/// (`0x32c250(1, m, 1)`), its sound 0, each piece thrown (spin 90..360°/s, gravity 10..20, `randf(−5, −2)` back,
/// `randf_signed(0, 2)` across, `randf(1, 6)` up a second through its rows), four pointed chunks `rand_vec(0, 0.25)`
/// about (−0.8, 0, 1.5) in its frame at `rand_vec(0, 5/60)` plus 1/12 back, `randf(0.5, 1)` of the size, colour
/// 0x50404040 (crate::moby::shards::throw_chunks), and its smoke ([`smoke_0d0d`]); then state 3. Not ported: the
/// count it lowers (`$gp−0x3c0`, 0x12cf's).
pub fn update_0d0d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, set_len};
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= mode::TARGET;
            m.set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            let m = &mut w.mobys[id];
            m.mode |= 1;
            m.set_pv_i(0, 0x78);
            m.has_collision = false;
            crate::moby::bolts::level_drop(w, id, 1.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let pieces = w.class(0x0d0d).map_or(0, |c| c.pieces.len());
            for k in 0..pieces.min(0x10) {
                let rng = &mut ctx.game.rng;
                let gravity = rng.randf(10.0, 20.0);
                let spin = rng.randf(90.0, 360.0);
                let v = [rng.randf(-2.0, -5.0) / 60.0, rng.randf_signed(0.0, 2.0) / 60.0, rng.randf(1.0, 6.0) / 60.0];
                let m = &w.mobys[id];
                let v = through(&m.rows, v);
                let (pos, rot) = (m.pos, m.rot);
                crate::moby::shards::spawn_shard(w, ctx, 0x0d0d, Some(id), pos, rot, None, spin, gravity, k as u8, v, 0xff);
            }
            let m = &w.mobys[id];
            let at = add(through(&m.rows, [-0.8, 0.0, 1.5]), m.pos);
            let back = set_len(m.rows[0], -0.083_333_336);
            crate::moby::shards::throw_chunks(w, ctx, 0.25, 5.0, (0.5, 1.0), id, at, 4, true, back, 0x5040_4040, None);
            smoke_0d0d(w, id, ctx, -0.083_333_336, -0.25);
            w.mobys[id].set_state(3, -1);
        }
        _ => {}
    }
    w.mobys[id].hit_slot = -1;
}

/// 0x0e93 (`0x42fb20`), a breakable lamp. At its start two glows 4 above it (`0x346250`: size 409500, colour
/// 0x28208080, the def's first frame; size 367500 pulled 0.75 toward the camera, 0x20208080, its second; far 0x60). A
/// hit with the flags 0x4010000 while `+0x68`[0] = 0 breaks it (state 2): its pieces thrown (`0x3311f8`: gravity
/// 10..20, spin 90..360°/s, `randf(−5, 2)` back, `randf_signed(0, 2)` across, `randf(1, 6)` up a second through its
/// rows; pieces 0, 1 and 3 pointed, the rest balls; piece 1 left out; no bolts there; its sound 0), the level's bolts
/// from 4 up (`0x32c250(4, m, 1)`), piece 1 a ball with no spin and gravity 15 at (0, −0.1, 0.05) a tick through its
/// rows (`0x2df348`), four pointed debris chunks `rand_vec(0, 0.25)` about (0, 0.2, 4.3) in its frame at
/// `rand_vec(0, 5/60)` plus (0, 0.05, 0.025) through its rows, `randf(0.5, 1)` of the size, colour 0x50404040
/// (crate::moby::shards::throw_chunks), its smoke ([`smoke_0e93`]); then it is deleted (`0x3157a0`).
pub fn update_0e93(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::add;
    match w.mobys[id].state {
        0 => {
            let pos = w.mobys[id].pos;
            if let Some(sys) = ctx.parts.as_deref_mut() {
                let rng = &mut ctx.game.rng;
                crate::particles::glow_def(sys, rng, 409_500.0, 0.0, [0.0, 0.0, 4.0], id, pos, 0x60, false, 0x2820_8080, -1, 0);
                crate::particles::glow_def(sys, rng, 367_500.0, 0.75, [0.0, 0.0, 4.0], id, pos, 0x60, false, 0x2020_8080, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
            w.mobys[id].hit_slot = -1;
        }
        2 => {
            crate::moby::shards::throw_all(w, id, ctx, [-5.0, 0.0, 1.0, 0.0, 2.0, 2.0, 6.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xb, 0, 1, 5, 0xffff);
            crate::moby::bolts::level_drop(w, id, 4.0, 1, ctx);
            let m = &w.mobys[id];
            let (rows, pos, rot, o_class) = (m.rows, m.pos, m.rot, m.o_class);
            crate::moby::shards::spawn_ball(w, ctx, o_class, Some(id), pos, rot, 0.0, 15.0, 1, through(&rows, [0.0, -0.1, 0.05]), 0xff);
            let at = add(through(&rows, [0.0, 0.2, 4.3]), pos);
            let vel = through(&rows, [0.0, 0.05, 0.025]);
            crate::moby::shards::throw_chunks(w, ctx, 0.25, 5.0, (0.5, 1.0), id, at, 4, true, vel, 0x5040_4040, None);
            smoke_0e93(w, id, ctx, 0.016_666_668, 0.233_333_33);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x42fea8(a, b, m)`: thirty staged puffs (crate::particles::staged, def 0x1003c) from 4 above it (each jittered
/// `randf(−0.25, 0.25)` a way, `0x325bd0`), along its +y row ± `randf(−0.8, 0.8)` of its first and `randf(0.25, 1)` of
/// its up at `randf(a, b)` a tick, `randf(0.2, 0.6)` across, growing in over `10·f`, easing over `30·f` (sinking 0.0025
/// a tick of it all) and out over `7·f` ticks (f `randf(0.5, 1.5)`), coloured `0x80004080..0x80408080` to
/// `0x00004080..0x00408080`.
fn smoke_0e93(w: &MobyWorld, id: usize, ctx: &mut Ctx, a: f32, b: f32) {
    use crate::hero::physics::{add, set_len};
    let m = &w.mobys[id];
    let base = add(m.pos, [0.0, 0.0, 4.0]);
    let rows = m.rows;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..30 {
        let c = set_len(rows[0], rng.randf(-0.8, 0.8));
        let d = set_len(rows[2], rng.randf(0.25, 1.0));
        let v = set_len(add(add(rows[1], c), d), rng.randf(a, b));
        let f = rng.randf(0.5, 1.5);
        let (i, e, o) = ((f * 10.0) as i32, (f * 30.0) as i32, (f * 7.0) as i32);
        let size = rng.randf(0.2, 0.6);
        let sink = -0.0025 * (i + e + o) as f32;
        let c0 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_4080, 0x8040_8080);
        let c1 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x0000_4080, 0x0040_8080);
        let mut p = base;
        for x in p.iter_mut() { *x += rng.randf(-0.25, 0.25); }
        crate::particles::staged(sys, rng, p, [v[0], v[1], v[2], size], [v[0], v[1], v[2] + sink, 0.01], c0, c1, i as i16, e as i16, o as i16, 0x1003c);
    }
}

fn through(rows: &[[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

/// `0x425010(a, b, m)`: twenty staged puffs (crate::particles::staged, def 0x1003c) from 1.75 up and 0.5 back in its
/// frame, each along its facing ± `randf(−1, 1)` across and `randf(−0.15, 1)` up at `randf(a, b)` a tick, `randf(0.2,
/// 0.6)` across, growing in over `10·f`, easing over `30·f` (sinking 1/720 a tick of it all) and out over `7·f` ticks
/// (f `randf(0.5, 1.5)`), coloured `0x80004080..0x80408080` to `0x80008040..0x00408040`.
fn smoke_0d0d(w: &MobyWorld, id: usize, ctx: &mut Ctx, a: f32, b: f32) {
    use crate::hero::physics::{add, scale, set_len};
    let m = &w.mobys[id];
    let base = add(add(scale(m.rows[2], 1.75), m.pos), scale(m.rows[0], -0.5));
    let rows = m.rows;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for _ in 0..20 {
        let c = set_len(rows[1], rng.randf(-1.0, 1.0));
        let d = set_len(rows[2], rng.randf(-0.15, 1.0));
        let v = set_len(add(add(rows[0], c), d), rng.randf(a, b));
        let f = rng.randf(0.5, 1.5);
        let (i, e, o) = ((f * 10.0) as i32, (f * 30.0) as i32, (f * 7.0) as i32);
        let size = rng.randf(0.2, 0.6);
        let sink = -0.001_388_888_9 * (i + e + o) as f32;
        let c0 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_4080, 0x8040_8080);
        let c1 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8000_8040, 0x0040_8040);
        crate::particles::staged(sys, rng, base, [v[0], v[1], v[2], size], [v[0], v[1], v[2] + sink, 0.01], c0, c1, i as i16, e as i16, o as i16, 0x1003c);
    }
}

/// The power node 0x0c82's class and the bomb that kills it.
pub const NODE: i16 = 0x0c82;
pub const BOMB: i16 = 0x09c8;

/// 0x0c82 (`0x413be8`), a power node feeding the lock 0x0c81. Gone at the start once its mission is done; else twice
/// its class's size, mode 0x8000 one time in two, its height kept (`+0x68`[8]) and two random phases. It bobs ½ about
/// that height (`[4]` on 0.0582 a tick), its spin phase `[0x14]` back 0.1047 a tick and its scroll `[0x10]` back 1/15
/// (in −1..1). Alive (1) it keeps its looped sound 0 (the handle `[0x18]`), fades in `[0xc]` (at 1/60, out past 64
/// from the camera) and, drawn, sparkles ([`node_sparkle`]); a hit with the flags 0x10000 from a bomb 0x09c8 within 5
/// kills it (2): no collision, hidden, its sound stopped. Not ported: its draw `0x414118` (an arc to the lock's cuboid
/// `[0]`, `0x2f03a8`), the word `0x18cfdc` its death sets to 0xffff.
pub fn update_0c82(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add_rot, approach, dist, dist2};
    match w.mobys[id].state {
        0 => {
            if ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize).is_some_and(|&b| b & 0x80 != 0) {
                w.delete(id);
                return;
            }
            let s = w.class(NODE).map_or(1.0, |c| c.scale);
            let rng = &mut ctx.game.rng;
            let flip = rng.randi(2) != 0;
            let m = &mut w.mobys[id];
            m.scale = s + s;
            if flip { m.mode |= 0x8000 } else { m.mode &= 0x7fff }
            m.set_state(1, -1);
            m.set_pv_f(8, m.pos[2]);
            let (a, b) = (ctx.game.rng.rand_angle(), ctx.game.rng.rand_angle());
            let m = &mut w.mobys[id];
            m.set_pv_f(4, a);
            m.set_pv_f(0x14, b);
            w.changed = true;
        }
        1 => {
            if w.mobys[id].drawn != 0 { node_sparkle(w, id, ctx); }
            let h = w.mobys[id].pv_i(0x18);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x18, h);
            }
            let far = 64.0 < dist2(w.mobys[id].pos, ctx.host.camera_pos());
            let m = &mut w.mobys[id];
            let mut f = m.pv_f(0xc);
            approach(if far { 0.0 } else { 1.0 }, 1.0 / 60.0, &mut f);
            m.set_pv_f(0xc, f);
            let bombed = w.hit_message(id).is_some_and(|h| {
                h.tmpl.flags & 0x10000 != 0 && h.tmpl.source.is_some_and(|s| w.mobys.get(s).is_some_and(|b| b.o_class == BOMB && dist(w.mobys[id].pos, b.pos) < 5.0))
            });
            if bombed {
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.has_collision = false;
                m.mode = (m.mode & !mode::TARGET) | 0x41;
                let h = m.pv_i(0x18);
                m.set_pv_i(0x18, -1);
                ctx.host.stop_sound(h);
                w.changed = true;
            }
            w.clear_hit(id);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(4), 0.058_178_3);
    m.set_pv_f(4, a);
    m.set_pv_f(0x14, add_rot(m.pv_f(0x14), -0.104_719_76));
    m.pos[2] = a.sin() * 0.5 + m.pv_f(8);
    let mut s = m.pv_f(0x10) - 0.066_666_67;
    if 1.0 < s { s -= 1.0 } else if s < -1.0 { s += 1.0 }
    m.set_pv_f(0x10, s);
    w.changed = true;
}

/// `0x413f50`: two glints 1.5 up its frame — one still there (size 6, 0x60006080, 5..10 ticks), one within 1.5 of it
/// sinking `randf(2, 5.5)` a second (size 0.75, 0x60004090, 40..90 ticks).
fn node_sparkle(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::add;
    let m = &w.mobys[id];
    let p = add(through(&m.rows, [0.0, 0.0, 1.5]), m.pos);
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let v = rng.rand_vec(0.0, 0.0);
    let life = rng.rand_range(5, 10) as u16;
    let rot = rng.randi(0xff) as u8;
    crate::particles::glint(sys, 6.0, p, v, 0x6000_6080, life, rot, false, hero);
    let at = add(rng.rand_vec(0.0, 1.5), p);
    let sink = -(rng.randf(2.0, 5.5) / 60.0);
    let life = rng.rand_range(0x28, 0x5a) as u16;
    let rot = rng.randi(0xff) as u8;
    crate::particles::glint(sys, 0.75, at, [0.0, 0.0, sink], 0x6000_4090, life, rot, false, hero);
}

/// `0x320b50(group, on)`: every member of the group hooked (`once & 4`, `0x330490`).
fn hook_group(w: &mut MobyWorld, group: i32, on: bool) {
    let Some(g) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in g {
        if let Some(m) = w.mobys.get_mut(i as usize) {
            if on { m.once |= 4 } else { m.once &= !4 }
        }
    }
}

/// 0x0c81 (`0x412e98`), the lock: four power nodes (`+0x68`[0..3]) hold it. At the start, its mission done, it hooks
/// its group `[4]` and goes; else hidden, update distance 0x65, its fade `[7]` 1 within 66 of the camera, its panels
/// `[0xc..0xf]` 1, its phases `[8]` 0, `[9]` ½, `[0xb]` random. With all four nodes dead (state 2) it fades out (2:
/// `[7]` down 1/60 a tick) and at 0 deletes them, hooks its group (the doors and bridges), does its mission, keeps a
/// checkpoint at its cuboid `[5]` and goes. Showing (`[7]` ≠ 0): Ratchet against it gives a glint where a line from
/// his centre meets it; in view its phases turn (`[0xb]` 0.0407, `[0xa]` 1/48, `[8]` / `[9]` 0.0111 a tick); its fade
/// eases to 1 within 68 of the camera (1/120). A panel goes with the nodes it stands for (`[0xc]` node 0, `[0xd]`
/// nodes 0 and 1, `[0xe]` 2 and 3, `[0xf]` 3). Not ported: its draw `0x413460` (seven effect panels).
pub fn update_0c81(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add_rot, approach, dist2};
    let node = |w: &MobyWorld, k: usize| usize::try_from(w.mobys[id].pv_i(4 * k)).ok().filter(|&n| n < w.mobys.len());
    let dead = |w: &MobyWorld, k: usize| node(w, k).is_some_and(|n| w.mobys[n].state == 2);
    let cam = ctx.host.camera_pos();
    match w.mobys[id].state {
        0 => {
            let done = ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize).is_some_and(|&b| b & 0x80 != 0);
            if done {
                hook_group(w, w.mobys[id].pv_i(0x10), true);
                w.delete(id);
                return;
            }
            let b = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, 0.0);
            m.set_pv_f(0x24, 0.5);
            m.set_pv_f(0x2c, b);
            m.set_pv_f(0x1c, if 66.0 < dist2(cam, m.pos) { 0.0 } else { 1.0 });
            for k in 0..4 { m.set_pv_f(0x30 + 4 * k, 1.0); }
            m.update_dist = 0x65;
            m.set_state(1, -1);
            m.drawn = 0;
            m.mode |= 1;
        }
        1 => {
            if (0..4).all(|k| dead(w, k)) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut f = m.pv_f(0x1c);
            approach(0.0, 1.0 / 60.0, &mut f);
            m.set_pv_f(0x1c, f);
            if f <= 0.0 {
                for k in 0..4 { if let Some(n) = node(w, k) { w.delete(n); } }
                hook_group(w, w.mobys[id].pv_i(0x10), true);
                ctx.game.mission_done(w.mobys[id].mission);
                if let Some((pos, rot)) = ctx.level.cuboid_place(w.mobys[id].pv_i(0x14)) { ctx.game.save_checkpoint(pos, rot); }
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    let f = w.mobys[id].pv_f(0x1c);
    if f == 0.0 {
        if dist2(w.mobys[id].pos, cam) < 64.0 {
            let mut f = f;
            approach(1.0, 0.008_333_334, &mut f);
            w.mobys[id].set_pv_f(0x1c, f);
        }
    } else {
        let h = &*ctx.hero;
        if h.wall_moby == Some(id) || h.push_moby == Some(id) {
            // `0x2e1e10(0x189f00, (its x, his centre's y, z), 2)`: the line from his centre; a glint where it meets.
            let from = [h.pos[0], h.pos[1], h.pos[2] + 1.0];
            let to = [w.mobys[id].pos[0], from[1], from[2]];
            if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(2), ctx.hero_moby)) {
                let hero = h.pos;
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    let rng = &mut ctx.game.rng;
                    let mut p = o.point;
                    p[2] += rng.randf(-0.5, 0.5);
                    let life = rng.rand_range(0x1e, 0x3c) as u16;
                    let rot = rng.randi(0xff) as u8;
                    crate::particles::glint(sys, 1.0, p, [0.0; 3], 0x6010_6070, life, rot, false, hero);
                }
            }
        }
        let m = &mut w.mobys[id];
        m.set_pv_f(0x2c, add_rot(m.pv_f(0x2c), 0.040_725_4));
        let wrap = |x: f32| if 1.0 < x { x - 1.0 } else { x };
        m.set_pv_f(0x28, wrap(m.pv_f(0x28) + 0.020_833_334));
        m.set_pv_f(0x20, wrap(m.pv_f(0x20) + 0.011_1));
        m.set_pv_f(0x24, wrap(m.pv_f(0x24) + 0.011_1));
        let far = 68.0 < dist2(m.pos, cam);
        let mut f = m.pv_f(0x1c);
        approach(if far { 0.0 } else { 1.0 }, 0.008_333_334, &mut f);
        m.set_pv_f(0x1c, f);
    }
    let alive: [bool; 4] = std::array::from_fn(|k| !dead(w, k));
    let m = &mut w.mobys[id];
    if m.pv_f(0x34) != 0.0 && !alive[0] && !alive[1] { m.set_pv_f(0x34, 0.0); }
    if m.pv_f(0x38) != 0.0 && !alive[2] && !alive[3] { m.set_pv_f(0x38, 0.0); }
    if m.pv_f(0x30) != 0.0 && !alive[0] { m.set_pv_f(0x30, 0.0); }
    if m.pv_f(0x3c) != 0.0 && !alive[3] { m.set_pv_f(0x3c, 0.0); }
}

/// The conveyor bridges' globals (`$gp+0x1d20..0x1d2c`) and the cuboids their beams draw over.
#[derive(Clone, Debug, Default)]
pub struct Conveyor {
    /// `$gp+0x1d24`: the belts' way, −1 (the data's) or 1; `reversed` holds 1.
    pub reversed: bool,
    /// `$gp+0x1d28`: the belts' speed a tick.
    pub speed: f32,
    /// The cuboids' matrices (`0x1b368c`), copied as the belts read them for their draws.
    pub cuboids: Vec<(i32, [[f32; 4]; 4])>,
}

/// `$gp+0x1d2c`: the beam's pulse, degrees a second.
const BELT_PULSE: f32 = 200.0;

/// The group's members of class `class`.
fn group_of(w: &MobyWorld, id: usize, class: i16) -> Vec<usize> {
    let g = w.mobys[id].group;
    if g < 0 { return Vec::new(); }
    w.groups.get(g as usize).map_or(Vec::new(), |v| v.iter().map(|&i| i as usize).filter(|&i| w.mobys.get(i).is_some_and(|m| m.o_class == class)).collect())
}

fn to_frame(v: crate::hero::physics::V3, r: &[crate::hero::physics::V3; 3]) -> crate::hero::physics::V3 {
    [crate::hero::physics::dot(v, r[0]), crate::hero::physics::dot(v, r[1]), crate::hero::physics::dot(v, r[2])]
}

/// 0x0ab0 (`0x3eba50`), a conveyor bridge's piece, in a group with its belt surfaces 0x0aaf. `+0x68`[0] its role: 0
/// a panel, else an end (the first end found becomes 2, the belt). The first to start sets the group up (`0x3ebfd8`):
/// each piece keeps its place `[0xc..0xe]`, its yaw `[0xf]` and the first's `[9]` (`[0xa]`, the frame the last panel
/// left); the ends wait (2); the panels (1) are folded, hidden and not solid, half a unit along the first's second
/// row and down from the ends' midpoint, turned π about x and a quarter turn more about z. Hooked (`once & 4`) for 6
/// ticks an end starts its looped sound 0 (`[0xb]`) and sets them off (`0x3ec2a8`): role 1 to 6 (done), 2 to 7, the
/// panels to 3. A panel waits 10 frames after the last one left and for none farther from home along its line
/// (`0x3ec380`); then shown and solid (4) it slides home along its line (its speed `[3]`: `0x323170`, accel 1/60, 60 at
/// first), then (5) turns home from where it stopped (`[4]` 0 → 1: accel and brake 1/360, at most 1/6; the place
/// lerped, its yaw a quarter turn and its roll π less by it) and rests (6). The belt (7) waits for every piece to rest,
/// stops its sound and makes the group solid (`0x3ec608`); then (8) runs: its speed eases (by 1/225) to the way / 15,
/// the belt surfaces carry Ratchet (and write their platform blocks) by it along its second row (`0x3ec6a8`); Ratchet
/// walking against it for 30 ticks turns the way; its pulse `[5]`, scroll `[6]` and fade `[7]` run and it draws
/// ([`draw_0ab0`]). Not ported: `[1]`'s camera (`0x1395bb`, `0x311738`), the switch `$gp+0x1d20` that opens them all.
pub fn update_0ab0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, add_rot, approach, dot, ease, scale, sub};
    use crate::moby::attach::rows_of;
    use std::f32::consts::{FRAC_PI_2, PI};
    let pieces = group_of(w, id, 0x0ab0);
    match w.mobys[id].state {
        0 => {
            let mut sum = [0.0; 3];
            let mut first_end = false;
            for &p in &pieces {
                let m = &mut w.mobys[p];
                let (pos, yaw) = (m.pos, m.rot[2]);
                m.set_pv_v(0x30, pos);
                m.set_pv_f(0x3c, yaw);
                m.set_pv_i(0x28, id as i32 + 1);
                if m.pv_i(0) == 0 {
                    m.set_state(1, -1);
                } else {
                    if !first_end {
                        first_end = true;
                        m.set_pv_i(0, 2);
                    }
                    sum = add(sum, pos);
                    m.set_state(2, -1);
                }
            }
            let mid = scale(sum, 0.5);
            let mut off = scale(w.mobys[id].rows[1], 0.5);
            off[2] -= 0.5;
            for &p in &pieces {
                let m = &mut w.mobys[p];
                if m.pv_i(0) != 0 { continue; }
                m.update_dist = 0xff;
                m.rot[2] = add_rot(m.rot[2], FRAC_PI_2);
                m.rot[0] = PI;
                m.pos = add(mid, off);
                m.has_collision = false;
                m.mode = (m.mode & !mode::TARGET) | 0x41;
                w.rebuild(p, ctx.host);
            }
            w.changed = true;
            return;
        }
        2 => {
            let m = &mut w.mobys[id];
            if m.once & 4 == 0 { return; }
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if t < 6 { return; }
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x2c, h);
            for &p in &pieces {
                let m = &mut w.mobys[p];
                let s = match m.pv_i(0) { 1 => 6, 2 => 7, _ => 3 };
                m.set_state(s, -1);
            }
            return;
        }
        3 => {
            let lead = usize::try_from(w.mobys[id].pv_i(0x28) - 1).ok().filter(|&l| l < w.mobys.len()).unwrap_or(id);
            let frame = ctx.game.frame as f32;
            if (frame - w.mobys[lead].pv_f(0x24)).abs() <= 10.0 { return; }
            let off = |w: &MobyWorld, p: usize| {
                let m = &w.mobys[p];
                let r = rows_of([0.0, 0.0, m.pv_f(0x3c)]);
                to_frame(sub(m.pv_v(0x30), m.pos), &r)[1].abs()
            };
            let mine = off(w, id);
            if pieces.iter().any(|&p| p != id && w.mobys[p].state == 3 && mine < off(w, p)) { return; }
            w.mobys[lead].set_pv_f(0x24, frame);
            let solid = w.class(0x0ab0).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[id];
            m.set_state(4, -1);
            m.mode = (m.mode & !0x41) | mode::TARGET;
            m.has_collision = solid;
            m.set_pv_f(0xc, 0.0);
            w.changed = true;
        }
        4 => {
            let first = w.mobys[id].state_ticks == 0;
            let m = &mut w.mobys[id];
            m.state_ticks = m.state_ticks.wrapping_add(1);
            let accel = if first { 60.0 } else { 1.0 / 60.0 };
            let r = rows_of([0.0, 0.0, m.pv_f(0x3c)]);
            let home = m.pv_v(0x30);
            let mut local = to_frame(sub(m.pos, home), &r);
            let mut v = m.pv_f(0xc);
            if local[1].abs() <= v.abs() {
                local[1] = 0.0;
                m.set_state(5, -1);
            } else {
                ease(0.0, accel, accel, 1.0, &mut local[1], &mut v);
            }
            m.set_pv_f(0xc, v);
            m.pos = add(home, through(&r, local));
            w.rebuild(id, ctx.host);
            w.changed = true;
            return;
        }
        5 => {
            let m = &mut w.mobys[id];
            if m.state_ticks == 0 {
                let p = m.pos;
                m.set_pv_v(0x40, p);
                m.set_pv_f(0x10, 0.0);
                m.set_pv_f(0xc, 0.0);
            }
            m.state_ticks = m.state_ticks.wrapping_add(1);
            let (mut t, mut v) = (m.pv_f(0x10), m.pv_f(0xc));
            if (t - 1.0).abs() < v.abs() {
                t = 1.0;
                m.set_state(6, -1);
            } else {
                ease(1.0, 0.002_777_778, 0.002_777_778, 0.166_666_67, &mut t, &mut v);
            }
            m.set_pv_f(0x10, t);
            m.set_pv_f(0xc, v);
            let (from, home) = (m.pv_v(0x40), m.pv_v(0x30));
            m.pos = add(from, scale(sub(home, from), t));
            m.rot[2] = add_rot(m.pv_f(0x3c), (1.0 - t) * FRAC_PI_2);
            m.rot[0] = (1.0 - t) * PI;
            w.rebuild(id, ctx.host);
            w.changed = true;
        }
        7 => {
            if pieces.iter().any(|&p| !matches!(w.mobys[p].state, 6 | 7)) { return; }
            let h = w.mobys[id].pv_i(0x2c);
            ctx.host.stop_sound(h);
            w.mobys[id].set_pv_i(0x2c, -1);
            if let Some(g) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned() {
                for i in g {
                    let i = i as usize;
                    let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
                    w.mobys[i].has_collision = solid;
                }
            }
            w.mobys[id].set_state(8, -1);
            w.changed = true;
        }
        8 => {
            let c = &mut w.conveyor;
            let target = if c.reversed { 1.0 } else { -1.0 } * 0.066_666_67;
            approach(target, 0.004_444_444, &mut c.speed);
            let speed = c.speed;
            let v = scale(w.mobys[id].rows[1], speed);
            if belt_carry(w, id, ctx, v) {
                let m = &mut w.mobys[id];
                if speed == target && dot(ctx.hero.vel, v) < -0.001 {
                    let n = m.pv_i(0x20) + 1;
                    m.set_pv_i(0x20, n);
                    if 0x1e < n { w.conveyor.reversed = !w.conveyor.reversed; }
                } else {
                    m.set_pv_i(0x20, 0);
                }
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(0x14, add_rot(m.pv_f(0x14), BELT_PULSE * 0.000_290_888_25));
            let mut s = m.pv_f(0x18) + speed * 0.5;
            if 1.0 < s { s -= 1.0 } else if s < -1.0 { s += 1.0 }
            m.set_pv_f(0x18, s);
            let mut f = m.pv_f(0x1c);
            approach(1.0, 1.0 / 60.0, &mut f);
            m.set_pv_f(0x1c, f);
            let k = m.pv_i(8);
            if let Some(cb) = usize::try_from(k).ok().and_then(|i| ctx.level.cuboids.get(i)) {
                let mat = cb.matrix;
                let cs = &mut w.conveyor.cuboids;
                match cs.iter_mut().find(|(i, _)| *i == k) { Some(e) => e.1 = mat, None => cs.push((k, mat)) }
            }
            w.register_draw(id);
        }
        _ => {}
    }
}

/// `0x3ec6a8(belt, v)`: each belt surface 0x0aaf of the group moves things on it by `v` this tick (its platform
/// block, crate::moby::platform, when it has one; Ratchet by crate::moby::carry_hero); true when Ratchet stands on one.
fn belt_carry(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, v: crate::hero::physics::V3) -> bool {
    let mut on = false;
    for s in group_of(w, id, 0x0aaf) {
        if ctx.hero.ground_moby == Some(s) && ctx.hero.grounded_ticks != 0 { on = true; }
        let m = &mut w.mobys[s];
        if let Some(b) = crate::moby::platform::block(m) {
            let r = m.rot;
            crate::moby::platform::write(m, b, v, r, r);
        }
        crate::moby::carry_hero(ctx, s, [0.0; 3], 0.0, v, 0.0);
    }
    on
}

/// `0x3ec7c8`, the belt's draw: over its cuboid `[2]` (`0x1b368c`), a strip of n quads along the cuboid's second axis
/// (n its length, whole), each from x = 1 to −1 and y a 2/n step of −1..1 in the cuboid's frame, twice: effect 5 with
/// texture (0.5, 0.5..1.5) coloured `sin(pulse)` of the way from 0x30805000 to 0x20805000, and effect 0x10 with s
/// scrolling by `[6]`, t 0.15..0.85, coloured `sin(pulse + π)` of the way from 0x30508000 to 0x20508000; the alphas
/// × its fade `[7]`; additive.
pub fn draw_0ab0(w: &MobyWorld, id: usize, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::len;
    let m = &w.mobys[id];
    let k = m.pv_i(8);
    let Some(&(_, mat)) = w.conveyor.cuboids.iter().find(|(i, _)| *i == k) else { return };
    let n = len([mat[1][0], mat[1][1], mat[1][2]]) as i32;
    if n < 1 { return; }
    let (p, scroll, fade) = (m.pv_f(0x14), m.pv_f(0x18), m.pv_f(0x1c));
    let fade_rgba = |c: u32| if fade < 1.0 { crate::hud::lerp_rgba(fade, c & 0xff_ffff, c) } else { c };
    let c0 = fade_rgba(crate::hud::lerp_rgba(p.sin() * 0.5 + 0.5, 0x3080_5000, 0x2080_5000));
    let c1 = fade_rgba(crate::hud::lerp_rgba((p + std::f32::consts::PI).sin() * 0.5 + 0.5, 0x3050_8000, 0x2050_8000));
    let world = |x: f32, y: f32| -> crate::hero::physics::V3 { std::array::from_fn(|j| x * mat[0][j] + y * mat[1][j] + mat[3][j]) };
    let step = 2.0 / n as f32;
    for i in 0..n {
        let y0 = -1.0 + step * i as f32;
        let corners = [world(1.0, y0 + step), world(1.0, y0), world(-1.0, y0 + step), world(-1.0, y0)];
        out.push(crate::fx::FxQuad { fx: 5, additive: true, corners, st: [[0.5, 0.5], [0.5, 0.5], [0.5, 1.5], [0.5, 1.5]], rgba: [c0; 4] });
        out.push(crate::fx::FxQuad { fx: 0x10, additive: true, corners, st: [[scroll, 0.15], [1.0 + scroll, 0.15], [scroll, 0.85], [1.0 + scroll, 0.85]], rgba: [c1; 4] });
    }
}
