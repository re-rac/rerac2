//! The floor button **0x08a1** (L00 `0x3b8de0`, on levels 0, 1, 6, 13, 14, 17, 18, 19 and 20; the decomp's
//! `func_L00_003B79A0`; placed on 6, 13 and 17): it sinks into place, glows until Ratchet stands on it (or its mission
//! is done), then counts the press in the level's uid bits.
//!
//! **Its pvars:** `+0x00` a path (−1 none), `+0x04` its tag, `+0x08` pressed too by its mission (`+0xb0`) being done,
//! `+0x0c` the glow's phase.
//!
//! * **0**: the phase random, 0.35 down; pressed before — its uid gone for good (L06 `0x21ecc4`, L00 `0x21e1c4`:
//!   crate::game::GameState::uid_dead), its uid count not 0, or by its mission and that done — it is pressed (2, its
//!   timer byte `+0xbc` 2, its colour `0x80208020`); else 1.
//! * **1**: the glow pulses (6° a tick: `(sin·4 − 3)·128` within 32..128, grey) until Ratchet stands on it on the ground
//!   (`0x18a15c` it, `0x18a16e` 0) or its mission is done (with `+0x08`): its uid counted (at most 7), its timer byte 1,
//!   2, its colour `0x80208020` and its class sound 0.
//!
//! Not ported: on each press its path's points whose `w` is its tag get `w` 0 (the level's paths `0x1fe500`; no port
//! reads the points' `w`).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x08a1;

fn pressed(w: &mut MobyWorld, id: usize, timer: u8) {
    let m = &mut w.mobys[id];
    m.state = 2;
    m.timer = timer;
    m.glow = 0x8020_8020;
}

/// 0x08a1 (module docs).
pub fn update_08a1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let ph = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0xc, ph);
            m.pos[2] -= 0.35;
            let (uid, mission, by_mission) = (m.uid, m.mission as i32, m.pv_i(8) != 0);
            let dead = ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0;
            if dead || ctx.game.uid_count(uid) != 0 || (by_mission && crate::moby::director::mission_is_done(&*ctx.game, mission)) {
                pressed(w, id, 2);
            } else {
                w.mobys[id].state = 1;
            }
            w.changed = true;
        }
        1 => {
            let m = &mut w.mobys[id];
            let ph = crate::hero::physics::add_rot(m.pv_f(0xc), f32::from_bits(0x3dd6_7752));
            m.set_pv_f(0xc, ph);
            let k = (((ph.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(0x20, 0x80) as u32;
            m.glow = k << 16 | k << 8 | 0x8000_0000 | k;
            let on = ctx.hero.ground_moby == Some(id) && ctx.hero.air_ticks == 0;
            let (uid, mission, by_mission) = (m.uid, m.mission as i32, m.pv_i(8) != 0);
            if !on && !(by_mission && crate::moby::director::mission_is_done(&*ctx.game, mission)) { return; }
            ctx.game.count_uid(uid);
            pressed(w, id, 1);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
        }
        _ => {}
    }
}
