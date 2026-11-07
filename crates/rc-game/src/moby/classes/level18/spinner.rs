//! Damosel's turning target **0x10f8** (`0x43b4b8`, one placed; the decomp's `func_L18_00439FC8`). Nothing while the
//! level's variant (`0x1c9cc0`) is not 0. Its pvars: the health block `+0x20` (health 18, its most `+0x24` 0x12), the
//! colour flash `+0x50` (crate::moby::ai::flash), the turn `+0x64` (degrees a second), its looped sound `+0x68`.
//! * **0**: the health and its most set; 1.
//! * **1**: its sound 0 looped (`0x37c8d8` = L00 `0x3605f0`, mode 4) while not playing; its yaw on by `+0x64`; a hit
//!   with the flags 0x230000 (`0x338e08` = L00 `0x31dce8`) doing damage (`0x338e68` = L00 `0x31dd48`,
//!   crate::moby::ai::damage_of): the flash's colour (0xff, 0x40, 0) started (`0x33b248` = L00 `0x320128`) and the
//!   damage off its health; the flash's tick (`0x33b308` = L00 `0x3201e8`); the hit record let go. Nothing here reads
//!   the health.

use crate::hero::physics::add_rot;
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x10f8;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const FLASH: usize = 0x50;
const TURN: usize = 0x64;
const SOUND: usize = 0x68;
const HIT_FLAGS: u32 = 0x23_0000;
/// Degrees a second as radians a tick (`0x3998825c`).
const DEG_TICK: f32 = f32::from_bits(0x3998_825c);

/// 0x10f8 (module docs).
pub fn update_10f8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 0 { return; }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_s(MOST, 0x12);
            m.set_pv_f(HEALTH, 18.0);
            m.set_state(1, -1);
        }
        1 => {
            let h = w.mobys[id].pv_i(SOUND);
            if h == -1 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(SOUND, h);
            }
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], m.pv_f(TURN) * DEG_TICK);
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & HIT_FLAGS != 0);
            let dmg = ai::damage_of(w, id, hit.as_ref(), false);
            let m = &mut w.mobys[id];
            if 0.0 < dmg {
                m.set_pv_b(FLASH + 7, 0xff);
                m.set_pv_b(FLASH + 8, 0x40);
                m.set_pv_b(FLASH + 9, 0);
                ai::flash_start(m, FLASH);
                m.set_pv_f(HEALTH, m.pv_f(HEALTH) - dmg);
            }
            ai::flash(m, FLASH);
            w.clear_hit(id);
        }
        _ => {}
    }
}
