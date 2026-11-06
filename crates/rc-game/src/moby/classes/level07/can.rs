//! 0x12a1 (`0x411ef8`, 4 placed; the decomp's `func_L07_00410B48`, also L09 `0x423390` and L12 `0x429908`): a
//! breakable can. **0**: state 1. **1**: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 (`0x3140c0`) → 2; the hit
//! record let go every tick. **2**: its class sound 0 (`0x356570`), all its pieces thrown (`0x3219e0` = L00 `0x32b580`,
//! [`shards::throw_all`]: the `$gp+0x27e0` / `+0x27d8` / `+0x27d0` ranges, every piece a shard, sound 0 again, flags 0:
//! the level's bolts), deleted (`0x3078e8`).

use crate::moby::{shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12a1;

/// The hit flags the can answers (`0x3140c0(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;

/// `$gp+0x27e0`: the pieces' velocity ranges (x / y -3..3, z 1..7, a second).
const RANGE: [f32; 8] = [-3.0, -3.0, 1.0, 0.0, 3.0, 3.0, 7.0, 0.0];
/// `$gp+0x27d8`: their spin.
const SPIN: (f32, f32) = (90.0, 360.0);
/// `$gp+0x27d0`: their gravity.
const GRAVITY: (f32, f32) = (10.0, 20.0);

pub fn update_12a1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            w.delete(id);
        }
        _ => {}
    }
}
