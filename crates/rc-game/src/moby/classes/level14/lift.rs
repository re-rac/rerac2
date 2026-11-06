//! The prison's lifts **0x0ae8** (5 placed) and **0x0ae9** (one), one update (`0x3e01f8`, the decomp's
//! `func_US2_L14_003E01F8`): a platform between its place and the height of its cuboid, sent by △ while Ratchet stands
//! on it (with its group of prisoners, when it waits for them, all aboard or one of them far off in height) or
//! called by him from the other end.
//!
//! **Its pvars:** `+0x20` its platform block (crate::moby::platform), `+0x60` the cuboid whose centre's height is its
//! top (−1: deleted at the set-up), `+0x64` the prison's way graph moby, `+0x68` its group of prisoners (255 none),
//! `+0x6c` it waits for them, `+0x70` its bottom (its height at the set-up), `+0x74` its speed, `+0x78` its looped
//! sound, `+0x7c` the end it is at or going to (0 the bottom, 1 the top).
//!
//! **0** the set-up: no pvars or no cuboid, deleted; its bottom its height, at the bottom, the graph's links by it
//! opened (`0x3e0618`), 1. **1** waiting: Ratchet on the ground on it — its prisoners kept on it (`0x3e0718`); not
//! waiting for them, or each of them with a movement block standing on it ([`all_aboard`], `0x3e0958`), or one of
//! them more than 1.5 off its height ([`one_far`], `0x3e0a58`): the prompt 0x12ec (`0x32f8e0` = L00 `0x327a28`, owner
//! 4) and △ sends it. Off it, within 24 of it across and at the other end (within 1 of the top's height, or above
//! 0.25 under the bottom), he calls it. Sent or called: 2, the end the other, the links closed. **2** moving: toward
//! the end's height (`0x3267b0` = L00 `0x31ed78`, crate::hero::physics::ease: 1/1800 up, 1/900 braking, at most 1/6
//! a tick); going up with Ratchet in the cuboid more than 1 under it, to 1.5 over the top braking 1/450. Within its
//! speed of the end (not so held): there, its speed 0, 1 and the links opened. Its platform block (`0x32a0a0` = L00
//! `0x322668`, crate::moby::platform::write) takes the move.
//!
//! **Each tick**: faster than 0.01 a tick its class sound 0 looped (`0x36ada0` = L00 `0x3605f0`) while not playing,
//! else stopped (`0x36aaa0` = L00 `0x3602f0`). The port carries Ratchet with the move (crate::moby::carry_hero, as
//! crate::moby::classes::level12::lifts).
//!
//! **The way graph** (the robots' network moby 0x0f2d at `+0x64`, crate::moby::classes::endako_d::net_link,
//! `0x4220d0`): [`ends`] (`0x3e0618`) opens the link between the record under its middle and the record 2 along its
//! first row (against it at the top) and closes the one on the other side, or (sent off) closes both.
//!
//! **Its prisoners kept on it** ([`keep`], `0x3e0718`, while Ratchet stands on it): each living member of its group
//! in sub-state 1 (`0x3caa18`) more than 1 off its middle along its first row or 1.5 along its second is sent back
//! onto it (held to 0.5 / 1.25 along those, crate::moby::classes::endako_d::send_to, `0x3cab28`).
//!
//! **Driven** ([`drive`], `0x3e0b68`; [`set_end`], `0x3e0ba8`): another class (0x112c) takes it over — its set-up
//! run if it has not been, then [`update_driven`] (`0x3e0bb8`) in place of its update: toward the end `+0x7c` names
//! (crate::hero::physics::ease: 1/900 up, 1/450 braking, at most 1 a tick) with no prompt, call or graph, its
//! sound as above.

use crate::hero::physics::{add, dist2, dot, ease, scale, V3};
use crate::moby::classes::endako_d;
use crate::moby::{ai, Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x0ae8;
pub const CLASS_B: i16 = 0x0ae9;

const BLOCK: usize = 0x20;
const CUBOID: usize = 0x60;
const NET: usize = 0x64;
const GROUP: usize = 0x68;
const WAITS: usize = 0x6c;
const BOTTOM: usize = 0x70;
const SPEED: usize = 0x74;
const SOUND: usize = 0x78;
const END: usize = 0x7c;

/// The prompt's owner and text.
const OWNER: i32 = 4;
const TEXT: u32 = 0x12ec;

/// Its prisoners: the living members of its group `+0x68` (none for 255).
fn prisoners(w: &MobyWorld, id: usize) -> Vec<usize> {
    let g = w.mobys[id].pv_i(GROUP);
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
        .into_iter().filter(|&k| w.mobys.get(k).is_some_and(|o| o.state != 0xfe && o.state != 0xfd)).collect()
}

/// `0x3e0958`: each of its prisoners with a movement block (pvars word 0x18) stands on it (that block's floor moby).
fn all_aboard(w: &MobyWorld, id: usize) -> bool {
    prisoners(w, id).into_iter().all(|k| {
        let o = &w.mobys[k];
        o.pv_i(0x18) == 0 || ai::link(o, ai::mv(o) + 0x88) == Some(id)
    })
}

/// `0x3e0a58`: one of its prisoners is more than 1.5 off its height.
fn one_far(w: &MobyWorld, id: usize) -> bool {
    let z = w.mobys[id].pos[2];
    prisoners(w, id).into_iter().any(|k| 1.5 < (w.mobys[k].pos[2] - z).abs())
}

/// The top: its cuboid's centre's height.
fn top(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<f32> {
    usize::try_from(w.mobys[id].pv_i(CUBOID)).ok().and_then(|i| ctx.level.cuboids.get(i)).map(|c| c.matrix[3][2])
}

/// 0x0ae8 / 0x0ae9 (module docs).
pub fn update_0ae8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            if w.mobys[id].pvars.is_empty() || w.mobys[id].pv_i(CUBOID) == -1 {
                w.delete(id);
                return;
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(END, 0);
            m.set_pv_f(BOTTOM, m.pos[2]);
            // The port's: the sound's handle none (the original's test that the channel is its own needs none).
            m.set_pv_i(SOUND, -1);
            ends(w, id, ctx, false);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let hero = ctx.hero.pos;
            let on = ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0;
            let go = if on {
                keep(w, id, ctx);
                if w.mobys[id].pv_i(WAITS) == 0 || all_aboard(w, id) || one_far(w, id) {
                    ctx.game.prompt.request(OWNER, TEXT, 1);
                    ctx.hero.items.pressed & button::TRIANGLE != 0
                } else {
                    false
                }
            } else {
                let m = &w.mobys[id];
                let there = if m.pv_i(END) == 0 {
                    top(w, id, ctx).is_some_and(|t| (hero[2] - t).abs() < 1.0)
                } else {
                    -0.25 < hero[2] - m.pv_f(BOTTOM)
                };
                there && dist2(hero, m.pos) < 24.0
            };
            if go {
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.set_pv_i(END, (m.pv_i(END) + 1) & 1);
                ends(w, id, ctx, true);
            }
        }
        2 => {
            let hero = ctx.hero.pos;
            let up = w.mobys[id].pv_i(END) != 0;
            let mut t = if up { top(w, id, ctx).unwrap_or(0.0) } else { w.mobys[id].pv_f(BOTTOM) };
            let mut brake = f32::from_bits(0x3a91_a2b4);
            let z = w.mobys[id].pos[2];
            let held = up && hero[2] < z - 1.0 && ctx.level.in_cuboid(w.mobys[id].pv_i(CUBOID), hero);
            if held {
                t += 1.5;
                brake = f32::from_bits(0x3b11_a2b4);
            }
            let m = &mut w.mobys[id];
            if held || m.pv_f(SPEED) < (z - t).abs() {
                let (mut z, mut v) = (m.pos[2], m.pv_f(SPEED));
                ease(t, f32::from_bits(0x3a11_a2b4), brake, f32::from_bits(0x3e2a_aaab), &mut z, &mut v);
                m.pos[2] = z;
                m.set_pv_f(SPEED, v);
                crate::moby::platform::write(m, BLOCK, [0.0, 0.0, v], rot, rot);
            } else {
                m.pos[2] = t;
                m.set_pv_f(SPEED, 0.0);
                m.set_state(1, -1);
                crate::moby::platform::write(m, BLOCK, [0.0; 3], rot, rot);
                ends(w, id, ctx, false);
            }
        }
        _ => {}
    }
    tail(w, id, ctx, old, rot);
}

/// Both updates' tail: the looped sound by its speed, Ratchet carried with the move.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, old: V3, rot: V3) {
    let h = w.mobys[id].pv_i(SOUND);
    if 0.01 < w.mobys[id].pv_f(SPEED).abs() {
        if !(h != -1 && ctx.host.sound_playing(h)) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(SOUND, h);
        }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
    }
    let m = &w.mobys[id];
    let (pos, yaw) = (m.pos, m.rot[2]);
    crate::moby::carry_hero(ctx, id, old, rot[2], pos, yaw);
    w.changed = true;
}

/// `0x3e0618(m, off)`: the graph's links by it (module docs); `off` (sent off) closes both.
fn ends(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: bool) {
    let Some(net) = endako_d::network(w, w.mobys[id].pv_i(NET)) else { return };
    let m = &w.mobys[id];
    let side = if m.pv_i(END) != 0 { -2.0 } else { 2.0 };
    let (p, row) = (m.pos, m.rows[0]);
    let (a, b) = (add(scale(row, side), p), add(scale(row, -side), p));
    endako_d::net_link(w, net, ctx, a, p, !off);
    endako_d::net_link(w, net, ctx, b, p, false);
}

/// `0x3e0718(m)`: its prisoners kept on it (module docs).
fn keep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (p, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    for k in prisoners(w, id) {
        if !endako_d::sub_one(w, k) { continue; }
        let d = crate::hero::physics::sub(w.mobys[k].pos, p);
        // `0x32dd58` with its rows' transpose (`0x2f7528`): the offset in its frame.
        let mut l = [dot(d, rows[0]), dot(d, rows[1]), dot(d, rows[2])];
        let mut held = false;
        if 1.0 < l[0].abs() {
            held = true;
            l[0] = l[0].clamp(-0.5, 0.5);
        }
        if 1.5 < l[1].abs() {
            held = true;
            l[1] = l[1].clamp(-1.25, 1.25);
        }
        if !held { continue; }
        let back: V3 = std::array::from_fn(|i| l[0] * rows[0][i] + l[1] * rows[1][i] + l[2] * rows[2][i]);
        endako_d::send_to(w, k, ctx, add(back, p));
    }
}

/// `0x3e0b68(m)`: the lift taken over by another class (module docs).
pub fn drive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { update_0ae8(w, id, ctx); }
    w.mobys[id].update = Some(update_driven);
}

/// `0x3e0ba8(m, end)`: the end a driven lift goes to (0 the bottom, 1 the top).
pub fn set_end(w: &mut MobyWorld, id: usize, end: i32) {
    if let Some(m) = w.mobys.get_mut(id) { m.set_pv_i(END, end); }
}

/// `0x3e0bb8`: a driven lift's update (module docs).
pub fn update_driven(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let t = if w.mobys[id].pv_i(END) != 0 { top(w, id, ctx).unwrap_or(0.0) } else { w.mobys[id].pv_f(BOTTOM) };
    let m = &mut w.mobys[id];
    let (mut z, mut v) = (m.pos[2], m.pv_f(SPEED));
    ease(t, f32::from_bits(0x3a91_a2b4), f32::from_bits(0x3b11_a2b4), 1.0, &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(SPEED, v);
    crate::moby::platform::write(m, BLOCK, [0.0, 0.0, v], rot, rot);
    tail(w, id, ctx, old, rot);
}
