//! The thrown rocks, class **0x0a80** (its update [`update_0a80`]: L16 `0x3c4cc8`, L22 `0x3d1cc8`, L23 `0x3d9378`;
//! made by [`spawn`]: L16 `0x3c4e68`, L22 `0x3d1e68`, L23 `0x3d9518`; the decomp's shared `func_L16_003C3870` /
//! `func_L16_003C3A10`): Snivelak's rock face 0x0e85 (crate::moby::classes::level16::rocks) throws them when it breaks,
//! the orbits' asteroid clusters 0x0d3d (crate::moby::classes::asteroid_cluster) when they break up.
//!
//! **A rock** (pvars: `+0x00` its velocity, `+0x10` its spins, `+0x20` its fall a tick, `+0x24` its life): made in its
//! owner's place (`0x322bd8` = L00 `0x32a748`: updated and drawn at any distance, drawn, the owner, its place, angles
//! and light, rebuilt, mode's low bits cleared), at `pos` with the angles `rot`, 0.666..1 of its class's size, moving
//! `randvec(0.05, 0.1167)` a tick, spinning `randvec(10′, 90′)` a tick, falling 7.5..20 a second² (L16
//! `$gp+0x13c0..0x13e0`, L22 `$gp+0x1188..0x11a8`, L23 `$gp+0x11d8..0x11f8`: the same values on the three levels),
//! living 180..240 ticks. Each tick it falls (`0x31edc0` = L00 `0x326618`: its velocity down the gravity's up,
//! crate::gravity::Gravity::up_at), moves, spins; a 1.5 sphere at it meeting the world or a moby (`0x2d8bb0` = L00
//! `0x2df8d8`, flags 0, itself left out) puts it at the pushed-out centre and, moving into the face, bounces it off
//! (the velocity less 1.25 × its share along the face's normal, then half its old speed), its life 0 and its alpha at
//! most 0x60. Its life out (`0x2e7e58` = L00 `0x2eeab0`), it fades 4 a tick and goes below 5. (L16 addresses.)

use crate::collision::QueryFlags;
use crate::hero::physics::{add, add_rot, dot, len, scale, set_len, sub, V3};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0a80;

const VEL: usize = 0;
const SPIN: usize = 0x10;
const FALL: usize = 0x20;
const LIFE: usize = 0x24;

/// π/10800: a minute of arc.
const MINUTE: f32 = f32::from_bits(0x3998_825c);

/// `0x3c4e68(owner, pos, rot)`: a rock (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, pos: V3, rot: V3) -> Option<usize> {
    w.class(CLASS)?;
    let id = w.create(CLASS);
    w.adopt(id, Some(owner), &*ctx.host);
    let base = w.class(CLASS).map_or(1.0, |c| c.scale);
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
