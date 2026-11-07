//! The museum's shooting exhibit (L21 addresses, the decomp's L30).
//!
//! * **0x11d4** ([`update_11d4`], `0x414f70`, 2 placed; the decomp's `func_US2_L30_00414F70`): a gun that fires the
//!   thugs' kind of shot (crate::moby::classes::thugs's 0x0ea7) as its sequence passes frame time 1.
//!   * Its pvars: `+0x00` the kind of shot (0 flying; else one that stays, and the gun is then gone), `+0x04` /
//!     `+0x08` the cuboids that offer its two editors (−1 none), `+0x0c` the editor's menu, `+0x10..+0x1c` a colour's
//!     bytes the editors show (read by nothing else).
//!   * **0** (set-up): the colour's bytes from `$gp+0x2de8`, the shots' spin `$gp+0x2dc4` = 5 (the port's [`SPIN`]:
//!     nothing else writes it outside the editors); then 1.
//!   * **1**: Ratchet in the cuboid `+0x04` (`0x327920` = L00 `0x321858`): the prompt 0x1338 held (`0x32c268` = L00
//!     `0x327ae0`, crate::prompt::Prompt::force); in `+0x08`: the prompt 0x1343.
//!   * **Each tick**: the colour's bytes written back to `$gp+0x2de8` (its low byte the top one's: unread); at frame
//!     time 1 of its sequence (`0x3298b8` = L00 `0x323bf8`, ai::reached) a shot ([`fire`]) from its joint 0
//!     (`0x318ee0` = L00 `0x312728`) at 35 a second along its yaw (`+0xf8`), its damage 0; with `+0x00` the shot is of
//!     the kind that stays, and the gun deleted (`0x317e40`).
//! * **0x11ea** ([`update_11ea`], `0x417108`; the decomp's `func_L30_00416A88`): the shot. Its pvars: `+0x00` its
//!   velocity (a tick), `+0x10` its owner, `+0x14` its damage, `+0x20` 0.15 (unread), `+0x24` its life, `+0x28` the
//!   beam's scroll; its kind is the byte `+0xbc`.
//!   * Kind 0, each tick: life over 240, a 0.6 sphere of its damage (`0x3238e0` = L00 `0x31d180`, ai::attack_sphere,
//!     flags 0x10003) and 1. **0**: the line to its next place (`0x2e5630` = L00 `0x2de878`; its hit, flags 3, on a
//!     moby it meets), else a 0.3 sphere where it is (`0x2e6690` = L00 `0x2df8d8`, flags 0): meeting its owner it
//!     flies on; a moby, a 0.6 sphere where it is (`0x2e8268` = L00 `0x2e14b0`, its damage, flags 0x10003, along its
//!     way × 1.5), the world (× 3), then 1. Else it moves on. **1**: a small green blast (`0x405628` = L16 `0x4074a8`,
//!     crate::moby::blast::burst_green: 0.25, 25 / 20 / 20 sprites, 20 puffs, no streaks, the flash ball) and gone.
//!   * Every tick it lives through (any kind): its rows turned [`SPIN`]° about its left row (`0x327cc8` = L00
//!     `0x321c00`) and its beam drawn (the second list, `0x2ee6a0`; the draw `0x417630` = L13 `0x40f2d0`,
//!     crate::fx::trails::shot_beam).
//!
//! The thugs' shot (L13 `0x40edb0`) differs: no kind, a moby is met only by the line, its sphere at the point met and
//! 0.3 on a moby, flags 1 / 0x10001, a blue blast.
//!
//! Not ported: the editors (states 2 and 3: L2 + R2 held in a cuboid, `0x138320 & 3`; their menu moby 0x118a,
//! `0x413b08` / `0x413ba0`, over the shots' look `$gp+0x2dc4..` and the colour), as the emitter 0x1317's
//! (crate::moby::classes::level21::emitter).

use crate::collision::{coll_line_m, coll_sphere_m, QueryFlags};
use crate::hero::physics::{add, add_rot, atan2, len2, scale, set_len, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, blast, mode, Ctx, MobyWorld};

pub const GUN: i16 = 0x11d4;
pub const SHOT: i16 = 0x11ea;

/// `$gp+0x2dc4` as the gun's set-up leaves it: the shots' turn a tick, in degrees.
pub const SPIN: f32 = 5.0;

const KIND: usize = 0x00;
const CUBOIDS: usize = 0x04;

const S_VEL: usize = 0x00;
const S_OWNER: usize = 0x10;
const S_DAMAGE: usize = 0x14;
const S_LIFE: usize = 0x24;

/// 0x11d4 (`0x414f70`, module docs).
pub fn update_11d4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            for (k, text) in [(0, 0x1338), (1, 0x1343)] {
                let c = w.mobys[id].pv_i(CUBOIDS + 4 * k);
                if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) { ctx.game.prompt.force(1, text, 1); }
            }
        }
        _ => {}
    }
    if !ai::reached(w, id, ctx, 1.0) { return; }
    let m = &w.mobys[id];
    let at = ctx.host.joint_matrix(m, 0).map_or(m.pos, |(_, p)| p);
    let a = m.rot[2];
    let v = set_len([a.cos(), a.sin(), 0.0], f32::from_bits(0x3f15_5556));
    if m.pv_i(KIND) != 0 {
        fire(w, ctx, id, v, at, 1, 0.0);
        w.delete(id);
    } else {
        fire(w, ctx, id, v, at, 0, 0.0);
    }
}

/// `0x417408(owner, vel, pos, kind, damage)`: a shot 0x11ea at `pos` flying `vel` a tick: its update and draw
/// distances 0xff, drawn, its kind, the owner's light, mode | 0x208 | 0x100, its angles along its way (its yaw a
/// quarter turn on; a random roll drawn and lost to the rows from the angles), alpha 0x80, scale `$gp+0x2e6c`
/// (0.0001: its look is its beam); two twinkles at it (`0x33b190` = L00 `0x33a6a0`: 8..10 × 0.3 growing to it,
/// `$gp+0x2dc8` 0x80103080, and × 0.04 to × 0.4, `$gp+0x2dcc` white).
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, vel: V3, pos: V3, kind: u8, damage: f32) -> usize {
    let k = w.create(SHOT);
    let (light, amb) = (w.mobys[owner].light, w.mobys[owner].ambient);
    let m = &mut w.mobys[k];
    m.update = Some(update_11ea);
    m.mode &= !mode::NO_UPDATE;
    (m.update_dist, m.draw_dist, m.drawn, m.timer, m.state) = (0xff, 0xff, 1, kind, 0);
    (m.light, m.ambient) = (light, amb);
    m.pos = pos;
    m.set_pv_v(S_VEL, vel);
    ai::set_link(m, S_OWNER, Some(owner));
    m.set_pv_f(S_DAMAGE, damage);
    m.set_pv_f(0x20, f32::from_bits(0x3e19_999a));
    m.mode |= 0x208;
    m.rot[2] = add_rot(atan2(vel[0], vel[1]), f32::from_bits(0x3fc9_0fdc));
    m.rot[0] = -atan2(len2(vel), vel[2]);
    ctx.game.rng.rand_angle();
    let m = &mut w.mobys[k];
    m.rows = crate::hero::physics::euler_rows(m.rot);
    m.alpha = 0x80;
    m.mode |= mode::KEEP_ROWS;
    m.scale = f32::from_bits(0x38d1_b717);
    m.set_pv_i(S_LIFE, 0);
    let s = ctx.game.rng.randf(8.0, 10.0);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * f32::from_bits(0x3e99_999a), s, 0.0, pos, 15, 0x8010_3080, 0, 10, [0.0; 3], 0);
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * f32::from_bits(0x3d23_d70a), s * f32::from_bits(0x3ecc_cccd), 0.0, pos, 15, 0xffff_ffff, 1, -10, [0.0; 3], 0);
    }
    k
}

/// 0x11ea (`0x417108`, module docs).
pub fn update_11ea(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].timer == 0 {
        let m = &mut w.mobys[id];
        let life = m.pv_i(S_LIFE) + 1;
        m.set_pv_i(S_LIFE, life);
        let dmg = m.pv_f(S_DAMAGE);
        if 0xf0 < life {
            let pos = w.mobys[id].pos;
            ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f19_999a), dmg, 0.0, pos, 0x1_0003, 0, 1, 0);
            w.mobys[id].set_state(1, -1);
        }
        match w.mobys[id].state {
            0 => {
                let m = &w.mobys[id];
                let (pos, vel, owner) = (m.pos, m.pv_v(S_VEL), ai::link(m, S_OWNER));
                let next = add(pos, vel);
                let (coll, scene) = (ctx.coll, ctx.scene);
                let mut t = HitTemplate::new(dmg, Some(id), 3, [vel[0], vel[1], vel[2], 0.0]);
                t.kind = (SHOT as u16 as u32) << 16;
                let line = coll.and_then(|c| coll_line_m(c, scene, pos, next, QueryFlags(0), Some(id)));
                if let Some(o) = line.as_ref() {
                    if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
                }
                // The moby met (`0x1c0098`): the line's, else the sphere's.
                let met = line.map(|o| o.moby).or_else(|| coll.and_then(|c| coll_sphere_m(c, scene, pos, f32::from_bits(0x3e99_999a), QueryFlags(0), Some(id))).map(|o| o.moby));
                if let Some(met) = met.filter(|k| k.is_none() || *k != owner) {
                    let d = scale(vel, if met.is_some() { 1.5 } else { 3.0 });
                    let mut t = HitTemplate::new(dmg, Some(id), 0x1_0003, [d[0], d[1], d[2], 5_627.925]);
                    t.kind = (SHOT as u16 as u32) << 16 | 1 << 8;
                    w.sphere_hit(scene, pos, f32::from_bits(0x3f19_999a), 0, Some(id), Some(&t));
                    w.mobys[id].set_state(1, -1);
                    return;
                }
                w.mobys[id].pos = next;
            }
            1 => {
                let pos = w.mobys[id].pos;
                let b = blast::Blast { size: 0.25, bursts: [25, 20, 20], puffs: 20, streaks: 0, light: true, shake: false };
                blast::burst_green(w, ctx, pos, &b);
                w.delete(id);
                return;
            }
            _ => {}
        }
    }
    let m = &mut w.mobys[id];
    let a = SPIN * f32::from_bits(0x3c8e_fa36);
    let axis = m.rows[1];
    m.rows[2] = crate::camera::rot(m.rows[2], a, axis);
    m.rows[0] = crate::camera::rot(m.rows[0], a, axis);
    w.changed = true;
    w.register_draw(id);
}
