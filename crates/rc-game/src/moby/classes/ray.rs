//! The ray, class **0x13ba** (G-REN-019): a shaft of light between two points drawn as a row of camera-facing
//! squares, owned by another moby. Its update is one function on six levels (L00 `0x41aad0`, L03 `0x42c920`, L06
//! `0x46bcc8`, L09 `0x429810`, L14 `0x4391d8`, L18 `0x448168`; the decomp's `func_L00_004195C8`), its draw L06
//! `0x46bd60` (`func_L00_00419668`); levels 3, 6 and 9 make it ([`spawn`], [`ends`], [`colour`]).
//!
//! **pvars** (`+0x68`): `+0x00` the start (its `w` the squares' size there), `+0x10` the end (`w` the size there),
//! `+0x20` the squares, `+0x2c` the alpha, `+0x30` the middle and `+0x3c` the length (the view test's sphere),
//! `+0x40` the colour, `+0x44` the owner, `+0x48` the effect texture.
//!
//! **Made** ([`spawn`], L06 `0x46c178(from, to, colour, owner, squares, sized)`): mode 0x100, its update and draw
//! distances the owner's (`+0x30` / `+0x32`; 0x96 without one), state 0, its squares and owner, its ends and colour;
//! not `sized`: its length 0 (always drawn). Its texture by the level (`0x1a79f8`): 0x48 on 0 and 9, 0x3f on 3, 0x44 on
//! 6, 0x43 on 18. **The ends** ([`ends`], `0x46c2e8`): the start, the end, the middle (half way, `w`
//! too) as its place, the length. **The colour** ([`colour`], `0x46c388`): its alpha the word's top byte (signed),
//! its colour the rest.
//!
//! **The update** ([`update`], `0x46bcc8`): the owner dead (state ≥ 0x80): gone. Else with its length 0, or its
//! sphere (the middle, the length) in view within its draw distance (`0x2f0080`), its draw ([`draw`], the frame's
//! second list, `0x2e7920`).
//!
//! **The draw** ([`draw`], `0x46bd60`): its texture additive (`0x2ecef8(fx, 1, 0x80)`); a square of the camera's left
//! and up rows (`0x1b8f80` / `0x1b8f90`; the corners −l−u, u−l, l−u, l+u) of the start's size, 0.75 toward the camera
//! from the start, stepped `squares` times by a `1/(squares − 1)` of the way to the end and of the size's change, each
//! step drawn (after its move) in the colour at `alpha · (squares − i) / squares` (truncated, held to 0..255; none
//! at 0).

use crate::hero::physics::{add, dist, scale, set_len, sub, V3};
use crate::moby::{mode, Ctx, MobyWorld};

/// The class.
pub const CLASS: i16 = 0x13ba;

const FROM: usize = 0x00;
const TO: usize = 0x10;
const SQUARES: usize = 0x20;
const ALPHA: usize = 0x2c;
const MID: usize = 0x30;
const LENGTH: usize = 0x3c;
const RGB: usize = 0x40;
const OWNER: usize = 0x44;
const FX: usize = 0x48;

fn set_v4(m: &mut crate::moby::Moby, off: usize, v: [f32; 4]) { for (k, x) in v.iter().enumerate() { m.set_pv_f(off + 4 * k, *x); } }
fn pv_v4(m: &crate::moby::Moby, off: usize) -> [f32; 4] { [m.pv_f(off), m.pv_f(off + 4), m.pv_f(off + 8), m.pv_f(off + 12)] }

/// L06 `0x46c178(from, to, colour, owner, squares, sized)` (module docs): None when the level has no such class.
pub fn spawn(w: &mut MobyWorld, level: i32, from: [f32; 4], to: [f32; 4], rgba: u32, owner: Option<usize>, squares: i32, sized: bool) -> Option<usize> {
    w.class(CLASS)?;
    let (ud, dd) = owner.and_then(|o| w.mobys.get(o)).map_or((0x96, 0x96), |o| (o.update_dist, o.draw_dist));
    let id = w.create(CLASS);
    let m = &mut w.mobys[id];
    m.update = Some(update);
    m.mode = (m.mode & !mode::NO_UPDATE) | mode::KEEP_ROWS;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = ud;
    m.draw_dist = dd;
    m.drawn = 1;
    m.state = 0;
    m.set_pv_i(SQUARES, squares);
    m.set_pv_i(OWNER, owner.map_or(0, |o| o as i32));
    ends(w, id, from, to);
    colour(w, id, rgba);
    let m = &mut w.mobys[id];
    if !sized { m.set_pv_f(LENGTH, 0.0); }
    let fx = match level { 0 | 9 => Some(0x48), 3 => Some(0x3f), 6 => Some(0x44), 0x12 => Some(0x43), _ => None };
    if let Some(fx) = fx { m.set_pv_i(FX, fx); }
    w.changed = true;
    Some(id)
}

/// L06 `0x46c2e8(ray, from, to)` (module docs).
pub fn ends(w: &mut MobyWorld, id: usize, from: [f32; 4], to: [f32; 4]) {
    let Some(m) = w.mobys.get_mut(id) else { return };
    set_v4(m, FROM, from);
    set_v4(m, TO, to);
    let mid: [f32; 4] = std::array::from_fn(|k| from[k] + (to[k] - from[k]) * 0.5);
    set_v4(m, MID, mid);
    m.pos = [mid[0], mid[1], mid[2]];
    m.set_pv_f(LENGTH, dist([from[0], from[1], from[2]], [to[0], to[1], to[2]]));
}

/// L06 `0x46c388(ray, rgba)` (module docs).
pub fn colour(w: &mut MobyWorld, id: usize, rgba: u32) {
    let Some(m) = w.mobys.get_mut(id) else { return };
    m.set_pv_i(ALPHA, (rgba as i32) >> 24);
    m.set_pv_i(RGB, (rgba & 0xff_ffff) as i32);
}

/// L06 `0x46bcc8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let owner = m.pv_link(OWNER).and_then(|o| w.mobys.get(o));
    if owner.is_some_and(|o| 0x80 <= o.state) { return w.delete(id); }
    let length = m.pv_f(LENGTH);
    let mid = m.pv_v(MID);
    if length == 0.0 || !ctx.view.is_some_and(|v| v.culled(m.draw_dist as f32, [mid[0], mid[1], mid[2], length])) {
        w.register_draw(id);
    }
}

/// L06 `0x46bd60` (module docs). `rows`: the camera's (forward, left, up).
pub fn draw(w: &MobyWorld, id: usize, cam: V3, rows: [V3; 3], out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let n = m.pv_i(SQUARES);
    let (from, to) = (pv_v4(m, FROM), pv_v4(m, TO));
    let (l, u) = (rows[1], rows[2]);
    let corners = [sub(scale(l, -1.0), u), sub(u, l), sub(l, u), add(l, u)];
    let start = [from[0], from[1], from[2]];
    let centre = add(start, set_len(sub(cam, start), 0.75));
    let mut v = corners.map(|c| add(scale(c, from[3]), centre));
    let div = (n - 1) as f32;
    let grow = (to[3] - from[3]) / div;
    let step = scale(sub([to[0], to[1], to[2]], start), 1.0 / div);
    let steps = corners.map(|c| add(scale(c, grow), step));
    let (alpha, rgb) = (m.pv_i(ALPHA) as f32, m.pv_i(RGB) as u32);
    let fx = m.pv_i(FX).max(0) as usize;
    for i in 0..n.max(0) {
        for k in 0..4 { v[k] = add(v[k], steps[k]); }
        let a = ((alpha * (n - i) as f32 / n as f32) as i32).clamp(0, 255);
        if a <= 0 { continue; }
        let c = rgb.wrapping_add((a as u32) << 24);
        out.push(crate::fx::FxQuad { fx, additive: true, corners: v, st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [c; 4] });
    }
}
