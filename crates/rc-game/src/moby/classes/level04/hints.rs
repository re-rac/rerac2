//! Barlow's hint manager, **0x0c26** ([`update_0c26`], L04 `0x423a38`, one placed; the decomp's
//! `func_L04_00422318`): the level's help texts by where Ratchet is and what he has, read from its pvars (`+0x68`,
//! words): the cuboids `+0x00`, `+0x04`, `+0x08`, `+0x0c`, `+0x14`, `+0x20`, `+0x28`, `+0x2c`, `+0x40`, `+0x44`,
//! `+0x48`; `+0x1c` the liquid grid 0x0960 the Thermanator freezes (crate::water::grid, its frozen byte `+0x14`);
//! `+0x10` / `+0x18` two timers. "Idle" below is the help panel with nothing up or asked for (`0x2967c8`, `+0x24` −1).
//!
//! **0**: updated at any distance, `+0x10` 2400, 1. **1**, every tick in this order:
//! * 0x9d7 (slot 0x2e) in `+0x00` with the panel idle, and the slot off after (asked or not);
//! * in `+0x0c` with the panel idle and Clank not on Ratchet's back (item 2, `0x1a7b02`): `+0x10` counted, and past
//!   1200 0x9de (0x35) asked, `+0x10` 0;
//! * 0x9cc (0x23) in `+0x28` with the panel idle, Clank on his back, Ratchet in his states 0 or 2 and `+0x40` not yet
//!   reached on this level (`0x13962c`, a bit per level): asked past 300 ticks of its last showing (or never shown),
//!   else its last showing now;
//! * `+0x40` reached: `0x13962c` its level's bit;
//! * 0x9da (0x31) in `+0x08` with the panel idle, item 0xd not had (`0x1a7b0d`) and the slot never shown;
//! * in his movement group 0x12 with the panel idle in `+0x14`: `+0x18` counted, and past 240 0x9db (0x32) as
//!   0x9cc's (1080 ticks); else in the group 0x11 the slot off, otherwise `+0x18` 0;
//! * 0xa3c (0x93, the missions on the map) queued (`0x3173a0` = L00 `0x2f66b0`) once (`0x13962d` bit 4) in `+0x44`
//!   with Clank away from his back (`0x18c31c`);
//! * 0x9c9 (0x20) in `+0x48` unless shown on this level;
//! * with the Thermanator (item 0x27): 0x9dc (0x33) in `+0x2c` as 0x9cc's (1080 ticks), and the slot off once the
//!   Thermanator in the hand has its arc on something not ice (its pvars `+0x54` / `+0x5c`); with the grid `+0x1c`:
//!   in `+0x20` while it is not frozen and it has not been frozen yet — the slot shown before: 0x9dd (0x34) as
//!   0x9cc's (1080 ticks); never: counted as shown (once, its last showing now, its level's bit); either way the
//!   tick ends there — else the grid frozen once it had been seen thawed turns the slot off for good
//!   (`$gp+0x1cec` / `+0x1cf0`, here the pvars `+0x50` / `+0x54`);
//! * 0x9b5 (0x0c) in `+0x04` unless shown on this level.
//!
//! The asks are `0x315f60` (crate::game::GameState::help). Each "past N ticks of its last showing" is
//! `play_time − last > N` unsigned (`0x1a7390`; the slot's `+4`, crate::game::GameState::help_last).
//!
//! Not ported: the stats some ask (`0x18c8c4`, `0x18c918` / `0x18c91c`, `0x18c7c0`, the one before 0x9b5: G-HERO-005,
//! read as none, as crate::moby::classes::maktar_hints does — each of those hints shows as to a player who never
//! did the thing); the short `0x193f6c` 0x9d7 asks (the help record of the text's number, `0x18c958 + 0xc·0x9d7`,
//! past the table: read as 0); the sun's lens flare (`0x382ce8` = L00's, its 14 sprites `0x422da8` on the first
//! list, their brightness by a depth query `0x422a00` at the sun's place (80000, −70000, 140000)).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c26;
/// The Thermanator.
const THERMANATOR: usize = 0x27;
/// The port's: `$gp+0x1cec` (the slot 0x34 off once the grid is frozen) and `$gp+0x1cf0` (the grid seen thawed).
const FROZE: usize = 0x50;
const THAWED: usize = 0x54;

/// 0x0c26 (`0x423a38`): see the module.
pub fn update_0c26(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pvars.len() < 0x58 { m.pvars.resize(0x58, 0); }
            m.update_dist = 0xff;
            m.set_pv_i(0x10, 2400);
            m.set_state(1, -1);
            return;
        }
        1 => {}
        _ => return,
    }
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(k);
    let inside = |ctx: &Ctx, z: i32| ctx.level.in_cuboid(z, ctx.hero.pos);
    let idle = |ctx: &Ctx| ctx.game.help_state == 0 && ctx.game.help_req.is_none();
    let have = |ctx: &Ctx, item: usize| ctx.game.items.get(item).is_some_and(|&b| b != 0);
    let level = ctx.game.level.max(0) as u32;
    let level_bit = 1u32.checked_shl(level).unwrap_or(0);
    let now = ctx.game.play_time;
    // `0x18c958 + 0xc·slot`: shown past `n` ticks of its last showing (or never), else its last showing now.
    let again = |ctx: &mut Ctx, text: i32, slot: usize, n: u32| {
        let old = ctx.game.help_last[slot];
        if n < (now.wrapping_sub(old) as u32) || old == 0 {
            ctx.game.help(text, slot as i32);
        } else if old < now {
            ctx.game.help_last[slot] = now;
        }
    };
    if inside(ctx, pv(w, 0)) && idle(ctx) {
        ctx.game.help(0x9d7, 0x2e);
        ctx.game.help_counts[0x2e] = -1;
    }
    if inside(ctx, pv(w, 0xc)) && idle(ctx) && !ctx.game.clank {
        let n = pv(w, 0x10) + 1;
        w.mobys[id].set_pv_i(0x10, n);
        if 1200 < n {
            ctx.game.help(0x9de, 0x35);
            w.mobys[id].set_pv_i(0x10, 0);
        }
    }
    let reached = ctx.game.flags.get(0x74).is_some_and(|&b| (b >> level) & 1 != 0);
    if inside(ctx, pv(w, 0x28)) && idle(ctx) && ctx.game.clank && matches!(ctx.hero.state, 0 | 2) && !reached {
        again(ctx, 0x9cc, 0x23, 300);
    }
    if inside(ctx, pv(w, 0x40)) {
        if let Some(b) = ctx.game.flags.get_mut(0x74) { *b |= level_bit as u8; }
    }
    if inside(ctx, pv(w, 8)) && idle(ctx) && !have(ctx, 0xd) && (ctx.game.help_levels[0x31] as i32) >= 0 {
        ctx.game.help(0x9da, 0x31);
    }
    if ctx.hero.group == 0x12 && idle(ctx) && inside(ctx, pv(w, 0x14)) {
        let n = pv(w, 0x18) + 1;
        w.mobys[id].set_pv_i(0x18, n);
        if 240 < n { again(ctx, 0x9db, 0x32, 1080); }
    } else if ctx.hero.group == 0x11 {
        ctx.game.help_counts[0x32] = -1;
    } else {
        w.mobys[id].set_pv_i(0x18, 0);
    }
    if ctx.game.hint_flags[0] & 0x10 == 0 && inside(ctx, pv(w, 0x44)) && ctx.hero.body.clank_away {
        ctx.game.hint_flags[0] |= 0x10;
        ctx.game.help_later(0xa3c, 0x93);
    }
    if inside(ctx, pv(w, 0x48)) && (ctx.game.help_levels[0x20] >> level) & 1 == 0 {
        ctx.game.help(0x9c9, 0x20);
    }
    if have(ctx, THERMANATOR) && inside(ctx, pv(w, 0x2c)) {
        again(ctx, 0x9dc, 0x33, 1080);
        if ctx.hero.items.item == THERMANATOR as i32 {
            let arc = ctx.hero.items.hand.and_then(|h| w.mobys.get(h)).is_some_and(|h| h.pv_i(0x54) != 0 && h.pv_i(0x5c) == 0);
            if arc { ctx.game.help_counts[0x33] = -1; }
        }
    }
    let grid = usize::try_from(pv(w, 0x1c)).ok().filter(|_| have(ctx, THERMANATOR)).and_then(|g| w.mobys.get(g)).map(|g| g.pv_b(0x14) != 0);
    if let Some(frozen) = grid {
        if inside(ctx, pv(w, 0x20)) && !frozen && pv(w, FROZE) == 0 {
            let g = &mut *ctx.game;
            if g.help_counts[0x34] != 0 {
                again(ctx, 0x9dd, 0x34, 1080);
            } else {
                if (g.help_counts[0x34] as u16) < 0xffff { g.help_counts[0x34] = g.help_counts[0x34].wrapping_add(1); }
                if g.help_last[0x34] < now { g.help_last[0x34] = now; }
                g.help_levels[0x34] |= level_bit | 0x8000_0000;
            }
            return;
        }
        if frozen && pv(w, THAWED) != 0 {
            w.mobys[id].set_pv_i(FROZE, 1);
            ctx.game.help_counts[0x34] = -1;
        } else if !frozen {
            w.mobys[id].set_pv_i(THAWED, 1);
        }
    }
    if inside(ctx, pv(w, 4)) && (ctx.game.help_levels[0xc] >> level) & 1 == 0 {
        ctx.game.help(0x9b5, 0xc);
    }
}
