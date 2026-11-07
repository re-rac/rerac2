//! Notak's hint manager **0x0cdd** (L06 `0x4321e8`, one placed; the decomp's `func_L06_00430C60`): help texts by
//! where Ratchet is (crate::game::GameState::help, `0x334848` = L00 `0x2f5270`; the slots `0x18c958 + 0xc·slot`:
//! crate::game::GameState::help_counts / help_last / help_levels). Its pvars: the cuboids `+0x00`, `+0x04`, `+0x08`,
//! `+0x18`, `+0x1c`, `+0x34`; `+0x10` the times a crank was let go short of its end, `+0x14` a timer, `+0x2c` a crank
//! taken in `+0x08`, `+0x30` the Thermanator used in `+0x00`. "Idle" is the help panel with nothing up or asked for
//! (`0x287ac8`, `+0x24` −1). "As before past N" is: asked past N ticks of the slot's last showing (`play_time − last`
//! unsigned, `0x1a7390`) or with none, else its last showing now.
//!
//! **0**: updated at any distance (`+0x30` 0xff), `+0x2c` / `+0x10` 0; 1. **1**, every tick in this order:
//! * in `+0x00` with the panel idle and the Thermanator (item 0x27, `0x1a7b27`) not had: 0x9df (0x36) as before past
//!   1080;
//! * in `+0x00`, its uid not gone for good (`0x21ecc4`, crate::game::GameState::uid_dead) nor counted: the Thermanator
//!   in use (`0x18c2d4` 0x27) sets `+0x30` and counts its uid (at most 7); then with the panel idle and `+0x30` not
//!   set, slot 0x37 — shown before: 0x9e0 as before past 1080; never: counted as shown (its last showing now, this
//!   level's bit and bit 31);
//! * in `+0x08`: Ratchet in his state 0x37 sets `+0x2c`; out of it with `+0x2c`, each wrench crank 0x0b57 of the run
//!   list (`0x1b2828`, crate::moby::MobyWorld::run_list) within 3 of him short of its end clears `+0x2c` and counts
//!   `+0x10`: at 3 and 6 0x9e2 (0x39);
//! * in his movement group 0x12 with the panel idle in `+0x04`: `+0x14` counted, past 240 (still in it) 0x9db (0x32)
//!   while the slot was never dismissed; else in the group 0x11 the slot off, otherwise `+0x14` 0;
//! * in `+0x18`: the level's flag (`$gp+0x1b78`) set; in `+0x1c` with the panel idle and that flag: slot 0x38 shown
//!   before, 0x9e1 (0x38); never: counted as shown;
//! * past 324000 ticks of play since the map's last use (`0x18c7c0`) in `+0x34`, slot 0x0c not shown on this level:
//!   0x9b5 (0x0c).
//!
//! Not ported: the map's last use `0x18c7c0` (G-HERO-005; read as never, as crate::moby::classes::endako_b does).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0cdd;

const LET_GO: usize = 0x10;
const TIMER: usize = 0x14;
const TAKEN: usize = 0x2c;
const USED: usize = 0x30;
/// The Thermanator.
const THERMANATOR: usize = 0x27;

/// 0x0cdd (module docs).
pub fn update_0cdd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.state = 1;
            m.set_pv_i(TAKEN, 0);
            m.set_pv_i(LET_GO, 0);
            return;
        }
        1 => {}
        _ => return,
    }
    let pv = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(k);
    let inside = |ctx: &Ctx, c: i32| ctx.level.in_cuboid(c, ctx.hero.pos);
    let idle = |ctx: &Ctx| ctx.game.help_state == 0 && ctx.game.help_req.is_none();
    let now = ctx.game.play_time;
    let level_bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
    let again = |ctx: &mut Ctx, text: i32, slot: usize| {
        let old = ctx.game.help_last[slot];
        if 1080 < now.wrapping_sub(old) as u32 || old == 0 {
            ctx.game.help(text, slot as i32);
        } else if (old as u32) < now as u32 {
            ctx.game.help_last[slot] = now;
        }
    };
    // Slot `slot` counted as shown (`+0` up by one, `+4` now, `+8` this level's bit and bit 31).
    let first = |ctx: &mut Ctx, slot: usize| {
        let g = &mut *ctx.game;
        g.help_counts[slot] = g.help_counts[slot].wrapping_add(1);
        if (g.help_last[slot] as u32) < now as u32 { g.help_last[slot] = now; }
        g.help_levels[slot] |= level_bit | 0x8000_0000;
    };
    let had = ctx.game.items.get(THERMANATOR).is_some_and(|&b| b != 0);
    if inside(ctx, pv(w, 0)) && idle(ctx) && !had { again(ctx, 0x9df, 0x36); }
    if inside(ctx, pv(w, 0)) {
        let uid = w.mobys[id].uid;
        let dead = ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0;
        if !dead && ctx.game.uid_count(uid) == 0 {
            if ctx.hero.items.use_item == THERMANATOR as i32 {
                w.mobys[id].set_pv_i(USED, 1);
                ctx.game.count_uid(uid);
            }
            if idle(ctx) && pv(w, USED) == 0 {
                if ctx.game.help_counts[0x37] != 0 { again(ctx, 0x9e0, 0x37) } else { first(ctx, 0x37) }
            }
        }
    }
    if inside(ctx, pv(w, 8)) {
        if ctx.hero.state == 0x37 { w.mobys[id].set_pv_i(TAKEN, 1); }
        if pv(w, TAKEN) != 0 && ctx.hero.state != 0x37 {
            for k in w.run_list(ctx.host.camera_pos()) {
                let c = &w.mobys[k];
                let near = c.o_class == super::crank::CLASS && crate::hero::physics::dist(c.pos, ctx.hero.pos) < 3.0 && c.pv_f(0) != 1.0;
                if near && pv(w, TAKEN) != 0 {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(TAKEN, 0);
                    let n = m.pv_i(LET_GO) + 1;
                    m.set_pv_i(LET_GO, n);
                    if n == 3 || n == 6 { ctx.game.help(0x9e2, 0x39); }
                }
            }
        }
    }
    if ctx.hero.group == 0x12 && idle(ctx) && inside(ctx, pv(w, 4)) {
        let n = pv(w, TIMER) + 1;
        w.mobys[id].set_pv_i(TIMER, n);
        if inside(ctx, pv(w, 4)) && 240 < n && ctx.game.help_counts[0x32] == 0 { ctx.game.help(0x9db, 0x32); }
    } else if ctx.hero.group == 0x11 {
        ctx.game.help_counts[0x32] = -1;
    } else {
        w.mobys[id].set_pv_i(TIMER, 0);
    }
    if inside(ctx, pv(w, 0x18)) { w.level06.hint_seen = 1; }
    if inside(ctx, pv(w, 0x1c)) && idle(ctx) && w.level06.hint_seen != 0 {
        if ctx.game.help_counts[0x38] != 0 { ctx.game.help(0x9e1, 0x38); } else { first(ctx, 0x38); }
    }
    // `now − 0x18c7c0` (the map's last use, none) past 324000 ticks.
    if 324_000 < now as u32 && inside(ctx, pv(w, 0x34)) && ctx.game.help_levels[0xc] & level_bit == 0 {
        ctx.game.help(0x9b5, 0xc);
    }
}
