//! Todano's hint manager **0x0c2e** (`0x3ec848`, one placed; the decomp's `func_L12_003EB470`): help texts by where
//! Ratchet is (crate::game::GameState::help, `0x2f8958` = L00 `0x2f5270`; the slots `0x18c958 + 0xc·slot`). Its pvars:
//! the cuboids `+0x0` / `+0x4` / `+0x8`, the moby `+0xc` (the box 0x0fb8) and the countdown `+0x10`.
//! * **0**: updated at any distance, the countdown −1; 1.
//! * **1**, each tick:
//!   - in `+0x0` while the Tractor Beam holds something (`0x18a3e0`, crate::moby::classes::tractor::held) with slot
//!     0x79 never dismissed: 0xa22 (0x79);
//!   - in `+0x4` without item 0x33 (`0x1a7b33`): slot 0x5c — counted once (its time and this level marked) the first
//!     time, then 0xa05 again past 0x438 ticks of its last (or with none), else its time kept;
//!   - in `+0x8` with the moby `+0xc` a live 0x0fb8: the countdown 0x2a30 when off; at 0, slot 0x5d not shown on this
//!     level: 0xa06 (0x5d);
//!   - with item 7 (`0x1a7b07`) and Ratchet not held (state 0x6f), slot 0x7f not shown on this level: 0xa28 (0x7f);
//!   - the countdown run down while not off (`0x2f2190`, crate::moby::countdown).

use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c2e;

const BOX: i16 = 0x0fb8;
const T: usize = 0x10;

fn had(ctx: &Ctx, item: usize) -> bool { ctx.game.items.get(item).is_some_and(|&b| b != 0) }

/// Slot `k` shown on this level (`0x18c958 + 0xc·k + 8`, the level's bit).
fn shown_here(ctx: &Ctx, k: usize) -> bool {
    let bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
    ctx.game.help_levels.get(k).is_some_and(|&l| l & bit != 0)
}

/// 0x0c2e (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_pv_i(T, -1);
            m.set_state(1, -1);
        }
        1 => {
            let pv = |k: usize| w.mobys[id].pv_i(k);
            let inside = |ctx: &Ctx, c: i32| ctx.level.in_cuboid(c, ctx.hero.pos);
            if inside(ctx, pv(0)) && crate::moby::classes::tractor::held(w).is_some() && ctx.game.help_counts.get(0x79).copied().unwrap_or(-1) == 0 {
                ctx.game.help(0xa22, 0x79);
            }
            if inside(ctx, pv(4)) && !had(ctx, 0x33) {
                let now = ctx.game.play_time;
                let bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
                let g = &mut *ctx.game;
                let last = g.help_last[0x5c];
                if g.help_counts[0x5c] != 0 {
                    if 0x438 < now.wrapping_sub(last) as u32 || last == 0 {
                        g.help(0xa05, 0x5c);
                    } else if (last as u32) < now as u32 {
                        g.help_last[0x5c] = now;
                    }
                } else {
                    g.help_counts[0x5c] = 1;
                    if (last as u32) < now as u32 { g.help_last[0x5c] = now; }
                    g.help_levels[0x5c] |= bit | 0x8000_0000;
                }
            }
            if inside(ctx, pv(8)) {
                let k = usize::try_from(pv(0xc)).ok().filter(|&k| k < w.mobys.len());
                if k.is_some_and(|k| w.mobys[k].o_class == BOX && w.mobys[k].state != 0xfe && w.mobys[k].state != 0xfd) {
                    let t = pv(T);
                    if t < 0 {
                        w.mobys[id].set_pv_i(T, 0x2a30);
                    } else if t == 0 && !shown_here(ctx, 0x5d) {
                        ctx.game.help(0xa06, 0x5d);
                    }
                }
            }
            if had(ctx, 7) && ctx.hero.state != 0x6f && !shown_here(ctx, 0x7f) {
                ctx.game.help(0xa28, 0x7f);
            }
            let mut t = w.mobys[id].pv_i(T);
            if 0 <= t {
                countdown(&mut t);
                w.mobys[id].set_pv_i(T, t);
            }
        }
        _ => {}
    }
}
