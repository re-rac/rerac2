//! Tabora's crystals (L08 addresses; the engine functions named by their L08 addresses with the L00 ones the port
//! knows them by).
//!
//! * **0x096a** ([`update_096a`], `0x3d43a8`, 65 placed; the decomp's `func_L08_003D2F50`): a crystal formation, the
//!   twin of Grelbin's ice 0x11c7 (crate::moby::classes::level19::props::update_11c7; both crack and leave through
//!   [`crack`] and [`leave`]).
//!   * Each tick: drawn, its shadow probed within 48 of the camera (`0x32b058` = L00 `0x31da58`, range 45). It is gone
//!     when its uid is in the level's table or counted in the save (with a mission, `+0xb0` ≠ 0xff). In a scene
//!     (`0x1a8f00` = 2) hidden (`+0x31` 0, mode | 1), else shown; nothing more while one plays or is asked for
//!     (`0x3800c0`).
//!   * **Set-up** (state 0): it sinks 0.2..0.7 into the ground under it (`0x32a160`, from 0.5 up); 1.
//!   * While drawn, its glitter ([`glitter`], `0x3cd908`).
//!   * **A hit** with the flags 0x230000 ([`crack`]): its damage (`0x32b348` = L00 `0x31dd48`) off its health
//!     (`+0x20`), a white flash (`+0x50`), sequence 3 cut in at the damage taken so far (at most 15) and held (speed 0);
//!     then at joint 0 a small explosion (`0x32e558` = L00 `0x320538`: flashes 2 and 1 past 9, 4 embers, 2 billows, 5
//!     flares) and up to three chips (class 0x096d, [`emit`]) thrown from points of its body; while it lives one of its
//!     sounds 0..2.
//!   * **At no health**, with its crystal (class 0x08c4) loaded: the crystal left in its place ([`leave`]); at the hit
//!     (or 2 above it) the big explosion (flashes 6 and 4 past 3, size 2, 20 embers, 6 billows, 32 flares, the shake,
//!     three debris); three chips from its place; its sound 3; one nanotech carrier (class 0x0e78, [`throw`],
//!     `0x410f98`) thrown 5 a second a random way, 60..80° up; deleted.
//! * **0x08c4** ([`update_08c4`], `0x3cdba0`; the decomp's `func_US2_L08_003CDBA0`): the crystal a formation leaves.
//!   * **0**: alpha 0x50, mode | 0x800, sequence 0; 1.
//!   * **1**: [`glitter`] while drawn; Ratchet within [`PICKUP_RANGE`] across: its pull 0.1, sequence 1 (20 ticks'
//!     blend) at twice its pull's speed; 2.
//!   * **2**: the pull eases to 1 (0.03 a tick); it flies at Ratchet's chest (0.6 up): its first 180 ticks at 20 a
//!     second times the pull (90 and a 0.8 reach while he drives, his state 0x31), then twice the pull a tick. It
//!     shrinks 0.01 of its class's scale a tick while above 0.1 or near. Arrived and shrunk to 1 % — or Ratchet in
//!     his groups 0x14 / 0x19 or his states 0x39 / 0x6f — it is taken: the formation's sound 3, the crystal count
//!     `0x1a7a22` up one (crate::game::GameState::crystals), the formation's uid
//!     marked gone and counted (`0x381e60`'s steps, crate::game::GameState::uid_gone / count_uid), deleted; with no
//!     formation, crystal or 0x1002 left (`0x436fa0` = L19 `0x42ae68`) skill point 5 (crate::skill_points). Else
//!     [`glitter`].
//! * **0x0e78** ([`update_0e78`], `0x4110b0`; the decomp's `func_L08_0040FAE8`): the carrier of a nanotech cluster
//!   (crate::moby::classes::nanotech) a formation throws: it moves by its velocity; meeting the world or a moby (a line,
//!   else a sphere of 0.3 0.3 up) it is put on the face, its velocity bounced off it and cut to 3/4 (under 0.01 long:
//!   deleted); its velocity falls by its gravity; it turns about its y by its spin. With its cluster gone or out of
//!   [2, 1021]³ it is deleted; else the cluster is put at its place and angles.
//! * **0x096d** / **0x096e** (`0x3d49b8` = L12 `0x400850`): the chips, crate::moby::classes::level12::debris.
//!
//! **The shared pieces**: [`emit`] (L03 `0x327c38`, also L08 `0x335bf0`, L17 `0x3292e8`: chips from a moby), its
//! spawn `0x336230` = L12 `0x32ad40` (crate::moby::classes::level12::debris::spawn), [`glitter`] (also L19
//! `0x3c1a30`), [`crack`] and [`leave`] (with Grelbin's ice).
//!
//! The engine functions: `0x31f7c8` = L00 `0x3127c8` (a joint), `0x328518` the sequence cut in at a frame,
//! `0x3285f0` = L00 `0x31b5f0` (crate::moby::classes::props::set_seq), `0x32e148` / `0x32e208` = L00 `0x320128` /
//! `0x3201e8` (crate::moby::ai::flash_start / flash), `0x32b7a8` = L00 `0x31e100` (crate::moby::shards::random_points),
//! `0x328cb8` = L00 `0x31bbc0` (crate::rng::Rng::rand_vec), `0x349e88` / `0x349d10` = L00 `0x33b5e0` / `0x33b468`
//! (crate::particles::glint / sparkle), `0x333c00` (crate::moby::classes::shot::polar), `0x3afcb0` = L02 `0x3a9bf0`
//! (crate::moby::classes::nanotech::create_at), `0x2ea670` / `0x2eb6d0` the line and sphere tests
//! (crate::collision::coll_line_m / coll_sphere_m), `0x329b48` (L19 `0x31a2d8`, the class's live mobys), `0x36eb38` /
//! `0x36ebe0` the class sounds, `0x31e320` / `0x31e688` = L00 `0x311320` / `0x311688` (create / delete).
//!
//! The formation waits while its place is outside the chunk the level shows (`0x32fb48` = L00 `0x321a78`,
//! crate::level::LevelData::in_variant), after its mission test.
//!
//! Not ported: the reward flag `0x1a903c`; the
//! explosions' direction words (crate::moby::blast has none).

use crate::collision::QueryFlags;
use crate::hero::physics::{add, add_rot, dist2, dot, len, scale, set_len, sub, V3};
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::level12::debris;
use crate::moby::classes::props::set_seq;
use crate::moby::hits::HitRecord;
use crate::moby::{ai, Ctx, MobyWorld};

pub const CRYSTAL: i16 = 0x096a;
pub const LEFT: i16 = 0x08c4;
pub const CARRIER: i16 = 0x0e78;
/// The chips (both run `0x3d49b8`).
pub const CHIP: i16 = 0x096d;
pub const CHIP_B: i16 = 0x096e;
/// `0x436fa0`'s third class.
const OTHER: i16 = 0x1002;

/// The health and flash blocks (`+0x20`, its most `+0x24`; `+0x50`).
const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
/// The hit flags the formations answer.
const HIT_FLAGS: u32 = 0x23_0000;

/// `$gp+0xe90..+0xea8` (`0x1afe80`): 0x08c4's reach across, its flight's speeds (a second; driving), its pull's ease,
/// its first pull and its shrink.
const PICKUP_RANGE: f32 = 20.0;
const FLY: (f32, f32) = (20.0, 90.0);
const PULL_EASE: f32 = 0.03;
const PULL: f32 = 0.1;
const SHRINK: f32 = 0.01;

/// `$gp+0x12b0..+0x12b8` (`0x1b02a0`): the carrier's speed (a second) and its pitch range (degrees).
const THROW_SPEED: f32 = 5.0;
const THROW_PITCH: (f32, f32) = (60.0, 80.0);

/// The skill point taken with the last crystal.
const SKILL_POINT: usize = 5;

/// `0x1aa368`: [`emit`]'s velocity range (a tick).
const EMIT_RANGE: (f32, f32) = (f32::from_bits(0x3d88_8889), f32::from_bits(0x3dcc_ccce));

/// The formation's crack (0x096a's and 0x11c7's): a hit with the flags 0x230000 and its damage
/// (crate::moby::ai::damage_of) above 0 comes off the health `+0x20`, flashes it white (`+0x50`) and cuts sequence
/// `seq` in at the damage taken so far (`+0x24` less the health; at most `most`). The damage and the hit.
pub fn crack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, seq: u8, most: i32) -> (f32, Option<HitRecord>) {
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & HIT_FLAGS != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    if 0.0 < dmg {
        let m = &mut w.mobys[id];
        let hp = m.pv_f(HEALTH) - dmg;
        m.set_pv_f(HEALTH, hp);
        for k in 7..10 { m.set_pv_b(FLASH + k, 0xff); }
        ai::flash_start(m, FLASH);
        let taken = (m.pv_s(HEALTH + 4) as f32 - hp) as i32;
        ctx.host.set_anim(&w.mobys[id], seq, taken.clamp(0, most) as u8, 0);
    }
    (dmg, hit)
}

/// What a broken formation leaves (0x096a's crystal, 0x11c7's 0x1266): a `class` in its place, alpha 0x50, mode
/// | 0x800, 0.65 of its class's scale, update and draw distances 0xff, drawn, state 0, its timer byte 0, lit as
/// Ratchet is (`0x2ab650`), linked back at `+4`. None when the class is not loaded.
pub fn leave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16) -> Option<usize> {
    let s = w.class(class)?.scale * 0.65;
    let n = w.create(class);
    let pos = w.mobys[id].pos;
    let lit = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient));
    let q = &mut w.mobys[n];
    (q.alpha, q.scale, q.update_dist, q.draw_dist, q.drawn, q.state, q.timer, q.pos) = (0x50, s, 0xff, 0xff, 1, 0, 0, pos);
    q.mode |= 0x800;
    if let Some((light, ambient)) = lit { (q.light, q.ambient) = (light, ambient); }
    ai::set_link(q, 4, Some(id));
    w.rebuild(n, &*ctx.host);
    w.changed = true;
    Some(n)
}

/// `0x335bf0(m, first, count, pick, picks, most, alt, offset)` (L03 `0x327c38`): chips (crate::moby::classes::
/// level12::debris::spawn, life 60..120, no spin given) off moby `id`, each moving a random way 1/15..1/10 a tick
/// (its z made upward and half again) plus `offset`; with `alt` their flags 2. `count` of the classes `first`,
/// `first + 1`, … at its place and angles; then, at up to `most` points of its body (crate::moby::shards::
/// random_points, density 1000), one each of `pick + randi(picks)` at random angles. A class not loaded makes none.
#[allow(clippy::too_many_arguments)]
pub fn emit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, first: i32, count: i32, pick: i32, picks: i32, most: i32, alt: bool, offset: V3) {
    let flags = if alt { 2.0 } else { 0.0 };
    let vel = |ctx: &mut Ctx| {
        let mut v = ctx.game.rng.rand_vec(EMIT_RANGE.0, EMIT_RANGE.1);
        v[2] = v[2].abs() * 1.5;
        add(v, offset)
    };
    if 0 < first && 0 < count {
        for i in 0..count {
            let v = vel(ctx);
            let class = (first + i) as i16;
            let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
            if w.class(class).is_some() { debris::spawn(w, ctx, id, class, pos, rot, 0, flags, Some(v), None); }
        }
    }
    if 0 < pick && 0 < picks {
        let points = crate::moby::shards::random_points(w, id, ctx, 1000.0, most.max(0) as usize, 0xffff);
        for &p in points.iter().take(most.max(0) as usize) {
            let v = vel(ctx);
            let g = &mut ctx.game.rng;
            let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
            let class = (pick + g.randi(picks)) as i16;
            if w.class(class).is_some() { debris::spawn(w, ctx, id, class, p, rot, 0, flags, Some(v), None); }
        }
    }
}

/// `$gp+0xeac..+0xee0` (`0x1afe9c`): the glints' speeds (a second: the first 1..2, each next one 0..0.333 more),
/// life, colours and sizes; the ring's two colours, its sparkles' frame list and softness.
const GLINT_SPEED: (f32, f32) = (1.0, 2.0);
const GLINT_MORE: (f32, f32) = (0.0, 0.333);
const GLINT_LIFE: u16 = 0x3c;
const GLINT_RGBA: (u32, u32) = (0x5080_4080, 0x4080_4080);
const GLINT_SIZE: (f32, f32) = (1.0, 0.6);
const RING_RGBA: (u32, u32) = (0x40ff_40ff, 0x10ff_40ff);
const RING_DEF: u8 = 0x35;

/// `0x3cd908` (L19 `0x3c1a30`): a crystal's glitter at its joint 0. One tick in four four glints (crate::particles::
/// glint: the first [`GLINT_RGBA`].0 at a random 1..2 a second, three more each faster by up to 0.333 a second).
/// Every tick a ring of four sparkles (crate::particles::sparkle, two ticks, soft) from 0.3 of the way behind it
/// from the camera, a tenth of that apart, sized 3..6 by its scale against its class's, coloured and turned by the
/// frame counter (`0x1b2508`).
pub fn glitter(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::particles::{glint, sparkle};
    let m = &w.mobys[id];
    let at = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let hero = ctx.hero.pos;
    let k = 0.016_666_668;
    if ctx.game.rng.randi(4) == 0 {
        let mut v = ctx.game.rng.rand_vec(GLINT_SPEED.0 * k, GLINT_SPEED.1 * k);
        let rot = ctx.game.rng.randi(0xff) as u8;
        if let Some(sys) = ctx.parts.as_deref_mut() { glint(sys, GLINT_SIZE.0, at, v, GLINT_RGBA.0, GLINT_LIFE, rot, false, hero); }
        for _ in 0..3 {
            v = add(v, ctx.game.rng.rand_vec(GLINT_MORE.0 * k, GLINT_MORE.1 * k));
            let rot = ctx.game.rng.randi(0xff) as u8;
            if let Some(sys) = ctx.parts.as_deref_mut() { glint(sys, GLINT_SIZE.1, at, v, GLINT_RGBA.1, GLINT_LIFE, rot, false, hero); }
        }
    }
    let back = scale(sub(ctx.host.camera_pos(), at), -0.3);
    let step = scale(back, 0.1);
    let mut p = add(back, at);
    let size = m.scale / w.class(m.o_class).map_or(1.0, |c| c.scale);
    let f = w.frame;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for j in 0..4 {
        let t = (0xff - ((f + j * 20) & 0xff)) as f32;
        let c = crate::hud::lerp_rgba((0.5 - t * 0.003_921_569).abs(), RING_RGBA.0, RING_RGBA.1);
        let mut b = f & 0xff;
        if j & 2 != 0 { b *= 2; }
        if j & 1 != 0 { b = 0xff - b; }
        sparkle(sys, (j as f32 + 3.0) * size, p, c, (b & 0xff) as u8, RING_DEF, true, 2, false, hero);
        p = add(p, step);
    }
}

pub fn update_096a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::shadow_volumes::probe_near(w, id, ctx, 48.0, 45);
    let m = &w.mobys[id];
    if m.mission != 0xff {
        let dead = ctx.game.uid_dead.get(m.uid.max(0) as usize).is_some_and(|&d| d != 0);
        if dead || ctx.game.uid_count(m.uid) != 0 { return w.delete(id); }
    }
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    let m = &mut w.mobys[id];
    if ctx.game.scenes.mode == crate::scene::SCENE {
        m.drawn = 0;
        m.mode |= 1;
    } else {
        m.drawn = 1;
        m.mode &= !1;
    }
    if ctx.game.scenes.in_scene() { return; }
    if w.mobys[id].state == 0 {
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0) - ctx.game.rng.randf(0.2, 0.7);
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        m.set_state(1, -1);
    }
    if w.mobys[id].drawn != 0 { glitter(w, id, ctx); }
    let (dmg, hit) = crack(w, id, ctx, 3, 15);
    if 0.0 < dmg {
        ctx.host.set_anim_speed(&w.mobys[id], 0.0);
        let m = &w.mobys[id];
        let at = ctx.host.joint(m, 0).unwrap_or(m.pos);
        let e = Explosion { flash: 2.0, flash2: 1.0, near: 9.0, size: 1.0, embers: 4, billows: 2, flares: 5, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
        blast::explode(w, ctx, Some(id), Some(at), &e);
        emit(w, id, ctx, -1, 0, CHIP as i32, 1, 3, false, [0.0; 3]);
        let s = ctx.game.rng.rand_range(0, 2);
        if 0.0 < w.mobys[id].pv_f(HEALTH) { ctx.host.play_sound(&w.mobys[id], s, 0); }
    }
    w.clear_hit(id);
    ai::flash(&mut w.mobys[id], FLASH);
    if 0.0 < w.mobys[id].pv_f(HEALTH) { return; }
    let pos = w.mobys[id].pos;
    let target = hit.map_or(add(pos, [0.0, 0.0, 2.0]), |h| [h.point[0], h.point[1], h.point[2]]);
    if leave(w, id, ctx, LEFT).is_none() { return; }
    let e = Explosion {
        flash: 6.0, flash2: 4.0, near: 3.0, size: 2.0, light: 20.0, embers: 20, billows: 6, flares: 32, sound: -1, shake: true,
        debris: 3, lod: -1, flags: 0x81_0001, ..Default::default()
    };
    blast::explode(w, ctx, Some(id), Some(target), &e);
    emit(w, id, ctx, CHIP as i32, 3, 0, 0, 0, false, [0.0; 3]);
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    let yaw = ctx.game.rng.rand_angle();
    let pitch = ctx.game.rng.randf(THROW_PITCH.0, THROW_PITCH.1) * 0.017_453_294;
    let v = crate::moby::classes::shot::polar(THROW_SPEED * 0.016_666_668, yaw, pitch);
    let spin = ctx.game.rng.randf(1.0, 10.0);
    let fall = ctx.game.rng.randf(8.0, 12.0);
    throw(w, id, ctx, add(target, v), v, spin, fall);
    w.delete(id);
}

/// `0x410f98(owner, pos, vel, spin, gravity)`: a carrier (class 0x0e78) at `pos` moving `vel`, turning `spin`° a
/// second about its y and falling `gravity` a second²; update distance 0x40, state 0, random angles; its nanotech
/// cluster made at its place (`+0x1c`). Its pvars: `+0x00` the velocity, `+0x10` the gravity, `+0x14` the spin,
/// `+0x18` its owner (left out of its tests), `+0x1c` the cluster.
pub fn throw(w: &mut MobyWorld, owner: usize, ctx: &mut Ctx, pos: V3, vel: V3, spin: f32, gravity: f32) -> Option<usize> {
    w.class(CARRIER)?;
    let n = w.create(CARRIER);
    let g = &mut ctx.game.rng;
    let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let m = &mut w.mobys[n];
    (m.update_dist, m.timer, m.sub_state, m.state, m.pos, m.rot) = (0x40, 0, 0, 0, pos, rot);
    ai::set_link(m, 0x18, Some(owner));
    m.set_pv_v(0, vel);
    m.set_pv_f(0x14, spin * 0.000_290_888_25);
    m.set_pv_f(0x10, gravity * 0.000_277_777_78);
    let c = crate::moby::classes::nanotech::create_at(w, pos, 0);
    ai::set_link(&mut w.mobys[n], 0x1c, c);
    w.changed = true;
    Some(n)
}

pub fn update_0e78(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (pos, mut vel, owner) = (m.pos, m.pv_v(0), ai::link(m, 0x18));
    let to = add(pos, vel);
    let mut at = None;
    if let Some(coll) = ctx.coll {
        let line = crate::collision::coll_line_m(coll, ctx.scene, pos, to, QueryFlags(0), owner);
        at = match line {
            Some(o) => Some((o.point, o.normal)),
            None => {
                let up = add(to, [0.0, 0.0, 0.3]);
                crate::collision::coll_sphere_m(coll, ctx.scene, up, 0.3, QueryFlags(0), owner)
                    .map(|o| (sub(o.pushed_centre.unwrap_or(up), [0.0, 0.0, 0.3]), o.normal))
            }
        };
    }
    let m = &mut w.mobys[id];
    match at {
        Some((p, n)) => {
            m.pos = p;
            let n = set_len(n, 1.0);
            vel = scale(sub(vel, scale(n, 2.0 * dot(vel, n))), 0.75);
            if len(vel) < 0.01 {
                m.set_pv_v(0, vel);
                return w.delete(id);
            }
        }
        None => m.pos = to,
    }
    vel[2] -= m.pv_f(0x10);
    m.set_pv_v(0, vel);
    m.rot[1] = add_rot(m.rot[1], m.pv_f(0x14));
    let (pos, rot) = (m.pos, m.rot);
    let cluster = ai::link(m, 0x1c).filter(|&c| w.mobys.get(c).is_some_and(|o| o.o_class == crate::moby::classes::nanotech::CLASS && o.state != 0xfe && o.state != 0xfd));
    let out = pos.iter().any(|&x| !(2.0..=1021.0).contains(&x));
    match cluster {
        Some(c) if !out => {
            let o = &mut w.mobys[c];
            (o.pos, o.rot) = (pos, rot);
        }
        _ => w.delete(id),
    }
}

pub fn update_08c4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.state_ticks = m.state_ticks.wrapping_add(1);
    match m.state {
        0 => {
            m.alpha = 0x50;
            m.mode |= 0x800;
            m.set_state(1, -1);
            set_seq(w, id, ctx, 0, 0, 0);
        }
        1 => {
            if dist2(m.pos, ctx.hero.pos) < PICKUP_RANGE {
                m.set_pv_f(0, PULL);
                set_seq(w, id, ctx, 1, 0, 0x14);
                ctx.host.set_anim_speed(&w.mobys[id], PULL + PULL);
                w.mobys[id].set_state(2, -1);
            }
            if w.mobys[id].drawn != 0 { glitter(w, id, ctx); }
        }
        2 => {
            let pull = m.pv_f(0);
            let pull = pull + (1.0 - pull) * PULL_EASE;
            m.set_pv_f(0, pull);
            ctx.host.set_anim_speed(&w.mobys[id], pull + pull);
            let m = &mut w.mobys[id];
            let mut v = sub(ctx.hero.pos, m.pos);
            v[2] += 0.6;
            let d = len(v);
            let mut reach = f32::from_bits(0x3df5_c28f);
            let speed = if (m.state_ticks as u16) < 180 {
                if ctx.hero.state != 0x31 {
                    FLY.0 * (pull * 0.016_666_668)
                } else {
                    reach = f32::from_bits(0x3f4c_cccd);
                    FLY.1 * (pull * 0.016_666_668)
                }
            } else {
                pull + pull
            };
            m.pos = add(m.pos, set_len(v, speed));
            let base = w.class(LEFT).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            if 0.1 < m.scale || d < reach || d < speed { m.scale -= SHRINK * base; }
            let h = &*ctx.hero;
            if (d < speed && m.scale < base * 0.01) || h.group == 0x14 || h.group == 0x19 || h.state == 0x39 || h.state == 0x6f {
                take(w, id, ctx);
            } else {
                glitter(w, id, ctx);
            }
        }
        _ => {}
    }
}

/// 0x08c4 taken (module docs).
fn take(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_sound_of(&w.mobys[id], CRYSTAL, 3, 0);
    ctx.game.crystals = ctx.game.crystals.wrapping_add(1);
    if let Some(f) = ai::link(&w.mobys[id], 4).and_then(|f| w.mobys.get(f)) {
        let (uid, mission) = (f.uid, f.mission);
        ctx.game.uid_gone(uid, mission);
        ctx.game.count_uid(uid);
    }
    w.delete(id);
    let live = |c: i16| w.mobys.iter().filter(|o| o.o_class == c && o.state != 0xfe && o.state != 0xfd).count();
    if live(CRYSTAL) + live(OTHER) + live(LEFT) == 0 { ctx.game.award_skill_point(SKILL_POINT); }
}
