//! Gorn's convoy ship 0x1216 (L15 `0x438750`, 1 placed; the decomp's `func_L15_00436990`): the ship that flies its
//! path (`+0x268`, the level's paths `0x1fe480`) on the space-AI library ([`crate::moby::classes::space`], the AI
//! state 0xc), waiting at some points.
//!
//! * **Its start** (state 0, once the library's ready flag is set, taken as set): `0x438968` — health 100 (`+0x148`,
//!   `+0x180`), active (`0x37c130(m, 1)`, [`space::start`]), state 1, drawn from anywhere, the rows kept, scale 0.2;
//!   its path speed 10 a second (`+0x278`, which the AI block's `+0x154` points to: the state 0xc's top speed), the
//!   point `+0x26c` 0, `+0x270` / `+0x274` 0, the AI block's `+0x150` 0xffff (`0x379c68`). With its leader word
//!   (`+0x264`) into the leaders' list with the callback `0x438a28` ([`restart`]: its group's activation by the
//!   missions, crate::moby::classes::space::activate_group).
//! * **State 1**: at its goal (`0x3850a0`, [`space::arrived`]) the next point (`+0x26c`); the path's last but one
//!   reached it stops (state 3: nothing more). A point's fourth word not −1 and odd is a wait of (w >> 1) seconds
//!   (state 2, `+0x274`), its goal the next point anyway, the goal's timer 2000000; then the AI state 0xc, the AI tick
//!   ([`space::think`]) and the hits alone (`0x383130(m, 0, 1, 1)`).
//! * **State 2**: the wait; out → 1.
//! * **The path's markers** each tick in states 0 and 1 ([`markers`]): a marker (crate::particles::marker, type 0x29)
//!   at each point of its path, green, for 2 ticks.

use crate::moby::classes::space::{self, a};
use crate::moby::{mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1216;

const PATH: usize = 0x268;
const POINT: usize = 0x26c;
const WAIT: usize = 0x274;
const SPEED: usize = 0x278;

fn path(w: &MobyWorld, id: usize, ctx: &Ctx) -> Vec<[f32; 4]> {
    usize::try_from(w.mobys[id].pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default()
}

/// `0x438a28` (the decomp's `func_L15_00436C68`): at the path's first point going for the second (the point
/// `+0x26c` 1), its velocity 1/12 a tick toward it, the goal's timer 2000000.
pub(crate) fn restart(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let pts = path(w, id, ctx);
    w.mobys[id].set_pv_i(POINT, 1);
    let (Some(p0), Some(p1)) = (pts.first(), pts.get(1)) else { return };
    w.mobys[id].pos = [p0[0], p0[1], p0[2]];
    space::set_goal(w, id, ctx, [p1[0], p1[1], p1[2]]);
    let v = crate::hero::physics::scale(crate::hero::physics::sub(space::ai_v(w, id, a::GOAL), w.mobys[id].pos), 1.0 / 12.0);
    space::set_ai_v(w, id, a::VEL, v);
    space::set_ai_i(w, id, a::TIMER, 2_000_000);
}

/// `0x35b558(m, path, 0x8000ff00, 0x81, 0.75, 1)`: a marker at each point of its path (crate::particles::marker::path).
fn markers(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let points = usize::try_from(w.mobys[id].pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p));
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::marker::path(sys, id, points.map(|v| v.as_slice()), 0x8000_ff00, 0x81, 0.75, 1); }
}

/// `0x438750`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(space::STARTED) == 0 {
        // `0x438968`.
        space::start(w, id, ctx);
        let m = &mut w.mobys[id];
        m.mode |= mode::KEEP_ROWS;
        m.scale = 0.2;
        m.set_pv_f(SPEED, 10.0);
        m.set_pv_i(POINT, 0);
        m.set_pv_i(0x270, 0);
        m.set_pv_i(WAIT, 0);
        space::set_ai_i(w, id, a::SPEED_AT, SPEED as i32 + 1);
        space::set_ai_i(w, id, 0x150, 0xffff);
        markers(w, id, ctx);
        return;
    }
    match w.mobys[id].state {
        1 => {
            markers(w, id, ctx);
            if space::arrived(w, id) {
                let pts = path(w, id, ctx);
                let index = w.mobys[id].pv_i(POINT);
                w.mobys[id].set_pv_i(POINT, index + 1);
                if index + 1 == pts.len() as i32 - 1 {
                    w.mobys[id].state = 3;
                } else {
                    let act = pts.get(index.max(0) as usize).map_or(-1.0, |p| p[3]);
                    if act != -1.0 && act as i32 & 1 != 0 {
                        w.mobys[id].set_pv_i(WAIT, (act as i32 >> 1) * 0x3c);
                        w.mobys[id].state = 2;
                    }
                    if let Some(p) = pts.get((index + 1).max(0) as usize) {
                        space::set_goal(w, id, ctx, [p[0], p[1], p[2]]);
                    }
                    space::set_ai_i(w, id, a::TIMER, 2_000_000);
                }
            }
            space::set_ai_i(w, id, a::STATE, 0xc);
            space::think(w, id, ctx);
            space::hits_with(w, id, ctx, false, true, true);
            w.rebuild(id, &*ctx.host);
        }
        2 => {
            let mut t = w.mobys[id].pv_i(WAIT);
            if crate::moby::countdown(&mut t) { w.mobys[id].state = 1; }
            w.mobys[id].set_pv_i(WAIT, t);
        }
        _ => {}
    }
}
