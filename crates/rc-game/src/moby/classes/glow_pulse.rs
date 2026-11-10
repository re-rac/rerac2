//! The glow pulser **0x115f** ([`update_115f`], L12 `0x4254e0`, 2 placed; the museum `0x413468`, 2; the decomp's
//! `func_L12_00424078`): it pulses the glow words of its group and of up to four mobys between two colours. L12
//! addresses: `0x320328` = L00 `0x31c8e0` (crate::moby::classes::props::group_alive), `0x32a698` the pulse
//! ([`pulse`]: `0x2f2b48` = L00 `0x2ef460` the sine, `0x2f36e8` = L00 `0x2f0000` crate::hud::lerp_rgba).

use crate::moby::{Ctx, MobyWorld};

/// `0x32a698(a, c₀, c₁)`: between `c₀` and `c₁` by `0.5 + 0.5·sin a`.
fn pulse(a: f32, c0: u32, c1: u32) -> u32 { crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, c0, c1) }

/// 0x115f (L12 `0x4254e0`): pvars `[0..0xc]` / `[0x10..0x1c]` the two colours' bytes (r, g, b, a as words),
/// `[0x20]` the period in ticks, `[0x24]` the count, `[0x28]` / `[0x2c]` the colours, `[0x30..0x3c]` up to four
/// mobys (−1 ends them). **0**: the colours packed, the count of mobys before the first −1; 2 with any (1 without;
/// both print a debug line). **1**: with two of its group (`+0x21`) alive or more (`0x320328`), each drawn member's
/// glow word `+0x78` the pulse ([`pulse`]) at `(frame % period) / period · 2π − π`; then as 2. **2**: the same for
/// each drawn moby of its list.
pub fn update_115f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    match m.state {
        0 => {
            let pack = |b: usize| (0..4).fold(0u32, |c, k| c | (m.pv_i(b + 4 * k) as u32) << (8 * k));
            let (c0, c1) = (pack(0), pack(0x10));
            let n = (0..4).take_while(|&k| m.pv_i(0x30 + 4 * k) != -1).count() as i32;
            let m = &mut w.mobys[id];
            m.set_pv_i(0x28, c0 as i32);
            m.set_pv_i(0x2c, c1 as i32);
            m.set_pv_i(0x24, n);
            m.set_state(if n == 0 { 1 } else { 2 }, -1);
            return;
        }
        1 | 2 => {}
        _ => return,
    }
    let period = m.pv_i(0x20);
    if period == 0 { return; }
    let (c0, c1) = (m.pv_i(0x28) as u32, m.pv_i(0x2c) as u32);
    let a = (ctx.game.frame % period) as f32 * std::f32::consts::TAU / period as f32 - std::f32::consts::PI;
    let glow = pulse(a, c0, c1);
    let mut list: Vec<usize> = Vec::new();
    if m.state == 1 && super::props::group_alive(w, m.group as u8 as i32) >= 2 {
        list.extend(w.groups.get(m.group as u8 as usize).into_iter().flatten().copied());
    }
    let n = m.pv_i(0x24);
    list.extend((0..n.clamp(0, 4) as usize).filter_map(|k| usize::try_from(m.pv_i(0x30 + 4 * k)).ok()));
    for i in list {
        if let Some(o) = w.mobys.get_mut(i) { if o.drawn != 0 { o.glow = glow; } }
    }
}
