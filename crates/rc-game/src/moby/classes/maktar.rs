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
