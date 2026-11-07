//! Boldan's walkway cuboids **0x0f55** (`0x41d270`, one placed; the decomp's `func_L13_0041BDA8`): up to eight cuboids
//! (pvars `+0x00..+0x1c`, −1 none). **0**: updated at any distance; 1. **Each tick**: for each cuboid holding Ratchet
//! (`0x32b7c0` = L00 `0x321858`), his walkways' normal `0x18a110` the cuboid's first row made −1 long (`0x2f9800` = L00
//! `0x2ef138`) and the word `0x18a11c` 10 (crate::gravity::Gravity::walkway_normal / walkway_k; frame mode 1, their
//! reader, is not ported).

use crate::hero::physics::set_len;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f55;

/// 0x0f55 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        w.mobys[id].update_dist = 0xff;
        w.mobys[id].state = 1;
    }
    for i in 0..8 {
        let c = w.mobys[id].pv_i(4 * i);
        if c == -1 || !ctx.level.in_cuboid(c, ctx.hero.pos) { continue; }
        let Some(v) = ctx.level.cuboids.get(c as usize) else { continue };
        let r = v.matrix[0];
        ctx.hero.gravity.walkway_normal = set_len([r[0], r[1], r[2]], -1.0);
        ctx.hero.gravity.walkway_k = 10.0;
    }
}
