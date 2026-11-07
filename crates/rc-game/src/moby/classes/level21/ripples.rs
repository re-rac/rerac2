//! The Insomniac Museum's rippling pool **0x1198** ([`update_1198`], `0x3852f0`; the decomp's `func_L30_003840E0`):
//! a 24 × 24 height field over a 4 × 4 square (`−2 + k/6` across and along from its place), its own unit
//! (`0x384100..0x3845a8`), drawn by its callback ([`draw`], `0x384b38`) as textured quads. It is the liquid grid's
//! design (crate::water::grid: the same wave step and clock) in code of its own — one patch, no links, the damping
//! always — and its draw (a refraction and a reflection map) is not the grid's or the mesh water's
//! (crate::water::mesh): only the quad (crate::fx::FxQuad) is shared.
//!
//! **The unit** (its words at `$gp−0x480..−0x40c`, its heights at `0x2338e0`: three buffers of 24 × 24, `[row][col]`;
//! here in the moby's own pvars, as the class has none):
//! * the init (`0x384120`): the heights 0 (the places `−2 + k/6` are [`place`]);
//! * the clock (`0x3842e8`): with `f ≥ 0.95` the step (`0x3841c8`: `f` 0.25, every inner cell of the other buffer
//!   `n = 0.25·Σ8 − itself` over the newest, then `n − n/16`; the two swap), the drawn buffer the one before; else
//!   buffer 2 `= other·(1 − f) + newest·f` (`0x384100`), drawn; then `f += 0.25` (from 1.1);
//! * a drop: the cell `(x, y)` of the newest set (from (12, 12), moving (2, 1) a drop, turning back out of 4..20);
//! * a poke (`0x3843f8(x, y, r, h)`): the cells within `r` of `(x, y)` raised to `h·√(2r² − d²)` (and their
//!   mirrors about it set to that).
//!
//! **The update**. **0**: 1, its height raised by `0x1b3438` (0 on the disc), the init, its update distance 0xff.
//! **1**: unless □ is held the clock; with the drop's timer at 0.99 or more, one tick in 200 (`0x322188(200)` = 1) a
//! drop of 0.1 (the timer 0), and the timer + 0.3; ○ held: a poke at (12, 12) of radius 2, 0.2; its draw on the first
//! list (`0x2ee448` = L00 `0x2e7718`).
//!
//! **The draw** (`0x384b38`, with `0x3845a8` for each vertex of the drawn buffer: `p` the place over its own and
//! the camera's way `v`; the normal from the cells along and below, (0, 0, 1) on the last row and column): the
//! refraction's UV — `v` bent through the face (`η = 1 / −1.33`) and carried to 15 under it, `(p + r·t − origin)/4
//! + 0.5` (held to ±4); the reflection's — `v` (made a unit) mirrored in the face (`0x2ef2a8`), `r·2/(2|r|) +
//! 0.5`; the alpha `!clamp(800·(v·n)², 0, 200)`. Every cell twice: FX 0x48 with the refraction's UVs (ALPHA `Cs·FIX`,
//! FIX 0x80: the colour as it is; here the normal blend at 0x80), then FX 0x47 with the reflection's 0.05 higher
//! (ALPHA `(Cs − Cd)·As + Cd` by the vertex alpha), both grey (0x808080), testing Z without writing it (TEST
//! `0x53001`, `0x5360b`).

use crate::fx::FxQuad;
use crate::hero::physics::{add, cross, dot, len, scale, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x1198;

const N: usize = 24;
const BUF: usize = N * N;

/// The unit's words in the pvars (module docs; `$gp−0x480..`).
mod pv {
    pub const NEWEST: usize = 0x00;
    pub const OTHER: usize = 0x04;
    pub const DRAWN: usize = 0x08;
    pub const DRAWN_B: usize = 0x0c;
    pub const CLOCK: usize = 0x10;
    pub const DX: usize = 0x14;
    pub const DY: usize = 0x18;
    pub const X: usize = 0x1c;
    pub const Y: usize = 0x20;
    pub const TIMER: usize = 0x24;
    pub const HEIGHTS: usize = 0x40;
}

/// `0x1b3438`: the height it is raised at its set-up.
const RAISE: f32 = 0.0;
/// `$gp−0x46c`: the clock's step.
const STEP: f32 = 0.25;
/// `$gp−0x468` / `−0x464` (the refraction's `η = a / −b`), `−0x460` (its depth), `−0x45c` (the view's length),
/// `−0x458` (the reflection's scale), `−0x454..−0x44c` (the alpha's scale and bounds), `−0x448` (the second layer's
/// lift).
const ETA: (f32, f32) = (1.0, 1.33);
const DEPTH: f32 = 15.0;
const VIEW: f32 = 1.0;
const REFLECT: f32 = 2.0;
const BRIGHT: (f32, f32, f32) = (800.0, 0.0, 200.0);
const LIFT: f32 = 0.05;
/// `$gp−0x430` / `−0x42c`: the poke's radius and height.
const POKE: (i32, f32) = (2, 0.2);
/// `$gp−0x418..−0x40c`: the drop's height, the timer's step and its threshold.
const DROP: f32 = 0.1;
const DROP_STEP: f32 = 0.3;
const DROP_AT: f32 = 0.99;

/// The place of row `i`, column `j` (`0x2353e0`): `(−2 + i/6, −2 + j/6)`.
fn place(i: usize, j: usize) -> [f32; 2] { [i as f32 * 0.166_666_67 - 2.0, j as f32 * 0.166_666_67 - 2.0] }

fn at(b: i32, i: usize, j: usize) -> usize { pv::HEIGHTS + 4 * (b.clamp(0, 2) as usize * BUF + i * N + j) }

fn h(m: &crate::moby::Moby, b: i32, i: usize, j: usize) -> f32 { m.pv_f(at(b, i, j)) }

/// `0x384120` and the unit's words as the level's data has them.
fn init(m: &mut crate::moby::Moby) {
    for k in 0..3 * BUF { m.set_pv_f(pv::HEIGHTS + 4 * k, 0.0); }
    m.set_pv_i(pv::NEWEST, 0);
    m.set_pv_i(pv::OTHER, 1);
    m.set_pv_i(pv::DRAWN, 2);
    m.set_pv_i(pv::DRAWN_B, 2);
    m.set_pv_f(pv::CLOCK, 1.1);
    m.set_pv_i(pv::DX, 2);
    m.set_pv_i(pv::DY, 1);
    m.set_pv_i(pv::X, 12);
    m.set_pv_i(pv::Y, 12);
    m.set_pv_f(pv::TIMER, 0.0);
}

/// `0x3841c8` (module docs).
fn step(m: &mut crate::moby::Moby) {
    let (src, dst) = (m.pv_i(pv::NEWEST), m.pv_i(pv::OTHER));
    for r in 1..N - 1 {
        for c in 1..N - 1 {
            let s = |i: usize, j: usize| h(m, src, i, j);
            let sum = s(r - 1, c) + s(r + 1, c) + s(r, c - 1) + s(r, c + 1) + s(r - 1, c - 1) + s(r - 1, c + 1) + s(r + 1, c - 1) + s(r + 1, c + 1);
            let v = sum * 0.25 - h(m, dst, r, c);
            m.set_pv_f(at(dst, r, c), v - v * 0.0625);
        }
    }
    m.set_pv_i(pv::NEWEST, dst);
    m.set_pv_i(pv::OTHER, src);
}

/// `0x3842e8` (module docs).
fn clock(m: &mut crate::moby::Moby) {
    let f = m.pv_f(pv::CLOCK);
    if 0.95 <= f {
        m.set_pv_f(pv::CLOCK, STEP);
        step(m);
        let (newest, other) = (m.pv_i(pv::NEWEST), m.pv_i(pv::OTHER));
        m.set_pv_i(pv::DRAWN, other);
        m.set_pv_i(pv::DRAWN_B, newest);
    } else {
        m.set_pv_i(pv::DRAWN, 2);
        m.set_pv_i(pv::DRAWN_B, 2);
        let (newest, other) = (m.pv_i(pv::NEWEST), m.pv_i(pv::OTHER));
        for i in 0..N {
            for j in 0..N {
                let v = h(m, other, i, j) * (1.0 - f) + h(m, newest, i, j) * f;
                m.set_pv_f(at(2, i, j), v);
            }
        }
    }
    let f = m.pv_f(pv::CLOCK) + STEP;
    m.set_pv_f(pv::CLOCK, f);
}

/// `0x3843f8(x, y, r, h)` (module docs).
fn poke(m: &mut crate::moby::Moby, x: i32, y: i32, r: f32, height: f32) {
    let extent = r as i32;
    let rr = 2.0 * (r * r);
    let b = m.pv_i(pv::NEWEST);
    let cell = |i: i32, j: i32| -> Option<usize> { (0..N as i32).contains(&i).then_some(())?; (0..N as i32).contains(&j).then(|| at(b, i as usize, j as usize)) };
    for j in y - extent..=y {
        for i in x - extent..=x {
            let d = (((x - i) * (x - i) + (y - j) * (y - j)) as f32).sqrt();
            let mut v = height * (rr - d * d).abs().sqrt();
            let Some(k) = cell(i, j) else { continue };
            v = v.max(m.pv_f(k));
            for (a, b) in [(i, j), (2 * x - i, j), (i, 2 * y - j), (2 * x - i, 2 * y - j)] {
                if let Some(k) = cell(a, b) { m.set_pv_f(k, v); }
            }
        }
    }
}

/// 0x1198 (module docs).
pub fn update_1198(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.state = 1;
            m.pos[2] += RAISE;
            init(m);
            m.update_dist = 0xff;
        }
        1 => {
            if ctx.hero.items.held & button::SQUARE == 0 { clock(m); }
            let timer = m.pv_f(pv::TIMER);
            if DROP_AT <= timer && ctx.game.rng.randi(200) == 1 {
                let (mut dx, mut dy) = (m.pv_i(pv::DX), m.pv_i(pv::DY));
                let x = m.pv_i(pv::X) + dx;
                let y = m.pv_i(pv::Y) + dy;
                m.set_pv_f(pv::TIMER, 0.0);
                if 16 < (x - 4) as u32 { dx = -dx; }
                if 16 < (y - 4) as u32 { dy = -dy; }
                m.set_pv_i(pv::X, x);
                m.set_pv_i(pv::Y, y);
                m.set_pv_i(pv::DX, dx);
                m.set_pv_i(pv::DY, dy);
                let b = m.pv_i(pv::NEWEST);
                if (0..N as i32).contains(&x) && (0..N as i32).contains(&y) { m.set_pv_f(at(b, x as usize, y as usize), DROP); }
            }
            let t = m.pv_f(pv::TIMER) + DROP_STEP;
            m.set_pv_f(pv::TIMER, t);
            if ctx.hero.items.held & button::CIRCLE != 0 { poke(m, 12, 12, POKE.0 as f32, POKE.1); }
            w.register_draw_first(id);
        }
        _ => {}
    }
}

/// `0x2ef2a8`: `v` mirrored in the face of normal `n` when it runs into it.
fn reflect(v: V3, n: V3) -> V3 {
    let d = dot(v, n);
    if d < 0.0 { sub(v, scale(n, 2.0 * d)) } else { v }
}

/// `0x384b38` (module docs): the first list's callback.
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let o = m.pos;
    let b = m.pv_i(pv::DRAWN);
    let point = |i: usize, j: usize| -> V3 { let p = place(i, j); [p[0], p[1], h(m, b, i, j)] };
    // `0x3845a8`: each vertex's two UVs and alpha.
    let mut refr = [[[0.0f32; 2]; N]; N];
    let mut refl = [[[0.0f32; 2]; N]; N];
    let mut alpha = [[0u8; N]; N];
    let eta = ETA.0 / -ETA.1;
    for i in 0..N {
        for j in 0..N {
            let mut p = point(i, j);
            let n = if i < N - 1 && j < N - 1 {
                let (q, s) = (point(i, j + 1), point(i + 1, j));
                set_len(cross(sub(p, q), sub(q, s)), 1.0)
            } else {
                [0.0, 0.0, 1.0]
            };
            let height = p[2];
            p = add(p, o);
            let mut v = sub(cam, p);
            let along = scale(set_len(v, VIEW), eta);
            let d = dot(n, v);
            let t = (1.0 - eta * eta * (1.0 - d * d)).abs().sqrt();
            let r = sub(scale(n, eta * d - t), along);
            let k = (DEPTH + height) / r[2];
            refr[i][j] = [((p[0] + r[0] * k - o[0]) * 0.25 + 0.5).clamp(-4.0, 4.0), ((p[1] + r[1] * k - o[1]) * 0.25 + 0.5).clamp(-4.0, 4.0)];
            v = set_len(v, VIEW);
            let r = reflect(v, scale(n, -1.0));
            let k = 1.0 / (len(r) * 2.0);
            refl[i][j] = [r[0] * REFLECT * k + 0.5, r[1] * REFLECT * k + 0.5];
            let d = dot(v, n);
            let lit = (BRIGHT.0 * d * d / dot(v, v)).min(BRIGHT.2).max(BRIGHT.1);
            alpha[i][j] = !(lit as i32 as u8);
        }
    }
    for (fx, uv, lift, fixed) in [(0x48, &refr, 0.0, true), (0x47, &refl, LIFT, false)] {
        for r in 0..N - 1 {
            for c in 0..N - 1 {
                let k = [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)];
                let corners = k.map(|(i, j)| { let p = point(i, j); [p[0] + o[0], p[1] + o[1], p[2] + o[2] + lift] });
                let st = k.map(|(i, j)| uv[i][j]);
                let rgba = k.map(|(i, j)| if fixed { 0x8080_8080 } else { (alpha[i][j] as u32) << 24 | 0x80_8080 });
                out.push(FxQuad { fx, additive: false, corners, st, rgba });
            }
        }
    }
}
