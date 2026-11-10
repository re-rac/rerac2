//! The glider's launch pad **0x0988** ([`update_0988`], L08 `0x3d5310`, L09 `0x3c3df0`, L19 `0x3c19f0`, one placed on
//! each; the decomp's `func_L08_003D3EB8`, the same code on the three levels), where Ratchet's glider 0x0905
//! (crate::moby::classes::glider) waits: its glow pulses (`0x335a18` = L01 `0x31cf58`, L00 `0x326048`, as
//! crate::moby::classes::oozla::update_0fa0) from 0x80aaaad2 to 0x80396464, its phase (`+0x0c`) on 0.0611 a tick;
//! nothing without pvars.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0988;

/// 0x0988 (module docs).
pub fn update_0988(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < 0x10 { return; }
    let phase = m.pv_f(0xc);
    m.set_pv_f(0xc, crate::hero::physics::add_rot(phase, 0.061_086_536));
    m.glow = crate::hud::lerp_rgba(phase.sin() * 0.5 + 0.5, 0x80aa_aad2, 0x8039_6464);
}
