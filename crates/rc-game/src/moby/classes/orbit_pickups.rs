//! The orbit fights' pickups, health **0x0e3a**, gun charge **0x0e3b** and specials **0x0e3d** (L22: made `0x3fc6f8` /
//! `0x3fdc80` / `0x3fe6e8`, their updates `0x3fc810` / `0x3fdd70` / `0x3fe7d8`; L23: made `0x4037b0` / `0x404d38` /
//! `0x405700`, their updates `0x4038c8` / `0x404e28` / `0x4057f0`; the decomp's `func_L22_003FB1F8` …
//! `func_L22_003FD2D8`; L22 addresses below). Each fight lays them out and hands them out its own way
//! (crate::moby::classes::level22::pickups, crate::moby::classes::level23::pickups): Giant Clank (or Dobbo orbit's
//! opponent) takes one by coming within 12 of it while short.
//!
//! **Each tick** ([`update`]): their distance from Giant Clank (health `+0x2c`, always, alpha 255 past 12; the others
//! `+0x44` while drawn, else 2000, alpha 127 past 12). At rest: turned ¹⁄₅₃ rad a tick about their own z and stood up
//! on the sphere. Taken: the level sound 0x12 on its first tick toward Giant Clank; turned `((45 − t)·22 + 155)′` a
//! tick, alpha down to 2, shrunk ×(45 − t/45)·0.99/45 a tick, set on the taker raised 7.5·t/45 above him; gone at 46
//! ticks (the first health pickup made instead parked in state 0x62, which does nothing). Staggered ones (`+0x2a`) test
//! every 32 ticks for the ground under them (from 3 above to 6 below): with none they are put back on the surface,
//! pushed 3.7 along a random row and dropped onto the ground from 3 above (a line 0x12), and the test stops.
//!
//! The fight's words the update reads (the planetoid's radius `0x1b2a24` for a staggered one's re-drop, the opponent
//! `$gp+0x2d4` a pickup it takes goes to) are Dobbo orbit's (crate::moby::classes::level22::Globals): L23's copy reads
//! its own copies of them, which nothing there writes, and reaches neither read (Damosel orbit's fight makes no
//! staggered pickups and its opponent takes none).
//!
//! Not ported: the health pickups' sparkles (`0x3fd0f0` at their creation with the first one's direction tables,
//! `0x3fd548` at rest, their random draws with them) and the light slots `+0x60..+0x80`, the arcs to Giant Clank while
//! he takes one (`0x3fceb8` / `0x3fe408` / `0x3fee58`), the gun's scale ×1.2 on its first taken tick (the same tick's
//! earlier once-test has always spent the flag), the owner word `+0x20`.

use crate::hero::physics::{add, dist, scale, set_len, V3};
use crate::moby::attach::{rows_of, turn};
use crate::moby::classes::planetoid::{down_at, surface};
use crate::moby::frames::upright;
use crate::moby::{Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const HEALTH: i16 = 0x0e3a;
pub const GUN: i16 = 0x0e3b;
pub const SPECIAL: i16 = 0x0e3d;

/// The kinds' classes (0 health, 1 the gun, 2 specials) and their scales (`$gp+0x360`, `+0x364`, `+0x36c`: × the
/// class's).
const CLASSES: [i16; 3] = [HEALTH, GUN, SPECIAL];
const SCALES: [f32; 3] = [4.0, f32::from_bits(0x4069_999a), f32::from_bits(0x4069_999a)];
/// `$gp+0x340`: the reach.
pub const REACH: f32 = 12.0;
/// `$gp+0x2c8` / `0x1af2b8`: the push.
pub const PUSH: f32 = f32::from_bits(0x406c_cccd);
/// The at-rest turn (rad a tick) and the taken one's unit (rad an arc minute).
const SPIN: f32 = 0.018_907_737;
const MINUTE: f32 = 0.000_290_888_25;
const FORTYFIFTH: f32 = 0.022_222_223;

/// The class code's own words (crate::moby::MobyWorld::orbit_pickups; L22's, L23 has its own copies).
#[derive(Clone, Debug, Default)]
pub struct Words {
    /// `0x1af32c`: the staggered ones made (their first tick count `(n + 2) % 31`).
    pub staggered: i32,
    /// `0x1b1cec` (the decomp's `D_L22_001B1D6C`): the first health pickup made.
    pub first_health: Option<usize>,
}

/// `0x3a7a10` (L23 `0x41f720`): the pickup's distance from Giant Clank as its update left it.
pub fn hero_dist(w: &MobyWorld, p: usize) -> f32 { w.mobys[p].pv_f(if w.mobys[p].o_class == HEALTH { 0x2c } else { 0x44 }) }

/// The fight's words the update reads (module docs).
fn fight(w: &MobyWorld) -> (f32, Option<usize>) { (w.level22.radius, w.level22.boss) }

/// `0x3fc6f8` / `0x3fdc80` / `0x3fe6e8(boss, pos, staggered, scale)`: the pickup at `pos`, update distance 0xff, draw
/// distance 0x50, drawn, mode | 0x108 (the health's | 0x41 too), its scale × the kind's, its rows the identity;
/// staggered ones flagged (`+0x2a`) and their tick count `(n + 2) % 31`. Damosel orbit's fight makes its pickups
/// with the same code (L23 `0x4037b0` / `0x404d38` / `0x405700`).
pub fn create(w: &mut MobyWorld, kind: usize, pos: V3, staggered: bool) -> usize {
    let id = w.create(CLASSES[kind]);
    let n = w.orbit_pickups.staggered;
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
        if w.orbit_pickups.first_health.is_none() { w.orbit_pickups.first_health = Some(id); }
    }
    if staggered { w.orbit_pickups.staggered = n + 1; }
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
            if health && w.orbit_pickups.first_health == Some(id) { w.mobys[id].state = 0x62; } else { w.delete(id); }
            return;
        }
        let boss = fight(w).1;
        let to = match (state, boss) {
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
    let radius = fight(w).0;
    let mut p = surface(ctx, radius, p, 0.0);
    let axis = w.mobys[id].rows[ctx.game.rng.randi(2) as usize];
    let s = if ctx.game.rng.randi(2) != 0 { PUSH } else { -PUSH };
    p = surface(ctx, radius, add(p, scale(axis, s)), 0.0);
    if let Some(hit) = w.coll_line(ctx, add(p, scale(down, -3.0)), add(p, scale(down, 3.0)), 0x12, None) { p = hit.point; }
    let m = &mut w.mobys[id];
    m.pos = p;
    m.set_pv_s(0x2a, 0);
}
