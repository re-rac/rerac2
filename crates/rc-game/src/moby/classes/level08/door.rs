//! The liquid lid **0x11fe** (L08 `0x430568`, 2 placed; L19 `0x422c70`, 15 placed; the decomp's
//! `func_L08_0042EF00`, its draw `func_L08_0042E678` = L08 `0x42fce0` / L19 `0x4223e8`; the same code and data on
//! both levels): a square of layered, scrolling quads that the game sends as its own GS packets, rising from its low
//! height to its high one once opened. Its model is not drawn. Its pvars: `+0x00` / `+0x04` the high and low
//! heights, `+0x08` the phase, `+0x0c` the rate (degrees a second), `+0x10` its texture (an FX index − 0x3f),
//! `+0x14` its colour, `+0x18` the sheen's texture (−1 none), `+0x1c` the sheen's colour, `+0x20` the frame grid's
//! colour (its alpha byte `+0x23` the grid's switch).
//!
//! Its sub-state (`+0x95`):
//! * **0**: the rate × π / 10800 (`0x3998..`: degrees a second to radians a tick), phase 0, not drawn (`+0x31` 0),
//!   mode | 1; its mission (`+0xb0`) done: phase π, 3; else 1.
//! * **1**: its mission done or hooked (`0x3393c0` = L00 `0x32a838`): on level 19 the level sound 18 (`0x36ed30` =
//!   L00 `0x3607e8(18, 0, moby)`); 2.
//! * **2**: its phase at π or more, or below −3: phase π, 3.
//! * **Every tick**: its height `low + h·cos(phase − π) + h` (`h` half the high less the low; `0x2fba78` /
//!   `0x2fb018` = L00 `0x2efea8` / `0x2ef448`). The phase moves by the rate (`0x2fba30` = L00 `0x2efe60`) in
//!   sub-state 2 below π (and from −3), when Ratchet's body (`0x18c0d4`) is 4, or when his movement group is neither
//!   20 nor 25 and his state none of 0x39 / 0x78 / 0x79. Its sphere (its place, radius `scale·11.32 / class scale`)
//!   in view within its draw distance (`0x2fbc50` = L00 `0x2f0080`): its draw ([`draw`]) on the first list
//!   (`0x2f3488` = L00 `0x2e7718`), the moby kept in the frame's list of lids (`0x2b2540`, up to the count
//!   `$gp+0x2e8c`, cleared when the frame counter `0x1b2508` moves on).
//!
//! Each frame (the first lid's tick of it, `$gp+0x2e80` the frame it was done for): the lids' height bob
//! `$gp+0x2e84 = 0.1·sin(wrap(frame · 0.01))` (`0x32e460` = L00 `0x320440`) and their texture's scroll
//! `s = t = fmod(frame · 0.001, 1)` (`0x2fbb98` = L00 `0x2effc8`; the quads span `s..s + 2`).
//!
//! Not ported: the frame grid of its draw — with `+0x23` set, a grid of 2 × 2 cells over its square at its height
//! ± 0.1 toward the camera, wavering by `sin(wrap(frame · 0.06))` (0.08 in x, 0.06 in y), each cell the frame's copy
//! (`0x2ea6c8(−1)`) at its corners' screen places (`0x2f2e18`) in `+0x20`'s colour, ALPHA `Cs·FIX(0x80)`: `+0x23`
//! is 0 in every placed lid on both levels.

use crate::fx::FxQuad;
use crate::hero::physics::{sub, unit, V3};
use crate::moby::{Ctx, MobyWorld};
use std::f32::consts::PI;

pub const CLASS: i16 = 0x11fe;

const HIGH: usize = 0x00;
const LOW: usize = 0x04;
const PHASE: usize = 0x08;
const RATE: usize = 0x0c;
const TEX: usize = 0x10;
const COLOUR: usize = 0x14;
const SHEEN_TEX: usize = 0x18;
const SHEEN_COLOUR: usize = 0x1c;

/// `$gp+0x2e60..0x2e68` (L08 `0x1b1e50`): the scroll's rate, the bob's rate and its reach.
const SCROLL_RATE: f32 = f32::from_bits(0x3a83_126f);
const BOB_RATE: f32 = f32::from_bits(0x3c23_d70a);
const BOB: f32 = f32::from_bits(0x3dcc_cccd);
/// The lids' heights off the bobbing height, toward the camera: the two crossed layers, the plain one.
const LAYERS: [f32; 3] = [0.06, 0.04, 0.0];

/// `0x430568` (module docs).
pub fn update_11fe(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let done = crate::moby::director::mission_is_done(ctx.game, w.mobys[id].mission as i32);
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(RATE, m.pv_f(RATE) * 0.000_290_888_25);
            m.set_pv_f(PHASE, 0.0);
            m.drawn = 0;
            m.mode |= 1;
            if done {
                m.set_pv_f(PHASE, PI);
                m.sub_state = 3;
            } else {
                m.sub_state = 1;
            }
        }
        1 => {
            if done || w.mobys[id].once & 4 != 0 {
                if ctx.game.level == 19 { ctx.host.play_level_sound(Some(&w.mobys[id]), 18, 0); }
                w.mobys[id].sub_state = 2;
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let p = m.pv_f(PHASE);
            if PI <= p || p < -3.0 {
                m.set_pv_f(PHASE, PI);
                m.sub_state = 3;
            }
        }
        _ => {}
    }
    let hero = (ctx.hero.mode, ctx.hero.group, ctx.hero.state);
    let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    let p = m.pv_f(PHASE);
    let moving = m.sub_state == 2 && p < PI && -3.0 <= p;
    let rate = if moving { m.pv_f(RATE) } else { 0.0 };
    let h = (m.pv_f(HIGH) - m.pv_f(LOW)) * 0.5;
    m.pos[2] = m.pv_f(LOW) + (crate::hero::physics::sub_rot(p, PI).cos() * h + h);
    if hero.0 == 4 || (hero.1 != 20 && hero.1 != 25 && !matches!(hero.2, 0x39 | 0x78 | 0x79)) {
        m.set_pv_f(PHASE, crate::hero::physics::add_rot(p, rate));
    }
    let r = m.scale * 11.32 / class_scale;
    let sphere = [m.pos[0], m.pos[1], m.pos[2], r];
    if !ctx.view.is_some_and(|v| v.culled(m.draw_dist as f32, sphere)) { w.register_draw_first(id); }
}

/// `0x320440`: an angle wrapped to [−π, π).
fn wrap(a: f32) -> f32 { ((a + PI) * 0.159_154_94).rem_euclid(1.0) * std::f32::consts::TAU - PI }

/// `0x42fce0` (a first-list callback; module docs): the last lid of the frame's list draws every lid's quads, each
/// layer for all of them before the next. Each lid's square (half-size `scale·8 / class scale`) at its height plus
/// the bob, pushed toward the camera (up when the camera is above it, down below): the lid in its colour with the
/// list's last lid's texture, `s, t` across `scroll..scroll + 2` — at 0.06 (x mirrored), at 0.04 (x mirrored
/// again) and at 0 (both mirrored) — each with ALPHA `(Cs − Cd)·FIX + Cd`, FIX 0x28 (`$gp+0x2e18..`), its texture
/// its own (`+0x10` + 0x3f) in the first layer and the list's last lid's in the other two (the packet keeps it); then, when the last lid has a sheen (`+0x18` ≠ −1), each one's
/// sheen at its height: additive (ALPHA 0x48), FX `+0x18` + 0x3f in `+0x1c`, each corner's `s, t` the x and y of the
/// way from the camera (`0x1b9b00`) to it mirrored in the up plane (`0x2fae78` with (0, 0, 1)) and made unit
/// (`0x2fad08`). Here the lids take the normal blend at their colours' alpha (0x28 in every placed lid: the FIX).
pub fn draw(w: &MobyWorld, id: usize, frame: i32, cam: V3, out: &mut Vec<FxQuad>) {
    let lids: Vec<usize> = w.draws_first.iter().copied().filter(|&k| w.mobys.get(k).is_some_and(|m| m.o_class == CLASS && m.state < 0x80)).collect();
    if lids.last() != Some(&id) { return; }
    let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    let f = frame as f32;
    let bob = BOB * wrap(f * BOB_RATE).sin();
    let s = (f * SCROLL_RATE) % 1.0;
    let st = [[s, s], [s + 2.0, s], [s, s + 2.0], [s + 2.0, s + 2.0]];
    let square = |k: usize, push: f32, flip: [f32; 2]| {
        let m = &w.mobys[k];
        let r = m.scale * 8.0 / class_scale;
        let sign = if m.pos[2] < cam[2] { -1.0 } else { 1.0 };
        let z = m.pos[2] + bob + sign * push;
        let c = |x: f32, y: f32| [m.pos[0] + x * r * flip[0], m.pos[1] + y * r * flip[1], z];
        [c(-1.0, -1.0), c(1.0, -1.0), c(-1.0, 1.0), c(1.0, 1.0)]
    };
    let tex = w.mobys[*lids.last().unwrap_or(&id)].pv_i(TEX) + 0x3f;
    for (n, (layer, flip)) in LAYERS.iter().zip([[1.0, 1.0], [-1.0, 1.0], [-1.0, -1.0]]).enumerate() {
        for &k in &lids {
            let rgba = w.mobys[k].pv_i(COLOUR) as u32;
            // The first layer sets each lid's own texture into the packet; the others keep the last one's.
            let fx = if n == 0 { w.mobys[k].pv_i(TEX) + 0x3f } else { tex };
            out.push(FxQuad { fx: fx.max(0) as usize, additive: false, corners: square(k, *layer, flip), st, rgba: [rgba; 4] });
        }
    }
    if w.mobys[id].pv_i(SHEEN_TEX) == -1 { return; }
    for &k in &lids {
        let m = &w.mobys[k];
        let corners = square(k, 0.0, [1.0, 1.0]);
        let st = corners.map(|p| {
            let d = sub(p, cam);
            let v = unit([d[0], d[1], -d[2]]);
            [v[0], v[1]]
        });
        let rgba = m.pv_i(SHEEN_COLOUR) as u32;
        out.push(FxQuad { fx: (m.pv_i(SHEEN_TEX) + 0x3f).max(0) as usize, additive: true, corners, st, rgba: [rgba; 4] });
    }
}
