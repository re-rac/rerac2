//! The prison's guard **0x1100** (L14 `0x4284f8`, 1 placed; the decomp's `func_US2_L14_004284F4`), the one the
//! director (crate::moby::classes::level14::director, its `+0x48`) waits on: sparking while Clank is about, it shorts
//! out when hit, the screen going dark until he leaves, then it fades away. Its pvars: `+0x00` the dark's level,
//! `+0x04` done (the director's `0x4287f0`), `+0x08` its looped sound.
//!
//! * **0**: 1, no sound.
//! * **1** (only as Clank, `0x18c0d4` = 1): drawn, its sparks (`0x4288b8`, crate::moby::sparks on L14's data); a hit
//!   (flags 0x10000; any other record let go: `0x325680(m, 0x10000, 0)`) or hooked (`0x3336b8` = L00 `0x32a838`): the
//!   text 0x2f53 for 240 ticks (`0x316988` = L00 `0x30efb8`, crate::prompt::Prompt::announce), 2. Its class sound 0
//!   looped (mode 4, `0x36ada0` = L00 `0x3605f0`) unless its own plays.
//! * **2**: the dark ([`draw`]); its level eased (`0x3264b0` = L00 `0x31eb18`, crate::hero::physics::approach, 1/60 a
//!   tick) toward 0.75 + 0.05 · sin(the frame's share of 30 as a turn − π) (`$gp+0x2770..0x2780`); past 180 ticks
//!   done; its sound stopped; Ratchet back (`0x18c0d4` = 0): the text 0x2f54 for 240 ticks, hidden and untargeted
//!   (mode | 0x41, & ~0x1000), 3.
//! * **3**: the dark; its level toward 0 by 0.333/60 a tick; at 0 deleted.
//!
//! **The dark** ([`draw`], `0x428830`, on the frame's first list `0x2ef098` = L00 `0x2e7718`): the colour from
//! 0x00000040 to 0x80000000 by the level (`0x2f7c08` = L00 `0x2f0000`, crate::hud::lerp_rgba) over the whole screen
//! (`0x303d60(0, 0, 600, 500, colour, 0)` = L00 `0x2fc158` under ALPHA 0x44: crate::moby::MobyWorld::screen_tints).
//!
//! Not ported: the animation word `+0x98` cleared with the hiding (the engine's animation stands in).

use crate::hero::physics::approach;
use crate::moby::{stop_loop, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1100;

const LEVEL: usize = 0x00;
const DONE: usize = 0x04;
const SOUND: usize = 0x08;

/// `$gp+0x2770..0x2780`: the flicker's period (ticks), amplitude, base and rate (a second); `0x1b1774` the fade out's.
const PERIOD: i32 = 30;
const AMP: f32 = 0.05;
const BASE: f32 = 0.75;
const RATE: f32 = 1.0;
const OUT_RATE: f32 = f32::from_bits(0x3eaa_7efa);
const DT: f32 = f32::from_bits(0x3c88_8889);
/// `$gp+0x2788` / `0x1b177c`: the dark's colours at level 0 and 1.
const CLEAR: u32 = 0x40;
const DARK: u32 = 0x8000_0000;

fn clank(ctx: &Ctx) -> bool { ctx.hero.mode == 1 }

/// 0x1100's update (module docs).
pub fn update_1100(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0xc { w.mobys[id].pvars.resize(0xc, 0); }
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            w.mobys[id].set_pv_i(SOUND, -1);
        }
        1 => {
            if !clank(ctx) { return; }
            if w.mobys[id].drawn != 0 { crate::moby::sparks::emit(w, id, ctx, &crate::moby::sparks::L14); }
            let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x1_0000 != 0);
            if !hit { w.clear_hit(id); }
            if hit || w.mobys[id].once & 4 != 0 {
                ctx.game.prompt.announce(0x2f53, 240);
                w.mobys[id].set_state(2, -1);
            }
            let h = w.mobys[id].pv_i(SOUND);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(SOUND, h);
            }
        }
        2 => {
            let a = (ctx.game.frame.rem_euclid(PERIOD) as f32 * std::f32::consts::TAU / PERIOD as f32 - std::f32::consts::PI).sin();
            let m = &mut w.mobys[id];
            let mut l = m.pv_f(LEVEL);
            approach(AMP * a + BASE, RATE * DT, &mut l);
            m.set_pv_f(LEVEL, l);
            w.register_draw_first(id);
            let m = &mut w.mobys[id];
            let before = m.state_ticks as u16;
            m.state_ticks = before.wrapping_add(1) as i16;
            if 181 <= before { m.set_pv_i(DONE, 1); }
            stop_loop(w, id, ctx, SOUND);
            if ctx.hero.mode == 0 {
                ctx.game.prompt.announce(0x2f54, 240);
                let m = &mut w.mobys[id];
                m.mode = (m.mode | 0x41) & 0xefff;
                m.set_state(3, -1);
            }
        }
        3 => {
            w.register_draw_first(id);
            let m = &mut w.mobys[id];
            let mut l = m.pv_f(LEVEL);
            approach(0.0, OUT_RATE * DT, &mut l);
            m.set_pv_f(LEVEL, l);
            if l == 0.0 { w.delete(id); }
        }
        _ => {}
    }
}

/// `0x428830` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize) {
    let l = w.mobys[id].pv_f(LEVEL);
    w.screen_tints.push(crate::hud::lerp_rgba(l, CLEAR, DARK));
}
