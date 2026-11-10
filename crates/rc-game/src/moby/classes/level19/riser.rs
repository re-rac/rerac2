//! Grelbin's riser **0x1271** ([`update_1271`], L19 addresses): a platform that rises into place once a moby is
//! killed. Its platform block at `+0x20` (crate::moby::platform) as the movers' (crate::moby::classes::floe), whose
//! poke of a linked moby it shares.

use crate::hero::physics::{add, sub};
use crate::moby::classes::floe::poke;
use crate::moby::{Ctx, MobyWorld};

/// Its platform block and that block's flags (`+0x5c`, 1 set here).
const BLOCK: usize = 0x20;
const FLAGS: usize = 0x5c;

pub const CLASS: i16 = 0x1271;

/// 0x1271 (`0x4251f0`, 5 placed; the decomp's v1.01 `func_L19_00423EC0`): a platform that rises 6 into place once
/// the moby `+0x7c` has been killed (its uid counted in the save).
/// * **0**: its place kept (`+0x60`). With no moby, or it gone or counted already, it stays and stops updating (mode
///   2, state 4); else updated from any distance, 6 lower, 1.
/// * **1**: the moby's uid counted: the moby `+0x80`'s timer byte 1, 2.
/// * **2**: after `+0x78` ticks (`0x2ec0d0` = L00 `0x2eeab0`), 3.
/// * **3**: it rises over `+0x70` ticks (`(cos πt + 1) / 2` of the 6 left); up, 4.
/// * **4**: once the moby `+0x80`'s timer byte is clear (or none), it stops updating (mode 2).
///
/// Out of 4 its platform block (`+0x20`) gets the tick's move and turn.
///
/// Not ported: the level's state table (`0x21ea30` in v1.01) as a set-off.
pub fn update_1271(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const START: usize = 0x60;
    const DURATION: usize = 0x70;
    const PROGRESS: usize = 0x74;
    const WAIT: usize = 0x78;
    const WATCHED: usize = 0x7c;
    const LINKED: usize = 0x80;
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let watched = usize::try_from(w.mobys[id].pv_i(WATCHED)).ok().and_then(|k| w.mobys.get(k));
    let counted = watched.is_some_and(|o| ctx.game.uid_count(o.uid) != 0);
    match w.mobys[id].state {
        0 => {
            let gone = watched.is_none_or(|o| o.state == 0xfe || o.state == 0xfd);
            let m = &mut w.mobys[id];
            m.set_pv_i(FLAGS, m.pv_i(FLAGS) | 1);
            m.set_pv_f(PROGRESS, 0.0);
            let pos = m.pos;
            m.set_pv_v(START, pos);
            if gone || counted {
                m.mode |= 2;
                m.set_state(4, -1);
            } else {
                m.update_dist = 0xff;
                m.pos[2] -= 6.0;
                m.set_state(1, -1);
            }
        }
        1 => if counted {
            let k = w.mobys[id].pv_i(LINKED);
            poke(w, k);
            w.mobys[id].set_state(2, -1);
        },
        2 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(WAIT);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(WAIT, t);
            if out { m.set_state(3, -1); }
        }
        3 => {
            let m = &mut w.mobys[id];
            let p = (m.pv_f(PROGRESS) + 1.0 / m.pv_f(DURATION)).min(1.0);
            m.set_pv_f(PROGRESS, p);
            let t = ((p * std::f32::consts::PI).cos() + 1.0) * 0.5;
            m.pos = add(m.pv_v(START), [0.0, 0.0, -6.0 * t]);
            if 1.0 <= p { m.set_state(4, -1); }
        }
        4 => {
            let k = w.mobys[id].pv_i(LINKED);
            if usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_none_or(|o| o.timer == 0) { w.mobys[id].mode |= 2; }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state < 4 {
        let (pos, rot) = (m.pos, m.rot);
        crate::moby::platform::write(m, BLOCK, sub(pos, old_pos), old_rot, rot);
    }
    w.changed = true;
}
