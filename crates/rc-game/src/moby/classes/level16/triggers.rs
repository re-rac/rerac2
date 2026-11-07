//! Snivelak's triggers (L16 addresses; the engine functions named by their L16 addresses with the L00 ones the port
//! knows them by).
//!
//! * **0x1107** ([`update_1107`], `0x41a160`, one placed; the decomp's `func_L16_00418D28`): once the moby `+0x68`[0]
//!   is gone (state 0xfe / 0xfd) and Ratchet is in the cuboid `[1]` (`0x31ae60` = L00 `0x321858`), the timer bytes
//!   (`+0xbc`) of the mobys `[4..8]` (−1 none) set to 1; then it rests (2).
//! * **0x1154** ([`update_1154`], `0x41ddc8`, 3 placed; the decomp's `func_L16_0041C990`): its pvars the moby `[0]`,
//!   the cuboid `[1]`, the group `[2]` and the mobys `[4..12]` it wakes. **0**: with neither `[1]` nor `[2]` it rests
//!   (4); with `[1]` the cuboid's wait (1), else the group's (2). **1**: the moby `[0]` gone and Ratchet in `[1]`: each
//!   of the mobys woken ([`wake`]: a trooper 0x0ea3 alarmed, `+0x24c` = 1 (crate::moby::classes::thugs), anything else
//!   its timer byte 1); 4. **2**: no member of `[2]` alive (`0x315fd0` = L00 `0x31c8e0`,
//!   crate::moby::classes::props::group_alive): the moby `[4]` woken once for each of `[4..12]` not −1 (the game's
//!   loop reads `[4]` every time); 4.
//! * **0x1145** ([`update_1145`], `0x41dad8`, one placed; the decomp's `func_L16_0041C6A0`): untargeted and hidden
//!   (`+0x31` 0, mode | 1) at its first tick; then Ratchet standing on it (`0x18a15c`, on the ground `0x18a160`) is hit
//!   for 200 × his health limit (`0x1a7a08`; flags 1, its class in the kind word, along (0, 10, 0): `0x2e84e8` = L00
//!   `0x2ef138`, `0x3169a0` / `0x316bc0` = L00 `0x31d158` / `0x31d378`).

use crate::moby::director::{gone, link};
use crate::moby::{Ctx, MobyWorld};

pub const OPENER: i16 = 0x1107;
pub const WAKER: i16 = 0x1154;
pub const FLOOR: i16 = 0x1145;

/// A moby the triggers wake: a trooper alarmed, anything else its timer byte set.
fn wake(w: &mut MobyWorld, k: i32) {
    let Some(k) = link(w, k) else { return };
    let e = &mut w.mobys[k];
    if e.o_class == crate::moby::classes::thugs::CLASS { e.set_pv_i(crate::moby::classes::thugs::ALARM, 1) } else { e.timer = 1 }
}

/// The moby named by pvar word `off` gone (−1: read as gone).
fn waited_gone(w: &MobyWorld, id: usize, off: usize) -> bool {
    usize::try_from(w.mobys[id].pv_i(off)).map_or(true, |k| gone(w, k))
}

/// 0x1107 (module docs).
pub fn update_1107(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { w.mobys[id].state = 1; }
    if w.mobys[id].state != 1 { return; }
    if !waited_gone(w, id, 0) || !ctx.level.in_cuboid(w.mobys[id].pv_i(4), ctx.hero.pos) { return; }
    for off in [0x10, 0x14, 0x18, 0x1c] {
        let k = w.mobys[id].pv_i(off);
        if let Some(k) = (k != -1).then(|| link(w, k)).flatten() { w.mobys[k].timer = 1; }
    }
    w.mobys[id].state = 2;
}

/// 0x1154 (module docs).
pub fn update_1154(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(4 * k);
    match w.mobys[id].state {
        0 => {
            let next = if pv(w, 1) == -1 && pv(w, 2) == -1 { 4 } else if pv(w, 1) != -1 { 1 } else { 2 };
            w.mobys[id].set_state(next, -1);
        }
        1 => {
            if waited_gone(w, id, 0) && ctx.level.in_cuboid(pv(w, 1), ctx.hero.pos) {
                for k in 4..12 {
                    let e = pv(w, k);
                    if e != -1 { wake(w, e); }
                }
                w.mobys[id].set_state(4, -1);
            }
        }
        2 => {
            if crate::moby::classes::props::group_alive(w, pv(w, 2)) == 0 {
                for k in 4..12 {
                    if pv(w, k) != -1 { wake(w, pv(w, 4)); }
                }
                w.mobys[id].set_state(4, -1);
            }
        }
        _ => {}
    }
}

/// 0x1145 (module docs).
pub fn update_1145(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.drawn = 0;
        m.mode |= 1;
    }
    if m.state != 1 || ctx.hero.ground_moby != Some(id) || ctx.hero.grounded_ticks == 0 { return; }
    let mut t = crate::moby::hits::HitTemplate::new(ctx.game.max_health as f32 * 200.0, Some(id), 1, [0.0, 10.0, 0.0, 1.0]);
    t.kind = (FLOOR as u16 as u32) << 16;
    if let Some(h) = ctx.hero_moby { w.deliver_hit_any(h, &t, [0.0, 0.0, 0.0, 1.0]); }
}
