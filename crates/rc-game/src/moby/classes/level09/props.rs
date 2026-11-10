//! Dobbo's small props (L09 addresses). Its pads 0x0efc / 0x0e80, which other levels share: crate::moby::classes::pads.
//!
//! * **0x0c21** ([`update_0c21`], `0x3e4630`, 30 placed): a spinner about its y angle. Its speed (`+0xbc`) is
//!   150..200° a second and its way random (`+0x95`).
//! * **0x0bf4** / **0x0bf7** ([`update_0bf4`], `0x3def10` / `0x3defb0`, 27 and 6 placed): sequence 0 started at a
//!   third of the way through for every third moby (by its address mod 3; the port uses its index), then state 1.

use crate::moby::{Ctx, MobyWorld};

pub const SPINNER: i16 = 0x0c21;
pub const SWAY_A: i16 = 0x0bf4;
pub const SWAY_B: i16 = 0x0bf7;

pub fn update_0c21(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.timer = ctx.game.rng.rand_range(0x96, 0xc8) as u8;
            if ctx.game.rng.randi(2) != 0 { m.sub_state = 1; }
            m.set_state(1, -1);
        }
        1 => {
            let step = m.timer as f32 * 0.000_290_888_25;
            m.rot[1] = crate::hero::physics::add_rot(m.rot[1], if m.sub_state != 0 { -step } else { step });
        }
        _ => {}
    }
}

pub fn update_0bf4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let n = ctx.host.seq_frames(&w.mobys[id], 0).unwrap_or(0) as f32;
    let f = (n * ((id % 3) as f32 * 0.333)) as u8;
    ctx.host.set_anim(&w.mobys[id], 0, f, 1);
    w.mobys[id].set_state(1, -1);
}

pub const WHEEL: i16 = 0x0a14;

/// 0x0a14 (`0x3ca110`, 1 placed; the decomp's `func_L09_003C8CD8`): a wheel that carries its moby group (`+4`)
/// and tie group (`+0`) round with it (`0x320b58` / `0x321068` = L00 `0x3246c0` / `0x324bd0`,
/// crate::moby::MobyWorld::attach / carry; up to 100 members), turning about its x at `+8`° a second. With neither
/// group (both −1) it waits in 0.
pub fn update_0a14(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (ties, mobys) = (m.pv_i(0), m.pv_i(4));
            if ties == -1 && mobys == -1 { return; }
            w.mobys[id].update_dist = 0xff;
            w.attach(id, 100, mobys, ties, -1);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.rot[0] = crate::hero::physics::add_rot(m.rot[0], m.pv_f(8) * 0.000_290_888_25);
            w.carry(id, &*ctx.host);
            w.changed = true;
        }
        _ => {}
    }
}

pub const FIELD: i16 = 0x0e60;

/// 0x0e60's `$gp+0x2578` / `+0x257c`: its growth's speed and ease (a second², /3600); `+0x2580` / `+0x2584` its alpha's
/// base and span; `+0x2588` / `0x1b157c` its two colours (the second one time in seven); `+0x2590` / `+0x2594` its
/// wobble (radians) and the wobble's phase (degrees a second).
const GROW: (f32, f32) = (24.0, 24.0);
const ALPHA: (u8, f32) = (16, 64.0);
const FIELD_RGB: (u32, u32) = (0x8040_4000, 0x8010_a000);
const WOBBLE: (f32, f32) = (0.06, 360.0);

/// 0x0e60 (`0x40c138`, 12 placed; the decomp's `func_L09_0040AC48`): an energy field that opens along its two axes
/// while its timer byte (`+0xbc`) is set, holds `+0x20` + `+0x24` seconds, and closes.
///
/// * **0**: hidden without collision (mode | 0x2141); then 1.
/// * **1**: with `+0xbc` set, 2; else hidden.
/// * **2**: on its first tick ([`open`], `0x40c4f8`) it shows with its class's collision. Both stretches start at
///   0.2. The one more across the camera's view (`+0x14`) opens first, and its loop sound 0 starts (mode 4,
///   `+0x1c`). Each stretch eases to 1 ([`GROW`]); the second starts once the first is past 0.6. When both are
///   open, 3.
/// * **3**: it holds for its time, then 4.
/// * **4**: the stretches ease back to 0.2 in turn. Then the timer byte clears, it hides, its sound stops, 1.
///
/// Every tick in 2..4 [`look`] (`0x40c5b8`) runs.
pub fn update_0e60(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rate = (GROW.0 / 3600.0, GROW.1 / 3600.0);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | 0x2141) & 0xefff;
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            if m.timer != 0 {
                m.set_state(2, -1);
            } else {
                m.has_collision = false;
                m.mode = (m.mode | 0x41) & 0xefff;
            }
        }
        2 => {
            if w.mobys[id].first() { open(w, id, ctx); }
            let m = &mut w.mobys[id];
            let k = m.pv_i(0x14) as usize & 1;
            let n = (k + 1) & 1;
            stretch(m, k, k, 1.0, rate, |x| x < 1.0);
            if 0.6 < m.pv_f(4 * k) { stretch(m, n, n, 1.0, rate, |_| true); }
            look(w, id, ctx);
            let m = &mut w.mobys[id];
            if 1.0 <= m.pv_f(4 * n) { m.set_state(3, -1); }
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_i(0x18, 240); }
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if (m.pv_f(0x20) + m.pv_f(0x24)) * 60.0 < t as f32 { m.set_state(4, -1); }
            look(w, id, ctx);
        }
        4 => {
            let m = &mut w.mobys[id];
            let k = m.pv_i(0x14) as usize & 1;
            let n = (k + 1) & 1;
            // The original eases each with the other's speed slot.
            stretch(m, n, k, 0.2, rate, |x| 0.2 < x);
            if m.pv_f(4 * n) < 0.6 { stretch(m, k, n, 0.2, rate, |_| true); }
            look(w, id, ctx);
            let m = &mut w.mobys[id];
            if m.pv_f(4 * k) <= 0.2 {
                m.timer = 0;
                m.has_collision = false;
                m.mode = (m.mode | 0x41) & 0xefff;
                m.set_state(1, -1);
                let h = m.pv_i(0x1c);
                if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0x1c, -1);
            }
        }
        _ => {}
    }
}

/// Stretch `k` (`+4·k`) eased toward `t` with speed slot `v` (`+8 + 4·v`) while `when` holds.
fn stretch(m: &mut crate::moby::Moby, k: usize, v: usize, t: f32, rate: (f32, f32), when: impl Fn(f32) -> bool) {
    let (mut x, mut s) = (m.pv_f(4 * k), m.pv_f(8 + 4 * v));
    if !when(x) { return; }
    crate::hero::physics::ease(t, rate.0, rate.1, 1.0, &mut x, &mut s);
    m.set_pv_f(4 * k, x);
    m.set_pv_f(8 + 4 * v, s);
}

/// `0x40c4f8` (with the first tick's show): stretches 0.2, still; which opens first.
fn open(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let has = w.class(FIELD).is_some_and(|c| c.has_collision);
    let fwd = ctx.host.camera_rows()[0];
    let m = &mut w.mobys[id];
    m.mode = (m.mode & !0x41) | 0x1000;
    m.has_collision = has;
    m.set_pv_f(0, 0.2);
    m.set_pv_f(4, 0.2);
    m.set_pv_f(8, 0.0);
    m.set_pv_f(0xc, 0.0);
    let r = crate::hero::physics::euler_rows(m.rot);
    let (a, b) = (crate::hero::physics::dot(r[0], fwd).abs(), crate::hero::physics::dot(r[1], fwd).abs());
    m.set_pv_i(0x14, (b < a) as i32);
    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
    w.mobys[id].set_pv_i(0x1c, h);
}

/// `0x40c5b8`: its colour, its wobble, its alpha by how far open (flickering ±9), its rows stretched.
fn look(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = if ctx.game.rng.randi(7) != 0 { FIELD_RGB.0 } else { FIELD_RGB.1 };
    let jitter = ctx.game.rng.rand_range(-9, 9);
    let m = &mut w.mobys[id];
    m.set_ambient_word(c);
    let ph = crate::hero::physics::add_rot(m.pv_f(0x10), WOBBLE.1 * 0.000_290_888_25);
    m.set_pv_f(0x10, ph);
    m.rot[0] = ph.cos() * WOBBLE.0;
    m.rot[1] = ph.sin() * WOBBLE.0;
    let (a, b) = (m.pv_f(0), m.pv_f(4));
    m.alpha = (ALPHA.0 as i32 + (ALPHA.1 * (a + b) * 0.5) as i32 + jitter).clamp(0, 255) as u8;
    m.mode |= 0x800 | crate::moby::mode::KEEP_ROWS;
    let mut r = crate::hero::physics::euler_rows(m.rot);
    r[0] = crate::hero::physics::scale(r[0], a);
    r[1] = crate::hero::physics::scale(r[1], b);
    m.rows = r;
}

pub const MOVER: i16 = 0x094d;

/// 0x094d (`0x3c2230`, 1 placed; the decomp's `func_L09_003C0DF8`): a mover that glides to cuboid `+0x20`'s place
/// and angles once its trigger moby (`+0x2c`) is armed (its timer byte `+0xbc`), then rests.
///
/// * **0**: with no cuboid it stops updating (mode 2) and goes to 4. Otherwise it keeps its place and angles
///   (`+0`, `+0x10`). If its uid's save count is set, it is put at the cuboid at once and goes to 3; otherwise 1.
/// * **1**: once the trigger is armed, it arms its link (`+0x30`, with its state 1), counts its uid, and goes to 2.
/// * **2**: its progress `+0x28` rises by 1 / `+0x24` a tick. Its place and angles follow the cosine ease between
///   the start and the cuboid. At 1, state 3.
/// * **3**: once its link is gone or disarmed, it stops updating.
///
/// Not ported: the level's collected table `0x21e9f0` as a second test in 0.
pub fn update_094d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, add_rot, scale, sub, sub_rot};
    let point = ctx.level.cuboid_place(w.mobys[id].pv_i(0x20));
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let Some((p, r)) = point else {
                m.mode |= 2;
                return m.set_state(4, -1);
            };
            let (pos, rot) = (m.pos, m.rot);
            m.set_pv_v(0, pos);
            m.set_pv_v(0x10, rot);
            if ctx.game.uid_count(m.uid) != 0 {
                (m.pos, m.rot) = (p, r);
                m.mode |= 2;
                m.set_state(3, -1);
            } else {
                m.set_state(1, -1);
            }
        }
        1 => {
            let t = w.mobys[id].pv_i(0x2c);
            let armed = usize::try_from(t).ok().and_then(|t| w.mobys.get(t)).is_some_and(|o| o.state < 0xfd && o.timer != 0);
            if !armed { return; }
            if let Some(l) = usize::try_from(w.mobys[id].pv_i(0x30)).ok().filter(|&l| l < w.mobys.len()) { w.mobys[l].timer = 1; }
            ctx.game.count_uid(w.mobys[id].uid);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            let Some((p, r)) = point else { return };
            let m = &mut w.mobys[id];
            let t = (m.pv_f(0x28) + 1.0 / m.pv_f(0x24)).min(1.0);
            m.set_pv_f(0x28, t);
            let e = (1.0 - (t * std::f32::consts::PI).cos()) * 0.5;
            let (p0, r0) = (m.pv_v(0), m.pv_v(0x10));
            m.pos = add(p0, scale(sub(p, p0), e));
            m.rot = std::array::from_fn(|k| add_rot(r0[k], sub_rot(r[k], r0[k]) * e));
            if 1.0 <= t { m.set_state(3, -1); }
        }
        3 => {
            let l = w.mobys[id].pv_i(0x30);
            let held = usize::try_from(l).ok().and_then(|l| w.mobys.get(l)).is_some_and(|o| o.timer != 0);
            if !held { w.mobys[id].mode |= 2; }
        }
        _ => {}
    }
}
