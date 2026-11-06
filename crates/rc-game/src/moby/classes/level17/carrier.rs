//! Smolg's carriers (L17 addresses): a floating crane that lowers its platform onto a dock and lifts it again.
//!
//! * **0x08b1** ([`update_08b1`], `0x3c5b20`, 11 placed; the decomp's `func_L17_003C45A0`). Its pvars: `+0x00` /
//!   `+0x04` the bob's phase and offset, `+0x08` / `+0x0c` the sway's phases, `+0x10` its platform (a 0x093b, −1
//!   none), `+0x14` its passenger, `+0x18` its dock (a 0x0935), `+0x1c` the timer, `+0x20` its path, then the
//!   rise-and-fall motion's words (`+0x28` / `+0x2c` the rise's and the fall's time, `+0x30` the rise, `+0x34` its
//!   state, `+0x38` the timer, `+0x3c` the low height, `+0x40` / `+0x44` the rise's range, `+0x48` / `+0x4c` the wait's;
//!   crate::moby::classes::level17::floaters::rise_fall), `+0x50` its spin a tick, `+0x54` its working sound, `+0x58`
//!   a moby it rides (−1 none), `+0x5c` stopped, `+0x60` its idle sound.
//!   * Its glow flashes as the buoys' (crate::moby::classes::level17::floaters::flash_glow).
//!   * **0** (once): its idle sound none. Riding a moby: 15 above it (`$gp+0x1254`), its platform 5.3 above it
//!     (`$gp+0x1258`), 1, and no more this tick. Else with a dock and a path, 17.5 above the path's last point (5 + 2.5 +
//!     10: `$gp+0x1244`, `+0x124c`); its platform 2.5 below it (`$gp+0x1250` −2.5, less 5); its low height kept; a spin of
//!     0.25..0.75° either way; 1.
//!   * Riding a moby, every tick: its beam drawn (the first list, `0x2e87c8` = L00 `0x2e7718`), it and its platform
//!     kept above the moby as at the start; no more.
//!   * Without a platform it rises and falls (its motion's 0: the low height its own, the rise in its range, then
//!     [`rise_start`]); with one it stays at its low height. It bobs 5 (1.5 with a platform; `$gp+0x1228` / `+0x1224`)
//!     at 45°/s (8 s, `$gp+0x1220`), sways 0.07 (`$gp+0x122c`; 5 s, 6 s) and spins. Its idle sound 2 (mode 4) is kept
//!     playing.
//!   * With a platform of class 0x093b: its beam drawn; with a passenger, a path and a dock, its platform hangs 5 below
//!     its beam's top (2.5 down its frame), the dock's place 2.5 up its target:
//!     * **1**: its working sound stopped; the passenger's timer byte set: 2, the timer 2 s (`$gp+0x1238`). Not
//!       stopped.
//!     * **2**: the platform lowered to the target by the sine of the share gone; its working sound 0 (mode 4) kept
//!       playing; not stopped; at the end 3, the timer 5 s (`$gp+0x123c`).
//!     * **3**: on its first tick the platform's sound 0. The platform at the target's height. The dock in its second
//!       leg's wait (`0x3c91e0`: `+0xbc` 2) stops its working sound, else its sound 1 (mode 4) is kept playing. With a
//!       dock (index not 0) of class 0x0935: at the end of its first leg (or with none) and its second leg not
//!       dropping, the platform at the target and the timer 0; or its first leg riding out (1) with the platform within
//!       1 of the target, the timer 0. Else at the end of the timer its working sound stopped, 4, the platform's sound 1
//!       (class 0x093b's, `0x363d28` = L00 `0x360698`), the timer 1 s (`$gp+0x1240`).
//!     * **4**: the platform raised by the sine of the share left; the passenger's timer byte 0; its sound 1 (mode 4)
//!       kept playing; at the end 1.
//!     * Then within 20 of the target (`$gp+0x1248`; past 0.01) the platform swings toward it: its pitch
//!       `π/2 − (π/2 + atan2(the height to it, the distance across))·(1 − d/20)`, at its drop below the top along the
//!       yaw to it (`0x327278` = L00 `0x325100`); the platform's roll that less a quarter turn back, its own roll the
//!       sum of both; its working sound stopped.
//! * **0x093b** ([`update_093b`], `0x3c97d0`, 3 placed; the decomp's `func_L17_003C8248`): the carrier's platform. Its
//!   glow pulses (crate::moby::classes::level17::lifts::pulse); the carrier moves it.
//! * [`draw_08b1`] (`0x3c6780`; the decomp's `func_L17_003C5200`): the beam from its platform to the top (2.5 down
//!   its frame), in steps of 0.3: quads of FX 15 (`0x2e8268` = L00 `0x2e71b8`), normal blending (ALPHA 0x44), all
//!   `0x80808080`, 0.1 to either side across `step × (camera − point)` at each step's end, the first one's start a
//!   point; ST (0,0), (1,0), (0,1), (1,1).
//!
//! Not ported: the debug prints (`0x2a5e70`) in its states 2 and 4 and the states it leaves in `0x1b3674` /
//! `$gp+0x4688`; the quads' `+0x70` word (5).

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, atan2, cross, dist, dist2, len, set_len, sub, V3};
use crate::moby::classes::level17::floaters::{flash_glow, rise_fall, rise_start, Motion};
use crate::moby::classes::props::{bob, sway};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CARRIER: i16 = 0x08b1;
pub const PLATFORM: i16 = 0x093b;

const PLATFORM_OFF: usize = 0x10;
const PASSENGER: usize = 0x14;
const DOCK: usize = 0x18;
const TIMER: usize = 0x1c;
const PATH: usize = 0x20;
const SPIN: usize = 0x50;
const SOUND: usize = 0x54;
const RIDES: usize = 0x58;
const STOPPED: usize = 0x5c;
const IDLE: usize = 0x60;

const MOTION: Motion = Motion { rise: 0x30, state: 0x34, timer: 0x38, base: 0x3c, min_wait: 0x48, max_wait: 0x4c, rise_t: 0x28, fall_t: 0x2c };

const DEG_S: f32 = 0.000_290_888_25;
const HALF_PI: f32 = f32::from_bits(0x3fc9_0fdc);
const BOB: [f32; 2] = [5.0, 1.5];
const BOB_T: f32 = 8.0;
const SWAY: f32 = f32::from_bits(0x3d8f_5c29);
const LOWER: f32 = 2.0;
const HOLD: f32 = 5.0;
const RAISE: f32 = 1.0;
const HANG: f32 = 5.0;
const SWING: f32 = 20.0;
const DOCK_UP: f32 = 2.5;
const TOP: f32 = -2.5;
const RIDE_UP: f32 = 15.0;
const RIDE_PLATFORM: f32 = f32::from_bits(0x40a9_999a);

fn idx(w: &MobyWorld, k: i32) -> Option<usize> { usize::try_from(k).ok().filter(|&k| k < w.mobys.len()) }

/// The top of its beam: 2.5 down its frame.
fn top(m: &crate::moby::Moby) -> V3 { add(m.pos, std::array::from_fn(|k| m.rows[2][k] * TOP)) }

/// Its sound at `off` stopped (when playing) and none.
fn stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) {
    let h = w.mobys[id].pv_i(off);
    if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(off, -1);
}

/// Its sound `sound` (mode 4) at `off` kept playing.
fn keep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize, sound: i32) {
    let h = w.mobys[id].pv_i(off);
    if h < 0 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], sound, 4);
        w.mobys[id].set_pv_i(off, h);
    }
}

/// Riding the moby `src`: 15 above it, its platform 5.3 above it.
fn ride(w: &mut MobyWorld, id: usize, src: usize) {
    let at = w.mobys[src].pos;
    w.mobys[id].pos = [at[0], at[1], at[2] + RIDE_UP];
    if let Some(p) = idx(w, w.mobys[id].pv_i(PLATFORM_OFF)) { w.mobys[p].pos = [at[0], at[1], at[2] + RIDE_PLATFORM]; }
}

fn tick(w: &mut MobyWorld, id: usize) -> bool {
    let mut t = w.mobys[id].pv_i(TIMER);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(TIMER, t);
    out
}

/// 0x08b1 (module docs).
pub fn update_08b1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let has_platform = w.mobys[id].pv_i(PLATFORM_OFF) != -1;
    let amp = BOB[has_platform as usize];
    flash_glow(w, id, ctx);
    if w.mobys[id].state == 0 {
        w.mobys[id].set_pv_i(IDLE, -1);
        if let Some(src) = idx(w, w.mobys[id].pv_i(RIDES)) {
            ride(w, id, src);
            w.mobys[id].set_state(1, -1);
            return;
        }
        let (dock, path) = (w.mobys[id].pv_i(DOCK), w.mobys[id].pv_i(PATH));
        if dock != -1 && path != -1 {
            if let Some(last) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).and_then(|p| p.last()) {
                w.mobys[id].pos[2] = last[2] + HANG + DOCK_UP + 10.0;
            }
        }
        let z = w.mobys[id].pos[2];
        if let Some(p) = idx(w, w.mobys[id].pv_i(PLATFORM_OFF)) { w.mobys[p].pos[2] = z - TOP - HANG; }
        w.mobys[id].set_pv_f(MOTION.base, z);
        let s = ctx.game.rng.randf(f32::from_bits(0x3b8e_fa37), f32::from_bits(0x3c56_7752));
        let s = if ctx.game.rng.randi(2) != 0 { -s } else { s };
        w.mobys[id].set_pv_f(SPIN, s);
        w.mobys[id].set_state(1, -1);
    }
    if let Some(src) = idx(w, w.mobys[id].pv_i(RIDES)) {
        w.register_draw_first(id);
        ride(w, id, src);
        return;
    }
    if !has_platform {
        if w.mobys[id].pv_i(MOTION.state) == 0 {
            let m = &mut w.mobys[id];
            let z = m.pos[2];
            m.set_pv_f(MOTION.base, z);
            let (lo, hi) = (m.pv_f(0x40), m.pv_f(0x44));
            let r = ctx.game.rng.randf(lo, hi);
            w.mobys[id].set_pv_f(MOTION.rise, r);
            rise_start(w, id, ctx, &MOTION);
        } else {
            rise_fall(w, id, ctx, &MOTION);
        }
    } else {
        let m = &mut w.mobys[id];
        m.pos[2] = m.pv_f(MOTION.base);
    }
    let m = &mut w.mobys[id];
    let z = m.pos[2];
    bob(m, amp, 360.0 / BOB_T * DEG_S, z, 0, 4);
    sway(m, SWAY, 360.0 / 5.0 * DEG_S, 360.0 / 6.0 * DEG_S, 8, 0xc);
    m.rot[2] = add_rot(m.rot[2], m.pv_f(SPIN));
    let off = top(&w.mobys[id]);
    keep(w, id, ctx, IDLE, 2);
    let Some(plat) = idx(w, w.mobys[id].pv_i(PLATFORM_OFF)).filter(|&p| w.mobys[p].o_class == PLATFORM) else { return };
    w.register_draw_first(id);
    let (pass, path, dock) = { let m = &w.mobys[id]; (m.pv_i(PASSENGER), m.pv_i(PATH), m.pv_i(DOCK)) };
    if pass == -1 || path == -1 || dock == -1 { return; }
    let (Some(pass), Some(dock_id)) = (idx(w, pass), idx(w, dock)) else { return };
    let mut target = w.mobys[dock_id].pos;
    target[2] += DOCK_UP;
    let hang = off[2] - HANG;
    let drop = hang - target[2];
    w.mobys[plat].pos = [off[0], off[1], hang];
    match w.mobys[id].state {
        1 => {
            stop(w, id, ctx, SOUND);
            if w.mobys[pass].timer != 0 {
                w.mobys[id].set_state(2, -1);
                w.mobys[id].set_pv_i(TIMER, (LOWER * 60.0) as i32);
            }
            w.mobys[plat].pos[2] = hang;
            w.mobys[id].set_pv_i(STOPPED, 0);
        }
        2 => {
            let r = w.mobys[id].pv_i(TIMER) as f32 / (LOWER * 60.0);
            w.mobys[plat].pos[2] = hang - drop * ((1.0 - r) * HALF_PI).sin();
            keep(w, id, ctx, SOUND, 0);
            w.mobys[id].set_pv_i(STOPPED, 0);
            if tick(w, id) {
                w.mobys[id].set_state(3, -1);
                w.mobys[id].set_pv_i(TIMER, (HOLD * 60.0) as i32);
            }
        }
        3 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[plat], 0, 0); }
            w.mobys[plat].pos[2] = hang - drop;
            let mut ready = false;
            if w.mobys[dock_id].pv_i(0xbc) == 2 { stop(w, id, ctx, SOUND); } else { keep(w, id, ctx, SOUND, 1); }
            if dock != 0 && w.mobys[dock_id].o_class == crate::moby::classes::level17::lifts::DOCK {
                let d = &w.mobys[dock_id];
                let (leg1, t1, leg2) = (d.pv_i(0xac), d.pv_i(0xa0), d.pv_i(0xbc));
                if (leg1 == 2 || t1 == -1) && leg2 != 3 {
                    ready = true;
                    w.mobys[plat].pos = target;
                    w.mobys[id].set_pv_i(TIMER, 0);
                } else if leg1 == 1 && dist(w.mobys[plat].pos, target) < 1.0 {
                    w.mobys[id].set_pv_i(TIMER, 0);
                    ready = true;
                }
            }
            if !ready && tick(w, id) {
                stop(w, id, ctx, SOUND);
                w.mobys[id].set_state(4, -1);
                ctx.host.play_sound_of(&w.mobys[id], PLATFORM, 1, 0);
                w.mobys[id].set_pv_i(TIMER, (RAISE * 60.0) as i32);
            }
        }
        4 => {
            let r = w.mobys[id].pv_i(TIMER) as f32 / (RAISE * 60.0);
            w.mobys[plat].pos[2] = hang - drop * (r * HALF_PI).sin();
            w.mobys[pass].timer = 0;
            keep(w, id, ctx, SOUND, 1);
            if tick(w, id) { w.mobys[id].set_state(1, -1); }
        }
        _ => {}
    }
    let ppos = w.mobys[plat].pos;
    let d = dist(target, ppos);
    if d < SWING && 0.01 < d {
        let blend = 1.0 - d / SWING;
        let reach = off[2] - ppos[2];
        let a = atan2(dist2(off, target), target[2] - off[2]);
        let pitch = HALF_PI - (HALF_PI + a) * blend;
        let yaw = atan2(target[0] - off[0], target[1] - off[1]);
        let p = -pitch;
        // `0x327278(out, r, yaw, pitch)`.
        let v = [yaw.cos() * reach * p.cos(), yaw.sin() * reach * p.cos(), p.sin() * reach];
        w.mobys[plat].pos = add(off, v);
        let roll = add_rot(p, HALF_PI);
        w.mobys[plat].rot[1] = roll;
        let m = &mut w.mobys[id];
        m.rot[1] = add_rot(m.rot[1], roll);
        stop(w, id, ctx, SOUND);
    }
}

/// 0x093b (module docs).
pub fn update_093b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { crate::moby::classes::level17::lifts::pulse(w, id, ctx); }

/// 0x08b1's beam (module docs).
pub fn draw_08b1(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let end = top(m);
    let Some(p) = idx(w, m.pv_i(PLATFORM_OFF)) else { return };
    let mut cur = w.mobys[p].pos;
    let mut delta = sub(end, cur);
    let length = len(delta);
    let mut along = 0.0;
    let (mut c0, mut c2) = (cur, cur);
    const ST: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    while along < length {
        let step = (length - along).min(f32::from_bits(0x3e99_999a));
        along += step;
        delta = set_len(delta, step);
        cur = add(cur, delta);
        let side = set_len(cross(delta, sub(cam, cur)), f32::from_bits(0x3dcc_cccd));
        let (c1, c3) = (add(cur, side), sub(cur, side));
        out.push(FxQuad { fx: 15, additive: false, corners: [c0, c1, c2, c3], st: ST, rgba: [0x8080_8080; 4] });
        (c0, c2) = (c1, c3);
    }
}
