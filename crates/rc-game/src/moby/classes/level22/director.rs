//! Level 0x16's director (L22 addresses).

use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x133a;

/// The help text it repeats and its slot.
const TEXT: i32 = 0x9fd;
const SLOT: i32 = 0x54;
/// Its period (ticks).
const EVERY: i32 = 1800;

/// 0x133a (`0x416f48`; the decomp's `func_L22_00415A48`): a help text every 1800 ticks.
/// * **0**: update distance 0xff, the wait `+0x00` 1800; then 1.
/// * **1**: the wait counts down (`0x2f4458` = L00 `0x2eeab0`). Once out, with both of Giant Clank's counts at 0
///   (the block `0x18b6a8` points at, `0x1b18a0`: its words `+8` and `+0xc`; the hero's level start sets the pointer,
///   L22 `0x2a8c34`, and Giant Clank's state 0x5e spends `+0xc`, `0x2d7e18`), the help text 0x9fd in slot 0x54
///   (`0x2fac00` = L00 `0x2f5270`) and the wait again. While a count is up the wait stays out and the test runs each tick.
///
/// Not ported: Giant Clank (the hero is never him yet, crate::camera), so his counts are never read as empty and the
/// text never shows.
pub fn update_133a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.update_dist = 0xff;
        m.set_pv_i(0, EVERY);
    }
    if m.state != 1 { return; }
    let mut t = m.pv_i(0);
    let out = countdown(&mut t);
    m.set_pv_i(0, t);
    if out && giant_clank_empty(ctx) {
        ctx.game.help(TEXT, SLOT);
        w.mobys[id].set_pv_i(0, EVERY);
    }
}

/// Giant Clank's two counts both 0 (`0x18b6a8` → `+8`, `+0xc`): no Giant Clank in the port (above).
fn giant_clank_empty(_ctx: &Ctx) -> bool { false }
