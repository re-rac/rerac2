//! 0x0e30 (`0x3f6d60`, 1 placed; the decomp's `func_L07_003F59C0`): a block the Tractor Beam drags along its path
//! (`+0xe0`), riding a lift (`+0xe8`) in cuboid `+0xec`, a platform Ratchet stands on (its block at `+0xa0`).
//!
//! * **0**: shown, its drag's range 9 (`+0x88`), `+0xdc` bit 1; 1. **1**: the drag (`0x31c940` = L01 `0x31bd40`,
//!   crate::moby::classes::tractor::drag: radius `$gp+0x1f0c`), then [`keep`] (`0x3f6ef0`).
//! * Every tick: its scale `$gp+0x1f14` of its class's; drawn, its glow word (`+0x78`) pulsing between `$gp+0x1f00` / `+0x1f04`
//!   once a second (`0x31da88` = L01 `0x32c0b8`); its platform block written with the tick's move and turn.
//!
//! Not ported: [`keep`]'s push of Ratchet off the lift while it climbs past 202..204 with the camera outside the
//! cuboid (`0x2b7e90(pos, angles, 0x6f, 1)`, a hero state the port's teleport does not take).

use crate::hero::physics::{sub, V3};
use crate::moby::{platform, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e30;

/// `$gp+0x1f00` / `+0x1f04`: the pulse's colours.
const PULSE: (u32, u32) = (0x8080_8080, 0x8040_4040);
/// `$gp+0x1f0c`: the drag's radius; `+0x1f14` its scale against its class's; `+0x1f18` its height over the lift.
const RADIUS: f32 = 0.75;
const SCALE: f32 = 0.5;
const OVER: f32 = 0.3;

pub fn update_0e30(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let s = w.class(CLASS).map_or(1.0, |c| c.scale) * SCALE;
    w.mobys[id].scale = s;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= 0x1000;
            m.set_pv_f(0x88, 9.0);
            m.set_pv_i(0xdc, m.pv_i(0xdc) | 1);
            m.set_state(1, -1);
        }
        1 => {
            let path = w.mobys[id].pv_i(0xe0);
            crate::moby::classes::tractor::drag(w, id, ctx, RADIUS, path);
            keep(w, id, ctx);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.drawn != 0 {
        let phase = (ctx.game.frame % 60) as f32 * std::f32::consts::TAU / 60.0;
        m.glow = crate::moby::classes::electrolyzer::pulse_colour(phase - std::f32::consts::PI, PULSE.0, PULSE.1);
    }
    let (pos, rot) = (m.pos, m.rot);
    platform::write(m, 0xa0, sub(pos, old_pos), old_rot, rot);
}

/// `0x3f6ef0`: kept off the lift's sides — while the lift is below 218.5 (above 199.7), within 0.75 of 4 behind
/// (ahead of) the cuboid's y it is pushed out by up to 1/6 a tick, its drag's y speed stopped — and, inside the
/// cuboid, set 0.3 over the lift, its drag's z speed stopped.
fn keep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let lift = usize::try_from(w.mobys[id].pv_i(0xe8)).ok().filter(|&i| i < w.mobys.len());
    let Some(lift) = lift else { return };
    let cuboid = w.mobys[id].pv_i(0xec);
    let Some((c, _)) = ctx.level.cuboid_place(cuboid) else { return };
    let lz = w.mobys[lift].pos[2];
    let m = &mut w.mobys[id];
    let diff = m.pos[1] - c[1];
    for (on, off) in [(lz < 218.5, 4.0), (199.7 < lz, -4.0)] {
        if !on { continue; }
        let step = diff + off;
        if RADIUS <= step.abs() { continue; }
        let push = if step < 0.0 { -(step + RADIUS) } else { RADIUS - step };
        let push = push.clamp(-1.0 / 6.0, 1.0 / 6.0);
        m.pos[1] += push;
        m.set_pv_f(0x54, 0.0);
        m.set_pv_f(0x64, m.pv_f(0x64) + push);
    }
    let p: V3 = m.pos;
    if ctx.level.in_cuboid(cuboid, p) {
        m.pos[2] = lz + OVER;
        m.set_pv_f(0x68, m.pos[2]);
        m.set_pv_f(0x58, 0.0);
    }
}
