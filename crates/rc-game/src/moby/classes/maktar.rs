//! Maktar Nebula's small objects (L02 class updates; no names in any reference, so they go by o_class).
//!
//! * 0x0afc (`0x3f0618`): no update (an empty function); 0x0873 creates one in its place.
//! * 0x0aaf (`0x3eb9f8`): on its first tick sequence state 1, no collision, mode 0x43 without TARGET ([`update_0aaf`]).
//! * 0x0873 (`0x3c1d20`): on its first tick a 0x0afc in its place ([`update_0873`]).
//! * 0x0a24 (`0x3d90f0`) / 0x0a55 (`0x3dd430`): on their first tick at their path's first point ([`update_0a24`],
//!   [`update_0a55`]).
//! * 0x0a4b (`0x3dd3c8`): its shadow's slab probed while drawn within 64 of the camera, range 0x3c ([`update_0a4b`]).
//! * 0x0adf (`0x3ecbb8`): a platform swinging along its cuboid, carrying Ratchet ([`update_0adf`]).
//! * 0x0e93 (`0x42fb20`): a breakable (crate::moby::classes::props::update_breakable: a hit with the flags 0x4010000
//!   while `+0x68`[0] = 0); its start's two glows (`0x346250`), its break's shards, burst, debris and sound are not
//!   ported, nor the count of them `0x12cf` keeps (`$gp−0x3c0`; at 10 left, once: `0x367f58`, `0x32c310(0x1233)`).
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
