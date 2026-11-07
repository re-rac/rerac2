//! Gorn's storm 0x1274 (L15 `0x4398d8`, 1 placed; the decomp's `func_L15_00437B18`): each tick it registers three
//! callbacks — `0x439ff8` on the list `0x300580` (L00 `0x2e77e0`), `0x43a020` on the frame's first list (`0x3004b8`,
//! L00 `0x2e7718`) and `0x43a2d8` on the draws (`0x3006c0`, L00 `0x2e7920`); the storm is on while `$gp+0x283c` is set
//! (1 in the level's data, nothing clears it). Its pvar `+0` is the player's ship (by index).
//!
//! * **The lightning and its hits** (`0x43a020`, run here in its update, [`update`]): when its timer (`+0xc`) is out,
//!   a bolt (`0x433e70`, class 0x11aa: from `a` to `b`, 6..10 ticks `+0x50`, twice that `+0x54`) from 800 up, 800 out
//!   along the ship's facing turned ±20° about its up, down to 400 — one more half the time; the next in 90..350
//!   ticks. With the ship at 565 or higher, a bolt from 800 straight down to 100 under it, and the hit; at 415 or
//!   lower the hit alone: the sparks 10 ahead of it (`0x43a8c0`), the ship's sound 16, 40 off the ship (`0x382740`,
//!   crate::moby::classes::space::hurt_ship).
//! * **The cloud layers** (`0x43a2d8`, [`draw`]): quads 0.5 in front of the camera, 0.35 each way along its other
//!   rows (texture FX 0x48, normal blend, the colour 0x808080): the first scrolling 0.00025 a frame down its t, its
//!   alpha `128 − min(650 − z, 90)` by the ship's height; the second 0.0015 along s at `90 − that`; below 465 (with
//!   `$gp+0x2834`, 1) two more, 0.0025 along t and 0.0035 along s, at `64 − 3·min(z − 422, 42)` (0..128) and 40
//!   less. Each scroll wraps at −8.
//!
//! The bolts 0x11aa (`0x433f68`, [`update_bolt`]): their life (`+0x54`) counted, then gone.
//!
//! Not ported: the bolts' arcs and draw (`0x433fd0`, the four channels `0x269fe0` of ten points, `0x4328e8`), the
//! hit's sparks (`0x43a8c0`: 40 of the particle `0x2c2210`), the storm's sky layer `0x439928` (asm, from the first
//! two callbacks).

use crate::hero::physics::{add, scale, sub, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1274;
/// The lightning bolt.
pub const BOLT: i16 = 0x11aa;

const TIMER: usize = 0xc;
const SCROLL: [usize; 4] = [0x4, 0x10, 0x14, 0x18];

fn ship(w: &MobyWorld, id: usize) -> Option<usize> {
    let i = w.mobys[id].pv_i(0);
    w.mobys.iter().position(|m| m.index as i32 == i && m.state < 0x80).or_else(|| w.mobys.iter().position(|m| m.o_class == crate::ship::SHIP_CLASS && m.state < 0x80))
}

/// `0x433e70(a, b)`: a bolt from `a` to `b` (updated from 0xff, drawn from 0xff, state 1).
fn bolt(w: &mut MobyWorld, ctx: &mut Ctx, a: V3, b: V3) {
    if w.class(BOLT).is_none() { return; }
    let r = ctx.game.rng.randf(6.0, 10.0) as i32;
    let id = w.create(BOLT);
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 1;
    m.pos = a;
    m.set_pv_v(0, a);
    m.set_pv_v(0x10, b);
    m.set_pv_i(0x50, r);
    m.set_pv_i(0x54, r * 2);
    w.rebuild(id, &*ctx.host);
}

/// `0x433f68`: in state 1 its life counted, then gone.
pub fn update_bolt(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    if w.mobys[id].state != 1 { return; }
    let mut t = w.mobys[id].pv_i(0x54);
    if countdown(&mut t) { return w.delete(id); }
    w.mobys[id].set_pv_i(0x54, t);
}

/// `0x4398d8`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.register_draw(id);
    let mut t = w.mobys[id].pv_i(TIMER);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(TIMER, t);
    if !out { return; }
    let Some(s) = ship(w, id) else { return };
    let rows = w.mobys[s].rows;
    let sp = w.mobys[s].pos;
    let turn_by = ctx.game.rng.randf(-20.0, 20.0) * 0.017_453_294;
    let mut a = add(scale(crate::camera::rot(rows[0], turn_by, rows[2]), 800.0), sp);
    a[2] = 800.0;
    let mut b = a;
    b[2] = 400.0;
    bolt(w, ctx, a, b);
    if ctx.game.rng.randi(2) != 0 { bolt(w, ctx, a, b); }
    let next = ctx.game.rng.randf(90.0, 350.0) as i32;
    w.mobys[id].set_pv_i(TIMER, next);
    let z = sp[2];
    if 565.0 <= z {
        let (mut a, mut b) = (sp, sp);
        a[2] = 800.0;
        b[2] = z - 100.0;
        bolt(w, ctx, a, b);
    }
    if 565.0 <= z || z <= 415.0 {
        ctx.host.play_sound_of(&w.mobys[id], crate::ship::SHIP_CLASS, 16, 0);
        crate::moby::classes::space::hurt_ship(w, 40.0);
    }
}

/// `0x43a2d8` (module docs): the cloud layers.
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let rows = w.cam_rows;
    let z = ship(w, id).map_or(cam[2], |s| w.mobys[s].pos[2]);
    let c = add(cam, scale(rows[0], 0.5));
    let (up, side) = (scale(rows[1], 0.35), scale(rows[2], 0.35));
    let corners = [add(sub(c, up), side), sub(sub(c, up), side), add(add(c, up), side), sub(add(c, up), side)];
    let step = |m: &mut crate::moby::Moby, o: usize, by: f32| {
        let mut v = m.pv_f(o) - by;
        if v <= -8.0 { v += 8.0; }
        m.set_pv_f(o, v);
        v
    };
    let m = &mut w.mobys[id];
    let quad = |st: [[f32; 2]; 4], alpha: i32| crate::fx::FxQuad { fx: 0x48, additive: false, corners, st, rgba: [((alpha.clamp(0, 255) as u32) << 24) | 0x80_8080; 4] };
    let along_t = |v: f32| [[0.0, v], [0.0, v + 1.0], [1.0, v], [1.0, v + 1.0]];
    let along_s = |v: f32| [[v, 0.0], [v, 1.0], [v + 1.0, 0.0], [v + 1.0, 1.0]];
    let high = ((650.0 - z) as i32).min(90);
    let v = step(m, SCROLL[0], 0.000_25);
    out.push(quad(along_t(v), 128 - high));
    let v = step(m, SCROLL[1], 0.0015);
    out.push(quad(along_s(v), 90 - high));
    if z < 465.0 {
        let fade = ((z - 422.0) as i32).min(42);
        let alpha = (64 - fade * 3).clamp(0, 128);
        let v = step(m, SCROLL[2], 0.0025);
        out.push(quad(along_t(v), alpha));
        let v = step(m, SCROLL[3], 0.0035);
        out.push(quad(along_s(v), (alpha - 40).max(0)));
    }
}
