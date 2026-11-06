//! The prison's floor switch **0x113b** (`0x4310f0`, one placed; the decomp's `func_L14_0042FB10`): a pad that
//! glows and hums until Ratchet steps onto it, then sets the prison's story flag, hooks two mobys and finishes a
//! mission.
//!
//! **Its pvars:** `+0x00` / `+0x04` the two mobys it hooks (moby indices; `+0x00` −1 none), `+0x08` the glow's phase,
//! `+0x0c` pressed, `+0x10` its mission (255 none), `+0x14` its looped sound.
//!
//! **Each tick**: the phase on by 6°, the glow's level `(4·sin − 3)·128` held to 32..128. **0** the set-up: mode
//! `|= 0x10`, 1, no sound. **1** waiting: its class sound 1 looped (`0x36ada0` = L00 `0x3605f0`) while not playing;
//! Ratchet on the ground on it (`0x18a15c`, `0x18a160`) within 0.5 across of joint 0 (`0x31a198` = L00 `0x3127c8`)
//! presses it: 2, the hum stopped, sounds 0 and 2, the flag byte `0x1395e3` (crate::game::GameState::flags `0x2b`)
//! 1, the two mobys hooked (`0x333698` = L00 `0x32a818`), its mission done (`0x31ac78` = L00 `0x3132a8`,
//! crate::game::GameState::mission_done); its colour `+0x78` the level in its first byte. **2** pressed: the level in its
//! second and third bytes.

use crate::hero::physics::dist2;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x113b;

const HOOK_A: usize = 0x00;
const HOOK_B: usize = 0x04;
const PHASE: usize = 0x08;
const PRESSED: usize = 0x0c;
const MISSION: usize = 0x10;
const SOUND: usize = 0x14;

/// `0x1395e3 − 0x1395b8`: the flag byte it sets.
const FLAG: usize = 0x2b;

/// 0x113b (module docs).
pub fn update_113b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return; }
    let m = &mut w.mobys[id];
    let ph = crate::hero::physics::add_rot(m.pv_f(PHASE), f32::from_bits(0x3dd6_7752));
    m.set_pv_f(PHASE, ph);
    let level = (((ph.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(0x20, 0x80) as u32;
    let st = m.state;
    match st {
        0 => {
            m.mode |= 0x10;
            m.set_state(1, -1);
            m.set_pv_i(SOUND, -1);
        }
        1 => {
            let h = m.pv_i(SOUND);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
                w.mobys[id].set_pv_i(SOUND, h);
            }
            let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
            let d = dist2(ctx.hero.pos, at);
            w.mobys[id].set_pv_i(PRESSED, 0);
            if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 && d < 0.5 {
                w.mobys[id].set_state(2, -1);
                let h = w.mobys[id].pv_i(SOUND);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(SOUND, -1);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                if let Some(b) = ctx.game.flags.get_mut(FLAG) { *b = 1; }
                let (a, b) = (w.mobys[id].pv_i(HOOK_A), w.mobys[id].pv_i(HOOK_B));
                if a != -1 {
                    for k in [a, b] {
                        if let Some(o) = usize::try_from(k).ok().and_then(|k| w.mobys.get_mut(k)) { o.once |= 4; }
                    }
                }
                ctx.game.mission_done(w.mobys[id].pv_i(MISSION) as u8);
            }
            w.mobys[id].glow = level | 0x8000_0000;
        }
        2 => {
            if m.pv_i(PRESSED) == 0 { m.set_pv_i(PRESSED, 1); }
            m.glow = level << 16 | level << 8 | 0x8000_0000;
        }
        _ => {}
    }
}
