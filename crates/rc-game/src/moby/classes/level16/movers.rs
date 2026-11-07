//! Snivelak's movers (L16 addresses; the engine functions named by their L16 addresses with the L00 ones the port
//! knows them by).
//!
//! **0x10fc** / **0x10fd** / **0x10fe** ([`update_10fd`], `0x4199f8`, 4 + 6 + 3 placed; the decomp's
//! `func_L16_004185C0`): a platform that, once the moby `+0x6c` is in state 2, turns and drives along its first row
//! for good. Its pvars: `+0x20` its platform block (crate::moby::platform), `+0x68` ≠ 0: backwards, `+0x6c` the moby
//! it waits on, `+0x70` its looped sound.
//! * **0**: update distance 0xff, the loop −1; 0x10fc without collision, hidden and untargeted (`+0x98` 0, mode `&
//!   0xefff | 0x41`); 1.
//! * **1**: the moby `+0x6c` in state 2: 2.
//! * **2**: forwards its yaw less (`0x2e9298` = L00 `0x2efea8`) and backwards more (`0x2e9250` = L00 `0x2efe60`)
//!   0.26° a tick (0x10fd forwards 0.258°), and a step along its first row (backwards its opposite) of 0.0533 (0x10fe),
//!   1/30 (0x10fd) or 0.0433 (0x10fc). 0x10fc carries the moby `+0x6c` by the same step and, forwards, keeps its class
//!   sound 0 looping (`0x3584d0` = L00 `0x3605f0`).
//!
//! Every tick its block written with the tick's move and turn (`0x31bc70` = L00 `0x322668`, crate::moby::platform::
//! write); Ratchet on it carried by them (crate::moby::carry_hero_with stands in for his read of the block).
//!
//! **0x114c** ([`update_114c`], `0x41dbd8`, 4 placed; the decomp's `func_L16_0041C7A0`): a flipper. Once its timer
//! byte (`+0xbc`) is set it rolls over, waits, bobs, rolls back and is set again. Its pvars: `+0x00` / `+0x04` the
//! bob's last offset and phase, `+0x10` the roll (degrees, by fives), `+0x14` / `+0x18` the wait and the bob's time
//! (seconds), `+0x1c` / `+0x20` / `+0x24` / `+0x28` the waits, the timer byte and the roll kept at the set-up.
//! * **0**: `+0x00..0x14` 0; the waits, its timer byte and its roll (`+0xf0`) kept; 1.
//! * **1**: its timer byte set: 4.
//! * **4**: below 180, its roll 5° more (`0x2e9250` = L00 `0x2efe60`) and `+0x10` 5 more; else 2.
//! * **2**: the wait out (`0x2e7ee8` = L00 `0x2eeb40`, a frame of 1/60 s off): 3.
//! * **3**: the bob's time out: 5; and the bob ([`bob`], `0x41dd80` → `0x31d910` = L00 `0x324430`: 0.1 high at
//!   120 × π/10800 a tick (`$gp+0x2e40` / `+0x2e44`), by the change from the last offset).
//! * **5**: above 0, its roll 5° less (`0x2e9298` = L00 `0x2efea8`) and `+0x10` 5 less; else the kept timer byte,
//!   waits and roll back, 1.
//!
//! **0x1104** ([`update_1104`], `0x419dd8`, 2 placed; the decomp's `func_L16_004189A0`): a lift along a path, out and
//! back each time its timer byte is set. Its pvars: `+0x20` its platform block, `+0x60` the path (crate::level::
//! LevelData::paths), `+0x64` the wait at the far end (ticks), `+0x68` the speed (a second, a tick from the set-up),
//! `+0x6c` the point it is at.
//! * **0**: at the path's first point, at point 0, its speed / 60; 1 (the path −1 prints the game's warning,
//!   `0x2a6170` = L00 `0x2a48f0`, not ported).
//! * **1**: its timer byte set: 2.
//! * **2**: a step of its speed toward the next point (`0x2e8218` / `0x2e84e8` / `0x2e81e8` = L00 `0x2eeea0` /
//!   `0x2ef138` / `0x2eee70`); within 0.88 of a step of it (`0x2e83a8` = L00 `0x2ef018`) the next one on; at the last,
//!   3.
//! * **3**: after the wait (its ticks `+0x96`): 4.
//! * **4**: the same toward the point before, held while Ratchet is under it (lower, within 3 across, `0x2e83e0` =
//!   L00 `0x2ef050`, and 2.5 up or down); at point 0 its timer byte cleared, 1.
//!
//! Then its ticks counted, its block written (`0x31bc70` = L00 `0x322668`) and Ratchet on it carried.

use crate::hero::physics::{add, add_rot, dist, dist2, scale, set_len, sub, sub_rot, V3};
use crate::moby::{Ctx, MobyWorld};

pub const MOVER_A: i16 = 0x10fc;
pub const MOVER_B: i16 = 0x10fd;
pub const MOVER_C: i16 = 0x10fe;
pub const FLIPPER: i16 = 0x114c;
pub const LIFT: i16 = 0x1104;

const BLOCK: usize = 0x20;
const BACK: usize = 0x68;
const WAIT_MOBY: usize = 0x6c;
const LOOP: usize = 0x70;

/// 0x10fc / 0x10fd / 0x10fe (module docs).
pub fn update_10fd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.state = 1;
        m.set_pv_i(LOOP, -1);
        if class == MOVER_A {
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x41;
        }
    }
    let link = usize::try_from(w.mobys[id].pv_i(WAIT_MOBY)).ok().filter(|&k| k < w.mobys.len());
    if w.mobys[id].state == 1 && link.is_some_and(|k| w.mobys[k].state == 2) { w.mobys[id].state = 2; }
    if w.mobys[id].state == 2 {
        let back = w.mobys[id].pv_i(BACK) != 0;
        let (turn, speed) = match class {
            MOVER_C => (0x3b94_b24c, 0x3d5a_740f),
            MOVER_B => (if back { 0x3b94_b24c } else { 0x3b93_8d7b }, 0x3d08_8889),
            _ => (0x3b94_b24c, 0x3d31_7e4b),
        };
        let (turn, speed) = (f32::from_bits(turn), f32::from_bits(speed));
        let m = &mut w.mobys[id];
        m.rot[2] = if back { add_rot(m.rot[2], turn) } else { sub_rot(m.rot[2], turn) };
        let step = scale(m.rows[0], if back { -speed } else { speed });
        m.pos = add(m.pos, step);
        if class == MOVER_A {
            if let Some(k) = link { w.mobys[k].pos = add(w.mobys[k].pos, step); }
            if !back && !ctx.host.sound_playing(w.mobys[id].pv_i(LOOP)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(LOOP, h);
            }
        }
    }
    let m = &mut w.mobys[id];
    let (new, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(new, old), old_rot, rot);
    let flags = m.pv_i(BLOCK + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], new, rot[2]);
}

/// 0x114c (module docs).
pub fn update_114c(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    use crate::moby::classes::props::bob;
    // 5°.
    let step = f32::from_bits(0x3db2_b8c4);
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            for o in (0..0x14).step_by(4) { m.set_pv_i(o, 0); }
            m.set_pv_f(0x28, m.rot[0]);
            m.set_pv_i(0x24, m.timer as i32);
            m.set_pv_f(0x1c, m.pv_f(0x14));
            m.set_pv_f(0x20, m.pv_f(0x18));
            m.set_state(1, -1);
        }
        1 => {
            if m.timer != 0 { m.set_state(4, -1); }
        }
        2 => {
            if m.count_f(0x14) { m.set_state(3, -1); }
        }
        3 => {
            if m.count_f(0x18) { m.set_state(5, -1); }
            bob(m, 0.1, 120.0 * f32::from_bits(0x3998_825c), -1.0, 4, 0);
        }
        4 => {
            if m.pv_i(0x10) < 0xb4 {
                m.rot[0] = add_rot(m.rot[0], step);
                m.set_pv_i(0x10, m.pv_i(0x10) + 5);
            } else {
                m.set_state(2, -1);
            }
        }
        5 => {
            if m.pv_i(0x10) < 1 {
                m.timer = m.pv_i(0x24) as u8;
                m.set_pv_f(0x14, m.pv_f(0x1c));
                m.set_pv_f(0x18, m.pv_f(0x20));
                m.rot[0] = m.pv_f(0x28);
                m.set_state(1, -1);
            } else {
                m.rot[0] = sub_rot(m.rot[0], step);
                m.set_pv_i(0x10, m.pv_i(0x10) - 5);
            }
        }
        _ => {}
    }
}

/// 0x1104 (module docs).
pub fn update_1104(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const PATH: usize = 0x60;
    const WAIT: usize = 0x64;
    const SPEED: usize = 0x68;
    const AT: usize = 0x6c;
    let near = f32::from_bits(0x3f61_47ae);
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let level = ctx.level;
    let points = usize::try_from(w.mobys[id].pv_i(PATH)).ok().and_then(|i| level.paths.get(i));
    let point = |k: i32| -> Option<V3> { points.and_then(|p| usize::try_from(k).ok().and_then(|k| p.get(k))).map(|p| [p[0], p[1], p[2]]) };
    let n = points.map_or(0, |p| p.len() as i32);
    let m = &mut w.mobys[id];
    let step = |m: &mut crate::moby::Moby, to: V3| m.pos = add(m.pos, set_len(sub(to, m.pos), m.pv_f(SPEED)));
    match m.state {
        0 => {
            if let Some(p) = point(0) { m.pos = p; }
            m.set_pv_i(AT, 0);
            m.set_pv_f(SPEED, m.pv_f(SPEED) * f32::from_bits(0x3c88_8889));
            m.set_state(1, -1);
        }
        1 => {
            if m.timer != 0 { m.set_state(2, -1); }
        }
        2 => {
            if let Some(to) = point(m.pv_i(AT) + 1) {
                step(m, to);
                if dist(m.pos, to) <= m.pv_f(SPEED) * near { m.set_pv_i(AT, m.pv_i(AT) + 1); }
            }
            if n - 1 <= m.pv_i(AT) { m.set_state(3, -1); }
        }
        3 => {
            if m.pv_i(WAIT) <= m.state_ticks as u16 as i32 { m.set_state(4, -1); }
        }
        4 => {
            let h = ctx.hero.pos;
            let under = h[2] < m.pos[2] && dist2(m.pos, h) < 3.0 && (m.pos[2] - h[2]).abs() < 2.5;
            if let Some(to) = point(m.pv_i(AT) - 1) {
                if !under { step(m, to); }
                if dist(m.pos, to) <= m.pv_f(SPEED) * near { m.set_pv_i(AT, m.pv_i(AT) - 1); }
            }
            if m.pv_i(AT) <= 0 {
                m.timer = 0;
                m.set_state(1, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    m.state_ticks = m.state_ticks.wrapping_add(1);
    let (new, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(new, old), old_rot, rot);
    let flags = m.pv_i(BLOCK + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], new, rot[2]);
}
