//! Level 0x09's director (Dobbo; L09 addresses).

use crate::moby::director::{Census, Tally};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c2b;

/// The four cuboids (pvars `+0x00..+0x0c`), Ratchet in the second since the level started (`+0x10`), the three hints'
/// once-flags (`+0x14` / `+0x18` / `+0x1c`), the census (`+0x58` its cursor, `+0x5c` its count).
const REGIONS: usize = 0;
const ENTERED: usize = 0x10;
const HINT_14: usize = 0x14;
const HINT_18: usize = 0x18;
const HINT_1C: usize = 0x1c;
const F2C: usize = 0x2c;
const CURSOR: usize = 0x58;
const COUNT: usize = 0x5c;

/// The flag bytes it counts in (`0x1395b8 +`): the deaths in the second and third cuboids, the third hint's shows.
const DEATHS_B: usize = 0x7a;
const DEATHS_C: usize = 0x79;
const SHOWN_D: usize = 0x7d;

/// The skill point its census gives (`0x1a7a74`).
const POINT: usize = 0x0c;
/// The classes it counts: the troopers, the flyers, 0x0d30, the gunship, 0x0d90.
const COUNTED: [i16; 5] = [0x0d0f, 0x0d57, 0x0d30, 0x0cbc, 0x0d90];

/// 0x0c2b (`0x3e46f8`; the decomp's v2.00 `func_US2_L09_003E46F8`): Dobbo's hints and its kill count.
/// * **0**: update distance 0xff, `+0x10` and `+0x2c` 0; the death counts (`0x1395b8 + 0x7a` / `+0x79`) held below 126.
///   Then 1, where it stays.
/// * **The census** (with Ratchet's kills alone so far, `0x1a9040`, and the skill point not had): 40 mobys a tick of
///   the troopers 0x0d0f (not in state 5), the flyers 0x0d57, 0x0d30 (not busy, its pvar `+0x3d6`), the gunships
///   0x0cbc and 0x0d90 alive (state below 0x80); a pass with none is skill point 0x0c (crate::skill_points).
/// * **The hints**, by Ratchet's place:
///   * in cuboid `+0x00`, with help slot 0x59 never dismissed, scene 0x5f seen (`0x1397c4` < 0) and item 0x15 not had
///     (`0x1a7b15`): the help text 0xa02 (slot 0x59);
///   * in cuboid `+0x04` once: `+0x10` set. His death there (state 0x39, its first tick) counts in `+0x7a`; there with
///     the armour 0 (`0x1a7a20`), on every fifth death, once a visit and not dead: the text 0x9fb (slot 0x52, if it was
///     never shown) and the count on past it;
///   * in cuboid `+0x08`: his death counts in `+0x79`; there without item 0x17 (`0x1a7b17`), on every fifth, once a
///     visit and not dead: the text 0x9fe (slot 0x55, if never shown) and the count on;
///   * in cuboid `+0x0c`, with the stat `0x18c750` (`+0x1bc` set, `+0x1c0` its time) not set or 432000 ticks old,
///     shown fewer than 3 times (`+0x7d`), once a visit: the text 0x9e3 (slot 0x3a, if never shown) and `+0x7d` up.
///
/// Not ported: the flag `0x1a9040` (1 as the level loads, cleared by a kill that was not Ratchet's — the enemies'
/// part is not ported, crate::moby::classes::trooper), so the census always runs; the stat `0x18c750`
/// (G-HERO-005, read as never set); the skill point's reward flag `0x1a903c` (as crate::skill_points).
pub fn update_0c2b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.update_dist = 0xff;
        m.set_pv_i(ENTERED, 0);
        m.set_pv_i(F2C, 0);
        for k in [DEATHS_B, DEATHS_C] {
            if let Some(b) = ctx.game.flags.get_mut(k) { if 126 <= *b { *b = 125; } }
        }
    }
    if w.mobys[id].state != 1 { return; }
    census(w, id, ctx);
    hints(w, id, ctx);
}

/// The census (above).
fn census(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.skill_points[POINT] != 0 { return; }
    let mut c = Census { cursor: w.mobys[id].pv_i(CURSOR).max(0) as usize, count: w.mobys[id].pv_i(COUNT) };
    let empty = c.step(w, true, |m| {
        if !COUNTED.contains(&m.o_class) { return Tally::Pass; }
        if m.o_class == 0x0d30 && m.pv_s(0x3d6) != 0 { return Tally::Skip; }
        if m.o_class == 0x0d0f && m.state == 5 { return Tally::Skip; }
        if m.state < 0x80 { Tally::Count } else { Tally::Pass }
    });
    let m = &mut w.mobys[id];
    m.set_pv_i(CURSOR, c.cursor as i32);
    m.set_pv_i(COUNT, c.count);
    if empty { ctx.game.award_skill_point(POINT); }
}

/// A help slot's text never shown (`0x18c958 + 0xc·slot + 8` ≥ 0: bit 31 clear).
fn never_shown(ctx: &Ctx, slot: usize) -> bool { ctx.game.help_levels.get(slot).is_none_or(|&b| (b as i32) >= 0) }

fn flag(ctx: &Ctx, k: usize) -> u8 { ctx.game.flags.get(k).copied().unwrap_or(0) }
fn bump(ctx: &mut Ctx, k: usize) { if let Some(b) = ctx.game.flags.get_mut(k) { *b = b.wrapping_add(1); } }

/// The hints (above).
fn hints(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let region = |w: &MobyWorld, k: usize| w.mobys[id].pv_i(REGIONS + 4 * k);
    let at = |ctx: &Ctx, c: i32| ctx.level.in_cuboid(c, ctx.hero.pos);
    // The death's first tick (state 0x39, `0x189fd8` 0).
    let died = ctx.hero.state == 0x39 && ctx.hero.timer == 0;

    if ctx.game.help_count(0x59) == 0 && ctx.game.scenes_seen.get(0x5f).copied().unwrap_or(false)
        && ctx.game.items.get(0x15).copied().unwrap_or(0) == 0 && at(ctx, region(w, 0)) {
        ctx.game.help(0xa02, 0x59);
    }
    if at(ctx, region(w, 1)) { w.mobys[id].set_pv_i(ENTERED, 1); }
    if died && w.mobys[id].pv_i(ENTERED) != 0 { bump(ctx, DEATHS_B); }
    if ctx.game.armor <= 0 && at(ctx, region(w, 1))
        && flag(ctx, DEATHS_B) % 5 == 4 && w.mobys[id].pv_i(HINT_1C) == 0 && ctx.hero.state != 0x39 {
        if never_shown(ctx, 0x52) { ctx.game.help(0x9fb, 0x52); }
        w.mobys[id].set_pv_i(HINT_1C, 1);
        bump(ctx, DEATHS_B);
    }
    if at(ctx, region(w, 2)) && died { bump(ctx, DEATHS_C); }
    if ctx.game.items.get(0x17).copied().unwrap_or(0) == 0 && at(ctx, region(w, 2))
        && flag(ctx, DEATHS_C) % 5 == 4 && w.mobys[id].pv_i(HINT_18) == 0 && ctx.hero.state != 0x39 {
        if never_shown(ctx, 0x55) { ctx.game.help(0x9fe, 0x55); }
        w.mobys[id].set_pv_i(HINT_18, 1);
        bump(ctx, DEATHS_C);
    }
    // `0x18c750`: never set in the port, so its age test passes.
    if at(ctx, region(w, 3)) && flag(ctx, SHOWN_D) < 3 && w.mobys[id].pv_i(HINT_14) == 0 {
        if never_shown(ctx, 0x3a) { ctx.game.help(0x9e3, 0x3a); }
        bump(ctx, SHOWN_D);
        w.mobys[id].set_pv_i(HINT_14, 1);
    }
}
