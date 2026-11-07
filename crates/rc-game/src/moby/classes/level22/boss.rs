//! 0x0e0b, Dobbo's orbit's opponent (L22 `0x39eb38`, asm only in the decomp, its L23 copy `0x3a3170` unplaced; its
//! frame `0x3a2a00`, the decomp's `func_US2_L22_003A2A00`; one placed): a giant robot that fights Giant Clank on the
//! planetoid with its fists, the guns of its rider and the specials, walking the level's paths
//! (crate::moby::classes::level22::planet) and taking the pickups (crate::moby::classes::level22::pickups).
//!
//! **Its parts** (made at its set-up, `0x3168d0`): the rider 0x0e0a (`+0x440`, its rows the identity; the guns at its
//! joints 3 and 4 and the fists' hits are its), the 0x0e0c (`+0x444`) and the 0x0f8f (`+0x448`). Each tick (`0x3a58c0`,
//! [`place_parts`]) the rider sits at the opponent's joint 1 (crate::moby::classes::enemies::place_part; only its place
//! while its sequence is 1..3 or its turn `+0x460` runs), playing at the opponent's speed; the 0x0e0c and, unless the
//! opponent is hidden, the 0x0f8f at the rider's joint 5 (the 0x0f8f (−2.55, 0, 2.6) off in its frame). In the
//! rider's sequences 1..3 it turns to Ratchet about its own z (`0x3a60d8`, [`turn_rider`]: `0x31f230` with the
//! opponent's turn limits, `+0x124` / `+0x128`·2 / `+0x12c`, its speed `+0x458`), then for 30 ticks back toward the
//! opponent's joint 1's x (`+0x124`·2 / `+0x128`·4), stood up on the sphere each tick. Their light, glow and
//! distances follow the opponent's (the 0x0e0c's and the 0x0f8f's draw distance 0x50), their scales the class's ×1.45
//! (`$gp+0x35c`; the 0x0f8f's ×1.2, `$gp+0x2f0`); the rider's shadow within 24 of the camera (`0x322f30`, 0x16).
//!
//! **The wait** ([`update_0e0b`], before its set-up, `$gp+0x2a4` 0): Ratchet's presses dropped (`0x138324`); in state 0
//! the level's scene trigger 0x0f6e (`$gp+0x2a0`) set going (update distance 0xff, its scene list 0, no level, `[0xa]`
//! 0, hooked: `0x32f090(t, 1)`), state 0x27 (sub 11), hidden. In 0x27 while a 0x0f6e is still about it waits; then
//! state 0 (shown) and the set-up follows.
//!
//! **The set-up** ([`set_up`], `0x3a2a00` in state 0): its health from `+0x326` (3600) — × 0.9 past 90000 ticks of
//! fighting (`$gp+0x2a8`, `$gp+0x2ac`), × 0.6 past 126000 (`$gp+0x2b0`) — kept as its most `0x1b2a28`, `+0x20` / `+0x24`;
//! the intro's count `$gp+0x2c4` 1; the skill flag `+0x4ec` 1; the fight's words cleared, the skip table its five
//! (crate::moby::classes::level22::planet); of the level's mobys the body 0x027d (Giant Clank is put there with the
//! angles (0.00161, 0.29459, 1.57105)), itself (`$gp+0x2d4`), the director 0x133a hidden (mode | 0x41), the 0x0f6e
//! set as at the wait; Giant Clank's health `+0x324` (1200); the planetoid's radius its distance from the centre;
//! mode | 0x4100; its reaction block (`+0x60`: hurt state 0x25 sub 8, the shares `+0x7c..+0x9c`, the damage scales
//! `+0xb8` 9/most and `+0xbc` 900/most), its health block (`+0x20`: the wait `+0x26` 60), its mover (`+0x110`: radius
//! 2.5, height 0.75, steps 2.75, the walk `+0x130` / `+0x134` 1/900, most 0.2, the turn left π/4, no ledge test);
//! `+0x380` `0x320f00(30)`; the paths linked, the pickups set up; its parts made; `+0x384` 30.
//!
//! **Its frame** ([`frame`], `0x3a2a00` each tick): its parts' looks; the down at it (`+0x200`) and its rows stood up on
//! it; the damage (crate::moby::ai::take_damage, the reaction block `+0x60`); in the intro (`$gp+0x2c4` counted on;
//! Giant Clank held in a landing lag of 10, in its first 30 ticks his state idle and his gun put away; itself in state
//! 1 sub 1) ✕ or △ (`0x138324 & 0x50`) starts the fight: state 5, a landing lag of 20. Out of it, Giant Clank out of
//! health puts it in state 2 (sub 1, `+0x384` 30). Its scale ×1.45, its shadow within 24 of the camera.
//!
//! **Each tick** ([`update_0e0b`], `0x39ec90` on): the hit-landed timer `0x1b2a30` counted down (while it runs its hit
//! record is dropped); its hit (`0x3231c0(m, 0x30000, 1)`, kept); its frame; past 200 intro ticks only the parts. The
//! fight's ticks counted out of the intro; set up, the damages kept (`0x1b2a14..`: `+0x330` 4, `+0x334` 6, `+0x338` 48,
//! `+0x33c` 0.35) and the skip table's scan. `+0x41c` its health between ⅙ and ⅔; a hit by anything but Giant Clank
//! clears the skill flag. Facing him (`+0x3d0`, within 45°; in subs 4 / 5 out of the sequences 0xb / 0xc the face
//! turn's `0x330660`) and within 60° (`+0x3d4`); hit while not facing him, the path walk ends. Kept: his and its place
//! on the surface (`+0x3f0` / `+0x400`), their heights (`+0x410` / `+0x414`), his distance (`+0x3dc`) and across
//! (`+0x3e0`), the walk test to him within 50 (`+0x3d8`). Its death asked (`+0x344`): Giant Clank invincible for
//! 32000 ticks, idle, his gun put away; itself 0x26 sub 10. The fight's phase (`+0x41e`, `0x1b2a08`): 1 below ⅔ of
//! its most, 2 below ⅓ (`+0x464` 1200, once). The rider pushes what is within 5.4 of it (1, flags 0x4000000).
//!
//! **Its subs** (`+0x95`): **0** near him (35, within 60°, clear): the chase 0x1d with gun or specials left, else at
//! random the run-off 0x12 or the closing walk 0x1c (0x12 from phase 1 on); in mid health the run-off at ⅙ or less, the
//! chase with more than half its gun or specials, else the run-off within 55 or the walk; phase 2's timer: the
//! run-off 0x13. **3**: at ⅙ or less the run-off. **4**: phase 2 or ⅙: the run-off; in the air over 20 ticks in 0x1e:
//! the run-off. **5**: its exhausts' particles (`0x3a7008`, not ported). **6**: → 0x1e. **8** the knock-back: the
//! flight's hit ahead (5 at 4 up and 4 along its way, 100, 0x90000); the 0x0f8f's idle shuffle; its first tick the
//! wait 30, the flight's speed ×0.6 at its death, 50 damage the level sound 0x13, from Giant Clank's jump a blast at
//! the rider and the flight 0.95 away from him, its sequence again (0x3a5a20, below), the 0x0f8f's 6, both at 0.9,
//! its special given up in 4 / 8; the flight (crate::moby::ai::flight): dead → `+0x344`; over → 5 (sub 0), `+0x37c`
//! 240. **9**: after 46 ticks, near him or he is in the air over it ([`near`], `0x3a75d8`), out of phase 2's timer:
//! the punch 0x1f. **10** the death ([`dying`]).
//!
//! **Its states** (`+0x20`): **1** the intro: facing him, held. **2** / **3** / **4** Giant Clank down: facing him,
//! its idle with a fidget every 27..33 ticks; 3 after 143 ticks and 4 after 1201: the run-off. **5**: after the
//! `+0x380` wait (counted) the walk to him 0x7 (from 20 across, or in a phase) or 0x6. **6** / **7**: to him on the
//! paths; within 20 (33) across the attack choice 0x1e. **0xf**: to him, blocked 3 ahead: the punch (sequence 9).
//! **0x12** the run-off: the point by him ([`planet::beside_hero`]), → 0x13. **0x13**: to it on the paths, the
//! rider's guns firing at him in their sequence 3 at frames 1 / 4 (crate::moby::classes::level22::shot; in the gun's
//! reach of 70 with charge, else its shots at their end); within 8 → 0x14; past 30 ticks, the turn left within
//! `+0x140` and him behind it: the hop 0x21. **0x14** there: after 30 ticks to the nearest pickup it needs within 30
//! (0x15, [`nearest_pickup`]), else a pickup's worth of its own ([`heal`], `0x3a73c0`) and the next choice. **0x15** to
//! the pickup, → 0x16 (idle 30 ticks) → the run-off or 0x14, or 5. **0x1c** / **0x1d** the closing walk / chase:
//! within 20 (29) across, clear: 0x1e. **0x1e** the attack choice: near (20): the punch, or 1 in 5 with gun charge the
//! hop with the special; within 33 the hop with the guns or the special by their counts, or the shooting 0x20; else
//! the run-off. **0x1f** the punches (sequences 9 / 10 / 13): the rider's joint 3 / 4 at frames 7..12 (4, 10, 0x10007),
//! the double at frames 17..20 between its joints 3 and 4 (4, 100, push 14.5, 0x10007; within 6 of Giant Clank's
//! chest the hit-landed timer 20); after the double, again (`0x1b2a30` out, or 3 in 4 with its health at most 7× his)
//! or the run-off, else the taunts 3 / 4; its walk turn limits (115, 17.5, 1440, 720); then to him. **0x20** the
//! shooting: the guns at him (the 0x0f8f's poses) for 120..240 ticks, sequences 0 / 0xb / 0xc by his distance, its
//! goal 20 off him on the paths. **0x21** the hop: the nearest path point 8 above → 0x22, the special mode `+0x3a6`.
//! **0x22**: up to it (`0x324250`: ⅓ most), → 0x23 (360..600 ticks). **0x23** on the path at 8: the guns (`+0x3a6` 1)
//! or the special (`+0x3a6` 0, `0x3ba568` 3 up and 1 ahead; its seq 0xe) or a fixed stance (2); it keeps between 32
//! and 38 of him along the path, the next slot at an end ([`planet::next_path`] mode 3 / 2), bobbing 1.7 sin(t),
//! t += 0.11; out of time, health or charge → 0x24. **0x24**: down to the surface → 5.
//!
//! **After the states**: its shadow mover 0x0d55 (`+0x388`, `0x3f9a40` / its place `0x3f9ae0`) kept at it; the
//! frame times `+0x38c` / `+0x44c` (the rider's); the parts placed, the pickups' tick, its flash `+0x50`, its ticks
//! `+0x96`, `+0x37c` / `+0x464` / `0x1b2a10` counted down; on a skipped slot in the walk's state 3 its punches stop
//! (`+0x43a` / `+0x34c`); the 0x0f8f's poses; in the sequences 0xb / 0xc it is put on the ground 12 down.
//!
//! **The death** ([`dying`], sub 10): the shadow mover hidden; the first tick: the skill point 8 (`0x1a7a70`) when
//! only Giant Clank hit it, a 60-frame fade to black; at 26 ticks the script camera at the view, its targets 0.66·55
//! back and 0.66·45 up from it on its way from him; then each tick the camera's aim at it and the way cleared (a 50
//! hit along the line from the camera to the rider, and the sphere 10 at the camera); one black frame each tick to
//! 30; at 30 the blast: its bolts, a smoke burst, five pieces of the rider and two of its own, two blasts, hidden with
//! its parts, the 0x0f8f dropped from 40 above (−0.8 a tick, then a blast and hidden), sparks every 8 and the class
//! sound 13 every 16 ticks; past 360 ticks all gone and the level left for Dobbo (`0x314780`: level 9).
//!
//! Not ported: the voices (`0x3a7660`), the camera record's reset (`0x42c170(+0x46c)`), the clock's seed, the
//! exhausts (`0x3a7008`) and the sparks (`0x3a7a70`) with their random draws, Ratchet's voice 0x2b at the double's
//! hit, the hook pair `0x312fe0`, the pieces' own update (`0x3a7c88`: the shards' here), the `+0x98` swap around the
//! fists' hits, `0x1395e6`; the dead hit from `$gp+0x2d8` (never set).

use super::{pickups, planet, shot, special};
use crate::hero::physics::{add, add_rot, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::ai;
use crate::moby::frames::{self, upright};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x0e0b;
/// Its parts (`+0x440`, `+0x444`, `+0x448`) and the shadow mover (`+0x388`).
pub const RIDER: i16 = 0x0e0a;
pub const PART_B: i16 = 0x0e0c;
pub const PART_C: i16 = 0x0f8f;
pub const SHADOW: i16 = 0x0d55;
/// The body Giant Clank is put at, and the scene trigger it waits for.
const BODY: i16 = 0x027d;
const TRIGGER: i16 = 0x0f6e;

/// `$gp+0x35c`: its scale over the class's; `$gp+0x2f0`: the 0x0f8f's.
const SCALE: f32 = f32::from_bits(0x3fb9_999a);
const SCALE_C: f32 = 1.2;
/// `$gp+0x3ac`: its near distance.
const NEAR: f32 = 20.0;
/// `$gp+0x2b4`: the death camera's scale.
const CAM: f32 = f32::from_bits(0x3f28_f5c3);

fn pv_grow(w: &mut MobyWorld, id: usize) { if w.mobys[id].pvars.len() < 0x4f0 { w.mobys[id].pvars.resize(0x4f0, 0); } }

/// A part (`off`) still there.
fn part(w: &MobyWorld, id: usize, off: usize, class: i16) -> Option<usize> {
    ai::link(&w.mobys[id], off).filter(|&p| w.mobys.get(p).is_some_and(|m| m.o_class == class && m.state < 0xfd))
}
fn rider(w: &MobyWorld, id: usize) -> Option<usize> { part(w, id, 0x440, RIDER) }

fn seq(w: &MobyWorld, ctx: &Ctx, id: usize) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_b) }
/// `+0x60 & 2`: the sequence ended this tick.
fn ended(w: &MobyWorld, ctx: &Ctx, id: usize) -> bool { ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0) }
fn ftime(w: &MobyWorld, ctx: &Ctx, id: Option<usize>) -> f32 { id.map_or(0.0, |i| ctx.host.frame_time(&w.mobys[i])) }
fn speed(w: &MobyWorld, ctx: &Ctx, id: usize) -> f32 { ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed) }

/// `0x32a7f0`: the first time since the last sub-state change.
fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn hp(w: &MobyWorld, id: usize) -> f32 { w.mobys[id].pv_f(0x20) }
fn most(w: &MobyWorld) -> i32 { w.level22.most }
/// The opponent's record (`0x1b2a34` → `0x1b1890`).
fn rec(w: &MobyWorld) -> [i32; 4] { w.level22.boss_counts }
fn state(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }

/// `0x3313f8(m)`: held where it is (the frames' move to its own place).
fn hold(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = w.mobys[id].pos;
    frames::move_to(w, id, ctx, p);
}

/// `0x3a5a20(speed, m, seq, frame, blend, flags)` (the decomp's `func_L22_003A45F0`): flag 1 (out of `+0x34e`) keeps
/// its sequence in `+0x3ec`, sets its walk limits (the chase's 64 / 2.5 for sequence 11 at 1.5 a tick, `+0x138` ½;
/// else 14 / 2.5 at 1.3, ⅓; turns 1440 / 720; then `+0x128` ×2 and `+0x134` ×8) and starts the sequence; out of the
/// punches' hold (`+0x34c`) its speed (out of 11 / 12) and the rider's follow it, and flag 2 starts the sequence on the
/// rider.
#[allow(clippy::too_many_arguments)]
fn anim(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, s: u8, frame: u8, blend: u8, flags: u32, sp: f32) {
    if flags & 1 != 0 && w.mobys[id].pv_s(0x34e) == 0 {
        let cur = seq(w, ctx, id);
        let m = &mut w.mobys[id];
        m.set_pv_i(0x3ec, cur as i32);
        let (a, b) = if s == 11 { (64.0, 2.5) } else { (14.0, 2.5) };
        ai::senses(m, a * 0.000_277_777_8, b * 0.016_666_668, 1440.0 * 4.848_137_5e-6, 720.0 * 0.000_290_888_25);
        let (v, most) = if s == 11 { (1.5, 0.5) } else { (1.3, f32::from_bits(0x3eaa_aaab)) };
        ctx.host.set_anim_speed(&w.mobys[id], v);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x138, most);
        m.set_pv_f(0x128, m.pv_f(0x128) * 2.0);
        m.set_pv_f(0x134, m.pv_f(0x134) * 8.0);
        if cur != s { ctx.host.set_anim(&w.mobys[id], s, frame, blend); }
    }
    if w.mobys[id].pv_s(0x34c) != 0 { return; }
    let cur = seq(w, ctx, id);
    if !(11..=12).contains(&cur) { ctx.host.set_anim_speed(&w.mobys[id], sp); }
    let f = speed(w, ctx, id);
    let Some(r) = rider(w, id) else { return };
    ctx.host.set_anim_speed(&w.mobys[r], f);
    if flags & 2 != 0 { ctx.host.set_anim(&w.mobys[r], s, frame, blend); }
}

fn set_part_anim(w: &MobyWorld, ctx: &mut Ctx, id: usize, off: usize, class: i16, s: u8, frame: u8, blend: u8) {
    if let Some(p) = part(w, id, off, class) { ctx.host.set_anim(&w.mobys[p], s, frame, blend); }
}
fn part_c(w: &MobyWorld, ctx: &mut Ctx, id: usize, s: u8, blend: u8) { set_part_anim(w, ctx, id, 0x448, PART_C, s, 0, blend); }

/// `0x3a5c18(m, flags, target, mode, player)`: with gun charge left the rider fires from its joint 3 (flag 1) and 4
/// (2) at Giant Clank, strong with its record's timer (crate::moby::classes::level22::shot); one charge off.
fn shoot(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, flags: u32) {
    if w.level22.boss_counts[2] == 0 { return; }
    let Some(r) = rider(w, id) else { return };
    let strong = w.level22.boss_counts[0] != 0;
    for (bit, joint) in [(1, 3), (2, 4)] {
        if flags & bit == 0 { continue; }
        let Some(at) = ctx.host.joint(&w.mobys[r], joint) else { continue };
        shot::fire(w, ctx, r, at, ctx.hero_moby, false, strong);
    }
    w.level22.boss_counts[2] -= 1;
}

/// `0x3a75d8(m)`: near him (within 20), or he is in the air (11 ticks) within 20 across and 6 in height.
fn near(w: &MobyWorld, ctx: &Ctx, id: usize) -> bool {
    let m = &w.mobys[id];
    if m.pv_f(0x3dc) < NEAR { return true; }
    let t = (m.pv_f(0x414) - m.pv_f(0x410)).abs();
    if ctx.hero.air_ticks < 0xb { return false; }
    m.pv_f(0x3e0) < NEAR && t < 6.0
}

/// `0x3a73c0(m)`: a pickup's worth of its own, from a random kind on (health below ⅓ of its most; half its gun's or
/// specials' most) — health left out at ⅙ or less.
fn heal(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let mut i = ctx.game.rng.randi(3);
    let n = most(w);
    if hp(w, id) <= (n / 6) as f32 { i = 0; }
    let first = i;
    loop {
        let m = &w.mobys[id];
        match i {
            0 => {
                if hp(w, id) < (n / 3) as f32 {
                    let h = hp(w, id) + m.pv_s(0x322) as f32;
                    w.mobys[id].set_pv_f(0x20, h);
                    return;
                }
            }
            1 | 2 => {
                let (k, off) = if i == 1 { (2, 0x32a) } else { (3, 0x32e) };
                let top = m.pv_s(off) as i32;
                let c = &mut w.level22.boss_counts[k];
                if *c < top {
                    *c = (*c + top / 2).min(top);
                    return;
                }
            }
            _ => {}
        }
        i = (i + 1) % 3;
        if i == first { return; }
    }
}

/// `0x3a5558(m, out)`: the nearest pickup within 30 (on the ground: under 3 above the surface) of the kinds it needs —
/// health below ⅚ of its most, the gun at 2 or less, the specials out.
fn nearest_pickup(w: &MobyWorld, ctx: &Ctx, id: usize) -> Option<V3> {
    let n = most(w);
    let rec = rec(w);
    let mut flags = 0;
    if rec[2] <= 2 { flags |= 2; }
    if rec[3] <= 0 { flags |= 2; }
    if hp(w, id) < (n - n / 6) as f32 { flags |= 1; }
    let mut best = 30.0;
    let mut out = None;
    let pos = w.mobys[id].pos;
    for kind in 0..3 {
        if flags & (1 << kind) == 0 { continue; }
        for k in (0..50).rev() {
            let Some(p) = w.level22.pickups.slots[kind][k] else { continue };
            let Some(pm) = w.mobys.get(p) else { continue };
            let d = dist(pm.pos, pos);
            if d < best && ctx.hero.gravity.height(pm.pos) - w.level22.radius < 3.0 {
                out = Some(pm.pos);
                best = d;
            }
        }
    }
    out
}

/// `0x324250(target, step, accel, max, &v, &vel)` (L00 `0x31ed78`): `v` toward `target`, the velocity built by `step`
/// to at most `√(2·accel·d)` (and `max`), braked by `accel` to land on it; the step taken.
fn ease(target: f32, step: f32, accel: f32, max: f32, v: &mut f32, vel: &mut f32) -> f32 {
    let toward = |x: &mut f32, t: f32, s: f32| { *x += (t - *x).clamp(-s, s); };
    let d = target - *v;
    if *vel == 0.0 && d == 0.0 { return 0.0; }
    if 0.0 <= *vel * d {
        let stopping = *vel * *vel / accel * 0.5;
        if d.abs() < stopping {
            if stopping < d.abs() + vel.abs() { toward(vel, 0.0, accel) } else { toward(vel, 0.0, accel * 1.1) }
        } else {
            let want = (2.0 * accel * d).abs().sqrt().min(max);
            toward(vel, if d < 0.0 { -want } else { want }, step);
        }
        if vel.abs() < d.abs() {
            *v += *vel;
            *vel
        } else {
            *v = target;
            d
        }
    } else {
        *vel += if 0.0 <= *vel { -accel } else { accel };
        *v += *vel;
        *vel
    }
}

/// 0x0e0b (`0x39eb38`, module docs).
pub fn update_0e0b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pv_grow(w, id);
    if !w.level22.started {
        ctx.hero.items.pressed = 0;
        if w.level22.trigger.is_none() && w.mobys[id].state == 0 {
            set_trigger(w);
            if let Some(t) = w.level22.trigger {
                w.mobys[t].set_pv_b(8, 0);
                w.mobys[t].once |= 4;
            }
            state(w, id, 0x27, 0xb);
            w.mobys[id].mode |= 0x41;
            return;
        }
        if w.mobys[id].state == 0x27 {
            if w.mobys.iter().any(|m| m.o_class == TRIGGER && m.state < 0xfd) {
                ctx.hero.items.pressed = 0;
                return;
            }
            w.level22.trigger = None;
            w.level22.started = true;
            state(w, id, 0, -1);
            w.mobys[id].mode &= 0xffbe;
            return;
        }
    }
    countdown(&mut w.level22.immune);
    if w.level22.immune != 0 { w.clear_hit(id); }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x30000 != 0);
    frame(w, id, ctx);
    if 200 < w.level22.intro {
        place_parts(w, id, ctx);
        return;
    }
    if w.level22.intro == 0 { ctx.game.orbit_ticks += 1; }
    if w.mobys[id].state != 0 {
        let m = &w.mobys[id];
        w.level22.damage = [m.pv_f(0x330), m.pv_f(0x334), m.pv_f(0x338), m.pv_f(0x33c)];
        planet::scan(w, ctx, id);
    }
    w.level22.flag = false;
    let n = most(w);
    let h = hp(w, id) as i32;
    let mid = h < (n << 1) / 3 && n / 6 < h;
    w.mobys[id].set_pv_s(0x41c, mid as i16);
    if hit.is_some_and(|h| h.tmpl.source != ctx.hero_moby) { w.mobys[id].set_pv_i(0x4ec, 0); }
    let s = seq(w, ctx, id);
    let ss = w.mobys[id].sub_state;
    if (4..=5).contains(&ss) && !(11..=12).contains(&s) {
        let f = frames::face_point(w, id, ctx.hero.pos);
        w.mobys[id].set_pv_i(0x3d0, f as i32);
    } else {
        let m = &mut w.mobys[id];
        let a = planet::turn_to(&m.rows, sub(ctx.hero.pos, m.pos));
        m.set_pv_i(0x3d4, (a <= f32::from_bits(0x3f86_0a92)) as i32);
        m.set_pv_i(0x3d0, (a <= std::f32::consts::FRAC_PI_4) as i32);
    }
    if hit.is_some() && w.mobys[id].pv_i(0x3d0) == 0 { w.mobys[id].set_pv_s(0x350, 0); }
    let r = w.level22.radius;
    let (pos, hero) = (w.mobys[id].pos, ctx.hero.pos);
    let hero_moby_pos = ctx.hero_moby.and_then(|k| w.mobys.get(k)).map_or(hero, |m| m.pos);
    let (hf, of) = (planet::surface(ctx, r, hero, 0.0), planet::surface(ctx, r, pos, 0.0));
    let (oh, hh) = (ctx.hero.gravity.height(pos), ctx.hero.gravity.height(hero_moby_pos));
    let clear = dist(hero, pos) < 50.0 && planet::clear_walk(w, ctx, id, pos, hero);
    let m = &mut w.mobys[id];
    m.set_pv_v(0x3f0, hf);
    m.set_pv_v(0x400, of);
    m.set_pv_f(0x410, oh);
    m.set_pv_f(0x414, hh);
    m.set_pv_f(0x3dc, dist(hero, pos));
    m.set_pv_f(0x3e0, dist(hf, of));
    m.set_pv_i(0x3d8, clear as i32);
    if m.pv_i(0x344) != 0 {
        ctx.hero.damage.invincible = 32000;
        state(w, id, 0x26, 10);
        put_gun_away(ctx);
        w.mobys[id].set_pv_i(0x344, 0);
    }
    phase(w, id);
    if let Some(rd) = rider(w, id) {
        let c = w.mobys[rd].pos;
        ai::attack_sphere(w, rd, ctx, f32::from_bits(0x40ac_cccd), 1.0, 1.0, c, 0x400_0000, 0, 1, 0);
    }
    if !by_sub(w, id, ctx, hit) { return; }
    by_state(w, id, ctx);
    after(w, id, ctx);
}

/// The scene trigger set going (the wait and the set-up: update distance 0xff, `[0xc]` 0xff, `[8]` / `[0xa]` 0).
fn set_trigger(w: &mut MobyWorld) {
    for k in 0..w.mobys.len() {
        let m = &mut w.mobys[k];
        if m.o_class != TRIGGER || 0xfd <= m.state { continue; }
        m.pvars.resize(m.pvars.len().max(0x10), 0);
        m.update_dist = 0xff;
        m.set_pv_b(0xc, 0xff);
        m.set_pv_b(8, 0);
        m.set_pv_b(0xa, 0);
        w.level22.trigger = Some(k);
    }
}

/// Giant Clank idle with his gun put away (`0x18c0b4` 0, `0x317480(him, 0x18b690)`, `0x18b69c` 0, `0x2c9b20(0, 1)`).
fn put_gun_away(ctx: &mut Ctx) {
    let g = &mut ctx.hero.giant;
    g.gun = None;
    g.gun_on = false;
    ctx.hero.request_state(0, true);
}

/// The fight's phase (module docs).
fn phase(w: &mut MobyWorld, id: usize) {
    let n = most(w);
    let h = hp(w, id);
    let m = &mut w.mobys[id];
    if m.pv_s(0x41e) != 0 || ((n << 1) / 3) as f32 <= h || m.pv_s(0x468) != 0 {
        if m.pv_s(0x41e) == 1 && h < (n / 3) as f32 && m.pv_s(0x46a) == 0 {
            m.set_pv_s(0x41e, 2);
            m.set_pv_i(0x464, 0x4b0);
            m.set_pv_s(0x46a, 1);
            w.level22.phase = 2;
            w.level22.flag = true;
        }
    } else {
        m.set_pv_s(0x41e, 1);
        m.set_pv_s(0x468, 1);
        w.level22.phase = 1;
        w.level22.flag = true;
    }
}

/// The sub-state's part (module docs); false: the tick ends there.
fn by_sub(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, hit: Option<crate::moby::hits::HitRecord>) -> bool {
    let n = most(w);
    let rec = rec(w);
    let h = hp(w, id);
    let m = &w.mobys[id];
    let (p41e, p464, p418, p41c) = (m.pv_s(0x41e), m.pv_i(0x464), m.pv_i(0x418), m.pv_s(0x41c));
    match m.sub_state {
        0 => {
            if !(m.pv_f(0x3dc) < 35.0 && m.pv_i(0x3d4) != 0 && m.pv_i(0x3d8) != 0) { return true; }
            if p418 != 0 && m.state != 0xf { state(w, id, 6, 4); return true; }
            if p464 != 0 && p41e == 2 { state(w, id, 0x13, 9); return true; }
            if p41c == 0 {
                if rec[3] == 0 && rec[2] == 0 {
                    if ctx.game.rng.randi(2) == 0 || p41e != 0 { state(w, id, 0x12, 9) } else { state(w, id, 0x1c, 3) }
                } else {
                    state(w, id, 0x1d, 3);
                }
            } else if h <= (n / 6) as f32 {
                state(w, id, 0x12, 9);
            } else if ((m.pv_s(0x32a) / 2) as i32) < rec[3] || ((m.pv_s(0x32e) / 2) as i32) < rec[2] {
                state(w, id, 0x1d, 3);
            } else if ctx.game.rng.randi(2) != 0 || (rec[3] == 0 && rec[2] == 0) {
                if ctx.game.rng.randi(2) != 0 && p41e == 0 {
                    state(w, id, 0x1c, 3);
                } else if w.mobys[id].pv_f(0x3dc) < 55.0 {
                    state(w, id, 0x12, 9);
                }
            } else {
                state(w, id, 0x1d, 3);
            }
        }
        3 => {
            if p418 != 0 { state(w, id, 6, 4) } else if h <= (n / 6) as f32 { state(w, id, 0x12, 9) }
        }
        4 => {
            if p418 != 0 {
                state(w, id, 6, 4);
            } else if (p464 != 0 && p41e == 2) || h <= (n / 6) as f32 {
                state(w, id, 0x12, 9);
            } else if 0x15 <= ctx.hero.air_ticks && m.state == 0x1e {
                state(w, id, 0x12, 9);
            }
        }
        6 => {
            if p418 == 0 { state(w, id, 0x1e, 4); }
        }
        8 => knock(w, id, ctx, hit),
        9 => {
            if 0x2e <= m.state_ticks as u16 && near(w, ctx, id) && (p41e != 2 || p464 == 0) {
                anim(w, ctx, id, 0xd, 0, 4, 3, 1.0);
                state(w, id, 0x1f, 4);
            }
        }
        10 => return dying(w, id, ctx),
        _ => {}
    }
    true
}

/// Sub 8, the knock-back (module docs).
fn knock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, hit: Option<crate::moby::hits::HitRecord>) {
    let r = w.level22.radius;
    let v = w.mobys[id].pv_v(0xe0);
    if 0.0 < len(v) {
        if w.mobys[id].pv_f(0x410) < r {
            let p = planet::surface(ctx, r, w.mobys[id].pos, 0.0);
            w.mobys[id].pos = p;
        }
        let m = &w.mobys[id];
        let c = add(add(crate::gravity::to_world([0.0, 0.0, m.pv_f(0x94)], &m.rows), m.pos), set_len(v, 4.0));
        ai::attack_sphere(w, id, ctx, 5.0, 100.0, 0.0, c, 0x9_0000, 0, 1, 0);
    }
    if !first_sub(w, id) {
        if let Some(c) = part(w, id, 0x448, PART_C) {
            if ended(w, ctx, c) && ctx.game.rng.randi(3) == 0 { ctx.host.set_anim(&w.mobys[c], 7, 0, 4); }
        }
    } else {
        w.mobys[id].set_pv_s(0x26, 0x1e);
        if let Some(hr) = hit {
            let dmg = hr.tmpl.damage;
            if 0.0 < len(v) && hp(w, id) - dmg <= 0.0 { w.mobys[id].set_pv_v(0xe0, set_len(v, 0.6)); }
            if dmg == 50.0 { ctx.host.play_level_sound_after(None, 0x13, 0); }
            if ctx.hero_view.seq_b == 4 && hr.tmpl.source == ctx.hero_moby {
                let m = &w.mobys[id];
                let way = set_len(sub(m.pv_v(0x400), m.pv_v(0x3f0)), f32::from_bits(0x3f73_3333));
                w.mobys[id].set_pv_v(0xe0, way);
                if let Some(rd) = rider(w, id) {
                    let at = w.mobys[rd].pos;
                    crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 1.2, bursts: [10, 10, 10], puffs: 0x14, streaks: 0x32, light: false, shake: true });
                }
            }
        }
        let s = seq(w, ctx, id);
        anim(w, ctx, id, s, 0, 4, 2, 1.0);
        part_c(w, ctx, id, 6, 4);
        if let Some(rd) = rider(w, id) { ctx.host.set_anim_speed(&w.mobys[rd], 0.9); }
        ctx.host.set_anim_speed(&w.mobys[id], 0.9);
        let s = seq(w, ctx, id);
        if s == 4 || s == 8 { special::end_for(w, ctx, id); }
    }
    if w.mobys[id].pv_s(0x43a) == 0 { w.mobys[id].set_pv_s(0x350, 0); }
    w.mobys[id].set_pv_f(0x3e4, 0.0);
    w.mobys[id].set_pv_f(0x3e8, 0.0);
    let f = ai::flight(w, id, ctx);
    if f & 1 != 0 {
        w.mobys[id].set_pv_i(0x344, 1);
    } else if f & 2 != 0 {
        w.mobys[id].set_pv_i(0x37c, 0xf0);
        state(w, id, 5, 0);
        anim(w, ctx, id, 0, 0, 4, 3, 1.0);
    }
}

/// Sub 10, the death (module docs); false: the tick ends.
fn dying(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    ctx.hero.land_lag = 10;
    if let Some(s) = part(w, id, 0x388, SHADOW) { w.mobys[s].mode |= 0x41; }
    let t = w.mobys[id].state_ticks as u16 as i32;
    let half = 30;
    if w.mobys[id].first() {
        if w.mobys[id].pv_i(0x4ec) != 0 { ctx.game.award_skill_point(crate::skill_points::point::GIANT_ORBIT); }
        ctx.game.blocking_fade(60);
    } else if half < t {
        if ai::reached(w, id, ctx, 27.0) && w.mobys[id].mode & 1 == 0 && half - 4 < t {
            blow_up(w, id, ctx);
        } else {
            let t = w.mobys[id].state_ticks as u16;
            if 0x168 < t {
                for off in [0x388, 0x440, 0x444, 0x448] {
                    if let Some(p) = ai::link(&w.mobys[id], off).filter(|&p| p < w.mobys.len()) { w.delete(p); }
                    ai::set_link(&mut w.mobys[id], off, None);
                }
                ctx.game.go_to_level(9, true);
                w.delete(id);
                return false;
            }
            let shown = w.mobys[id].mode & 1 == 0;
            let c = part(w, id, 0x448, PART_C);
            if 0x104 < t && !shown && c.is_some_and(|c| w.mobys[c].mode & 1 == 0) {
                drop_hat(w, id, ctx, c.unwrap());
            } else {
                if t & 7 == 0 && shown {
                    // A blast at a random joint of the rider or its own.
                    let (on, j) = if ctx.game.rng.randi(2) == 0 { (rider(w, id), ctx.game.rng.randi(6)) } else { (Some(id), ctx.game.rng.randi(4)) };
                    let at = on.and_then(|o| ctx.host.joint(&w.mobys[o], j as usize));
                    let s = ctx.game.rng.randf(1.0, f32::from_bits(0x3ff3_3333));
                    if let Some(at) = at {
                        crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: s, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: false, shake: true });
                    }
                }
                if t & 0xf == 0 && shown { ctx.host.play_sound(&w.mobys[id], 13, 0); }
            }
        }
    } else if t == half - 4 {
        ctx.game.blocking_fade(1);
        let p = planet::surface(ctx, w.level22.radius, w.mobys[id].pos, 0.0);
        w.mobys[id].pos = p;
        death_camera(w, id, ctx, true);
        anim(w, ctx, id, 0xe, 0, 4, 3, f32::from_bits(0x3f19_999a));
        w.mobys[id].set_pv_f(0x138, f32::from_bits(0x3c23_d70a));
    } else if half - 4 < t {
        death_camera(w, id, ctx, false);
        ctx.game.blocking_fade(1);
        clear_view(w, id, ctx);
    } else {
        ctx.game.blocking_fade(1);
    }
    w.mobys[id].set_pv_v(0xe0, [0.0; 3]);
    hold(w, id, ctx);
    true
}

/// The death's script camera: at the view, its targets back from it on its way from him (`first`), or aimed at it.
fn death_camera(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, first: bool) {
    let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    let euler = crate::moby::attach::angles_of(&rows);
    ctx.game.cameras.camera_script(cam, euler, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
    let m = &w.mobys[id];
    let pos = m.pos;
    let r = w.level22.radius;
    if first {
        let a = sub(m.pv_v(0x3f0), pos);
        let eye = planet::surface(ctx, r, add(set_len(a, -(CAM * 55.0)), pos), CAM * 45.0);
        let a = unit(a);
        let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 1.0, 0.0], |h| h.rows[1]);
        let look = [a, up, cross(a, up)];
        ctx.game.cameras.camera_script_targets(Some(eye), Some(crate::moby::attach::angles_of(&look)));
    } else {
        let a = unit(sub(pos, cam));
        let up = unit(m.rows[2]);
        let c = unit(cross(a, up));
        ctx.game.cameras.camera_script_targets(None, Some(crate::moby::attach::angles_of(&[a, c, up])));
    }
}

/// The way from the camera cleared (module docs: a 50 hit from Giant Clank along the lines to the rider, the sphere 10).
fn clear_view(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cam = ctx.host.camera_pos();
    let Some(hm) = ctx.hero_moby.filter(|&h| h < w.mobys.len()) else { return };
    let mut t = crate::moby::hits::HitTemplate::new(50.0, Some(hm), 0xb_0000, [0.0, 0.0, 0.0, f32::from_bits(0x45af_df66)]);
    t.kind = (w.mobys[hm].o_class as u16 as u32) << 16 | 0x107;
    ai::attack_sphere(w, id, ctx, 10.0, 100.0, 0.0, cam, 0x9_0000, 0, 1, 0);
    let Some(rd) = rider(w, id) else { return };
    let to = w.mobys[rd].pos;
    let hero_up = w.mobys[hm].rows[2];
    let back = add(scale(hero_up, 4.0), ctx.hero.pos);
    let s = CAM * 6.875;
    let (x, z) = (ctx.host.camera_rows()[1], ctx.host.camera_rows()[2]);
    let froms = [cam, back, add(scale(x, s), cam), add(scale(x, -s), cam), add(scale(z, s), cam), add(scale(z, -s), cam)];
    for from in froms {
        if let Some(h) = w.coll_line(ctx, from, to, 0x10, None) {
            if let Some(o) = h.moby { w.deliver_hit_any(o, &t, [h.point[0], h.point[1], h.point[2], 1.0]); }
        }
    }
}

/// The blast at 30 ticks of the death (module docs).
fn blow_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, 25.0, 100.0, 0.0, pos, 0x9_0000, 0, 1, 0);
    let hero = ctx.hero.pos;
    frames::move_to(w, id, ctx, hero);
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    crate::moby::blast::smoke_burst(w, ctx, 2.0, 7.0, Some(id), pos, -1);
    let rd = rider(w, id);
    for (src, n) in [(rd, 5), (Some(id), 2)] {
        let Some(src) = src else { continue };
        for i in 0..n {
            let mut v = ctx.game.rng.rand_vec(22.0 * 0.016_666_668, 29.0 * 0.016_666_668);
            v[2] = v[2].abs() * -0.5;
            let v = crate::gravity::to_world(v, &w.mobys[src].rows);
            let s = &w.mobys[src];
            let (class, at, rot) = (s.o_class, s.pos, s.rot);
            crate::moby::shards::spawn_ball(w, ctx, class, Some(src), at, rot, 30.0, 15.0, i as u8, v, 0xff);
        }
    }
    if let Some(rd) = rd {
        let at = w.mobys[rd].pos;
        crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 7.5, bursts: [0x73, 0x50, 0x46], puffs: 0x4e, streaks: 0x4b, light: true, shake: false });
    }
    crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 3.0, bursts: [0x2d, 0x28, 0x28], puffs: 0x28, streaks: 0x3a, light: true, shake: true });
    w.mobys[id].mode |= 0x41;
    for off in [(0x388, SHADOW), (0x440, RIDER), (0x444, PART_B)] {
        if let Some(p) = part(w, id, off.0, off.1) { w.mobys[p].mode |= 0x41; }
    }
    if let Some(c) = part(w, id, 0x448, PART_C) {
        w.mobys[c].pos = pos;
        ctx.host.set_anim(&w.mobys[c], 6, 0, 4);
        ctx.host.set_anim_speed(&w.mobys[c], f32::from_bits(0x404c_cccd));
        w.level22.hat = 40.0;
        let p = planet::surface(ctx, w.level22.radius, pos, 40.0);
        w.mobys[c].pos = p;
    }
}

/// The 0x0f8f dropped after the blast (module docs).
fn drop_hat(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: usize) {
    w.level22.hat -= 0.8;
    let r = w.level22.radius;
    if 0.0 < w.level22.hat {
        let p = planet::surface(ctx, r, w.mobys[c].pos, w.level22.hat);
        w.mobys[c].pos = p;
        ctx.host.set_anim_speed(&w.mobys[c], f32::from_bits(0x404c_cccd));
        ctx.host.advance(&w.mobys[c]);
    } else {
        let m = &w.mobys[c];
        let p = planet::surface(ctx, r, add(scale(m.rows[0], f32::from_bits(0x400c_cccd)), m.pos), 0.5);
        crate::moby::blast::burst(w, ctx, p, &crate::moby::blast::Blast { size: 0.9, bursts: [10, 10, 10], puffs: 10, streaks: 0x14, light: false, shake: true });
        ctx.host.play_sound(&w.mobys[id], 12, 0);
        w.mobys[c].mode |= 0x41;
    }
}

/// The state's part (module docs).
fn by_state(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = most(w);
    let rec = rec(w);
    let m = &w.mobys[id];
    let st = m.state;
    let t = m.state_ticks as u16;
    match st {
        1 => {
            if 199 < w.level22.intro { return; }
            frames::face_point(w, id, ctx.hero.pos);
            if w.mobys[id].first() {
                anim(w, ctx, id, 0, 0, 0, 3, 1.0);
                part_c(w, ctx, id, 7, 0);
            }
            if w.mobys[id].pv_s(0x1d0) == 0 { hold(w, id, ctx); }
        }
        2..=4 => {
            if st != 4 { frames::face_point(w, id, ctx.hero.pos); }
            if w.mobys[id].first() {
                anim(w, ctx, id, 0, 0, 4, 3, 1.0);
                part_c(w, ctx, id, 4, 4);
            } else if st == 2 {
                fidget(w, id, ctx);
            }
            let limit = match st { 3 => 0x8f, 4 => 0x4b1, _ => u16::MAX };
            if st != 2 {
                if t < limit { fidget(w, id, ctx) } else { state(w, id, 0x12, 9) }
            }
            let m = &mut w.mobys[id];
            let mut k = m.pv_s(0x384) as i32;
            countdown(&mut k);
            m.set_pv_s(0x384, k as i16);
            if w.mobys[id].pv_s(0x1d0) == 0 { hold(w, id, ctx); }
        }
        5 => {
            let m = &w.mobys[id];
            if m.pv_i(0x464) != 0 && m.pv_s(0x41e) == 2 { return run_off(w, id, ctx, m.prev_state); }
            if m.pv_s(0x380) == 0 {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x3ac, 0);
                let s = if NEAR <= m.pv_f(0x3e0) || m.pv_s(0x41e) != 0 { 7 } else { 6 };
                state(w, id, s, 0);
            } else {
                let m = &mut w.mobys[id];
                let mut k = m.pv_s(0x380) as i32;
                countdown(&mut k);
                m.set_pv_s(0x380, k as i16);
            }
            w.mobys[id].set_pv_i(0x190, 0);
            anim(w, ctx, id, 0xc, 0, 4, 3, 1.0);
        }
        6 => {
            let m = &w.mobys[id];
            let near = m.pv_f(0x3e0) < NEAR;
            if m.pv_i(0x418) == 0 {
                if near { state(w, id, 0x1e, 4) } else { planet::follow(w, ctx, id) }
            } else if near {
                state(w, id, 0xf, 6);
            } else {
                planet::follow(w, ctx, id);
            }
        }
        7 => {
            if 33.0 <= w.mobys[id].pv_f(0x3e0) { planet::follow(w, ctx, id) } else { state(w, id, 0x1e, 4) }
        }
        0xf => {
            let hero = ctx.hero.pos;
            frames::move_to(w, id, ctx, hero);
            let m = &w.mobys[id];
            let ahead = add(scale(m.rows[0], 3.0), m.pos);
            if !planet::clear_walk(w, ctx, id, m.pos, ahead) {
                anim(w, ctx, id, 9, 0, 4, 3, 1.0);
                state(w, id, 0x1f, 6);
            }
        }
        0x12 => {
            let p = w.mobys[id].prev_state;
            run_off(w, id, ctx, p);
        }
        0x13 => {
            guns_or_seq(w, id, ctx, true);
            let m = &w.mobys[id];
            if dist(m.pos, planet::goal(w, ctx, id)) < 8.0 {
                state(w, id, 0x14, 9);
                return;
            }
            planet::follow(w, ctx, id);
            let m = &w.mobys[id];
            if 0x1e < m.state_ticks as u16 && m.pv_f(0x43c) <= m.pv_f(0x140) {
                let v = unit(sub(m.pos, ctx.hero.pos));
                if dot(m.rows[0], v) < -0.5 && len(v) < 10.0 {
                    state(w, id, 0x21, 9);
                    let s = if rec[3] == 0 && rec[2] == 0 { 2 } else { 3 };
                    let m = &mut w.mobys[id];
                    m.set_pv_s(0x3a6, s);
                    m.set_pv_s(0x350, 0);
                }
            }
        }
        0x14 => {
            if w.mobys[id].first() && seq(w, ctx, id) == 4 {
                anim(w, ctx, id, 0, 0, 0x14, 3, 1.0);
                return;
            }
            if w.mobys[id].state_ticks as u16 <= 0x1e { return; }
            if let Some(p) = nearest_pickup(w, ctx, id) {
                let m = &mut w.mobys[id];
                m.set_pv_v(0x3b0, p);
                m.set_pv_i(0x3ac, 1);
                state(w, id, 0x15, 9);
                anim(w, ctx, id, 0xb, 0, 4, 3, 1.0);
                return;
            }
            heal(w, ctx, id);
            choose_after_rest(w, id, ctx);
        }
        0x15 => {
            if dist(w.mobys[id].pos, planet::goal(w, ctx, id)) < 8.0 {
                state(w, id, 0x16, 9);
                anim(w, ctx, id, 0, 0, 0x14, 3, 1.0);
            } else {
                planet::follow(w, ctx, id);
            }
        }
        0x16 => {
            if t < 0x1f { return; }
            let h = hp(w, id);
            if h < ((n << 1) / 3) as f32 && ((n / 6) as f32) < h {
                if w.mobys[id].pv_f(0x3dc) < 55.0 { state(w, id, 0x12, 9) } else { state(w, id, 0x14, 9) }
            } else {
                state(w, id, 5, 0);
            }
        }
        0x1c | 0x1d => {
            if w.mobys[id].first() {
                if st == 0x1c { anim(w, ctx, id, 0xb, 0, 6, 3, 1.0) } else { anim(w, ctx, id, 0xc, 0, 0x14, 3, 1.0) }
            }
            let m = &w.mobys[id];
            let reach = if st == 0x1c { NEAR } else { 29.0 };
            if m.pv_f(0x3e0) < reach && m.pv_i(0x3d8) != 0 { state(w, id, 0x1e, 4); }
            w.mobys[id].set_pv_i(0x3ac, 0);
            planet::follow(w, ctx, id);
        }
        0x1e => choose_attack(w, id, ctx),
        0x1f => punches(w, id, ctx),
        0x20 => shooting(w, id, ctx),
        0x21 => {
            if w.mobys[id].first() {
                anim(w, ctx, id, 5, 0, 4, 3, 1.0);
                part_c(w, ctx, id, 0, 4);
            }
            planet::nearest_path(w, ctx, id, true);
            let m = &w.mobys[id];
            let (k, pt) = (m.pv_s(0x354).clamp(0, 31) as usize, m.pv_s(0x356).max(0) as usize);
            let q = planet::slot(w, ctx, id, k).get(pt).map_or(m.pos, |q| [q[0], q[1], q[2]]);
            let q = planet::surface(ctx, w.level22.radius, q, 8.0);
            let d = dist(m.pos, q);
            let m = &mut w.mobys[id];
            m.set_pv_v(0x360, q);
            m.set_pv_f(0x3a0, d);
            state(w, id, 0x22, 5);
            let m = &mut w.mobys[id];
            m.set_pv_s(0x3a4, 0);
            if m.pv_s(0x3a6) == 3 { m.set_pv_s(0x3a6, if rec[2] == 0 { 0 } else { 1 }); }
        }
        0x22 => {
            let m = &w.mobys[id];
            let p = m.pv_v(0x360);
            if dist(m.pos, p) < 2.0 {
                w.mobys[id].pos = p;
                state(w, id, 0x23, 5);
                anim(w, ctx, id, 6, 0, 0x14, 3, 1.0);
                let k = ctx.game.rng.rand_range(0x168, 600);
                let m = &mut w.mobys[id];
                m.set_pv_s(0x384, k as i16);
                m.set_pv_s(0x386, 0);
                m.set_pv_f(0x3a8, 0.0);
            } else {
                let v = sub(p, m.pos);
                let (mut x, mut vel) = (0.0, m.pv_f(0x3a8));
                ease(len(v), f32::from_bits(0x3c36_0b61), f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3eaa_aaab), &mut x, &mut vel);
                let m = &mut w.mobys[id];
                m.set_pv_f(0x3a8, vel);
                m.pos = add(m.pos, set_len(v, vel));
                if ended(w, ctx, id) && seq(w, ctx, id) == 5 { anim(w, ctx, id, 6, 0, 0x14, 3, 1.0); }
            }
        }
        0x23 => on_path(w, id, ctx),
        0x24 => {
            let r = w.level22.radius;
            let h = ctx.hero.gravity.height(w.mobys[id].pos);
            if h <= r {
                state(w, id, 5, 0);
                let m = &mut w.mobys[id];
                m.set_pv_s(0x350, 0);
                m.set_pv_f(0x3e4, 0.0);
                m.set_pv_f(0x3e8, 0.0);
                return;
            }
            let mut e = h - r;
            let m = &w.mobys[id];
            let v = sub(planet::surface(ctx, r, m.pos, 0.0), m.pos);
            let mut vel = m.pv_f(0x3a8);
            ease(0.0, f32::from_bits(0x3c36_0b61), f32::from_bits(0x3c36_0b61), f32::from_bits(0x3eaa_aaab), &mut e, &mut vel);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x3a8, vel);
            m.pos = add(m.pos, set_len(v, -vel));
        }
        _ => {}
    }
}

/// States 2..4's fidget: the idle again from frame 0 every 27..33 ticks (`+0x384`).
fn fidget(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_s(0x384) < 1 {
        anim(w, ctx, id, 0, 0, 0x14, 3, 1.0);
        let k = ctx.game.rng.randi(7) as i16 + 0x1b;
        w.mobys[id].set_pv_s(0x384, k);
    }
}

/// 0x12, the run-off (from `prev`): its walk sequence 0xb (unless it was running off), the point by him, → 0x13.
fn run_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, prev: u8) {
    if prev != 0x12 && seq(w, ctx, id) != 0xb { anim(w, ctx, id, 0xb, 0, 4, 3, 1.0); }
    planet::beside_hero(w, ctx, id);
    state(w, id, 0x13, 9);
}

/// 0x13's guns: in the rider's sequence 3 the shots at frames 1 / 4 (in reach 70 with charge, or before its end),
/// else its sequence 0xb at its frame; out of 3 within 70 with charge (and no phase-2 timer) the rider's 3.
fn guns_or_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, _moving: bool) {
    let rd = rider(w, id);
    let rec = rec(w);
    let d = w.mobys[id].pv_f(0x3dc);
    if rd.is_some_and(|r| seq(w, ctx, r) == 3) {
        let r = rd.unwrap();
        if !ended(w, ctx, r) || (rec[2] != 0 && d <= 70.0) {
            fire_at_frames(w, id, ctx, r);
        } else {
            let f = ftime(w, ctx, Some(id));
            anim(w, ctx, id, 0xb, f as i32 as u8, 4, 2, 1.0);
        }
    } else if d < 70.0 && rec[2] != 0 && w.mobys[id].pv_i(0x464) == 0 {
        anim(w, ctx, id, 3, 0, 4, 2, 1.0);
        part_c(w, ctx, id, 7, 4);
    }
}

/// The rider's shots as its frame passes 2 (joint 4) and 4 (joint 3), against the frame it had last tick (`+0x44c`).
fn fire_at_frames(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: usize) {
    let last = w.mobys[id].pv_f(0x44c);
    let f = ftime(w, ctx, Some(r));
    if 4.0 <= f && last < 4.0 {
        shoot(w, ctx, id, 1);
    } else if 2.0 <= f && last < 2.0 {
        shoot(w, ctx, id, 2);
    }
}

/// 0x14's choice after 30 ticks with no pickup to fetch (module docs).
fn choose_after_rest(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = most(w);
    let rec = rec(w);
    let h = hp(w, id);
    let to_check = if h < ((n << 1) / 3) as f32 {
        if h <= (n / 6) as f32 {
            true
        } else if ctx.game.rng.randi(2) == 0 {
            !(rec[3] == 0 && rec[2] == 0)
        } else {
            false
        }
    } else {
        true
    };
    if to_check {
        let m = &w.mobys[id];
        if m.pv_i(0x464) == 0 || m.pv_s(0x41e) != 2 {
            if h <= (n / 6) as f32 || (rec[3] == 0 && rec[2] == 0) || ctx.game.rng.randi(5) != 0 {
                state(w, id, 5, 0);
            } else {
                state(w, id, 0x1d, 3);
            }
            return;
        }
    }
    if w.mobys[id].pv_f(0x3dc) < 55.0 { state(w, id, 0x12, 9); }
}

/// 0x1e, the attack choice (module docs).
fn choose_attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rec = rec(w);
    let m = &w.mobys[id];
    if m.pv_i(0x3d0) == 0 || m.pv_i(0x3d8) == 0 { return planet::follow(w, ctx, id); }
    let d = m.pv_f(0x3dc);
    if d < NEAR {
        if ctx.game.rng.randi(5) != 0 || rec[2] == 0 {
            anim(w, ctx, id, 0xd, 0, 4, 3, 1.0);
            state(w, id, 0x1f, 4);
        } else {
            state(w, id, 0x21, 5);
            w.mobys[id].set_pv_s(0x3a6, 3);
        }
        return;
    }
    if 33.0 <= d { return planet::follow(w, ctx, id); }
    let mut specials = 0 < w.mobys[id].pv_s(0x41e) && rec[3] != 0;
    if rec[2] != 0 {
        let mut guns = true;
        if specials {
            if ctx.game.rng.randi(2) == 0 { specials = false } else { guns = false }
        }
        if guns {
            let c = rec[2];
            let k = if c < 0x32 {
                if c < 3 { 1 } else { ctx.game.rng.randi(c / 2) }
            } else {
                c - ctx.game.rng.randi(0x23)
            };
            w.mobys[id].set_pv_s(0x430, k as i16);
            if ctx.game.rng.randi(4) == 0 {
                state(w, id, 0x21, 5);
                w.mobys[id].set_pv_s(0x3a6, 1);
            } else {
                state(w, id, 0x20, 4);
            }
            return;
        }
    }
    if !specials { return state(w, id, 0x12, 9); }
    state(w, id, 0x21, 5);
    w.mobys[id].set_pv_s(0x3a6, 0);
}

/// 0x1f, the punches (module docs).
fn punches(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = seq(w, ctx, id);
    let rd = rider(w, id);
    let rt = ftime(w, ctx, rd);
    let hero = ctx.hero.pos;
    match s {
        9 | 10 => {
            if 7.0 < rt && rt < 12.0 {
                if let Some(r) = rd { ai::attack_joint(w, r, ctx, 4.0, 10.0, 1.0, if s == 10 { 4 } else { 3 }, 0x1_0007, 0, 2, 0); }
            }
            if s == 10 {
                if ended(w, ctx, id) { anim(w, ctx, id, 0xd, 0, 4, 3, 1.0); }
                frames::move_to(w, id, ctx, hero);
            } else {
                if ended(w, ctx, id) {
                    planet::follow(w, ctx, id);
                    anim(w, ctx, id, 10, 0, 4, 3, 1.0);
                }
                let t = ftime(w, ctx, Some(id));
                if 0.0 < t && t < 23.0 { frames::move_to(w, id, ctx, hero); }
            }
        }
        0xd => {
            if w.mobys[id].first() { w.level22.immune = 0; }
            if !ended(w, ctx, id) {
                if 17.0 < rt && rt < 20.0 {
                    let r = rd.unwrap();
                    if let (Some(a), Some(b)) = (ctx.host.joint(&w.mobys[r], 4), ctx.host.joint(&w.mobys[r], 3)) {
                        let f = scale(add(b, a), 0.5);
                        ai::attack_sphere(w, id, ctx, 4.0, 100.0, 14.5, f, 0x1_0007, 0, 2, 0);
                        let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
                        if dist(f, add(scale(up, 4.0), hero)) < 6.0 { w.level22.immune = 0x14; }
                    }
                }
            } else if w.level22.immune == 0 || (ctx.game.rng.randi(4) != 0 && hp(w, id) <= (ctx.hero.giant.hp * 7) as f32) {
                if !near(w, ctx, id) {
                    state(w, id, 0x12, 9);
                } else {
                    anim(w, ctx, id, 0xd, 0, 4, 3, 1.0);
                    state(w, id, 0x1f, 4);
                }
            } else {
                let full = hp(w, id) == w.mobys[id].pv_s(0x326) as f32 && ctx.hero.giant.hp < 100;
                state(w, id, if full { 4 } else { 3 }, 2);
            }
            ai::senses(&mut w.mobys[id], 115.0 * 0.000_277_777_8, 17.5 * 0.016_666_668, 1440.0 * 4.848_137_5e-6, 720.0 * 0.000_290_888_25);
            frames::move_to(w, id, ctx, hero);
        }
        _ => {}
    }
    let m = &w.mobys[id];
    if m.sub_state == 6 || m.pv_f(0x3dc) <= NEAR { return; }
    if w.level22.immune == 0 { state(w, id, 0x12, 9); }
}

/// 0x20, the shooting (module docs).
fn shooting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rec = rec(w);
    if !w.mobys[id].first() {
        if let Some(c) = part(w, id, 0x448, PART_C) {
            if ended(w, ctx, c) {
                let k = if ctx.game.rng.randi(4) != 0 { 1 } else { 7 };
                ctx.host.set_anim(&w.mobys[c], k, 0, 4);
            }
        }
    } else {
        anim(w, ctx, id, 3, 0, 4, 2, 1.0);
        part_c(w, ctx, id, 1, 4);
        let k = ctx.game.rng.rand_range(0x78, 0xf0);
        let m = &mut w.mobys[id];
        m.set_pv_s(0x384, k as i16);
        m.set_pv_s(0x386, 0);
    }
    let m = &w.mobys[id];
    let rd = rider(w, id);
    if m.pv_s(0x384) < 1 || rec[2] == 0 || m.pv_i(0x3d8) == 0 || 60.0 < m.pv_f(0x3dc) {
        state(w, id, 5, 0);
    } else if rd.is_some_and(|r| !ended(w, ctx, r)) {
        let r = rd.unwrap();
        if seq(w, ctx, r) == 3 {
            fire_at_frames(w, id, ctx, r);
        } else {
            let m = &mut w.mobys[id];
            let mut k = m.pv_s(0x386) as i32;
            countdown(&mut k);
            m.set_pv_s(0x386, k as i16);
            if k < 1 {
                anim(w, ctx, id, 3, 0, 4, 2, 1.0);
                part_c(w, ctx, id, 1, 4);
            }
        }
    } else if m.pv_f(0x3dc) < NEAR {
        state(w, id, 5, 0);
    } else {
        w.mobys[id].set_pv_s(0x386, 0);
    }
    if w.mobys[id].pv_f(0x3e0) < NEAR {
        anim(w, ctx, id, 0xd, 0, 4, 3, 1.0);
        state(w, id, 0x1f, 4);
        return;
    }
    let s = seq(w, ctx, id);
    let m = &w.mobys[id];
    let d = m.pv_f(0x3dc);
    if s == 0xc {
        if d < 37.0 {
            anim(w, ctx, id, 0, 0, 4, 1, 1.0);
            return;
        }
    } else if s != 0xb {
        if rec[2] < m.pv_s(0x430) as i32 {
            anim(w, ctx, id, 0xb, 0, 4, 1, 1.0);
            let m = &mut w.mobys[id];
            m.set_pv_s(0x430, 1000);
            m.set_pv_i(0x3ac, 0);
        }
        let d = w.mobys[id].pv_f(0x3dc);
        if 42.0 < d {
            anim(w, ctx, id, 0xc, 0, 4, 1, 1.0);
            w.mobys[id].set_pv_i(0x3ac, 0);
        } else if 25.0 <= d {
            hold(w, id, ctx);
        } else {
            anim(w, ctx, id, 0xb, 0, 4, 1, 1.0);
            let m = &w.mobys[id];
            let p = add(m.pos, scale(sub(m.pos, ctx.hero.pos), 20.0));
            let q = planet::nearest_point(w, ctx, id, p, true).unwrap_or(m.pv_v(0x3b0));
            let m = &mut w.mobys[id];
            m.set_pv_v(0x3b0, q);
            m.set_pv_i(0x3ac, 1);
        }
        return;
    } else if (m.pv_s(0x430) as i32) <= rec[2] && 32.0 < d {
        anim(w, ctx, id, 0, 0, 4, 1, 1.0);
        return;
    }
    planet::follow(w, ctx, id);
}

/// 0x23, on the path (module docs).
fn on_path(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rec = rec(w);
    let rd = rider(w, id);
    let m = &w.mobys[id];
    let mode3 = m.pv_s(0x3a6);
    let stop = m.pv_s(0x384) < 1 || m.pv_i(0x3d8) == 0 || m.pv_i(0x418) != 0 || (m.pv_i(0x464) != 0 && m.pv_s(0x41e) == 2)
        || (mode3 == 1 && (rec[2] == 0 || 0x78 < m.state_ticks as u16));
    if stop {
        state(w, id, 0x24, 5);
        anim(w, ctx, id, 2, 0, 4, 2, 1.0);
        w.mobys[id].set_pv_f(0x3a8, 0.0);
    } else if let Some(r) = rd {
        if !ended(w, ctx, r) {
            match seq(w, ctx, r) {
                3 => fire_at_frames(w, id, ctx, r),
                0xe => {
                    let f = ftime(w, ctx, Some(r));
                    if 22.0 < f && f < 55.0 {
                        ctx.host.set_anim_speed(&w.mobys[r], 1.0);
                    } else if (10.0..=22.0).contains(&f) {
                        let m = &w.mobys[id];
                        let at = add(m.pos, add(set_len(m.rows[2], 3.0), m.rows[0]));
                        special::hold_by(w, ctx, id, at, false);
                        ctx.host.set_anim_speed(&w.mobys[r], f32::from_bits(0x3eb3_3333));
                    }
                }
                _ => {
                    let m = &mut w.mobys[id];
                    let mut k = m.pv_s(0x386) as i32;
                    countdown(&mut k);
                    m.set_pv_s(0x386, k as i16);
                    if k < 1 {
                        if mode3 == 1 {
                            anim(w, ctx, id, 3, 0, 4, 2, 1.0);
                            part_c(w, ctx, id, 5, 4);
                        } else if mode3 == 0 {
                            let c = &mut w.level22.boss_counts[3];
                            if 0 < *c { *c -= 1; }
                            anim(w, ctx, id, 0xe, 0, 4, 2, 1.0);
                            part_c(w, ctx, id, 5, 4);
                        }
                    }
                }
            }
        } else {
            match mode3 {
                1 => {
                    anim(w, ctx, id, 2, 0, 4, 2, 1.0);
                    w.mobys[id].set_pv_s(0x386, 0);
                }
                2 => {
                    if 30.0 < w.mobys[id].pv_f(0x3dc) {
                        state(w, id, 0x24, 5);
                        w.mobys[id].set_pv_f(0x3a8, 0.0);
                    }
                }
                0 => {
                    if rec[3] == 0 {
                        state(w, id, 0x24, 5);
                        anim(w, ctx, id, 6, 0, 4, 3, 1.0);
                        w.mobys[id].set_pv_f(0x3a8, 0.0);
                    } else {
                        let c = &mut w.level22.boss_counts[3];
                        if 0 < *c { *c -= 1; }
                    }
                }
                _ => {}
            }
        }
    }
    along_path(w, id, ctx);
}

/// 0x23's walk along its slot between 32 and 38 of him (module docs), then the bob.
fn along_path(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let r = w.level22.radius;
    let f = w.mobys[id].pv_f(0x3dc);
    if 38.0 <= f || f <= 32.0 {
        let mut flip = false;
        let fv = (f - 35.0).abs();
        let mode = if 32.0 < f {
            let m = &mut w.mobys[id];
            if 1 < m.pv_s(0x3a4) as u16 {
                flip = true;
                m.set_pv_s(0x352, -m.pv_s(0x352));
            }
            m.set_pv_s(0x3a4, 1);
            3
        } else {
            let m = &mut w.mobys[id];
            let k = m.pv_s(0x3a4);
            if k != 0 && k != 2 {
                flip = true;
                m.set_pv_s(0x352, -m.pv_s(0x352));
            }
            m.set_pv_s(0x3a4, 2);
            2
        };
        let m = &w.mobys[id];
        let k = m.pv_s(0x354).clamp(0, 31) as usize;
        let pts = planet::slot(w, ctx, id, k);
        let at = |n: i32| pts.get(n as usize).map_or([0.0; 3], |q| [q[0], q[1], q[2]]);
        if (m.state_ticks as u16) < 0x3c && !flip {
            let (pos, hero) = (m.pos, ctx.hero.pos);
            let pt = m.pv_s(0x356) as i32;
            let mut dir = m.pv_s(0x352);
            let mut n = pt + 1;
            let a = if n < pts.len() as i32 { planet::surface(ctx, r, at(n), 8.0) } else { n = -1; dir = 1; [0.0; 3] };
            let mut j = pt - 1;
            let b = if j < 0 { j = -1; dir = -1; [0.0; 3] } else { planet::surface(ctx, r, at(j), 8.0) };
            let to_hero = sub(hero, pos);
            let fa = if n != -1 { dot(sub(a, pos), to_hero) } else { 0.0 };
            let fb = if j != -1 { dot(sub(b, pos), to_hero) } else { 0.0 };
            if mode == 3 {
                if n == -1 {
                    if 0.0 < fb { dir = -1 } else if j == -1 && 0.0 < fa { dir = 1 }
                } else if j == -1 {
                    if 0.0 < fa { dir = 1; }
                } else if fb < fa && 0.0 < fa {
                    dir = 1;
                }
            } else if n == -1 {
                if fb < 0.0 { dir = -1 } else if j == -1 && fa < 0.0 { dir = 1 }
            } else if j == -1 {
                if fa < 0.0 { dir = 1; }
            } else {
                dir = if fb <= fa { -1 } else { 1 };
            }
            w.mobys[id].set_pv_s(0x352, dir);
        }
        let m = &w.mobys[id];
        let wp = m.pv_v(0x360);
        if !flip && 2.0 <= dist(m.pos, wp) {
            let v = set_len(sub(wp, m.pos), (fv * 0.0625).min(0.5));
            let p = planet::surface(ctx, r, add(m.pos, v), 8.0);
            let m = &mut w.mobys[id];
            m.pos = p;
            m.set_pv_f(0x3e8, 0.0);
        } else {
            let n = m.pv_s(0x356) as i32 + m.pv_s(0x352) as i32;
            if n < 0 {
                let bits = w.level22.links[k][0];
                planet::next_path(w, ctx, id, bits, mode, true);
            } else if pts.len() as i32 <= n {
                let bits = w.level22.links[k][1];
                planet::next_path(w, ctx, id, bits, mode, true);
            } else {
                w.mobys[id].set_pv_s(0x356, n as i16);
            }
            let m = &w.mobys[id];
            let (k, pt) = (m.pv_s(0x354).clamp(0, 31) as usize, m.pv_s(0x356).max(0) as usize);
            let q = planet::slot(w, ctx, id, k).get(pt).map_or(m.pos, |q| [q[0], q[1], q[2]]);
            w.mobys[id].set_pv_v(0x360, planet::surface(ctx, r, q, 8.0));
        }
    }
    // `0x329498(1.7, 0.11, −1, m, +0x3e4, +0x3e8, row 2)`: the bob.
    let m = &mut w.mobys[id];
    let up = m.rows[2];
    let a = add_rot(m.pv_f(0x3e4), f32::from_bits(0x3de1_47ae));
    m.set_pv_f(0x3e4, a);
    m.pos = sub(m.pos, scale(up, m.pv_f(0x3e8)));
    let q = a.sin() * f32::from_bits(0x3fd9_999a);
    m.set_pv_f(0x3e8, q);
    m.pos = add(m.pos, scale(up, q));
}

/// The tail after the states (module docs).
fn after(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    match part(w, id, 0x388, SHADOW) {
        None => {
            let s = w.create(SHADOW);
            let sc = w.class(SHADOW).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[s];
            (m.drawn, m.state, m.pos) = (1, 0, pos);
            m.scale = sc * f32::from_bits(0x3da3_d70a);
            (m.update_dist, m.draw_dist) = (0xff, 0xff);
            m.pvars.resize(m.pvars.len().max(0x20), 0);
            m.set_pv_v(0, pos);
            ai::set_link(m, 0x10, Some(id));
            m.set_pv_i(0x14, 0);
            ai::set_link(&mut w.mobys[id], 0x388, Some(s));
        }
        Some(s) => w.mobys[s].set_pv_v(0, pos),
    }
    let rd = rider(w, id);
    let (t, rt) = (ftime(w, ctx, Some(id)), ftime(w, ctx, rd));
    w.mobys[id].set_pv_f(0x38c, t);
    w.mobys[id].set_pv_f(0x44c, rt);
    place_parts(w, id, ctx);
    pickups::tick(w, id, ctx);
    {
        let m = &mut w.mobys[id];
        ai::flash(m, 0x50);
        if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
        let mut k = m.pv_i(0x37c);
        if k != 0 { k -= 1; }
        m.set_pv_i(0x37c, k);
        let mut k = m.pv_i(0x464);
        countdown(&mut k);
        m.set_pv_i(0x464, k);
        countdown(&mut w.level22.timer);
    }
    turn_rider(w, id, ctx);
    path_punch_stop(w, id, ctx);
    if let Some(c) = part(w, id, 0x448, PART_C) {
        let st = w.mobys[id].state;
        let ss = w.mobys[id].sub_state;
        if !ended(w, ctx, c) {
            if seq(w, ctx, c) == 6 && ss != 8 && st != 0x26 { ctx.host.set_anim(&w.mobys[c], 7, 0, 10); }
        } else if !(4..=5).contains(&ss) && !matches!(ss, 8 | 1 | 2) && st != 0x26 {
            let k = if ctx.game.rng.randi(5) == 0 { 7 } else { 0 };
            ctx.host.set_anim(&w.mobys[c], k, 0, 10);
        }
    }
    if (11..=12).contains(&seq(w, ctx, id)) {
        let m = &w.mobys[id];
        let a = m.pos;
        let b = add(set_len(m.pv_v(0x200), 12.0), a);
        if let Some(h) = w.coll_line(ctx, a, b, 0x12, None) { w.mobys[id].pos = h.point; }
    }
}

/// On a skipped slot in the walk's state 3 its punches stop (`+0x43a`, `+0x34c`; the rider's joint hits at frames
/// 7..12, 4 / 50 / flags 0x8b0000), and pick up again at the rider's end.
fn path_punch_stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rd = rider(w, id);
    let m = &w.mobys[id];
    if m.pv_s(0x350) != 3 {
        let m = &mut w.mobys[id];
        m.set_pv_s(0x43a, 0);
        m.set_pv_s(0x34c, 0);
        return;
    }
    if planet::skipped(w, m.pv_s(0x354) as i32) && (11..=12).contains(&seq(w, ctx, id)) {
        let rs = rd.map_or(0xff, |r| seq(w, ctx, r));
        let rt = ftime(w, ctx, rd);
        if (rs == 9 || rs == 10) && 7.0 < rt && rt < 12.0 {
            let joint = if rs == 9 { 4 } else { 3 };
            if let Some(r) = rd { ai::attack_joint(w, r, ctx, 4.0, 50.0, 0.0, joint, 0x8b_0000, 0, 2, 0); }
        }
        let re = rd.is_some_and(|r| ended(w, ctx, r));
        let next = match rs { 9 if re => Some(10), 9 => None, 10 if !re => None, _ => Some(9) };
        if let Some(k) = next {
            w.mobys[id].set_pv_s(0x34c, 0);
            anim(w, ctx, id, k, 0, 4, 2, 1.0);
            w.mobys[id].set_pv_s(0x34c, 1);
        }
        w.mobys[id].set_pv_s(0x43a, 1);
        return;
    }
    if w.mobys[id].pv_s(0x43a) != 0 && rd.is_some_and(|r| ended(w, ctx, r)) {
        let m = &mut w.mobys[id];
        m.set_pv_s(0x43a, 0);
        m.set_pv_s(0x34c, 0);
        let (s, t) = (seq(w, ctx, id), ftime(w, ctx, Some(id)));
        anim(w, ctx, id, s, t as i32 as u8, 4, 2, 1.0);
    }
}

/// `0x3a58c0` (module docs).
fn place_parts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::classes::enemies::place_part;
    if let Some(r) = rider(w, id) {
        let s = seq(w, ctx, r);
        if (1..=3).contains(&s) || w.mobys[id].pv_s(0x460) != 0 {
            if let Some(p) = ctx.host.joint(&w.mobys[id], 1) { w.mobys[r].pos = p; }
        } else {
            place_part(w, id, r, 1, ctx);
        }
        let sp = speed(w, ctx, id);
        ctx.host.set_anim_speed(&w.mobys[r], sp);
    }
    let Some(r) = rider(w, id) else { return };
    if let Some(b) = part(w, id, 0x444, PART_B) { place_part(w, r, b, 5, ctx); }
    if let Some(c) = part(w, id, 0x448, PART_C) {
        if w.mobys[id].mode & 1 == 0 {
            place_part(w, r, c, 5, ctx);
            let m = &mut w.mobys[c];
            m.pos = add(m.pos, crate::gravity::to_world([-2.55, 0.0, 2.6], &m.rows));
        }
    }
}

/// `0x3a60d8` (module docs).
fn turn_rider(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::attach::{rows_of, turn};
    let Some(r) = rider(w, id) else { return };
    let s = seq(w, ctx, r);
    let turning = (1..=3).contains(&s);
    let k = w.mobys[id].pv_s(0x460);
    if !turning && k == 0 { return; }
    let rp = w.mobys[r].pos;
    if (turning && k == 0) || (!turning && k == 30) { w.mobys[id].set_pv_f(0x458, 0.0); }
    let m = &w.mobys[id];
    let (target, accel, brake) = if turning {
        let v = sub(ctx.hero.pos, rp);
        let l = crate::gravity::to_local(v, &w.mobys[r].rows);
        (crate::hero::physics::atan2(l[0], l[1]), m.pv_f(0x124), m.pv_f(0x128) * 2.0)
    } else {
        let x = ctx.host.joint_matrix(&w.mobys[id], 1).map_or([1.0, 0.0, 0.0], |(rows, _)| rows[0]);
        let l = crate::gravity::to_local(x, &w.mobys[r].rows);
        (crate::hero::physics::atan2(l[0], l[1]), m.pv_f(0x124) * 2.0, m.pv_f(0x128) * 4.0)
    };
    let m = &w.mobys[id];
    let (max, mut v) = (m.pv_f(0x12c), m.pv_f(0x458));
    let mut a = 0.0;
    crate::hero::physics::turn_ease(target, accel, brake, max, &mut a, &mut v, 0);
    w.mobys[id].set_pv_f(0x458, v);
    let up = scale(w.mobys[id].pv_v(0x200), -1.0);
    ctx.host.advance(&w.mobys[r]);
    let rm = &mut w.mobys[r];
    rm.rows = turn(&rows_of([0.0, 0.0, v]), &rm.rows);
    rm.mode |= 0x106;
    rm.rows = turn_rows(&rm.rows, upright(&rm.rows, up));
    let m = &mut w.mobys[id];
    if turning {
        m.set_pv_s(0x460, 30);
    } else {
        let mut k = k as i32;
        countdown(&mut k);
        m.set_pv_s(0x460, k as i16);
    }
}

/// `0x3a2a00` (module docs: the set-up in state 0, then the frame).
fn frame(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { set_up(w, id, ctx); }
    let class_scale = |w: &MobyWorld, c: i16| w.class(c).map_or(1.0, |c| c.scale);
    let (dd, ud, light, ambient, glow) = { let m = &w.mobys[id]; (m.draw_dist, m.update_dist, m.light, m.ambient, m.glow) };
    for (off, class, s, d) in [(0x440, RIDER, SCALE, None), (0x444, PART_B, SCALE, Some(0x50)), (0x448, PART_C, SCALE_C, Some(0x50))] {
        let Some(p) = part(w, id, off, class) else { continue };
        let sc = class_scale(w, class) * s;
        let m = &mut w.mobys[p];
        (m.draw_dist, m.update_dist, m.light, m.ambient, m.glow, m.drawn, m.scale) = (d.unwrap_or(dd), ud, light, ambient, glow, 1, sc);
        if off == 0x440 { crate::shadow_volumes::probe_near(w, p, ctx, 24.0, 0x16); }
    }
    let down = planet::down_at(ctx, w.mobys[id].pos);
    {
        let m = &mut w.mobys[id];
        m.set_pv_v(0x200, down);
        m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
    }
    ai::take_damage(w, id, ctx, false);
    w.mobys[id].scale = class_scale(w, CLASS) * SCALE;
    let g = &mut w.level22;
    if g.intro == 0 {
        if ctx.hero.giant.hp <= 0 && w.mobys[id].sub_state != 1 {
            let m = &mut w.mobys[id];
            m.set_pv_s(0x350, 0);
            m.set_pv_f(0x3e4, 0.0);
            m.set_pv_f(0x3e8, 0.0);
            m.set_state(2, 1);
            m.set_pv_s(0x384, 0x1e);
        }
    } else {
        g.intro += 1;
        if 32000 < g.intro { g.intro = 1; }
        ctx.hero.land_lag = 10;
        if w.mobys[id].sub_state != 1 {
            let m = &mut w.mobys[id];
            m.set_pv_s(0x350, 0);
            m.set_pv_f(0x3e4, 0.0);
            m.set_pv_f(0x3e8, 0.0);
            m.set_state(1, 1);
        }
        if ctx.hero.items.pressed & 0x50 == 0 {
            if w.level22.intro < 0x1e { put_gun_away(ctx); }
        } else {
            w.level22.intro = 0;
            w.mobys[id].set_state(5, 0);
            ctx.hero.items.pressed = 0;
            ctx.hero.land_lag = 0x14;
        }
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
}

/// `0x3a2a00`'s set-up (module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ticks = ctx.game.orbit_ticks;
    let full = w.mobys[id].pv_s(0x326) as i32;
    let most = if 126_000 < ticks {
        (full as f32 * 0.6) as i32
    } else if 90_000 < ticks {
        (full as f32 * 0.9) as i32
    } else {
        full
    };
    {
        let g = &mut w.level22;
        g.most = most;
        g.intro = 1;
        g.phase = 0;
        g.timer = 0;
        g.drones_back = 0;
        g.flag = true;
        g.skip = planet::SKIP;
        g.skip_n = 5;
        g.scan_at = 0;
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(0x4ec, 1);
    for off in [0x460, 0x41e, 0x46a, 0x468, 0x34e, 0x34c] { m.set_pv_s(off, 0); }
    m.set_pv_i(0x464, 0);
    m.set_pv_i(0x344, 0);
    let mut body = None;
    for k in 0..w.mobys.len() {
        let m = &mut w.mobys[k];
        if 0xfd <= m.state { continue; }
        match m.o_class {
            BODY => body = Some(k),
            CLASS => w.level22.boss = Some(k),
            0x133a => m.mode |= 0x41,
            _ => {}
        }
    }
    set_trigger(w);
    if let Some(b) = body {
        let p = w.mobys[b].pos;
        ctx.hero.teleport(p, [0.00161, 0.29459, 1.57105]);
    }
    let pos = w.mobys[id].pos;
    let m = &mut w.mobys[id];
    ctx.hero.giant.hp = m.pv_s(0x324) as i32;
    w.level22.radius = dist(pos, ctx.hero.gravity.centre);
    let m = &mut w.mobys[id];
    m.mode |= 0x100;
    let wait = ctx.game.rng.randi(0x1e);
    let m = &mut w.mobys[id];
    for (off, b) in [(0x77, 8u8), (0x72, 8), (0x76, 8), (0xce, 9), (0xcf, 0x12), (0xd3, 0xff), (0xcc, 9), (0xcd, 0x12), (0xd2, 0xff), (0xd7, 1), (0x70, 0x25), (0x71, 4), (0x1f8, 0xc), (0x1f9, 0xb), (0x28, 3)] {
        m.set_pv_b(off, b);
    }
    let f = most as f32;
    for (off, v) in [
        (0x80, 0x3d88_8889u32), (0x7c, 0x3f15_5556), (0x88, 0x3daa_aaab), (0x8c, 0x3c88_8889), (0x90, 0x4040_0000), (0x94, 0x4080_0000),
        (0x9c, 0x3ecc_cccd), (0x134, 0x3b91_a2b4), (0x138, 0x3e4c_ccce), (0x110, 0x4020_0000), (0x114, 0x3f40_0000), (0x120, 0x4000_0000),
        (0x130, 0x3b91_a2b4), (0x118, 0x4030_0000), (0x84, 0x3d2a_aaab), (0x144, 0), (0x11c, 0x4030_0000), (0x140, 0x3f49_0fdc),
    ] {
        m.set_pv_i(off, v as i32);
    }
    m.set_pv_i(0xa8, 8);
    m.set_pv_s(0x380, wait as i16);
    m.set_pv_f(0xbc, 900.0 / f);
    m.set_pv_f(0xb8, 9.0 / f);
    m.set_pv_s(0x26, 0x3c);
    m.set_pv_f(0x20, f);
    m.set_pv_i(0x148, m.pv_i(0x148) | 1);
    m.set_pv_s(0x24, f as i32 as i16);
    (m.update_dist, m.draw_dist) = (0xff, 0xff);
    for (off, v) in [(0x3ec, -1), (0x4e4, 300), (0x374, -1), (0x3e4, 0), (0x3e8, 0), (0x388, 0), (0x37c, 0)] { m.set_pv_i(off, v); }
    for (off, v) in [(0x382, -1i16), (0x35c, -1), (0x35a, -1), (0x358, -1), (0x378, 0), (0x37a, 0), (0x35e, 0), (0x350, 0)] { m.set_pv_s(off, v); }
    m.mode |= 0x4000;
    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
    planet::link_paths(w, ctx, id);
    pickups::set_up(w, id, ctx);
    let (ppos, prot, plight) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light) };
    for (off, class) in [(0x440, RIDER), (0x444, PART_B), (0x448, PART_C)] {
        let p = w.create(class);
        let m = &mut w.mobys[p];
        (m.pos, m.rot, m.light, m.update_dist, m.drawn) = (ppos, prot, plight, 0xff, 1);
        if off == 0x440 { m.rows = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]; }
        w.rebuild(p, &*ctx.host);
        ai::set_link(&mut w.mobys[id], off, Some(p));
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(0x384, 0x1e);
    m.set_pv_s(0x43a, 0);
}

/// 0x0d55, the opponent's pointer (L22 `0x3f9af8`, L23 `0x400cc0`; the decomp's `func_L22_003F8658`): gone with its
/// owner (`+0x10`); else 3 ahead of the camera and 1.2 up in its frame (`$gp+0x1b50`), turned toward the owner's place
/// (`+0x00`; its yaw and pitch), spun 120′ a tick about its x (`$gp+0x1b70`), its scale the class's × (0.08 + 0.14 ×
/// `+0x18`), `+0x0c` 3.
pub fn update_0d55(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x20 { w.mobys[id].pvars.resize(0x20, 0); }
    let owner = ai::link(&w.mobys[id], 0x10).filter(|&o| w.mobys.get(o).is_some_and(|m| m.state < 0xfd));
    let sc = w.class(SHADOW).map_or(1.0, |c| c.scale);
    let (a, b) = (f32::from_bits(0x3da3_d70a), f32::from_bits(0x3e61_47ae));
    let k = w.mobys[id].pv_f(0x18);
    w.mobys[id].scale = sc * (a + (b - a) * k);
    if owner.is_none() {
        w.delete(id);
        return;
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(0xc, 3.0);
    let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    m.pos = add(crate::gravity::to_world([3.0, 0.0, 1.2], &rows), cam);
    let t = m.pv_v(0);
    m.rot[2] = crate::hero::physics::atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
    let flat = ((t[0] - m.pos[0]).powi(2) + (t[1] - m.pos[1]).powi(2)).sqrt();
    m.rot[1] = -crate::hero::physics::atan2(flat, t[2] - m.pos[2]);
    m.rot[0] = add_rot(m.rot[0], 120.0 * 0.000_290_888_25);
}
