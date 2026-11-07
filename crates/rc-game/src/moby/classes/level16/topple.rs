//! Snivelak's toppling post **0x0e7f** ([`update_0e7f`], `0x3f64a8`, one placed; the decomp's `func_L16_003F4FA8`):
//! it stands with a Swingshot pull target 0x099d at its top (crate::moby::classes::endako_b::update_099d); pulled, it
//! falls flat and bounces to rest. Its pvars: `+0x00` the target, `+0x04` its fall's speed. Its sounds: 0 the pull, 1
//! each landing (`0x3584d0` = L00 `0x3605f0`).
//!
//! * With no pvars: a 0x0e7f made in its place (`0x30aae8` = L00 `0x311320`, `0x322bd8` = L00 `0x32a748`) and itself
//!   deleted (`0x30ae50`).
//! * **0**: its pitch its yaw less 90° (`0x2e9298` = L00 `0x2efea8`); the target made at its place, at twice its
//!   class's scale; 1.
//! * **1**: the target hooked (`0x322cc8` = L00 `0x32a838`): sound 0, 2.
//! * **2**: its pitch eased down to 0 (`0x3185c0` = L00 `0x31ed78`: 0.00053 a tick², a brake of 174.5, at most 290.9),
//!   sound 0 on its first tick; at 0: sound 1, 7.
//! * **7** / **9**: up to 5° / 2° (1° a tick², a brake of 0.025°, at most 50°), sound 1 on the first tick; then
//!   **8** / **10** back down to 0 (0.025° a tick², a brake of 10°, at most 166.7°), sound 1 and the next (8 → 9; 10
//!   rests there). **3..6** (up to 25° and 15°) are never reached from 2.
//!
//! Every tick the target sits at (0, 8, −0.5) in its frame (`0x2e86c8` / `0x2e81e8` = L00 `0x2ef318` / `0x2eee70`).

use crate::hero::physics::{add, ease, sub_rot};
use crate::moby::director::link;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e7f;
/// The Swingshot's pull target (crate::moby::classes::endako_b::update_099d).
pub const TARGET: i16 = 0x099d;

const CHILD: usize = 0x00;
const VEL: usize = 0x04;

/// `0x322bd8(m, owner)` (L00 `0x32a748`): updated and drawn from any distance, drawn, the owner, its place, angles,
/// light word and ambient; mode & 0xfff8.
fn adopt(w: &mut MobyWorld, id: usize, owner: usize) {
    let (pos, rot, light, ambient) = { let o = &w.mobys[owner]; (o.pos, o.rot, o.light, o.ambient) };
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.parent = Some(owner);
    (m.pos, m.rot, m.light, m.ambient) = (pos, rot, light, ambient);
    m.mode &= 0xfff8;
}

/// The fall and the bounces: `0x31ed78` on the pitch toward `to`, its speed `+0x04`; true once it has reached it
/// (rising: at or past it; falling: at or below 0), the speed then 0.
fn swing(m: &mut crate::moby::Moby, to: f32) -> bool {
    let (mut x, mut v) = (m.rot[0], m.pv_f(VEL));
    let up = to != 0.0;
    if up {
        ease(to, f32::from_bits(0x3c8e_fa36), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3f5f_66f5), &mut x, &mut v);
    } else {
        ease(0.0, f32::from_bits(0x39e4_c38a), f32::from_bits(0x3e32_b8c4), f32::from_bits(0x403a_2b22), &mut x, &mut v);
    }
    m.rot[0] = x;
    m.set_pv_f(VEL, v);
    let done = if up { to <= x } else { x <= 0.0 };
    if done { m.set_pv_f(VEL, 0.0); }
    done
}

/// 0x0e7f (module docs).
pub fn update_0e7f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() {
        let k = w.create(CLASS);
        adopt(w, k, id);
        w.delete(id);
        return;
    }
    let s = w.mobys[id].state;
    match s {
        0 => {
            let m = &mut w.mobys[id];
            m.rot[0] = sub_rot(m.rot[2], f32::from_bits(0x3fc9_0fdc));
            let k = w.create(TARGET);
            w.mobys[id].set_pv_i(CHILD, k as i32);
            adopt(w, k, id);
            let scale = w.class(TARGET).map_or(1.0, |c| c.scale);
            w.mobys[k].scale = scale + scale;
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if link(w, w.mobys[id].pv_i(CHILD)).is_some_and(|k| w.mobys[k].once & 4 != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let m = &mut w.mobys[id];
            let mut x = m.rot[0];
            let mut v = m.pv_f(VEL);
            ease(0.0, f32::from_bits(0x3a0b_ccd4), f32::from_bits(0x432e_886f), f32::from_bits(0x4391_71b3), &mut x, &mut v);
            m.rot[0] = x;
            m.set_pv_f(VEL, v);
            if x <= 0.0 {
                m.set_pv_f(VEL, 0.0);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                w.mobys[id].set_state(7, -1);
            }
        }
        3 | 5 | 7 | 9 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            // 25°, 15°, 5°, 2°.
            let to = f32::from_bits([0x3edf_66f4, 0x3e86_0a93, 0x3db2_b8c4, 0x3d0e_fa36][(s as usize - 3) / 2]);
            if swing(&mut w.mobys[id], to) { w.mobys[id].set_state(s + 1, -1); }
        }
        4 | 6 | 8 => {
            if swing(&mut w.mobys[id], 0.0) {
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                // 6 goes back to 7, as 2 does.
                w.mobys[id].set_state(if s == 6 { 7 } else { s + 1 }, -1);
            }
        }
        10 => {
            swing(&mut w.mobys[id], 0.0);
        }
        _ => {}
    }
    let m = &w.mobys[id];
    let r = m.rows;
    let at = add(m.pos, std::array::from_fn(|k| 8.0 * r[1][k] - 0.5 * r[2][k]));
    if let Some(k) = link(w, m.pv_i(CHILD)) { w.mobys[k].pos = at; }
}
