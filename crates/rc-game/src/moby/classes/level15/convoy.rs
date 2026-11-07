//! Gorn's convoy ship 0x1216 (L15 `0x438750`, 1 placed; the decomp's `func_L15_00436990`): the ship that flies its
//! path (`+0x268`, the level's paths `0x1fe480`) on the space-AI library ([`crate::moby::classes::space`], the AI
//! state 0xc), waiting at some points.
//!
//! * **Its start** (state 0, once the library's ready flag is set, taken as set): `0x438968` — health 100 (`+0x148`,
//!   `+0x180`), active (`0x37c130(m, 1)`, [`space::start`]), state 1, drawn from anywhere, the rows kept, scale 0.2;
//!   its path speed 10 a second (`+0x278`, which the AI block's `+0x154` points to: the state 0xc's top speed), the
//!   point `+0x26c` 0, `+0x270` / `+0x274` 0, the AI block's `+0x150` 0xffff (`0x379c68`). With its leader word
//!   (`+0x264`) into the leaders' list with the callback `0x438a28` (its group's activation by the mission puts it
//!   at the path's first point going for the second, its velocity 1/12 a tick toward it: not ported with the
//!   missions' activation `0x37b280`).
//! * **State 1**: at its goal (`0x3850a0`, [`space::arrived`]) the next point (`+0x26c`); the path's last but one
//!   reached it stops (state 3: nothing more). A point's fourth word not −1 and odd is a wait of (w >> 1) seconds
//!   (state 2, `+0x274`), its goal the next point anyway, the goal's timer 2000000; then the AI state 0xc, the AI tick
//!   ([`space::think`]) and the hits alone (`0x383130(m, 0, 1, 1)`).
//! * **State 2**: the wait; out → 1.
//!
//! Not ported: the path's markers each tick in states 0 and 1 (`0x35b558(m, path, 0x8000ff00, 0x81, 0.75, 1)`: a
//! particle of type 0x29 at each point, its frame from `0x1ff364`, 2 ticks; crate::particles has not the type).

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
        return;
    }
    match w.mobys[id].state {
        1 => {
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
