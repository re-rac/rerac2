//! The Thermanator (item 0x27, crate::hero::items::ROWS; L00's record `0x264720`: attachment 0, stance 1, use
//! sequence 0x38 (the firing arm's 0x39 moving, 0x3e crouched), the use keeps the stance, icon 0xea82, no ammo): the
//! hand moby **0x0fb0** (`0x4105a8`, the decomp's `func_00333AF8`, called by the item pass while it is the hand item)
//! and its two shots, the cold **0x0bd8** that freezes the liquid grid (crate::water::grid) and the hot **0x0bd7**
//! that thaws it. The hand fires only at a grid mover (class type 0x12) it can reach: the cold shot at water, the
//! hot one at ice. A mover's hits (`+0xbc`, its group's with it: `0x31c970`, crate::moby::MobyWorld::poke_group) up
//! to its turn (`+0x15`, 25) flip it (the grid's state 1).
//!
//! **The hand** (pvars: `+0x10` the arc's end, `+0x20` the face's normal there, `+0x30` the muzzle (joint 0,
//! `0x3127c8`), `+0x40` a mover is aimed at, `+0x48` the meter's eased fill, `+0x4c` its fill, `+0x50` the fade,
//! `+0x54` the moby the arc met, `+0x58` the shots' wait, `+0x5c` the mover is ice, `+0x60` it is turning). Each tick:
//! * the arc (`0x37e328(1e-5, reach, 0.25, 11 a second², ..)`, no target: the reach 4.5, + 6.5 in the look stance
//!   0x1d; Ratchet's look stance 1 going to its twin 0x1d, crate::moby::classes::gravity_bomb::launch_with) followed
//!   to what it meets (`0x37e988`, crate::moby::classes::gravity_bomb::landing_hit);
//! * a grid mover met: its turn 25 (its group's too, `0x410b00`), the fill its hits / its turn (`0x2effa8`), on ice
//!   `1 − that` (0 → −0.001); aimed at, ice or water, turning; met nothing: the fill −1000;
//! * **0** → **2** (1 on the way); **2**: ○ held (`0x138320 & 0x18b050`), a mover aimed at and not turning: the use
//!   (`0x2af5e0`), **3**, a shot ([`spawn`]: the cold one at water, the hot one at ice) from the muzzle along the
//!   arc, its class sound 1 (`0x360698(1, 0, hand, 0xfb0)`), the wait 7 (hot: 14); **3**: let go, no mover, or
//!   turning: the use over (`0x2af848`), **2**; else the wait out: another shot along the arc jittered ±0.005 on each
//!   axis at its speed, the sound, the wait again; **4** (put away): nothing;
//! * put away (`0x18b064` = 3): **4**, the fade −0.02 a tick; else +0.01; held to 0..1;
//! * the meter's fill eased by 0.01 a tick toward the fill (taken at once from below −100 or to it);
//! * at a fill of 0..1 the side meter (`0x2fc520(fill, 0, c, c₂)`, crate::hud::SideMeter) in `c` pulsing
//!   `0x601020ff` .. `0x600000c0` and `c₂` `0x60ffc000` .. `0x40ffc000` by `sin(2π·(frame % 90)/90 − π)` (`0x3271e0`);
//! * its draw ([`draw`], `0x410b58`): in states 2..3 with a mover aimed at, the landing marker at the arc's end
//!   (`0x37dfc0(1, end, normal, 0)`, crate::moby::classes::gravity_bomb::marker_quads).
//!
//! **The cold shot 0x0bd8** (`0x3e3178` spawns it: the hand `+0x30`, its velocity `+0x00`, two spins of
//! `randf(180°, 360°)` a second `+0x14` / `+0x18`, update and draw distances 0xff, half its class's size; `0x3e2c28`,
//! its draw `0x3e35f8` empty). Pvars: `+0x20` where it hit, `+0x34` the ticks, `+0x38` its mode (0 nothing, 1 the
//! water met, 2 above a still water), `+0x40` (never set), `+0x44` the mover was turning, `+0x48` the mover.
//! * **0**: mode | 8; the dialogue's line 0xdb8 asked for (`0x1a63a8`, crate::audio::speech) when it is free; → 1.
//! * **1**: it moves, falls 11 a second², spins; eight frost grains about it (`randf_signed(0, 0.09)` on each axis,
//!   20..40 ticks, its velocity: crate::particles::frost); the line from its last place (flags 0, itself left out;
//!   the hand and Ratchet are no hit, `0x3e3280`) — met: there (and `+0x20`), the hit ([`hit_cold`], `0x3e3400`):
//!   mode 2 → **2**; mode 1 → its z the grid's height there (`0x37dc10`), a burst unless the mover was turning
//!   ([`burst`], `0x3e3320`), **3** for 80 ticks; mode 0 → **3**.
//! * **2**: it moves, falls and spins; the hit again on the mover kept: mode 1 → its class sound 0, **3** as above.
//! * **3**: it moves, its velocity × 0.8 and falling 8.8 a second², its spins × 0.8; its alpha −2 (to 0); a bubble
//!   rising to the water's height above it (`0x31d7a0`, crate::water::grid::Grid::surface: size `randf(0.05, 0.1)`·
//!   210000, `(randf(±0.5), randf(±0.5), randf(−0.5, 2))` at `randf(1/15, 2/15)·0.2` a tick,
//!   crate::particles::water::bubble); its ticks out: **4**. **4**: gone.
//!
//! **The hit** ([`hit_cold`], `0x3e3400`): the moby the line met (in mode 2 the one kept); nothing met (not in mode
//! 2): nothing. Not of class type 0x12: mode 0. A mover not turning (`+0x16`): the grid's height at it (`0x37dc10`;
//! 0 off the patches; in mode 2 at the first hit) under the shot and the mover still (`+0x0c` 0): mode 2, the mover
//! kept. Else, thawed (`+0x14` 0): the ripple (`0x37d5a0(.., 1, x, y, 0.5, −0.03)` on every patch,
//! crate::water::grid::Grid::disturb, added) and, Ratchet not in the water (`0x2adff8`: the swim groups 0x11 / 0x12,
//! the states 0x67 / 0x7f / 0x72 / 0x73), one hit ([`count`]). Mode 1, `+0x44` whether it was turning.
//!
//! **The hot shot 0x0bd7** (`0x3e1a80` spawns it: the hand `+0x40`, its velocity, two spins `randf_signed(180°,
//! 360°)` a second, the distances, half size, `+0x5c` 30; `0x3e16c0`, the decomp's `func_L00_003E0240` and L04's
//! `func_L04_0041AD18`). Pvars: `+0x20` the face's normal and `+0x30` the place it hit, `+0x44` / `+0x58` its fade
//! ticks and level, `+0x48` its flames, `+0x4c` its puffs, `+0x50` the puffs made, `+0x54` the mover was turning.
//! * **0**: the counts 0 (the flames' table cleared once a level, `gp+0x14e8`); the line 0xdb9 asked for; → 1.
//! * **1**: it moves, falls, spins; a flame (`0x3e1d08`) and a puff (`0x3e2590`, half its step in size); the line
//!   (`0x3e1b90`) — met: there, **2**, 20 ticks and the level 128, mode | 8, its velocity 0, the thaw ([`hit_hot`],
//!   `0x3e1c18`), its class sound 0, the normal and the place kept; unless the mover was turning eight lava sparks
//!   (0.133 a tick, a random way 22.5°..180° from the level: crate::particles::lava::spark).
//! * **2**: a puff (0.1); it moves, falling 5.5 a second²; its alpha −4, its ticks down, its alpha −4 more from 5
//!   on; the ticks out and the alpha under 4: **3**.
//! * **3**: its alpha −1 (to 0); the level −4 (from 5 on), else −1 (to 0); no flames, no puffs and the level 0:
//!   **4**, no collision, mode | 0x41, out of the target list. **4**: its 30 ticks out: gone.
//! * Every tick but the last: its flames turn (`0x3e1dd8`: 36° a tick, gone at 360°) and its puffs age (`0x3e27e8`:
//!   50..60 ticks); its draw (`0x3e1ee0`).
//!
//! **The thaw** ([`hit_hot`], `0x3e1c18`): the moby the line met, of class type 0x12, not turning: on ice two hits;
//! `+0x54` whether it was turning.
//!
//! **A hit** ([`count`]): the mover's `+0xbc` up; at 25 or more the dialogue's held line let go (`0x2b6d28`); its
//! group (`+0x21`) all given the count.
//!
//! **The hot shot's draw** ([`draw_hot`], `0x3e1ee0`): its scorch, flames, glow and puffs.
//!
//! Not ported: the puffs' heat shimmer ([`draw_hot`]), the reach's debug
//! override (`0x381320`: 4.5 here), the target's missing-pvars message (`0x2a48f0`).

use crate::hero::physics::{add, add_rot, atan2, dist, len, len2, scale, set_len, sub, unit, V3};
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

/// The item.
pub const ITEM: i32 = 0x27;
/// The hand's class (item 0x27's row `+0x04`).
pub const HAND_CLASS: i16 = 0xfb0;
/// The cold shot (freezes) and the hot shot (thaws).
pub const COLD: i16 = 0xbd8;
pub const HOT: i16 = 0xbd7;
/// The grid movers' class type and their pvars: still (`+0x0c` the rate), ice, the turn, turning.
const TARGET_KIND: i16 = 0x12;
const RATE: usize = 0x0c;
const FROZEN: usize = 0x14;
const TURN_AT: usize = 0x15;
const TURNING: usize = 0x16;
/// A mover's turn (`0x19`).
const TURN: u8 = 25;

/// The arc (`0x37e328`'s arguments).
const LEAST: u32 = 0x3727_c5ac;
const REACH: f32 = 4.5;
const LOOK_REACH: f32 = 6.5;
const SPEED: f32 = 0.25;
/// 11 a second² (`0x3b483fb8`): the arc's and the shots' fall.
const GRAVITY: u32 = 0x3b48_3fb8;

/// The hand's pvars.
mod hand {
    pub const END: usize = 0x10;
    pub const NORMAL: usize = 0x20;
    pub const MUZZLE: usize = 0x30;
    pub const AIMED: usize = 0x40;
    pub const EASED: usize = 0x48;
    pub const FILL: usize = 0x4c;
    pub const FADE: usize = 0x50;
    pub const MET: usize = 0x54;
    pub const WAIT: usize = 0x58;
    pub const ICE: usize = 0x5c;
    pub const TURNING: usize = 0x60;
}

/// The cold shot's pvars.
mod cold {
    pub const AT: usize = 0x20;
    pub const OWNER: usize = 0x30;
    pub const TICKS: usize = 0x34;
    pub const MODE: usize = 0x38;
    pub const KEEP: usize = 0x40;
    pub const TURNING: usize = 0x44;
    pub const MOVER: usize = 0x48;
}

/// The hot shot's pvars.
mod hot {
    pub const NORMAL: usize = 0x20;
    pub const AT: usize = 0x30;
    pub const OWNER: usize = 0x40;
    pub const TICKS: usize = 0x44;
    pub const FLAMES: usize = 0x48;
    pub const PUFFS: usize = 0x4c;
    pub const MADE: usize = 0x50;
    pub const TURNING: usize = 0x54;
    pub const LEVEL: usize = 0x58;
    pub const GONE: usize = 0x5c;
}

/// The hot shots' flames (`0x247880`, 100 of 0x40 bytes) and puffs (`0x2533e0`, 300 of 0x50) (module docs).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    flames: Vec<Option<Flame>>,
    puffs: Vec<Option<Puff>>,
    /// `gp+0x14e8`: the flames' table cleared this level.
    cleared: bool,
}

impl Mem {
    fn ready(&mut self) {
        if self.flames.len() < 100 { self.flames.resize(100, None); }
        if self.puffs.len() < 300 { self.puffs.resize(300, None); }
    }
}

/// A flame (`0x40` bytes): `+0x00` its way out in the shot's frame, `+0x10` its place, `+0x20` that way as a unit,
/// `+0x30` its turn, `+0x34` its shot.
#[derive(Clone, Copy, Debug)]
struct Flame {
    owner: usize,
    off: V3,
    pos: V3,
    turn: f32,
}

/// A puff (`0x50` bytes): `+0x00` its place, `+0x10` its velocity, `+0x20` / `+0x24` its spin and angle, `+0x28` its
/// size, `+0x2c` / `+0x30` its colour and the colour drawn, `+0x34` / `+0x38` its alpha and the alpha drawn, `+0x3c`
/// its blend (1 normal, 2 additive), `+0x40` / `+0x44` its life left and its life, `+0x48` its shot.
#[derive(Clone, Copy, Debug)]
struct Puff {
    owner: usize,
    pos: V3,
    vel: V3,
    spin: f32,
    angle: f32,
    size: f32,
    color: u32,
    color2: u32,
    alpha: i32,
    alpha2: i32,
    kind: i32,
    life: i32,
    duration: i32,
}

fn kind(w: &MobyWorld, m: usize) -> i16 { w.mobys.get(m).and_then(|x| w.class(x.o_class)).map_or(0, |c| c.kind) }

fn pvars(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].pvars.len() < crate::moby::PVAR_BYTES { w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0); }
}

/// `0x2adff8`: Ratchet in the water.
fn in_water(ctx: &Ctx) -> bool { matches!(ctx.hero.group, 0x11 | 0x12) || matches!(ctx.hero.state, 0x67 | 0x7f | 0x72 | 0x73) }

/// `0x410b00(group, v)`: every member of the group gets its turn `+0x15` = `v`.
fn set_turn(w: &mut MobyWorld, group: i32, v: u8) {
    let Some(members) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in members {
        if let Some(m) = w.mobys.get_mut(i) {
            if TURN_AT < m.pvars.len() { m.set_pv_b(TURN_AT, v); }
        }
    }
}

/// A mover's group (`+0x21`, 0xff none).
fn group(w: &MobyWorld, m: usize) -> Option<i32> { Some(w.mobys[m].group as u8).filter(|&g| g != 0xff).map(i32::from) }

/// `0x4105a8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pvars(w, id);
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    w.mobys[id].set_pv_v(hand::MUZZLE, muzzle);
    let reach = if ctx.hero.state == 0x1d { REACH + LOOK_REACH } else { REACH };
    {
        let m = &mut w.mobys[id];
        ai::set_link(m, hand::MET, None);
        m.set_pv_i(hand::AIMED, 0);
    }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let g = f32::from_bits(GRAVITY);
    let dir = super::gravity_bomb::launch_with(ctx, f32::from_bits(LEAST), reach, SPEED, g, muzzle, None);
    let landed = super::gravity_bomb::landing_hit(w, ctx, id, muzzle, dir, g);
    if let Some((p, n, _)) = landed {
        let m = &mut w.mobys[id];
        m.set_pv_v(hand::END, p);
        m.set_pv_v(hand::NORMAL, n);
    }
    let met = landed.and_then(|(_, _, t)| t).filter(|&t| t < w.mobys.len());
    ai::set_link(&mut w.mobys[id], hand::MET, met);
    match met {
        Some(t) => {
            if kind(w, t) == TARGET_KIND && !w.mobys[t].pvars.is_empty() {
                w.mobys[t].set_pv_b(TURN_AT, TURN);
                if let Some(gr) = group(w, t) { set_turn(w, gr, TURN); }
                let x = &w.mobys[t];
                let ratio = x.timer as f32 / x.pv_b(TURN_AT) as f32;
                let ice = x.pv_b(FROZEN);
                let turning = x.pv_s(TURNING) != 0;
                let mut fill = ratio;
                if ice != 0 {
                    fill = 1.0 - ratio;
                    if fill == 0.0 { fill = f32::from_bits(0xba83_126f); }
                }
                let m = &mut w.mobys[id];
                m.set_pv_f(hand::FILL, fill);
                m.set_pv_i(hand::ICE, ice as i32);
                m.set_pv_i(hand::AIMED, 1);
                m.set_pv_i(hand::TURNING, turning as i32);
            }
        }
        None => w.mobys[id].set_pv_f(hand::FILL, -1000.0),
    }
    let it = ctx.hero.items;
    let held = it.held & it.trigger != 0;
    let (aimed, turning) = { let m = &w.mobys[id]; (met.is_some() && m.pv_i(hand::AIMED) != 0, m.pv_i(hand::TURNING) != 0) };
    match w.mobys[id].state {
        0 | 1 => w.mobys[id].state = 2,
        2 => {
            if held && aimed && !turning {
                ctx.hero.items.use_start(ctx.hero.group);
                w.mobys[id].state = 3;
                fire(w, id, ctx, muzzle, dir);
            }
        }
        3 => {
            if !held || !aimed || turning {
                let idle = ctx.hero.idle_seq();
                ctx.hero.items.use_end(ctx.hero.state, ctx.hero_view.seq_b, idle);
                w.mobys[id].state = 2;
            } else {
                let mut t = w.mobys[id].pv_i(hand::WAIT);
                let out = countdown(&mut t);
                w.mobys[id].set_pv_i(hand::WAIT, t);
                if out {
                    let l = len(dir);
                    let g = &mut ctx.game.rng;
                    let j = f32::from_bits(0x3ba3_d70a);
                    let mut s = dir;
                    for v in &mut s { *v += g.randf(-j, j); }
                    fire(w, id, ctx, muzzle, set_len(s, l));
                }
            }
        }
        _ => {}
    }
    {
        let m = &mut w.mobys[id];
        let fade = if ctx.hero.items.ready == 3 {
            m.state = 4;
            m.pv_f(hand::FADE) - f32::from_bits(0x3ca3_d70a)
        } else {
            m.pv_f(hand::FADE) + f32::from_bits(0x3c23_d70a)
        };
        m.set_pv_f(hand::FADE, fade.clamp(0.0, 1.0));
        let f = m.pv_f(hand::FILL);
        let s = m.pv_f(hand::EASED);
        let step = f32::from_bits(0x3c23_d70a);
        let eased = if f <= -100.0 || s < -100.0 {
            f
        } else if f < s - step {
            s - step
        } else if s + step < f {
            s + step
        } else {
            f
        };
        m.set_pv_f(hand::EASED, eased);
    }
    // `0x3271e0(frame, 90, c0, c1)`: `0x2f0000(c0, c1, sin(2π·(frame % 90)/90 − π)·0.5 + 0.5)`.
    let pi = std::f32::consts::PI;
    let t = (2.0 * (ctx.game.frame % 90) as f32 * pi / 90.0 - pi).sin() * 0.5 + 0.5;
    let a = crate::hud::lerp_rgba(t, 0x6010_20ff, 0x6000_00c0);
    let b = crate::hud::lerp_rgba(t, 0x60ff_c000, 0x40ff_c000);
    let eased = w.mobys[id].pv_f(hand::EASED);
    if (0.0..=1.0).contains(&eased) { ctx.game.side_meter.show(eased, 0, a, b); }
    w.register_draw(id);
}

/// A shot from the hand (the cold one at water, the hot one at ice), its sound and the wait.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, muzzle: V3, vel: V3) {
    let ice = w.mobys[id].pv_i(hand::ICE) != 0;
    spawn(w, ctx, if ice { HOT } else { COLD }, id, muzzle, vel);
    ctx.host.play_sound_of(&w.mobys[id], HAND_CLASS, 1, 0);
    w.mobys[id].set_pv_i(hand::WAIT, if ice { 14 } else { 7 });
}

/// `0x3e3178` (cold) / `0x3e1a80` (hot): the shot at `pos` moving `vel` a tick (module docs).
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, class: i16, owner: usize, pos: V3, vel: V3) -> Option<usize> {
    w.class(class)?;
    let id = w.create(class);
    let k = f32::from_bits(0x3998_825c);
    let g = &mut ctx.game.rng;
    let spins = if class == HOT {
        [g.randf_signed(180.0, 360.0) * k, g.randf_signed(180.0, 360.0) * k]
    } else {
        [g.randf(180.0, 360.0) * k, g.randf(180.0, 360.0) * k]
    };
    let m = &mut w.mobys[id];
    m.update = Some(if class == HOT { update_hot } else { update_cold });
    m.mode &= !mode::NO_UPDATE;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    ai::set_link(m, if class == HOT { hot::OWNER } else { cold::OWNER }, Some(owner));
    m.pos = pos;
    m.set_pv_v(0, vel);
    m.set_pv_i(0x10, 0);
    m.set_pv_f(0x14, spins[0]);
    m.set_pv_f(0x18, spins[1]);
    m.update_dist = 0xff;
    m.scale *= 0.5;
    m.draw_dist = 0xff;
    m.state = 0;
    if class == HOT { m.set_pv_i(hot::GONE, 30); }
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    Some(id)
}

/// The shots' step: moved by the velocity, falling `fall` a tick, spun (`0x2eee70`, `0x2efe60`).
fn step(w: &mut MobyWorld, id: usize, fall: f32) -> V3 {
    let m = &mut w.mobys[id];
    let old = m.pos;
    let mut v = m.pv_v(0);
    m.pos = add(m.pos, v);
    v[2] -= fall;
    m.set_pv_v(0, v);
    m.rot[1] = add_rot(m.rot[1], m.pv_f(0x14));
    m.rot[2] = add_rot(m.rot[2], m.pv_f(0x18));
    old
}

/// `0x3e3280` / `0x3e1b90`: the line from `from` to the shot (flags 0, the shot left out); the hand and Ratchet are no
/// hit. Met: the shot there; the moby met (None: the world) and the face's normal.
fn line(w: &mut MobyWorld, ctx: &Ctx, id: usize, from: V3) -> Option<(Option<usize>, V3)> {
    let h = w.coll_line(ctx, from, w.mobys[id].pos, 0, Some(id))?;
    if let Some(m) = h.moby {
        if w.mobys.get(m).is_some_and(|x| x.o_class == HAND_CLASS) || Some(m) == ctx.hero_moby { return None; }
    }
    w.mobys[id].pos = h.point;
    Some((h.moby.filter(|&m| m < w.mobys.len()), unit(h.normal)))
}

/// One hit (or `n`) on the mover `t` (module docs).
fn count(w: &mut MobyWorld, ctx: &mut Ctx, t: usize, n: u8) {
    let v = w.mobys[t].timer.wrapping_add(n);
    w.mobys[t].timer = v;
    if TURN <= v && ctx.host.speech_held() { ctx.host.speech_go(); }
    if let Some(g) = group(w, t) { w.poke_group(g, v); }
}

/// `0x3e3400` (module docs). `met`: the line's moby this tick (unread in mode 2).
fn hit_cold(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, met: Option<Option<usize>>) {
    let mode = w.mobys[id].pv_i(cold::MODE);
    let target = met.flatten();
    if target.is_none() && mode != 2 { return; }
    let object = if mode == 2 { ai::link(&w.mobys[id], cold::MOVER) } else { target };
    let Some(t) = object.filter(|&t| t < w.mobys.len()) else { return };
    if kind(w, t) != TARGET_KIND {
        w.mobys[id].set_pv_i(cold::MODE, 0);
        return;
    }
    if !w.mobys[t].pvars.is_empty() {
        if w.mobys[t].pv_s(TURNING) == 0 {
            let p = if mode != 2 { w.mobys[id].pos } else { w.mobys[id].pv_v(cold::AT) };
            let h = w.liquid.height(p).unwrap_or(0.0);
            if h < w.mobys[id].pos[2] && w.mobys[t].pv_f(RATE) == 0.0 {
                ai::set_link(&mut w.mobys[id], cold::MOVER, Some(t));
                w.mobys[id].set_pv_i(cold::MODE, 2);
                return;
            }
            if w.mobys[t].pv_b(FROZEN) == 0 {
                let [x, y, _] = w.mobys[id].pos;
                let n = w.liquid.patches.len();
                w.liquid.disturb(x, y, 0.5, f32::from_bits(0xbcf5_c28f), 0..n, true);
                if !in_water(ctx) { count(w, ctx, t, 1); }
            }
            w.mobys[id].set_pv_i(cold::TURNING, 0);
        } else {
            w.mobys[id].set_pv_i(cold::TURNING, 1);
        }
    }
    w.mobys[id].set_pv_i(cold::MODE, 1);
}

/// `0x3e3320`: 48 drops from the shot (0..0.05 out a random way, 0.05..0.108 up, 90..120 ticks, kind 0 or 1:
/// crate::particles::water::drop).
pub fn burst(ctx: &mut Ctx, at: V3) {
    for _ in 0..48 {
        let g = &mut ctx.game.rng;
        let a = g.rand_angle();
        let r = g.randf(0.0, f32::from_bits(0x3d4c_ccce));
        let up = g.randf(f32::from_bits(0x3d4c_ccce), f32::from_bits(0x3ddd_dddf));
        let life = g.rand_range(90, 120);
        let kind = g.randi(2);
        let v = [a.cos() * r, a.sin() * r, up];
        if let Some(s) = ctx.parts.as_deref_mut() { crate::particles::water::drop(s, &mut ctx.game.rng, at, v, kind, life); }
    }
}

/// The cold shot's water met (states 1 and 2): the landing splash under it from 1 above (`0x326b28`,
/// crate::particles::rain::land: a ring on the water, or a splash on the ground), its z the grid's height there, the
/// burst unless the mover was turning, **3** for 80 ticks.
fn splash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = w.mobys[id].pos;
    let aux = w.weather.aux(ctx.level, &*ctx.game);
    let cam = ctx.host.camera_pos();
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::rain::land(sys, &mut ctx.game.rng, ctx.coll, ctx.scene, aux.map, aux.live, cam, [p[0], p[1], p[2] + 1.0]);
    }
    if let Some(h) = w.liquid.height(p) { w.mobys[id].pos[2] = h; }
    if w.mobys[id].pv_i(cold::TURNING) == 0 { burst(ctx, w.mobys[id].pos); }
}

/// `0x3e2c28` (module docs).
pub fn update_cold(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pvars(w, id);
    let fall = f32::from_bits(GRAVITY);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.mode |= 8;
        m.set_pv_i(cold::KEEP, 0);
        m.set_pv_i(cold::MODE, 0);
        ai::set_link(m, cold::MOVER, None);
        ctx.host.request_speech(0xdb8);
    }
    match w.mobys[id].state {
        1 => {
            let old = step(w, id, fall);
            let (pos, vel) = (w.mobys[id].pos, w.mobys[id].pv_v(0));
            for _ in 0..8 {
                let g = &mut ctx.game.rng;
                let j = f32::from_bits(0x3db8_51ec);
                let d = [g.randf_signed(0.0, j), g.randf_signed(0.0, j), g.randf_signed(0.0, j)];
                let life = g.rand_range(20, 40);
                if let Some(s) = ctx.parts.as_deref_mut() { crate::particles::frost::grain(s, &mut ctx.game.rng, add(d, pos), vel, life); }
            }
            let Some((met, _)) = line(w, ctx, id, old) else { return };
            let at = w.mobys[id].pos;
            w.mobys[id].set_pv_v(cold::AT, at);
            hit_cold(w, id, ctx, Some(met));
            match w.mobys[id].pv_i(cold::MODE) {
                2 => w.mobys[id].state = 2,
                m => {
                    if m != 0 { splash(w, id, ctx); }
                    w.mobys[id].state = 3;
                    w.mobys[id].set_pv_i(cold::TICKS, 80);
                }
            }
        }
        2 => {
            step(w, id, fall);
            hit_cold(w, id, ctx, None);
            if w.mobys[id].pv_i(cold::MODE) == 1 {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].state = 3;
                w.mobys[id].set_pv_i(cold::TICKS, 80);
                splash(w, id, ctx);
            }
        }
        3 => {
            let k = f32::from_bits(0x3f4c_cccd);
            {
                let m = &mut w.mobys[id];
                let v = m.pv_v(0);
                m.pos = add(m.pos, v);
                let mut v = scale(v, k);
                v[2] -= f32::from_bits(0x3b20_32fa);
                m.set_pv_v(0, v);
                m.rot[1] = add_rot(m.rot[1], m.pv_f(0x14));
                m.rot[2] = add_rot(m.rot[2], m.pv_f(0x18));
                m.set_pv_f(0x14, m.pv_f(0x14) * k);
                m.set_pv_f(0x18, m.pv_f(0x18) * k);
                m.alpha = if m.alpha < 3 { 0 } else { m.alpha - 2 };
            }
            let pos = w.mobys[id].pos;
            let probe = [pos[0], pos[1], pos[2] + 1.0];
            let top = w.liquid.surface(probe).unwrap_or(probe[2]);
            let g = &mut ctx.game.rng;
            let v = [g.randf(-0.5, 0.5), g.randf(-0.5, 0.5), g.randf(-0.5, 2.0)];
            let speed = g.randf(f32::from_bits(0x3d88_8889), f32::from_bits(0x3e08_8889)) * f32::from_bits(0x3e4c_cccd);
            let v = set_len(v, speed);
            let size = g.randf(f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3dcc_cccd)) * 210_000.0;
            if let Some(s) = ctx.parts.as_deref_mut() { crate::particles::water::bubble(s, &mut ctx.game.rng, size, top, pos, v); }
            let mut t = w.mobys[id].pv_i(cold::TICKS);
            if countdown(&mut t) { w.mobys[id].state = 4; }
            w.mobys[id].set_pv_i(cold::TICKS, t);
        }
        4 => {
            if w.mobys[id].pv_i(cold::KEEP) == 0 { w.delete(id); }
        }
        _ => {}
    }
}

/// `0x3e1c18` (module docs).
fn hit_hot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, met: Option<usize>) {
    let Some(t) = met else { return };
    if kind(w, t) != TARGET_KIND || w.mobys[t].pvars.is_empty() { return; }
    if w.mobys[t].pv_s(TURNING) != 0 {
        w.mobys[id].set_pv_i(hot::TURNING, 1);
        return;
    }
    if w.mobys[t].pv_b(FROZEN) != 0 { count(w, ctx, t, 2); }
    w.mobys[id].set_pv_i(hot::TURNING, 0);
}

/// `0x3e1d08`: a flame for the shot, its way out three `randf(±1)` at 0.1275, its turn 0.
fn flame(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.therm.ready();
    let Some(slot) = w.therm.flames.iter().position(Option::is_none) else { return };
    let g = &mut ctx.game.rng;
    let off = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
    w.therm.flames[slot] = Some(Flame { owner: id, off: set_len(off, 0.1275), pos: w.mobys[id].pos, turn: 0.0 });
    let n = w.mobys[id].pv_i(hot::FLAMES);
    w.mobys[id].set_pv_i(hot::FLAMES, n + 1);
}

/// `0x3e2590(shot, at, size)`: a puff for the shot at `at` (the shot's place; its w −1 would put it 0.3 behind the
/// shot, never the case here) jittered ±0.015 on each axis: its alpha 80..200, its colour `(c, c/2, c/4)` for `c`
/// 96..128, its blend by the shot's puffs made (`+0x50`: even 1, odd 2), its life 50..60, its spin `randf_signed(180,
/// 720)` degrees a second, a random angle, rising `randf(0.01, 0.015)` a tick.
fn puff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, size: f32) {
    w.therm.ready();
    let Some(slot) = w.therm.puffs.iter().position(Option::is_none) else { return };
    let g = &mut ctx.game.rng;
    let alpha = g.rand_range(80, 200);
    let c = g.rand_range(96, 128) as u32;
    let color = c + ((c << 7) & 0xff00) + ((c << 14) & 0xff_0000);
    let kind = w.mobys[id].pv_i(hot::MADE) % 2 + 1;
    let life = g.rand_range(50, 60);
    let spin = g.randf_signed(180.0, 720.0) * f32::from_bits(0x3998_825c);
    let angle = g.rand_angle();
    let j = f32::from_bits(0x3c75_c28f);
    let mut pos = w.mobys[id].pos;
    for k in &mut pos { *k += g.randf(-j, j); }
    let vel = [0.0, 0.0, g.randf(f32::from_bits(0x3c23_d70a), j)];
    w.therm.puffs[slot] = Some(Puff { owner: id, pos, vel, spin, angle, size, color, color2: color, alpha, alpha2: alpha, kind, life, duration: life });
    let m = &mut w.mobys[id];
    m.set_pv_i(hot::PUFFS, m.pv_i(hot::PUFFS) + 1);
    m.set_pv_i(hot::MADE, m.pv_i(hot::MADE) + 1);
}

/// `0x3e1dd8` + `0x3e27e8`: the shot's flames ride it (their way out through its rows, from its place) and turn 36°
/// (gone at 360°); its puffs age — the drawn alpha its alpha by the life left, the drawn colour half that share of its
/// colour (`0x2f0000`), the angle on by the spin, the size up `randf(0.001, 0.003)`, the place on by the velocity, the
/// velocity × 0.99, the life down (out: gone).
fn age(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.therm.ready();
    let (step, full) = (f32::from_bits(0x3f20_d97d), f32::from_bits(0x40c9_0fdb));
    let (rows, at) = (w.mobys[id].rows, w.mobys[id].pos);
    let mut gone = 0;
    for f in w.therm.flames.iter_mut() {
        let done = match f {
            Some(x) if x.owner == id => {
                let o = x.off;
                x.pos = std::array::from_fn(|k| o[0] * rows[0][k] + o[1] * rows[1][k] + o[2] * rows[2][k] + at[k]);
                x.turn += step;
                full <= x.turn
            }
            _ => false,
        };
        if done {
            *f = None;
            gone += 1;
        }
    }
    let n = w.mobys[id].pv_i(hot::FLAMES);
    w.mobys[id].set_pv_i(hot::FLAMES, n - gone);
    let mut gone = 0;
    for p in w.therm.puffs.iter_mut() {
        let done = match p {
            Some(x) if x.owner == id => {
                let t = x.life as f32 / x.duration as f32;
                x.alpha2 = (x.alpha as f32 * t) as i32;
                x.color2 = crate::hud::lerp_rgba(t * 0.5, 0, x.color);
                x.angle = add_rot(x.angle, x.spin);
                x.size += ctx.game.rng.randf(f32::from_bits(0x3a83_126f), f32::from_bits(0x3b44_9ba6));
                x.pos = add(x.pos, x.vel);
                x.vel = scale(x.vel, 0.99);
                countdown(&mut x.life)
            }
            _ => false,
        };
        if done {
            *p = None;
            gone += 1;
        }
    }
    let n = w.mobys[id].pv_i(hot::PUFFS);
    w.mobys[id].set_pv_i(hot::PUFFS, n - gone);
}

/// `0x3e16c0` (module docs).
pub fn update_hot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pvars(w, id);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.state = 1;
        m.set_pv_i(hot::FLAMES, 0);
        m.set_pv_i(hot::PUFFS, 0);
        m.set_pv_i(hot::MADE, 0);
        w.therm.ready();
        if !w.therm.cleared {
            w.therm.flames.iter_mut().for_each(|f| *f = None);
            w.therm.cleared = true;
        }
        ctx.host.request_speech(0xdb9);
    }
    match w.mobys[id].state {
        1 => {
            let old = step(w, id, f32::from_bits(GRAVITY));
            flame(w, id, ctx);
            let size = dist(old, w.mobys[id].pos) * 0.5;
            puff(w, id, ctx, size);
            if let Some((met, n)) = line(w, ctx, id, old) {
                let at = w.mobys[id].pos;
                {
                    let m = &mut w.mobys[id];
                    m.state = 2;
                    m.set_pv_i(hot::TICKS, 20);
                    m.set_pv_i(hot::LEVEL, 128);
                    m.mode |= 8;
                    m.set_pv_v(0, [0.0; 3]);
                }
                hit_hot(w, id, ctx, met);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_pv_v(hot::NORMAL, n);
                w.mobys[id].set_pv_v(hot::AT, at);
                if w.mobys[id].pv_i(hot::TURNING) == 0 {
                    for _ in 0..8 {
                        let g = &mut ctx.game.rng;
                        let yaw = g.rand_angle();
                        let pitch = g.randf(f32::from_bits(0x3ec9_0fdb), f32::from_bits(0x4049_0fdb));
                        let v = super::shot::polar(f32::from_bits(0x3e08_8889), yaw, pitch);
                        if let Some(s) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(s, &mut ctx.game.rng, -1.0, at, v, false); }
                    }
                }
            }
        }
        2 => {
            puff(w, id, ctx, 0.1);
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(0);
            m.pos = add(m.pos, v);
            v[2] -= f32::from_bits(0x3ac8_3fb8);
            m.set_pv_v(0, v);
            m.alpha = m.alpha.wrapping_sub(4);
            let mut t = m.pv_i(hot::TICKS);
            countdown(&mut t);
            m.set_pv_i(hot::TICKS, t);
            if 5 <= m.alpha { m.alpha -= 4; }
            if t == 0 && m.alpha < 4 { m.state = 3; }
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.alpha != 0 { m.alpha -= 1; }
            let l = m.pv_i(hot::LEVEL);
            if 5 <= l { m.set_pv_i(hot::LEVEL, l - 4) } else if 0 < l { m.set_pv_i(hot::LEVEL, l - 1) }
            if m.pv_i(hot::FLAMES) == 0 && m.pv_i(hot::PUFFS) == 0 && m.pv_i(hot::LEVEL) == 0 {
                m.state = 4;
                m.has_collision = false;
                m.mode = (m.mode | 0x41) & !mode::TARGET;
            }
        }
        4 => {
            let mut t = w.mobys[id].pv_i(hot::GONE);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(hot::GONE, t);
            if out {
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    age(w, id, ctx);
    // `0x2e7920(0x3e1ee0, shot)`: its draw ([`draw_hot`]).
    w.register_draw(id);
}

/// `0x3e1ee0`, the hot shot's draw: in states 2..3 its scorch where it hit (`0x3e2258`: FX 0x2e, normal blending,
/// `0x204080` at the level `+0x58`, lying on the face (its normal `+0x20`, 0.025 off it), size `0.75·(1 − (1 − t)²)`
/// for `t` = 1 − its ticks `+0x44` / 20); its flames (`0x3e2028`: each two additive FX 0x2e sprites, `0x103080` of
/// 0.125 and `0x7f7f7f` of 0.0625, 0.011 toward the camera, at the alpha `128·(cos(turn − π) + 1)/2`); itself (two
/// additive FX 0xb sprites at alphas `randi(0x20, 0x40)`: `0x7f7f7f` of 0.125 pushed 0.3 toward the camera and
/// `0x103080` of 0.225 pushed 0.31); its puffs (`0x3e2948` → `0x3e2af0`: an FX 0x2e sprite each, its drawn colour and
/// alpha, its blend, its size and angle). Not ported: the puffs' heat shimmer (`0x2ee160` / `0x2ee190` / `0x2ee188`:
/// a frame-distortion sprite of `2·size·a` with `a` = `(cos(angle) + 1)/2`, 0.05·cos(angle) off each axis).
pub fn draw_hot(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if (2..=3).contains(&m.state) {
        let f = m.pv_i(hot::TICKS) as f32 / 20.0;
        let e = 1.0 - f;
        let size = (1.0 - (1.0 - e) * (1.0 - e)) * 0.75;
        let n = m.pv_v(hot::NORMAL);
        let rgba = 0x0020_4080u32.wrapping_add((m.pv_i(hot::LEVEL) as u32) << 24);
        quad(out, m.pv_v(hot::AT), n, set_len(n, 0.025), 0.0, size, rgba, 0x2e, false);
    }
    for f in w.therm.flames.iter().flatten().filter(|f| f.owner == id) {
        let a = (((f.turn - std::f32::consts::PI).cos() + 1.0) * 0.5 * 128.0) as i32 as u32;
        sprite(out, cam, f.pos, 0x0010_3080u32.wrapping_add(a << 24), 0x2e, true, 0.125, 0.011);
        sprite(out, cam, f.pos, 0x007f_7f7fu32.wrapping_add(a << 24), 0x2e, true, 0.0625, 0.011);
    }
    let a = rng.rand_range(0x20, 0x40) as u32;
    sprite(out, cam, m.pos, 0x007f_7f7fu32.wrapping_add(a << 24), 0xb, true, 0.125, 0.3);
    let a = rng.rand_range(0x20, 0x40) as u32;
    sprite(out, cam, m.pos, 0x0010_3080u32.wrapping_add(a << 24), 0xb, true, 0.225, 0.31);
    for p in w.therm.puffs.iter().flatten().filter(|p| p.owner == id) {
        let d = sub(cam, p.pos);
        quad(out, p.pos, d, [0.0; 3], p.angle, p.size, p.color2.wrapping_add((p.alpha2 as u32) << 24), 0x2e, p.kind == 2);
    }
}

/// `0x3e23d8(pos, rgb, alpha, fx, blend, size, roll, push)` with no roll: [`quad`] facing the camera, `push` toward it.
#[allow(clippy::too_many_arguments)]
fn sprite(out: &mut Vec<crate::fx::FxQuad>, cam: V3, pos: V3, rgba: u32, fx: usize, additive: bool, size: f32, push: f32) {
    let d = sub(cam, pos);
    quad(out, pos, d, set_len(d, push), 0.0, size, rgba, fx, additive);
}

/// The quads of `0x3e23d8` / `0x3e2258` / `0x3e2af0`: the square (0, ±1, ±1) × `size` (`0x3e2168`, `0x3e21f0`) through
/// the rows of the angles (`0x2ef6d8`: `roll`, then pitch and yaw toward `toward`) at `pos + off` (`0x2eb998`).
#[allow(clippy::too_many_arguments)]
fn quad(out: &mut Vec<crate::fx::FxQuad>, pos: V3, toward: V3, off: V3, roll: f32, size: f32, rgba: u32, fx: usize, additive: bool) {
    let rows = crate::hero::physics::euler_rows([roll, -atan2(len2(toward), toward[2]), atan2(toward[0], toward[1])]);
    let at = add(pos, off);
    let corner = |y: f32, z: f32| add(at, add(scale(rows[1], y * size), scale(rows[2], z * size)));
    let corners = [corner(-1.0, -1.0), corner(-1.0, 1.0), corner(1.0, -1.0), corner(1.0, 1.0)];
    out.push(crate::fx::FxQuad { fx, additive, corners, st: [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]], rgba: [rgba; 4] });
}

/// `0x410b58` (module docs); `0x37dfc0`'s own tests: not in Ratchet's states 0x17..0x1b / 0x6f / 0x62 nor in a scene.
pub fn draw(w: &MobyWorld, id: usize, hero_state: i32, scene_mode: i32, frame: i32, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if !(2..=3).contains(&m.state) || m.pv_i(hand::AIMED) == 0 { return; }
    if matches!(hero_state, 0x17..=0x1b | 0x6f | 0x62) || scene_mode == crate::scene::SCENE { return; }
    super::gravity_bomb::marker_quads(m.pv_v(hand::END), m.pv_v(hand::NORMAL), super::gravity_bomb::MARKER_RGBA, frame, out);
}
