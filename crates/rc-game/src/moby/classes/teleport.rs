//! The teleport-in carrier, class 0x09bb (L02 start `0x2f0170`, update `0x3cb2d0`; the same code on every level that
//! has the class): a moby and up to three linked to it appear in a beam — the beam grows, a ring of glints bursts, and
//! the mobys grow out of nothing, their colour easing in from blue.
//!
//! **The start** ([`start`]) creates the carrier at `pos` (update distance 0xff) and hides the mobys (mode |= 0x103:
//! hidden, no update, rows kept; not drawn, no collision), keeping each one's mode (`+0x20..`) and ambient (`+0x30..`,
//! `0x3285d0`), and for the linked ones their place and angles in the moby's frame (packed as the staged particles'
//! velocities: `+0x40..` / `+0x4c..`, `0x327bc0`); the moby is put at `pos` facing `rot`.
//!
//! **The update**: the beam's growth `+8` runs to 1 at 8/60 a tick ([`draw`]); then, on its first tick, the burst
//! ([`burst`] `0x3cb600`), and the growth `+0xc` at 4/60 a tick, over which the mobys are shown, their rows x and y
//! scaled by it ([`grow`] `0x3cb428`); at 1 their modes and colours come back and the carrier goes.

use crate::hero::physics::{add, add_rot, approach, scale, sub, V3};
use crate::hud::lerp_rgba;
use crate::moby::attach::{angles_of, rows_of, turn};
use crate::moby::{Ctx, Moby, MobyWorld};
use crate::particles::{glint, pack_vel, unpack_vel};

pub const CLASS: i16 = 0x09bb;
/// The colour the mobys start from (`0x3cb428`).
const FROM: u32 = 0x80ff_8000;

/// `0x3285d0`: the ambient as a colour word.
fn ambient(m: &Moby) -> u32 { 0x8000_0000 | (m.ambient[2] as u32) << 16 | (m.ambient[1] as u32) << 8 | m.ambient[0] as u32 }

fn to_local(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

fn hide(m: &mut Moby) {
    m.mode |= 0x103;
    m.drawn = 0;
    m.has_collision = false;
}

/// `0x2f0170(m, pos, rot, linked, n)`: see the module; false when the level has no carrier class.
pub fn start(w: &mut MobyWorld, m: usize, pos: V3, rot: V3, linked: &[usize]) -> bool {
    if w.class(CLASS).is_none() { return false; }
    let c = w.create(CLASS);
    w.mobys[c].update_dist = 0xff;
    w.mobys[c].pos = pos;
    let (m_pos, m_rows) = (w.mobys[m].pos, w.mobys[m].rows);
    // `0x2f3880`: the inverse of the moby's rows (their transpose).
    let inv: [V3; 3] = std::array::from_fn(|i| std::array::from_fn(|j| m_rows[j][i]));
    let (mode, amb) = (w.mobys[m].mode, ambient(&w.mobys[m]));
    let cp = &mut w.mobys[c];
    cp.set_pv_i(0x10, m as i32);
    cp.set_pv_i(0x20, mode as i32);
    cp.set_pv_i(0x30, amb as i32);
    for (k, &l) in linked.iter().take(3).enumerate() {
        let x = &w.mobys[l];
        let local = to_local(sub(x.pos, m_pos), &inv);
        let angles = angles_of(&turn(&inv, &x.rows));
        let (mode, amb) = (x.mode, ambient(x));
        let cp = &mut w.mobys[c];
        cp.set_pv_i(0x14 + 4 * k, l as i32);
        cp.set_pv_i(0x24 + 4 * k, mode as i32);
        cp.set_pv_i(0x34 + 4 * k, amb as i32);
        cp.set_pv_i(0x40 + 4 * k, pack_vel(local) as i32);
        cp.set_pv_i(0x4c + 4 * k, pack_vel(angles) as i32);
        hide(&mut w.mobys[l]);
    }
    let x = &mut w.mobys[m];
    x.pos = pos;
    x.rot = rot;
    x.rows = rows_of(rot);
    hide(x);
    w.changed = true;
    true
}

/// The carried mobys: `+0x10` and the linked `+0x14..` (0 slots are none: the game's null pointers; slot 0 always set).
fn carried(c: &Moby) -> [Option<usize>; 4] {
    std::array::from_fn(|k| match (k, c.pv_i(0x10 + 4 * k)) {
        (0, i) => usize::try_from(i).ok(),
        (_, 0) => None,
        (_, i) => usize::try_from(i).ok(),
    })
}

/// 0x09bb (`0x3cb2d0`).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let c = &w.mobys[id];
            if c.pv_f(8) < 1.0 {
                let mut t = c.pv_f(8);
                approach(1.0, 8.0 / 60.0, &mut t);
                w.mobys[id].set_pv_f(8, t);
            } else {
                if 1.0 <= c.pv_f(0xc) {
                    let slots = carried(c);
                    for (k, s) in slots.iter().enumerate() {
                        let Some(m) = *s else { continue };
                        let (mode, amb) = (w.mobys[id].pv_i(0x20 + 4 * k) as u16, w.mobys[id].pv_i(0x30 + 4 * k) as u32);
                        let Some(x) = w.mobys.get_mut(m) else { continue };
                        x.mode = mode;
                        x.set_ambient_word(amb);
                    }
                    w.delete(id);
                    return;
                }
                if w.mobys[id].pv_i(4) == 0 {
                    w.mobys[id].set_pv_i(4, 1);
                    burst(w, id, ctx);
                }
                grow(w, id, ctx);
                let mut t = w.mobys[id].pv_f(0xc);
                approach(1.0, 4.0 / 60.0, &mut t);
                w.mobys[id].set_pv_f(0xc, t);
            }
            w.register_draw(id);
        }
        _ => {}
    }
}

/// `0x3cb600`: 32 pairs of glints 0.5 over the carrier fanning out round it (each `randf(17°, 28°)` on from the last),
/// `randf(6, 10)/60` a tick (the second of a pair 0.8 of it), sizes 0.3 and 0.2, `rand_range(15, 30)` ticks, their
/// colour `randf(0, 1)` of the way from 0x80800000 to 0x80808000.
fn burst(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = w.mobys[id].pos;
    let p = [c[0], c[1], c[2] + 0.5];
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let mut a = 0.0f32;
    for _ in 0..32 {
        let life = rng.rand_range(15, 30);
        let t = rng.randf(0.0, 1.0);
        let rgba = lerp_rgba(t, 0x8080_0000, 0x8080_8000);
        let s = rng.randf(6.0 / 60.0, 10.0 / 60.0);
        let v = [a.cos() * s, a.sin() * s, 0.0];
        a = add_rot(a, rng.randf(f32::from_bits(0x3e96_cbe4), f32::from_bits(0x3efb_53d2)));
        let rot = rng.randi(0xff) as u8;
        glint(sys, 0.3, p, v, rgba, life as u16, rot, false, hero);
        let rot = rng.randi(0xff) as u8;
        glint(sys, 0.2, p, scale(v, 0.8), rgba, life as u16, rot, false, hero);
    }
}

/// `0x3cb428`: the moby shown (collision back), its rows from its angles with x and y × the growth, its colour the
/// growth's way from blue to its own; each linked one shown at its place and angles in that frame, its colour the same
/// way to its own.
fn grow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = &w.mobys[id];
    let t = c.pv_f(0xc);
    let slots = carried(c);
    let Some(m) = slots[0].filter(|&m| m < w.mobys.len()) else { return };
    let amb = c.pv_i(0x30) as u32;
    let coll = w.class(w.mobys[m].o_class).is_some_and(|k| k.has_collision);
    let x = &mut w.mobys[m];
    x.mode &= !1;
    x.drawn = 1;
    x.has_collision = coll;
    let mut rows = rows_of(x.rot);
    rows[0] = scale(rows[0], t);
    rows[1] = scale(rows[1], t);
    x.rows = rows;
    x.set_ambient_word(lerp_rgba(t, FROM, amb));
    let m_pos = x.pos;
    w.rebuild(m, ctx.host);
    for k in 1..4 {
        let Some(l) = slots[k].filter(|&l| l < w.mobys.len()) else { continue };
        let c = &w.mobys[id];
        let (local, angles, amb) = (unpack_vel(c.pv_i(0x40 + 4 * (k - 1)) as u32), unpack_vel(c.pv_i(0x4c + 4 * (k - 1)) as u32), c.pv_i(0x30 + 4 * k) as u32);
        let coll = w.class(w.mobys[l].o_class).is_some_and(|k| k.has_collision);
        let x = &mut w.mobys[l];
        x.mode &= !1;
        x.drawn = 1;
        x.has_collision = coll;
        x.pos = add(m_pos, to_local(local, &rows));
        x.rows = turn(&rows, &rows_of(angles));
        x.set_ambient_word(lerp_rgba(t, FROM, amb));
        w.rebuild(l, ctx.host);
    }
    w.changed = true;
}

/// `0x3cb7d8` (its draw callback): the beam, a quad up from 1.5·`+0xc` under the carrier facing the camera's yaw
/// (`0x1ba518`), `+8 + 3·+0xc` wide and `6·(1 − +8) + 1 + 3·+0xc` tall, coloured 0x80ff0000 → 0x80804000 over `+8`
/// then → 0x00808000 over `+0xc`; effect texture 6, additive, drawn twice.
pub fn draw(w: &MobyWorld, id: usize, cam_rows: &[V3; 3], out: &mut Vec<crate::fx::FxQuad>) {
    let c = &w.mobys[id];
    let (t1, t2) = (c.pv_f(8), c.pv_f(0xc));
    let yaw = cam_rows[0][1].atan2(cam_rows[0][0]);
    let r = rows_of([0.0, 0.0, yaw]);
    let width = t1 + 3.0 * t2;
    let height = (1.0 - t1) * 6.0 + 1.0 + 3.0 * t2;
    let (side, up) = (scale(r[1], width), scale(r[2], height));
    let base = [c.pos[0], c.pos[1], c.pos[2] - 3.0 * t2 * 0.5];
    let rgba = if t1 < 1.0 { lerp_rgba(t1, 0x80ff_0000, 0x8080_4000) } else { lerp_rgba(t2, 0x8080_4000, 0x0080_8000) };
    let at = |y: f32, z: f32| add(add(base, scale(side, y)), scale(up, z));
    let q = crate::fx::FxQuad {
        fx: 6,
        additive: true,
        corners: [at(0.5, 0.0), at(0.5, 1.0), at(-0.5, 0.0), at(-0.5, 1.0)],
        st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]],
        rgba: [rgba; 4],
    };
    out.push(q);
    out.push(q);
}
