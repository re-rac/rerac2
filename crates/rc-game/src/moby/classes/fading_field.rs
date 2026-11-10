//! The fading field **0x119b** ([`update_119b`], L20 `0x43fe58`, 11 placed) and level 19's **0x1204** (L19
//! `0x423750`, 4 placed): the decomp's `func_L19_00422420`, the same code on the two levels. A field that fades away.
//! Its pvars: `+0x00` a moby it watches (−1 none), `+0x04` its fade, `+0x08` its scroll, `+0x0c` its size (its scale
//! is that over 9.77; 6.2 for a 0x1204), `+0x14` its loop sound.
//! * **0**: alpha 0x50, mode | 0xb0c, its first row and its sphere scaled to its size, its second row by 0.8, the
//!   ambient 0x80 grey; its mission done: 3.
//! * **1**: its loop sound 0 kept playing. The watched moby's uid counted (or the moby dead), or with none watched
//!   hooked: 2.
//! * **2**: its sound stopped; it fades by 0.05 a tick (alpha 80 times the fade); faded out, 3.
//! * **3**: its uid marked gone (crate::game::GameState::uid_gone), deleted.
//!
//! Not ported: the point light `+0x0a` freed at its end (`0x312160` = L00 `0x2fdc48`; this code never takes one).

use crate::hero::physics::scale;
use crate::moby::{Ctx, MobyWorld};

/// The level's mission `mission` done (`0x19b2e8 + level·0x10`, bit 7).
fn mission_done(ctx: &Ctx, mission: u8) -> bool {
    let g = &ctx.game;
    mission != 0xff && g.missions.get(g.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0)
}

fn live(w: &MobyWorld, k: i32) -> Option<usize> {
    usize::try_from(k).ok().filter(|&k| w.mobys.get(k).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}

pub const FIELD: i16 = 0x119b;
/// L19's class on the same code (its size's base 6.2).
pub const FIELD_L19: i16 = 0x1204;

/// 0x119b (module docs).
pub fn update_119b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let base = if w.mobys[id].o_class == FIELD_L19 { 6.2 } else { 9.77 };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x14, -1);
            m.alpha = 0x50;
            m.state = 1;
            m.mode |= 0xb0c;
            let s = m.pv_f(0xc) / base;
            m.rows[0] = scale(m.rows[0], s);
            m.rows[1] = scale(m.rows[1], 0.8);
            m.bsphere[3] *= s;
            m.set_pv_s(0xa, -1);
            m.set_pv_f(4, 1.0);
            m.ambient = [0x80; 3];
            if mission_done(ctx, m.mission) { m.set_state(3, -1); }
        }
        1 => {
            let h = w.mobys[id].pv_i(0x14);
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x14, h);
            }
            let watched = w.mobys[id].pv_i(0);
            let on = if watched == -1 {
                w.mobys[id].once & 4 != 0
            } else {
                // The watched moby's uid counted (`0x19b4a8 + level·0x400`, its nibble's low three bits); dead, on.
                live(w, watched).is_none_or(|k| ctx.game.uid_count(w.mobys[k].uid) != 0)
            };
            if on {
                let m = &mut w.mobys[id];
                m.set_pv_f(4, 1.0);
                m.state = 2;
            }
        }
        2 => {
            let h = w.mobys[id].pv_i(0x14);
            if h >= 0 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            let m = &mut w.mobys[id];
            let f = m.pv_f(4) - 0.05;
            m.set_pv_i(0x14, -1);
            m.set_pv_f(4, f);
            if f < 0.0 { m.state = 3; } else { m.alpha = (f * 80.0) as i32 as u8; }
        }
        3 => {
            let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
            ctx.game.uid_gone(uid, mission);
            w.delete(id);
            return;
        }
        _ => {}
    }
    // Its scroll: on by 0x10 a tick, back by 0x1000 past it; its class's texture coordinates moved the same
    // (`0x326a20` = L00 `0x310838`: 0x10 on s, −0xff0 at the wrap; every placed one moves the shared class).
    let m = &mut w.mobys[id];
    let (mut s, mut step) = (m.pv_s(8).wrapping_add(0x10), 0x10);
    if 0x1000 < s { s -= 0x1000; step = -0xff0; } else if s < 0 { s += 0x1000; step = 0x1010; }
    m.set_pv_s(8, s);
    let class = m.o_class;
    w.scroll_class(class, step, 0);
}
