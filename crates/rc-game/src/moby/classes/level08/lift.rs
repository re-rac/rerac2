//! 0x0aaa (`0x3e7028`, 2 placed; the decomp's `func_L08_003E5BB8`, also L09 `0x401350`, L18 `0x3edb70`, L19
//! `0x41fcb8`): a lift between its own place and angles and cuboid `+0x80`'s, a platform (its block at `+0x20`).
//!
//! * **0**: its place and angles kept (`+0x60` / `+0x70`); which end is the top (`+0x8f`: 0 when the cuboid is
//!   higher); its progress `+0x88` 0, its way `+0xbc` 0; 1.
//! * **1**: it sets off (its way flipped, 2) with Ratchet standing on it (`0x189e20`'s moby, his state not 0) — when
//!   it has no trigger (`+0x8e`) or its progress rounds to its top end — or when it is at one end and Ratchet is
//!   nearer the other in height.
//! * **2**: its loop sound 0 (mode 4) kept playing (`+0x90`); its progress on by `1 / +0x84` its way, within 0..1;
//!   its place and angles the cosine ease between the ends; at its end the loop stopped, its sound 1 (not for 0x0d49),
//!   3.
//! * **3**: back to 1 once Ratchet is off it.
//!
//! Below state 4, every tick its platform block gets the tick's move and turn (`0x318730`).

use crate::hero::physics::{add, add_rot, scale, sub, sub_rot, V3};
use crate::moby::{platform, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0aaa;

fn end(ctx: &Ctx, i: i32) -> Option<(V3, V3)> { ctx.level.cuboid_place(i) }

pub fn update_0aaa(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let on = ctx.hero.ground_moby == Some(id) && ctx.hero.state != 0;
    let hz = ctx.hero.pos[2];
    let far = end(ctx, w.mobys[id].pv_i(0x80));
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x90, -1);
            let (pos, rot) = (m.pos, m.rot);
            m.set_pv_v(0x60, pos);
            m.set_pv_v(0x70, rot);
            let top = far.is_some_and(|(p, _)| pos[2] < p[2]);
            m.set_pv_b(0x8f, if top { 0 } else { 1 });
            m.set_pv_f(0x88, 0.0);
            m.timer = 0;
            m.set_state(1, -1);
        }
        1 => {
            let Some((fp, _)) = far else { return };
            let m = &mut w.mobys[id];
            let t = m.pv_f(0x88);
            let (d1, d2) = ((m.pv_f(0x68) - hz).abs(), (fp[2] - hz).abs());
            let at_end = t.round() as i32 == m.pv_b(0x8f) as i32;
            if ((m.pv_b(0x8e) == 0 || at_end) && on) || (t < 0.1 && d2 < d1) || (0.9 < t && d1 < d2) {
                m.timer ^= 1;
                m.set_state(2, -1);
            }
        }
        2 => {
            let Some((fp, fr)) = far else { return };
            let h = w.mobys[id].pv_i(0x90);
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x90, h);
            }
            let m = &mut w.mobys[id];
            let step = 1.0 / m.pv_f(0x84);
            let up = m.timer != 0;
            let t = (m.pv_f(0x88) + if up { step } else { -step }).clamp(0.0, 1.0);
            m.set_pv_f(0x88, t);
            let e = (1.0 - (t * std::f32::consts::PI).cos()) * 0.5;
            let (p0, r0) = (m.pv_v(0x60), m.pv_v(0x70));
            m.pos = add(p0, scale(sub(fp, p0), e));
            m.rot = std::array::from_fn(|k| add_rot(r0[k], sub_rot(fr[k], r0[k]) * e));
            if if up { 1.0 <= t } else { t <= 0.0 } {
                let h = m.pv_i(0x90);
                if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                m.set_pv_i(0x90, -1);
                if m.o_class != 0x0d49 { ctx.host.play_sound(&w.mobys[id], 1, 0); }
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => if ctx.hero.ground_moby != Some(id) { w.mobys[id].set_state(1, -1) },
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state < 4 {
        let (pos, rot) = (m.pos, m.rot);
        platform::write(m, 0x20, sub(pos, old_pos), old_rot, rot);
    }
}
