//! Snivelak's rock face **0x0e85** ([`update_0e85`], `0x3f69f8`, 6 placed; the decomp's `func_L16_003F54F8`;
//! L16 addresses): broken, it throws the rocks 0x0a80 (crate::moby::classes::thrown_rock).
//!
//! **0x0e85**: **0**: draw distance 0x200, mode | 0x4000; 1. **1**: a hit with the flags 0x80000 (`0x317530`) breaks
//! it: 120 rocks (`$gp+0x2098`, [`rocks`], `0x3f6aa0`) and 200 puffs (`$gp+0x209c`, [`puffs`], `0x3f6c08`) out of
//! its face, its class sound 0 (`0x3584d0` = L00 `0x3605f0`), deleted (`0x30ae50`).

use crate::hero::physics::{add, scale, V3};
use crate::moby::classes::thrown_rock::spawn;
use crate::moby::{Ctx, MobyWorld};

pub const FACE: i16 = 0x0e85;

/// A random point on the face: x ±0..26, y ±0..8, z 0..30, none with |x| < 9 and |z| < 25; in its frame by its scale's
/// share of its class's.
fn face_point(w: &MobyWorld, id: usize, ctx: &mut Ctx) -> Option<V3> {
    let rng = &mut ctx.game.rng;
    let p = [rng.randf_signed(0.0, 26.0), rng.randf_signed(0.0, 8.0), rng.randf(0.0, 30.0)];
    if p[0].abs() < 9.0 && p[2].abs() < 25.0 { return None; }
    let m = &w.mobys[id];
    let k = m.scale / w.class(m.o_class).map_or(1.0, |c| c.scale);
    let p = scale(p, k);
    Some(add(std::array::from_fn(|i| p[0] * m.rows[0][i] + p[1] * m.rows[1][i] + p[2] * m.rows[2][i]), m.pos))
}

/// `0x3f6aa0`: 120 tries of a rock at a [`face_point`], at random angles.
fn rocks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for _ in 0..120 {
        let Some(p) = face_point(w, id, ctx) else { continue };
        let rng = &mut ctx.game.rng;
        let rot = [rng.rand_angle(), rng.rand_angle(), rng.rand_angle()];
        spawn(w, ctx, id, p, rot);
    }
}

/// `0x3f6c08`: 200 tries of a still puff (`0x32a920` = L00 `0x3327e8`, crate::particles::staged, the def −1) at a
/// [`face_point`]: between 0x80808080 / 0x80406080 and 0x20808080 / 0x20406080 (`0x2e93f0` = L00 `0x2f0000`), 6 or
/// 5 across (`$gp+0x20b0`), in 1..5 ticks, easing 60..180, out 60.
fn puffs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for _ in 0..200 {
        let Some(p) = face_point(w, id, ctx) else { continue };
        let rng = &mut ctx.game.rng;
        let c0 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8080_8080, 0x8040_6080);
        let c1 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x2080_8080, 0x2040_6080);
        let size = [6.0, 5.0][rng.randi(2) as usize];
        let a = rng.rand_range(1, 5);
        let b = rng.rand_range(0x3c, 0xb4);
        let Some(sys) = ctx.parts.as_deref_mut() else { continue };
        let v = [0.0, 0.0, 0.0, size];
        crate::particles::staged(sys, &mut ctx.game.rng, p, v, v, c0, c1, a as i16, b as i16, 0x3c, -1);
    }
}

/// 0x0e85 (module docs).
pub fn update_0e85(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.draw_dist = 0x200;
            m.mode |= crate::moby::mode::LINE;
            m.set_state(1, -1);
        }
        1 => {
            if !w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0008_0000 != 0) { return; }
            rocks(w, id, ctx);
            puffs(w, id, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
        }
        _ => {}
    }
}
