//! The rocks a breaking rock face throws, class **0x0a80** (its update [`update_0a80`], `0x3c4cc8`, from the level's
//! class table at `0x2a4bbc`; made by [`spawn`], `0x3c4e68`; the decomp's shared `func_L16_003C3870` /
//! `func_L16_003C3A10`, also L22 `0x3d1cc8` / `0x3d1e68` and L23 `0x3d9378` / `0x3d9518`), and Snivelak's rock face
//! **0x0e85** ([`update_0e85`], `0x3f69f8`, 6 placed; the decomp's `func_L16_003F54F8`).
//!
//! **A rock** (pvars: `+0x00` its velocity, `+0x10` its spins, `+0x20` its fall a tick, `+0x24` its life): made in its
//! owner's place (`0x322bd8` = L00 `0x32a748`: updated and drawn at any distance, drawn, the owner, its place, angles
//! and light, rebuilt, mode's low bits cleared), at `pos` with the angles `rot`, 0.666..1 of its class's size, moving
//! `randvec(0.05, 0.1167)` a tick, spinning `randvec(10′, 90′)` a tick, falling 7.5..20 a second² (`$gp+0x13c0..`),
//! living 180..240 ticks. Each tick it falls (`0x31edc0` = L00 `0x326618`: its velocity down the gravity's up,
//! crate::gravity::Gravity::up_at), moves, spins; a 1.5 sphere at it meeting the world or a moby (`0x2d8bb0` = L00
//! `0x2df8d8`, flags 0, itself left out) puts it at the pushed-out centre and, moving into the face, bounces it off
//! (the velocity less 1.25 × its share along the face's normal, then half its old speed), its life 0 and its alpha
//! at most 0x60. Its life out (`0x2e7e58` = L00 `0x2eeab0`), it fades 4 a tick and goes below 5.
//!
//! **0x0e85**: **0**: draw distance 0x200, mode | 0x4000; 1. **1**: a hit with the flags 0x80000 (`0x317530`) breaks
//! it: 120 rocks (`$gp+0x2098`, [`rocks`], `0x3f6aa0`) and 200 puffs (`$gp+0x209c`, [`puffs`], `0x3f6c08`) out of
//! its face, its class sound 0 (`0x3584d0` = L00 `0x3605f0`), deleted (`0x30ae50`).
//!
//! Uncertain: the spawner's ranges are L16's (`$gp+0x13c0..0x13d8`); the L22 / L23 copies read their own.

use crate::collision::QueryFlags;
use crate::hero::physics::{add, add_rot, dot, len, scale, set_len, sub, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const ROCK: i16 = 0x0a80;
pub const FACE: i16 = 0x0e85;

const VEL: usize = 0;
const SPIN: usize = 0x10;
const FALL: usize = 0x20;
const LIFE: usize = 0x24;

/// π/10800: a minute of arc.
const MINUTE: f32 = f32::from_bits(0x3998_825c);

/// `0x3c4e68(owner, pos, rot)`: a rock (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, pos: V3, rot: V3) -> Option<usize> {
    w.class(ROCK)?;
    let id = w.create(ROCK);
    let (opos, orot, light, ambient) = { let o = &w.mobys[owner]; (o.pos, o.rot, o.light, o.ambient) };
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.parent = Some(owner);
    (m.pos, m.rot, m.light, m.ambient) = (opos, orot, light, ambient);
    w.rebuild(id, &*ctx.host);
    w.mobys[id].mode &= !7;
    let base = w.class(ROCK).map_or(1.0, |c| c.scale);
    let rng = &mut ctx.game.rng;
    let s = rng.randf(f32::from_bits(0x3f2a_7efa), 1.0);
    let vel = rng.rand_vec(f32::from_bits(0x3d4c_ccce), f32::from_bits(0x3dee_eef0));
    let spin = rng.rand_vec(10.0 * MINUTE, 90.0 * MINUTE);
    let fall = rng.randf(7.5, 20.0) * f32::from_bits(0x3991_a2b4);
    let life = rng.rand_range(0xb4, 0xf0);
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.scale = base * s;
    m.pos = pos;
    m.rot = rot;
    m.set_pv_v(VEL, vel);
    m.set_pv_v(SPIN, spin);
    m.set_pv_f(FALL, fall);
    m.set_pv_i(LIFE, life);
    w.changed = true;
    Some(id)
}

/// 0x0a80 (module docs).
pub fn update_0a80(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    let m = &mut w.mobys[id];
    let vel = sub(m.pv_v(VEL), scale(up, m.pv_f(FALL)));
    m.set_pv_v(VEL, vel);
    m.pos = add(m.pos, vel);
    let spin = m.pv_v(SPIN);
    for k in 0..3 { m.rot[k] = add_rot(m.rot[k], spin[k]); }
    let pos = m.pos;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 1.5, QueryFlags(0), Some(id)));
    if let Some(h) = hit {
        let m = &mut w.mobys[id];
        if let Some(p) = h.pushed_centre { m.pos = p; }
        let mut v = m.pv_v(VEL);
        if dot(v, h.normal) < 0.0 {
            let n = set_len(h.normal, 1.0);
            let speed = len(v);
            v = add(v, scale(n, -dot(v, n) * 1.25));
            v = set_len(v, speed * 0.5);
            m.set_pv_v(VEL, v);
        }
        m.set_pv_i(LIFE, 0);
        if 0x60 < m.alpha { m.alpha = 0x60; }
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(LIFE);
    let out = countdown(&mut t);
    m.set_pv_i(LIFE, t);
    if out {
        if m.alpha < 5 {
            w.delete(id);
            return;
        }
        m.alpha -= 4;
    }
    w.changed = true;
}

/// A random point on the face: x ±0..26, y ±0..8, z 0..30, none with |x| < 9 and |z| < 25; in its frame by its scale's
/// share of its class's.
fn face_point(w: &MobyWorld, id: usize, ctx: &mut Ctx) -> Option<V3> {
    let rng = &mut ctx.game.rng;
    let p = [rng.randf_signed(0.0, 26.0), rng.randf_signed(0.0, 8.0), rng.randf(0.0, 30.0)];
    if p[0].abs() < 9.0 && p[2].abs() < 25.0 { return None; }
    let m = &w.mobys[id];
    let k = m.scale / w.class(m.o_class).map_or(1.0, |c| c.scale);
    let p = scale(p, k);
    Some(add(std::array::from_fn(|i| p[0] * m.rows[0][i] + p[1] * m.rows[1][i] + p[2] * m.rows[2][i]), m.pos))
}

/// `0x3f6aa0`: 120 tries of a rock at a [`face_point`], at random angles.
fn rocks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for _ in 0..120 {
        let Some(p) = face_point(w, id, ctx) else { continue };
        let rng = &mut ctx.game.rng;
        let rot = [rng.rand_angle(), rng.rand_angle(), rng.rand_angle()];
        spawn(w, ctx, id, p, rot);
    }
}

/// `0x3f6c08`: 200 tries of a still puff (`0x32a920` = L00 `0x3327e8`, crate::particles::staged, the def −1) at a
/// [`face_point`]: between 0x80808080 / 0x80406080 and 0x20808080 / 0x20406080 (`0x2e93f0` = L00 `0x2f0000`), 6 or
/// 5 across (`$gp+0x20b0`), in 1..5 ticks, easing 60..180, out 60.
fn puffs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for _ in 0..200 {
        let Some(p) = face_point(w, id, ctx) else { continue };
        let rng = &mut ctx.game.rng;
        let c0 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x8080_8080, 0x8040_6080);
        let c1 = crate::hud::lerp_rgba(rng.randf(0.0, 1.0), 0x2080_8080, 0x2040_6080);
        let size = [6.0, 5.0][rng.randi(2) as usize];
        let a = rng.rand_range(1, 5);
        let b = rng.rand_range(0x3c, 0xb4);
        let Some(sys) = ctx.parts.as_deref_mut() else { continue };
        let v = [0.0, 0.0, 0.0, size];
        crate::particles::staged(sys, &mut ctx.game.rng, p, v, v, c0, c1, a as i16, b as i16, 0x3c, -1);
    }
}

/// 0x0e85 (module docs).
pub fn update_0e85(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.draw_dist = 0x200;
            m.mode |= crate::moby::mode::LINE;
            m.set_state(1, -1);
        }
        1 => {
            if !w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0008_0000 != 0) { return; }
            rocks(w, id, ctx);
            puffs(w, id, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
        }
        _ => {}
    }
}
