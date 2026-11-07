//! The Heavy Lancer's shot, class 0x0c89 (spawned by `0x420038` from the Lancer's update when the hand is the Heavy
//! Lancer 0x0c05, crate::moby::classes::lancer; update L00 `0x420358`, draw callback `0x420a70`). A red blob from the
//! muzzle at the row's speed (per second / 60): with the arm raised at a wall (`0x18c30c`) back along the hand's row 1;
//! else toward the lock's aim point (its place plus the up there, `0x326448`, × its aim height), in first person toward
//! what the view meets 30 out held within 20° (`0x380ef8`, crate::moby::classes::pulse_rifle::view_aim), or along
//! Ratchet's forward row (`0x18a580`). It flies 30 / speed + 1 ticks.
//!
//! **Each tick** ([`update`]): its trail slot no longer its hand's (`0x32b388`, crate::moby::classes::chopper::Pool):
//! deleted. Its place rides the level's moving frame (`0x380e08`); the lock, while it is not deleted (state 0xfd /
//! 0xfe) or dead ([`dead`] `0x421810`), turns its velocity to the lock's aim point (its speed kept). In flight (state 0)
//! the gravity tracker turns the velocity (`0x37f618`), it steps, and the step's line (`0x2de878`, flags 0, Ratchet
//! left out) carries the row's damage (flags 0x10000, along the velocity with the no-limit 5627.987, kind
//! `class << 16 | 0x0101`). A hit puts it at the point: on the world the long [`impact`] (`0x421120(at, 0xf, 0)`); on
//! a moby with its collision the short one and Shock as item 0x3c (`0x4126b8`, crate::moby::weapon_mods); either way
//! its class sound 0 and hidden (state 1: the lock dropped, mode 0x41, 6 ticks). No hit and its time out: the
//! [`fizzle`] (`0x4215f0`) and hidden. Then its sprites (crate::particles): two red glows (420000, 2 ticks,
//! `0x205080`, `def[0x35]`) and a white one (252000) on it, five shrinking white sprites along the step's first
//! 0.4..2 lengths, every tenth frame a slow halo (`def[0x2d]`); and its draw is asked for. Hidden, its 6 ticks out:
//! the tracker given back (`0x37f5f8`), deleted. Out of the world's 2..1021 box: the same.
//!
//! **The draw** ([`draw`], `0x420a70`): the slot test again; its trail's points ride the moving frame; in flight its
//! place, 0.05 wide, is pushed on (40 at most); the phase (`$gp+0x2444`) down 0.1 (wrapping at −7); with two points
//! or more the ribbons ([`ribbon_quads`], `0x420d20`: FX 0x17 white at 0xff, FX 0xe `0x2020ff` at 0x40 four times as
//! wide; hidden both × its ticks left / 6) over the first 20 points and, with more, over all of them.
//!
//! **pvars**: +0x00 its trail's bank (`$gp+0x2440`, 0..15), +0x10 the trail's points, +0x14 the hand's trail slot,
//! +0x18 the hand's class, +0x1c the lock, +0x20 the velocity (per tick), +0x2c the speed, +0x34 the tracker, +0x38
//! the ticks left, +0x3c 1 / the ticks, +0x40 1.0; from [`TRAIL`] the trail's points (the original keeps them in the
//! hand's slot of the trail buffer `0x268dc0`, bank +0x00; here in the shot, as crate::moby::classes::shot does).
//!
//! Not ported: the impact's nine rings (`0x33ff78`, particle type 0x5c: the port's particles have no such type).

use crate::hero::physics::{add, add_rot, atan2, cross, len, scale, set_len, sub, unit, V3};
use crate::moby::classes::shot::polar;
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};

/// The shot's class.
pub const CLASS: i16 = 0x0c89;
/// The Heavy Lancer's item (its row, crate::hero::items::UPGRADE_ROWS), as the Shock mod takes it.
const ITEM: i32 = 0x3c;
/// Where the trail's points sit in the pvars and how many it keeps.
pub const TRAIL: usize = 0x80;
pub const TRAIL_POINTS: usize = 40;
/// `0x1fe8c0[0x35]` / `[0x2d]`: the glows' frames.
const GLOW_DEF: u8 = 0x35;
const HALO_DEF: u8 = 0x2d;
/// `$gp+0x2448` (`0x1b1438`): the class scale's factor.
const SCALE: f32 = 18.0;
/// `0x3c888889`: 1/60.
const TICK: f32 = 0.016_666_668;

/// The shots' globals: `$gp+0x2440` the next trail bank (0..15), `$gp+0x2444` the ribbons' phase.
#[derive(Clone, Copy, Debug, Default)]
pub struct Globals {
    pub bank: u32,
    pub phase: f32,
}

/// The jitter `0x31ba98(0, j)` on each axis.
fn jitter(g: &mut crate::rng::Rng, p: V3, j: f32) -> V3 { std::array::from_fn(|k| p[k] + g.randf_signed(0.0, j)) }

/// `0x32b388(+0x18, +0x14)`: the hand's trail slot is still its class's.
fn slot_held(w: &MobyWorld, id: usize) -> bool {
    let m = &w.mobys[id];
    usize::try_from(m.pv_i(0x14)).is_ok_and(|s| w.chopper.holds(m.pv_i(0x18) as i16, s))
}

/// Gone: its tracker given back (`0x37f5f8`), deleted (`0x311688`).
fn gone(w: &mut MobyWorld, id: usize) {
    crate::gravity::untrack(w, w.mobys[id].pv_i(0x34));
    w.delete(id);
}

/// `0x420038(at, owner, target)`: a shot from `at` (module docs). None when the class is not loaded or the hand's
/// trail slot is not its own (`0x32b388`: then the shot is deleted again).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, owner: usize, target: Option<usize>) -> Option<usize> {
    if w.class(CLASS).is_none() { return None; }
    let r = crate::hero::items::row(ctx.game, crate::hero::items::LANCER)?;
    let speed = r.speed * TICK;
    let target = target.filter(|&t| w.mobys.get(t).is_some());
    let vel = if ctx.hero.items.raised {
        set_len(w.mobys[owner].rows[1], -speed)
    } else if let Some(t) = target {
        let mut p = w.mobys[t].pos;
        if let Some((_, up)) = aim::health(w, t) { p = add(p, scale(ctx.hero.gravity.up_at(p), up)); }
        set_len(sub(p, at), speed)
    } else if ctx.hero.items.fp.on {
        super::pulse_rifle::view_aim(ctx, at, speed, 30.0, f32::from_bits(0x3eb2_b8c4))
    } else {
        set_len(ctx.hero.rows[0], speed)
    };
    let id = w.create(CLASS);
    let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    let ticks = (30.0 / speed) as i32 + 1;
    let (slot, hand_class) = (w.mobys[owner].pv_s(0x12) as i32, w.mobys[owner].o_class);
    {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.mode = mode::KEEP_ROWS;
        m.drawn = 1;
        m.draw_dist = 0xff;
        m.parent = Some(owner);
        m.state = 0;
        m.set_pv_v(0x20, vel);
        m.pos = at;
        m.set_pv_f(0x2c, speed);
        m.set_pv_i(0x38, ticks);
        m.set_pv_f(0x3c, 1.0 / ticks as f32);
        m.scale = class_scale * SCALE;
        m.set_pv_i(0x1c, target.map_or(0, |t| t as i32));
    }
    // The lock's health block marked.
    if let Some(t) = target.filter(|&t| aim::health(w, t).is_some()) {
        let h = crate::moby::ai::health(&w.mobys[t]);
        let f = w.mobys[t].pv_b(h + 0x1e) | 0x80;
        w.mobys[t].set_pv_b(h + 0x1e, f);
    }
    {
        let m = &mut w.mobys[id];
        m.set_pv_f(0x40, 1.0);
        m.set_pv_i(0x14, slot);
        m.set_pv_i(0x18, hand_class as i32);
    }
    if !slot_held(w, id) {
        w.delete(id);
        return None;
    }
    w.mobys[id].set_pv_i(0x10, 0);
    let g = &mut w.heavy_shot;
    g.phase = 0.0;
    if 0xf < g.bank { g.bank = 0; }
    let bank = g.bank;
    g.bank = if 0xf < bank + 1 { 0 } else { bank + 1 };
    w.mobys[id].set_pv_i(0, bank as i32);
    // `0x37f538(shot, 0, 1, 1, 1)` with the shot's place Ratchet's for the call.
    let k = crate::gravity::track(w, ctx.hero, id, Some(ctx.hero.pos), [1.0; 3]);
    w.mobys[id].set_pv_i(0x34, k);
    // `0x2e7920(0x420a70)`: its draw.
    w.register_draw(id);
    w.rebuild(id, &*ctx.host);
    Some(id)
}

/// `0x421810(m)`: a moby that is no crate (`0x3204e8`) and has a health block with no health left.
fn dead(w: &MobyWorld, m: usize) -> bool { !aim::is_crate(w, m) && aim::health(w, m).is_some_and(|(hp, _)| hp <= 0.0) }

/// State 1: hidden, the lock dropped, 6 ticks left.
fn hide(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.state = 1;
    m.set_pv_i(0x1c, 0);
    m.mode |= 0x41;
    m.set_pv_i(0x38, 6);
}

/// `0x420358` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !slot_held(w, id) {
        gone(w, id);
        return;
    }
    let from = crate::gravity::ride(w, w.mobys[id].pos);
    w.mobys[id].pos = from;
    // The lock: dropped once deleted or dead, else the velocity turned to its aim point.
    if let Some(t) = w.mobys[id].pv_link(0x1c) {
        if w.mobys.get(t).is_none_or(|x| matches!(x.state, 0xfd | 0xfe)) || dead(w, t) {
            w.mobys[id].set_pv_i(0x1c, 0);
        } else {
            let d = sub(aim::aim_point(w, t), from);
            let l = len(d);
            if l == 0.0 {
                w.mobys[id].set_pv_i(0x1c, 0);
            } else {
                let s = w.mobys[id].pv_f(0x2c);
                w.mobys[id].set_pv_v(0x20, scale(d, s / l));
            }
        }
    }
    match w.mobys[id].state {
        0 => {
            let vel = crate::gravity::turn(w, ctx.hero, w.mobys[id].pv_i(0x34), w.mobys[id].pv_v(0x20), None);
            w.mobys[id].set_pv_v(0x20, vel);
            let to = add(from, vel);
            w.mobys[id].pos = to;
            let damage = crate::hero::items::row(ctx.game, crate::hero::items::LANCER).map_or(0.0, |r| r.damage);
            let d = unit(vel);
            let mut t = HitTemplate::new(damage, Some(id), 0x10000, [d[0], d[1], d[2], crate::moby::classes::lancer::NO_KNOCK_LIMIT]);
            t.kind = (w.mobys[id].o_class as u16 as u32) << 16 | 0x101;
            let cam = ctx.host.camera_pos();
            match w.coll_line(ctx, from, to, 0, ctx.hero_moby) {
                Some(o) => {
                    if let Some(m) = o.moby { w.deliver_hit_any(m, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
                    w.mobys[id].pos = o.point;
                    match o.moby {
                        None => {
                            if let Some(sys) = ctx.parts.as_deref_mut() { impact(sys, &mut ctx.game.rng, o.point, cam, false); }
                        }
                        Some(m) if w.mobys[m].has_collision => {
                            if let Some(sys) = ctx.parts.as_deref_mut() { impact(sys, &mut ctx.game.rng, o.point, cam, true); }
                            let owner = w.mobys[id].parent;
                            crate::moby::weapon_mods::shock(w, ctx, m, ITEM, owner);
                        }
                        Some(_) => {}
                    }
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                    hide(w, id);
                }
                None => {
                    let mut n = w.mobys[id].pv_i(0x38);
                    let done = countdown(&mut n);
                    w.mobys[id].set_pv_i(0x38, n);
                    if done {
                        if let Some(sys) = ctx.parts.as_deref_mut() { fizzle(sys, &mut ctx.game.rng, to, cam); }
                        hide(w, id);
                    }
                }
            }
            let at = w.mobys[id].pos;
            if let Some(sys) = ctx.parts.as_deref_mut() { flight_fx(sys, &mut ctx.game.rng, from, at, ctx.game.frame); }
            w.register_draw(id);
        }
        1 => {
            let mut n = w.mobys[id].pv_i(0x38);
            let done = countdown(&mut n);
            w.mobys[id].set_pv_i(0x38, n);
            if done {
                gone(w, id);
                return;
            }
            w.register_draw(id);
        }
        _ => {}
    }
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(2.0..=1021.0).contains(&c)) { gone(w, id); }
}

/// The sprites of a tick in flight from `from` to `at` (module docs).
fn flight_fx(sys: &mut crate::particles::Particles, g: &mut crate::rng::Rng, from: V3, at: V3, frame: i32) {
    use crate::particles::sprite;
    let glow = sys.def_first(GLOW_DEF);
    let spin = g.randi(0xff);
    if let Some(i) = sprite(sys, g, 420_000.0, 420_000.0, 0.0, at, [0.0; 3], 2, 0x20_5080, spin as u8, 0x40, 0x10, 1) { sys.pool.recs[i][2] = glow; }
    let spin = (spin + 0x80) % 0xff;
    if let Some(i) = sprite(sys, g, 420_000.0, 420_000.0, 0.0, at, [0.0; 3], 2, 0x20_5080, spin as u8, 0x40, 0x10, 1) { sys.pool.recs[i][2] = glow; }
    let white = f32::from_bits(0x4876_1801);
    if let Some(i) = sprite(sys, g, white, white, 0.0, at, [0.0; 3], 2, 0xff_ffff, ((spin + 0x40) % 0xff) as u8, 0x30, 0x10, 1) { sys.pool.recs[i][2] = glow; }
    let step = scale(sub(at, from), 0.4);
    let mut q = from;
    for _ in 0..5 {
        let spin = g.randi(0xff);
        let f = g.randf(0.2, 0.6);
        let p = jitter(g, q, 0.1);
        sprite(sys, g, f * 210_000.0, f * 42_000.0, 0.0, p, [0.0; 3], 0xf, 0x80_8080, spin as u8, 0x20, 0, 1);
        q = add(q, step);
    }
    if frame % 10 == 0 {
        let halo = sys.def_first(HALO_DEF);
        if let Some(i) = sprite(sys, g, 2100.0, 420_000.0, 0.0, at, [0.0; 3], 0x1e, 0x20_2080, 0, 0x20, 0, 1) { sys.pool.recs[i][2] = halo; }
    }
}

/// `0x421120(at, _, short)`: twice a cinder (crate::particles::cinder: 30000..60000, falling 0.5, 0..0.5 a second
/// down and out at 55..80° below the level, `0x4f93e5ff` → `0x102434ee`, with embers, 25..60 ticks — `short` 10..20)
/// jittered 0.15 about the point, which moves 0.15 toward the camera before each.
pub fn impact(sys: &mut crate::particles::Particles, g: &mut crate::rng::Rng, at: V3, cam: V3, short: bool) {
    let mut p = at;
    for _ in 0..2 {
        let pitch = g.randf(f32::from_bits(0xbf75_be0d), f32::from_bits(0xbfb2_b8c4));
        let yaw = g.rand_angle();
        let vel = polar(g.randf(0.0, 0.5) * TICK, yaw, pitch);
        p = add(p, set_len(sub(cam, p), 0.15));
        let size = g.randf(30_000.0, 60_000.0);
        let q = jitter(g, p, 0.15);
        let life = if short { g.rand_range(10, 20) } else { g.rand_range(0x19, 0x3c) };
        crate::particles::cinder::cinder(sys, g, size, 0.5, q, vel, 0x4f93_e5ff, 0x1024_34ee, life, 1, None, None, 1);
    }
}

/// `0x4215f0(at)`: twenty white sparks (0.05..0.2 of 210000, jittered 0.1, 0.5..1 a second toward the camera within
/// 80° of yaw and up to 80° up, 30 ticks, growing 500 a tick).
pub fn fizzle(sys: &mut crate::particles::Particles, g: &mut crate::rng::Rng, at: V3, cam: V3) {
    use crate::particles::spark;
    let to_cam = atan2(cam[0] - at[0], cam[1] - at[1]);
    for _ in 0..20 {
        let speed = g.randf(0.5, 1.0);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        // A draw the game makes and drops.
        let _ = g.randf(0.05, 0.2);
        let pitch = g.randf(0.0, 1.396_263_4);
        let yaw = add_rot(to_cam, g.randf_signed(0.0, 1.396_263_4));
        let vel = polar(speed * TICK, yaw, pitch);
        let p = jitter(g, at, 0.1);
        let size = g.randf(0.05, 0.2) * 210_000.0;
        spark(sys, g, size, 500.0, 1.0, 0.0, 1.0, p, vel, 0x1e, 0x7f, 0x7f_7f7f, spin);
    }
}

/// `0x420d20(_, wide, phase, points, 0x17, 0xe, n, inner, outer)`: the ribbons' quads (crate::fx), segment by
/// segment: at each point the corners ± its width along unit((point − camera) × (point − previous)) (the first point
/// toward the second; the outer `wide` times as far); at point k ≥ 1 the colour lerped from `0x1010ff` by
/// f = 1 − k / n (`0x2f0000`) and the alpha × f; the first point's corners the colours as given; on every quad s runs
/// along it from `phase` to `phase + 1` and t across it (0 at +, 1 at −); additive.
#[allow(clippy::too_many_arguments)]
pub fn ribbon_quads(pts: &[[f32; 4]], n: usize, inner: u32, outer: u32, wide: f32, cam: V3, phase: f32, out: &mut Vec<crate::fx::FxQuad>) {
    if pts.len() < 2 { return; }
    let n = n.min(pts.len());
    let p3 = |k: usize| [pts[k][0], pts[k][1], pts[k][2]];
    let side = |k: usize| {
        let p = p3(k);
        let along = if k == 0 { sub(p3(1), p) } else { sub(p, p3(k - 1)) };
        set_len(cross(sub(p, cam), along), pts[k][3])
    };
    let shade = |c: u32, f: f32| crate::hud::lerp_rgba(f, 0x10_10ff, c & 0xff_ffff) | (((c >> 24) as f32 * f) as i32 as u32) << 24;
    let st = [[phase, 0.0], [phase, 1.0], [phase + 1.0, 0.0], [phase + 1.0, 1.0]];
    let (mut p, mut d) = (p3(0), side(0));
    let (mut ci, mut co) = (inner, outer);
    for k in 1..n {
        let (q, e) = (p3(k), side(k));
        let f = 1.0 - k as f32 / n as f32;
        let (ni, no) = (shade(inner, f), shade(outer, f));
        out.push(crate::fx::FxQuad { fx: 0x17, additive: true, corners: [add(p, d), sub(p, d), add(q, e), sub(q, e)], st, rgba: [ci, ci, ni, ni] });
        let (d4, e4) = (scale(d, wide), scale(e, wide));
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [add(p, d4), sub(p, d4), add(q, e4), sub(q, e4)], st, rgba: [co, co, no, no] });
        (p, d, ci, co) = (q, e, ni, no);
    }
}

/// `0x420a70`, the draw callback (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    if !slot_held(w, id) {
        gone(w, id);
        return;
    }
    let n = w.mobys[id].pv_i(0x10).clamp(0, TRAIL_POINTS as i32) as usize;
    for k in 0..n {
        let p = crate::gravity::ride(w, w.mobys[id].pv_v(TRAIL + 0x10 * k));
        w.mobys[id].set_pv_v(TRAIL + 0x10 * k, p);
    }
    let m = &mut w.mobys[id];
    if m.state == 0 {
        let n = (m.pv_i(0x10) + 1).min(TRAIL_POINTS as i32);
        m.set_pv_i(0x10, n);
        for k in (1..n as usize).rev() {
            let o = TRAIL + 0x10 * k;
            let (v, wd) = (m.pv_v(o - 0x10), m.pv_f(o - 4));
            m.set_pv_v(o, v);
            m.set_pv_f(o + 0xc, wd);
        }
        let p = m.pos;
        m.set_pv_v(TRAIL, p);
        m.set_pv_f(TRAIL + 0xc, 0.05);
    }
    let g = &mut w.heavy_shot;
    g.phase -= 0.1;
    if g.phase <= -7.0 { g.phase += 7.0; }
    let phase = g.phase;
    let m = &w.mobys[id];
    let n = m.pv_i(0x10).clamp(0, TRAIL_POINTS as i32) as usize;
    if n < 2 { return; }
    let (mut a1, mut a2) = (0xffu32, 0x40u32);
    if m.state == 1 {
        let f = m.pv_i(0x38) as f32 / 6.0;
        a1 = (255.0 * f) as i32 as u32;
        a2 = (64.0 * f) as i32 as u32;
    }
    let (inner, outer) = (a1 << 24 | 0xff_ffff, a2 << 24 | 0x20_20ff);
    let pts: Vec<[f32; 4]> = (0..n).map(|k| { let o = TRAIL + 0x10 * k; let v = m.pv_v(o); [v[0], v[1], v[2], m.pv_f(o + 0xc)] }).collect();
    ribbon_quads(&pts, n.min(20), inner, outer, 4.0, cam, phase, out);
    if 20 < n { ribbon_quads(&pts, n, inner, outer, 4.0, cam, phase, out); }
}
