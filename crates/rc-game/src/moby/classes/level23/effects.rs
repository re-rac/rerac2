//! Damosel orbit's effect objects of its opponent 0x0f7f's fight (L23 addresses; the opponent is not ported, and the
//! words it keeps — crate::moby::classes::level23::Globals — stay as the level starts them):
//!
//! * **0x1251** ([`update_1251`], `0x42b1c0`, the decomp's `func_L23_00429CC0`; one placed): its set-up (`0x42b2c8`,
//!   state 0: its sound handles −1, draw distance 0x50, update 0xff, its words 0, at (500, 500, 575), state 1). With
//!   the opponent about: its guard up (`+0x604`) the draw `0x42a018`, else with `$gp+0x2654` in 1..`$gp+0x2658` the draw
//!   `0x42a438`, else below 0 the lights `0x42bdc0`, else its `+0x04` 0; out of the second the sound `0x1b1b94` let go
//!   (its fade `0x1b3930` 0.02), out of the third `0x1b1b96`. Its ticks counted (`0x42a4f8`).
//! * **0x1252** ([`update_1252`], `0x42c1c0`, `func_L23_0042ACC0`; one placed): its set-up (`0x42c210`: draw distance
//!   0x50, update 0xff, its words 0, at (500, 500, 585), state 1); with the opponent about and `0x1b1430` set the draw
//!   `0x42c2b0`; its ticks counted (`0x42c290`).
//!
//! Not ported: the draws and lights (`0x42a018`, `0x42a438`, `0x42bdc0`, `0x42c2b0`), the sound handles they hold, and
//! the set-up's tables for them (`0x42b2c8`: the points `0x2a7170` above ground and the edges `0x2a7a74` that use them,
//! `0x25e8b0` / `0x25e9d0` / `0x1b3980`).

use crate::moby::{Ctx, MobyWorld};

pub const C1251: i16 = 0x1251;
pub const C1252: i16 = 0x1252;

fn set_up(w: &mut MobyWorld, id: usize, at: [f32; 3]) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < 0x10 { m.pvars.resize(0x10, 0); }
    if m.state != 0 { return; }
    (m.draw_dist, m.update_dist) = (0x50, 0xff);
    for off in [0, 4, 8] { m.set_pv_i(off, 0); }
    m.pos = at;
    m.set_state(1, 0);
}

fn count(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
}

/// 0x1251 (`0x42b1c0`, module docs).
pub fn update_1251(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    set_up(w, id, [500.0, 500.0, 575.0]);
    let o = w.mobys.iter().position(|m| m.o_class == super::targets::OPPONENT && m.state < 0xfd);
    if let Some(o) = o {
        let g = &w.level23;
        let guard = w.mobys[o].pv_i(0x604) != 0;
        let second = 0 < g.hero_guard && g.hero_guard < g.hero_guard_top;
        if !guard && !second && 0 <= g.hero_guard { w.mobys[id].set_pv_i(4, 0); }
    }
    count(w, id);
}

/// 0x1252 (`0x42c1c0`, module docs).
pub fn update_1252(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    set_up(w, id, [500.0, 500.0, 585.0]);
    count(w, id);
}
