//! Two classes of the hooks (the once-flag `+0xbe` bit 2, `once & 4`): one hooks groups as Ratchet walks in, the
//! other blows apart when hooked.
//!
//! * **0x0d3c** (`0x3ef980`; the decomp's `func_L07_003EE640`): eight cuboids `+0x68`[0..7], each with a group
//!   `[8..15]` (−1 none). **0**: 1. **1**: for each with a group and Ratchet (`0x189ea0`) in its cuboid (`0x317920`),
//!   the group's members hooked (`0x312c98(group, 1)` = L02 `0x320b50`, crate::moby::classes::maktar::hook_group)
//!   and the group −1, so each hooks once.
//! * **0x0d3b** (`0x3ef6f0`; the decomp's `func_L07_003EE3B0`) ([`update_0d3b`]).

use crate::gravity::to_world;
use crate::hero::physics::{add, V3};
use crate::moby::classes::{explosion, maktar::hook_group};
use crate::moby::{shards, Ctx, MobyWorld};

pub const TRIGGER: i16 = 0x0d3c;
pub const BLAST: i16 = 0x0d3b;

/// 0x0d3c (`0x3ef980`): module docs.
pub fn update_0d3c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            for k in 0..8 {
                let (vol, group) = (w.mobys[id].pv_i(4 * k), w.mobys[id].pv_i(0x20 + 4 * k));
                if group != -1 && ctx.level.in_cuboid(vol, ctx.hero.pos) {
                    hook_group(w, group, true);
                    w.mobys[id].set_pv_i(0x20 + 4 * k, -1);
                }
            }
        }
        _ => {}
    }
}

/// `$gp+0x1be0`: where the explosion starts, in its frame.
const BLAST_AT: V3 = [0.0, 0.0, 1.5];
/// `$gp+0x1bc0` / `$gp+0x1bd0`: the pieces' velocity, a second in its frame (x / y / z from the first to the second).
const VEL_LO: V3 = [-1.0, 10.0, 1.0];
const VEL_HI: V3 = [1.0, 10.0, 3.0];
/// `$gp+0x1bf0`: their spin, degrees a second (either way, `0x311ca8`).
const SPIN: (f32, f32) = (45.0, 90.0);
/// `$gp+0x1bf8`: their gravity.
const GRAVITY: (f32, f32) = (5.0, 10.0);
/// The pieces (0..8).
const PIECES: u8 = 9;

/// 0x0d3b (`0x3ef6f0`). **0**: 1. **1**: hooked (`0x320c98`): the big explosion (`0x3f0768`, crate::moby::classes::
/// explosion::start, no ground) at [`BLAST_AT`] in its frame facing its second row (`+0xd0`); 2. **2**: its first
/// tick (`0x320c28`) its class sound 0; its nine pieces (`0x2d1cb0` mode 2, crate::moby::shards::spawn_piece: no sound,
/// [`VEL_LO`]..[`VEL_HI`] through its rows, a sixtieth a tick, [`SPIN`], [`GRAVITY`]), each given this update
/// (`+0x64`), mode 0x2000 and state 3; deleted. **3** (a piece): its first tick its wait (`+0x96`) 0..60 (`0x311bf0`);
/// it tumbles (`0x2d20f0`, crate::moby::shards::step, its end ignored); the wait out, it fades 4 alpha a tick and goes.
pub fn update_0d3b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.mobys[id].once & 4 != 0 {
                let m = &w.mobys[id];
                let at = add(to_world(BLAST_AT, &m.rows), m.pos);
                let dir = m.rows[1];
                explosion::start(w, ctx, 0.0, at, dir);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            for k in 0..PIECES {
                let rng = &mut ctx.game.rng;
                let l: V3 = std::array::from_fn(|i| rng.randf(VEL_LO[i], VEL_HI[i]) * 0.016_666_668);
                let vel = to_world(l, &w.mobys[id].rows);
                let rng = &mut ctx.game.rng;
                let spin = rng.randf_signed(SPIN.0, SPIN.1);
                let gravity = rng.randf(GRAVITY.0, GRAVITY.1);
                if let Some(p) = shards::spawn_piece(w, id, ctx, spin, gravity, k, vel, 0xff) {
                    let m = &mut w.mobys[p];
                    m.update = Some(update_0d3b);
                    m.mode |= 0x2000;
                    m.set_state(3, -1);
                }
            }
            w.delete(id);
        }
        3 => {
            if w.mobys[id].first() {
                let t = ctx.game.rng.rand_range(0, 60);
                w.mobys[id].state_ticks = t as i16;
            }
            shards::step(w, id, ctx);
            let m = &mut w.mobys[id];
            let t = m.state_ticks as u16;
            let out = t == 0 || {
                m.state_ticks = t.wrapping_sub(1) as i16;
                t == 1
            };
            if out {
                if 4 <= m.alpha { m.alpha -= 4; } else { w.delete(id); }
            }
        }
        _ => {}
    }
}
