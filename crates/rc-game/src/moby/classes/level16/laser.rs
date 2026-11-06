//! Level 16's sweeping laser **0x1162** (`0x41df68`, 5 placed; the decomp's `func_L16_0041CB30`, its draw
//! `func_L16_0041CF80` = `0x41e3b8`): an emitter sliding to and fro, its beam reaching 25 ahead along its yaw. Its
//! pvars: `+0x00` its first place, `+0x10` its second, `+0x20` its speed, `+0x24` its hum, `+0x28` its hit sound.
//!
//! * **0**: up 0.57; its first place where it is, its second 9.5 off to its left (its yaw + π/2, `0x2fbb78` = L00
//!   `0x2efe60`); no sounds; 1.
//! * **1** / **2**: its hum (the sound 0 of class 0x113a's table, looped: `0x358578` = L00 `0x360698(0, 4)`) kept
//!   playing; it springs (`0x3184d0` = L00 `0x31ec88`, crate::moby::ai::spring_to: 1/900, 1/450, 1/15) toward its
//!   second place (1) or its first (2), within 1/30 of it the other state.
//! * **Every tick**: its draw ([`draw`]) on the first list (`0x2e0968` = L00 `0x2e7718`); a hit of 4 (`0x3169a0` =
//!   L00 `0x31d158`, flags 1, its class in the kind word) along its beam to the first moby met (`0x2d7b50` = L00
//!   `0x2de878`, flags 0, itself skipped) — that being Ratchet's, the sound 1 of 0x113a's table once (unless it
//!   still plays); the Spiderbot out (`0x1b0614`, else the upgraded one `0x1b0f40`) within 0.15 of the beam (the
//!   closest point, `0x316728`) and not blowing up already (5): it blows up (`0x3d0c08` / `0x3f4ff8` = L00
//!   `0x3e0b28` / `0x403d98` with (1, 2): crate::moby::classes::spiderbot::end).

use crate::fx::FxQuad;
use crate::hero::physics::{add, cross, dist, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1162;
/// The class whose sound table its sounds are from.
const SOUNDS: i16 = 0x113a;

const FIRST: usize = 0x00;
const SECOND: usize = 0x10;
const SPEED: usize = 0x20;
const HUM: usize = 0x24;
const HIT_SOUND: usize = 0x28;
/// The beam's reach.
const REACH: f32 = 25.0;

/// Its beam's end.
fn beam_end(m: &crate::moby::Moby) -> V3 {
    let yaw = m.rot[2];
    add(m.pos, [yaw.cos() * REACH, yaw.sin() * REACH, 0.0])
}

/// `0x41df68` (module docs).
pub fn update_1162(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.pos[2] += 0.57;
            m.set_pv_v(FIRST, m.pos);
            let a = crate::hero::physics::add_rot(m.rot[2], std::f32::consts::FRAC_PI_2);
            m.set_pv_v(SECOND, add(m.pos, [a.cos() * 9.5, a.sin() * 9.5, 0.0]));
            m.set_pv_i(HUM, -1);
            m.set_pv_i(HIT_SOUND, -1);
            m.set_state(1, -1);
        }
        s @ (1 | 2) => {
            let h = m.pv_i(HUM);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 0, 4);
                w.mobys[id].set_pv_i(HUM, h);
            }
            let m = &mut w.mobys[id];
            let goal = m.pv_v(if s == 1 { SECOND } else { FIRST });
            let mut v = m.pv_f(SPEED);
            crate::moby::ai::spring_to(&mut m.pos, goal, 0.001_111_111_1, 0.002_222_222_3, 0.066_666_67, &mut v);
            m.set_pv_f(SPEED, v);
            if dist(m.pos, goal) < 0.033_333_335 { m.set_state(3 - s, -1); }
        }
        _ => {}
    }
    w.register_draw_first(id);
    let m = &w.mobys[id];
    let (from, to) = (m.pos, beam_end(m));
    let mut t = crate::moby::hits::HitTemplate::new(4.0, Some(id), 1, [0.0; 4]);
    t.kind = (m.o_class as u16 as u32) << 16;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id)));
    if let Some(o) = hit {
        if let Some(target) = o.moby {
            w.deliver_hit_any(target, &t, [o.point[0], o.point[1], o.point[2], 1.0]);
            if Some(target) == ctx.hero_moby {
                let h = w.mobys[id].pv_i(HIT_SOUND);
                if !(h != -1 && ctx.host.sound_playing(h)) {
                    let h = ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 1, 0);
                    w.mobys[id].set_pv_i(HIT_SOUND, h);
                }
            }
        }
    }
    use crate::moby::classes::spiderbot::{BOT_CLASS, PLUS_BOT};
    let bot = w.mobys.iter().position(|b| b.o_class == BOT_CLASS && b.state < 0x80).or_else(|| w.mobys.iter().position(|b| b.o_class == PLUS_BOT && b.state < 0x80));
    if let Some(b) = bot {
        let p = w.mobys[b].pos;
        let ab = sub(to, from);
        let l2 = crate::hero::physics::dot(ab, ab);
        let k = if l2 == 0.0 { 0.0 } else { (crate::hero::physics::dot(sub(p, from), ab) / l2).clamp(0.0, 1.0) };
        let near = add(from, crate::hero::physics::scale(ab, k));
        if dist(near, p) < 0.15 && w.mobys[b].state != 5 { crate::moby::classes::spiderbot::end(w, true, 2); }
    }
}

/// `0x41e3b8` (a first-list callback): its beam from it to 25 ahead as two quads facing the camera (`0x1b9a80`) —
/// each end's side `(end − camera) × the way` 0.15 long, then 0.05 — of FX 14 additive (ALPHA 0x48): the wide one
/// in 0x80243278, the narrow one white (0x80808080), s across and t along.
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    beam(m.pos, beam_end(m), cam, out);
}

/// The beam's two quads from `from` to `to` ([`draw`]'s; also Grelbin's laser 0x11f0's draw, L19 `0x421e78`, the
/// same code: crate::moby::classes::level19::props::draw_11f0).
pub fn beam(from: V3, to: V3, cam: V3, out: &mut Vec<FxQuad>) {
    let way = sub(to, from);
    for (half, rgba) in [(0.15, 0x8024_3278u32), (0.05, 0x8080_8080)] {
        let a = set_len(cross(sub(from, cam), way), half);
        let b = set_len(cross(sub(to, cam), way), half);
        out.push(FxQuad { fx: 14, additive: true, corners: [sub(from, a), add(from, a), sub(to, b), add(to, b)], st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [rgba; 4] });
    }
}
