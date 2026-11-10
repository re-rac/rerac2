//! Level 13's own breakables (L13 addresses; the engine functions named by their L13 addresses with the L00 ones the
//! port knows them by). Each waits in state 1 for the hit of crate::moby::classes::breakables ([`hit`]) and breaks in
//! state 2. Level 13's 0x130b and 0x130f, which the museum shares, are there too.
//!
//! * **0x130a** ([`update_130a`], `0x38d9c0`, 28 placed): a glow at its head; every piece, the bolts and a blast at
//!   the head.
//! * **0x130d** ([`update_130d`], `0x38dc58`, 11 placed): a glow; its class sound, four times the bolts, a blast and
//!   its four pieces thrown four times, a quarter turn apart.
//!
//! The engine functions: `0x3345e0` = L00 `0x32b580` (crate::moby::shards::throw_all), `0x330570` = L00 `0x327388`
//! ([`bolts::level_drop`]), `0x330680` = L00 `0x327498` ([`shards::break_into`]), `0x348fc0` = L00 `0x33f550`
//! (crate::particles::glow_def), `0x3fa180` = L00 `0x3f8348` (crate::moby::blast::burst), `0x32f248` a point in its
//! frame ([`frame_point`]), `0x36a110` the class sound, `0x2fa528` add_rot, `0x2f9d80` the rows from the angles.

use super::super::breakables::{blast_at, hit, throw};
use crate::hero::physics::{add, add_rot, euler_rows, V3};
use crate::moby::{bolts, shards, Ctx, MobyWorld};

/// `0x32f248(x, y, z, m, out)`: a point in the moby's frame (its rows, unscaled) plus its place.
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(std::array::from_fn(|k| v[0] * m.rows[0][k] + v[1] * m.rows[1][k] + v[2] * m.rows[2][k]), m.pos)
}

/// `$gp−0x470`: 0x130a's pieces' velocity ranges (z 3..9).
const H_130A: [f32; 8] = [-3.0, -3.0, 3.0, 0.0, 3.0, 3.0, 9.0, 0.0];

/// `$gp−0x450`: 0x130a's head in its frame.
const HEAD_130A: V3 = [0.0, 0.0, f32::from_bits(0x4074_49ba)];

/// 0x130a (`0x38d9c0`). **0**: a glow at its head (`0x348fc0`: 210000 of 0x50204040 pulled 0.75 at (0, 0, 3.817)
/// unturned, the def's frame 1, lasting, far its draw distance), 1. **1**: the hit ([`hit`]) → 2. **2**: its pieces
/// ([`throw`], `$gp−0x470`), the blast of size 0.3 ([`blast_at`]) at its head in its frame; deleted.
pub fn update_130a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 210_000.0, 0.75, HEAD_130A, id, pos, far, false, 0x5020_4040, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            throw(w, id, ctx, H_130A);
            let at = frame_point(w, id, HEAD_130A);
            blast_at(w, ctx, at, 0.3);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x130d (`0x38dc58`). **0**: a glow (`0x348fc0`: 420000 of 0x40204080 pulled 1 at (0, 0, 2) unturned, the def's
/// frame 1, lasting, far its draw distance), 1. **1**: the hit ([`hit`]) → 2. **2**: its class sound 0 (`0x36a110`),
/// four times the level's bolts (`0x330570(0, m, 4)`), the blast of size 0.5 ([`blast_at`]) at its place; then four
/// times its yaw turned a quarter (`0x2fa528`), its rows rebuilt (`0x2f9d80`) and its pieces 0..3 thrown
/// (`0x330680(3, 6, 3, 6, m, 0xf, 0, 0xff)`: 3..6 a second across and up); deleted.
pub fn update_130d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 420_000.0, 1.0, [0.0, 0.0, 2.0], id, pos, far, false, 0x4020_4080, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            bolts::level_drop(w, id, 0.0, 4, ctx);
            let pos = w.mobys[id].pos;
            blast_at(w, ctx, pos, 0.5);
            for _ in 0..4 {
                let m = &mut w.mobys[id];
                m.rot[2] = add_rot(m.rot[2], std::f32::consts::FRAC_PI_2);
                m.rows = euler_rows(m.rot);
                shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xf, [0.0; 3], 0xff);
            }
            w.delete(id);
        }
        _ => {}
    }
}
