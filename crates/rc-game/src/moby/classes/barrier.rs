//! The barrier **0x112e** (L13 `0x420d98`, 5 placed; L16 `0x41a6e0`, 5; the decomp's `func_L13_0041F8B8` /
//! `func_L16_004192A8`, one code): a curtain drawn by a callback (L13 `0x4210b8`, the decomp's `func_L13_0041FBE0`),
//! itself hidden (mode | 1, not drawn). Its pvars (floats): `[0]` its fade (1), `[1]` / `[2]` its two edge bands'
//! phases, `[3]` the scroll, `[4..7]` 1, `[8]` the pulse's phase (a random angle).
//!
//! * **0** the set-up above; update distance 0xff; then 1.
//! * **1** on level 16: Ratchet's sphere (0.5 at 0.75 up his up row from his feet, `0x2ea440` = L00 `0x2df8d8`, not
//!   meeting him) meeting it, or Ratchet standing on it (`0x18a15c`) on the ground (`0x18a160`): he dies (`0x2be1f8` =
//!   L00 `0x2b31f0`, crate::hero::Hero::die; the hits it builds first, 100 and 10, are delivered by nothing).
//! * Each tick while `[0]` ≠ 0: the pulse on 140° a second, the scroll by `1.25 + randf_signed(0.25, 0.5)` a second
//!   (wrapped past 1), the bands by a third a second each, `[0]` toward 1 by 1/120 (`0x328d90` = L00 `0x31eb18`); its
//!   draw put on the frame's first list (`0x2f21f8` = L00 `0x2e7718`).
//!
//! The tunables: L13 `$gp+0x29f0..0x29f8` (L16 `$gp+0x2c50..`, the same values).
//!
//! Not ported: the curtain's draw (`0x4210b8`: seven packets of the level's words `0x1b1a40..`, the faces turned
//! about the camera's side, the bands' colours eased by the pulse and the fade, through `0x2e1510`).

use crate::hero::physics::{add, add_rot, approach, scale};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x112e;

/// The pulse (degrees a second), the scroll's base and the bands' speed (a second).
const PULSE: f32 = 140.0;
const SCROLL: f32 = 1.25;
const BANDS: f32 = f32::from_bits(0x3f2a_7efa);

/// 0x112e (module docs).
pub fn update_112e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x24 { w.mobys[id].pvars.resize(0x24, 0); }
    match w.mobys[id].state {
        0 => {
            let a = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0, 1.0);
            m.set_pv_f(0x20, a);
            m.set_pv_f(8, 0.5);
            m.set_pv_f(4, 0.0);
            m.set_pv_f(0, 1.0);
            for k in 0..4 { m.set_pv_f(0x10 + 4 * k, 1.0); }
            m.update_dist = 0xff;
            m.set_state(1, -1);
            m.drawn = 0;
            m.mode |= 1;
        }
        1 => {
            if ctx.game.level == 0x10 {
                let at = add(ctx.hero.pos, scale(ctx.hero.rows[2], 0.75));
                let met = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, at, 0.5, crate::collision::QueryFlags(0), ctx.hero_moby)).is_some_and(|o| o.moby == Some(id));
                if met { ctx.hero.die(); }
                if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 { ctx.hero.die(); }
            }
        }
        _ => {}
    }
    if w.mobys[id].pv_f(0) != 0.0 {
        let r = ctx.game.rng.randf_signed(0.25, 0.5);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x20, add_rot(m.pv_f(0x20), PULSE * f32::from_bits(0x3998_825c)));
        let mut s = m.pv_f(0xc) + (SCROLL + r) * f32::from_bits(0x3c88_8889);
        if 1.0 < s { s -= 1.0; }
        m.set_pv_f(0xc, s);
        for k in [4, 8] {
            let v = m.pv_f(k) + BANDS * f32::from_bits(0x3c88_8889);
            m.set_pv_f(k, if 1.0 < v { v - 1.0 } else { v });
        }
        let mut f = m.pv_f(0);
        approach(1.0, f32::from_bits(0x3c08_8889), &mut f);
        m.set_pv_f(0, f);
    }
}
