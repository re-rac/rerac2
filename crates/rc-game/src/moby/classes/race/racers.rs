//! The racers **0x0b2d** (L04 `0x405aa0`, L11 `0x3f0b58`; the decomp's `func_L04_004045E0`): the hoverbikes Ratchet
//! races, each with its rider 0x08ef on it, driving their routes (crate::moby::classes::race, L04 addresses).
//!
//! **Its pvars** (0x2e0; the decomp's `L04RacerState`): `+0x54[6]` / `+0x6c[4]` / `+0x7c` its routes (`+0x50` the
//! first's fallback), `+0x80` the jets' group, `+0x84` / `+0x88` the finish's and the half way's cuboids, `+0x8c` its
//! rider, `+0x90` / `+0x94` its sphere's radius and height, `+0x98` the height it is lost under, `+0x9c` its grid
//! slot (`+0xa2` the slot it starts from), `+0xa1` the short cut's rest, `+0xa4` bounced, `+0xa5` in the air (`+0xa6`
//! the ticks), `+0xa8` its place along its route (`+0xf0` the point there, `+0x114` / `+0x118` index and fraction),
//! `+0xb0` / `+0xc0` its start's place and angles, `+0xd0` / `+0xe0` its velocity now and before, `+0x100` a push,
//! `+0x110` its route now, `+0x11c` how far along the race it is (`+0x120` / `+0x124` the branch and point), `+0x128`
//! / `+0x12c` / `+0x130` the roll's, pitch's and yaw's speeds, `+0x134` the speed it aims at, `+0x138` its speed
//! (`+0x158` its rate), `+0x13c` the jets' boost (`+0x140` the boost's ticks left, `+0x142` on a jet), `+0x144` the
//! surface flag of its move, `+0x148` / `+0x14c` a bounce's ticks and lift, `+0x150` the wobble's phase, `+0x154` its
//! fall, `+0x15c` past half way, `+0x160` its lap, `+0x164` the bike (the master), `+0x168` near the bike, `+0x16c` /
//! `+0x170` its lateral offset and lane (`+0x174` the offset's timer), `+0x178` a bump's ticks, `+0x17c` / `+0x17e`
//! the rider's flinch and its rest, `+0x180` the taunt's rest, `+0x184[6]` random phases, `+0x1a0` its finish time,
//! `+0x1a8` its skill (1..10), `+0x1ac` the walls in a row, `+0x1b0` its race time (s), `+0x1b4` the route's length,
//! `+0x1b8` the ticks of this lap, `+0x1bc` the bumps while following, `+0x1c0` following the bike (ticks),
//! `+0x1e4..+0x1f0` the follow's offset, its ease and step, `+0x1f4` / `+0x1f8` a swerve (its roll and strength),
//! `+0x240` on the short cut, `+0x244` on water, `+0x248` the push taken, `+0x24c` stuck, `+0x2d0` the time it is
//! behind, `+0x2d4` the pace. The port's `+0x2e0`: the surface its ground probe found last (the original's global
//! `0x2e23e0` read after other queries).
//!
//! **States**: 0 set-up (on the ground, its start kept, its routes' steps, six random phases; hidden; 1); 1 idle
//! (the race is not on); 2 racing ([`pace`], [`jets`], [`drive`], [`laps`], [`looks`]); 3 crashed (back on its
//! route's nearest point, out of sight after 300 ticks and 5 from the bike it races again); 4 stopped (its and its
//! rider's animations frozen; 1). While the bike is in its states 0, 1 or 4 it does nothing; while the bike counts
//! its ticks of the race (its `+0xbc` 5 / 6) the lap's ticks run.
//!
//! Not ported (effects): the speed lines' draw `0x405530` (its scaled axes `+0x200..`, `0x2edd50`), the trail
//! `0x416390` and its lights (`0x362ba0`), the sprays `0x40d260` / `0x40d5d0` and the sparks of a wall hit, the crash's
//! debris mobys 0x0b3d..0x0b40 (`0x408538`, their update `0x408c60`), the rider's head aim (`0x324208` on `+0x250`),
//! the shadow probe's slab (`0x31da58`: done by crate::shadow_volumes::probe_near).

use super::{ground, in_view, lateral, nearest_index, off_track, slide, Paths};
use crate::hero::physics::{add, add_rot, approach, atan2, dist, dot, len, rot_dist, scale, spring, sub, sub_rot, turn_spring};
use crate::moby::{ai, countdown, Ctx, Moby, MobyWorld};
use std::f32::consts::{FRAC_PI_2, PI};

const SURFACE: usize = 0x2e0;

fn bike_of(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], 0x164).filter(|&b| b < w.mobys.len()) }
fn rider_of(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], 0x8c).filter(|&b| b < w.mobys.len()) }

/// `0x2eeae0` on a short: 1 down; true when it was 0 or just reached it.
fn countdown_s(m: &mut Moby, off: usize) -> bool {
    let mut v = m.pv_s(off) as i32;
    let r = countdown(&mut v);
    m.set_pv_s(off, v as i16);
    r
}
fn countdown_i(m: &mut Moby, off: usize) -> bool {
    let mut v = m.pv_i(off);
    let r = countdown(&mut v);
    m.set_pv_i(off, v);
    r
}

/// Its offset along its route past the nearest point: 15 on water, else 5.
fn lead(m: &Moby) -> f32 { if m.pv_i(0x244) != 0 { 15.0 } else { 5.0 } }

/// Its place along its route `t` into its words (`+0xa8`, `+0x114`, `+0x118`, `+0xf0`).
fn set_along(paths: &Paths, m: &mut Moby, t: f32) {
    let r = m.pv_i(0x110);
    m.set_pv_f(0xa8, t);
    super::place_along(paths, m, r, t, 0xa8);
}

pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let class_scale = w.class(super::RACER).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        if m.pvars.len() < 0x2e4 { m.pvars.resize(0x2e4, 0); }
        m.scale = class_scale * 0.25;
        m.update_dist = 0xff;
        m.draw_dist = 0x7f;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 9.0, 8);
    if let Some(b) = bike_of(w, id) {
        let (bs, bt) = (w.mobys[b].state, w.mobys[b].timer);
        if matches!(bs, 0 | 1 | 4) { return; }
        if bs == 2 && (5..=6).contains(&bt) {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x1b8, m.pv_i(0x1b8) + 1);
        }
    }
    match w.mobys[id].state {
        0 => set_up(w, id, ctx),
        2 => {
            if let Some(b) = bike_of(w, id) {
                if w.mobys[b].state == 2 && w.mobys[b].timer == 0 { return; }
            }
            pace(w, id, ctx);
            jets(w, id, ctx);
            drive(w, id, ctx);
            laps(w, id, ctx);
            looks(w, id, ctx);
        }
        3 => crashed(w, id, ctx),
        4 => {
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            if let Some(r) = rider_of(w, id) { ctx.host.set_anim_speed(&w.mobys[r], 0.0); }
            w.mobys[id].state = 1;
        }
        _ => {}
    }
}

/// State 0 (module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, r) = (w.mobys[id].pos, w.mobys[id].pv_f(0x90));
    if let Some(h) = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, r, crate::collision::QueryFlags(4), Some(id))) {
        w.mobys[id].pos = h.pushed_centre.unwrap_or(pos);
    }
    let m = &mut w.mobys[id];
    let (p, rot) = (m.pos, m.rot);
    m.set_pv_v(0xb0, p);
    m.set_pv_v(0xc0, rot);
    // `0x404148`: its routes' steps.
    let routes: Vec<i32> = (0..6).map(|k| m.pv_i(0x54 + 4 * k)).chain((0..4).map(|k| m.pv_i(0x6c + 4 * k))).chain([m.pv_i(0x7c)]).collect();
    for r in routes { w.race.paths.ensure(ctx.level, r); }
    let m = &mut w.mobys[id];
    m.state = 1;
    m.set_pv_i(0x30, 0);
    m.mode |= 0x41;
    for k in 0..6 {
        let f = ctx.game.rng.randf(0.0, 1.0);
        w.mobys[id].set_pv_f(0x184 + 4 * k, f);
    }
}

/// `0x403e98` (`func_L04_004029d8`): a racer put on the grid for the race the bike starts (`0x40bb18` calls it): at its
/// start, its slot, its aim `0.369 − (slot − 1)·0.0117 − (1 − skill/10)·0.0333`, the speeds and flags cleared, its
/// route its first (`+0x54`, else `+0x50`) at 5 along, a random lateral offset (±1.5, a new one in 3..6 s), its
/// collision and its target bit on, its rider made (`0x3e5e40(m, 0x8ef, 0.25)`) and shown at it; 2.
pub fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let slot = m.pv_b(0x9c);
    m.set_pv_b(0xa2, slot);
    m.pos = m.pv_v(0xb0);
    m.rot = m.pv_v(0xc0);
    m.set_pv_b(0xa0, 0);
    let aim = 0.369_166_67 - (slot as i32 - 1) as f32 * 0.011_666_667 - (1.0 - m.pv_i(0x1a8) as f32 * 0.1) * 0.033_333_335;
    m.set_pv_f(0x134, aim);
    for o in [0x138, 0x158, 0x13c, 0x160, 0x24c, 0xac, 0x114, 0x118, 0x128, 0x12c, 0x130, 0x244, 0x1f4, 0x144, 0x154, 0x150, 0x148, 0x178, 0x248, 0x14c, 0x15c, 0x170] { m.set_pv_i(o, 0); }
    m.set_pv_s(0x142, 0);
    m.set_pv_s(0x140, 0);
    m.set_pv_b(0xa4, 0);
    m.set_pv_b(0xa5, 0);
    m.set_pv_s(0xa6, 0);
    m.set_pv_b(0xa1, 0);
    m.set_pv_v(0xd0, [0.0; 3]);
    m.set_pv_v(0xe0, [0.0; 3]);
    m.set_pv_v(0x100, [0.0; 3]);
    let route = if m.pv_i(0x54) == -1 { m.pv_i(0x50) } else { m.pv_i(0x54) };
    m.set_pv_i(0x110, route);
    m.set_pv_f(0x1f8, 1.0);
    w.race.paths.ensure(ctx.level, route);
    let paths = &w.race.paths;
    let m = &mut w.mobys[id];
    set_along(paths, m, 5.0);
    let off = ctx.game.rng.randf(-1.5, 1.5);
    m.set_pv_f(0x16c, off);
    let t = ctx.game.rng.randf(3.0, 6.0);
    m.set_pv_i(0x174, (t * 60.0) as i32);
    let solid = w.class(super::RACER).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.has_collision = solid;
    m.mode = (m.mode & 0xffbe) | 0x1400;
    if rider_of(w, id).is_none() {
        let r = make_rider(w, id, ctx);
        ai::set_link(&mut w.mobys[id], 0x8c, r);
    }
    if let Some(r) = rider_of(w, id) {
        let (p, rot) = (w.mobys[id].pos, w.mobys[id].rot);
        let rm = &mut w.mobys[r];
        rm.mode &= 0xffbe;
        rm.pos = p;
        rm.rot = rot;
        rm.update_dist = 0;
        ctx.host.advance(&w.mobys[r]);
        w.rebuild(r, &*ctx.host);
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(0x17e, 0);
    m.set_pv_s(0x17c, -1);
    m.set_pv_i(0x180, 0);
    m.state = 2;
    m.set_pv_i(0x120, -1);
    for o in [0x240, 0x1c0, 0x124, 0x11c, 0x1ac, 0x1a0, 0x19c] { m.set_pv_i(o, 0); }
}

/// `0x3e5e40(m, 0x8ef, 0.25)`: the rider made at the racer (updated and drawn at any distance, a quarter of its
/// class's scale, rebuilt).
fn make_rider(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> Option<usize> {
    let (p, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let class_scale = w.class(super::RIDER)?.scale;
    let r = w.create(super::RIDER);
    let rm = &mut w.mobys[r];
    rm.update_dist = 0xff;
    rm.drawn = 1;
    rm.draw_dist = 0xff;
    rm.state = 0;
    rm.scale = class_scale * 0.25;
    rm.pos = p;
    rm.rot = rot;
    w.rebuild(r, &*ctx.host);
    Some(r)
}

/// State 3 (module docs; `0x405aa0` case 3).
fn crashed(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    if w.mobys[id].state_ticks == 0 {
        let route = w.mobys[id].pv_i(0x110);
        let k = nearest_index(&w.race.paths, level, &w.mobys[id], route, true);
        let paths = &w.race.paths;
        let m = &mut w.mobys[id];
        m.pos = paths.point(route, k);
        let t = k as f32 * paths.step(route) + lead(m);
        set_along(paths, m, t);
    }
    if 300 <= w.mobys[id].state_ticks as u16 {
        if let Some(b) = bike_of(w, id) {
            let pos = w.mobys[id].pos;
            let seen = ctx.view.is_none_or(|v| !v.culled(60.0, [pos[0], pos[1], pos[2], 0.0])) && dist(pos, ctx.host.camera_pos()) < 60.0;
            let d = sub(w.mobys[b].pos, pos);
            if !seen && 25.0 < dot(d, d) {
                let solid = w.class(super::RACER).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.has_collision = solid;
                m.mode = (m.mode & 0xffbe) | 0x1400;
                if let Some(r) = rider_of(w, id) { w.mobys[r].mode &= 0xffbe; }
                let set = super::RouteSet::of(&w.mobys[b]);
                let m = &w.mobys[id];
                let (mut link, mut point) = (m.pv_i(0x120), m.pv_i(0x124));
                let p = super::find_progress(&w.race.paths, &set, pos, &mut link, &mut point, None);
                let m = &mut w.mobys[id];
                m.set_pv_i(0x120, link);
                m.set_pv_i(0x124, point);
                m.set_pv_f(0x11c, p);
                m.state = 2;
            }
        }
    }
    let m = &mut w.mobys[id];
    m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16;
}

/// `0x402208(m, s, place)` (`func_L04_00400d48`): back on its route at the nearest point (Barlow: the first route
/// while the bike has no branch, the short cut's route `+0x6c` with the record's flag 4 — its point 130 past the
/// branch 1's third point), `place`: put there; its place 5 (15 on water) past it; no longer following, the bike's
/// pull `+0x63c` 1 and its follower `+0x4c4` none.
pub fn back_on_route(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, place: bool) {
    let Some(b) = bike_of(w, id) else { return };
    let level = ctx.game.level;
    let mut k = -1;
    if level == 4 {
        if w.mobys[b].pv_i(0x314) == -1 {
            let r = w.mobys[id].pv_i(0x54);
            w.mobys[id].set_pv_i(0x110, r);
        } else if super::record(level, w.race.race).flags & 4 != 0 {
            let r = w.mobys[id].pv_i(0x6c);
            w.mobys[id].set_pv_i(0x110, r);
            if w.mobys[id].pv_i(0x120) == 1 && 3 <= w.mobys[id].pv_i(0x124) { k = 0x82; }
        }
    }
    let route = w.mobys[id].pv_i(0x110);
    w.race.paths.ensure(ctx.level, route);
    if k == -1 { k = nearest_index(&w.race.paths, level, &w.mobys[id], route, true); }
    let paths = &w.race.paths;
    let m = &mut w.mobys[id];
    if place { m.pos = paths.point(route, k); }
    let t = k as f32 * paths.step(route) + lead(m);
    set_along(paths, m, t);
    m.set_pv_i(0x1c0, 0);
    let bm = &mut w.mobys[b];
    bm.set_pv_f(0x63c, 1.0);
    bm.set_pv_i(0x4c4, 0);
}

/// `0x4049f0(m, s)` (`func_L04_00403530`, the decomp's `UpdateRouteFollowerPace`): the pace from the race time left
/// (`(route length − progress) / (race time + behind − this lap's ticks/60)`, at least 6.645 kept at `+0x2d4`); not
/// while following. The route's branches chosen ([`branches`]); the aim from the pace (22.15 on the last laps past
/// two thirds, 13.29..28.15) less a little by its slot; near the bike (within 12..150 behind it, on Ratchet's lap,
/// the bike racing) it closes in, it tells the bike it is just ahead of it (`+0x734`, `+0x720`), and within 10 of it
/// it closes harder, taunts (its rider's sequences 7 / 8 ahead, 5 / 6 beside, by the side the bike is on) and keeps a
/// lateral offset toward it; the aim held to 0.1477..0.4692. Then its place along its route: 5 (15 on water) on past
/// the point when it is that near, again from the nearest point when the point is more than 60° off its way; the
/// markers it passed (±1000) set a new lateral offset or none; the offset's timer; the lane's point beside its way.
fn pace(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        if m.pv_i(0x180) != 0 { m.set_pv_i(0x180, m.pv_i(0x180) - 1); }
        if m.pv_s(0x17e) != 0 { m.set_pv_s(0x17e, m.pv_s(0x17e) - 1); }
    }
    let m = &w.mobys[id];
    let mut speed = (m.pv_f(0x1b4) - m.pv_f(0x11c)) / ((m.pv_f(0x1b0) + m.pv_f(0x2d0)) - m.pv_i(0x1b8) as f32 * 0.016_666_668);
    let m = &mut w.mobys[id];
    m.set_pv_f(0x2d4, speed.max(6.645));
    if 0 < m.pv_i(0x1c0) { return; }
    branches(w, id, ctx);
    let m = &w.mobys[id];
    if speed < 0.0 || (22.15 <= speed && 2 <= m.pv_i(0x160) && 0.66 < m.pv_f(0x11c) / m.pv_f(0x1b4)) { speed = 22.15; }
    let speed = speed.min(28.15).max(13.29);
    let slot = m.pv_b(0xa2) as i32;
    w.mobys[id].set_pv_f(0x134, speed * 0.016_666_668 - (slot - 1) as f32 * 0.011_666_667);
    let lap = w.race.lap;
    let surface = w.mobys[id].pv_i(SURFACE);
    if let Some(b) = bike_of(w, id).filter(|_| w.mobys[id].pv_b(0xa5) == 0 && surface != 5) {
        let bike_prog = w.mobys[b].pv_f(0x310);
        let me = w.mobys[id].pv_f(0x11c);
        let delta = bike_prog - me;
        let racing = w.mobys[b].timer == 5;
        let my_lap = w.mobys[id].pv_i(0x160);
        if me + 12.0 < bike_prog && bike_prog - 150.0 < me && my_lap <= lap && ai::link(&w.mobys[b], 0x730) != Some(id) && racing {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x134, m.pv_f(0x134) + (delta - 12.0) * 0.001_666_666_8);
        }
        if racing && w.mobys[b].pv_i(0x724) == 0 && 1.0 < delta && delta < 10.0 {
            ai::set_link(&mut w.mobys[b], 0x734, Some(id));
            w.mobys[b].set_pv_i(0x720, 1);
        }
        if w.mobys[id].pv_i(0x168) != 0 && slot != 1 && delta.abs() < 10.0 && lap <= my_lap {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x134, m.pv_f(0x134) + delta * 0.016_666_668 + delta * 0.016_666_668);
            let rider = rider_of(w, id);
            let m = &w.mobys[id];
            if bike_prog < me && m.pv_i(0x180) == 0 && m.pv_s(0x17e) == 0 && rider.is_some() {
                let ahead = me - bike_prog;
                let bp = w.mobys[b].pos;
                let side = sub_rot(m.rot[2], atan2(bp[0] - m.pos[0], bp[1] - m.pos[1]));
                let r = rider.unwrap_or(0);
                if ahead < 10.0 && 2.0 < ahead && ctx.game.rng.randi(1) == 0 {
                    ctx.host.set_anim(&w.mobys[r], if 0.0 < side { 8 } else { 7 }, 0, 12);
                    let t = ctx.game.rng.rand_range(120, 180);
                    w.mobys[id].set_pv_i(0x180, t);
                } else if -5.0 < me - bike_prog && ctx.game.rng.randi(3) == 0 {
                    ctx.host.set_anim(&w.mobys[r], if 0.0 < side { 5 } else { 6 }, 0, 12);
                    let t = ctx.game.rng.rand_range(120, 300);
                    w.mobys[id].set_pv_i(0x180, t);
                } else {
                    w.mobys[id].set_pv_i(0x180, 60);
                }
            }
            let route = w.mobys[id].pv_i(0x110);
            let bp = w.mobys[b].pos;
            let paths = &w.race.paths;
            let n = paths.count(route);
            let mut closest = -1;
            let mut best = 9_999_999.0;
            for i in 0..n {
                let d = dist(bp, paths.point(route, i));
                if d < best { best = d; closest = i; }
            }
            let m = &mut w.mobys[id];
            if closest == 0 || closest == n - 1 {
                m.set_pv_f(0x16c, 0.0);
            } else {
                let (next, prev) = (closest + 1, closest - 1);
                let nd = dist(paths.point(route, next), bp);
                let pd = dist(paths.point(route, prev), bp);
                let other = if nd < pd { next } else { prev };
                let c = paths.point(route, closest);
                let mut off = lateral(c, paths.point(route, other), bp);
                let to_next = sub(paths.point(route, next), c);
                let to_bike = sub(bp, c);
                if 0.0 < crate::hero::physics::cross(to_next, to_bike)[2] { off = -off; }
                m.set_pv_f(0x16c, off.clamp(-1.5, 1.5));
            }
            m.set_pv_i(0x174, -1000);
        } else if (w.mobys[id].pv_i(0x174) as f32) < 0.0 {
            w.mobys[id].set_pv_f(0x16c, 0.0);
        }
        let m = &mut w.mobys[id];
        m.set_pv_f(0x134, m.pv_f(0x134).min(0.469_166_7).max(0.147_666_68));
    }
    let route = w.mobys[id].pv_i(0x110);
    let level = ctx.game.level;
    let paths = &w.race.paths;
    let total = paths.step(route) * paths.count(route) as f32;
    let old = w.mobys[id].pv_i(0x114);
    let m = &mut w.mobys[id];
    let d = super::xy_dist(m.pv_v(0xf0), m.pos);
    let near = lead(m);
    let mut t = m.pv_f(0xa8);
    if d < near {
        t += near - d;
        if total < t { t -= total; }
    }
    set_along(paths, m, t);
    let to = m.pv_v(0xf0);
    if 1.047_197_7 < rot_dist(m.rot[2], atan2(to[0] - m.pos[0], to[1] - m.pos[1])) {
        let i = nearest_index(paths, level, m, route, false);
        let t = i as f32 * paths.step(route) + lead(m);
        set_along(paths, m, t);
    }
    let now = m.pv_i(0x114);
    for i in old..=now {
        let f = if i == 0 { paths.step(route) } else { paths.w(route, i) };
        if 1000.0 <= f {
            let o = (m.pv_f(0x16c) + ctx.game.rng.randf(-0.25, 0.25)).clamp(-1.5, 1.5);
            m.set_pv_f(0x16c, o);
            let t = ctx.game.rng.randf(3.0, 6.0);
            m.set_pv_i(0x174, (t * 60.0) as i32);
        } else if f <= -1000.0 {
            m.set_pv_f(0x16c, 0.0);
            m.set_pv_i(0x174, -1000);
        }
    }
    if 0 < m.pv_i(0x174) {
        let t = m.pv_i(0x174) - 1;
        m.set_pv_i(0x174, t);
        if t == -1 {
            let o = (m.pv_f(0x16c) + ctx.game.rng.randf(-0.25, 0.25)).clamp(-1.5, 1.5);
            m.set_pv_f(0x16c, o);
            let t = ctx.game.rng.randf(3.0, 6.0);
            m.set_pv_i(0x174, (t * 60.0) as i32);
        }
    }
    // The lane's point (the decomp's `toPeer`, read by nothing after).
}

/// `0x404250(m)` (`func_L04_00402d90`): the route's branches. Joba: on the first route at points 186..204 after five
/// walls in a row onto the route `+0x7c` (at 4); at points 44..50 of the first route, not on the short cut, while the
/// bike's gate moby (`+0x1a4`) is shut, one in (0.9 − (10 − skill)·0.03) takes the short cut `+0x5c` (its nearest
/// point on, a new lateral offset) — else, while it is open, nine in ten mark it taken; either way no other branch
/// this tick. Barlow: on the first route at points 426..434 after two walls, in the bike's cuboid `+0x1a0`, onto the
/// route `+0x7c` (at 2); on `+0x6c` at points 171..174 after four walls back onto the first. Then, its short cut's rest
/// counting down (`+0xa1`): at a branch's start (`+0x58[5]`, and with the record's flag 4 `+0x6c[4]`) within 5 of its
/// point, one in two takes it, else it waits 120 ticks; off the first route at the end of its route, back onto the
/// first (its nearest point on, a new lateral offset). Its rest out, the short cut's mark goes.
fn branches(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    let Some(b) = bike_of(w, id) else { return };
    if w.mobys[id].pv_i(0x54) == -1 { return; }
    let mut others = true;
    let route_of = |m: &Moby, o: usize| m.pv_i(o);
    let new_offset = |m: &mut Moby, g: &mut crate::rng::Rng| {
        let o = (m.pv_f(0x16c) + g.randf(-0.25, 0.25)).clamp(-1.5, 1.5);
        m.set_pv_f(0x16c, o);
        let t = g.randf(3.0, 6.0);
        m.set_pv_i(0x174, (t * 60.0) as i32);
    };
    let onto = |w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: i32| {
        w.race.paths.ensure(ctx.level, r);
        w.mobys[id].set_pv_i(0x110, r);
        let k = nearest_index(&w.race.paths, level, &w.mobys[id], r, false);
        let paths = &w.race.paths;
        let m = &mut w.mobys[id];
        let t = k as f32 * paths.step(r) + lead(m);
        set_along(paths, m, t);
    };
    let onto_at = |w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: i32, t: f32| {
        w.race.paths.ensure(ctx.level, r);
        let m = &mut w.mobys[id];
        m.set_pv_i(0x110, r);
        m.set_pv_i(0x114, 0);
        m.set_pv_f(0x16c, 0.0);
        set_along(&w.race.paths, &mut w.mobys[id], t);
        w.mobys[id].set_pv_i(0x174, -1000);
    };
    if level == 0xb {
        let m = &w.mobys[id];
        let first = m.pv_i(0x110) == route_of(m, 0x54);
        let p = m.pv_i(0x124);
        if first && (186..205).contains(&p) && 5 <= m.pv_i(0x1ac) {
            let r = m.pv_i(0x7c);
            onto_at(w, id, ctx, r, 4.0);
        }
        let m = &w.mobys[id];
        if m.pv_i(0x240) == 0 && m.pv_i(0x120) == -1 && (44..51).contains(&m.pv_i(0x124)) {
            let gate = usize::try_from(w.mobys[b].pv_i(0x1a4)).ok().and_then(|g| w.mobys.get(g));
            let shut = gate.is_some_and(|g| g.pv_i(0x14) == 0);
            if shut {
                let skill = w.mobys[id].pv_i(0x1a8);
                if ctx.game.rng.randf(0.0, 1.0) < 0.9 - (10 - skill) as f32 * 0.03 {
                    let r = w.mobys[id].pv_i(0x5c);
                    onto(w, id, ctx, r);
                    let m = &mut w.mobys[id];
                    new_offset(m, &mut ctx.game.rng);
                    m.set_pv_i(0x240, 1);
                    m.set_pv_b(0xa1, 0xb4);
                    others = false;
                    return finish_branches(w, id, ctx, others);
                }
            } else if ctx.game.rng.randf(0.0, 1.0) < 0.9 {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x240, 1);
                m.set_pv_b(0xa1, 0xb4);
            }
            others = false;
        }
    }
    if level == 4 {
        let m = &w.mobys[id];
        let first = m.pv_i(0x110) == route_of(m, 0x54);
        let p = m.pv_i(0x124);
        if first && (426..435).contains(&p) && 2 <= m.pv_i(0x1ac) {
            let cub = w.mobys[b].pv_i(0x1a0);
            if ctx.level.in_cuboid(cub, m.pos) {
                let r = m.pv_i(0x7c);
                onto_at(w, id, ctx, r, 2.0);
            }
        }
        let m = &w.mobys[id];
        if m.pv_i(0x110) == route_of(m, 0x6c) && (171..175).contains(&m.pv_i(0x124)) && 3 < m.pv_i(0x1ac) {
            let r = m.pv_i(0x54);
            onto(w, id, ctx, r);
        }
    }
    finish_branches(w, id, ctx, others)
}

/// The tail of [`branches`] (module docs there).
fn finish_branches(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, others: bool) {
    let level = ctx.game.level;
    let race = w.race.race;
    let rest_out = {
        let m = &mut w.mobys[id];
        let mut a = m.pv_b(0xa1) as i32;
        let r = countdown(&mut a);
        m.set_pv_b(0xa1, a as u8);
        r
    };
    let first = w.mobys[id].pv_i(0x54);
    let back = |w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: i32| {
        w.race.paths.ensure(ctx.level, r);
        w.mobys[id].set_pv_i(0x110, r);
        let k = nearest_index(&w.race.paths, level, &w.mobys[id], r, false);
        let paths = &w.race.paths;
        let m = &mut w.mobys[id];
        let t = k as f32 * paths.step(r) + lead(m);
        set_along(paths, m, t);
        let o = (m.pv_f(0x16c) + ctx.game.rng.randf(-0.25, 0.25)).clamp(-1.5, 1.5);
        m.set_pv_f(0x16c, o);
        let t = ctx.game.rng.randf(3.0, 6.0);
        m.set_pv_i(0x174, (t * 60.0) as i32);
    };
    let at_end = |w: &MobyWorld, id: usize| {
        let m = &w.mobys[id];
        let r = m.pv_i(0x110);
        r != first && m.pv_i(0x114) == w.race.paths.count(r) - 1
    };
    if !rest_out || w.mobys[id].pv_i(0x240) != 0 || w.mobys[id].pv_i(0x110) != first || !others {
        if at_end(w, id) { back(w, id, ctx, first); }
    } else {
        let mut picks: Vec<i32> = (0..5).map(|k| w.mobys[id].pv_i(0x58 + 4 * k)).collect();
        if super::record(level, race).flags & 4 != 0 { picks.extend((0..4).map(|k| w.mobys[id].pv_i(0x6c + 4 * k))); }
        for r in picks {
            if r == -1 { continue; }
            w.race.paths.ensure(ctx.level, r);
            let d = sub(w.race.paths.point(r, 0), w.mobys[id].pv_v(0xf0));
            if dot(d, d) < 25.0 {
                if ctx.game.rng.randi(2) == 0 { w.mobys[id].set_pv_i(0x110, r); } else { w.mobys[id].set_pv_b(0xa1, 0x78); }
            }
        }
        let r = w.mobys[id].pv_i(0x110);
        if r != first {
            back(w, id, ctx, r);
        }
    }
    if w.mobys[id].pv_b(0xa1) == 0 { w.mobys[id].set_pv_i(0x240, 0); }
}

/// `0x403c70(m, s)` (`func_L04_004027b0`): over a jet of its group (`+0x80`; 0x0474 / 0x0cef within √20, inside its
/// box 2.8 × 1.25 × 1.5 in the jet's frame) it takes the jet's boost ticks (the jet's pvar short `+0`; the jet's sound
/// 0 the first tick, not after the race); while they last the boost eases up to 0.1 (1/180 a tick), else down
/// (1/360).
fn jets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let group = w.mobys[id].pv_i(0x80);
    if group == -1 { return; }
    let members = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    let bike_state = bike_of(w, id).map_or(0, |b| w.mobys[b].state);
    let pos = w.mobys[id].pos;
    for j in members {
        let Some(jet) = w.mobys.get(j) else { continue };
        if jet.o_class != super::JET && jet.o_class != super::JET_B { continue; }
        let d = sub(pos, jet.pos);
        if 20.0 < dot(d, d) { continue; }
        let local = [dot(d, jet.rows[0]), dot(d, jet.rows[1]), dot(d, jet.rows[2])];
        if local[0].abs() < 2.8 && local[1].abs() < 1.25 && local[2].abs() < 1.5 {
            let boost = jet.pv_s(0);
            if w.mobys[id].pv_s(0x140) == 0 && bike_state != 3 { ctx.host.play_sound(&w.mobys[j], 0, 0); }
            w.mobys[id].set_pv_s(0x140, boost);
        }
    }
    let m = &mut w.mobys[id];
    let mut b = m.pv_f(0x13c);
    if m.pv_s(0x140) != 0 {
        m.set_pv_s(0x142, 1);
        approach(0.1, 0.005_555_555_7, &mut b);
        m.set_pv_f(0x13c, b);
        if m.pv_s(0x140) != 0 { countdown_s(m, 0x140); }
    } else {
        m.set_pv_s(0x142, 0);
        approach(0.0, 0.002_777_777_8, &mut b);
        m.set_pv_f(0x13c, b);
    }
}

/// `0x4023a0(m, s)` (`func_L04_00400ee0`): the racer's move (the decomp's hover movement). Its ground (`0x31ce20`)
/// and its height over it; on water (surface 7, on the ground) `+0x244`.
/// * **Following the bike** (`+0x1c0` > 0): the bike's frame; a pass beside it starts the swerve in (`+0x1f0` 1, the
///   follow's ease 0.05, the swerve 20° to the side, strength 2); while swerving in its yaw is turned at the bike's
///   (0.9, ±90°, 0.45 after a bump) and the follow's offset eased toward 1 (0.0025, at most 0.2 a tick), else eased
///   slowly (1/3600, at most 0.133); its point: beside the bike by that offset plus 5 along the bike's velocity, in
///   its own frame at least 5 ahead; the bike's pull `+0x63c` 1.75 by how far across it is (at least 1); the aim the
///   bike's speed, more by how far behind (12.5 per unit, held to 1..4 each way, × 3%); off when 60° off its way, the
///   bike airborne, more than 11 away, or the aim under 0.1 (`+0x1f0` 5); behind time it is held to the pace; the
///   speed sprung to the aim (0.027, 0.2, 1/180); the velocity toward the point at the speed (its z kept); the yaw
///   sprung to the bike's (0.03, 0.3, 4.17°); the ticks out: back on its route.
/// * **Else**: on a slow surface its speed eases to 0.05 a little at a time; on plain ground over 0.19 to 0.19 (the
///   spray); else the speed springs to the aim (by its skill: 0.027 + (skill/10)·0.05, 0.0162 early in the race for
///   the slots past 10; half on water). While the bike counts down it stays; else its yaw springs toward its point
///   (faster when slow and in view; quicker on water). The bike's follower takes the bike's speed; behind time held to
///   the pace. The velocity along its yaw at that speed (on water eased in by a fortieth, its push taken off once,
///   held to the speed).
///
/// Then a bump's push (`+0x178`: by its ticks × 0.025); the fall (a bounce's lift; over 0.3 up: down by 12 / 3600 (more
/// on Barlow's last jumps); under: up by 0.23/300); airborne over 0.44 (its ticks), a landing after 13 ticks
/// falling bounces it off water-like ground (its lift and ticks by the fall). Its pitch: the bounce's lift, else
/// tilted to the ground's normal (or in the air nose down 50° along its yaw); the wobble (±0.004). The move
/// ([`slide`], the racer mode): on water slowed 15% on a hit; the walls counted; lost under its height or off Joba's
/// track, by a hit, after two walls bumped while following, or stuck for 26 ticks in the race: crashed (2). A wall
/// (1): its yaw turned away (0.011, 0.3) and its rider flinches 1..4 by the side (its rest 30).
/// A crash: state 3 (its ticks 0), off water, back on the route if following, its target bit and collision off,
/// hidden, a blast (0.42; bursts 4, 4, 4; puffs 8, streaks 4; the flash), its rider hidden.
fn drive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(b) = bike_of(w, id) else { return };
    let level = ctx.game.level;
    let start_pos = w.mobys[id].pos;
    let (gz, surface, gn) = ground(ctx, start_pos);
    w.mobys[id].set_pv_i(SURFACE, surface);
    let height = start_pos[2] - gz;
    let water = surface == 7 && w.mobys[id].pv_b(0xa5) == 0;
    w.mobys[id].set_pv_i(0x244, water as i32);
    let mut speed = 0.0f32;
    let mut limit = 0.0f32;
    let (bpos, brows, byaw, b_air) = { let bm = &w.mobys[b]; (bm.pos, bm.rows, bm.rot[2], bm.pv_s(0x274)) };
    if 0 < w.mobys[id].pv_i(0x1c0) {
        let m = &w.mobys[id];
        let delta = sub(m.pos, bpos);
        let e0 = [dot(delta, brows[0]), dot(delta, brows[1]), dot(delta, brows[2])];
        let side = e0[0].abs();
        let m = &mut w.mobys[id];
        if m.pv_i(0x1f0) == 0 && 1.0 <= m.pv_f(0x1e4) && side < 1.5 && 1.0 <= e0[1].abs() {
            m.set_pv_i(0x1f0, 1);
            m.set_pv_f(0x1e8, 0.05);
            m.set_pv_f(0x1f8, 2.0);
            m.set_pv_f(0x1f4, if 0.0 < e0[1] { 0.349_065_9 } else { -0.349_065_9 });
        }
        m.set_pv_f(0x2b0, 0.0);
        let (mut x, mut v) = (m.pv_f(0x1e4), m.pv_f(0x1ec));
        if m.pv_i(0x1f0) == 1 && b_air == 0 {
            let mut a = sub_rot(atan2(delta[0], delta[1]), m.rot[2]) * 0.9;
            a = a.clamp(-FRAC_PI_2, FRAC_PI_2);
            if 0 < m.pv_i(0x178) { a *= 0.45; }
            m.set_pv_f(0x2b0, a);
            crate::hero::physics::ease(m.pv_f(0x1e8), 0.0025, 0.0025, 0.200_000_02, &mut x, &mut v);
            if m.pv_f(0x1e8) == 1.0 && 1.0 <= x {
                m.set_pv_f(0x1e8, 1.0);
                m.set_pv_i(0x1f0, 0);
            }
        } else {
            crate::hero::physics::ease(m.pv_f(0x1e8), 0.000_277_777_8, 0.000_277_777_8, 0.133_333_34, &mut x, &mut v);
        }
        m.set_pv_f(0x1e4, x);
        m.set_pv_f(0x1ec, v);
        let side_dir = scale(brows[1], if 0.0 < e0[1] { x } else { -x });
        let bvel = w.mobys[b].pv_v(0x290);
        let c0 = add(add(bpos, side_dir), scale(bvel, 5.0));
        let m = &mut w.mobys[id];
        let d = sub(c0, m.pos);
        let mut d0 = [dot(d, m.rows[0]), dot(d, m.rows[1]), dot(d, m.rows[2])];
        let rate = (e0[0].abs() * 0.8).clamp(0.0, 1.0);
        let pull = ((1.0 - rate) * 1.75).max(1.0);
        let bspeed = w.mobys[b].pv_f(0x30c) + w.mobys[b].pv_f(0x27c);
        let b_lost = w.mobys[b].pv_i(0x3a4);
        w.mobys[b].set_pv_f(0x63c, pull);
        let m = &mut w.mobys[id];
        let mut rate = (0.1 - e0[0]) * 12.5;
        if 0.0 < rate {
            rate = if rate < 1.0 { 0.0 } else { rate.min(4.0) };
        } else if -1.0 < rate {
            rate = 0.0;
        } else {
            rate = rate.max(-4.0);
        }
        let mut aim = bspeed;
        aim += aim * (rate * 0.03);
        m.set_pv_f(0x134, aim);
        if 1.047_197_6 < m.pv_f(0x1d4).abs() || 0 < b_lost || 121.0 < dot(e0, e0) || aim < 0.100_000_01 {
            m.set_pv_i(0x1c0, 0);
            m.set_pv_i(0x1f0, 5);
        }
        if 0.0 < m.pv_f(0x2d0) {
            let c = m.pv_f(0x2d4) * 0.016_666_668;
            if c < m.pv_f(0x134) { m.set_pv_f(0x134, c); }
        }
        let (mut s, mut sv) = (m.pv_f(0x138), m.pv_f(0x158));
        spring(m.pv_f(0x134), 0.027, 0.2, 0.005_555_555_7, &mut s, &mut sv);
        m.set_pv_f(0x138, s);
        m.set_pv_f(0x158, sv);
        if d0[0] < 5.0 { d0[0] = 5.0; }
        let world = add(add(scale(m.rows[0], d0[0]), scale(m.rows[1], d0[1])), scale(m.rows[2], d0[2]));
        let l = len(world);
        let mut vel = if l != 0.0 { scale(world, s / l) } else { world };
        vel[2] = m.pv_f(0xe8);
        m.set_pv_v(0xd0, vel);
        let (mut yaw, mut yv) = (m.rot[2], m.pv_f(0x130));
        turn_spring(byaw, 0.03, 0.3, 0.072_722_06, &mut yaw, &mut yv, 0);
        m.rot[2] = yaw;
        m.set_pv_f(0x130, yv);
        let t = m.pv_i(0x1c0) - 1;
        m.set_pv_i(0x1c0, t);
        if t <= 0 { back_on_route(w, id, ctx, false); }
    } else {
        let mut idle = true;
        let clock = w.race.clock;
        let bstate = (w.mobys[b].state, w.mobys[b].timer);
        let in_sight = in_view(ctx, &w.mobys[id], w.mobys[id].draw_dist as f32);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x2b0, 0.0);
        m.set_pv_i(0x2b4, 0);
        m.set_pv_i(0x2b8, 0);
        m.set_pv_i(0x1f4, 0);
        let air = m.pv_b(0xa5) != 0;
        let mut s = m.pv_f(0x138);
        if m.pv_i(0x144) != 0 && !air {
            approach(0.050_000_004, 0.003_055_555_7, &mut s);
            idle = false;
        } else if surface == 0 && !air {
            if 0.191_666_68 < s {
                approach(0.191_666_68, 0.003_194_444_6, &mut s);
                idle = false;
            }
            m.set_pv_s(0x140, 0);
        }
        m.set_pv_f(0x138, s);
        if idle {
            let rate = if clock < 240 && 11 <= m.pv_i(0x9c) { 0.0162 } else { (m.pv_i(0x1a8) / 10) as f32 * 0.05 + 0.027 };
            let (mut s, mut sv) = (m.pv_f(0x138), m.pv_f(0x158));
            spring(m.pv_f(0x134), if water { rate * 0.5 } else { rate }, 0.2, if water { 0.002_777_777_8 } else { 0.005_555_555_7 }, &mut s, &mut sv);
            m.set_pv_f(0x138, s);
            m.set_pv_f(0x158, sv);
        }
        if bstate.0 == 2 && (3..=4).contains(&bstate.1) {
            m.set_pv_f(0x138, 0.0);
            speed = 0.0;
        } else {
            speed = m.pv_f(0x138) + m.pv_f(0x13c);
            if m.pv_i(0x1f0) == 5 && 0.33225 <= m.pv_f(0x138) { m.set_pv_i(0x1f0, 0); }
            if m.pv_i(0x1f0) == 4 && m.pv_i(0x168) == 0 { m.set_pv_i(0x1f0, 0); }
            let to = m.pv_v(0xf0);
            let heading = atan2(to[0] - m.pos[0], to[1] - m.pos[1]);
            let mut t = (m.pv_f(0x138) * 2.708_803_7).min(1.0);
            if !in_sight { t = 0.0; }
            limit = 0.072_722_06 + (0.034_906_592 - 0.072_722_06) * t;
            let mut factor = ((1.0 - t) + 1.0).min(1.9);
            if water {
                limit *= 1.25;
                factor = 2.75;
            }
            let damping = factor * 0.03;
            limit = limit.max(2.908_882_6e-5);
            let (mut yaw, mut yv) = (m.rot[2], m.pv_f(0x130));
            turn_spring(heading, damping, 0.3, limit, &mut yaw, &mut yv, 0);
            m.rot[2] = yaw;
            m.set_pv_f(0x130, yv);
        }
        if ai::link(&w.mobys[b], 0x730) == Some(id) { speed = w.mobys[b].pv_f(0x30c) + w.mobys[b].pv_f(0x27c); }
        let m = &mut w.mobys[id];
        if 0.0 < m.pv_f(0x2d0) { speed = speed.min(m.pv_f(0x2d4) * 0.016_666_668); }
        let yaw = m.rot[2];
        let mut d = [yaw.cos() * speed, yaw.sin() * speed, 0.0];
        if water {
            let mut vel = m.pv_v(0xd0);
            if m.pv_i(0x178) != 0 && m.pv_i(0x248) != 0 {
                vel = sub(vel, scale(m.pv_v(0x100), m.pv_i(0x178) as f32 * 0.025));
                m.set_pv_i(0x248, 0);
            }
            d = scale(d, 0.025);
            vel = super::cap(add(vel, d), speed);
            m.set_pv_v(0xd0, vel);
        } else {
            m.set_pv_v(0xd0, d);
        }
    }
    let m = &mut w.mobys[id];
    if !countdown_i(m, 0x178) {
        let v = add(m.pv_v(0xd0), scale(m.pv_v(0x100), m.pv_i(0x178) as f32 * 0.025));
        m.set_pv_v(0xd0, v);
        m.set_pv_i(0x248, 1);
    }
    let prev_z = m.pv_f(0xe8);
    let vz = if 8 <= m.pv_i(0x148) {
        prev_z + 0.003_333_333_4
    } else if 0.3 < height {
        let extra = if level == 4 && m.pv_b(0xa5) != 0 && m.pv_i(0x120) == -1 && 476 <= m.pv_i(0x124) {
            (m.pv_f(0x13c) * 9.999_999).clamp(0.0, 1.0) * 3.0 + 1.5
        } else {
            0.0
        };
        let t = ((height - 0.2 + 0.1) * 3.333_333_3).clamp(0.0, 1.0);
        prev_z - (extra + 12.0) * t * 0.000_277_777_8
    } else {
        let t = ((0.2 - height + 0.1) * 5.0).clamp(0.0, 1.0) * 0.23;
        prev_z + t * 0.003_333_333_4
    };
    m.set_pv_f(0xd8, vz);
    if 0.44 < height {
        m.set_pv_b(0xa5, 1);
        m.set_pv_s(0xa6, m.pv_s(0xa6).wrapping_add(1));
    } else {
        if m.pv_b(0xa5) != 0 && 13 <= m.pv_s(0xa6) && m.pv_f(0x154) < 0.0 && surface != 0 && m.pv_b(0xa4) == 0 && surface != 5 {
            let sink = m.pv_f(0x154).min(-0.05);
            m.set_pv_f(0x154, sink);
            let t = (1.0 - (sink + 0.35) * 2.857_142_9).clamp(0.0, 1.0);
            m.set_pv_i(0x148, (t * 5.0) as i32 + 15);
            m.set_pv_b(0xa4, 1);
            m.set_pv_f(0x14c, t.max(0.5) * 0.06);
            m.set_pv_f(0xe8, sink * -0.3);
        }
        m.set_pv_b(0xa5, 0);
        m.set_pv_s(0xa6, 0);
    }
    let roll = m.rot[0];
    if m.pv_b(0xa4) != 0 {
        m.rot[1] += m.pv_f(0x14c);
        countdown_i(m, 0x148);
        if m.pv_i(0x148) == 0 { m.set_pv_b(0xa4, 0); }
    } else {
        let n = if m.pv_b(0xa5) == 0 { gn } else { crate::moby::classes::shot::polar(1.0, add_rot(m.rot[2], PI), 0.872_664_7) };
        let (rows, mut rot, mut v) = (m.rows, m.rot, m.pv_f(0x12c));
        super::tilt(&rows, &mut rot, &mut v, n, 0.03, 0.3, 0.0);
        m.rot = rot;
        m.set_pv_f(0x12c, v);
    }
    m.rot[0] = roll;
    let ph = add_rot(m.pv_f(0x150), 0.15);
    m.set_pv_f(0x150, ph);
    m.rot[1] += ph.sin() * 0.004;
    m.rot[0] += ph.cos() * 0.004;
    let vel = m.pv_v(0xd0);
    m.set_pv_v(0xe0, vel);
    m.set_pv_f(0x154, m.pv_f(0xe8));
    let (aim, hgt, rad) = (m.pv_f(0x134), m.pv_f(0x94), m.pv_f(0x90));
    let (mut pos, mut vel, mut prev) = (m.pos, m.pv_v(0xd0), m.pv_v(0xe0));
    let (mut result, s) = slide(w, ctx, id, false, &mut pos, &mut vel, &mut prev, speed, limit, aim, hgt, rad);
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.set_pv_v(0xe0, prev);
    m.set_pv_i(0x144, s.flag);
    if result != 0 && m.pv_i(0x244) != 0 { vel = scale(vel, 0.85); }
    m.set_pv_v(0xd0, vel);
    if s.wall { m.set_pv_i(0x1ac, m.pv_i(0x1ac) + 1); } else { m.set_pv_i(0x1ac, 0); }
    if m.pos[2] < m.pv_f(0x98) || off_track(level, m.pv_i(0x120), m.pv_i(0x124), m.pos[2]) { result = 2; }
    let mut v60 = s.bounce;
    if w.hit_message(id).is_some() {
        result = 2;
        v60 = w.mobys[id].pv_v(0xd0);
    }
    w.clear_hit(id);
    let btimer = w.mobys[b].timer;
    let m = &mut w.mobys[id];
    if 2 <= m.pv_i(0x1ac) && 0 < m.pv_i(0x1c0) && 0 < m.pv_i(0x178) && 4 <= m.pv_i(0x1bc) { result = 2; }
    if (5..=6).contains(&btimer) {
        let mut d = sub(m.pos, start_pos);
        d[2] = 0.0;
        if dot(d, d) < 0.01 {
            m.set_pv_i(0x24c, m.pv_i(0x24c) + 1);
            if 26 <= m.pv_i(0x24c) { result = 2; }
        } else {
            m.set_pv_i(0x24c, 0);
        }
    }
    match result {
        1 => {
            let h = s.hit.unwrap_or(m.pos);
            let to = atan2(h[0] - m.pos[0], h[1] - m.pos[1]);
            let (mut yaw, mut yv) = (m.rot[2], m.pv_f(0x130));
            turn_spring(add_rot(to, PI), 0.011, 0.3, 6.0, &mut yaw, &mut yv, 0);
            m.rot[2] = yaw;
            m.set_pv_f(0x130, yv);
            if let Some(r) = rider_of(w, id) {
                let m = &mut w.mobys[id];
                let a = sub_rot(m.rot[2], to);
                let dir = if 0.523_598_85 < a && a < 2.617_994 { 2 } else if a < -0.523_598_85 && -2.617_994 < a { 1 } else if a.abs() < 0.523_598_85 { 4 } else { 3 };
                if m.pv_s(0x17e) != 0 && m.pv_s(0x17c) == dir {
                    m.set_pv_s(0x17e, 30);
                } else {
                    m.set_pv_s(0x17e, 30);
                    m.set_pv_s(0x17c, dir);
                    ctx.host.set_anim(&w.mobys[r], dir as u8, 3, 3);
                }
            }
        }
        2 => {
            let at = add(m.pos, m.pv_v(0xd0));
            m.state_ticks = 0;
            m.state = 3;
            m.set_pv_i(0x244, 0);
            let following = 0 < m.pv_i(0x1c0);
            if following { back_on_route(w, id, ctx, true); }
            let m = &mut w.mobys[id];
            m.mode &= 0xefff;
            m.has_collision = false;
            // `v60` (the bounce, or the velocity on a hit) throws the debris (not ported).
            let _ = v60;
            m.mode = (m.mode | 0x41) & 0xfbff;
            crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 0.42, bursts: [4, 4, 4], puffs: 8, streaks: 4, light: true, shake: false });
            if let Some(r) = rider_of(w, id) { w.mobys[r].mode |= 0x41; }
        }
        _ => {}
    }
}

/// `0x405978(m, s)` (`func_L04_004044b8`): through the half way's cuboid (`+0x88`) it is marked; through the finish's
/// (`+0x84`) marked, a lap: its rider cheers (sequence 9) past the first; under 4 laps the time it is behind grows by
/// its race time less this lap's (not below 0), the lap's ticks and the mark cleared, the lap counted; at the third
/// with the bike racing its finish time is the race clock.
fn laps(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if ctx.level.in_cuboid(m.pv_i(0x88), m.pos) { w.mobys[id].set_pv_i(0x15c, 1); }
    let m = &w.mobys[id];
    if !ctx.level.in_cuboid(m.pv_i(0x84), m.pos) || m.pv_i(0x15c) == 0 { return; }
    let lap = m.pv_i(0x160);
    if 0 < lap {
        if let Some(r) = rider_of(w, id) { ctx.host.set_anim(&w.mobys[r], 9, 0, 0xc); }
    }
    if lap < 4 {
        let clock = w.race.clock;
        let racing = bike_of(w, id).is_some_and(|b| w.mobys[b].state == 2);
        let m = &mut w.mobys[id];
        let behind = (m.pv_f(0x2d0) + m.pv_f(0x1b0) - m.pv_i(0x1b8) as f32 * 0.016_666_667).max(0.0);
        m.set_pv_f(0x2d0, behind);
        m.set_pv_i(0x1b8, 0);
        m.set_pv_i(0x15c, 0);
        m.set_pv_i(0x160, lap + 1);
        if lap + 1 == 3 && racing { m.set_pv_i(0x1a0, clock); }
    }
}

/// `0x4055c0(m, s)` (`func_L04_00404100`): the lean — its roll sprung (0.035, 0.45, 1.58°) to −30° × the turn
/// (`a·|a|` slow, `1 − (1 − c)²` fast, by its speed) or to a swerve's roll (its strength ×) until it is reached; its
/// rider at it (a sequence 1..9 played out goes back to 0; shown within 30, its matrix placed here).
fn looks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let total = (m.pv_f(0x138) + m.pv_f(0x13c)).max(0.083_333_336);
    let mut a = m.pv_f(0x130) * 13.750_985;
    let b = (total * 2.708_803_7).min(1.0);
    let mut c = m.pv_f(0x130) * 28.647_884;
    a *= a.abs();
    c = 1.0 - (1.0 - c) * (1.0 - c);
    let (factor, angle) = if m.pv_f(0x1f4) != 0.0 {
        if (m.rot[0] - m.pv_f(0x1f4)).abs() < 0.003 { m.set_pv_f(0x1f4, 0.0); }
        (m.pv_f(0x1f8), m.pv_f(0x1f4))
    } else {
        (1.0, ((1.0 - b) * a + b * c).clamp(-1.0, 1.0) * -0.523_598_85)
    };
    let (mut r, mut v) = (m.rot[0], m.pv_f(0x128));
    turn_spring(angle, factor * 0.035, 0.45, factor * 0.027_634_384, &mut r, &mut v, 0);
    m.rot[0] = r;
    m.set_pv_f(0x128, v);
    match rider_of(w, id) {
        Some(r) => {
            if let Some(a) = ctx.host.anim(&w.mobys[r]) {
                if a.flags & 2 != 0 && a.seq_b != 0 { ctx.host.set_anim(&w.mobys[r], 0, 0, 3); }
            }
            let (p, rot) = (w.mobys[id].pos, w.mobys[id].rot);
            let rm = &mut w.mobys[r];
            rm.draw_dist = 30;
            rm.drawn = 1;
            rm.pos = p;
            rm.rot = rot;
            rm.update_dist = 0;
            ctx.host.advance(&w.mobys[r]);
            w.rebuild(r, &*ctx.host);
            w.mobys[r].mode |= 6;
            w.mobys[r].drive = true;
        }
        None => {
            let r = make_rider(w, id, ctx);
            ai::set_link(&mut w.mobys[id], 0x8c, r);
        }
    }
}
