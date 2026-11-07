//! The ship shack's vendor **0x1309** (L24 `0x37ae10`, the decomp's `func_US2_L24_0037AE10`): the stand that opens the
//! vendor's menu on the ship's kind (game mode 5 with it, `0x325408` = L00 `0x32c5f0(5, 1, moby, 0, 0)`;
//! crate::game::GameState::vendor_open, its kind by its class in `0x374038`: the ship's, 4).
//! * **0**: 1.
//! * **1**: its glow (`0x3231f0` = L00 `0x3272c0`): between `$gp−0x480` (0xff645050) and `$gp−0x47c` (0xff789178) by
//!   half the sine of its phase `+0x0` plus a half, the phase on 2.5° a tick. The prompt 0x12eb ("△ Activate Vendor",
//!   `0x323528` = L00 `0x327a28(1, 0x12eb, 1)`) while Ratchet may use it: in his movement group 0 or 1 (or any in his
//!   state 3), within 3 across and 0.7 up or down, facing it within π/2, not in his states 0x1c / 0x31. △ pressed
//!   (`0x138324 & 0x10`) with the prompt his: mode 5, 2.
//! * **2**: back in play (`0x1a8f00` = 0), 1.
//!
//! Not ported: the menu's ship kind (crate::vendor).

use crate::hero::physics::{atan2, dist2, rot_dist};
use crate::moby::{Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x1309;

const PHASE: usize = 0x0;
/// `$gp−0x480` / `$gp−0x47c`, and the phase's step (`0x3d32b8c4`).
const GLOW: (u32, u32) = (0xff64_5050, 0xff78_9178);
const STEP: f32 = f32::from_bits(0x3d32_b8c4);
const PROMPT: u32 = 0x12eb;
const OWNER: i32 = 1;

/// 0x1309 (module docs).
pub fn update_1309(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let m = &mut w.mobys[id];
            let phase = m.pv_f(PHASE);
            m.glow = crate::hud::lerp_rgba(phase.sin() * 0.5 + 0.5, GLOW.0, GLOW.1);
            m.set_pv_f(PHASE, crate::hero::physics::add_rot(phase, STEP));
            let h = &*ctx.hero;
            let mut ok = h.group == 0 || h.state == 3 || h.group == 1;
            if 3.0 < dist2(m.pos, h.pos) || 0.7 < (h.pos[2] - m.pos[2]).abs() { ok = false; }
            if std::f32::consts::FRAC_PI_2 < rot_dist(h.rot[2], atan2(m.pos[0] - h.pos[0], m.pos[1] - h.pos[1])) { ok = false; }
            if h.state == 0x1c || h.state == 0x31 { ok = false; }
            let mine = ok && ctx.game.prompt.request(OWNER, PROMPT, 1) != 0;
            if ctx.hero.items.pressed & button::TRIANGLE != 0 && mine {
                ctx.game.vendor_open = Some(id);
                w.mobys[id].state = 2;
            }
        }
        2 => {
            if ctx.game.scenes.mode == crate::scene::PLAY && ctx.game.vendor_open.is_none() { w.mobys[id].state = 1; }
        }
        _ => {}
    }
}
