//! Level 14's flickering screen **0x12b9** (`0x4337b0`, 2 placed; the decomp's `func_L14_004321D0`): a quad of
//! static in front of it, now and then showing a picture. Its pvars: `+0x00` / `+0x04` the shortest and longest
//! showing, `+0x08` the static's window (a fraction of the texture), `+0x0c` the timer, `+0x10` its facing (in its
//! frame), `+0x1c` the push along it, `+0x20` / `+0x24` the half width and height, `+0x28` / `+0x2c` the two static
//! textures, `+0x30..+0x38` the picture's three frames, `+0x3c` the ticks a frame, `+0x40` its loop sound.
//!
//! * **0**: no sound, the five textures + 0x3f (FX indices), 10 ticks a frame; 1.
//! * **1**: 2 or 3 at random (`0x323360(2, 3)`).
//! * **2** (static) / **3** (the picture): on the first tick the sound stopped (`0x36aaa0`) and the timer
//!   `rand_range(shortest, longest)`; its class sound 0 (2) or 1 (3) kept looping (`0x36ada0(k, 4)`). The timer out:
//!   1.
//! * **Every tick**: its draw ([`draw`], `0x433a30`) registered.

use crate::hero::physics::{add, cross, scale, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12b9;

const SOUND: usize = 0x40;

/// 0x12b9 (module docs).
pub fn update_12b9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(SOUND, -1);
            for off in [0x28, 0x34, 0x2c, 0x30, 0x38] { m.set_pv_i(off, m.pv_i(off) + 0x3f); }
            m.set_pv_i(0x3c, 10);
            m.set_state(1, -1);
        }
        1 => {
            let to = ctx.game.rng.rand_range(2, 3) as u8;
            w.mobys[id].set_state(to, -1);
        }
        s @ (2 | 3) => {
            if w.mobys[id].first() {
                let h = w.mobys[id].pv_i(SOUND);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                let (lo, hi) = (w.mobys[id].pv_i(0), w.mobys[id].pv_i(4));
                let t = ctx.game.rng.rand_range(lo, hi);
                let m = &mut w.mobys[id];
                m.set_pv_i(SOUND, -1);
                m.set_pv_i(0xc, t);
            }
            let h = w.mobys[id].pv_i(SOUND);
            if h == -1 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], (s - 2) as i32, 4);
                w.mobys[id].set_pv_i(SOUND, h);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(0xc);
            let out = countdown(&mut t);
            m.set_pv_i(0xc, t);
            if out { m.set_state(1, -1); }
        }
        _ => {}
    }
    w.register_draw(id);
}

fn unit(v: V3) -> V3 {
    let l = crate::hero::physics::len(v);
    if 0.0 < l { scale(v, 1.0 / l) } else { v }
}

/// `0x433a30`: its facing through its rows (`0x2f6ee0`) makes a frame (`0x392250`: row 0 the facing, row 1 it × up,
/// row 2 that × row 0, at its place) pushed `+0x1c` along row 0 (`0x392fb0`); the quad (0, ∓1, ±1) scaled by
/// (`+0x20`, `+0x24`) in it (`0x392318`). Below state 3 one of the two static textures at random (`0x323360(0, 1)`)
/// over a random window `+0x08` across (`randf(0, 1 − w)` for s, then t); in 3 the picture's frame
/// `(timer / +0x3c) mod 3` whole. Colour 0x80808080, ALPHA row 2 (`Cs`: the texture as it is, `0x392d98(_, fx, 2)`),
/// under TEST 0x53001 (z GEQUAL, no z write). Here the quad takes the normal blend (ALPHA 0x44 at vertex alpha 0x80,
/// the same where the texture is solid).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let f = m.pv_v(0x10);
    let dir = unit(std::array::from_fn(|k| f[0] * m.rows[0][k] + f[1] * m.rows[1][k] + f[2] * m.rows[2][k]));
    let r1 = unit(cross(dir, [0.0, 0.0, 1.0]));
    let r2 = cross(r1, dir);
    let at = add(m.pos, scale(dir, m.pv_f(0x1c)));
    let (y, z) = (m.pv_f(0x20), m.pv_f(0x24));
    let corner = |a: f32, b: f32| add(at, add(scale(r1, a * y), scale(r2, b * z)));
    let corners = [corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, 1.0), corner(1.0, -1.0)];
    let (fx, st) = if m.state < 3 {
        let fx = m.pv_i(0x28 + rng.rand_range(0, 1) as usize * 4);
        let k = m.pv_f(8);
        let s = rng.randf(0.0, 1.0 - k);
        let t = rng.randf(0.0, 1.0 - k);
        (fx, [[s, t], [s, t + k], [s + k, t], [s + k, t + k]])
    } else {
        let n = m.pv_i(0x3c);
        if n == 0 { return; }
        (m.pv_i(0x30 + ((m.pv_i(0xc) / n) % 3) as usize * 4), [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]])
    };
    out.push(crate::fx::FxQuad { fx: fx.max(0) as usize, additive: false, corners, st, rgba: [0x8080_8080; 4] });
}
