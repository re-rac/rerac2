//! 0x090d (`0x3d1520`, 1 placed; the decomp's `func_L08_003D00C8`): a seesaw tipping about its y angle under
//! Ratchet's weight on either of its two end mobys (`+0x14` / `+0x18`, kept at its joints 0 / 1, `0x3d1498`).
//!
//! * **0**: its tip speed `+8` 0; its ends found (not if deleted); the lock (`+0x10`: moby `+0xc`'s pvar byte
//!   `+0x14` set, `0x3d14f0`); 1.
//! * **1**: Ratchet standing on an end (`0x189e20 + 0x33c`) tips it by `$gp+0x11a0` × cos(tilt) × his xy distance
//!   (`0x2fac20`) / `$gp+0x11b0`, degrees a second² (the other way for the second end); past
//!   ±`$gp+0x11a4`° it is pushed back by `$gp+0x11ac` per `$gp+0x11a8`° beyond. Its tilt on by its speed; crossing
//!   1, 5 or 10° a second, its sound 0. Past `$gp+0x11b4`° with the lock set, stopped and held (2). Its speed ×
//!   `$gp+0x11b8`.
//! * **2**: held until the lock clears, then 1.
//!
//! Not ported: the same push while Ratchet hangs from an end (his mode 3, `+0x588`).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x090d;

/// `$gp+0x11a0`: the push; `+0x11a4` / `+0x11a8` / `+0x11ac` the spring's start, span and strength (degrees);
/// `+0x11b0` the push's divisor; `+0x11b4` the lock's tilt; `+0x11b8` the damping.
const PUSH: f32 = 30.0;
const SPRING: (f32, f32, f32) = (0.0, 20.0, 30.0);
const DIV: f32 = 10.0;
const LOCK: f32 = 0.0;
const DAMP: f32 = 0.97;

const DEG: f32 = 0.017_453_294;
const DEG_S2: f32 = 4.848_137_5e-6;

fn locked(w: &MobyWorld, i: i32) -> bool {
    usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).is_some_and(|m| m.pv_b(0x14) != 0)
}

fn live(w: &MobyWorld, i: i32) -> i32 {
    match usize::try_from(i).ok().and_then(|k| w.mobys.get(k)) { Some(m) if m.state < 0xfd => i, _ => -1 }
}

pub fn update_090d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let (a, b) = (live(w, w.mobys[id].pv_i(0x14)), live(w, w.mobys[id].pv_i(0x18)));
            let lock = locked(w, w.mobys[id].pv_i(0xc));
            let m = &mut w.mobys[id];
            m.set_pv_f(8, 0.0);
            m.set_pv_i(0, a);
            m.set_pv_i(4, b);
            m.set_pv_i(0x10, lock as i32);
            m.set_state(1, -1);
        }
        1 => {
            let on = ctx.hero.ground_moby.map_or(-1, |g| g as i32);
            let m = &w.mobys[id];
            let tilt = m.rot[1];
            let reach = crate::hero::physics::dist2(ctx.hero.pos, m.pos);
            let push = PUSH * DEG_S2 * tilt.cos() * reach / DIV;
            let mut d = 0.0;
            if on != -1 && on == m.pv_i(0) { d = push; }
            if on != -1 && on == m.pv_i(4) { d = -push; }
            let start = SPRING.0 * DEG;
            if start < tilt {
                d -= SPRING.2 * ((tilt - start) / (SPRING.1 * DEG) * DEG_S2);
            } else if tilt < -start {
                d -= SPRING.2 * ((start + tilt) / (SPRING.1 * DEG) * DEG_S2);
            }
            let prev = m.pv_f(8);
            let v = prev + d;
            let m = &mut w.mobys[id];
            m.set_pv_f(8, v);
            m.rot[1] = crate::hero::physics::add_rot(tilt, v);
            let crossed = [0.000_290_888_25, 0.001_454_441_3, 0.002_908_882_7].iter().any(|&x| prev.abs() < x && x < v.abs());
            if crossed { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let lock = locked(w, w.mobys[id].pv_i(0xc));
            let m = &mut w.mobys[id];
            if LOCK * DEG < m.rot[1].abs() && lock {
                m.set_pv_f(8, 0.0);
                m.set_state(2, -1);
            }
            m.set_pv_i(0x10, lock as i32);
            m.set_pv_f(8, m.pv_f(8) * DAMP);
            ends(w, id, ctx);
        }
        2 => if !locked(w, w.mobys[id].pv_i(0xc)) { w.mobys[id].set_state(1, -1) },
        _ => {}
    }
}

/// `0x3d1498`: its two ends at its joints 0 and 1.
fn ends(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    for (k, slot) in [0usize, 4].into_iter().enumerate() {
        let Some(e) = usize::try_from(w.mobys[id].pv_i(slot)).ok().filter(|&e| e < w.mobys.len()) else { continue };
        if let Some(p) = ctx.host.joint(&w.mobys[id], k) { w.mobys[e].pos = p; }
    }
    w.changed = true;
}
