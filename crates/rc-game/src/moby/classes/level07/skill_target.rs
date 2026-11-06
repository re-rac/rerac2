//! 0x0f67 (`0x409480`; the decomp's `func_US2_L07_00409480`): a breakable that gives skill point 0x10 (`0x1a7a78`,
//! crate::skill_points) when it breaks.
//!
//! * **0**: 1.
//! * **1**: with `+0x68`[0] = 0, a hit with [`BREAK_FLAGS`] (`0x3140c0`) breaks it (2), unless the hit's source moby
//!   (the record's `+0x20`) is within 64 (`0x2e50d8`) and the line from it to this one (both half a unit up;
//!   `0x2d4778(.., 2, m, 0)`, this moby left out) meets something. The hit let go every tick (`+0xa8` = −1).
//! * **2**: its class sound 0; its pieces thrown (`0x3219e0` = L00 `0x32b580`, [`shards::throw_all`]: the `$gp+0x24f0`
//!   / `+0x24e8` / `+0x24e0` ranges, modes 0xfff, no sound, none skipped (`$gp+0x2510` = −1), flags 0x10: the level's
//!   bolts and nine more, mask 0xffff); `[1]` 480; the skill point (the award written inline:
//!   crate::game::GameState::award_skill_point); hidden (mode 0x41: `$gp+0x2510` < 0; else its `+0x7c` would keep
//!   that piece shown); 3.
//! * **3**: `[1]` counted down (`0x2e4b88`); out, deleted.
//!
//! Not ported: its 200 particles in state 2 (`0x4096d8` → `0x336a18` = L00 `0x340608`, a particle kind the port lacks:
//! each from a random point 5 across, 10..25 up in its frame (`0x31c0f0`), a random way at 1/60..0.233 a tick plus
//! 1/12 up, 480 ticks) and the level float `0x1aa6b8` it sets to 0.99 for those 480 ticks (kept at `0x1b3118`, put
//! back in state 3).

use crate::hero::physics::{len, sub};
use crate::moby::{countdown, shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f67;

/// The hit flags it answers (`0x3140c0(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;
/// `0x1a7a78` = `0x1a7a68` + 0x10: its skill point.
const POINT: usize = 0x10;
/// The reach within which a wall between the hit's source and it stops the break (`0x2e50d8` < 64).
const REACH: f32 = 64.0;

/// `$gp+0x24f0`: the pieces' velocity ranges (x / y −5..5, z 2..10, a second).
const RANGE: [f32; 8] = [-5.0, -5.0, 2.0, 0.0, 5.0, 5.0, 10.0, 0.0];
/// `$gp+0x24e8`: their spin.
const SPIN: (f32, f32) = (90.0, 360.0);
/// `$gp+0x24e0`: their gravity.
const GRAVITY: (f32, f32) = (10.0, 20.0);
/// `$gp+0x2510`: the piece not thrown (−1 none; nothing writes it).
const SKIP: i32 = -1;
/// `+0x68`[1]: the ticks before it goes.
const LINGER: i32 = 480;

pub fn update_0f67(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.mobys[id].pv_i(0) == 0 {
                if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & BREAK_FLAGS != 0) {
                    let up = |p: [f32; 3]| [p[0], p[1], p[2] + 0.5];
                    let a = up(w.mobys[id].pos);
                    let breaks = match h.tmpl.source.and_then(|s| w.mobys.get(s)) {
                        Some(s) => {
                            let b = up(s.pos);
                            REACH <= len(sub(b, a)) || w.coll_line(ctx, b, a, 2, Some(id)).is_none()
                        }
                        None => true,
                    };
                    if breaks { w.mobys[id].set_state(2, -1); }
                }
            }
            w.clear_hit(id);
        }
        2 => {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xfff, -1, SKIP, 0x10, 0xffff);
            w.mobys[id].set_pv_i(4, LINGER);
            ctx.game.award_skill_point(POINT);
            w.mobys[id].mode |= 0x41;
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            let mut t = w.mobys[id].pv_i(4);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(4, t);
            if out { w.delete(id); }
        }
        _ => {}
    }
}
