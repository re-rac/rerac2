//! 0x0578 (L01 `0x39a900`): the weather, on Oozla (level 1) and levels 15 and 19 — the three levels with a height map
//! (core header `+0xa4`, rc_formats::height_map). Rain everywhere but on levels 0xc, 0x13 and 0x200, which snow.
//!
//! In its variant's region (`0x3178b8`) it stamps the loaded variant on the height map (`0x1a94c0` = `0x1c9a40`) and,
//! every tick from its first, rains around the camera: the drops of [`tick`] (crate::particles::rain) and, for the
//! rain, the splashes of [`land`]. The wind ([`reset_wind`]) changes every 2..10 s.
//!
//! Not ported: the snow's flakes (type 0x49, `0x332f80`: only levels 0xc / 0x13 / 0x200 snow, none of them ported);
//! the frame-load words `0x1b208c` / `0x1b2090` that cut the drops by up to 40 % past 0.8 (read as 0: every drop).

use std::f32::consts::TAU;

use crate::hero::physics::{len, len2, sub, V3};
use crate::level::LevelData;
use crate::moby::{Ctx, MobyWorld};
use crate::particles::{rain, water, Particles};
use crate::rng::Rng;

/// `$gp+0x5fc`: the rain's slowest fall a second.
const MIN_FALL: f32 = 0.5;
/// `$gp+0x600`: the drops start this far above the camera.
const ABOVE: f32 = 10.0;
/// `$gp+0x604`: the reach of the rain ahead of the camera.
const REACH: f32 = 18.0;
/// `$gp+0x614`: the far drops' kind.
const FAR_KIND: i16 = 1;
/// `$gp+0x618`: the drops' ground is never lower than this far under the camera.
const BELOW: f32 = 5.0;
/// `$gp−0x76c4` (`0x1a792c`): a tick in seconds.
const TICK: f32 = 1.0 / 60.0;

/// The weather's globals (the overlay's `.bss`).
#[derive(Clone, Debug, Default)]
pub struct Weather {
    /// `0x1b2ef0` / `0x1b2ef4`: the wind, a tick.
    pub wind: [f32; 2],
    /// `0x1b2f00`: seconds until the wind changes.
    pub timer: f32,
    /// `0x1b2f04`: the drops a tick (4..8 with the wind's strength).
    pub count: i32,
    /// `0x1b27a0`: the camera's move this tick (the drops move with it).
    pub cam_move: V3,
    /// `0x1a94c0`: the variant the height map is stamped with.
    pub variant: i32,
}

impl Weather {
    /// What the rain's particle updates read (crate::particles::rain::RainAux).
    pub fn aux<'a>(&self, level: &'a LevelData, game: &crate::game::GameState) -> rain::RainAux<'a> {
        rain::RainAux { map: level.height_map.as_ref(), live: self.variant == game.variant, cam_move: self.cam_move, level: game.level }
    }
}

/// `0x39a7b8(snow)`: a new wind, each way `randf(−m, m)` a tick (m 5/60, the snow's 2.5/60), for `randf(2, 10)` s
/// (the snow's `randf(20, 10)`), and the drops a tick 4 + 4 · its strength (the snow's 2 + 1 ·): the wind's xy length
/// over the length of `(m, m)` (the game takes `(m, m, z)` with z a stale stack word; here 0).
pub fn reset_wind(wx: &mut Weather, rng: &mut Rng, snow: bool) {
    let (m, wait, most, least) = if snow { (2.5, 20.0, 3.0, 2.0) } else { (5.0, 2.0, 8.0, 4.0) };
    let m = m / 60.0;
    wx.wind = [rng.randf(-m, m), rng.randf(-m, m)];
    wx.timer = rng.randf(wait, 10.0);
    wx.count = ((len2([wx.wind[0], wx.wind[1], 0.0]) / len([m, m, 0.0])) * (most - least) + least) as i32;
}

/// 0x0578 (`0x39a900`).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    w.weather.variant = ctx.game.variant;
    let snow = matches!(ctx.game.level, 0xc | 0x13 | 0x200);
    let cam = ctx.host.camera_pos();
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            w.mobys[id].state = 2;
            reset_wind(&mut w.weather, &mut ctx.game.rng, snow);
        }
        2 => tick(w, id, ctx, cam, snow),
        _ => {}
    }
    w.mobys[id].set_pv_v(0x30, cam);
}

/// The ticks from the first (state 2): the wind's timer (`0x2e3e78`; at its end a new wind); the camera's move since
/// the last tick (pvars `+0x30`); then `n` drops (the wind's count) over a disc of reach 18 about the point 16 ahead
/// of the camera — each way its own angle and distance — 10 above the camera, falling 18 a second (less 0.15 for each
/// unit past 8 from the camera, at least 0.5; the snow 5 and 0.2) with the wind (±1/30 a tick, the snow's ±1/60),
/// kind 0 within 8 of the camera, else kind 1; and 1.4·n more about the camera within 8, at full speed, kind 0. A drop
/// is made only where the rain's top (`0x31c630`) is under its start, and ends there (or 5 under the camera, the
/// higher). The rain then tests 1.4·n places within 30 of the point 30 ahead and 1.4·n within 8 of the point 4 ahead
/// for a splash ([`land`]).
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, cam: V3, snow: bool) {
    let rng = &mut ctx.game.rng;
    let wx = &mut w.weather;
    if TICK < wx.timer {
        wx.timer -= TICK;
    } else {
        wx.timer = 0.0;
        reset_wind(wx, rng, snow);
    }
    wx.cam_move = sub(cam, w.mobys[id].pv_v(0x30));
    let (falloff, fall, jitter) = if snow { (0.2, 5.0, 1.0 / 60.0) } else { (0.15, 18.0, 1.0 / 30.0) };
    let n = wx.count;
    let (wind, map, live) = (wx.wind, ctx.level.height_map.as_ref(), wx.variant == ctx.game.variant);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    // `0x31a538(d, yaw, pitch)`: d along the camera's view (its forward row), at 10 above the camera.
    let fwd = ctx.host.camera_rows()[0];
    let ahead = |d: f32| [cam[0] + fwd[0] * d, cam[1] + fwd[1] * d, cam[2] + ABOVE];
    let around = |rng: &mut Rng, c: V3, reach: f32| {
        let (a, r) = (rng.randf(0.0, TAU), rng.randf(0.0, reach));
        let x = c[0] + a.cos() * r;
        let (a, r) = (rng.randf(0.0, TAU), rng.randf(0.0, reach));
        [x, c[1] + a.sin() * r, c[2]]
    };
    let make = |sys: &mut Particles, rng: &mut Rng, p: V3, kind: i16, vel: V3| {
        let h = rain::top(map, live, p);
        if h < cam[2] + ABOVE {
            let low = cam[2] - BELOW;
            let ground = if low < h { h } else { low };
            if !snow { rain::drop(sys, rng, ground, p, kind, vel); }
        }
    };
    let c = ahead(16.0);
    for _ in 0..n {
        let p = around(rng, c, REACH);
        let d = len2(sub(p, cam));
        let speed = if d <= 8.0 { fall } else { (fall - (d - 8.0) * falloff).max(MIN_FALL) };
        let vel = [wind[0] + rng.randf(-jitter, jitter), wind[1] + rng.randf(-jitter, jitter), -(speed / 60.0)];
        make(sys, rng, p, if d <= 8.0 { 0 } else { FAR_KIND }, vel);
    }
    let n = (n as f32 * 1.4) as i32;
    let c = [cam[0], cam[1], cam[2] + ABOVE];
    for _ in 0..n {
        let vel = [rng.randf(-jitter, jitter) + wind[0], rng.randf(-jitter, jitter) + wind[1], -(fall / 60.0)];
        let p = around(rng, c, 8.0);
        make(sys, rng, p, 0, vel);
    }
    if snow { return; }
    for (d, reach) in [(30.0, 30.0), (4.0, 8.0)] {
        let c = ahead(d);
        for _ in 0..n {
            let p = around(rng, c, reach);
            land(sys, rng, ctx.coll, ctx.scene, map, live, cam, p);
        }
    }
}

/// `0x31c778(p)`: a drop landing under `p`: a line down from the rain's top over `p` plus one step to two steps and 2
/// under it (no map: from `p` to 20 under the camera, inside 0..1024 only; nothing when the top is 5 or more above
/// the camera) through the world and the mobys (flags 0x12); on water, quicksand or a slide (surfaces 0, 3, 4) a ring
/// 0.61 above the hit (`0x32f9b8(randf(0.4, 0.6), 5250, …)`, crate::particles::water::ring), else a splash with the
/// alpha `(d·3.1833334 + 64) & 0xff` for its xy distance d from the camera.
#[allow(clippy::too_many_arguments)]
fn land(sys: &mut Particles, rng: &mut Rng, coll: Option<&rc_formats::collision::Collision>, scene: Option<&crate::collision::mobys::MobyScene>, map: Option<&rc_formats::height_map::HeightMap>, live: bool, cam: V3, p: V3) {
    let (from, to) = match map {
        None => {
            if p[0] <= 0.0 || p[1] <= 0.0 || 1024.0 <= p[0] || 1024.0 <= p[1] { return; }
            (p, [p[0], p[1], cam[2] - 20.0])
        }
        Some(m) => {
            let t = rain::top(map, live, p) + m.step;
            if cam[2] + 5.0 <= t { return; }
            ([p[0], p[1], t], [p[0], p[1], t - (m.step + m.step) - 2.0])
        }
    };
    let Some(coll) = coll else { return };
    let Some(hit) = crate::collision::coll_line_m(coll, scene, from, to, crate::collision::QueryFlags(0x12), None) else { return };
    let at = hit.point;
    if matches!(hit.surface_id(), 0 | 3 | 4) {
        let size = rng.randf(0.4, 0.6);
        water::ring(sys, rng, size, 5250.0, [at[0], at[1], at[2] + 0.61], false, None);
    } else {
        let d = len2(sub(at, cam));
        rain::splash(sys, at, ((d * 3.183_333_4 + 64.0) as i32) & 0xff);
    }
}
