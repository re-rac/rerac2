//! Damosel orbit's pickups (L23 `0x41df58`, `0x41e1c0`, `0x41e258`, `0x41dcd8` / `0x41ddc0`, `0x41e6f0`, `0x41f720`;
//! the decomp's `func_L23_0041CA58` … `func_L23_0041E220`): the opponent 0x0f7f (crate::moby::classes::level23::boss)
//! runs them. The pickups themselves are the orbit fights' shared classes and code (health 0x0e3a, gun charge 0x0e3b,
//! specials 0x0e3d: made `0x4037b0` / `0x404d38` / `0x405700`, their update `0x4038c8` / `0x404e28` / `0x4057f0`,
//! crate::moby::classes::orbit_pickups; the distance `0x41f720` the update leaves is its `hero_dist`); here only Giant
//! Clank takes them.
//!
//! * **The set-up** ([`set_up`], `0x41df58`; `0x41e1c0` first): the rows emptied; Giant Clank's record (`0x1b1f20`,
//!   crate::hero::giant::GiantBlock::counts) its timers 0 and its gun (`+0x2fa`) and specials (`+0x2fe`) full. Seven
//!   different points drawn from the segments of the opponent's 47 slots get a pickup each, but on its slots 10, 27,
//!   28, 31 and 15.
//! * **The tick** ([`tick`], `0x41e258`): with the opponent alive, each pickup within 12 of Giant Clank while his
//!   sequence is not the hurt (6) is his if he is short ([`take`]: state 0x10, alpha 0x7f) and leaves its row; else
//!   its alpha fades with the distance (127 · d / 12, 0.1..1). Then one time in 30 with fewer than 12 out: a point
//!   (crate::moby::classes::level23::paths::random_point), pushed 5 along a random row of each pickup within 4 of it
//!   (its sign a second draw) and put back on the surface, dropped from 3 above onto the ground (a line 0x12), and a
//!   pickup there ([`spawn`]).
//! * **The spawn** ([`spawn`], `0x41dcd8` → `0x41ddc0`): its kind drawn (0 the gun, 1 a special, 2 health; health left
//!   out while Giant Clank's sequence is the hurt); the first free slot of the row gets it (none: none made).
//! * **The take** ([`take`], `0x41e6f0`): kind 0 raises his health (`0x18b694`) by `+0x2f0` to `+0x2f4`, kind 1 the
//!   gun's charge by `+0x2f8` to `+0x2fa`, kind 2 the specials by `+0x2fc` to `+0x2fe`; false when already full.
//!
//! Not ported: the HUD's pickup words (`0x1b36cc`, `0x1b36d0`, `$gp+0x46dc`, `$gp+0x46e0`, `0x1b36d4`).

use super::paths;
use crate::hero::physics::{add, dist, scale};
use crate::moby::classes::orbit_pickups::{create, hero_dist};
use crate::moby::{Ctx, MobyWorld};

const ROW: usize = 50;
/// The set-up's slots left out.
const SKIPPED: [usize; 5] = [10, 27, 28, 31, 15];
/// `$gp+0x2404`: the push off another pickup.
const PUSH: f32 = 5.0;

/// The rows (`0x258f50`, three of 50), their counts (`0x1b36b8`) and the total (`0x1b36c4`).
#[derive(Clone, Debug)]
pub struct Pickups {
    pub slots: [[Option<usize>; ROW]; 3],
    pub counts: [i32; 3],
    pub total: i32,
}

impl Default for Pickups {
    fn default() -> Self { Pickups { slots: [[None; ROW]; 3], counts: [0; 3], total: 0 } }
}

/// `0x41df58` (module docs).
pub fn set_up(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx) {
    let (gun, special) = (w.mobys[boss].pv_s(0x2fa) as i32, w.mobys[boss].pv_s(0x2fe) as i32);
    w.level23.pickups = Pickups::default();
    ctx.hero.giant.counts = [0, 0, gun, special];
    ctx.game.rng.randi(3);
    let n = w.level23.paths.points.len();
    let mut cumulative = vec![0i32; n];
    let mut total = 0;
    for (i, c) in cumulative.iter_mut().enumerate() {
        *c = total;
        total += paths::slot(w, i).len() as i32 - 1;
    }
    let mut chosen = [-1i32; 7];
    // The port's: too few segments to draw seven different ones (the game would draw for ever).
    for i in 0..if 7 <= total { 7 } else { 0 } {
        let pick = loop {
            let c = ctx.game.rng.randi(total);
            if !chosen.contains(&c) { break c; }
        };
        chosen[i] = pick;
        let j = (0..n - 1).find(|&j| cumulative[j] <= pick && pick < cumulative[j + 1]).unwrap_or(n - 1);
        let Some(&p) = paths::slot(w, j).get((pick - cumulative[j]).max(0) as usize) else { continue };
        if !SKIPPED.contains(&j) { spawn(w, ctx, p, false); }
    }
}

/// `0x41e258` (module docs).
pub fn tick(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx) {
    for kind in 0..3 {
        for k in 0..ROW {
            let Some(p) = w.level23.pickups.slots[kind][k] else { continue };
            if !(0.0 < w.mobys[boss].pv_f(0x20)) || ctx.hero_view.seq_b == 6 || !(dist(w.mobys[p].pos, ctx.hero.pos) < 12.0) { continue; }
            if take(w, boss, ctx, kind) {
                let m = &mut w.mobys[p];
                if m.state == 0 {
                    m.set_state(0x10, 0);
                    m.alpha = 0x7f;
                }
                let pk = &mut w.level23.pickups;
                pk.slots[kind][k] = None;
                pk.counts[kind] -= 1;
                pk.total -= 1;
            } else {
                let a = (hero_dist(w, p) * 0.083_333_336).clamp(0.1, 1.0);
                w.mobys[p].alpha = (a * 127.0) as i32 as u8;
            }
        }
    }
    if ctx.game.rng.randi(30) != 0 || 12 <= w.level23.pickups.total { return; }
    let r = w.level23.radius;
    let mut point = paths::random_point(w, ctx, 0.0);
    for kind in 0..3 {
        for k in 0..ROW {
            let Some(p) = w.level23.pickups.slots[kind][k] else { continue };
            if 4.0 <= dist(w.mobys[p].pos, point) { continue; }
            let axis = w.mobys[p].rows[ctx.game.rng.randi(2) as usize];
            let s = if ctx.game.rng.randi(2) != 0 { PUSH } else { -PUSH };
            point = paths::surface(ctx, r, add(point, scale(axis, s)), 0.0);
        }
    }
    let high = paths::surface(ctx, r, point, 3.0);
    if let Some(hit) = w.coll_line(ctx, high, point, 0x12, None) { point = hit.point; }
    spawn(w, ctx, point, false);
}

/// `0x41e6f0(boss, pickup, kind, 1)` (module docs).
pub fn take(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx, kind: usize) -> bool {
    let b = &w.mobys[boss];
    let g = &mut ctx.hero.giant;
    let (now, step, most) = match kind {
        0 => (g.hp, b.pv_s(0x2f0) as i32, b.pv_s(0x2f4) as i32),
        1 => (g.counts[2], b.pv_s(0x2f8) as i32, b.pv_s(0x2fa) as i32),
        _ => (g.counts[3], b.pv_s(0x2fc) as i32, b.pv_s(0x2fe) as i32),
    };
    if most <= now { return false; }
    let v = (now + step).min(most);
    match kind {
        0 => g.hp = v,
        1 => g.counts[2] = v,
        _ => g.counts[3] = v,
    }
    true
}

/// `0x41dcd8(pos, staggered)` → `0x41ddc0` (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, pos: crate::hero::physics::V3, staggered: bool) {
    let n = if ctx.hero_view.seq_b == 6 { 2 } else { 3 };
    let kind = match ctx.game.rng.randi(n) { 0 => 1, 1 => 2, _ => 0 };
    let Some(k) = w.level23.pickups.slots[kind].iter().position(|s| s.is_none()) else { return };
    let id = create(w, kind, pos, staggered);
    let pk = &mut w.level23.pickups;
    pk.slots[kind][k] = Some(id);
    pk.total += 1;
    pk.counts[kind] += 1;
}

/// `0x41e190(pos, kind)` → `0x41ddc0(boss, pos, kind, 0)`: a pickup of `kind` at `pos` in its row's first free slot
/// (none free: none made). The walkers 0x07e5 Giant Clank kills drop his this way
/// (crate::moby::classes::level23::drones).
pub fn drop_at(w: &mut MobyWorld, _ctx: &mut Ctx, pos: crate::hero::physics::V3, kind: usize) {
    let kind = kind.min(2);
    let Some(k) = w.level23.pickups.slots[kind].iter().position(|s| s.is_none()) else { return };
    let id = create(w, kind, pos, false);
    let pk = &mut w.level23.pickups;
    pk.slots[kind][k] = Some(id);
    pk.total += 1;
    pk.counts[kind] += 1;
}
