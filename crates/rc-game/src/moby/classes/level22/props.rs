//! Dobbo orbit's small props (L22 addresses).
//!
//! * **0x0a6c** ([`update_0a6c`], `0x3d1c88`, 13 placed; the decomp's v1.01 `func_L22_003D0840`, no C): on its first
//!   update it hides (mode |= 0x41) and goes to state 1 (`0x320e30` = L00 `0x31b8d0`), which does nothing.
//! * **0x12db** ([`update_12db`]): the asteroid clusters' skill point (crate::moby::classes::asteroid_cluster).

use crate::moby::classes::asteroid_cluster::CLUSTER;
use crate::moby::{Ctx, MobyWorld};

pub const HIDDEN: i16 = 0x0a6c;

pub fn update_0a6c(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state != 0 { return; }
    m.mode |= 0x41;
    m.set_state(1, -1);
}

/// The clusters' skill point watcher.
pub const WATCHER: i16 = 0x12db;

/// 0x12db (`0x387310`, one placed; the decomp's `func_US2_L22_00387310`): **0** updated at any distance, 1. **1**
/// every tick: no cluster 0x0d3d left on the moby list (`0x1b20a8`) but in its state 3 (broken up): skill point 0x18
/// (`0x1a7a80`; the level sound 1 `0x3640e0`, the message 0x1233 `0x32c0f8`, `0x1a903c` 1:
/// crate::game::GameState::award_skill_point).
pub fn update_12db(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let left = w.mobys.iter().any(|m| m.o_class == CLUSTER && m.state < 0x80 && m.state != 3);
            if !left { ctx.game.award_skill_point(crate::skill_points::point::CLUSTERS); }
        }
        _ => {}
    }
}
