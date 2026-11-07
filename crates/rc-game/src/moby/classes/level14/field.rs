//! Level 14's force field **0x0afe** (`0x3e3390`, 19 placed; the decomp's `func_L14_003E1E78`): a barrier, or a step
//! of a chain of them, that comes up, holds and goes down again. Its pvars: `+0x5c` flags (1 / 0x20 set when on),
//! `+0x60` the proximity head (on while Ratchet is below it), `+0x64` the contact kind, `+0x68` the next field of its
//! chain (a moby index, −1 none; `+0x6c` the runtime moby), `+0x70` / `+0x74` / `+0x78` the rise / hold / fall
//! times, `+0x7c` the timer, `+0x80` asked to come up, `+0x84` its own colour, `+0x88` the motes' wait.
//!
//! * **0**: its ambient (`0x31ad00`) kept as its colour (`0x80bbggrr`), the next field's moby, not asked, glow
//!   0x80008000 with mode | 0x10; a proximity head → 3, else 1.
//! * **1** (down): on its first tick hidden — not drawn (`+0x31` 0), no collision (`+0x98` 0), mode | 1. Asked, or a
//!   proximity head above Ratchet's feet → 2.
//! * **2** (rising): on its first tick drawn, its class's collision, mode `& !1 | 0x100`, the motes' wait 5, the
//!   timer the rise time. Its time out → 3.
//! * **3** (on): on its first tick mode `& !0x100`, flags | 0x20 (contact kind) or 1, the timer the hold time, glow
//!   0x80008000. Ratchet on it (`0x32cea8`, a plain field) or touching it (`0x18a0b4`, a contact one): the next
//!   field, when that is a 0x0afe, asked (`+0x80` = 1). A proximity head holds its timer while Ratchet is below it.
//!   Its time out → 4.
//! * **4** (falling): on its first tick mode | 0x100, not asked, flags 0, glow 0x80000080, the motes' wait 5, the
//!   timer the fall time. Its time out → 1.
//! * **2 and 4, every tick**: the motes' wait out: a mote ([`motes`], `0x3e3e98`) and the wait 5 again; its ambient
//!   the white `0xffffffff` lerped toward its colour by its fade ([`fade`], `0x2f7c08`); its first two rows its
//!   angles' (`0x2f7280`) times the fade (`0x3e3750`); its platform block written with no move (`0x32a188`); its
//!   draw ([`draw`], `0x3e3840`) registered.
//!
//! Its fade ([`fade`], `0x3e37c0`): rising `1 − timer / rise`, falling `timer / fall`, down 0, else 1.
//!
//! Stand-in: the contact fields' touch (`0x18a0b4`, a moby pointer the port does not keep: no writer of it is in the
//! level's or the boot's code by its address) is the moby Ratchet's capsule last pushed out of (`0x18a08c`).
//! Not ported: the matrix refresh after the rows' scale (`0x31b8e0`, the engine draws from the rows).

use crate::hero::physics::{add, cross, scale, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0afe;

const FLAGS: usize = 0x5c;
const HEAD: usize = 0x60;
const CONTACT: usize = 0x64;
const NEXT: usize = 0x68;
const NEXT_MOBY: usize = 0x6c;
const RISE: usize = 0x70;
const HOLD: usize = 0x74;
const FALL: usize = 0x78;
const TIMER: usize = 0x7c;
const ASKED: usize = 0x80;
const COLOUR: usize = 0x84;
const WAIT: usize = 0x88;

/// `$gp+0x1470..` (L14 `0x1b0460`): the shimmer's reach, its phase rate and its rows' spread.
const SHIMMER: f32 = f32::from_bits(0x3cf5_c28f);
const SHIMMER_RATE: f32 = 3.0;
const SHIMMER_SPREAD: f32 = 2.0;
/// `$gp+0x147c`: the half size at full fade (the draw's square and the motes' box); `+0x1480` the pull toward the camera.
const SIZE: f32 = f32::from_bits(0x3fb3_3333);
const PULL: f32 = 2.0;
/// `$gp+0x1484..0x148c`: the colour it fades from (b, g, r: 0xff each).
const WHITE: u32 = 0xffff_ffff;
/// `$gp+0x1490`: the motes' wait.
const MOTE_WAIT: i32 = 5;
/// `$gp+0x1494..0x14b4`: the motes' longest life, sizes, spin range, colour ranges (r, g, b), def entry, gravity.
const MOTE_LIFE: i32 = 42;
const MOTE_SIZE: [f32; 2] = [f32::from_bits(0x4776_1801), 0.0];
const MOTE_SPIN: i32 = 5;
const MOTE_RGB: [i32; 3] = [0xff, 0xff, 0];
const MOTE_KIND: usize = 3;
const MOTE_GRAVITY: f32 = f32::from_bits(0x3ba3_d70a);
/// The draw's mask (FX 0x3c) and the frame grid's cells a side.
const MASK_FX: usize = 0x3c;
const CELLS: usize = 8;

/// `0x3e37c0` (module docs).
pub fn fade(m: &crate::moby::Moby) -> f32 {
    let t = m.pv_i(TIMER) as f32;
    match m.state {
        4 => t / m.pv_i(FALL) as f32,
        2 => 1.0 - t / m.pv_i(RISE) as f32,
        1 => 0.0,
        _ => 1.0,
    }
}

fn next_field(w: &MobyWorld, id: usize) -> Option<usize> {
    let k = usize::try_from(w.mobys[id].pv_i(NEXT_MOBY)).ok()?;
    w.mobys.get(k).filter(|m| m.o_class == CLASS && m.state < 0x80).map(|_| k)
}

/// 0x0afe (module docs).
pub fn update_0afe(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hero_z = ctx.hero.pos[2];
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let [r, g, b] = m.ambient.map(|c| c as i32);
            m.set_pv_i(COLOUR, ((b << 16) + (g << 8) + r) | i32::MIN);
            let next = m.pv_i(NEXT);
            // The runtime moby (`$gp−0x0ad4 + index·0x100`): the port's mobys sit at their record index.
            m.set_pv_i(NEXT_MOBY, if next == -1 { -1 } else { next });
            m.set_pv_i(ASKED, 0);
            m.glow = 0x8000_8000;
            m.mode |= 0x10;
            let to = if m.pv_i(HEAD) == 0 { 1 } else { 3 };
            m.set_state(to, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            if m.first() {
                m.drawn = 0;
                m.has_collision = false;
                m.mode |= 1;
            }
            if m.pv_i(ASKED) != 0 || (m.pv_i(HEAD) != 0 && hero_z < m.pos[2]) { m.set_state(2, -1); }
        }
        2 => {
            let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[id];
            if m.first() {
                m.drawn = 1;
                m.mode = m.mode & !1 | 0x100;
                m.has_collision = solid;
                m.set_pv_i(WAIT, MOTE_WAIT);
                m.set_pv_i(TIMER, m.pv_i(RISE));
            }
            if m.count_i(TIMER) { m.set_state(3, -1); }
            changing(w, id, ctx);
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.first() {
                m.mode &= !0x100;
                let f = m.pv_i(FLAGS) | if m.pv_i(CONTACT) == 0 { 1 } else { 0x20 };
                m.set_pv_i(FLAGS, f);
                m.set_pv_i(TIMER, m.pv_i(HOLD));
                m.glow = 0x8000_8000;
            }
            let contact = w.mobys[id].pv_i(CONTACT) != 0;
            let on = if contact { ctx.hero.push_moby == Some(id) } else { crate::moby::ai::stands_on(ctx, id) };
            if on {
                if let Some(k) = next_field(w, id) { w.mobys[k].set_pv_i(ASKED, 1); }
            }
            let m = &mut w.mobys[id];
            if m.pv_i(HEAD) != 0 && hero_z < m.pos[2] { m.set_pv_i(TIMER, m.pv_i(HOLD)); }
            if m.count_i(TIMER) { m.set_state(4, -1); }
        }
        4 => {
            let m = &mut w.mobys[id];
            if m.first() {
                m.mode |= 0x100;
                m.set_pv_i(ASKED, 0);
                m.set_pv_i(FLAGS, 0);
                m.glow = 0x8000_0080;
                m.set_pv_i(WAIT, MOTE_WAIT);
                m.set_pv_i(TIMER, m.pv_i(FALL));
            }
            if m.count_i(TIMER) { m.set_state(1, -1); }
            changing(w, id, ctx);
        }
        _ => {}
    }
}

/// States 2 and 4's shared tail (module docs).
fn changing(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].count_i(WAIT) {
        motes(w, id, ctx);
        w.mobys[id].set_pv_i(WAIT, MOTE_WAIT);
    }
    let m = &mut w.mobys[id];
    let f = fade(m);
    let c = crate::hud::lerp_rgba(f, WHITE, m.pv_i(COLOUR) as u32);
    m.ambient = [c as u8, (c >> 8) as u8, (c >> 16) as u8];
    let rows = crate::moby::attach::rows_of(m.rot);
    m.rows = [scale(rows[0], f), scale(rows[1], f), rows[2]];
    if let Some(b) = crate::moby::platform::block(m) {
        let rot = m.rot;
        crate::moby::platform::write(m, b, [0.0; 3], rot, rot);
    }
    w.register_draw(id);
}

/// `0x3e3e98`: a mote (crate::particles::mote) within the field's square — x and y `±1.4·fade`, z `−1.4·fade..0` off
/// its place — at rest, its colour a random red and green (`0..0xff` each, no blue) lerped toward its colour by the
/// fade, its life `21..42`, its spin `0..5`, alpha 0x80 → 0, size 63000 → 0, falling 0.005 a tick, def entry 3.
fn motes(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (f, mut p, colour) = (fade(m), m.pos, m.pv_i(COLOUR) as u32);
    let g = &mut ctx.game.rng;
    p[0] += g.randf(SIZE * -f, SIZE * f);
    p[1] += g.randf(SIZE * -f, SIZE * f);
    p[2] += g.randf(SIZE * -f, 0.0);
    let r = g.rand_range(0, MOTE_RGB[0]) as u32;
    let gr = g.rand_range(0, MOTE_RGB[1]) as u32;
    let b = g.rand_range(0, MOTE_RGB[2]) as u32;
    let c = crate::hud::lerp_rgba(f, r << 16 | gr << 8 | 0xff00_0000 | b, colour);
    let life = g.rand_range(MOTE_LIFE / 2, MOTE_LIFE);
    let spin = g.rand_range(0, MOTE_SPIN);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::mote::mote(sys, g, MOTE_SIZE[0], MOTE_SIZE[1], MOTE_GRAVITY, p, [0.0; 3], life, c, spin, 0x80, 0, MOTE_KIND);
}

/// `0x3e3840` (a second-list callback while rising or falling): a square facing the camera — its rows the way to the
/// camera `a`, `b = a × down` and `c = b × a`, at its place pulled [`PULL`] toward the camera, the corners (0, ∓1, ±1)
/// × 1.4·fade — drawn first as a mask (FX 0x3c, colour 0x80808080, ALPHA 0x4a `Cd` under TEST 0x31001: its alpha only
/// into the frame), then as an 8 × 8 grid of cells textured with the frame copied as it stands (`0x2ea6c8(−1)`) at
/// ALPHA 0x54 (`(Cs − Cd)·Ad + Cd`: blended by that mask) under TEST 0x53001 (z GEQUAL, no z write). Each cell's
/// corner (row j, column k: `c0 + (c2 − c0)·k/8 + (c1 − c0)·j/8`) reads the copy at its screen place (`0x2eea28`)
/// moved by `(1 − fade)·0.03·sin(wrap(2·3·fade·π + n·π/2))` (n = j across, k down; `0x3e3e00` wraps to 0..2π),
/// held to 0..1 (`0x3e3ba0`): the world behind it wavers less as it comes up. Here each cell is one
/// [`crate::fx::FrameQuad`] with the mask's part of FX 0x3c (the explosion's shimmer does the same).
pub fn draw(w: &MobyWorld, id: usize, cam: V3, cam_rows: [V3; 3], frame: &mut Vec<crate::fx::FrameQuad>) {
    let m = &w.mobys[id];
    let f = fade(m);
    let unit = |v: V3| { let l = crate::hero::physics::len(v); if 0.0 < l { scale(v, 1.0 / l) } else { v } };
    let a = unit(sub(cam, m.pos));
    let b = unit(cross(a, crate::particles::GRAVITY));
    let c = cross(b, a);
    let at = add(m.pos, scale(a, PULL));
    let s = SIZE * f;
    let corner = |y: f32, z: f32| add(at, add(scale(b, y * s), scale(c, z * s)));
    let q = [corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, 1.0), corner(1.0, -1.0)];
    let (x, y) = (sub(q[2], q[0]), sub(q[1], q[0]));
    let wobble = |n: usize| {
        let mut p = (SHIMMER_RATE + SHIMMER_RATE) * f * std::f32::consts::PI + n as f32 * std::f32::consts::PI / SHIMMER_SPREAD;
        while std::f32::consts::TAU < p { p -= std::f32::consts::TAU; }
        while p < 0.0 { p += std::f32::consts::TAU; }
        (1.0 - f) * SHIMMER * p.sin()
    };
    let point = |j: usize, k: usize| add(q[0], add(scale(x, k as f32 * 0.125), scale(y, j as f32 * 0.125)));
    for j in 0..CELLS {
        for k in 0..CELLS {
            let cells = [(j, k), (j, k + 1), (j + 1, k), (j + 1, k + 1)];
            let corners = cells.map(|(j, k)| point(j, k));
            if !crate::fx::in_front(&corners, cam, cam_rows[0]) { continue; }
            frame.push(crate::fx::FrameQuad {
                source: crate::fx::FrameSource::Copy,
                mask: Some(MASK_FX),
                corners,
                st: cells.map(|(j, k)| [k as f32 * 0.125, j as f32 * 0.125]),
                offset: cells.map(|(j, k)| [wobble(j), wobble(k)]),
                clamp: 1.0,
                rgba: [0x8080_8080; 4],
            });
        }
    }
}
