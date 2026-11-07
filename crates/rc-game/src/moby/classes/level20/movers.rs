//! Yeedil's movers (L20 addresses; the engine functions by their L20 places with the L00 ones the port knows them
//! by). Each carries Ratchet through its platform block `+0x20` (crate::moby::platform, its flags `+0x5c`; the move
//! `0x3378b8` = L00 `0x322668`, crate::moby::carry_hero_with) at the end of its tick.
//!
//! * **0x1170** ([`update_1170`], `0x43ea70`, 3 placed; the decomp's `func_L20_0043D4D8`): a turntable that makes
//!   one turn when its timer byte (`+0xbc`) changes. Its pvars: `+0x60` the camera's facing (`(0, ±1, 0)`), `+0x70`
//!   the turn (2π), `+0x74` its speed (placed in degrees a tick, kept in radians), `+0x78` the turn's ticks, `+0x7c`
//!   its timer byte at the start, `+0x80` / `+0x84` its camera tweak held and its slot, `+0x88` its projection, `+0x8c`
//!   the projection's phase, `+0x90` its loop sound.
//!   * **0**: the flag 1, the turn 2π, the speed in radians, the facing (0, 1, 0), its timer byte kept; 1.
//!   * **1**: its timer byte changed: its projection made when it has none (its own class, `0x326280` = L00
//!     `0x311320`; `0x33fa28(child, 0)` = L00 `0x32a748`: updated and drawn from any distance, drawn, mode & 0xfff8;
//!     then mode | 2, no collision, alpha 0); the facing's y 1 with Ratchet at or past its y, else −1; 2.
//!   * **2**: on its first tick the turn's ticks `2π / speed` (truncated); while they run (`0x302c30` = L00
//!     `0x2eeab0`, crate::moby::countdown) its yaw turns by the speed (`0x303fd0` = L00 `0x2efe60`) and its class
//!     sound 1 loops (`0x3766c0` = L00 `0x3605f0`); out, its timer byte back to the kept one, 1, the sound stopped
//!     (`0x3763c0` = L00 `0x3602f0`).
//!   * **Each tick**: Ratchet on it (`0x339a48` = L00 `0x325398`, crate::moby::ai::stands_on)
//!     asks once for a camera tweak of priority 5 (`0x2eb400` = L00 `0x2d7468`; `0x43ea08`: distance 8, pivot height
//!     2.5, the firm springs, the stick left on, facing the facing once, not again after a rest); off it, the tweak
//!     let go (`0x2eb4a0` = L00 `0x2d7508`). Its projection, while it has one, looks as Oozla's (`0x3413a8` = L01
//!     `0x321ba0`, crate::moby::classes::oozla::hologram: on in its state 2); faded out it is deleted, else it takes
//!     its angles and stands a hundredth of the way from it to the camera (`0x1b9fc0`), its matrix rebuilt
//!     (`0x328d10` = L00 `0x313dc0`).
//! * **0x1306** ([`update_1306`], `0x39ff80`, 2 placed; the decomp's `func_L20_0039EBB8`): a platform that rises and
//!   drops back along y. Its pvars: `+0x60` its place at the start, `+0x70` / `+0x74` the high and the low offsets,
//!   `+0x78` the period (ticks), `+0x7c` its countdown, `+0x80` the phase's offset. **0**: on its first tick its place
//!   kept; 1. **1**: on its first tick the countdown its period; at its end 1 again (a new period). Its place the kept
//!   one, its y `low + (high − low)·phase` past it, the phase `countdown / period + offset` less whole turns.
//! * **0x1243** ([`update_1243`], `0x4484f0`, one placed; the decomp's `func_L20_00446FA8`): a gate that slides
//!   along y between its start and `+0x60`. Its pvars: `+0x64` its start, `+0x68` its speed, `+0x6c` its rate
//!   (0.0011666667), `+0x70` the middle, `+0x74` at the far end, `+0x78` it starts there, `+0x7c` its loop sound.
//!   * **0**: the flag 1, its rate, its start, the middle; at the far end by `+0x78`; 1.
//!   * **1**: hooked (`0x33fb18` = L00 `0x32a838`), with Ratchet on it or across the middle from it: to the far end
//!     (2) or back (3); its class sound 0 looped when not playing (`0x376378` = L00 `0x3602a8`).
//!   * **2** / **3**: on the first tick still; within 0.03 of its end the sound stopped and, Ratchet neither on it
//!     nor over it (`0x448870` = L14 `0x3dfde0`, crate::moby::classes::level14::movers::over), at the end, 1; else
//!     eased to its end (`0x333eb8` = L00 `0x31ed78`, crate::camera::approach: its rate, at most 0.2 a tick).
//! * **0x1283** ([`update_1283`], `0x44a880`, one placed; the decomp's `func_L20_00449338`): a lift that rises
//!   `+0x80` once. Its pvars: `+0x60` / `+0x70` its place and angles at the start, `+0x84` the rise's ticks, `+0x88` an
//!   area (−1 none), `+0x8c` it follows its mission, `+0x90` the rise's progress. Its trigger: with `+0x8c` its
//!   mission (`+0xb0`) done, or Ratchet in the area (`0x33fb28` = L00 `0x32a848`, crate::level::LevelData::in_area).
//!   * **0**: the progress 0, its place kept; triggered already it stands risen (mode | 2: no more updates), 3; else
//!     1.
//!   * **1**: its timer byte, hooked or triggered: 2.
//!   * **2**: the progress up by `1 / +0x84` (at most 1), its height its start's plus the rise by it; at 1, 3.
//!   * Under 3 its move into its platform block.

use super::misc::{mission_done, ride};
use crate::hero::physics::{add, add_rot, scale, sub};
use crate::moby::{ai, countdown, keep_loop, stop_loop, Ctx, MobyWorld};

/// `0x2eb400(5)` (L00 `0x2d7468`) then `0x43ea08`: a camera tweak facing `dir` once.
fn ask_tweak(ctx: &mut Ctx, dir: [f32; 3]) -> i32 {
    let Some(s) = ctx.game.cam_tweaks.request(5) else { return -1 };
    let t = &mut ctx.game.cam_tweaks.slots[s];
    (t.dist, t.pivot_h) = (8.0, 2.5);
    t.firm = true;
    t.no_stick = false;
    t.face_once = true;
    t.face_rest = false;
    t.dir = dir;
    s as i32
}

pub const TURNTABLE: i16 = 0x1170;

const PROJECTION: usize = 0x88;

/// 0x1170 (module docs).
pub fn update_1170(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let speed = m.pv_f(0x74) * f32::from_bits(0x3c8e_fa36);
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            m.set_pv_f(0x70, f32::from_bits(0x40c9_0fdb));
            m.set_pv_f(0x74, speed);
            let t = m.timer as i32;
            m.set_pv_i(0x60, 0);
            m.set_pv_i(0x68, 0);
            m.set_pv_i(0x80, 0);
            m.set_pv_i(0x7c, t);
            m.set_pv_f(0x6c, 1.0);
            m.set_pv_f(0x64, 1.0);
            m.set_state(1, -1);
        }
        1 => {
            if w.mobys[id].timer as i32 != w.mobys[id].pv_i(0x7c) {
                if ai::link(&w.mobys[id], PROJECTION).filter(|&c| c < w.mobys.len()).is_none() {
                    let c = w.create(w.mobys[id].o_class);
                    let cm = &mut w.mobys[c];
                    cm.update_dist = 0xff;
                    cm.draw_dist = 0xff;
                    cm.drawn = 1;
                    cm.mode = (cm.mode & 0xfff8) | crate::moby::mode::NO_UPDATE;
                    cm.has_collision = false;
                    cm.alpha = 0;
                    ai::set_link(&mut w.mobys[id], PROJECTION, Some(c));
                }
                let m = &mut w.mobys[id];
                let side = if m.pos[1] <= ctx.hero.pos[1] { 1.0 } else { -1.0 };
                m.set_pv_f(0x64, side);
                m.set_state(2, -1);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            if m.first() {
                let t = (m.pv_f(0x70) / m.pv_f(0x74)) as i32;
                m.set_pv_i(0x78, t);
            }
            let mut t = m.pv_i(0x78);
            let out = countdown(&mut t);
            m.set_pv_i(0x78, t);
            if !out {
                m.rot[2] = add_rot(m.rot[2], m.pv_f(0x74));
                keep_loop(w, id, ctx, 0x90, 1);
            } else {
                let m = &mut w.mobys[id];
                m.timer = m.pv_i(0x7c) as u8;
                m.set_state(1, -1);
                stop_loop(w, id, ctx, 0x90);
            }
        }
        _ => {}
    }
    let on = crate::moby::ai::stands_on(ctx, id);
    let held = w.mobys[id].pv_i(0x80) != 0;
    if on && !held {
        let dir = w.mobys[id].pv_v(0x60);
        let s = ask_tweak(ctx, dir);
        w.mobys[id].set_pv_i(0x84, s);
        w.mobys[id].set_pv_i(0x80, 1);
    } else if !on && held {
        let s = w.mobys[id].pv_i(0x84);
        if let Ok(s) = usize::try_from(s) { ctx.game.cam_tweaks.release(s); }
        w.mobys[id].set_pv_i(0x80, 0);
    }
    ride(w, id, ctx, 0x20, old, old_rot);
    let Some(c) = ai::link(&w.mobys[id], PROJECTION).filter(|&c| c < w.mobys.len()) else { return };
    let turning = w.mobys[id].state == 2;
    // The phase lives in the table's `+0x8c`; the port lends it to the projection's first pvar for the look.
    let phase = w.mobys[id].pv_f(0x8c);
    w.mobys[c].set_pv_f(0, phase);
    crate::moby::classes::oozla::hologram(&mut w.mobys[c], 0, turning, &mut ctx.game.rng);
    let phase = w.mobys[c].pv_f(0);
    w.mobys[id].set_pv_f(0x8c, phase);
    if w.mobys[c].alpha == 0 {
        w.delete(c);
        ai::set_link(&mut w.mobys[id], PROJECTION, None);
        return;
    }
    let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let off = scale(sub(ctx.host.camera_pos(), pos), f32::from_bits(0x3c23_d70a));
    let cm = &mut w.mobys[c];
    cm.rot = rot;
    cm.pos = add(off, pos);
    w.rebuild(c, ctx.host);
}

pub const BOBBER: i16 = 0x1306;

/// 0x1306 (module docs).
pub fn update_1306(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            if m.first() { let p = m.pos; m.set_pv_v(0x60, p); }
            m.set_state(1, -1);
        }
        1 => {
            if m.first() { m.set_pv_i(0x7c, m.pv_i(0x78)); }
            let mut t = m.pv_i(0x7c);
            if countdown(&mut t) { m.set_state(1, -1); }
            m.set_pv_i(0x7c, t);
            m.pos = m.pv_v(0x60);
            let mut phase = m.pv_i(0x7c) as f32 / m.pv_i(0x78) as f32 + m.pv_f(0x80);
            let (high, low, y) = (m.pv_f(0x70), m.pv_f(0x74), m.pos[1]);
            while 1.0 < phase { phase -= 1.0; }
            m.pos[1] = y + (low + (high - low) * phase);
        }
        _ => {}
    }
    ride(w, id, ctx, 0x20, old, old_rot);
}

pub const GATE: i16 = 0x1243;

/// 0x1243 (module docs).
pub fn update_1243(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let state = w.mobys[id].state;
    match state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            let (to, start) = (m.pv_f(0x60), m.pos[1]);
            m.set_pv_f(0x6c, f32::from_bits(0x3a98_ead7));
            m.set_pv_f(0x64, start);
            m.set_pv_f(0x70, (start + to) * 0.5);
            let far = m.pv_i(0x78) != 0;
            m.set_pv_i(0x74, far as i32);
            m.set_state(1, -1);
        }
        1 => {
            let m = &w.mobys[id];
            if m.once & 4 != 0 {
                let mid = m.pv_f(0x70);
                let across = (m.pos[1] - mid) * (ctx.hero.pos[1] - mid) < 0.0;
                if crate::moby::ai::stands_on(ctx, id) || across {
                    let to = if w.mobys[id].pv_i(0x74) == 0 { 2 } else { 3 };
                    w.mobys[id].set_state(to, -1);
                    keep_loop(w, id, ctx, 0x7c, 0);
                }
            }
        }
        2 | 3 => {
            let (end, at_far) = if state == 2 { (0x60, 1) } else { (0x64, 0) };
            if w.mobys[id].first() { w.mobys[id].set_pv_i(0x68, 0); }
            let to = w.mobys[id].pv_f(end);
            if (to - w.mobys[id].pos[1]).abs() < f32::from_bits(0x3cf5_c28f) {
                stop_loop(w, id, ctx, 0x7c);
                if !crate::moby::classes::level14::movers::over(id, ctx) {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x74, at_far);
                    m.pos[1] = to;
                    m.set_state(1, -1);
                }
            } else {
                let m = &mut w.mobys[id];
                let a = m.pv_f(0x6c);
                let (mut y, mut v) = (m.pos[1], m.pv_f(0x68));
                crate::camera::approach(to, a, a, f32::from_bits(0x3e4c_ccce), &mut y, &mut v);
                m.pos[1] = y;
                m.set_pv_f(0x68, v);
            }
        }
        _ => {}
    }
    ride(w, id, ctx, 0x20, old, old_rot);
}

pub const RISER: i16 = 0x1283;

/// 0x1283's trigger (module docs).
fn triggered(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let area = m.pv_i(0x88);
    (m.mission != 0xff && m.pv_i(0x8c) != 0 && mission_done(ctx, m.mission)) || (area != -1 && ctx.level.in_area(area, ctx.hero.pos))
}

/// 0x1283 (module docs).
pub fn update_1283(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => {
            let on = triggered(w, id, ctx);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x90, 0.0);
            let (p, r) = (m.pos, m.rot);
            m.set_pv_v(0x60, p);
            m.set_pv_v(0x70, r);
            if on {
                m.mode |= crate::moby::mode::NO_UPDATE;
                m.pos[2] = m.pv_f(0x68) + m.pv_f(0x80);
                m.set_state(3, -1);
            } else {
                m.set_state(1, -1);
            }
        }
        1 => {
            let m = &w.mobys[id];
            if m.timer != 0 || m.once & 4 != 0 || triggered(w, id, ctx) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            let m = &mut w.mobys[id];
            let p = (m.pv_f(0x90) + 1.0 / m.pv_f(0x84)).min(1.0);
            m.set_pv_f(0x90, p);
            m.pos[2] = m.pv_f(0x68) + p * m.pv_f(0x80);
            if 1.0 <= p { m.set_state(3, -1); }
        }
        _ => {}
    }
    if w.mobys[id].state < 3 { ride(w, id, ctx, 0x20, old, old_rot); }
}
