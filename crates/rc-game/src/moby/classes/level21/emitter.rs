//! The museum's sprite emitter, class **0x1317** ([`update_1317`], `0x418d30`, 3 placed; the decomp's
//! `func_US2_L30_00418D30`): every few ticks a sprite from its place, all of it set by its pvars. The developers'
//! editor for those pvars is not ported.
//!
//! Its pvars (the editor's fields): `+0x00` the sprite's velocity, `+0x0c` the timer (a float), `+0x10` its life,
//! `+0x14` / `+0x18` / `+0x1c` its r / g / b, `+0x20` its spin, `+0x24` / `+0x28` its alpha from / to, `+0x2c` /
//! `+0x30` its size from / to (× 210000), `+0x34` its fall, `+0x38` its w, `+0x3c` the editor, `+0x40` / `+0x44` the
//! particle type and frame its look is taken from, `+0x48` the cuboid that offers the editor (−1 none), `+0x4c` /
//! `+0x4e` the wait's least and most (shorts).
//!
//! * **0** (set-up): the most made at least 0, the least within 0..most; the timer `0x3221c8(least, most)` (L00
//!   `0x31b9e0`, crate::rng::Rng::rand_range); then 1.
//! * **1**: Ratchet in its cuboid (`0x327920` = L00 `0x321858`, crate::level::LevelData::in_cuboid): the prompt 0x1338
//!   held (`0x32c268(1, 0x1338, 1)` = L00 `0x327ae0`, crate::prompt::Prompt::force).
//! * **Each tick**: the timer down by 1; when it was 0, the bounds kept again (the least within 0..most), the timer
//!   drawn anew and a sprite (`0x33fa68` = L00 `0x33ef78`, crate::particles::sprite) at its place, given ALPHA 0xff
//!   (`+0x09`) and the frame `*def[+0x40][+0x44]` (`0x1ff7c0`).
//!
//! Not ported: the editor (state 2: L2 + R2 held in state 1, `0x138320 & 3`; its menu moby 0x118a, `0x413490` /
//! `0x413ba0`).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1317;

const VEL: usize = 0x00;
const TIMER: usize = 0x0c;
const LIFE: usize = 0x10;
const SPIN: usize = 0x20;
const ALPHA0: usize = 0x24;
const ALPHA1: usize = 0x28;
const SIZE0: usize = 0x2c;
const SIZE1: usize = 0x30;
const FALL: usize = 0x34;
const W: usize = 0x38;
const TYPE: usize = 0x40;
const FRAME: usize = 0x44;
const CUBOID: usize = 0x48;
const LEAST: usize = 0x4c;
const MOST: usize = 0x4e;

/// The least kept within 0..most (`0x418d88`), then the wait drawn.
fn rearm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let (lo, hi) = (m.pv_s(LEAST), m.pv_s(MOST));
    if hi < lo { m.set_pv_s(LEAST, hi) } else if lo < 0 { m.set_pv_s(LEAST, 0) }
    let (lo, hi) = (m.pv_s(LEAST), m.pv_s(MOST));
    let t = ctx.game.rng.rand_range(lo as i32, hi as i32);
    w.mobys[id].set_pv_f(TIMER, t as f32);
}

pub fn update_1317(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_s(MOST) < 0 { m.set_pv_s(MOST, 0); }
            rearm(w, id, ctx);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let c = w.mobys[id].pv_i(CUBOID);
            if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) { ctx.game.prompt.force(1, 0x1338, 1); }
        }
        _ => {}
    }
    let t = w.mobys[id].pv_f(TIMER);
    w.mobys[id].set_pv_f(TIMER, t - 1.0);
    if t != 0.0 { return; }
    rearm(w, id, ctx);
    let m = &w.mobys[id];
    let rgb = (m.pv_i(0x1c) as u32) << 16 | (m.pv_i(0x18) as u32) << 8 | m.pv_i(0x14) as u32;
    let k = f32::from_bits(0x484d_1400);
    let (pos, vel, life, spin) = (m.pos, m.pv_v(VEL), m.pv_i(LIFE), m.pv_i(SPIN));
    let (a0, a1, s0, s1, fall, wv) = (m.pv_i(ALPHA0), m.pv_i(ALPHA1), m.pv_f(SIZE0) * k, m.pv_f(SIZE1) * k, m.pv_f(FALL), m.pv_i(W));
    let (ty, frame) = (m.pv_i(TYPE), m.pv_i(FRAME));
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    if let Some(i) = crate::particles::sprite(sys, &mut ctx.game.rng, s0, s1, fall, pos, vel, life, rgb, spin as u8, a0, a1, wv) {
        let f = sys.def_frame(ty as u8, frame.max(0) as usize);
        let r = &mut sys.pool.recs[i];
        r[9] = 0xff;
        r[2] = f;
    }
}
