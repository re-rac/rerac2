//! The mobys' draw callbacks (`0x2e7920(fn, moby)`: registered during the moby's update, called by the frame's draw
//! after every update): the quads they send to the GS (`0x2eb998`, an FX texture, ALPHA 0x48 additive or 0x44),
//! which the engine draws through its fx_draw. They run after the tick ([`callbacks`]) in the moby order, as the frame
//! calls them, so the random draws some make (a glow's roll) come after the tick's.
//!
//! | Callback | Moby | Draws |
//! |---|---|---|
//! | `0x3d8340` | 0x0b0a | its three glows ([`glow`]: the pilot light at `+0x3a0`, the nozzle `+0x3b0` and the mouth `+0x3c0`); climbing, its wire (`0x3d8498`) |
//! | `0x3dc220` | 0x0b0b | its three glows ([`glow`]: the visor at joint 4, the chest at joint 5, below it joint 6 at (0.25, 0, 0)) |
//! | `0x3f4998` | 0x0ce9 in glow mode | a glow at each tie of its group ([`crate::moby::classes::tie_lights::draw`]) |
//! | `0x3e4050` | the Lancer's shot | its trail's newest point and the trail ([`crate::moby::classes::shot::draw_trail`]) |
//! | `0x3a17f0` | the nanotech cluster 0x326 (the first list) | its sphere, halo and crate glass ([`crate::moby::classes::nanotech::draw`]) |
//! | `0x3efd68` | the Gravity Bomb in the hand 0xce3 | where its throw lands ([`crate::moby::classes::gravity_bomb::draw_marker`]) |
//! | `0x3f0e80` | the Gravity Bomb's blast 0xce5 | its shockwave and flares ([`crate::moby::classes::gravity_bomb::draw_blast`]) |
//! | `0x3cac30` | Oozla's conveyor 0x0a7b out | its belt of light ([`crate::moby::classes::oozla::draw_0a7b`]) |
//! | `0x3d2ea8` | Oozla's hatch 0x0a9d open | its light ([`crate::moby::classes::oozla::draw_0a9d`]) |
//! | `0x38e4c8` | the Tractor Beam holding | its beam ([`crate::moby::classes::tractor::draw`]) |
//! | `0x4137d0` | the space fights' laser 0xffa | a streak ([`crate::ship::draw_laser`]) |
//!
//! Before them, the frame's blob shadows (`0x2e7ab0`, crate::shadows: FX 0, ALPHA 0x44) go first into the same quads.
//!
//! The frame's first list (`0x2e7718`, run by `0x2e7768` before the particles, `0x3311e0`; [`Lists::first`]) holds on
//! Aranos the arrival ship's draw (`0x362b40`, which the engine's created-moby draw does), the ship's shadow
//! (`0x361af8`), class 0xd0's ribbon (`0x39ac30`, not placed) and the nanotech clusters' glow; its other users are
//! other levels' classes (G-REN-003).
//!
//! Not ported: 0x0b0a's emerge flash
//! (`0x3d8920` → `0x2ee190`, FX 0x3b), 0x0b0b's wire (`0x3db790`), the group branch of `0x3dc220` (a grouped 0x0b0b
//! draws every drawn member of its class in the group: here each draws its own).

use crate::hero::physics::{add, cross, len, scale, sub, V3};
use crate::moby::MobyWorld;

/// One quad (`0x2eb998`): FX texture, additive (0x48) or normal (0x44), corners in GS strip order, ST, RGBA.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FxQuad {
    pub fx: usize,
    pub additive: bool,
    pub corners: [V3; 4],
    pub st: [[f32; 2]; 4],
    pub rgba: [u32; 4],
}

const ST: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];

fn unit(v: V3) -> V3 {
    let l = len(v);
    if 0.0 < l { scale(v, 1.0 / l) } else { v }
}

/// `0x2edd50(a, b, phase, moby, points, n, c0, c1, fx)`: a quad behind each of a moby's `points` (in its frame: `rows`
/// and `pos`), from the point back along its facing by `b` and `a` to either side across the view (`(−forward) ×
/// (camera − pos)`); s runs from `phase` to `phase + 1` along it, t across; colours c0 at the point and c1 at the back;
/// additive (the effect texture's ALPHA row 1, `0x2ecef8(fx, 1, 0x80)`).
#[allow(clippy::too_many_arguments)]
pub fn strip(out: &mut Vec<FxQuad>, a: f32, b: f32, phase: f32, rows: &[V3; 3], pos: V3, cam: V3, points: &[V3], c0: u32, c1: u32, fx: usize) {
    let back = scale(rows[0], -1.0);
    let side = crate::hero::physics::set_len(cross(back, sub(cam, pos)), a);
    let back = crate::hero::physics::set_len(back, b);
    let st = [[phase, 0.0], [phase, 1.0], [phase + 1.0, 0.0], [phase + 1.0, 1.0]];
    for p in points {
        let w = add(add(add(scale(rows[0], p[0]), scale(rows[1], p[1])), scale(rows[2], p[2])), pos);
        let tail = add(w, back);
        out.push(FxQuad { fx, additive: true, corners: [sub(w, side), add(w, side), sub(tail, side), add(tail, side)], st, rgba: [c0, c0, c1, c1] });
    }
}

/// `0x2edf78(size, push, roll, point, fx, rgba, additive)`: a sprite facing the camera, `push` toward it: its rows the
/// way to the camera, that across gravity (`0x18a0e0`, down) and the third, turned by `roll` about the first; the
/// template `0x1aa310` (±1 square) × `size`.
#[allow(clippy::too_many_arguments)]
pub fn sprite(out: &mut Vec<FxQuad>, size: f32, push: f32, roll: f32, point: V3, cam: V3, fx: usize, rgba: u32, additive: bool) {
    let d = unit(sub(cam, point));
    let c = unit(cross(d, [0.0, 0.0, -1.0]));
    let b = unit(cross(c, d));
    let (s, co) = roll.sin_cos();
    let (c, b) = (add(scale(c, co), scale(b, s)), sub(scale(b, co), scale(c, s)));
    let p = add(point, scale(d, push));
    let corner = |y: f32, z: f32| add(p, add(scale(c, y * size), scale(b, z * size)));
    out.push(FxQuad { fx, additive, corners: [corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, 1.0), corner(1.0, -1.0)], st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [rgba; 4] });
}

/// `0x3d86a0(size, m, point, rgb, alpha, alpha2)`: a soft glow (FX 0xb, additive) at `point` pulled 0.5 toward the
/// camera, facing it with a random roll (`0x31bb00`): the template's ±2 square × `size` in `rgb` at `alpha`, then half
/// of it at `alpha2`.
pub fn glow(out: &mut Vec<FxQuad>, rng: &mut crate::rng::Rng, size: f32, point: V3, rgb: u32, alpha: u32, alpha2: u32, cam: V3) {
    let roll = rng.rand_angle();
    let f = unit(sub(cam, point));
    let p = add(point, scale(f, 0.5));
    // A basis across the view, turned by the roll.
    let up = if f[2].abs() < 0.99 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let r0 = unit(cross(up, f));
    let u0 = cross(f, r0);
    let (s, c) = roll.sin_cos();
    let r = add(scale(r0, c), scale(u0, s));
    let u = sub(scale(u0, c), scale(r0, s));
    for (k, a) in [(1.0, alpha), (0.5, alpha2)] {
        let h = 2.0 * size * k;
        let corner = |y: f32, z: f32| add(p, add(scale(r, y * h), scale(u, z * h)));
        let rgba = (rgb & 0xff_ffff) | a << 24;
        out.push(FxQuad { fx: 0xb, additive: true, corners: [corner(-1.0, -1.0), corner(1.0, -1.0), corner(-1.0, 1.0), corner(1.0, 1.0)], st: ST, rgba: [rgba; 4] });
    }
}

/// `$gp+0x12f0` / `+0x12f4`: 0x0b0a's glow colours.
const GLOW_0B0A: [u32; 2] = [0x0000_36f6, 0x0010_50b0];

/// `0x3d8340` (0x0b0a, every tick): climbing (states 4, 5, 10, or sub-state 2), its wire (`0x3d8498`: FX 0xe, 0.05
/// wide, white at alpha 0x1f, from `+0x380` to `+0x390`); then the glows.
fn draw_0b0a(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    if matches!(m.state, 4 | 5 | 10) || m.sub_state == 2 {
        let (a, b) = (m.pv_v(0x380), m.pv_v(0x390));
        let side = scale(unit(cross(sub(cam, a), unit(sub(b, a)))), 0.05);
        let c = 0x1f7f_7f7f;
        out.push(FxQuad { fx: 0xe, additive: true, corners: [add(a, side), sub(a, side), add(b, side), sub(b, side)], st: ST, rgba: [c; 4] });
    }
    glow(out, rng, 0.125, m.pv_v(0x3a0), GLOW_0B0A[0], 0x20, 0x20, cam);
    glow(out, rng, 0.15, m.pv_v(0x3b0), GLOW_0B0A[1], 0x30, 0x20, cam);
    glow(out, rng, 0.15, m.pv_v(0x3c0), GLOW_0B0A[1], 0x10, 0x30, cam);
}

/// 0x0b0b's glow colours (`0x3dc220`'s words): the visor orange, the chest blue.
const GLOW_0B0B: [u32; 2] = [0x0000_36f6, 0x00ff_3e00];

/// `0x3dc220` (0x0b0b, drawn): `0x3db998` (= [`glow`]) at joint 4 (0.125, orange, 0x20 / 0x20), joint 5 (0.15, blue,
/// 0x30 / 0x20) and joint 6's matrix × (0.25, 0, 0) (0.15, blue, 0x10 / 0x30).
fn draw_0b0b(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    if m.drawn == 0 { return; }
    let (Some(a), Some(b), Some((rows, p))) = (host.joint(m, 4), host.joint(m, 5), host.joint_matrix(m, 6)) else { return };
    let c = add(p, scale(rows[0], 0.25));
    glow(out, rng, 0.125, a, GLOW_0B0B[0], 0x20, 0x20, cam);
    glow(out, rng, 0.15, b, GLOW_0B0B[1], 0x30, 0x20, cam);
    glow(out, rng, 0.15, c, GLOW_0B0B[1], 0x10, 0x30, cam);
}

/// The frame's callbacks after the tick, by list: the first (`0x2e7718`'s, drawn before the particles) and the second
/// (`0x2e7920`'s, after them; the blob shadows and the ship's shadow first), each in the order the mobys registered
/// ([`MobyWorld::draws_first`] / [`MobyWorld::draws`]), the first list's run first. `hero_state`: Ratchet's
/// (`0x18c0b4`). `view`: the frame's camera for the callbacks' sphere checks (`0x2f0080`).
#[derive(Clone, Debug, Default)]
pub struct Lists {
    pub first: Vec<FxQuad>,
    pub second: Vec<FxQuad>,
}

pub fn callbacks(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3) -> Lists {
    let mut first = Vec::new();
    // The ship's shadow (`0x361af8`, a first-list callback; crate::travel).
    crate::travel::draw_ship_shadow(game, view, &mut first);
    for id in w.draws_first.clone() { one(w, host, game, hero_state, view, cam, id, &mut first); }
    let mut out = Vec::new();
    // The blob shadows first (`0x2e7ab0` runs before the callbacks' list, `0x2e7970`).
    crate::shadows::draw(&w.shadows, &mut out);
    for id in w.draws.clone() { one(w, host, game, hero_state, view, cam, id, &mut out); }
    Lists { first, second: out }
}

/// One registered callback (the class's).
#[allow(clippy::too_many_arguments)]
fn one(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, out: &mut Vec<FxQuad>) {
    use crate::moby::classes::tie_lights;
    if w.mobys.get(id).is_none_or(|m| 0x80 <= m.state) { return; }
    match w.mobys[id].o_class {
        0x0ce9 => tie_lights::draw(w, id, view, cam, out),
        0x0b0a => draw_0b0a(w, id, &mut game.rng, cam, out),
        0x0b0b => draw_0b0b(w, host, id, &mut game.rng, cam, out),
        crate::moby::classes::shot::SHOT_CLASS => crate::moby::classes::shot::draw_trail(&mut w.mobys[id], cam, &mut game.trail_phase, out),
        crate::moby::classes::nanotech::CLASS => crate::moby::classes::nanotech::draw(w, id, cam, out),
        crate::moby::classes::gravity_bomb::HAND_CLASS => crate::moby::classes::gravity_bomb::draw_marker(w, id, hero_state, game.frame, out),
        crate::moby::classes::gravity_bomb::BLAST_CLASS => crate::moby::classes::gravity_bomb::draw_blast(w, id, view, cam, out),
        0x0a7b => crate::moby::classes::oozla::draw_0a7b(w, id, out),
        0x0a9d => crate::moby::classes::oozla::draw_0a9d(w, id, out),
        0x0a17 => crate::moby::classes::oozla::draw_0a17(w, id, game, cam, host.camera_rows(), out),
        crate::vendor::CLASS => crate::moby::classes::vendor::draw(w, id, game, host.camera_rows(), out),
        c if crate::moby::classes::flock::CLASSES.contains(&c) => crate::moby::classes::flock::draw(w, id, view, cam, out),
        crate::moby::classes::teleport::CLASS => crate::moby::classes::teleport::draw(w, id, &host.camera_rows(), out),
        0x00bc => crate::moby::classes::tractor::draw(w, id, cam, -0.3 * (game.frame % 27) as f32, out),
        crate::ship::LASER_CLASS => crate::ship::draw_laser(w, id, cam, out),
        _ => {}
    }
}
