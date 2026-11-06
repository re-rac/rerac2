//! Dobbo's small props (L09 addresses).
//!
//! * **0x0c21** ([`update_0c21`], `0x3e4630`, 30 placed): a spinner about its y angle. Its speed (`+0xbc`) is
//!   150..200° a second and its way random (`+0x95`).
//! * **0x0bf4** / **0x0bf7** ([`update_0bf4`], `0x3def10` / `0x3defb0`, 27 and 6 placed): sequence 0 started at a
//!   third of the way through for every third moby (by its address mod 3; the port uses its index), then state 1.

use crate::moby::{Ctx, MobyWorld};

pub const SPINNER: i16 = 0x0c21;
pub const SWAY_A: i16 = 0x0bf4;
pub const SWAY_B: i16 = 0x0bf7;

pub fn update_0c21(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.timer = ctx.game.rng.rand_range(0x96, 0xc8) as u8;
            if ctx.game.rng.randi(2) != 0 { m.sub_state = 1; }
            m.set_state(1, -1);
        }
        1 => {
            let step = m.timer as f32 * 0.000_290_888_25;
            m.rot[1] = crate::hero::physics::add_rot(m.rot[1], if m.sub_state != 0 { -step } else { step });
        }
        _ => {}
    }
}

pub fn update_0bf4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let n = ctx.host.seq_frames(&w.mobys[id], 0).unwrap_or(0) as f32;
    let f = (n * ((id % 3) as f32 * 0.333)) as u8;
    ctx.host.set_anim(&w.mobys[id], 0, f, 1);
    w.mobys[id].set_state(1, -1);
}

pub const WHEEL: i16 = 0x0a14;

/// 0x0a14 (`0x3ca110`, 1 placed; the decomp's `func_L09_003C8CD8`): a wheel that carries its moby group (`+4`)
/// and tie group (`+0`) round with it (`0x320b58` / `0x321068` = L00 `0x3246c0` / `0x324bd0`,
/// crate::moby::MobyWorld::attach / carry; up to 100 members), turning about its x at `+8`° a second. With neither
/// group (both −1) it waits in 0.
pub fn update_0a14(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (ties, mobys) = (m.pv_i(0), m.pv_i(4));
            if ties == -1 && mobys == -1 { return; }
            w.mobys[id].draw_dist = 0xff;
            w.attach(id, 100, mobys, ties, -1);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.rot[0] = crate::hero::physics::add_rot(m.rot[0], m.pv_f(8) * 0.000_290_888_25);
            w.carry(id, &*ctx.host);
            w.changed = true;
        }
        _ => {}
    }
}
