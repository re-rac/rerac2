//! The orbit fight's pickups (L22; the decomp's `func_L22_003A3CA0` … `func_L22_003A42E8` and the classes'
//! `func_L22_003FB1F8` … `func_L22_003FD2D8`; the same code at L23's places, which L23's fight does not use). The
//! opponent 0x0e0b (crate::moby::classes::level22::boss) runs them: health (kind 0, class [`HEALTH`]), gun charge
//! (kind 1, [`GUN`]) and specials (kind 2, [`SPECIAL`]) lie about the planetoid, and Giant Clank or the opponent takes
//! one by coming within 12 of it while short of its limit.
//!
//! **The rows** (`0x23b4b0`, three of 50 slots, 0xf0 bytes apart), their counts (`0x1b2a38`) and the total
//! (`0x1b2a44`). **The records** (`0x1b1890 + 0x10·k`): the opponent's (k 0, `0x1b2a34` points at it) and Giant
//! Clank's (k 1, `0x1b18a0`, crate::hero::giant::GiantBlock::counts): `[0]` / `[1]` timers (the opponent's counted
//! down twice a tick here; `[0]` set makes the shots strong), `[2]` the gun's charge, `[3]` the specials. The
//! limits and steps are the opponent's pvars: `+0x320` / `+0x324` Giant Clank's health step and most (`0x18b694`),
//! `+0x322` / `+0x24` its own (`+0x20`, a float), `+0x328` / `+0x32a` the gun's, `+0x32c` / `+0x32e` the specials'.
//!
//! * **The set-up** ([`set_up`], `0x3a4ad0`; `0x3a50d0` first): the rows emptied, both records' timers 0 and their
//!   gun and specials full. Seven different points drawn from the segments of the opponent's 32 paths (`+0x2a0`,
//!   the level's paths) get a pickup each, but on its paths 10, 27, 28, 31 and 15; then down each point of path
//!   `+0x340` (from 3 above to 6 below, a line 0x12) the ground gets one, staggered ([`create`]), 27 at most.
//! * **The tick** ([`tick`], `0x3a5180`): each pickup within 12 of Giant Clank (its distance from its update) while
//!   he has health and his sequence is not the hurt (6) is his if he is short ([`take`]: state 0x10, alpha 0x7f),
//!   else its alpha fades with the distance (127 · d / 12, 0.1..1); otherwise one within 12 of the opponent while it
//!   lives and its sequence is not 4 or 8 is its if it is short (state 0x11). A pickup taken leaves its row. The
//!   opponent's timers counted down; then one time in 30 (`0x320f00(30)`) with fewer than 12 out: a point drawn
//!   ([`pick_point`]), pushed 3.7 along a random row of each pickup within 4 of it (its sign a second draw) and put
//!   back on the surface, dropped from 3 above onto the ground (a line 0x12), and a pickup there ([`spawn`]).
//! * **The spawn** ([`spawn`], `0x3a4f80` → `0x3a4df8`): its kind drawn (`0x320f00(3)`: 0 the gun, 1 a special, 2
//!   health; health left out — a draw of 2 — while Giant Clank's sequence is the hurt or the opponent's is 4); the
//!   first free slot of the row gets it (none: none made).
//! * **The classes** ([`update`], `0x3fc810` / `0x3fdd70` / `0x3fe7d8`): each tick their distance from Giant Clank
//!   (health `+0x2c`, always, alpha 255 past 12; the others `+0x44` while drawn, else 2000, alpha 127 past 12). At
//!   rest: turned ¹⁄₅₃ rad a tick about their own z and stood up on the sphere. Taken: the level sound 0x12 on its
//!   first tick toward Giant Clank; turned `((45 − t)·22 + 155)′` a tick, alpha down to 2, shrunk ×(45 − t/45)·0.99/45
//!   a tick, set on the taker raised 7.5·t/45 above him; gone at 46 ticks (the first health pickup made instead
//!   parked in state 0x62, which does nothing). Staggered ones (`+0x2a`) test every 32 ticks for the ground under
//!   them (from 3 above to 6 below): with none they are put back on the surface, pushed 3.7 along a random row and
//!   dropped onto the ground from 3 above (a line 0x12), and the test stops.
//!
//! Not ported: the pickup message words `0x1b2a48` / `0x1b2a4c` / `0x1b2a50` (`0x3a5718`; the HUD's), the health
//! pickups' sparkles (`0x3fd0f0` at their creation with the first one's direction tables, `0x3fd548` at rest, their
//! random draws with them) and the light slots `+0x60..+0x80`, the arcs to Giant Clank while he takes one
//! (`0x3fceb8` / `0x3fe408` / `0x3fee58`), the gun's scale ×1.2 on its first taken tick (the same tick's earlier
//! once-test has always spent the flag), the owner word `+0x20`.

use super::planet::{down_at, surface};
use crate::hero::physics::{add, dist, scale, set_len, V3};
use crate::moby::attach::{rows_of, turn};
use crate::moby::frames::upright;
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const HEALTH: i16 = 0x0e3a;
pub const GUN: i16 = 0x0e3b;
pub const SPECIAL: i16 = 0x0e3d;

/// The rows' classes (`0x3a4df8`: 0 `0x3fc6f8`, 1 `0x3fdc80`, 2 `0x3fe6e8`) and their scales (`$gp+0x360`,
/// `+0x364`, `+0x36c`: × the class's).
const CLASSES: [i16; 3] = [HEALTH, GUN, SPECIAL];
const SCALES: [f32; 3] = [4.0, f32::from_bits(0x4069_999a), f32::from_bits(0x4069_999a)];
/// `$gp+0x340`: the reach.
const REACH: f32 = 12.0;
/// `$gp+0x2c8` / `0x1af2b8`: the push.
const PUSH: f32 = f32::from_bits(0x406c_cccd);
const ROW: usize = 50;
/// `$gp+0x370` (copied from `0x1af580`, its count `0x1b2a54` = 5): the opponent's paths left out.
const SKIPPED: [usize; 5] = [10, 27, 28, 31, 15];
/// The at-rest turn (rad a tick) and the taken one's unit (rad an arc minute).
const SPIN: f32 = 0.018_907_737;
const MINUTE: f32 = 0.000_290_888_25;
const FORTYFIFTH: f32 = 0.022_222_223;

/// The rows, their counts and the stagger counter.
#[derive(Clone, Debug)]
pub struct Pickups {
    pub slots: [[Option<usize>; ROW]; 3],
    pub counts: [i32; 3],
    pub total: i32,
    /// `0x1af32c`: the staggered ones made (their first tick count `(n + 2) % 31`).
    pub staggered: i32,
    /// `0x1b1cec` (the decomp's `D_L22_001B1D6C`): the first health pickup made.
    pub first_health: Option<usize>,
}

impl Default for Pickups {
    fn default() -> Self { Pickups { slots: [[None; ROW]; 3], counts: [0; 3], total: 0, staggered: 0, first_health: None } }
}

/// `0x3a50d0` + `0x3a4ad0` (module docs), from the opponent's set-up.
pub fn set_up(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx) {
    let (gun, special) = (w.mobys[boss].pv_s(0x32a) as i32, w.mobys[boss].pv_s(0x32e) as i32);
    w.level22.pickups = Pickups { staggered: w.level22.pickups.staggered, first_health: w.level22.pickups.first_health, ..Default::default() };
    w.level22.boss_counts = [0, 0, gun, special];
    ctx.hero.giant.counts = [0, 0, gun, special];
    // `0x320f00(3)`, its result unused.
    ctx.game.rng.randi(3);
    let paths: Vec<Vec<[f32; 4]>> = (0..32).map(|i| path_of(ctx, w.mobys[boss].pv_i(0x2a0 + 4 * i))).collect();
    let mut cumulative = [0i32; 32];
    let mut total = 0;
    for (i, c) in cumulative.iter_mut().enumerate() {
        *c = total;
        total += paths[i].len() as i32 - 1;
    }
    let mut chosen = [-1i32; 7];
    // The port's: too few segments to draw seven different ones (the game would draw for ever).
    for i in 0..if 7 <= total { 7 } else { 0 } {
        let pick = loop {
            let c = ctx.game.rng.randi(total);
            if !chosen.contains(&c) { break c; }
        };
        chosen[i] = pick;
        let j = (0..31).find(|&j| cumulative[j] <= pick && pick < cumulative[j + 1]).unwrap_or(31);
        let Some(p) = paths[j].get((pick - cumulative[j]).max(0) as usize).copied() else { continue };
        if !SKIPPED.contains(&j) { spawn(w, ctx, boss, [p[0], p[1], p[2]], false); }
    }
    let other = path_of(ctx, w.mobys[boss].pv_i(0x340));
    let mut made = 0;
    for p in other {
        if 27 <= made { break; }
        let pt = [p[0], p[1], p[2]];
        let down = down_at(ctx, pt);
        let (below, above) = (add(pt, scale(down, 6.0)), add(pt, scale(down, -3.0)));
        if let Some(hit) = w.coll_line(ctx, above, below, 0x12, None) {
            spawn(w, ctx, boss, hit.point, true);
            made += 1;
        }
    }
}

fn path_of(ctx: &Ctx, id: i32) -> Vec<[f32; 4]> { usize::try_from(id).ok().and_then(|i| ctx.level.paths.get(i)).cloned().unwrap_or_default() }

/// `0x3a7a10`: the pickup's distance from Giant Clank as its update left it.
fn hero_dist(w: &MobyWorld, p: usize) -> f32 { w.mobys[p].pv_f(if w.mobys[p].o_class == HEALTH { 0x2c } else { 0x44 }) }

/// The opponent's sequence (`+0x43`).
fn seq(w: &MobyWorld, ctx: &Ctx, id: usize) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0, |a| a.seq_b) }

/// `0x3a5180` (module docs).
pub fn tick(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx) {
    for kind in 0..3 {
        for k in 0..ROW {
            let Some(p) = w.level22.pickups.slots[kind][k] else { continue };
            let taken = if 0 < ctx.hero.giant.hp && ctx.hero_view.seq_b != 6 && hero_dist(w, p) < REACH {
                if !take(w, boss, ctx, kind, true) {
                    let r = hero_dist(w, p) / REACH;
                    let r = if 1.0 < r { 1.0 } else if r < 0.1 { 0.1 } else { r };
                    w.mobys[p].alpha = (r * 127.0) as i32 as u8;
                    continue;
                }
                let m = &mut w.mobys[p];
                if m.state == 0 {
                    m.set_state(0x10, 0);
                    m.alpha = 0x7f;
                }
                true
            } else {
                let s = seq(w, ctx, boss);
                let b = &w.mobys[boss];
                if !(0.0 < b.pv_f(0x20)) || s == 4 || s == 8 { continue; }
                if !(dist(w.mobys[p].pos, b.pos) < REACH) || !take(w, boss, ctx, kind, false) { continue; }
                let m = &mut w.mobys[p];
                if m.state == 0 { m.set_state(0x11, 0); }
                true
            };
            if taken {
                let pk = &mut w.level22.pickups;
                pk.counts[kind] -= 1;
                pk.slots[kind][k] = None;
                pk.total -= 1;
            }
        }
    }
    for _ in 0..2 {
        let c = &mut w.level22.boss_counts;
        countdown(&mut c[1]);
        countdown(&mut c[0]);
    }
    if ctx.game.rng.randi(30) != 0 || 12 <= w.level22.pickups.total { return; }
    let mut point = pick_point(w, ctx, boss, 0.0);
    let radius = w.level22.radius;
    for kind in 0..3 {
        for k in 0..ROW {
            let Some(p) = w.level22.pickups.slots[kind][k] else { continue };
            if 4.0 <= dist(w.mobys[p].pos, point) { continue; }
            let axis = w.mobys[p].rows[ctx.game.rng.randi(2) as usize];
            let s = if ctx.game.rng.randi(2) != 0 { PUSH } else { -PUSH };
            point = surface(ctx, radius, add(point, scale(axis, s)), 0.0);
        }
    }
    let high = surface(ctx, radius, point, 3.0);
    if let Some(hit) = w.coll_line(ctx, high, point, 0x12, None) { point = hit.point; }
    spawn(w, ctx, boss, point, false);
}

/// `0x3a5718(boss, pickup, kind, hero)`: kind 0 raises Giant Clank's health (`hero`) or the opponent's by its step
/// to its most, kind 1 the record's gun charge, kind 2 its specials; false when already at the most.
pub fn take(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx, kind: usize, hero: bool) -> bool {
    let b = &mut w.mobys[boss];
    match kind {
        0 if hero => {
            let most = b.pv_s(0x324) as i32;
            let hp = &mut ctx.hero.giant.hp;
            if most <= *hp { return false; }
            *hp = (*hp + b.pv_s(0x320) as i32).min(most);
            true
        }
        0 => {
            let most = b.pv_s(0x24) as f32;
            let h = b.pv_f(0x20);
            if !(h < most) { return false; }
            let h = h + b.pv_s(0x322) as f32;
            b.set_pv_f(0x20, if most < h { most } else { h });
            true
        }
        1 | 2 => {
            let (step, most, i) = if kind == 1 { (0x328, 0x32a, 2) } else { (0x32c, 0x32e, 3) };
            let most = b.pv_s(most) as i32;
            let rec = if hero { &mut ctx.hero.giant.counts } else { &mut w.level22.boss_counts };
            if most <= rec[i] { return false; }
            rec[i] = (rec[i] + b.pv_s(step) as i32).min(most);
            true
        }
        _ => false,
    }
}

/// `0x3a4980(out, 1, h)`: one of the opponent's 32 paths (`0x320f00(32)`) with none of its points within 45 of
/// Giant Clank and not left out ([`SKIPPED`]), a point of it but its last (`0x320f00(n − 1)`), put `h` above the
/// surface. The port's: at most 1000 draws (the game draws until one passes).
pub fn pick_point(w: &MobyWorld, ctx: &mut Ctx, boss: usize, h: f32) -> V3 {
    let hero = ctx.hero.pos;
    for _ in 0..1000 {
        let i = ctx.game.rng.randi(32) as usize;
        let path = path_of(ctx, w.mobys[boss].pv_i(0x2a0 + 4 * i));
        if path.iter().any(|p| dist(hero, [p[0], p[1], p[2]]) < 45.0) || SKIPPED.contains(&i) { continue; }
        let k = ctx.game.rng.randi(path.len() as i32 - 1).max(0) as usize;
        let Some(p) = path.get(k) else { continue };
        return surface(ctx, w.level22.radius, [p[0], p[1], p[2]], h);
    }
    w.mobys[boss].pos
}

/// `0x3a4f80(pos, staggered)` → `0x3a4df8` (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, boss: usize, pos: V3, staggered: bool) {
    let n = if ctx.hero_view.seq_b == 6 || seq(w, ctx, boss) == 4 { 2 } else { 3 };
    let kind = match ctx.game.rng.randi(n) { 0 => 1, 1 => 2, _ => 0 };
    let Some(k) = w.level22.pickups.slots[kind].iter().position(|s| s.is_none()) else { return };
    let id = create(w, kind, pos, staggered);
    let pk = &mut w.level22.pickups;
    pk.slots[kind][k] = Some(id);
    pk.total += 1;
    pk.counts[kind] += 1;
}

/// `0x3fc6f8` / `0x3fdc80` / `0x3fe6e8(boss, pos, staggered, scale)`: the pickup at `pos`, update distance 0xff, draw
/// distance 0x50, drawn, mode | 0x108 (the health's | 0x41 too), its scale × the kind's, its rows the identity;
/// staggered ones flagged (`+0x2a`) and their tick count `(n + 2) % 31`.
fn create(w: &mut MobyWorld, kind: usize, pos: V3, staggered: bool) -> usize {
    let id = w.create(CLASSES[kind]);
    let n = w.level22.pickups.staggered;
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.set_pv_s(0x2a, staggered as i16);
    if staggered {
        m.state_ticks = ((n + 2) % 31) as i16;
    }
    m.update_dist = 0xff;
    m.mode |= 0x108;
    m.draw_dist = 0x50;
    m.drawn = 1;
    m.state = 0;
    m.scale *= SCALES[kind];
    m.pos = pos;
    m.rows = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    if kind == 0 {
        m.mode |= 0x41;
        // `0x3fd0f0`: the first health pickup made keeps the sparkles' direction tables.
        if w.level22.pickups.first_health.is_none() { w.level22.pickups.first_health = Some(id); }
    }
    if staggered { w.level22.pickups.staggered = n + 1; }
    id
}

/// 0x0e3a / 0x0e3b / 0x0e3d's update (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let health = w.mobys[id].o_class == HEALTH;
    if health && w.mobys[id].state == 0x62 { return; }
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let hero = ctx.hero.pos;
    {
        let m = &mut w.mobys[id];
        if !health && m.first() && m.state == 0x10 { ctx.host.play_level_sound_after(None, 0x12, 0); }
        if health || m.drawn != 0 {
            let d = dist(hero, m.pos);
            m.set_pv_f(if health { 0x2c } else { 0x44 }, d);
            if REACH <= d { m.alpha = if health { 255 } else { 127 }; }
        } else {
            m.set_pv_f(0x44, 2000.0);
        }
    }
    let mut down = down_at(ctx, w.mobys[id].pos);
    if health && w.mobys[id].first() && w.mobys[id].state == 0x10 { ctx.host.play_level_sound_after(None, 0x12, 0); }
    let (state, t) = (w.mobys[id].state, w.mobys[id].state_ticks as u16);
    if state == 0 {
        let m = &mut w.mobys[id];
        m.rows = turn(&rows_of([0.0, 0.0, SPIN]), &m.rows);
        m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
    } else {
        let m = &mut w.mobys[id];
        let a = ((45.0 - t as f32) * 22.0 + 155.0) * MINUTE;
        if !health && m.first() {
            m.alpha = 127;
            if m.o_class == GUN { m.scale *= 1.2; }
        }
        if 3 <= m.alpha { m.alpha -= 1; }
        let s = (45.0 - t as f32 * FORTYFIFTH) * (f32::from_bits(0x3f7d_70a4) * FORTYFIFTH);
        m.rows = turn(&rows_of([0.0, 0.0, a]), &m.rows).map(|r| scale(r, s));
        if 46 <= t {
            if health && w.level22.pickups.first_health == Some(id) { w.mobys[id].state = 0x62; } else { w.delete(id); }
            return;
        }
        let to = match (state, w.level22.boss) {
            (0x10, _) | (_, None) => hero,
            (_, Some(b)) => w.mobys[b].pos,
        };
        down = down_at(ctx, to);
        let m = &mut w.mobys[id];
        m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
        m.pos = add(to, set_len(down, -(t as f32 * FORTYFIFTH) * 7.5));
    }
    let m = &mut w.mobys[id];
    m.state_ticks = if t < 0xffff { t + 1 } else { 0 } as i16;
    if m.pv_s(0x2a) == 0 || m.state_ticks as u16 & 31 != 0 { return; }
    let p = m.pos;
    if w.coll_line(ctx, add(p, scale(down, -3.0)), add(p, scale(down, 6.0)), 0x12, None).is_some() { return; }
    let radius = w.level22.radius;
    let mut p = surface(ctx, radius, p, 0.0);
    let axis = w.mobys[id].rows[ctx.game.rng.randi(2) as usize];
    let s = if ctx.game.rng.randi(2) != 0 { PUSH } else { -PUSH };
    p = surface(ctx, radius, add(p, scale(axis, s)), 0.0);
    if let Some(hit) = w.coll_line(ctx, add(p, scale(down, -3.0)), add(p, scale(down, 3.0)), 0x12, None) { p = hit.point; }
    let m = &mut w.mobys[id];
    m.pos = p;
    m.set_pv_s(0x2a, 0);
}
