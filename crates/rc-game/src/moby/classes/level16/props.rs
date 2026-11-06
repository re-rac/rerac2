//! Snivelak's props (L16 addresses).
//!
//! * **0x0e73** ([`update_0e73`], `0x425f88`, 13 placed; the decomp's `func_L16_00424B48`): it waits drawn to 0x80.
//!   Once its timer byte (`+0xbc`) is set, it plays class 0xe71's sound 0 and sinks 0.133 a tick for 60 ticks; then
//!   it rests (2).
//! * **0x11b5** ([`update_11b5`], `0x41f7a8`, 23 placed): a hit with the flags 0x4010000 breaks it:
//!   * pieces 0..4 and 6 (mask 0x5f, `0x31dd30`: 1..3 / 3..7, still, no sound);
//!   * its class sound 0;
//!   * the level's bolts (`0x326110` = crate::moby::bolts::level_drop);
//!   * then it goes.
//! * **0x10f0** ([`update_10f0`], `0x418bd0`, 29 placed): a breakable that blinks.
//!   * Its colour switches between 0x7f60800e and 0x7f4c6c01 every two ticks (by the colour's low bit).
//!   * A hit with the flags 0x4010000, while `+0x68`[0] is 0, breaks it. All its pieces fly (`0x323a10` = L00
//!     `0x32b580`; the ranges `$gp+0x2b20` / `+0x2b18` / `+0x2b10`, every piece a shard, sound 0, the level's
//!     bolts). Blasts go off at three points in its scaled frame (size its scale, 12 of each sprite, its flash ball).
//!   * Then it goes (`$gp+0x2b40`, the piece left standing, is −1).
//!
//! Not ported: 0x11b5's ten streaks (L01 `0x2e1680`, from 20 × its scale up along its third row).

use crate::moby::{blast, bolts, shards, Ctx, MobyWorld};

pub const SINKER: i16 = 0x0e73;
pub const POT: i16 = 0x11b5;
pub const BLINKER: i16 = 0x10f0;

const BREAK_FLAGS: u32 = 0x0401_0000;

pub fn update_0e73(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        if m.timer != 0 {
            m.state = 1;
            ctx.host.play_sound_of(&w.mobys[id], 0x0e71, 0, 0);
        }
        w.mobys[id].draw_dist = 0x80;
    }
    let m = &mut w.mobys[id];
    if m.state == 1 {
        m.pos[2] -= 0.133_333_34;
        m.state_ticks = m.state_ticks.wrapping_add(1);
        if 0x3c <= m.state_ticks as u16 { m.state = 2; }
    }
}

fn hit(w: &MobyWorld, id: usize) -> bool { w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) }

pub fn update_11b5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if hit(w, id) { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            shards::break_into(w, id, ctx, (1.0, 3.0), (3.0, 7.0), 0x5f, [0.0; 3], 0xff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            bolts::level_drop(w, id, 0.0, 1, ctx);
            w.delete(id);
        }
        _ => {}
    }
}

/// `$gp+0x2b20` / `+0x2b18` / `+0x2b10`: 0x10f0's pieces' velocity ranges, spin and gravity; `0x1b1b50` its three
/// blasts' points.
const RANGE: [f32; 8] = [-5.0, -5.0, 2.0, 0.0, 5.0, 5.0, 8.0, 0.0];
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (15.0, 20.0);
const BLASTS: [[f32; 3]; 3] = [[0.0, 1.5, 11.0], [0.0, -2.0, 15.0], [2.0, -2.0, 20.0]];

pub fn update_10f0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].mode |= 0x10;
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(4);
            if crate::moby::countdown(&mut t) {
                let c = if m.glow & 1 != 0 { 0x7f60_800e } else { 0x7f4c_6c01 };
                m.glow = c;
                t = 2;
            }
            m.set_pv_i(4, t);
            if hit(w, id) && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            let m = &w.mobys[id];
            let (rows, pos, s) = (m.rows, m.pos, m.scale);
            for p in BLASTS {
                let at = std::array::from_fn(|k| (p[0] * rows[0][k] + p[1] * rows[1][k] + p[2] * rows[2][k]) * s + pos[k]);
                blast::burst(w, ctx, at, &blast::Blast { size: s, bursts: [12, 12, 12], puffs: 12, streaks: 12, light: true, shake: false });
            }
            w.delete(id);
        }
        _ => {}
    }
}
