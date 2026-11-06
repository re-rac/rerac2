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
//! **The curtain** ([`draw`], L13 `0x4210b8`, L16 `0x41aa00`): seven packets (L13 `$gp+0x2a00..0x2b80`, L16
//! `$gp+0x2c60..`, the same values) through the spin ([`crate::fx::spin`]: a half turn, 2 steps: each packet drawn
//! twice as a flat sheet ±1.2 along its second row), then again 7 higher. Its frame is its rows with the first 4 long
//! (turned to the camera's side; the sheet has no breadth along it) and the second 10, at its place.
//! * The main sheet 1.5..5.5 (FX 0x19, ST (½, ½)), `sin([8]) · ½ + ½` of the way from 0x60403020 to 0x40403020.
//! * Two bars of FX 0x29 (ST the unit square, scrolled by `[3]` one way on the lower and the other on the upper):
//!   2.3..2.7 and 3.6..4.0, 0x40804040 → 0x60804040 by the pulse; the same pair drawn apart by 3·`[1]` and by 3·`[2]`,
//!   0x60804040 fading out as the band's phase goes past 0.2 (in before it).
//! * All alphas × the fade `[0]` while it is under 1.

use crate::fx::spin::{spin, Packet};
use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, approach, dot, scale, set_len, sub, V3};
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
        // `0x2f21f8(0x4210b8, moby)`: the curtain on the frame's first list.
        w.register_draw_first(id);
    }
}

/// The curtain's sheets (`$gp+0x2a70` / `+0x2ab0` / `+0x2af0`: two corners at each height, 1.2 along the second row).
const SHEET: (f32, f32) = (1.5, 5.5);
const LOW: (f32, f32) = (2.3, 2.7);
const HIGH: (f32, f32) = (3.6, 4.0);
/// The second curtain's lift, the bands' spread and the turn's divisor of π (`$gp+0x2b70..0x2b78`); the packets
/// and steps (`$gp+0x2b7c`, `+0x2b80`) are 7 and 2.
const LIFT: f32 = 7.0;
const SPREAD: f32 = 3.0;
const DIVISOR: f32 = 1.0;
/// The FX textures (`$gp+0x2a00`, `0x1b19f4`) and the colours (`$gp+0x2a50`, `0x1b1a44..0x1b1a4c`).
const FX: [usize; 2] = [0x19, 0x29];
const MAIN: [u32; 2] = [0x6040_3020, 0x4040_3020];
const TRIM: [u32; 2] = [0x4080_4040, 0x6080_4040];

/// `0x4210b8`, the curtain (module docs).
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    use crate::hud::lerp_rgba;
    let m = &w.mobys[id];
    let mut r0 = set_len(m.rows[0], 4.0);
    if dot(sub(cam, m.pos), m.rows[0]) < 0.0 { r0 = scale(r0, -1.0); }
    let rows = [r0, set_len(m.rows[1], 10.0), m.rows[2]];
    let pulse = m.pv_f(0x20).sin() * 0.5 + 0.5;
    let edge = |t: f32| { let t = if 0.2 <= t { t } else { 1.0 - t * 5.0 }; lerp_rgba(t, TRIM[1], TRIM[1] & 0xff_ffff) };
    let (s1, s2) = (m.pv_f(4), m.pv_f(8));
    let mut c = [lerp_rgba(pulse, MAIN[0], MAIN[1]), lerp_rgba(pulse, TRIM[0], TRIM[1]), edge(s1), edge(s2)];
    let fade = m.pv_f(0);
    if fade < 1.0 { c = c.map(|x| lerp_rgba(fade, x & 0xff_ffff, x)); }
    let scroll = m.pv_f(0xc);
    let bar = |d: f32| [[d, 0.0], [d, 1.0], [1.0 + d, 0.0], [1.0 + d, 1.0]];
    let (up, down) = (bar(scroll), bar(-scroll));
    // (FX, colour, ST, sheet, its rise)
    let packets: [(usize, u32, [[f32; 2]; 4], (f32, f32), f32); 7] = [
        (FX[0], c[0], [[0.5, 0.5]; 4], SHEET, 0.0),
        (FX[1], c[1], up, LOW, 0.0),
        (FX[1], c[1], down, HIGH, 0.0),
        (FX[1], c[2], up, LOW, -s1 * SPREAD),
        (FX[1], c[2], down, HIGH, s1 * SPREAD),
        (FX[1], c[3], up, LOW, -s2 * SPREAD),
        (FX[1], c[3], down, HIGH, s2 * SPREAD),
    ];
    let turn = crate::moby::attach::rows_of([0.0, 0.0, -std::f32::consts::PI / DIVISOR]);
    for lift in [0.0, LIFT] {
        let mut ps = packets.map(|(fx, rgba, st, (lo, hi), rise)| {
            let (lo, hi) = (lo + lift + rise, hi + lift + rise);
            Packet::new(fx, rgba, st, [[0.0, 1.2, lo], [0.0, 1.2, hi], [0.0, 1.2, lo], [0.0, 1.2, hi]])
        });
        spin(out, &mut ps, &turn, 2, &rows, m.pos);
    }
}
