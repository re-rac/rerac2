//! Smolg's collectible **0x1249** (`0x4203e8`, one placed; the decomp's `func_US2_L17_004203E8`). Its pvar `+0x0` the
//! moby it hooks once taken (−1 none; the one placed, a 0x0f6e).
//! * **0**: gone (`0x312188` = L00 `0x311688`) with its uid gone for good (`0x21f044`) or counted (`0x19b4a8`'s 3-bit
//!   count; crate::game::GameState::uid_count). Else it turns 5° a tick and sheds two twinkles (`0x33d970` = L00
//!   `0x33a6a0`, crate::particles::twinkle: 3 ticks, green 0x208020ff and white 0x20ffffff, spinning opposite ways)
//!   about its place (each axis ±0.15) moving out at a random 0.05..0.1 a tick (`0x327278` = L00 `0x325100`: a random
//!   yaw, its pitch 20°..88°), of size 2..5 (×0.1 / ×1, ×0.03 / ×0.3), falling 0.006. Within 1.5 across of Ratchet:
//!   the flag `0x13963c` set, its uid counted (crate::game::GameState::bump_uid); with its moby hidden (mode | 1,
//!   `+0x31` 0), that moby hooked (`0x32d808` = L00 `0x32a818(m, 1)`), 1; else gone.
//! * **1**: once its moby is gone (state 0xfe / 0xfd; or with none): the message 0x1292 for 120 ticks (`0x329298` =
//!   L00 `0x327448`), then the help 0xa0f (slot 0x66) while the flags `0x1395b8 + 0x86` / `+ 0x89` leave one to go
//!   (two, one less for each set), else 0xa10 (0x67) (`0x2f6158` = L00 `0x2f5270`); gone.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1249;

const LINK: usize = 0x0;
/// `0x1395b8 + 0x84`: one taken.
const TAKEN: usize = 0x84;

fn link(w: &MobyWorld, id: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(LINK)).ok().filter(|&k| k < w.mobys.len()) }

/// 0x1249 (module docs).
pub fn update_1249(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return waiting(w, id, ctx); }
    let uid = w.mobys[id].uid;
    if ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0 || ctx.game.uid_count(uid) != 0 { return w.delete(id); }
    let m = &mut w.mobys[id];
    m.rot[2] = crate::hero::physics::add_rot(m.rot[2], f32::from_bits(0x3db2_b8c4));
    let at = m.pos;
    for _ in 0..2 {
        let g = &mut ctx.game.rng;
        let size = g.randf(2.0, 5.0);
        let sign: i8 = if g.randi(2) == 0 { -1 } else { 1 };
        let yaw = g.rand_angle();
        let pitch = g.randf(f32::from_bits(0x3eb2_b8c4), f32::from_bits(0x3fc4_9877));
        let r = g.randf(0.05, 0.1);
        // `0x325100(r, yaw, pitch)`.
        let vel = [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r];
        let pos: [f32; 3] = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.15));
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::twinkle(sys, g, size * 0.1, size, 0.006, pos, 3, 0x2080_20ff, 0, sign, vel, 0);
            crate::particles::twinkle(sys, g, size * 0.03, size * 0.3, 0.006, pos, 3, 0x20ff_ffff, 1, -sign, vel, 0);
        }
    }
    if crate::hero::physics::dist2(at, ctx.hero.pos) < 1.5 {
        if let Some(f) = ctx.game.flags.get_mut(TAKEN) { *f = 1; }
        ctx.game.bump_uid(uid);
        if w.mobys[id].pv_i(LINK) == -1 { return w.delete(id); }
        let m = &mut w.mobys[id];
        m.mode |= 1;
        m.drawn = 0;
        if let Some(k) = link(w, id) { w.mobys[k].once |= 4; }
        w.mobys[id].set_state(1, -1);
    }
}

/// State 1 (module docs).
fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(LINK) != -1 {
        let gone = link(w, id).is_none_or(|k| matches!(w.mobys[k].state, 0xfe | 0xfd));
        if !gone { return; }
    }
    ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1292, ticks: Some(0x78) });
    let flag = |k: usize| ctx.game.flags.get(k).is_some_and(|&b| b != 0);
    let mut left = if flag(0x86) { 1 } else { 2 };
    if flag(0x89) { left -= 1; }
    if 0 < left { ctx.game.help(0xa0f, 0x66); } else { ctx.game.help(0xa10, 0x67); }
    w.delete(id);
}
