//! Boldan's clock **0x0e9d** (`0x407d60`, one placed; the decomp's `func_L13_004067F8`): the second hand itself, with
//! its minute and hour hands, set from the console's clock and run a second each 60 ticks. Its pvars: the hour `+0x0`,
//! the second `+0x4`, the minute `+0x8`, the chime's wait `+0xc`, the minute hand `+0x10` (class 0x0e9f) and the hour
//! hand `+0x14` (0x0ea0), the chime's sound `+0x18`, the chimes left `+0x20`, a moby `+0x24` whose pvar `+0x38` it
//! sets at three o'clock, and a moby `+0x28` shown at the even hours.
//! * **0**: its ticks 0, 1, updated at any distance, drawn to 800; the wait and the chimes 0. The clock read
//!   (`0x1256d8` / `0x131a98`, crate::moby::MobyHost::clock: BCD hours, minutes, seconds). The three hands made
//!   (`0x31e320` = L00 `0x311320`: 0x0e9f, 0x0ea0, 0x0ea1, the last not kept) at its place and angles (the third with
//!   the hour hand's), updated at any distance, drawn to 800 and drawn, its light.
//! * **1**, at 60 ticks: a second on (the minutes and hours carried, the hours from 24 to 0), its ticks 0. The hour on
//!   the dial (past 12 less 12) even: the moby `+0x28` shown with its collision (mode `& 0xffbe | 0x1000`, then
//!   `& 0xfffd`), else hidden without (mode `& 0xefff | 0x43`). At three o'clock the moby `+0x24`'s pvar `+0x38` 1,
//!   else 0. On the hour (0 minutes 0 seconds) the chime: the level sound 0x10 (`0x35f5a0` = L00's, the def split +
//!   0x10, flags 0x21: crate::moby::MobyHost::play_level_sound_after) kept, the wait 10, the chimes the hour on the
//!   dial.
//! * **Each tick**: its x angle 6° a second, the minute hand's `6°·min + 0.1°·s`, the hour hand's `30°·h + 0.5°·min`;
//!   its ticks up (at most 65535); the wait run down (`0x2f2190`) with chimes left and the last chime done: sound
//!   0x11 (not kept), the wait 120, one chime less.

use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e9d;
const HANDS: [i16; 3] = [0x0e9f, 0x0ea0, 0x0ea1];
const DEG: f32 = f32::from_bits(0x3c8e_fa36);

const HOUR: usize = 0x0;
const SEC: usize = 0x4;
const MIN: usize = 0x8;
const WAIT: usize = 0xc;
const MIN_HAND: usize = 0x10;
const HOUR_HAND: usize = 0x14;
const CHIME: usize = 0x18;
const CHIMES: usize = 0x20;

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&k| k < w.mobys.len()) }

/// A BCD byte as the clock reads it (`(b / 16)·10 + fmod(b, 16)`).
fn bcd(b: u8) -> i32 { (b as i32 / 16) * 10 + (b as f32 % 16.0) as i32 }

fn make_hand(w: &mut MobyWorld, id: usize, class: i16, rot: crate::hero::physics::V3) -> usize {
    let (pos, light, ambient) = (w.mobys[id].pos, w.mobys[id].light, w.mobys[id].ambient);
    let h = w.create(class);
    let m = &mut w.mobys[h];
    (m.update_dist, m.draw_dist, m.drawn, m.state, m.light, m.ambient) = (0xff, 800, 1, 0, light, ambient);
    m.pos = pos;
    m.rot = rot;
    h
}

/// The dial's hour (past 12, less 12).
fn dial(hour: i32) -> f32 { let h = hour as f32; if 12.0 < h { h - 12.0 } else { h } }

/// 0x0e9d (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        (m.state_ticks, m.state, m.update_dist, m.draw_dist) = (0, 1, 0xff, 800);
        m.set_pv_i(WAIT, 0);
        m.set_pv_i(CHIMES, 0);
        let t = ctx.host.clock();
        let rot = w.mobys[id].rot;
        let a = make_hand(w, id, HANDS[0], rot);
        w.rebuild(a, &*ctx.host);
        let b = make_hand(w, id, HANDS[1], rot);
        w.rebuild(b, &*ctx.host);
        let hour_rot = w.mobys[b].rot;
        let c = make_hand(w, id, HANDS[2], hour_rot);
        w.rebuild(c, &*ctx.host);
        let m = &mut w.mobys[id];
        m.set_pv_i(MIN_HAND, a as i32);
        m.set_pv_i(HOUR_HAND, b as i32);
        m.set_pv_i(HOUR, bcd(t[3]));
        m.set_pv_i(MIN, bcd(t[2]));
        m.set_pv_i(SEC, bcd(t[1]));
    }
    if w.mobys[id].state == 1 && w.mobys[id].state_ticks == 60 {
        let m = &mut w.mobys[id];
        let (mut hour, mut min, mut sec) = (m.pv_i(HOUR), m.pv_i(MIN), m.pv_i(SEC));
        sec += 1;
        if 60 <= sec { sec = 0; min += 1; }
        if 60 <= min { min = 0; hour += 1; }
        if 24 <= hour { hour = 0; }
        m.set_pv_i(HOUR, hour);
        m.set_pv_i(MIN, min);
        m.set_pv_i(SEC, sec);
        let h = dial(hour);
        let t = h as i32;
        if let Some(k) = link(w, id, 0x28) {
            let coll = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
            let s = &mut w.mobys[k];
            if matches!(t, 0 | 2 | 4 | 6 | 8 | 10 | 12) {
                s.mode = ((s.mode & 0xffbe) | 0x1000) & 0xfffd;
                s.has_collision = coll;
            } else {
                s.has_collision = false;
                s.mode = (s.mode & 0xefff) | 0x43;
            }
        }
        if let Some(k) = link(w, id, 0x24) {
            let on = (hour == 3) as i32;
            if (w.mobys[k].pv_i(0x38) != 0) != (on != 0) { w.mobys[k].set_pv_i(0x38, on); }
        }
        if sec == 0 && min == 0 {
            let s = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 0x10, 0x21);
            let m = &mut w.mobys[id];
            m.set_pv_i(CHIME, s);
            m.set_pv_i(WAIT, 10);
            m.set_pv_i(CHIMES, h as i32);
        }
        w.mobys[id].state_ticks = 0;
    }
    let (hour, min, sec) = (w.mobys[id].pv_i(HOUR), w.mobys[id].pv_i(MIN), w.mobys[id].pv_i(SEC));
    w.mobys[id].rot[0] = (sec * 6) as f32 * DEG;
    if let Some(a) = link(w, id, MIN_HAND) { w.mobys[a].rot[0] = (min as f32 * 6.0 + sec as f32 * 0.1) * DEG; }
    if let Some(b) = link(w, id, HOUR_HAND) { w.mobys[b].rot[0] = (dial(hour) * 30.0 + min as f32 * 0.5) * DEG; }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16; }
    let mut t = m.pv_i(WAIT);
    let done = countdown(&mut t);
    m.set_pv_i(WAIT, t);
    if done && m.pv_i(CHIMES) != 0 && !ctx.host.sound_playing(m.pv_i(CHIME)) {
        m.set_pv_i(CHIME, -1);
        ctx.host.play_level_sound_after(Some(&w.mobys[id]), 0x11, 0x21);
        let m = &mut w.mobys[id];
        m.set_pv_i(WAIT, 120);
        m.set_pv_i(CHIMES, m.pv_i(CHIMES) - 1);
    }
}
