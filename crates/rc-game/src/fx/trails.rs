//! The strips of quads that follow a line of points: a shot's bolt along its way, a trail through a history of
//! places, a crackling strand through a path. Each is the engine's or a level's draw, ported once here for every
//! class that calls it (their draw callbacks, crate::fx).
//!
//! * [`bolt`] (L16 `0x406c40` / `0x415460`, L30 `0x404dc0` / `0x40ee50`): a shot drawn along its velocity. Users:
//!   L16's gunship shot 0x0ebd and drone shot 0x0f7a ([`shot_bolt`], their callbacks `0x406ad0` / `0x4152f0`); the
//!   lobbers' glob 0x0ea5 (L17 `0x40dad8` from `0x40d968`, L12 `0x4097b0`, L13 `0x40e558`, L16 `0x3fd028`;
//!   crate::moby::classes::level17::lobber::draw_0ea5).
//! * [`history`] (`0x2ec810`, L26 `0x2ece08`): a trail through the last places and turns of a moby. User: L26's
//!   mote 0x0c00.
//! * [`strand`] (L11 `0x46dc98`, L14 `0x423f60`): a scrolling core and a camera-facing halo through a path of
//!   points. Users: L11's fence 0x0ee0, L14's fence 0x0f80.
//!
//! Not ported: the quads' `+0x70` word (5 for the bolt's flares and the trail, 0 elsewhere) and `0x2eb998`'s last
//! argument (1 for the trail), which this port's quads have no use for; the GS test register the strand sets after it
//! (`0x37c948(0x47, 0x5360b)`).

use super::FxQuad;
use crate::hero::physics::{add, add_rot, cross, len, scale, set_len, sub, V3};
use crate::moby::MobyWorld;
use crate::quat::Q;

/// A bolt's look (its level's words; module docs).
#[derive(Clone, Copy, Debug)]
pub struct Bolt {
    /// The head's and the flares' FX textures.
    pub fx: [usize; 2],
    /// The head's colours at the shot and at its back (the tail fades the back's to `0x801010`, alpha 0).
    pub head: [u32; 2],
    /// The head's length back along the way, its half-width and where it starts along the way.
    pub length: f32,
    pub width: f32,
    pub from: f32,
    /// The flares' texture turn a call, in degrees.
    pub turn: f32,
    pub flares: [Flare; 4],
    /// The flares' colour at their backs.
    pub flare_end: u32,
    /// The glows' FX texture, size and colour (the second: 0.4 times the size, `0x80808080`).
    pub glow: (usize, f32, u32),
}

/// One of a [`Bolt`]'s flares: its colour at the front, half-width, start along the way and half-length back.
#[derive(Clone, Copy, Debug)]
pub struct Flare {
    pub rgba: u32,
    pub width: f32,
    pub from: f32,
    pub length: f32,
}

const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

/// `0x406c40(…, moby)` (its other arguments unused): the bolt of a shot at `pos` flying along `dir`, additive.
/// * The head: FX `fx[0]`, from `from` along the way to `length` back, `width` to either side across `back × (pos −
///   camera)`, `head[0]` at the front and `head[1]` at the back; then the tail, as long again behind it, from
///   `head[1]` to `0x801010`. Their s runs `7 − scroll .. 8 − scroll` (`s − scroll`, 7 added when at most 7), the
///   scroll on by 0.1 a call (4 taken off at 4).
/// * The flares: FX `fx[1]`, each from its `from` along the way to twice its `length` back, its `width` across, its
///   colour to `flare_end`; their ST the unit square turned by `spin` about its middle, `spin` on by `turn`° a call
///   (wrapped).
/// * Two glows at the shot (`0x2edf78`, crate::fx::sprite; 0.08 toward the camera, each a random roll): `glow`, and
///   0.4 times its size in `0x80808080`.
#[allow(clippy::too_many_arguments)]
pub fn bolt(out: &mut Vec<FxQuad>, rng: &mut crate::rng::Rng, look: &Bolt, pos: V3, dir: V3, scroll: &mut f32, spin: &mut f32, cam: V3) {
    let st = ST.map(|[s, t]| {
        let s = s - *scroll;
        [if s <= 7.0 { s + 7.0 } else { s }, t]
    });
    let s = *scroll + 0.1;
    *scroll = if 4.0 <= s { s - 4.0 } else { s };
    let rel = sub(pos, cam);
    let axis = set_len(dir, -look.length);
    let tip = add(pos, axis);
    // `0x2eef88(out, a, b)`: b × a.
    let mut side = set_len(cross(axis, rel), look.width);
    let base = add(pos, set_len(dir, look.from));
    let c = [add(base, side), sub(base, side), add(tip, side), sub(tip, side)];
    let [h0, h1] = look.head;
    out.push(FxQuad { fx: look.fx[0], additive: true, corners: c, st, rgba: [h0, h0, h1, h1] });
    let tail = [c[2], c[3], add(c[2], axis), add(c[3], axis)];
    out.push(FxQuad { fx: look.fx[0], additive: true, corners: tail, st, rgba: [h1, h1, 0x80_1010, 0x80_1010] });
    let (co, si) = (spin.cos(), spin.sin());
    let st = ST.map(|[u, v]| {
        let (u, v) = (u - 0.5, v - 0.5);
        [u * co - v * si + 0.5, u * si + v * co + 0.5]
    });
    *spin = add_rot(*spin, look.turn * 0.017_453_294);
    for f in &look.flares {
        side = set_len(side, f.width);
        let base = add(pos, set_len(dir, f.from));
        let axis = set_len(dir, -f.length);
        let tip = add(add(base, axis), axis);
        let e = look.flare_end;
        out.push(FxQuad { fx: look.fx[1], additive: true, corners: [add(base, side), sub(base, side), add(tip, side), sub(tip, side)], st, rgba: [f.rgba, f.rgba, e, e] });
    }
    let roll = rng.rand_angle();
    super::sprite(out, look.glow.1, 0.08, roll, pos, cam, look.glow.0, look.glow.2, true);
    let roll = rng.rand_angle();
    super::sprite(out, look.glow.1 * 0.4, 0.08, roll, pos, cam, look.glow.0, 0x8080_8080, true);
}

/// A shot's bolt callback (`0x406ad0` / `0x4152f0`): [`bolt`] twice, each after the arguments it does not use
/// (`0x31b9e0(0x5f, 0x6f)` then `0x31bb00`; `0x31b9e0(0x6f, 0x7f)` then `0x31bb00`). The shot's pvars: `+0x00` its
/// velocity (the way), `+0x1c` the flares' spin, `+0x28` the scroll.
pub fn shot_bolt(w: &mut MobyWorld, id: usize, look: &Bolt, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &mut w.mobys[id];
    let (pos, dir) = (m.pos, m.pv_v(0x00));
    let (mut spin, mut scroll) = (m.pv_f(0x1c), m.pv_f(0x28));
    for (a, b) in [(0x5f, 0x6f), (0x6f, 0x7f)] {
        rng.rand_range(a, b);
        rng.rand_angle();
        bolt(out, rng, look, pos, dir, &mut scroll, &mut spin, cam);
    }
    m.set_pv_f(0x1c, spin);
    m.set_pv_f(0x28, scroll);
}

/// `0x2efc98(q, m)`: the rows of the quaternion `q`, in the VU's order of operations.
fn quat_rows(q: Q) -> [V3; 3] {
    let [x, y, z, w] = q;
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (wx, wy, wz) = (x2 * w, y2 * w, z2 * w);
    let (xx, yx, zx) = (x2 * x, y2 * x, z2 * x);
    let (yy, zy) = (y2 * y, z2 * y);
    let zz = z2 * z;
    [
        [(1.0 - yy) - zz, yx - wz, zx + wy],
        [wz + yx, (1.0 - xx) - zz, zy - wx],
        [(0.0 - wy) + zx, wx + zy, (1.0 - xx) - yy],
    ]
}

/// `0x2ec810(phase, shape, n, places, turns, head, count, c₀, c₁)`: a trail through the last `count` (2 or more) of
/// a ring of places and turns, newest at `head`, going back (`(i + count − 1) % count`). At each, the `shape`'s points
/// (`x·r₀ + y·r₁ + z·r₂ + place`, the rows of its turn: `0x2efc98`, `0x2ef340`) are a ring; each pair of points
/// (`2k`, `2k + 1`) with the same pair of the ring before is a quad of FX 5 (`$gp−0x62d0`; additive, `$gp−0x62cc` /
/// `−0x62c8` = 2 / 1), s 0.5 and t 0 / 1 by the point (`0x1a8d30`). The newest ring is coloured
/// `0x2f0000(1/(count − 1), c₀, c₁)` (crate::hud::lerp_rgba), the k-th after it at `(phase + k − 1)/(count − 1)`
/// (at most 1). The rings take the quads' two corner pairs in turn, so every quad's corners are its strip's.
#[allow(clippy::too_many_arguments)]
pub fn history(out: &mut Vec<FxQuad>, phase: f32, shape: &[V3], places: &[V3], turns: &[Q], head: usize, count: usize, c: [u32; 2]) {
    let have = places.len().min(turns.len());
    if count < 2 || have < count || have <= head { return; }
    let step = 1.0 / (count - 1) as f32;
    let lerp = |t: f32| crate::hud::lerp_rgba(t, c[0], c[1]);
    let ring = |k: usize| -> Vec<V3> {
        let r = quat_rows(turns[k]);
        shape.iter().map(|s| add(add(add(scale(r[0], s[0]), scale(r[1], s[1])), scale(r[2], s[2])), places[k])).collect()
    };
    let col = lerp(step);
    let mut quads = vec![FxQuad { fx: 5, additive: true, corners: [[0.0; 3]; 4], st: [[0.5, 0.0], [0.5, 1.0], [0.5, 0.0], [0.5, 1.0]], rgba: [col; 4] }; shape.len() / 2];
    for (i, p) in ring(head).into_iter().enumerate() {
        if let Some(q) = quads.get_mut(i >> 1) { q.corners[i & 1] = p; }
    }
    let mut t = step * phase;
    let mut slot = 2;
    let mut idx = head;
    for _ in 1..count {
        idx = (idx + count - 1) % count;
        let col = lerp(t);
        t += step;
        if 1.0 < t { t = 1.0; }
        for (i, p) in ring(idx).into_iter().enumerate() {
            if let Some(q) = quads.get_mut(i >> 1) {
                q.corners[slot + (i & 1)] = p;
                q.rgba[slot + (i & 1)] = col;
            }
        }
        slot = (slot + 2) % 4;
        out.extend_from_slice(&quads);
    }
}

/// A [`strand`]'s textures and colours (its level's words): the core's FX (0xe) and RGB with its ends' colour, the
/// halo's FX (0x10) and RGB with its ends'.
#[derive(Clone, Copy, Debug)]
pub struct StrandLook {
    pub fx: [usize; 2],
    pub rgb: [u32; 2],
    pub ends: [u32; 2],
}

/// `0x423f60(f₀, f₁, extend, moby, points, a₀, a₁, n)` (L11 `0x46dc98`): through the first `n` points, additive (ALPHA
/// 0x48):
/// * the core: a ribbon of FX `fx[0]` `core` to either side across `(point − camera) × (next − point)` at each point
///   but the last, from point to point (`n − 2` quads), `rgb[0]` at alpha `alpha[0]`, the first quad's start and the
///   last one's end at `ends[0]`; s `scroll .. scroll + 1`;
/// * the halo: over each segment but the last (`n − 2` quads), both its ends at the camera's distance of the farther
///   one and lengthened by `extend` of it at each end, a quad of FX `fx[1]` `halo` to either side across the view,
///   `rgb[1]` at alpha `alpha[1]`, the first's start and the last's end at `ends[1]`.
#[allow(clippy::too_many_arguments)]
pub fn strand(out: &mut Vec<FxQuad>, p: &[V3], n: i32, scroll: f32, core: f32, halo: f32, extend: f32, alpha: [u32; 2], look: &StrandLook, cam: V3) {
    let n = n.clamp(0, p.len() as i32) as usize;
    if n < 2 { return; }
    let cc = look.rgb[0] | alpha[0] << 24;
    let hc = look.rgb[1] | alpha[1] << 24;
    // `0x2eef88(out, a, b)` = b × a, then 1/its length when not 0.
    let across = |a: V3, b: V3, k: f32| {
        let c = cross(b, a);
        let l = len(c);
        scale(c, k * if l != 0.0 { 1.0 / l } else { l })
    };
    let st = [[scroll, 0.0], [scroll, 1.0], [scroll + 1.0, 0.0], [scroll + 1.0, 1.0]];
    let s0 = across(sub(p[1], p[0]), sub(p[0], cam), core);
    let (mut q0, mut q1) = (add(p[0], s0), sub(p[0], s0));
    let mut rgba = [cc; 4];
    for i in 1..n - 1 {
        let s = across(sub(p[i + 1], p[i]), sub(p[i], cam), core);
        let (a, b) = (add(p[i], s), sub(p[i], s));
        if i == 1 {
            (rgba[0], rgba[1]) = (look.ends[0], look.ends[0]);
        } else if i == n - 2 {
            (rgba[2], rgba[3]) = (look.ends[0], look.ends[0]);
        } else if i == 2 {
            (rgba[0], rgba[1]) = (cc, cc);
        }
        out.push(FxQuad { fx: look.fx[0], additive: true, corners: [q0, q1, a, b], st, rgba });
        (q0, q1) = (a, b);
    }
    let mut rgba = [hc; 4];
    for i in 0..n - 2 {
        let (mut e, mut d) = (sub(p[i], cam), sub(p[i + 1], cam));
        let (le, ld) = (len(e), len(d));
        if ld < le {
            if ld == 0.0 { d = e; } else { d = scale(d, le / ld); }
        } else if le == 0.0 {
            e = d;
        } else {
            e = scale(e, ld / le);
        }
        let (a, b) = (add(cam, e), add(cam, d));
        let t = sub(b, a);
        let dd = scale(t, extend);
        let (a, b) = (sub(a, dd), add(b, dd));
        let s = across(sub(cam, a), t, halo);
        if i == 0 {
            (rgba[0], rgba[1]) = (look.ends[1], look.ends[1]);
        } else if i + 3 == n {
            (rgba[2], rgba[3]) = (look.ends[1], look.ends[1]);
        } else if i == 1 {
            (rgba[0], rgba[1]) = (hc, hc);
        }
        out.push(FxQuad { fx: look.fx[1], additive: true, corners: [add(a, s), sub(a, s), add(b, s), sub(b, s)], st: ST, rgba });
    }
}
