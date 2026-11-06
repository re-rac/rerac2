//! Snivelak's energy gate **0x10fb** (`0x4191e8`, 3 placed; the decomp's `func_L16_00417DB0`): a sheet of light that
//! closes behind Ratchet once its moby is gone, and opens when its group is. Its pvars: `+0x00` the moby it waits on
//! (the placed 0x0200s), `+0x04` the moby group that holds it, `+0x08` the cuboid Ratchet must be in, `+0x0c` its
//! sound's handle, `+0x10` / `+0x18` the sheet's two scrolls and `+0x14` / `+0x1c` their steps.
//!
//! * **0** no collision (`+0x98` 0), hidden (mode `| 0x41`: only its draw shows it), 1.
//! * **1** its moby gone (state 0xfd / 0xfe) and Ratchet in its cuboid (`0x31ae60` = L00 `0x321858`): its class's
//!   collision on, 2.
//! * **2** each scroll on by its step (back by 8 at 8), new steps `randf(0.003, 0.005)` / `randf(0.001, 0.0025)`; its
//!   draw (`0x419620`, crate::moby::classes::level14::sheet's with [`sheet::L16`]) registered; its group with no
//!   living member (`0x315fd0` = L00 `0x31c8e0`): no collision, 3 (off). Ratchet's sphere (0.5 at 0.75 above his
//!   feet, `0x2d8bb0` = L00 `0x2df8d8`) meeting a 0x10fb: three sparks at the point (each first its sound 1 looped
//!   unless it already plays; a twinkle `0x3324d0` = L00 `0x33a6a0`: size `randf(1, 2)`·0.1 growing from that,
//!   falling 0.006, 15 ticks, 0xff803040, its spin ±1, moving `randf(0.05, 0.1)` a tick a random way up
//!   20°..88°, from the point jittered `±(0..0.15)` each tick); else its sound stopped.

use crate::moby::classes::level14::sheet;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x10fb;

/// 0x10fb (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x20 { w.mobys[id].pvars.resize(0x20, 0); }
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.has_collision = false;
        m.mode |= 0x41;
        w.changed = true;
    }
    if w.mobys[id].state == 1 {
        let t = w.mobys[id].pv_i(0);
        let gone = usize::try_from(t).ok().and_then(|t| w.mobys.get(t)).is_none_or(|t| matches!(t.state, 0xfd | 0xfe));
        if gone && ctx.level.in_cuboid(w.mobys[id].pv_i(8), ctx.hero.pos) {
            let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[id];
            m.state = 2;
            m.has_collision = solid;
            w.changed = true;
        }
    }
    if w.mobys[id].state != 2 { return; }
    let rng = &mut ctx.game.rng;
    let (s, t) = (rng.randf(0.003, 0.005), rng.randf(0.001, 0.0025));
    let m = &mut w.mobys[id];
    for (x, v) in [(0x10, 0x14), (0x18, 0x1c)] {
        let a = m.pv_f(x) + m.pv_f(v);
        m.set_pv_f(x, if 8.0 <= a { a - 8.0 } else { a });
    }
    m.set_pv_f(0x14, s);
    m.set_pv_f(0x1c, t);
    w.register_draw(id);
    let group = w.mobys[id].pv_i(4);
    let living = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).map_or(0, |g| g.iter().filter(|&&k| w.mobys.get(k).is_some_and(|x| x.state < 0x80)).count());
    if living == 0 {
        let m = &mut w.mobys[id];
        m.has_collision = false;
        m.state = 3;
        w.changed = true;
    }
    let p = ctx.hero.pos;
    let at = [p[0], p[1], p[2] + 0.75];
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, at, 0.5, crate::collision::QueryFlags(0), ctx.hero_moby));
    let touched = hit.filter(|o| o.moby.and_then(|k| w.mobys.get(k)).is_some_and(|k| k.o_class == CLASS));
    let Some(hit) = touched else {
        let h = w.mobys[id].pv_i(0xc);
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0xc, -1);
        return;
    };
    let mut point = hit.point;
    for _ in 0..3 {
        let h = w.mobys[id].pv_i(0xc);
        if h == -1 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
            w.mobys[id].set_pv_i(0xc, h);
        }
        let rng = &mut ctx.game.rng;
        let speed = rng.randf(1.0, 2.0);
        let spin = if rng.randi(2) == 0 { -1 } else { 1 };
        let a = rng.rand_angle();
        let pitch = rng.randf(f32::from_bits(0x3eb2_b8c4), f32::from_bits(0x3fc4_980a));
        let r = rng.randf(0.05, f32::from_bits(0x3dcc_cccd));
        // `0x31daa0` = L00 `0x325100(r, a, pitch)`.
        let vel = [a.cos() * r * pitch.cos(), a.sin() * r * pitch.cos(), pitch.sin() * r];
        for k in 0..3 { point[k] += rng.randf_signed(0.0, f32::from_bits(0x3e19_999a)); }
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::twinkle(sys, &mut ctx.game.rng, speed * f32::from_bits(0x3dcc_cccd), speed, f32::from_bits(0x3bc4_9ba6), point, 0xf, 0xff80_3040, 0, spin, vel, 0);
        }
    }
}

/// `0x419620` (module docs).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, out: &mut Vec<crate::fx::FxQuad>) { sheet::draw_as(w, id, &sheet::L16, rng, out) }
