//! Level 17's vent **0x02be** (`0x3a2950`, 5 placed; the decomp's `func_L17_003A14A0`): a jet of vapour
//! (crate::particles::vapour) blown along its facing for a while, then a pause. Its pvars: `+0x00` the jet's speed,
//! `+0x04` / `+0x08` the ticks blowing / resting, `+0x0c` the timer, `+0x10` a sound handle (none is started),
//! `+0x14` the first rest.
//!
//! * **0**: the timer the first rest, alpha 0xff, down 3; 1.
//! * **1** (resting): no sound; the timer out (`0x2ef8d8` = L00 `0x2eeab0`): the timer the ticks blowing, 2.
//! * **2** (blowing): a velocity `speed·9 / 60` along its facing (its angles' matrix, `0x2f0538` = L00 `0x2ef6d8`)
//!   plus a jitter of `randf(−1, 1)` on each axis (`0x31c4e8`) × 0.05 of that speed (`$gp+0x2e0` 3, `$gp+0x2e4`
//!   0.05); with 10 ticks or more left, or one time in `10 − timer` (`0x31c450`), a puff at its place (`0x33b730`:
//!   flags 0x40, its owner, scale 0x80, shade 0x1ff), and again a third of the velocity further on; the timer out:
//!   the timer the ticks resting, 1, its sound stopped if it plays (`0x363980`).
//!
//! The second puff's step (the velocity / 3) is the first's: when the first is skipped the game's stack keeps an
//! older value; here the step is taken all the same.

use crate::hero::physics::{add, scale, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x02be;

const SPEED: usize = 0x00;
const BLOW: usize = 0x04;
const REST: usize = 0x08;
const TIMER: usize = 0x0c;
const SOUND: usize = 0x10;
const FIRST_REST: usize = 0x14;

/// `$gp+0x2e0` / `+0x2e4` (L17 `0x1af2d0`).
const SPEED_K: f32 = 3.0;
const JITTER_K: f32 = 0.05;

/// `0x3a2950` (module docs).
pub fn update_02be(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.set_pv_i(TIMER, m.pv_i(FIRST_REST));
            m.alpha = 0xff;
            m.state = 1;
            m.pos[2] -= 3.0;
        }
        1 => {
            m.set_pv_i(SOUND, -1);
            if m.count_i(TIMER) {
                m.set_pv_i(TIMER, m.pv_i(BLOW));
                m.state = 2;
            }
        }
        2 => {
            let g = &mut ctx.game.rng;
            let r: V3 = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
            let pos = m.pos;
            let rows = crate::hero::physics::euler_rows(m.rot);
            let x = m.pv_f(SPEED) * (SPEED_K * 3.0) * 0.016_666_668;
            let jitter = scale(r, x * JITTER_K);
            let vel = add(scale(rows[0], x), jitter);
            let step = scale(vel, 0.33);
            let left = m.pv_i(TIMER);
            let owner = id as i32;
            let mut at = pos;
            for k in 0..2 {
                if 10 <= left || ctx.game.rng.randi(10 - left) == 0 {
                    if k == 1 { at = add(at, step); }
                    if let Some(sys) = ctx.parts.as_deref_mut() {
                        crate::particles::vapour::vapour(sys, &mut ctx.game.rng, at, vel, 0x40, owner, 0x80, 0x1ff);
                    }
                }
            }
            let m = &mut w.mobys[id];
            if m.count_i(TIMER) {
                m.set_pv_i(TIMER, m.pv_i(REST));
                m.state = 1;
                let h = m.pv_i(SOUND);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(SOUND, -1);
            }
        }
        _ => {}
    }
}
