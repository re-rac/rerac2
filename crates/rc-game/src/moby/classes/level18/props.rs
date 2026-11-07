//! Damosel's props (L18 addresses; the engine functions by their L18 places with the L00 ones the port knows them by).
//!
//! * **0x1212** ([`update_1212`], `0x43fc50`, 12 placed; level 20's 11 at `0x447010`; the decomp's
//!   `func_L18_0043E760`): a bobbing glow with two arcs. Its pvars: `+0x140` its place at the start, `+0x150` /
//!   `+0x154` its bob's phase and last offset, `+0x158` / `+0x15c` its pulse and its facing (for its draws). **0**: the
//!   bob's phase random (`0x336af0` = L00 `0x31bb00`), its start kept, its arcs set up (`0x4402d8`), 1. **Drawn**: the
//!   pulse's phase `(frame mod 60)/60·2π − π`; its glow (`+0x78`) the pulse (`sin·½ + ½`, `0x341d10`) of 0x80202080 /
//!   0x20202040 and of 0x80400000 / 0x20000000 blended (`0x338d30` = L00 `0x31dc10`, crate::hud::lerp_rgba) by the
//!   square of how square its first row faces the camera (`0x1b9740`). **Each tick** it bobs 0.1 about its start's
//!   height at 300° a second (`0x33f1b0` = L00 `0x324430`, `$gp+0x2be8` / `+0x2bec`).
//! * **0x12a6** ([`update_12a6`], `0x4427e0`, 5 placed; the decomp's `func_L18_004412F0`): a breakable. **0**: 1.
//!   **1**: a hit with flags 0x4010000 (`0x338e08` = L00 `0x31dce8`) while `+0x68`[0] = 0 breaks it (2); its hit let
//!   go. **2**: every piece thrown (`0x346658` = L00 `0x32b580`, crate::moby::shards::throw_all: `$gp+0x2cf0` /
//!   `+0x2ce8` / `+0x2ce0`), the blast at it (`0x415e70` = L00 `0x3f8348`: 10 times its scale, sprites 5 / 5 / 5, 5 puffs,
//!   5 streaks, its flash ball), deleted (`0x32c688` = L00 `0x311688`).
//! * **0x11af** (4 placed), **0x11ae** (2), **0x11b3** (1), **0x11b4** (1) ([`update_11af`], `0x3fecc8`; the decomp's
//!   `func_L18_003FD858`): a slider that runs out along a direction and back. Its pvars: `+0x00..+0x10` four cuboids
//!   and `+0x10..+0x20` four groups (−1 none), `+0x20` / `+0x28` their flags (four halves each: set, the cuboid or
//!   group sends it out; clear, back), `+0x30` the direction, `+0x40` its start, `+0x50` the tick's step, `+0x60` /
//!   `+0x64` how far and how fast (a second), `+0x68` / `+0x6c` its ticks out and the run's, `+0x70` the moby whose
//!   place the cuboids test (−1 Ratchet), `+0x74` 1 always out, −1 never (cleared at the set-up).
//!   * **0**: its start kept, the run's ticks the distance over the speed (truncated), its step the distance over them
//!     along the direction; 1.
//!   * **1** (back): its step back each tick (the last onto its start). One frame in 16 its out test ([`out`],
//!     `0x3fef28`) → 2. Hooked (`0x34cd40` = L00 `0x32a838`): the hook cleared, 2.
//!   * **2** (out): its step on until the run's ticks. One frame in 16 its back test ([`back`], `0x3ff1d8`) → 1.
//!     Hooked: the hook cleared, 1.
//! * **0x0f06** ([`update_0f06`], `0x4310c0`, 4 placed; the decomp's `func_L18_0042FBC8`): a hopping nest of the
//!   breeding critters (crate::moby::classes::level18::breeder) that throws its brood and settles. Its pvars: `+0x00`
//!   its turns (a tick), `+0x10` / `+0x14` the cuboids that start its hopping and its throw, `+0x18` its brood's group,
//!   `+0x1c` its resting height, `+0x20` its speed up, `+0x24` / `+0x28` its wobble's period and size, `+0x2c` how
//!   many it throws.
//!   * **0**: its resting height kept; 1.
//!   * **1**: Ratchet in the first cuboid (`0x33c668` = L00 `0x321858`) with 4 or more of its brood hidden in the nest
//!     (`0x3f8a58`, crate::moby::classes::level18::breeder::brood_pick): 2.
//!   * **2**: each hop starts level at its resting height with up to 10 a second more up and new tilts (up to 90° a
//!     second), its class sound 0 (`0x37c8d8` = L00 `0x3605f0`); above its rest it rises by its speed under 30 a
//!     second², tilting one way going up and back coming down; below it, a new hop. Ratchet in the second cuboid with
//!     4 or more hidden: 3.
//!   * **3**: on its first tick level, it throws up to `+0x2c` of its brood (each the last hidden, while any are left;
//!     `0x3f8b60`, crate::moby::classes::level18::breeder::launch) from 0.4 above it, 10 a second up jittered by 5 a
//!     second (`0x33c5e8` = L00 `0x3217d8`), itself their parent (`+0xb8`); then a blast (size 1, 0x19 / 0x14 / 0x14
//!     sprites, 0x14 puffs, 0x2d streaks, the flash, the shake), its sound 0, 25..27 a second more up and on each axis
//!     no turn or a whole one over the flight (`0x4316e0` = L11 `0x43b5b8`: its ticks to land). It flies, turning; down
//!     at its rest, 4.
//!   * **4**: on its first tick level and still, its sound 0, its wobble 120 ticks and 15°; then it wobbles (x by the
//!     sine, y the cosine of its ticks over the period, `0x30a6e0` = L00 `0x2efef0` wrapped), period and size 0.99 of
//!     theirs each tick; under 0.1 at rest and 5.
//! * **0x12bf** ([`update_12bf`], `0x442928`, one placed; the decomp's `func_L18_00441438`): a breakable with health
//!   (`+0x20`, its health block). **0**: its sound none (`+0x100`), a target (mode 0x1000), the block's bytes `+0x48` /
//!   `+0x4a` 15 and 6; 1. **1**: on its first tick its sound stopped. A hit with flags 0x10000 (`0x338e08` = L00
//!   `0x31dce8`) from anything but a breeder 0x0b69 or a 0x0f9e takes its damage (`0x338e68` = L00 `0x31dd48`,
//!   crate::moby::ai::damage_of) off its health and is let go; at none: the general explosion at it (`0x33b658` = L00
//!   `0x320538`, crate::moby::blast::explode: 10 embers, 3 billows, 9 flares, flashes 4 / 2 past 9, size 0, light 15,
//!   the shake, debris, flags 0x810001), its class sound 0, all its pieces (mask 0x1fff, `0x341fb8` = L00 `0x327498`,
//!   crate::moby::shards::break_into: 3..6 / 3..6) thrown 0.1 a tick away from Ratchet across its frame (`0x341040` =
//!   L00 `0x326328`: its up's part dropped), its bolts (`0x335db0` = L00 `0x31adc0`, crate::moby::bolts::drop: 0, 3),
//!   deleted. With no hit the code calls an empty function (`0x427988`).
//! * **0x124a** ([`update_124a`], `0x441160`, one placed; the decomp's `func_US2_L18_00441160`): an orb Ratchet
//!   picks up. **0**: gone at once with the flag byte `0x1395b8 + 0x86` (crate::game::GameState::flags) or the item
//!   0x37 (`0x1a7b00`, crate::game::GameState::items) had; else it turns 5° a tick and gives off two pairs of twinkles
//!   (`0x355b90` = L00 `0x33a6a0`, crate::particles::twinkle: 3 ticks, 2..5 in size, 0x208020ff and 0x20ffffff, 0.05..0.1
//!   a tick a random way 20°..88° up, from up to 0.15 off its place); Ratchet within 1.5 across: the flag set, the moby
//!   `+0x00`'s timer byte 1 and it hidden (1), or with none deleted. **1**: once the moby `+0x00` is gone (or none), 40
//!   ticks on the message 0x1293 for 120 (`0x341f68` = L00 `0x327448`) and the help text 0xa0f (slot 0x66) while the
//!   flag bytes `+0x86` / `+0x89` leave one to go, else 0xa10 (slot 0x67) (`0x30fa60` = L00 `0x2f5270`); deleted.
//! * **0x0f54** ([`update_0f54`], `0x4369e8`, one placed; the decomp's `func_L18_004354F8`): a piece that swings
//!   into place (its pitch, `+0xf4`, from 4.276 to 0 over 180 ticks, `$gp+0x25e0..+0x25e8`) and fades in while either
//!   of its two mobys (`+0x00`, `+0x04`) is on (their pvar byte `+0x14`), solid once there, and back when either is off.
//!   Its pvars: `+0x08` / `+0x0c` two colours (for its particles), `+0x10` its turn a tick, `+0x14` its loop sound.
//!   * **0**: its place kept for its particles (`0x1b36a0`; `0x1b36b0` 0.2618), no collision, mode | 0x200, the level
//!     sound 16 looped on it (`0x37cad0` = L00 `0x3607e8`); 1.
//!   * **1**: on its first tick hidden (`+0x31` 0, mode 1) at 4.276, alpha 0x30, each of its mobys that is on told 25
//!     (its timer byte), its colours 0x407f6060 / 0x09ffffff; then either on: 2.
//!   * **2**: on its first tick shown, its turn the angle between its ends (crate::hero::physics::rot_dist) over 180;
//!     it turns, its alpha to 0x80 and its colours to 0x00ffffff / 0x02ffffff by its ticks over 180 (`0x338d30` = L00
//!     `0x31dc10`, crate::hud::lerp_rgba); past 180, 3.
//!   * **3**: on its first tick at 0, alpha 0x80, its colours the second pair, its collision on, each of its mobys that
//!     is off told 25, its sound stopped; then either off: 4.
//!   * **4**: on its first tick no collision, its turn back, the sound again; back the same way to 1.
//!
//! Not ported for 0x0f54: its particles (`0x436e98` each tick of 1, 2 and 4: the level particle `0x35b6b8`, which the
//! port lacks, on the place `0x1b36a0`).
//!
//! Not ported for 0x1212: its arcs (`0x4402d8` the set-up, `0x440430` each drawn tick: two ribbons from random points
//! of the table `0x2bd9b0`, their waves) and its three draws (`0x301f28(0x43ff80)`, `(0x43fe28)`, `(0x43fea0)`),
//! which are a render path of their own.

use crate::hero::physics::{add, add_rot, dot, set_len, sub, wrap_turn, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::{shards, Ctx, MobyWorld};

/// The level's mission `mission` done (`0x19b2e8 + level·0x10`, bit 7).
fn mission_done(ctx: &Ctx, mission: u8) -> bool {
    let g = &ctx.game;
    mission != 0xff && g.missions.get(g.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0)
}

pub const ARCS: i16 = 0x1212;
/// `$gp+0x2bd8..+0x2be4`: the pulses' colours (the first pair, the second pair).
const ARC_GLOW: [u32; 4] = [0x8020_2080, 0x2020_2040, 0x8040_0000, 0x2000_0000];
/// `$gp+0x2be8` / `+0x2bec`: the bob's height and speed (degrees a second).
const ARC_BOB: (f32, f32) = (0.1, 300.0);

/// 0x1212 (module docs).
pub fn update_1212(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let ph = ctx.game.rng.rand_angle();
        let m = &mut w.mobys[id];
        m.set_pv_f(0x150, ph);
        let p = m.pos;
        m.set_pv_v(0x140, p);
        m.set_state(1, -1);
    }
    if w.mobys[id].drawn != 0 {
        let phase = (ctx.game.frame % 60) as f32 * std::f32::consts::TAU / 60.0 - std::f32::consts::PI;
        let s = phase.sin() * 0.5 + 0.5;
        let a = crate::hud::lerp_rgba(s, ARC_GLOW[0], ARC_GLOW[1]);
        let b = crate::hud::lerp_rgba(s, ARC_GLOW[2], ARC_GLOW[3]);
        let cam = ctx.host.camera_pos();
        let m = &mut w.mobys[id];
        m.set_pv_f(0x158, phase.cos() * 0.5 + 0.5);
        let f = dot(set_len(sub(cam, m.pos), 1.0), m.rows[0]).abs();
        let f = f * f;
        m.set_pv_f(0x15c, f);
        m.glow = crate::hud::lerp_rgba(f, b, a);
    }
    let m = &mut w.mobys[id];
    let base = m.pv_f(0x148);
    crate::moby::classes::props::bob(m, ARC_BOB.0, ARC_BOB.1 * 0.000_290_888_24, base, 0x150, 0x154);
}

pub const PANEL: i16 = 0x12a6;
/// `$gp+0x2cf0` / `+0x2ce8` / `+0x2ce0`: the pieces' velocities (a second), spins and gravities.
const PANEL_H: [f32; 8] = [-3.0, -3.0, 1.0, 0.0, 3.0, 3.0, 7.0, 0.0];
const PANEL_SPIN: (f32, f32) = (90.0, 360.0);
const PANEL_GRAVITY: (f32, f32) = (10.0, 16.0);

/// 0x12a6 (module docs).
pub fn update_12a6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, PANEL_H, PANEL_SPIN, PANEL_GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            let (at, s) = (w.mobys[id].pos, w.mobys[id].scale);
            blast::burst(w, ctx, at, &Blast { size: s * 10.0, bursts: [5, 5, 5], puffs: 5, streaks: 5, light: true, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}

pub const SHELL: i16 = 0x12bf;

/// 0x12bf (module docs).
pub fn update_12bf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x100, -1);
            m.mode |= crate::moby::mode::TARGET;
            m.set_pv_b(0x48, 15);
            m.set_pv_b(0x4a, 6);
            m.set_state(1, -1);
        }
        1 => {
            if w.mobys[id].first() {
                let h = w.mobys[id].pv_i(0x100);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0x100, -1);
            }
            let Some(hit) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) else { return };
            let spared = hit.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x0b69 || s.o_class == 0x0f9e);
            if spared { return; }
            let dmg = crate::moby::ai::damage_of(w, id, Some(&hit), false);
            w.clear_hit(id);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
            if m.pv_f(0x20) <= 0.0 {
                let e = blast::Explosion {
                    flash: 4.0, flash2: 2.0, near: 9.0, size: 0.0, light: 15.0, embers: 10, billows: 3, flares: 9, sound: -1, shake: true,
                    debris: 1, lod: -1, flags: 0x81_0001, ..Default::default()
                };
                blast::explode(w, ctx, Some(id), None, &e);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                // `0x326328(m, out, d, 0, 0)`: `d` into its frame, its z there 0, back through its rows.
                let m = &w.mobys[id];
                let d = sub(m.pos, ctx.hero.pos);
                let r = m.rows;
                let (x, y) = (dot(r[0], d), dot(r[1], d));
                let v: V3 = std::array::from_fn(|k| x * r[0][k] + y * r[1][k]);
                let v = set_len(v, f32::from_bits(0x3dcc_ccce));
                shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1fff, v, 0xff);
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
            }
        }
        _ => {}
    }
}

pub const ORB: i16 = 0x124a;
/// `0x1395b8 + 0x86` / `+ 0x89` (crate::game::GameState::flags): the orb taken, and the other one.
const ORB_FLAG: usize = 0x86;
const ORB_OTHER: usize = 0x89;
/// `0x1a7b37`: the item 0x37 had.
const ORB_ITEM: usize = 0x37;

/// 0x124a (module docs).
pub fn update_124a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    fn flag(ctx: &Ctx, k: usize) -> bool { ctx.game.flags.get(k).is_some_and(|&b| b != 0) }
    let link = w.mobys[id].pv_i(0);
    if w.mobys[id].state == 0 {
        if flag(ctx, ORB_FLAG) || ctx.game.items.get(ORB_ITEM).is_some_and(|&b| b != 0) {
            w.delete(id);
            return;
        }
        let m = &mut w.mobys[id];
        m.rot[2] = add_rot(m.rot[2], 0.087_266_475);
        let pos = m.pos;
        for _ in 0..2 {
            let g = &mut ctx.game.rng;
            let size = g.randf(2.0, 5.0);
            let spin: i8 = if g.randi(2) == 0 { -1 } else { 1 };
            let yaw = g.rand_angle();
            let pitch = g.randf(0.349_065_9, 1.535_889_9);
            let vel = crate::moby::classes::shot::polar(g.randf(0.05, 0.1), yaw, pitch);
            let p = [pos[0] + g.randf_signed(0.0, 0.15), pos[1] + g.randf_signed(0.0, 0.15), pos[2] + g.randf_signed(0.0, 0.15)];
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::twinkle(sys, &mut ctx.game.rng, size * 0.1, size, 0.006, p, 3, 0x2080_20ff, 0, spin, vel, 0);
                crate::particles::twinkle(sys, &mut ctx.game.rng, size * 0.03, size * 0.3, 0.006, p, 3, 0x20ff_ffff, 1, -spin, vel, 0);
            }
        }
        if crate::hero::physics::dist2(pos, ctx.hero.pos) < 1.5 {
            if let Some(f) = ctx.game.flags.get_mut(ORB_FLAG) { *f = 1; }
            match usize::try_from(link).ok().filter(|&k| k < w.mobys.len()) {
                Some(k) => {
                    w.mobys[k].timer = 1;
                    let m = &mut w.mobys[id];
                    m.drawn = 0;
                    m.mode |= 1;
                    m.set_state(1, -1);
                }
                None => w.delete(id),
            }
        }
        return;
    }
    let gone = usize::try_from(link).ok().and_then(|k| w.mobys.get(k)).is_none_or(|o| o.state == 0xfe || o.state == 0xfd);
    if link == -1 || gone {
        let m = &mut w.mobys[id];
        let t = m.state_ticks as u16;
        m.state_ticks = t.wrapping_add(1) as i16;
        if 40 < t {
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1293, ticks: Some(120) });
            let mut n = if flag(ctx, ORB_FLAG) { 1 } else { 2 };
            if flag(ctx, ORB_OTHER) { n -= 1; }
            if 0 < n { ctx.game.help(0xa0f, 0x66); } else { ctx.game.help(0xa10, 0x67); }
            w.delete(id);
        }
    }
}

pub const SWING: i16 = 0x0f54;
/// `$gp+0x25e0..+0x25ed`: the swing's ticks, its ends, its alphas.
const SWING_TICKS: i32 = 180;
const SWING_ENDS: (f32, f32) = (4.276_06, 0.0);
const SWING_ALPHA: (u32, u32) = (0x30, 0x80);
/// `$gp+0x2618` / `0x1b160c`, `$gp+0x2620` / `0x1b1614`: its colours at its two ends.
const SWING_COLOURS: [[u32; 2]; 2] = [[0x407f_6060, 0x09ff_ffff], [0x00ff_ffff, 0x02ff_ffff]];

/// 0x0f54 (module docs).
pub fn update_0f54(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let on = |w: &MobyWorld, k: i32| usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_some_and(|m| m.pv_b(0x14) != 0);
    let tell = |w: &mut MobyWorld, k: i32, when_on: bool| {
        if let Some(m) = usize::try_from(k).ok().and_then(|k| w.mobys.get_mut(k)) {
            if (m.pv_b(0x14) != 0) == when_on { m.timer = 25; }
        }
    };
    let (a, b) = (w.mobys[id].pv_i(0), w.mobys[id].pv_i(4));
    let ticks = w.mobys[id].state_ticks as u16;
    let t = ticks as f32 / SWING_TICKS as f32;
    // The swing's look at share `t` from end `from` (0 or 1).
    let look = |m: &mut crate::moby::Moby, t: f32, from: usize| {
        let to = 1 - from;
        let al = [SWING_ALPHA.0, SWING_ALPHA.1];
        m.rot[1] = add_rot(m.rot[1], m.pv_f(0x10));
        m.alpha = crate::hud::lerp_rgba(t, al[from], al[to]) as u8;
        m.set_pv_i(8, crate::hud::lerp_rgba(t, SWING_COLOURS[from][0], SWING_COLOURS[to][0]) as i32);
        m.set_pv_i(0xc, crate::hud::lerp_rgba(t, SWING_COLOURS[from][1], SWING_COLOURS[to][1]) as i32);
    };
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_state(1, -1);
            m.has_collision = false;
            m.mode |= 0x200;
            let h = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 16, 4);
            w.mobys[id].set_pv_i(0x14, h);
        }
        1 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.drawn = 0;
                m.mode |= 1;
                m.rot[1] = SWING_ENDS.0;
                m.alpha = SWING_ALPHA.0 as u8;
                tell(&mut *w, b, true);
                tell(&mut *w, a, true);
                let m = &mut w.mobys[id];
                m.set_pv_i(8, SWING_COLOURS[0][0] as i32);
                m.set_pv_i(0xc, SWING_COLOURS[0][1] as i32);
            } else if on(&*w, a) || on(&*w, b) {
                w.mobys[id].set_state(2, -1);
            }
        }
        2 | 4 => {
            let back = w.mobys[id].state == 4;
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                let step = crate::hero::physics::rot_dist(SWING_ENDS.1, SWING_ENDS.0) / SWING_TICKS as f32;
                if back {
                    m.has_collision = false;
                    m.set_pv_f(0x10, -step);
                    let h = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 16, 4);
                    w.mobys[id].set_pv_i(0x14, h);
                } else {
                    m.drawn = 1;
                    m.mode &= !1;
                    m.set_pv_f(0x10, step);
                }
            }
            look(&mut w.mobys[id], t, usize::from(back));
            if SWING_TICKS < ticks as i32 { w.mobys[id].set_state(if back { 1 } else { 3 }, -1); }
        }
        3 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.rot[1] = SWING_ENDS.1;
                m.alpha = SWING_ALPHA.1 as u8;
                m.set_pv_i(8, SWING_COLOURS[1][0] as i32);
                m.set_pv_i(0xc, SWING_COLOURS[1][1] as i32);
                m.has_collision = solid;
                tell(&mut *w, b, false);
                tell(&mut *w, a, false);
                let h = w.mobys[id].pv_i(0x14);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0x14, -1);
            } else if !on(&*w, a) || !on(&*w, b) {
                w.mobys[id].set_state(4, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

pub const SLIDER_A: i16 = 0x11af;
pub const SLIDER_B: i16 = 0x11ae;
pub const SLIDER_C: i16 = 0x11b3;
pub const SLIDER_D: i16 = 0x11b4;

/// The point the slider's cuboids test: the moby `+0x70`'s place, else Ratchet's.
fn slider_point(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    usize::try_from(w.mobys[id].pv_i(0x70)).ok().and_then(|k| w.mobys.get(k)).map_or(ctx.hero.pos, |m| m.pos)
}

/// The group's live members (not 0xfe / 0xfd) all hidden (mode 1).
fn group_hidden(w: &MobyWorld, group: i32) -> bool {
    let Some(members) = usize::try_from(group).ok().filter(|&g| g < 0xff).and_then(|g| w.groups.get(g)) else { return true };
    members.iter().all(|&e| w.mobys.get(e).is_none_or(|m| m.state == 0xfe || m.state == 0xfd || m.mode & 1 != 0))
}

/// `0x3fef28` / `0x3ff1d8(m, p)`: the slider's out (`out`) or back test — `+0x74`, its mission done (out: yes, back:
/// no), one of the cuboids of that flag holding `p`, else every group of that flag (at least one) all hidden, which
/// then clears the flagged groups (those whose flag is set, as both tests do).
fn test(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: V3, out: bool) -> bool {
    let m = &w.mobys[id];
    match m.pv_i(0x74) {
        -1 => return !out,
        1 => return out,
        _ => {}
    }
    if mission_done(ctx, m.mission) { return out; }
    for i in 0..4 {
        let c = m.pv_i(4 * i);
        if c != -1 && (m.pv_s(0x20 + 2 * i) != 0) == out && ctx.level.in_cuboid(c, p) { return true; }
    }
    let (mut any, mut ok) = (false, true);
    for i in 0..4 {
        if !ok { break; }
        let g = m.pv_i(0x10 + 4 * i);
        if g == -1 || (m.pv_s(0x28 + 2 * i) != 0) != out { continue; }
        any = true;
        ok = group_hidden(w, g);
    }
    if ok && any {
        let m = &mut w.mobys[id];
        for i in 0..4 {
            if m.pv_s(0x28 + 2 * i) != 0 { m.set_pv_i(0x10 + 4 * i, -1); }
        }
    }
    ok && any
}

/// 0x11af / 0x11ae / 0x11b3 / 0x11b4 (module docs).
pub fn update_11af(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let slice = ctx.game.frame & 0xf == 0;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0x40, p);
            let d = m.pv_f(0x60);
            let n = (d / (m.pv_f(0x64) * 0.016_666_668)) as i32;
            m.set_pv_i(0x6c, n);
            m.set_pv_v(0x50, set_len(m.pv_v(0x30), d / n as f32));
            m.set_state(1, -1);
            m.set_pv_i(0x74, 0);
            m.set_pv_i(0x68, 0);
        }
        1 => {
            let m = &mut w.mobys[id];
            let k = m.pv_i(0x68);
            if 2 <= k {
                m.pos = sub(m.pos, m.pv_v(0x50));
                m.set_pv_i(0x68, k - 1);
            } else if 0 < k {
                m.pos = m.pv_v(0x40);
                m.set_pv_i(0x68, k - 1);
            }
            if slice {
                let p = slider_point(w, id, ctx);
                if test(w, id, ctx, p, true) { w.mobys[id].set_state(2, -1); }
            }
            let m = &mut w.mobys[id];
            if m.once & 4 != 0 {
                m.once &= !4;
                m.set_state(2, -1);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let k = m.pv_i(0x68);
            if k < m.pv_i(0x6c) {
                m.pos = add(m.pos, m.pv_v(0x50));
                m.set_pv_i(0x68, k + 1);
            }
            if slice {
                let p = slider_point(w, id, ctx);
                if test(w, id, ctx, p, false) { w.mobys[id].set_state(1, -1); }
            }
            let m = &mut w.mobys[id];
            if m.once & 4 != 0 {
                m.once &= !4;
                m.set_state(1, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

pub const NEST: i16 = 0x0f06;
/// `$gp+0x24e0..+0x24e8`: the hop's most speed up (a second) and its most tilts (degrees a second).
const NEST_HOP: f32 = 10.0;
const NEST_TILT: (f32, f32) = (90.0, 90.0);
/// `$gp+0x24ec` / `+0x24f0`: the throw's speed up and jitter (a second); `+0x2510` its place above the nest.
const NEST_THROW: (f32, f32) = (10.0, 5.0);
const NEST_THROW_AT: V3 = [0.0, 0.0, 0.4];
/// `$gp+0x24f4` / `+0x24f8`: the big hop's speed (a second); `+0x24fc` its turns' choice (0 or 1 whole turns).
const NEST_LEAP: (f32, f32) = (25.0, 27.0);
const NEST_TURNS: i32 = 2;
/// `$gp+0x2500` / `+0x2504` / `+0x2508`: the wobble's decay, period (ticks) and size (radians).
const NEST_DECAY: f32 = 0.99;
const NEST_PERIOD: i32 = 120;
const NEST_WOBBLE: f32 = 0.261_799;
/// `$gp+0x250c`: the gravity (a second²).
const NEST_GRAVITY: f32 = 30.0;
const NEST_BROOD: i32 = 4;

/// Its rise under the gravity while above its rest: true, else (down) false.
fn nest_fly(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.pos[2] < m.pv_f(0x1c) { return false; }
    m.pos[2] += m.pv_f(0x20);
    m.set_pv_f(0x20, m.pv_f(0x20) - NEST_GRAVITY * 0.000_277_777_78);
    true
}

/// 0x0f06 (module docs).
pub fn update_0f06(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::classes::level18::breeder;
    let hero = ctx.hero.pos;
    let group = w.mobys[id].pv_i(0x18);
    let brood = |w: &MobyWorld| breeder::brood_pick(w, group).1;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x1c, m.pos[2]);
            m.set_state(1, -1);
        }
        1 => {
            let c = w.mobys[id].pv_i(0x10);
            if c != -1 && ctx.level.in_cuboid(c, hero) && NEST_BROOD <= brood(&*w) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            if w.mobys[id].first() {
                let up = ctx.game.rng.randf(0.0, NEST_HOP * 0.016_666_668);
                let tx = ctx.game.rng.randf(0.0, NEST_TILT.0 * 0.000_290_888_25);
                let ty = ctx.game.rng.randf(0.0, NEST_TILT.1 * 0.000_290_888_25);
                let m = &mut w.mobys[id];
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.pos[2] = m.pv_f(0x1c);
                m.set_pv_f(0x20, m.pv_f(0x20) + up);
                m.set_pv_f(0, tx);
                m.set_pv_f(4, ty);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
            if nest_fly(w, id) {
                let m = &mut w.mobys[id];
                let (tx, ty) = if 0.0 < m.pv_f(0x20) { (m.pv_f(0), m.pv_f(4)) } else { (-m.pv_f(0), -m.pv_f(4)) };
                m.rot[0] = add_rot(tx, m.rot[0]);
                m.rot[1] = add_rot(ty, m.rot[1]);
            } else {
                w.mobys[id].set_state(2, -1);
            }
            let c = w.mobys[id].pv_i(0x14);
            if c != -1 && ctx.level.in_cuboid(c, hero) && NEST_BROOD <= brood(&*w) { w.mobys[id].set_state(3, -1); }
        }
        3 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.pos[2] = m.pv_f(0x1c);
                let n = m.pv_i(0x2c);
                let mut i = 0;
                let mut left = 50;
                while i < n && 0 < left {
                    let (k, c) = breeder::brood_pick(w, group);
                    left = c;
                    if let Some(k) = k {
                        let mut v = [0.0, 0.0, NEST_THROW.0 * 0.016_666_668];
                        crate::moby::classes::level08::tabora_ai::jitter(&mut ctx.game.rng, &mut v, NEST_THROW.1 * 0.016_666_668);
                        let at = add(w.mobys[id].pos, NEST_THROW_AT);
                        breeder::launch(w, k, at, v);
                        w.mobys[k].parent = Some(id);
                    }
                    i += 1;
                }
                if i != 0 {
                    let at = w.mobys[id].pos;
                    blast::burst(w, ctx, at, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                    let up = ctx.game.rng.randf(NEST_LEAP.0, NEST_LEAP.1) * 0.016_666_668;
                    let m = &mut w.mobys[id];
                    let v = m.pv_f(0x20) + up;
                    m.set_pv_f(0x20, v);
                    // `0x4316e0(z, rest, v, g)`: (v + √(v² + 2g(z − rest))) / g, truncated.
                    let g = NEST_GRAVITY * 0.000_277_777_78;
                    let t = ((v + (v * v + (g + g) * (m.pos[2] - m.pv_f(0x1c))).sqrt()) / g) as i32 as f32;
                    for k in 0..3 {
                        let a = ctx.game.rng.randi(NEST_TURNS) as f32 * std::f32::consts::TAU / t;
                        w.mobys[id].set_pv_f(4 * k, a);
                    }
                }
            }
            if nest_fly(w, id) {
                let m = &mut w.mobys[id];
                for k in 0..3 { m.rot[k] = add_rot(m.pv_f(4 * k), m.rot[k]); }
            } else {
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_v(0, [0.0; 3]);
                m.set_pv_f(0xc, 0.0);
                m.rot = [0.0; 3];
                m.pos[2] = m.pv_f(0x1c);
                m.set_pv_f(0x24, NEST_PERIOD as f32);
                m.set_pv_f(0x28, NEST_WOBBLE);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
            let m = &mut w.mobys[id];
            let amp = m.pv_f(0x28);
            if 0.1 <= amp {
                let a = wrap_turn(m.state_ticks as u16 as f32 * std::f32::consts::TAU / m.pv_f(0x24));
                m.rot[0] = amp * a.sin();
                m.rot[1] = amp * a.cos();
                m.set_pv_f(0x28, amp * NEST_DECAY);
                m.set_pv_f(0x24, m.pv_f(0x24) * NEST_DECAY);
            } else {
                m.rot = [0.0; 3];
                m.set_state(5, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
}
