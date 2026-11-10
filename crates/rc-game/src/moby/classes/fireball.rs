//! The fireball **0x0b7f** (L00 `0x3e0b90`, L06 `0x41e930`, L07 `0x3cb618`, L11 `0x3fa108`, L14 `0x3edf90`: the
//! decomp's `func_L00_003DF710`; made by [`shot`], L00 `0x3e1118` = `func_003280C8`, L07 `0x3cbba0`; its draw L07
//! `0x3cbcc8`). Its makers: level 7's gunner (crate::moby::classes::level07::gunner), level 6's sentry
//! (crate::moby::classes::level06::sentry), level 11's fighters (crate::moby::classes::level11::fighters) and level
//! 14's rappeller (crate::moby::classes::level14::rappeller). L07 addresses below.
//!
//! **Each tick** ([`update_0b7f`]): spinning (`+0x18` by `+0x1c` a tick, its rows from (½, ¼, 1)·the spin), it
//! moves `+0x00` a tick; the step's line (flags 0) or a sphere of 0.3 at it meeting anything but its maker hits there
//! (a sphere of 0.3 from its maker, damage `+0x14`, flags 0x10001) and it blows up the next tick (the general
//! explosion: 10 embers, 16 flares, 9 debris, its light 15; deleted). In flight, three sprites trail it each tick
//! (5 ticks, 0x808080; 1..1.5 → 0.2..0.4, 0.1..0.3 and 0.2..0.4 times 210000 by its size, alphas 0x4f / 0x5f / 0x7f,
//! the last two jittered 0.15), and its draw ([`draw_0b7f`]) two glows (effect texture 0xb, additive): 45 times its
//! scale, 0x103080 at alpha 0x5f..0x6f, and 22.5 times, 0x3f3f3f at 0x6f..0x7f. Its size `+0x20` grows 0.025 a tick
//! to 1, its scale `randf(0.01, 0.0125)` of it.
//!
//! Not ported: the fireball's carry by its maker's platform `+0x24` (`0x318370` / `0x318120` / `0x318290`); the
//! template the fireball's line test carries (`0x2d4778`'s last argument: the sphere's hit stands for it).

use crate::hero::physics::{add, add_rot, euler_rows, scale, V3};
use crate::moby::{ai, Ctx, MobyWorld};

pub const SHOT: i16 = 0x0b7f;
/// `$gp+0x1480` (`0x1b0470`): the fireball's trail colour.
const TRAIL_RGB: u32 = 0x0080_8080;

/// `0x3cbba0(damage, maker, vel, at, 0, platform)`: a fireball 0x0b7f at `at` moving `vel` a tick, its maker's light,
/// a random spin of 12°..36° a tick, alpha 0x5d, its rows kept. Shared (L00 `0x3e1118`, L14 `0x3ee518`).
pub fn shot(w: &mut MobyWorld, maker: usize, ctx: &mut Ctx, damage: f32, vel: V3, at: V3) {
    if w.class(SHOT).is_none() { return; }
    let (light, ambient) = (w.mobys[maker].light, w.mobys[maker].ambient);
    let k = w.create(SHOT);
    let rng = &mut ctx.game.rng;
    let (a, spin, s) = (rng.rand_angle(), rng.randf(f32::from_bits(0x3e56_7752), f32::from_bits(0x3f20_d97d)), rng.randf(f32::from_bits(0x3c23_d70a), f32::from_bits(0x3c4c_cccd)));
    let m = &mut w.mobys[k];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.light = light;
    m.ambient = ambient;
    m.pos = at;
    m.set_pv_v(0, vel);
    ai::set_link(m, 0x10, Some(maker));
    m.set_pv_f(0x14, damage);
    m.set_pv_f(0x18, a);
    m.set_pv_f(0x1c, spin);
    m.set_pv_f(0x20, 1.0);
    m.alpha = 0x5d;
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.update = Some(update_0b7f);
    m.mode &= !crate::moby::mode::NO_UPDATE;
    m.scale = s;
    w.changed = true;
}

/// 0x0b7f (L07 `0x3cb618`, module docs).
pub fn update_0b7f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 {
        let e = crate::moby::blast::Explosion { near: 9.0, size: 1.0, light: 15.0, embers: 10, flares: 0x10, sound: -1, debris: 9, lod: -1, ..Default::default() };
        let at = w.mobys[id].pos;
        crate::moby::blast::explode(w, ctx, Some(id), Some(at), &e);
        w.delete(id);
        return;
    }
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        let p = add_rot(m.pv_f(0x18), m.pv_f(0x1c));
        m.set_pv_f(0x18, p);
        m.rows = euler_rows([p * 0.5, p * 0.25, p]);
        let (from, vel) = (m.pos, m.pv_v(0));
        let to = add(from, vel);
        let maker = ai::link(m, 0x10).filter(|&o| w.mobys.get(o).is_some_and(|x| x.state < 0x80));
        let met = ctx.coll.and_then(|c| {
            crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id))
                .or_else(|| crate::collision::coll_sphere_m(c, ctx.scene, from, f32::from_bits(0x3e99_999a), crate::collision::QueryFlags(0), Some(id)))
        });
        if met.is_some_and(|o| maker.is_none() || o.moby != maker) {
            let damage = w.mobys[id].pv_f(0x14);
            ai::attack_sphere(w, maker.unwrap_or(id), ctx, f32::from_bits(0x3e99_999a), damage, 0.0, from, 0x1_0001, 0, 1, 0);
            w.mobys[id].state = 1;
            w.changed = true;
            return;
        }
        w.mobys[id].pos = to;
        let k = w.mobys[id].pv_f(0x20);
        let back = scale(vel, -0.5);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let rng = &mut ctx.game.rng;
            let big = k * 210_000.0;
            let mut p = to;
            for (spin, alpha, lo, hi, jitter) in [(3u8, 0x4f, 0.2, 0.4, false), (7, 0x5f, 0.1, 0.3, true), (0xc, 0x7f, 0.2, 0.4, true)] {
                if jitter { for c in &mut p { *c += rng.randf_signed(0.0, 0.15); } }
                let s0 = rng.randf(1.0, 1.5) * big;
                let s1 = rng.randf(lo, hi) * big;
                crate::particles::sprite(sys, rng, s0, s1, 0.0, p, back, 5, TRAIL_RGB, spin, alpha, 0, 0);
            }
        }
    }
    let m = &mut w.mobys[id];
    let k = m.pv_f(0x20);
    if k < 1.0 { m.set_pv_f(0x20, k + 0.025); }
    let s = ctx.game.rng.randf(f32::from_bits(0x3c23_d70a), f32::from_bits(0x3c4c_cccd));
    let m = &mut w.mobys[id];
    m.scale = s * m.pv_f(0x20);
    w.changed = true;
    w.register_draw(id);
}

/// `0x3cbcc8` (L07; module docs).
pub fn draw_0b7f(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if 1 < m.state { return; }
    let size = m.scale * m.pv_f(0x20);
    let a = rng.rand_range(0x5f, 0x6f) as u32;
    let roll = rng.rand_angle();
    crate::fx::sprite(out, size * 45.0, 1.0, roll, m.pos, cam, 0xb, 0x0010_3080 | a << 24, true);
    let a = rng.rand_range(0x6f, 0x7f) as u32;
    let roll = rng.rand_angle();
    crate::fx::sprite(out, size * 22.5, 1.0, roll, m.pos, cam, 0xb, 0x003f_3f3f | a << 24, true);
}
