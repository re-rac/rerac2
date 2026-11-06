//! Level 6's barrel, class **0x08b8** ([`update_08b8`], `0x402800`, 28 placed; the decomp's `func_L06_004012B0`, no C
//! there: ported from the v2 disassembly). Struck, it explodes after a moment, or one time in three flies up smoking
//! first.
//!
//! Its pvars: `+0x00` its timer, `+0x04` its rise's speed, `+0x08` the height it rises to, `+0x0c` −1, `+0x10` its
//! smoke's way.
//!
//! * **0** (set-up): `+0x10` = its place + its place 10 up (`0x32e430`, an add: the code passes its place twice, so
//!   the point is twice its place and 10 up; only its way is read); `+0x0c` −1; then 1.
//! * **1**: a hit with the flags 0x10000 (`0x35df18`, L00 `0x31dce8`) from a source that is not a 0x08b8: one in three
//!   (`0x35b998(3)` = 1, L00 `0x31b9a0`) its timer `0x35b9d8(0x14, 0xa)` (L00 `0x31b9e0`: 20..28) and its height its
//!   z + 20, 3; else its timer `0x35b9d8(0, 10)`, 2. The hit record is let go every tick.
//! * **2**: its timer out (`0x32e0a0`, crate::moby::countdown): it explodes ([`explode`]).
//! * **3** (flying up): its timer out: 2 (the next tick explodes). Else its z eased toward `+0x08` (`0x35efa8` = L00
//!   `0x31ed78`, crate::hero::physics::ease: 1/36, 1/36, at most 1/3, its speed in `+0x04`); its angles jittered by
//!   `0x35b9d8(-10, 10)`°, `(-10, 10)`° and `(-5, 25)`° (`0x32f438` = L00 `0x2efe60`, wrapped); its smoke's way
//!   `+0x10` set to its speed (`0x32e710`); two sparks ([`smoke`]).
//!
//! The engine functions: `0x3a1c98` = L00 `0x3605f0` (its class sound), `0x440a40` = L02 `0x425300`
//! (crate::moby::classes::explosion::start), `0x35d398` = L00 `0x31d180` (crate::moby::ai::attack_sphere),
//! `0x367668` = L00 `0x327498` (crate::moby::shards::break_into), `0x3516d0` = L00 `0x311688` (the delete),
//! `0x35ba30` = L00 `0x31ba38` (crate::rng::Rng::randf), `0x32e460` a subtraction, `0x37a718` = L00 `0x339848`
//! (crate::particles::spark). The camera is L06 `0x1b9840`, its shakes `0x1b9860` (crate::camera::Shakes).
//!
//! Uncertain: the explosion's way is the camera less a stack slot (`sp+0`) that state 2 never writes (state 0 leaves
//! its place there on another tick); the port takes its place, as the other callers of the big explosion do.

use crate::hero::physics::{add, add_rot, ease, set_len, sub, unit, V3};
use crate::moby::{ai, countdown, shards, Ctx, MobyWorld};
use crate::particles::rec;

pub const CLASS: i16 = 0x08b8;

const TIMER: usize = 0x00;
const SPEED: usize = 0x04;
const HEIGHT: usize = 0x08;
const WAY: usize = 0x10;

/// One degree (`0x3c8efa36`).
const DEG: f32 = 0.017_453_294;

pub fn update_08b8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(WAY, add(p, [p[0], p[1], p[2] + 10.0]));
            m.set_pv_i(0x0c, -1);
            m.state = 1;
        }
        1 => {
            let h = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
            if let Some(h) = h {
                let from_barrel = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == CLASS);
                if !from_barrel {
                    if ctx.game.rng.randi(3) == 1 {
                        let t = ctx.game.rng.rand_range(0x14, 0xa);
                        let m = &mut w.mobys[id];
                        m.set_pv_i(TIMER, t);
                        let z = m.pos[2] + 20.0;
                        m.set_pv_f(HEIGHT, z);
                        m.state = 3;
                    } else {
                        let t = ctx.game.rng.rand_range(0, 10);
                        let m = &mut w.mobys[id];
                        m.set_pv_i(TIMER, t);
                        m.state = 2;
                    }
                }
            }
            w.clear_hit(id);
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(TIMER);
            let done = countdown(&mut t);
            m.set_pv_i(TIMER, t);
            if done { explode(w, id, ctx); }
        }
        3 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(TIMER);
            let done = countdown(&mut t);
            m.set_pv_i(TIMER, t);
            if done {
                m.state = 2;
                return;
            }
            let (target, mut v) = (m.pv_f(HEIGHT), m.pv_f(SPEED));
            let k = f32::from_bits(0x3ce3_8e39);
            ease(target, k, k, f32::from_bits(0x3eaa_aaab), &mut m.pos[2], &mut v);
            m.set_pv_f(SPEED, v);
            for (axis, (lo, hi)) in [(-10, 10), (-10, 10), (-5, 25)].into_iter().enumerate() {
                let d = ctx.game.rng.rand_range(lo, hi) as f32 * DEG;
                let m = &mut w.mobys[id];
                m.rot[axis] = add_rot(m.rot[axis], d);
            }
            let m = &mut w.mobys[id];
            let way = set_len(m.pv_v(WAY), m.pv_f(SPEED));
            m.set_pv_v(WAY, way);
            smoke(w, id, ctx);
        }
        _ => {}
    }
}

/// State 3's smoke: a random way (`0x35ba30(-1, 1)` each axis) at `0.1..0.2 / 60` a tick, plus `1/600..1/60` back
/// along `+0x10`; a spark (`0x37a718`: size 79872, growing 39936, drag 1, falling −0.0002, 20 ticks, alpha 0x3c, grey
/// 0x606060, spin 3) at a random share of `+0x10` from its place, made soft (ALPHA 0x44) with type 0x17's frame
/// (`0x1ff41c`), and one time in two (`0x35b9d8(0, 1)`) additive again (0x48) in 0x707070 at alpha 0x7f; a second
/// (falling −0.0004, 7 ticks, alpha 0x7f, 0xb0b0b0) at its place on the same way.
fn smoke(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (pos, way, speed) = (m.pos, m.pv_v(WAY), m.pv_f(SPEED));
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let sixtieth = f32::from_bits(0x3c88_8889);
    let r: V3 = std::array::from_fn(|_| rng.randf(-1.0, 1.0));
    let vel = set_len(r, rng.randf(0.1, 0.2) * sixtieth);
    let back = set_len(way, -rng.randf(f32::from_bits(0x3ada_740f), sixtieth));
    let vel = add(vel, back);
    let at = add(set_len(way, rng.randf(0.0, 1.0) * speed), pos);
    let (size, grow) = (f32::from_bits(0x479c_4000), f32::from_bits(0x471c_4000));
    if let Some(i) = crate::particles::spark(sys, rng, size, grow, 1.0, f32::from_bits(0xb951_b717), 0.0, at, vel, 0x14, 0x3c, 0x60_6060, 3) {
        let frame = sys.def_first(0x17);
        let r = &mut sys.pool.recs[i];
        r[3] = 0x44;
        r[2] = frame;
        if rng.rand_range(0, 1) != 0 {
            let r = &mut sys.pool.recs[i];
            r[3] = 0x48;
            rec::set_u32(r, 4, 0x0070_707f);
            rec::set_i16(r, 0x3e, 0x7f);
            r[0x31] = 0x70;
            r[0x32] = 0x70;
            r[0x33] = 0x70;
        }
    }
    crate::particles::spark(sys, rng, size, grow, 1.0, f32::from_bits(0xb9d1_b717), 0.0, pos, vel, 7, 0x7f, 0xb0_b0b0, 3);
}

/// State 2's explosion: its class sound 0; the big explosion at its place (ground 5) facing the camera; the shakes
/// (`0x1b9860` 0.2 for 60 ticks, its length cleared; the roll `0x1b9880` 0.1 for 90); a hit sphere (`0x35d398`: radius 3,
/// damage 3, push 1, flags 0x10000, 0, 1, 0); its pieces of the mask 0xef (`0x367668`: 3..6 out, 3..6 up, (0, 0,
/// 0.05) along, no sound); deleted.
fn explode(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let pos = w.mobys[id].pos;
    let dir = unit(sub(ctx.host.camera_pos(), pos));
    crate::moby::classes::explosion::start(w, ctx, 5.0, pos, dir);
    let s = &mut ctx.game.shakes;
    s.ask(0, 0.2, 60);
    s.0[2].amp = 0.1;
    s.0[2].timer = 90;
    ai::attack_sphere(w, id, ctx, 3.0, 3.0, 1.0, pos, 0x1_0000, 0, 1, 0);
    shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xef, [0.0, 0.0, f32::from_bits(0x3d4c_ccce)], 0xff);
    w.delete(id);
}
