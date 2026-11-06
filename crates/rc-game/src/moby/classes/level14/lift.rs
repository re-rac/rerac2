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
//! Not ported: the prison's way graph (`0x4220d0`, the links between its nodes through the lift's two ends, 2 along
//! its first row each side, opened and closed; it belongs to the prisoners' walk, L03 `0x4199f0` / L02 `0x32edd8`) and
//! the prisoners kept on it (`0x3e0718`: a member in sub 1 more than 1 / 1.5 off its middle along its rows is sent
//! back onto it, `0x3cab28`, through the same graph).

use crate::hero::physics::{dist2, ease};
use crate::moby::{ai, Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x0ae8;
pub const CLASS_B: i16 = 0x0ae9;

const BLOCK: usize = 0x20;
const CUBOID: usize = 0x60;
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
            m.set_state(1, -1);
        }
        1 => {
            let hero = ctx.hero.pos;
            let on = ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0;
            let go = if on {
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
            }
        }
        _ => {}
    }
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
