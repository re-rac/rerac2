//! The kill cuboids **0x040f** ([`update_040f`], L13 `0x38d600`, 2 placed; L14 `0x38e460`, 1; L20 `0x39bcf0`, 1; the
//! decomp's `func_L13_0038C1B8`): cuboids that kill Ratchet (L13 addresses: `0x32b7c0` = L00 `0x321858`,
//! crate::level::LevelData::in_cuboid; `0x2cf1f8` = L00 `0x2c4560`, Ratchet's state).

use crate::moby::{Ctx, MobyWorld};

/// 0x040f (L13 `0x38d600`): its pvars `[0..0x80]` 32 cuboids that kill, `[0x80..0x100]` 32 that kill at once (−1 none),
/// `[0x100]` ≠ 0: the first kind only slows a glide. **0**: update distance 0xff, 1. **1**: Ratchet in one of the first
/// (`0x32b7c0`): with `[0x100]` his glide's sink 0.25 (`0x18c2e8`); else as Ratchet (`0x18c0d4` = 0) his fall out of
/// the level, state 0x74 (`0x2cf1f8(0x74, 1)`, not again while in it), another body his death (`0x2be1f8`, after a
/// fade `0x2f2770(10)`); in one of the second his death.
///
/// Not ported: the blocking fade to black of 10 frames (`0x2f2770`: it draws the frames itself before the death).
pub fn update_040f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.update_dist = 0xff;
            m.state = 1;
        }
        1 => {
            let p = ctx.hero.pos;
            let glide = m.pv_i(0x100) != 0;
            for k in 0..32 {
                let c = m.pv_i(4 * k);
                if c < 0 || !ctx.level.in_cuboid(c, p) { continue; }
                if glide {
                    ctx.hero.glide_sink = 0.25;
                } else if ctx.hero.mode == 0 {
                    if ctx.hero.state != 0x74 { ctx.hero.request_state(0x74, true); }
                } else {
                    ctx.hero.die();
                }
            }
            for k in 0..32 {
                let c = m.pv_i(0x80 + 4 * k);
                if 0 <= c && ctx.level.in_cuboid(c, p) { ctx.hero.die(); }
            }
        }
        _ => {}
    }
}

