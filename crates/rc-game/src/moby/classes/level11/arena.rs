//! Joba's arena (the level's chunk 1; L11 addresses): the pieces of crate::moby::classes::level11's gap G-LVL-009
//! ported so far. Maktar's arena (crate::moby::classes::arena) is the model; Joba's master 0x0d8a (`0x43f9e0`) is its
//! own code and not ported yet, so what follows waits on it.
//!
//! * **The crowd 0x126e** ([`update_crowd`]).

use crate::moby::{Ctx, MobyWorld};

/// The arena's master and its crowd.
pub const MASTER: i16 = 0x0d8a;
pub const CROWD: i16 = 0x126e;
/// The master's pvars: `+0x14c` its score (the crowd's mood), `+0x168` the cuboid of the arena's middle (`0x443038`).
const SCORE: usize = 0x14c;
const MIDDLE: usize = 0x168;

/// `0x443038(master)`: the arena's middle, the place of the master's cuboid `+0x168` (the cuboids at `0x1b370c`).
pub fn middle(w: &MobyWorld, master: usize, ctx: &Ctx) -> Option<crate::hero::physics::V3> {
    ctx.level.cuboid_place(w.mobys[master].pv_i(MIDDLE)).map(|c| c.0)
}

/// 0x126e (`0x4763d8`, 60 placed): one of the crowd in the stands. Maktar's 0x0e96 again
/// (crate::moby::classes::arena::crowd_step, the same levels `$gp+0x3b00..+0x3b10`: 2 / 3 / 4, the wave's 600
/// frames and 4°) with three differences: the master's score is its `+0x14c`, the wave goes round the arena's middle
/// ([`middle`]) and it is hidden (draw distance 0) also while `0x1a8ff4` is set (the gravity's cylinder mode,
/// crate::gravity: not ported, never set here). It waits while its master (the moby its `+0x0` names, `0x448fb8`) is
/// in its state 0 or 1; out of view (`0x3024c8(255, its place, 5)`) its draw distance 0.
pub fn update_crowd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let master = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&ms| w.mobys.get(ms).is_some_and(|m| 1 < m.state));
    let Some(ms) = master else { return };
    let p = w.mobys[id].pos;
    if ctx.view.is_some_and(|v| v.culled(255.0, [p[0], p[1], p[2], 5.0])) {
        w.mobys[id].draw_dist = 0;
        return;
    }
    w.mobys[id].draw_dist = 0xff;
    let score = w.mobys[ms].pv_f(SCORE);
    let c = middle(w, ms, ctx).unwrap_or_default();
    super::super::arena::crowd_step(w, id, ctx, score, [c[0], c[1]]);
}
