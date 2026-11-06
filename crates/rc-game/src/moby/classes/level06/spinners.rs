//! Level 6's two spinning breakables, **0x0fc1** (L06 `0x458888`, 18 placed; the decomp's `func_L06_00457270`) and
//! **0x0fc0** (L06 `0x458718`, 3; `func_L06_00457100`): each turns about its own z every tick (`0x364a90` = L00
//! `0x323c90`, [`crate::moby::attach::turned`]) until a hit with flags 0x4010000 (`0x35df18`, its record let go
//! either way) shatters it: flung the other way when the falling pole 0x12c4 (crate::moby::classes::level06::pole) hit
//! it, its pieces (`0x367668` = L00 `0x327498`, crate::moby::shards::break_into: 3..6 across and up), its class sound 0
//! and it goes.
//!
//! * 0x0fc1: 1° a tick; its pieces mask 7, flung 1/6 a tick along its first row (−1/6 by the pole); `+0x00` the pole's.
//! * 0x0fc0: `+0x00` its turn in degrees a second the other way (× π/180/60); its pieces mask 0x1fe, flung 1/6 a tick
//!   along the world's x (−1/6 by the pole); `+0x04` the pole's.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS_0FC1: i16 = 0x0fc1;
pub const CLASS_0FC0: i16 = 0x0fc0;

/// State 1's hit (flags 0x4010000): whether it came, and whether from the pole.
fn hit(w: &mut MobyWorld, id: usize) -> Option<bool> {
    let h = w.hit_message(id).filter(|h| h.tmpl.flags & 0x401_0000 != 0);
    w.clear_hit(id);
    let h = h?;
    Some(h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == super::pole::CLASS))
}

fn spin(w: &mut MobyWorld, id: usize, z: f32) {
    let m = &mut w.mobys[id];
    m.rot = crate::moby::attach::turned(m.rot, [0.0, 0.0, z]);
}

/// 0x0fc1 (module docs).
pub fn update_0fc1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0, 0);
            m.set_state(1, -1);
        }
        1 => {
            spin(w, id, f32::from_bits(0x3c8e_fa37));
            if let Some(pole) = hit(w, id) {
                let m = &mut w.mobys[id];
                if pole { m.set_pv_i(0, 1); }
                m.set_state(2, -1);
            }
        }
        2 => {
            let m = &w.mobys[id];
            let k = if m.pv_i(0) == 0 { f32::from_bits(0x3e2a_aaab) } else { -f32::from_bits(0x3e2a_aaab) };
            let v = m.rows[0].map(|c| c * k);
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 7, v, 0xff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0fc0 (module docs).
pub fn update_0fc0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(4, 0);
            m.set_state(1, -1);
        }
        1 => {
            let z = -(w.mobys[id].pv_f(0) * f32::from_bits(0x3998_825c));
            spin(w, id, z);
            if let Some(pole) = hit(w, id) {
                let m = &mut w.mobys[id];
                if pole { m.set_pv_i(4, 1); }
                m.set_state(2, -1);
            }
        }
        2 => {
            let k = if w.mobys[id].pv_i(4) == 0 { f32::from_bits(0x3e2a_aaab) } else { -f32::from_bits(0x3e2a_aaab) };
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1fe, [k, 0.0, 0.0], 0xff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
        }
        _ => {}
    }
}
