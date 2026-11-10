//! Level 4's small classes (L04 addresses; the copies on other levels noted per class).
//!
//! * **0x0969** (`0x3e9a40`, 16 placed), **0x0e1b** (`0x43cb58`, one) and level 11's **0x0e1c** (L11 `0x459b98`):
//!   an empty update (`jr ra`), crate::moby::classes::props::update_empty.
//!
//! Shared with other levels, so not here: the breakable 0x1293 (crate::moby::classes::breakables), the beacons 0x0a99 /
//! 0x0ab8 (crate::moby::classes::beacon) and the colour-cycling light 0x0cfd (crate::moby::classes::hue_light).

use crate::hero::physics::add;
use crate::moby::{Ctx, MobyWorld};

/// `0x2fc5b8`'s callers' ranges for 0x0f61 (`$gp+0x27b0..`): gravity 10..20, spin 90..360, the velocities' box.
const F61_GRAVITY: (f32, f32) = (10.0, 20.0);
const F61_SPIN: (f32, f32) = (90.0, 360.0);
const F61_H: [f32; 8] = [-8.0, -2.0, 6.0, 0.0, 8.0, 2.0, 12.0, 0.0];

/// 0x0f61 (`0x44c268`, 4 placed): a hidden breakable. **0**: not drawn (`+0x31` = 0), mode | 1; 1. **1**: a hit
/// with the flags 0x4010000 while `+0x68`[0] = 0 breaks it (2); the record let go. **2**: its pieces (`0x34e7b8`;
/// see below: [`F61_H`], spin / gravity as above, all pointed, its sound 0, flags 1: no bolts of
/// their own), five times the level's bolts (`0x3499a8`), and at 4 up its frame (`0x3484d0`) the explosion three
/// times (`0x342128` = L00 `0x320538`, crate::moby::blast::explode: flashes 3 / 1 past 9, size 2.25, light 15, 50
/// embers, 10 billows, 10 flares, the shake, 7 debris; then twice 5 embers, a billow and a flare with the colour
/// shifts 1 and 2, flags 0x810001); deleted.
///
/// Not ported: the tie group `+0x68`[1] its break hides (`$gp−0x5fdc`'s lists, the records `0x1b2b54` + 0x20·i,
/// `+0x1e` | 4); `0x34e7b8` gives each piece its velocity by its place in the pieces' box (x across `h[0]..h[4]` from
/// the box's one side to the other, y and z alike, through its rows, × 0.25..1) — here the pieces take
/// crate::moby::shards::throw_all's (`0x34e4e0`) random velocities in the same ranges.
pub fn update_0f61(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.drawn = 0;
            m.mode |= 1;
            m.set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            crate::moby::shards::throw_all(w, id, ctx, F61_H, F61_SPIN, F61_GRAVITY, 0x7f, 0, -1, 1, 0xffff);
            crate::moby::bolts::level_drop(w, id, 0.0, 5, ctx);
            let m = &w.mobys[id];
            let at = add(crate::hero::physics::scale(m.rows[2], 4.0 * m.scale), m.pos);
            use crate::moby::blast::{explode, Explosion};
            let e = Explosion { flash: 3.0, flash2: 1.0, near: 9.0, size: 2.25, light: 15.0, embers: 0x32, billows: 10, flares: 10, sound: -1, shake: true, debris: 7, lod: -1, colour: 0, flags: 0x81_0001, ..Default::default() };
            explode(w, ctx, Some(id), Some(at), &e);
            for colour in [1, 2] {
                let e = Explosion { near: 9.0, size: 2.25, embers: 5, billows: 1, flares: 1, sound: -1, lod: -1, colour, flags: 0x81_0001, ..Default::default() };
                explode(w, ctx, Some(id), Some(at), &e);
            }
            w.delete(id);
        }
        _ => {}
    }
}

/// Game flag byte `0x1395b8 + k` set (crate::game::GameState::flags).
fn set_flag(ctx: &mut Ctx, k: usize) {
    if let Some(b) = ctx.game.flags.get_mut(k) { *b = 1; }
}

/// 0x0e8b (`0x442890`, one placed) and level 6's **0x0e8f** / **0x0e90** (L06 `0x44c8d0` / `0x44c948`, the same code
/// with their own flags): **0**, with a cuboid `+0x68`[0]: Ratchet in it sets its game flag (0x0e8b `0x1395c8`, "L4
/// reached room frozen th…"; 0x0e8f `0x1395ce`; 0x0e90 `0x1395cf`; the game prints the flag's name) and → 1.
pub fn update_0e8b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let c = m.pv_i(0);
    if m.state != 0 || c == -1 || !ctx.level.in_cuboid(c, ctx.hero.pos) { return; }
    let k = match m.o_class { 0x0e8f => 0x16, 0x0e90 => 0x17, _ => 0x10 };
    set_flag(ctx, k);
    w.mobys[id].set_state(1, -1);
}

/// 0x0e8d (`0x442908`, one placed): **0**, hooked (its once bit 2, `0x34d798`): game flag `0x1395c7` ("L4 repair
/// bike electrol…") and → 1.
pub fn update_0e8d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 || w.mobys[id].once & 4 == 0 { return; }
    set_flag(ctx, 0xf);
    w.mobys[id].set_state(1, -1);
}

/// 0x1157 (`0x456cd0`, one placed): **0**: its mission (`+0xb0`) done on this level: deleted; else 1. **1**: the moby
/// `+0x68`[0] hooked (its once bit 2): the mission done (`0x3345f8` = L00 `0x3132a8`), scene 0x38 queued and the
/// queue started (`0x315bd8` / `0x315d20`, crate::scene), in the level's first frames straight in (`0x395508`),
/// deleted.
pub fn update_1157(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let k = ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize;
        if ctx.game.missions.get(k).is_some_and(|b| b & 0x80 != 0) {
            w.delete(id);
            return;
        }
        w.mobys[id].state = 1;
    }
    let hooked = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|i| w.mobys.get(i)).is_some_and(|m| m.once & 4 != 0);
    if w.mobys[id].state != 1 || !hooked { return; }
    let g = &mut *ctx.game;
    g.mission_done(w.mobys[id].mission);
    g.scenes.queue.push(0x38);
    g.scenes.start_queue(ctx.hero.hp);
    if g.frame < 5 { g.scenes.zoom = 1.2; }
    w.delete(id);
}

/// 0x123f (`0x457dd8`, one placed): **0**: without item 0x27 (`0x1a7b27`) → 1; with it: game flag `0x1395ca` set
/// (or past the level's fourth frame) → deleted when both, else 1; in the first four frames without the flag Ratchet
/// is put at its cuboid `+0x68`[1] (its place and angles, `0x2dfe20` = L00 `0x2b9e68`); its update distance 0xff.
/// **1**: Ratchet in its cuboid `+0x68`[0] with the item: the flag set, deleted.
pub fn update_123f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let item = ctx.game.items.get(0x27).copied().unwrap_or(0) != 0;
    let flag = ctx.game.flags.get(0x12).copied().unwrap_or(0) != 0;
    if w.mobys[id].state == 0 {
        if !item {
            w.mobys[id].state = 1;
        } else if flag || 4 < ctx.game.frame {
            if flag {
                w.delete(id);
                return;
            }
            w.mobys[id].state = 1;
        } else if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(4)) {
            ctx.hero.teleport(p, r);
        }
        w.mobys[id].update_dist = 0xff;
    }
    if w.mobys[id].state == 1 && ctx.level.in_cuboid(w.mobys[id].pv_i(0), ctx.hero.pos) && item {
        set_flag(ctx, 0x12);
        w.delete(id);
    }
}

/// Every member of group `g` (the list `0x1fb220[g]`, with two or more alive) hidden without collision (`hide`), or
/// shown with its class's.
fn group_shown(w: &mut MobyWorld, g: i32, hide: bool) {
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    if super::super::props::group_alive(w, g) < 2 { return; }
    for i in list {
        let solid = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[i];
        if hide {
            m.has_collision = false;
            m.mode |= 0x43;
        } else {
            m.mode &= 0xffbc;
            m.has_collision = solid;
        }
    }
}

/// 0x0d8f (`0x43a290`, one placed): **0**: Ratchet in its cuboid `+0x68`[4] → 1. **1**: its four groups
/// (`+0x68`[0..4]) hidden without collision ([`group_shown`]; the game prints "they're all off") → 2. **2**: Ratchet
/// out of the cuboid → 3. **3**: the groups shown, solid ("they're all on"), deleted.
pub fn update_0d8f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = w.mobys[id].pv_i(0x10);
    if w.mobys[id].state == 0 && ctx.level.in_cuboid(c, ctx.hero.pos) { w.mobys[id].state = 1; }
    if w.mobys[id].state == 1 {
        for k in 0..4 { let g = w.mobys[id].pv_i(4 * k); group_shown(w, g, true); }
        w.mobys[id].state = 2;
    }
    if w.mobys[id].state == 2 && !ctx.level.in_cuboid(c, ctx.hero.pos) { w.mobys[id].state = 3; }
    if w.mobys[id].state == 3 {
        for k in 0..4 { let g = w.mobys[id].pv_i(4 * k); group_shown(w, g, false); }
        w.delete(id);
    }
}

/// 0x0f9c (`0x45d818`, one placed): a hit with the flags 0x10000 (`0x33f8d0`): its sound 0, from 0.75 above it two
/// bursts of 15 debris chunks (`0x349c68` = L00 `0x327648`, crate::moby::shards::throw_chunks: reach 0.5, 3 a
/// second, 0.75..1.25 of their size, pointed, grey `0x50404040`), each moving a random 1/15..0.1 a tick (up 1.5 times
/// its height's size); the record let go, deleted.
pub fn update_0f9c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) { return; }
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let p = w.mobys[id].pos;
    let at = [p[0], p[1], p[2] + 0.75];
    for _ in 0..2 {
        let mut v = ctx.game.rng.rand_vec(f32::from_bits(0x3d88_8889), f32::from_bits(0x3dcc_ccce));
        v[2] = v[2].abs() * 1.5;
        crate::moby::shards::throw_chunks(w, ctx, 0.5, 3.0, (0.75, 1.25), id, at, 0xf, true, v, 0x5040_4040, None);
    }
    w.clear_hit(id);
    w.delete(id);
}

/// 0x0f71 (`0x44d438`, 33 placed; Ghidra has no function there): a lamp. **0**: its glow (`0x362ba0` = L00 `0x33f550`,
/// crate::particles::glow: 5250000 × its scale, pulled 15 × it, at (−21.5, 0, 0) × its scale in its frame, `0x40208080`,
/// lasting; its handle `+0x68`[1]); 1. **1**: a hit with the flags 0x4010000 while `+0x68`[0] = 0 breaks it (2); the
/// record let go. **2**: its pieces (`0x34e4e0`, crate::moby::shards::throw_all: x / y −5..5, z 5..10, spin 90..360,
/// gravity 10..20, all pointed, its sound 0, not through its rows, the level's bolts), 45 puffs of smoke
/// (`0x30dbb0`, crate::particles::staged_box: out of the line from (−15, 0, 0), drifting (−10, −4, −4)..(−3, 5, 4) a
/// second, kind 0), a blast at its place (`0x433d50` = L00 `0x3f8348(5 × its scale, 20, pos, 0, 0x19, 0x14, 0x14,
/// 0x14, 0x2d, 1, 1, ...)`, crate::moby::blast::burst: no shake, its light; the head point it works out first, (−21.5,
/// 0, 0) through its frame, is not used); deleted.
pub fn update_0f71(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (s, pos) = (m.scale, m.pos);
            let h = ctx.parts.as_deref_mut().and_then(|sys| crate::particles::glow(sys, &mut ctx.game.rng, s * 5_250_000.0, s * 15.0, [-21.5 * s, 0.0, 0.0], id, pos, -1, false, 0x4020_8080, -1));
            let m = &mut w.mobys[id];
            m.set_pv_i(4, h.map_or(0, |h| h as i32));
            m.set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            crate::moby::shards::throw_all(w, id, ctx, [-5.0, -5.0, 5.0, 0.0, 5.0, 5.0, 10.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 0, -1, 8, 0xffff);
            let m = &w.mobys[id];
            let (rows, pos, s) = (m.rows, m.pos, m.scale);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::staged_box(sys, &mut ctx.game.rng, &rows, pos, s, [-15.0, 0.0, 0.0], [-15.0, 0.0, 0.0], [-10.0, -4.0, -4.0], [-3.0, 5.0, 4.0], 0x2d, 0);
            }
            let b = crate::moby::blast::Blast { size: 5.0 * s, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false };
            crate::moby::blast::burst(w, ctx, pos, &b);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0c17's timer (the level's word `$gp+0x1ce8`; one placed, so kept in its pvars).
const C17_TIMER: usize = 0x100;

/// 0x0c17 (`0x423960`, one placed): a burning spot. Every tick a hit sphere of 1 at its place (`0x33ed50`: 1 damage,
/// push 1, flags 0x10001, kind bytes 5 / 1); the sphere's world-and-moby test (`0x3000e0` = L00 `0x2df8d8(1, pos, 1,
/// m)`) meeting Ratchet first starts 60 ticks (unless running) of smoke off him: each tick, points through his body
/// (`0x31fa80` → `0x31e100(14, hero, 20, ·, 0xffff)`, crate::moby::shards::random_points), from each a trail puff
/// (`0x332c10`, crate::particles::trail: 0.03 a random way and 0.015 up a tick, `0x7000a0ff` to `0x000000ff`, 20..35
/// ticks, 30 growing 40..60, additive); the timer down.
pub fn update_0c17(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    crate::moby::ai::attack_sphere(w, id, ctx, 1.0, 1.0, 1.0, pos, 0x10001, 5, 1, 0);
    let touched = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 1.0, crate::collision::QueryFlags(1), Some(id))).and_then(|o| o.moby);
    let mut t = w.mobys[id].pv_i(C17_TIMER);
    if touched.is_some() && touched == ctx.hero_moby && t <= 0 { t = 0x3c; }
    if t < 1 {
        w.mobys[id].set_pv_i(C17_TIMER, t);
        return;
    }
    if let Some(h) = ctx.hero_moby {
        let pts = crate::moby::shards::random_points(w, h, ctx, 14.0, 0x14, 0xffff);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let rng = &mut ctx.game.rng;
            for p in pts {
                let (yaw, pitch) = (rng.rand_angle(), rng.rand_angle());
                let r = f32::from_bits(0x3cf5_c290);
                let mut v = [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r];
                v[2] += f32::from_bits(0x3c75_c290);
                let life = rng.rand_range(0x14, 0x23);
                let grow = rng.rand_range(0x28, 0x3c) as i16;
                crate::particles::trail(sys, rng, p, v, 0x7000_a0ff, 0xff, life, 0x1e, grow, true);
            }
        }
    }
    crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(C17_TIMER, t);
}

/// `0x33e388(group, −1)`: the group's live members (not deleted, under state 0x80).
fn live_members(w: &MobyWorld, g: i32) -> usize {
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).map_or(0, |l| l.iter().filter(|&&k| w.mobys.get(k as usize).is_some_and(|m| m.state < 0x80)).count())
}

/// 0x0d61 (`0x43a100`, three placed): a barrier that sinks out of the way once its groups (`+0x68`[0..4]) are
/// cleared. **0**: Ratchet's colour words, mode | 0x800, 1. **1**: any of its groups with a live member keeps it
/// (with `+0x68`[4] in the challenge mode only its first group does) unless its mission's bit 0x80 is set; else its
/// class sound 0, 2. **2**: down 17 a second while at or over z 30; under it, 4.
pub fn update_0d61(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        if let Some(h) = ctx.hero_moby.and_then(|h| w.mobys.get(h)) {
            let (l, a) = (h.light, h.ambient);
            let m = &mut w.mobys[id];
            m.light = l;
            m.ambient = a;
        }
        let m = &mut w.mobys[id];
        m.mode |= 0x800;
        m.state = 1;
    }
    if w.mobys[id].state == 1 {
        let m = &w.mobys[id];
        let groups: Vec<i32> = (0..4).map(|k| m.pv_i(4 * k)).collect();
        let mut held = groups.iter().any(|&g| live_members(w, g) != 0);
        if m.pv_i(0x10) != 0 && ctx.game.challenge && live_members(w, groups[0]) == 0 { held = false; }
        let level = ctx.game.level.max(0) as usize;
        let done = ctx.game.missions.get(level * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
        if !held || done {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].state = 2;
        }
    }
    if w.mobys[id].state == 2 {
        let m = &mut w.mobys[id];
        if 30.0 <= m.pos[2] { m.pos[2] -= f32::from_bits(0x3e91_1112); }
        if m.pos[2] < 30.0 { m.state = 4; }
    }
}
