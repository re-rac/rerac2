//! The sliding hatch **0x1151** (L18 `0x3fc778`, 17 placed; the decomp's `func_L18_003FB308`): one half of a pair
//! that slides apart. It moves along its first row and its other half (the moby `+0x68`[0x24], an index) the other
//! way, both a step a tick.
//!
//! **Its pvars:** `+0x00` its place at the set-up, `+0x10` the step (its first row × `$gp+0x1544` / `$gp+0x1540`:
//! 2 / 20), `+0x20` the steps out, `+0x24` the other half, `+0x28` dust on opening.
//!
//! * **0**: the place and the step; the steps out 0; then 1.
//! * **1** (closing): two or more steps out, it moves a step back and the other half a step forward; one, both are
//!   put on its place. A step fewer either way.
//! * **2** (open, set by its opener): at the start its sound 0 of class 0x1152's table (`0x37c980` = L00
//!   `0x360698`); under 20 steps out, it moves a step out and the other half a step back; with `+0x28` the dust
//!   ([`update_1151`]'s note); after 240 ticks (`$gp+0x1548`) → 1.
//!
//! It counts its ticks in the state (`+0x96`) itself, to 0xffff.
//!
//! Its opener ([`open`], `0x3fc970`, the decomp's `func_L18_003FB500`, L19 `0x3e0eb8`, L20 `0x400af8`) and the place
//! inside it ([`entry`], `0x3fca58`, L19 `0x3e0fa0`, L20 `0x400be0`) serve the prey 0x0ef4's state 7
//! (crate::moby::classes::level18::prey).
//!
//! Not ported: the opener's other caller, the director's streamed spawns (`0x3fbd90`, the only one with dust). The
//! dust (`0x3fca90`: 1..2 puffs `0x35a848` = L02 `0x345d88`, the particle burst the port lacks, on the template
//! `$gp+0x1570`, jittered 0.5, 8..12 a second back along its first row).

use crate::hero::physics::{add, scale, sub};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1151;
/// The class whose sound table it plays from.
const SOUNDS: i16 = 0x1152;

const HOME: usize = 0x00;
const STEP: usize = 0x10;
const OUT: usize = 0x20;
const OTHER: usize = 0x24;
/// `$gp+0x1540` / `+0x1544` / `+0x1548`: its steps, the way out and the ticks it stays open.
const STEPS: i32 = 20;
const TRAVEL: f32 = 2.0;
const OPEN_TICKS: u16 = 240;

/// Moves the other half by `d`, or to `at`.
fn move_other(w: &mut MobyWorld, id: usize, d: Option<[f32; 3]>, at: [f32; 3]) {
    let Some(k) = usize::try_from(w.mobys[id].pv_i(OTHER)).ok().filter(|&k| k < w.mobys.len()) else { return };
    let o = &mut w.mobys[k];
    o.pos = match d {
        Some(d) => add(o.pos, d),
        None => at,
    };
}

/// `0x3fca58(hatch, out)`: the place inside it — its place at the set-up, 1 lower.
pub fn entry(w: &MobyWorld, id: usize) -> [f32; 3] {
    let p = w.mobys[id].pv_v(HOME);
    [p[0], p[1], p[2] - 1.0]
}

/// `0x3fc970(hatch, dust)`: opened — unless open already (state 2: true), state 2 with `+0x28` the dust; with dust an
/// explosion at it (crate::moby::blast::explode: 5 embers, 3 billows, 8 flares, flashes 5 / 4.9, near 32, size 2,
/// shaking, one piece of debris, flags 0x810001) and class 0x1152's sound 1 (`0x37c980` = L00 `0x360698`).
pub fn open(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, dust: bool) -> bool {
    if w.mobys[id].state == 2 { return true; }
    let m = &mut w.mobys[id];
    m.set_state(2, -1);
    m.set_pv_i(0x28, dust as i32);
    if dust {
        let e = crate::moby::blast::Explosion {
            embers: 5, billows: 3, flares: 8, flash: 5.0, flash2: 4.9, near: 32.0, size: 2.0, sound: -1, shake: true, debris: 1, lod: -1, flags: 0x81_0001,
            ..Default::default()
        };
        crate::moby::blast::explode(w, ctx, Some(id), None, &e);
        ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 1, 0);
    }
    w.changed = true;
    false
}

/// 0x1151 (module docs).
pub fn update_1151(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(HOME, p);
            let step = scale(m.rows[0], TRAVEL / STEPS as f32);
            m.set_pv_v(STEP, step);
            m.set_state(1, -1);
            m.set_pv_i(OUT, 0);
        }
        1 => {
            let m = &mut w.mobys[id];
            let n = m.pv_i(OUT);
            let (home, step) = (m.pv_v(HOME), m.pv_v(STEP));
            if 2 <= n {
                m.pos = sub(m.pos, step);
                m.set_pv_i(OUT, n - 1);
                move_other(w, id, Some(step), home);
            } else if 0 < n {
                m.pos = home;
                m.set_pv_i(OUT, n - 1);
                move_other(w, id, None, home);
            }
        }
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 0, 0); }
            let m = &mut w.mobys[id];
            let n = m.pv_i(OUT);
            if n < STEPS {
                let (home, step) = (m.pv_v(HOME), m.pv_v(STEP));
                m.pos = add(m.pos, step);
                m.set_pv_i(OUT, n + 1);
                move_other(w, id, Some(scale(step, -1.0)), home);
            }
            // `+0x28`: the dust (module docs: not ported).
            let m = &mut w.mobys[id];
            if OPEN_TICKS < m.state_ticks as u16 { m.set_state(1, -1); }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    w.changed = true;
}
