//! The breeding critter **0x0b69** (L18 `0x3f6548`, 229 placed; L19 `0x3db3a0`, 32; L20 `0x3fafe0`, 72; the
//! decomp's `func_L18_003F50D8`, one shared code): a small biter that hunts in packs, lunges at Ratchet and at the
//! moby it preys on, and breeds — a grown one throws up to three of its brood out of hiding, each growing to full
//! size as it lands. On the framework (crate::moby::ai): health `+0x20` (its status byte `+0x2e`: 2 kills it at
//! once; its damage byte `+0x34`), reaction `+0x50` (a hurt goes to state 17, sub 3), movement `+0x100` (radius 0.4,
//! height 0.2, steps 2 / 5, slope 90°, gravity 1/180, stop 1.4), target block `+0x1f0`.
//!
//! **Its pvars:** `+0x210` / `+0x220` its place and rotation at the set-up, `+0x230` the goal, `+0x240` the area its
//! target search watches and `+0x244` its range (height 0: only an area finds Ratchet), `+0x248` found,
//! `+0x24c` the yaw offset of its walk, `+0x250` an idle timer (480..600, read by nothing here), `+0x258` the moby it
//! preys on (`+0x280` its index), `+0x25c` its brood's group, `+0x260` its fence path, `+0x264` the yaw it faces,
//! `+0x268` its full size, `+0x26c` / `+0x29c` / `+0x274` the path it runs on from the start (backward, its speed
//! a second), `+0x270` the cuboid that wakes it, `+0x278` a patrol path and `+0x27c` the group whose life it holds
//! to, `+0x290` facing this tick, `+0x292` its lives (s16), `+0x294` / `+0x2a4` a throw's bounces and their limit
//! (2..4), `+0x296` how it starts, `+0x298` the brood thrown, `+0x29a` its path point, `+0x29e` 180 (unread),
//! `+0x2a0` its kill height (−1 none), `+0x2a6` settled after a throw.
//!
//! **The update** (`0x3f6548`): on level 0x13 (`0x1a79f8`) a scene (`0x1a8f00` = 2) hides it without collision or
//! target, else it is shown with them; then state 0 the set-up ([`setup`], `0x3f6608`), states 1 and 19 nothing, any
//! other the frame's checks ([`head`], `0x3f6848`), the state ([`states`], `0x3f6ac0`), the move by its sequence
//! ([`movement`], `0x3f8118`) and the tail ([`tail`], `0x3f8608`).
//!
//! **States** (`+0x20`): 1 hidden in the brood pool (no update, no collision: [`hide`], `0x3f8a00`). 2 waiting
//! unseen (radius 0.34) for Ratchet in its cuboid; then a touch sphere and after 10 ticks a throw from where it is.
//! 3 thrown ([`launch`], `0x3f8b60`, from state 1 or 2: full health, shown, its velocity): sequence 9, growing from
//! 0.001 by a tenth of its size a tick, a sphere of its radius + 0.2 pushing what it touches (flags 0x4000000);
//! landed (240 ticks or its bounces) → 7 out of 6 (sub 2). 6 idle (sequence 1 / 2, at their ends 1 in 49 sequence 10,
//! speed 0.85..1.15): a prey → 11, a patrol path → 13, its target found (Ratchet not in 0x6f) → 12. 7 / 8 sequence 0 /
//! 10, at its end back to the state before. 9 breeding (sequences 6, 7, 8): every 21 ticks a hidden one of its brood
//! thrown from 0.5 above it, 2/15 up and 2..3 a second each way across, until three or none left. 10 the bite
//! (sequence 5): before frame 10 fast turns to the prey (from 11) or its target; frames 10..17 every moby within 0.2
//! of joint 0 but its own kind (Ratchet always) takes its damage byte (flags 0x10001, 2 away across); at its end back,
//! or with Ratchet hit by it this tick to 8 (the state before kept). 11 to the prey (sequence 4, offset 1°): it
//! ready (its state 6, `+0xbc` 2) within 3.2 ahead → 10; near (5, its state 5 `+0xbc` 1 or 3 after 5) → 8; gone →
//! 12 via 8. 12 the hunt (sequence 4, offset ±15..30°): to its target; within 3.2 ahead → sequence 1 / 2, then
//! (not stuck) the bite or, after 60 ticks, 12 again; 1 in 30 with a hidden one in its brood → 9 (sub 2); lost → 6.
//! Every 8 frames its group (`+0x27c`) all gone clears it and the patrol path. 13 the patrol (slow turns, offset
//! ±15..30°) along `+0x278` (points passed within 3); every 64 frames the group gone → 12. 14 the run (from the
//! start, no hits that hurt or knock: shares 1): hidden until Ratchet is in its cuboid, then along its path
//! (points passed within 0.2) at its speed, sequence 1 within 8 of Ratchet, within 2 a bite sphere of a quarter of
//! his health limit (`0x1a7a08`) and, meeting a moby not its kind, its health 0 and the hurt (17); the path's end →
//! 15. 15 the leap off (velocity jittered 0.05, 1/12 up, sequence 13, falling): after 300 ticks its bolts and gone.
//! 16 the hop back when stuck walking (sequence 2: 1/15 back between frames 4 and 22, then played backward at 0.65
//! with 2/15 ahead and 0.139 up, then the state before). 17 hurt (the framework's flight; from 14 three times the
//! knock): dead (or 1 in 15 late in sequence 12) its bolts and gone; over → 12 (sub 2; from sub 1 via 7). 18 hit by
//! a 0x0f9e: untargeted (sequence 3 from sub 2), after its sequence or 60 ticks its bolts and gone.
//!
//! **Each tick** ([`head`]): a hit from a 0x0f9e (`0x338e08`, kept) from state 6 on → 18 and nothing more; the
//! frame's damage (`0x342658` = L00 `0x327e38`, crate::moby::ai::take_damage; the state before kept from the
//! passing states 7..10 / 16 / 17), hurt outside 14 its group roused ([`rouse`], `0x3f8f28`: each in sub 1 to 11 /
//! 13 / 12, sub 2, via 7); its hit cleared; under 90 (or its kill height) gone; standing on a 0x0f59 its bolts and
//! gone; stuck over 300 ticks (but in 2) or from state 6 outside its fence path, gone; the target search
//! (`0x345e10` = L00 `0x32ad38`); its prey's `+0x768` set; Ratchet in 0x6f sends 9..12 to 6. **Going**
//! ([`die`], `0x3f8928`): health 0, state 19, untargeted, a life used; with lives left (or a runner) back into the
//! pool through the fade (crate::moby::classes::death_fade, `0x406eb8` = L01 `0x3eb1d0`), else faded out. The
//! **tail** ([`tail`]): its ticks, the facing cleared, its shadow (`0x338b78` = L00 `0x31da58`, within 24 of the
//! camera, range 0x16), outside 5..1018 gone; the head aim (`0x341e28` = L00 `0x327308` on `+0x2b0`, list 2, 1.8:
//! crate::moby::joint_aim::head).
//!
//! **The move** ([`movement`], by its sequence `+0x42`): the push-out option (2) cleared each tick. Sequences 0..3,
//! 6..8, 10: the hop back's move (16), the run (14: its speed toward the goal, turned at the limits, `0x347ad8` = L07
//! `0x322f38`, crate::moby::ai::move_by), breeding still, else facing (`0x347ab8` = L00 `0x32ceb8`) or held
//! (`0x347a98` = L00 `0x32ce98`). 4 the walk (`0x347830` = L00 `0x32cc30`, crate::moby::ai::move_toward), its rise
//! held to 1/300 on the floor; hunting (11 / 12) stuck 16 ticks, still and pushed by nothing → 16. 5 the lunge:
//! frames 10..17 on clear ground, not stuck, 10 ahead (`0x3477d8` = L01 `0x322450`). 9 the throw: by its velocity
//! (its radius scaled by its size; the push-out option until settled), a push-out bouncing it (0.8 of the velocity
//! reflected, `0x309a98` = L00 `0x2ef2a8`), falling. 13 the leap's fall (15).
//!
//! The engine functions with the port's: `0x3368c0` = L00 `0x31b8d0` (crate::moby::Moby::set_state), `0x3365e0` =
//! L00 `0x31b5f0` (set_anim, crate::moby::classes::props::set_seq), `0x3458a0` = L00 `0x32a7c8` (crate::moby::Moby::
//! first), `0x3489d0` = L00 `0x32dcf0` (crate::moby::ai::senses), `0x336990` / `0x3369d0` / `0x336a28` = L00
//! `0x31b9a0` / `0x31b9e0` / `0x31ba38` (the rng), `0x32c880` = L00 `0x311880` (the frame time), `0x32d808` = L00
//! `0x3127c8` (a joint), `0x338158` = L00 `0x31d180` (crate::moby::ai::attack_sphere), `0x2fbd48` = L00 `0x2e14b0`
//! (the mobys a sphere touches), `0x2fa170` = L00 `0x2df8d8` (the sphere test), `0x3385d0` = L00 `0x31d4c8`
//! (crate::moby::MobyWorld::deliver_hit_any), `0x342ad0` = L00 `0x3282b0` (crate::moby::ai::react_start),
//! `0x3433a0` = L00 `0x328b80` (crate::moby::ai::flight), `0x33c5e8` = L00 `0x3217d8` (the jitter), `0x33c668` = L00
//! `0x321858` (crate::level::LevelData::in_cuboid), `0x33c880` (the fence path's polygon, crate::level::LevelData::
//! in_polygon), `0x335db0` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x32c688` = L00 `0x311688` (delete),
//! `0x33a350` = L00 `0x31f230` (turn_ease), `0x35e438` (the nearest path point), `0x3f8ff0` (L14 `0x425290`, the
//! path step, [`path_step`]).
//!
//! Not ported: the collision's first primitive's byte `+0x12` (`0x32c990`: 3 at the set-up and a throw, 2 when a
//! runner wakes). The
//! throw's bounce reflects off the push-out's direction: the game's off the last collision normal (`0x1bfbc0`), which
//! the port's push-out does not keep. The frames' slices (`frame & 7` / `& 0x3f` against the moby's address `>> 8`)
//! use its index.

use crate::hero::physics::{add, atan2, dist, dist2, dot, len, rot_dist, scale, set_len, sub, turn_ease, V3};
use crate::moby::classes::death_fade;
use crate::moby::classes::level08::tabora_ai::jitter;
use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{ai, bolts, mode, Ctx, MobyWorld, UpdateFn};

pub const CLASS: i16 = 0x0b69;
/// A hit from it sends one to state 18.
const EATER: i16 = 0x0f9e;
/// Standing on it kills one.
const KILL_FLOOR: i16 = 0x0f59;

const HEALTH: usize = 0x20;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const REACT: usize = 0x50;
const MV: usize = 0x100;
const T_BLOCK: usize = 0x1f0;
const HOME: usize = 0x210;
const HOME_ROT: usize = 0x220;
const GOAL: usize = 0x230;
const AREA: usize = 0x240;
const RANGE: usize = 0x244;
const FOUND: usize = 0x248;
const OFFSET: usize = 0x24c;
const IDLE: usize = 0x250;
const PREY: usize = 0x258;
const BROOD: usize = 0x25c;
const FENCE: usize = 0x260;
const FACE: usize = 0x264;
const SIZE: usize = 0x268;
const RUN_PATH: usize = 0x26c;
const CUBOID: usize = 0x270;
const RUN_SPEED: usize = 0x274;
const PATROL: usize = 0x278;
const WATCH: usize = 0x27c;
const PREY_INDEX: usize = 0x280;
const FACING: usize = 0x290;
const LIVES: usize = 0x292;
const BOUNCES: usize = 0x294;
const START: usize = 0x296;
const BRED: usize = 0x298;
const POINT: usize = 0x29a;
const BACKWARD: usize = 0x29c;
const KILL_Z: usize = 0x2a0;
const BOUNCE_MOST: usize = 0x2a4;
const SETTLED: usize = 0x2a6;
/// The head aim's record (crate::moby::joint_aim).
const HEAD_AIM: usize = 0x2b0;
/// The head's scale with the enemies' big heads (the tail's 1.8).
const HEAD_SCALE: f32 = f32::from_bits(0x3fe6_6666);
/// `$gp+0x13e0`: where a thrown one starts above its breeder.
const THROW_FROM: V3 = [0.0, 0.0, 0.5];

/// The normal limits (`0x3489d0(0.00444, 0.133, 0.00349, 0.209)`).
fn normal_senses(w: &mut MobyWorld, id: usize) {
    ai::senses(&mut w.mobys[id], 0.004_444_444_6, 0.133_333_34, 0.003_490_659, 0.209_439_55);
}

fn blob(w: &MobyWorld) -> bool { w.class(CLASS).is_some_and(|c| c.has_collision) }

fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn seq_b(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_b) }
fn speed(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed) }
fn ticks(w: &MobyWorld, id: usize) -> u16 { w.mobys[id].state_ticks as u16 }

/// `if seq ≠ randi(2) + 1 { sequence randi(2) + 1 }` (the game draws twice; the frame and the blend after).
fn idle_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, frame_rand: bool, blend: Option<u8>) {
    let r = ctx.game.rng.randi(2) as u8 + 1;
    if seq_b(w, id, ctx) == r { return; }
    let s = ctx.game.rng.randi(2) as u8 + 1;
    let f = if frame_rand { ctx.game.rng.randi(10) as u8 } else { 0 };
    let b = blend.unwrap_or_else(|| ctx.game.rng.rand_range(9, 15) as u8);
    ctx.host.set_anim(&w.mobys[id], s, f, b);
}

/// The moby `+0x258` it preys on.
fn prey(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], PREY).filter(|&k| k < w.mobys.len()) }

/// 0x0b69 (module docs).
pub fn update_0b69(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.level == 0x13 {
        let has = blob(w);
        let m = &mut w.mobys[id];
        if ctx.game.scenes.mode == crate::scene::SCENE {
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & !mode::TARGET;
        } else {
            m.mode = (m.mode & 0xffbe) | mode::TARGET;
            m.has_collision = has;
        }
    }
    match w.mobys[id].state {
        0 => setup(w, id, ctx),
        1 | 0x13 => {}
        _ => {
            // The game runs the four on whatever came of the one before; a deleted moby is left alone here.
            head(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
            states(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
            movement(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
            tail(w, id, ctx);
        }
    }
    w.changed = true;
}

/// `0x3f6608` (module docs): the health and reaction blocks, the movement, its size, its prey, its start (14 a
/// runner with a path, 1 into the pool, 2 waiting; else 6, sub 2).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let has = blob(w);
    let m = &mut w.mobys[id];
    // `0x30a7a8` (L00 `0x2effb8`): 15 and 6 as bytes.
    m.set_pv_b(0x48, 15);
    m.set_pv_b(0x4a, 6);
    // The reaction block (`+0x50`): a hurt to state 17, sub 3; the body's radius and centre 0.49; the bounce 0.4; the
    // flight's fence its fence path.
    m.set_pv_b(REACT + 0x10, 0x11);
    m.set_pv_f(REACT + 0x34, 0.49);
    let fence = m.pv_i(FENCE);
    m.set_pv_i(REACT + 0x7c, fence);
    m.set_pv_b(0x29, 1);
    m.set_pv_i(REACT + 0x48, 3);
    m.set_pv_f(REACT + 0x30, 0.49);
    m.set_pv_f(REACT + 0x3c, 0.4);
    if m.pv_s(HEALTH + 4) == 0 {
        m.set_pv_s(HEALTH + 4, 16);
        m.set_pv_f(HEALTH, 16.0);
    }
    m.set_pv_b(0x28, 3);
    m.set_pv_f(0x30, 0.5);
    m.set_pv_b(STATUS, 1);
    m.set_pv_f(MV + 0x34, 1.4);
    m.set_pv_f(MV + 4, 0.2);
    m.set_pv_f(MV, 0.4);
    m.set_pv_f(MV + 0x2c, 0.005_555_556);
    m.set_pv_f(MV + 8, 2.0);
    m.set_pv_f(MV + 0xc, 5.0);
    m.set_pv_f(MV + 0x10, 1.570_796_4);
    normal_senses(w, id);
    let m = &mut w.mobys[id];
    if fence != -1 { m.set_pv_i(MV + 0x3c, fence); }
    m.set_pv_f(SIZE, m.scale);
    let k = m.pv_i(PREY_INDEX);
    if k != -1 { ai::set_link(m, PREY, Some(k as usize)); }
    m.has_collision = has;
    m.set_state(6, 2);
    match m.pv_s(START) {
        14 => {
            if m.pv_i(RUN_PATH) != -1 { m.set_state(14, 2); }
        }
        1 => hide(w, id, ctx),
        2 => m.set_state(2, 0),
        _ => {}
    }
    let m = &mut w.mobys[id];
    let (p, r) = (m.pos, m.rot);
    m.set_pv_v(HOME, p);
    m.set_pv_v(HOME_ROT, r);
    w.changed = true;
}

/// `0x3f8a00`: into the brood pool — the reaction's options cleared, no collision, hidden, no update, untargeted, state
/// 1 (sub 0). Also the fade's callback.
fn hide(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_s(REACT + 0x38, 0);
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x43;
    m.set_state(1, 0);
    m.mode &= !mode::TARGET;
    w.changed = true;
}

/// `0x3f8928(m, now)`: gone (module docs); `now` without the fade.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, now: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_f(HEALTH, 0.0);
    m.set_state(0x13, 0);
    m.mode &= !mode::TARGET;
    let lives = m.pv_s(LIVES).wrapping_sub(1);
    m.set_pv_s(LIVES, lives);
    let back = lives != -1 || m.pv_s(START) == 14;
    match (now, back) {
        (true, true) => hide(w, id, ctx),
        (true, false) => w.delete(id),
        (false, true) => death_fade::start(w, id, ctx, 0.0, Some(hide as UpdateFn)),
        (false, false) => death_fade::start(w, id, ctx, 0.0, None),
    }
}

/// Its bolts (`0x335db0(m, 0, 1)`), then gone.
pub(crate) fn drop_and_die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    bolts::drop(w, id, 0, 1, ctx);
    die(w, id, ctx, false);
}

/// `0x3f8b60(m, at, vel)`: a throw (module docs), only from state 1 or 2.
pub(crate) fn launch(w: &mut MobyWorld, k: usize, at: V3, vel: V3) {
    let has = blob(w);
    let m = &mut w.mobys[k];
    if m.state.wrapping_sub(1) >= 2 { return; }
    m.set_pv_b(STATUS, 1);
    m.set_pv_f(HEALTH, m.pv_s(HEALTH + 4) as f32);
    m.drops = 0;
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = has;
    m.drawn = 0;
    m.mode = (m.mode & !mode::NO_UPDATE) | 1;
    m.set_state(3, 0);
    m.has_collision = has;
    m.mode |= mode::TARGET;
    m.pos = at;
    m.set_pv_v(MV + 0xa0, vel);
    m.set_pv_f(MV + 4, 0.01);
    m.set_pv_f(MV, 0.49);
    m.set_pv_f(MV + 8, 2.0);
    m.set_pv_f(MV + 0xc, 5.0);
    m.set_pv_i(MV + 0x80, 0);
    w.changed = true;
}

/// `0x3f8f28` → `0x3f8ef8` → `0x3f8e78`: each of its group in sub 1 roused — to 11 with a prey, 13 with a patrol
/// path, else 12 (sub 2), then 7.
fn rouse(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].group;
    if g < 0 { return; }
    let Some(members) = w.groups.get(g as usize).cloned() else { return };
    for e in members {
        let Some(m) = w.mobys.get_mut(e) else { break };
        if m.sub_state != 1 { continue; }
        let s = if m.pv_i(PREY) != 0 { 11 } else if m.pv_i(PATROL) != -1 { 13 } else { 12 };
        m.set_state(s, 2);
        m.set_state(7, -1);
    }
}

/// `0x3f8a58(group, &n)`: the last of the group of its kind hidden in state 1, and how many there are.
pub(crate) fn brood_pick(w: &MobyWorld, group: i32) -> (Option<usize>, i32) {
    let Some(members) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)) else { return (None, 0) };
    let mut n = 0;
    let mut pick = None;
    for &e in members {
        let Some(m) = w.mobys.get(e) else { break };
        if m.o_class == CLASS && m.state == 1 {
            n += 1;
            pick = Some(e);
        }
    }
    (pick, n)
}

/// The group's members all gone (or no such group).
fn group_gone(w: &MobyWorld, group: i32) -> bool {
    let Some(members) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)) else { return true };
    !members.iter().any(|&e| w.mobys.get(e).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}

/// `0x3f8828(m, p)`: drawn, within 3.2 of `p` across and facing it within 90°.
fn ahead(w: &MobyWorld, id: usize, p: V3) -> bool {
    let m = &w.mobys[id];
    m.drawn != 0 && dist2(m.pos, p) < 3.2 && rot_dist(m.rot[2], atan2(p[0] - m.pos[0], p[1] - m.pos[1])) < 1.570_796_5
}

/// `0x3f8748(m, p)`: face `p` this tick.
fn face(w: &mut MobyWorld, id: usize, p: V3) {
    let m = &mut w.mobys[id];
    m.set_pv_s(FACING, 1);
    m.set_pv_f(FACE, atan2(p[0] - m.pos[0], p[1] - m.pos[1]));
}

/// The sphere `+0x100 + +0x104` above its feet: `r`, `damage`, flags.
fn touch(w: &mut MobyWorld, id: usize, ctx: &Ctx, r: f32, damage: f32, flags: u32) -> V3 {
    let m = &w.mobys[id];
    let c = [m.pos[0], m.pos[1], m.pos[2] + m.pv_f(MV + 4) + m.pv_f(MV)];
    ai::attack_sphere(w, id, ctx, r, damage, 1.0, c, flags, 0, 1, 0);
    c
}

/// `0x35e438(p, path, 0)` (L14 `0x34cf20`): the path's point nearest `p` (−1 no path).
pub fn nearest_point(ctx: &Ctx, p: V3, path: i32) -> i16 {
    let Some(pts) = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).filter(|v| !v.is_empty()) else { return -1 };
    let mut best = 1e11;
    let mut at = 0;
    for (i, q) in pts.iter().enumerate() {
        let d = dist([q[0], q[1], q[2]], p).abs();
        if d < best {
            best = d;
            at = i;
        }
    }
    at as i16
}

/// `0x3f8ff0` on its own goal and point ([`path_step_at`]).
fn path_step(w: &mut MobyWorld, id: usize, ctx: &Ctx, near: f32, path: i32, backward: bool) -> bool { path_step_at(w, id, ctx, near, path, GOAL, POINT, backward) }

/// `0x3f8ff0(near, m, &goal, path, &point, backward)` (the shared L14 / L18 / L19 / L20 code: L14 `0x425290`, L19
/// `0x3dde48`, L20 `0x3fda88`; its users crate::moby::classes::level14::roamer and this): the goal (the pvar at
/// `goal`) its path point (the short at `point`); passed within `near` across, the next (or back); past the last
/// (first) false, the point wrapped.
#[allow(clippy::too_many_arguments)]
pub fn path_step_at(w: &mut MobyWorld, id: usize, ctx: &Ctx, near: f32, path: i32, goal: usize, point: usize, backward: bool) -> bool {
    let Some(pts) = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).filter(|v| !v.is_empty()) else { return false };
    let n = pts.len() as i16;
    let m = &mut w.mobys[id];
    let mut i = m.pv_s(point);
    let q = pts[(i.max(0) as usize).min(pts.len() - 1)];
    m.set_pv_v(goal, [q[0], q[1], q[2]]);
    m.set_pv_f(goal + 0xc, q[3]);
    let passed = dist2(m.pos, [q[0], q[1], q[2]]) < near;
    let ok = if !backward {
        if passed { i += 1; }
        if n <= i {
            i = 0;
            false
        } else {
            true
        }
    } else {
        if passed { i -= 1; }
        if i < 0 {
            i = n - 1;
            false
        } else {
            true
        }
    };
    m.set_pv_s(point, i);
    ok
}

/// `0x3f6848` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x338e08(m, 0x10000, 1)`: the record kept either way.
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    if hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == EATER) {
        let s = w.mobys[id].state;
        if s < 6 || s == 0x13 || s == 0x12 { return; }
        w.mobys[id].set_state(0x12, -1);
        return;
    }
    let saved = w.mobys[id].prev_state;
    if ai::take_damage(w, id, ctx, false) != 0 {
        let m = &mut w.mobys[id];
        if matches!(m.prev_state, 17 | 8 | 9 | 16 | 10 | 7) { m.prev_state = saved; }
        if m.prev_state != 14 { rouse(w, id); }
    }
    w.clear_hit(id);
    let m = &w.mobys[id];
    let kill_z = m.pv_f(KILL_Z);
    if m.pos[2] < 90.0 || (kill_z != -1.0 && m.pos[2] < kill_z) {
        die(w, id, ctx, false);
    }
    if w.mobys[id].pv_b(STATUS) == 2 {
        die(w, id, ctx, true);
        return;
    }
    let m = &w.mobys[id];
    if ai::link(m, MV + 0x88).and_then(|k| w.mobys.get(k)).is_some_and(|f| f.o_class == KILL_FLOOR) {
        drop_and_die(w, id, ctx);
        return;
    }
    let m = &w.mobys[id];
    if 300 < m.pv_i(MV + 0x80) && m.state != 2 {
        die(w, id, ctx, false);
        return;
    }
    let fence = m.pv_i(FENCE);
    if 6 <= m.state && fence != -1 && !ctx.level.in_polygon(fence, m.pos) {
        die(w, id, ctx, false);
        return;
    }
    let (area, range) = (m.pv_i(AREA), m.pv_f(RANGE));
    let found = ai::find_target(w, id, ctx, range, 0.0, area, None, false);
    w.mobys[id].set_pv_i(FOUND, found as i32);
    if let Some(p) = prey(w, id) { w.mobys[p].set_pv_s(0x768, 1); }
    if ctx.hero.state == 0x6f && (9..=12).contains(&w.mobys[id].state) { w.mobys[id].set_state(6, -1); }
}

/// `0x3f6ac0` (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        2 => waiting(w, id, ctx),
        3 => thrown(w, id, ctx),
        6 => idle(w, id, ctx),
        7 | 8 => {
            let s = if w.mobys[id].state == 7 { 0 } else { 10 };
            if w.mobys[id].first() {
                set_seq(w, id, ctx, s, 0, 10);
            } else if wrapped(w, id, ctx) {
                back(w, id);
            }
        }
        9 => breed(w, id, ctx),
        10 => bite(w, id, ctx),
        11 => stalk(w, id, ctx),
        12 => hunt(w, id, ctx),
        13 => patrol(w, id, ctx),
        14 => run(w, id, ctx),
        15 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                let mut v = m.pv_v(MV + 0xa0);
                jitter(&mut ctx.game.rng, &mut v, 0.05);
                let m = &mut w.mobys[id];
                m.set_pv_v(MV + 0xa0, v);
                m.set_pv_f(MV + 0xa8, 0.083_333_336);
                set_seq(w, id, ctx, 13, 0, 10);
            } else if 0x12d <= ticks(w, id) {
                drop_and_die(w, id, ctx);
            }
        }
        16 => hop(w, id, ctx),
        17 => hurt(w, id, ctx),
        0x12 => {
            if w.mobys[id].first() {
                w.mobys[id].mode &= !mode::TARGET;
                if w.mobys[id].sub_state == 2 { set_seq(w, id, ctx, 3, 0, 10); }
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if wrapped(w, id, ctx) || 0x3d <= ticks(w, id) {
                w.mobys[id].set_pv_f(HEALTH, 0.0);
                drop_and_die(w, id, ctx);
            }
        }
        _ => {}
    }
}

/// Back to the state before (`0x3368c0(m, +0x94, −1)`).
fn back(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let p = m.prev_state;
    m.set_state(p, -1);
}

/// State 2 (module docs).
fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.set_pv_f(MV, 0.34);
        m.set_pv_f(MV + 8, 0.1);
        m.set_pv_f(MV + 4, 0.0);
        set_seq(w, id, ctx, 9, 0, 0);
        let m = &mut w.mobys[id];
        m.drawn = 0;
        m.timer = 0;
        m.mode |= 1;
        return;
    }
    match w.mobys[id].timer {
        0 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(CUBOID), ctx.hero.pos) {
                let m = &mut w.mobys[id];
                m.state_ticks = 0;
                m.timer = 1;
            }
        }
        1 => {
            let r = w.mobys[id].pv_f(MV) * 2.0;
            touch(w, id, ctx, r, 10.0, 0x400_0000);
            if 10 < ticks(w, id) {
                let m = &w.mobys[id];
                let mut v = set_len(m.rows[0], 0.066_666_67);
                v[2] = 0.033_333_335;
                let at = m.pos;
                launch(w, id, at, v);
            }
        }
        _ => {}
    }
}

/// State 3 (module docs).
fn thrown(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].set_pv_s(SETTLED, 0);
        ctx.host.set_anim(&w.mobys[id], 9, 0, 3);
        ctx.host.advance(&w.mobys[id]);
        let m = &mut w.mobys[id];
        m.drawn = 1;
        m.scale = 0.001;
        m.mode &= !1;
        m.set_pv_s(BOUNCES, 0);
        let a = ctx.game.rng.rand_range(0xb4, 0xb4);
        let b = ctx.game.rng.rand_range(2, 4);
        let m = &mut w.mobys[id];
        m.set_pv_s(0x29e, a as i16);
        m.set_pv_s(BOUNCE_MOST, b as i16);
    } else {
        let t = ticks(w, id);
        let m = &mut w.mobys[id];
        if m.pv_s(SETTLED) == 0 && 0x1f <= t { m.set_pv_s(SETTLED, 1); }
        let size = m.pv_f(SIZE);
        if m.scale < size { m.scale += size * 0.1; }
        // `0x3f8790`: landed — 241 ticks or its bounces.
        if 0xf1 <= t || m.pv_s(BOUNCE_MOST) <= m.pv_s(BOUNCES) {
            m.scale = size;
            m.set_state(6, 2);
            m.set_state(7, -1);
        }
    }
    let r = w.mobys[id].pv_f(MV) + 0.2;
    touch(w, id, ctx, r, 10.0, 0x400_0000);
}

/// State 6 (module docs).
fn idle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        idle_seq(w, id, ctx, false, None);
        let t = ctx.game.rng.rand_range(0x1e0, 600);
        w.mobys[id].set_pv_i(IDLE, t);
    } else {
        let m = &mut w.mobys[id];
        if m.pv_i(PREY) != 0 {
            m.set_state(0xb, -1);
        } else if m.pv_i(PATROL) != -1 {
            m.set_state(0xd, -1);
        } else if m.pv_i(FOUND) == 1 && ctx.hero.state != 0x6f {
            m.set_state(0xc, -1);
        }
    }
    if !wrapped(w, id, ctx) { return; }
    if ctx.game.rng.randi(0x31) == 0 {
        if seq_b(w, id, ctx) != 10 {
            let b = ctx.game.rng.rand_range(9, 15) as u8;
            ctx.host.set_anim(&w.mobys[id], 10, 0, b);
        }
    } else {
        idle_seq(w, id, ctx, false, None);
    }
    let s = ctx.game.rng.randf(0.85, 1.15);
    ctx.host.set_anim_speed(&w.mobys[id], s);
}

/// State 9 (module docs).
fn breed(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 6, 0, 10);
        let m = &mut w.mobys[id];
        m.timer = 0;
        m.set_pv_s(BRED, 0);
        return;
    }
    match w.mobys[id].timer {
        0 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 7, 0, 10);
                w.mobys[id].timer = 1;
            }
        }
        1 => {
            let mut n = 0x32;
            if 0x14 < ticks(w, id) {
                w.mobys[id].state_ticks = 0;
                let (spawn, count) = brood_pick(w, w.mobys[id].pv_i(BROOD));
                n = count;
                if let Some(k) = spawn {
                    let at = add(w.mobys[id].pos, THROW_FROM);
                    let mut v = [0.0, 0.0, 0.133_333_34];
                    for c in 0..2 {
                        let d = ctx.game.rng.randf(2.0, 3.0) * 0.016_666_668;
                        v[c] += if ctx.game.rng.randi(2) != 0 { d } else { -d };
                    }
                    launch(w, k, at, v);
                    w.mobys[k].parent = w.mobys[id].parent;
                    let m = &mut w.mobys[id];
                    m.set_pv_s(BRED, m.pv_s(BRED) + 1);
                }
            }
            if 3 <= w.mobys[id].pv_s(BRED) || n == 0 {
                set_seq(w, id, ctx, 8, 0, 10);
                w.mobys[id].timer = 2;
            }
        }
        2 => {
            if wrapped(w, id, ctx) { back(w, id); }
        }
        _ => {}
    }
}

/// State 10 (module docs).
fn bite(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 5, 0, 10);
        return;
    }
    let f = ctx.host.frame_time(&w.mobys[id]);
    let a = seq_a(w, id, ctx);
    if a == 5 && f < 10.0 {
        ai::senses(&mut w.mobys[id], 0.083_333_336, 0.333_333_34, 0.193_925_5, 2.908_882_6);
        let p = match prey(w, id) {
            Some(k) if w.mobys[id].prev_state == 0xb => w.mobys[k].pos,
            _ => w.mobys[id].pv_v(T_BLOCK),
        };
        face(w, id, p);
    }
    if a == 5 && 10.0 < f && f < 17.0 {
        let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
        let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, at, 0.2, crate::collision::QueryFlags(0), Some(id))).unwrap_or_default();
        let damage = w.mobys[id].pv_b(DAMAGE) as f32;
        let pos = w.mobys[id].pos;
        for e in near {
            if Some(e) != ctx.hero_moby && w.mobys[e].o_class == CLASS { continue; }
            let ep = w.mobys[e].pos;
            let mut r = sub(ep, pos);
            r[2] = 0.0;
            let r = set_len(r, 2.0);
            let t = crate::moby::hits::HitTemplate::new(damage, Some(id), 0x10001, [r[0], r[1], r[2], 5_627.925]);
            w.deliver_hit_any(e, &t, [ep[0], ep[1], ep[2], 1.0]);
        }
    }
    if !wrapped(w, id, ctx) { return; }
    normal_senses(w, id);
    if ctx.hero.damage.hit_by == Some(id) {
        let m = &mut w.mobys[id];
        let p = m.prev_state;
        m.set_state(8, -1);
        m.prev_state = p;
    } else {
        back(w, id);
    }
}

/// State 11 (module docs).
fn stalk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 4, 0, 10);
        w.mobys[id].set_pv_f(OFFSET, 0.017_453_294);
        return;
    }
    let p = prey(w, id);
    if let Some(k) = p {
        let at = w.mobys[k].pos;
        w.mobys[id].set_pv_v(GOAL, at);
        let pm = &w.mobys[k];
        // `0x3fa2f0`: ready (its state 6, `+0xbc` 2).
        if pm.state == 6 && pm.timer == 2 && ahead(w, id, at) {
            w.mobys[id].set_state(10, -1);
            return;
        }
        // `0x3f87c8` → `0x3fa2b0`: within 5 and it in state 5 (`+0xbc` 1) or 3 after 5.
        let pm = &w.mobys[k];
        let open = (pm.state == 5 && pm.timer == 1) || (pm.state == 3 && pm.prev_state == 5);
        if dist2(w.mobys[id].pos, at) < 5.0 && open {
            w.mobys[id].set_state(8, -1);
            return;
        }
    }
    let gone = p.is_none_or(|k| { let pm = &w.mobys[k]; pm.state == 0xfe || pm.state == 0xfd || pm.mode & 1 != 0 });
    if !gone { return; }
    let m = &mut w.mobys[id];
    m.set_pv_i(PREY, 0);
    m.set_state(0xc, -1);
    m.set_state(8, -1);
}

/// A random yaw offset of 15..30° either way.
fn rand_offset(ctx: &mut Ctx) -> f32 {
    let f = ctx.game.rng.randf(15.0, 30.0);
    f * if ctx.game.rng.randi(2) != 0 { -0.017_453_294 } else { 0.017_453_294 }
}

/// State 12 (module docs).
fn hunt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 4, 0, 10);
        let o = rand_offset(ctx);
        let m = &mut w.mobys[id];
        m.set_pv_f(OFFSET, o);
        m.timer = 0;
    }
    let target = w.mobys[id].pv_v(T_BLOCK);
    match w.mobys[id].timer {
        0 => {
            w.mobys[id].set_pv_v(GOAL, target);
            let g = w.mobys[id].pv_i(WATCH);
            if g != -1 && ctx.game.frame & 7 == (id as i32) & 7 && group_gone(w, g) {
                let m = &mut w.mobys[id];
                m.set_pv_i(WATCH, -1);
                m.set_pv_i(PATROL, -1);
            }
            if ahead(w, id, target) {
                let m = &mut w.mobys[id];
                m.state_ticks = 0;
                m.timer = 1;
                idle_seq(w, id, ctx, false, Some(10));
            }
        }
        1 => {
            if ahead(w, id, target) && w.mobys[id].pv_i(MV + 0x80) < 2 {
                w.mobys[id].set_state(10, -1);
            } else if 0x3c < ticks(w, id) {
                w.mobys[id].set_state(0xc, -1);
            }
        }
        _ => {}
    }
    // `0x3f88e0`: 1 in 30, one of its brood hidden.
    if ctx.game.rng.randi(0x1e) == 0 && brood_pick(w, w.mobys[id].pv_i(BROOD)).0.is_some() {
        w.mobys[id].set_state(9, 2);
    }
    if w.mobys[id].pv_i(FOUND) != 1 { w.mobys[id].set_state(6, -1); }
}

/// State 13 (module docs).
fn patrol(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 4, 0, 10);
        let i = nearest_point(ctx, w.mobys[id].pos, w.mobys[id].pv_i(PATROL));
        w.mobys[id].set_pv_s(POINT, i);
        let o = rand_offset(ctx);
        w.mobys[id].set_pv_f(OFFSET, o);
        ai::senses(&mut w.mobys[id], 0.001_666_666_7, 0.050_000_004, 0.003_490_659, 0.104_719_77);
        return;
    }
    let path = w.mobys[id].pv_i(PATROL);
    path_step(w, id, ctx, 3.0, path, false);
    let g = w.mobys[id].pv_i(WATCH);
    if g == -1 || ctx.game.frame & 0x3f != (id as i32) & 0x3f { return; }
    if group_gone(w, g) {
        let m = &mut w.mobys[id];
        m.set_pv_i(WATCH, -1);
        m.set_pv_i(PATROL, -1);
        m.set_state(0xc, -1);
    }
}

/// State 14 (module docs).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.set_pv_f(REACT + 0x58, 1.0);
        m.set_pv_f(MV + 0x34, 0.001);
        m.set_pv_f(REACT + 0x5c, 1.0);
        let path = m.pv_i(RUN_PATH);
        let i = nearest_point(ctx, m.pos, path);
        let m = &mut w.mobys[id];
        m.set_pv_s(POINT, i);
        if let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|v| !v.is_empty() && 0 <= i) {
            let n = pts.len() as i16;
            let next = if m.pv_s(BACKWARD) == 0 {
                if i == n - 1 { 0 } else { i + 1 }
            } else if i == 0 {
                n - 1
            } else {
                i - 1
            };
            let at = |k: i16| { let q = pts[(k.max(0) as usize).min(pts.len() - 1)]; [q[0], q[1], q[2]] };
            let p = sub(at(next), at(i));
            let q = sub(at(i), m.pos);
            if dot(q, p) < 0.0 { m.set_pv_s(POINT, next); }
        }
        m.has_collision = false;
        m.mode = (m.mode & !mode::TARGET) | 0x41;
        idle_seq(w, id, ctx, true, Some(10));
        w.mobys[id].timer = 0;
        return;
    }
    if w.mobys[id].timer == 0 {
        let c = w.mobys[id].pv_i(CUBOID);
        if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) {
            let has = blob(w);
            let m = &mut w.mobys[id];
            m.timer = 1;
            m.mode = (m.mode & 0xffbe) | mode::TARGET;
            m.has_collision = has;
        }
        return;
    }
    let (path, backward) = (w.mobys[id].pv_i(RUN_PATH), w.mobys[id].pv_s(BACKWARD) != 0);
    if !path_step(w, id, ctx, 0.2, path, backward) { w.mobys[id].set_state(0xf, -1); }
    let pos = w.mobys[id].pos;
    let d = dist(pos, ctx.hero.pos);
    if 8.0 <= d { return; }
    set_seq(w, id, ctx, 1, 0, 10);
    if 2.0 <= d { return; }
    let r = w.mobys[id].pv_f(MV) + 0.2;
    let damage = ctx.game.max_health as f32 * 0.25;
    let c = touch(w, id, ctx, r, damage, 1);
    let hit = ctx.coll.and_then(|cl| crate::collision::coll_sphere_m(cl, ctx.scene, c, r, crate::collision::QueryFlags(1), Some(id)));
    let Some(k) = hit.and_then(|h| h.moby) else { return };
    if w.mobys.get(k).is_none_or(|o| o.o_class == CLASS) { return; }
    w.mobys[id].set_pv_f(HEALTH, 0.0);
    let mut d = set_len(w.mobys[id].rows[0], -1.0);
    jitter(&mut ctx.game.rng, &mut d, 0.3);
    ai::react_start(w, id, ctx, [d[0], d[1], d[2], 0.0], 2);
    w.mobys[id].set_state(0x11, 3);
}

/// State 16 (module docs).
fn hop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 2, 0, 10);
        let m = &mut w.mobys[id];
        let v = set_len(m.rows[0], -0.066_666_67);
        m.set_pv_v(MV + 0xa0, v);
        return;
    }
    let f = ctx.host.frame_time(&w.mobys[id]);
    let s = speed(w, id, ctx);
    if s == 1.0 && 23.0 < f {
        ctx.host.set_anim_speed(&w.mobys[id], -0.65);
        let m = &mut w.mobys[id];
        let v = set_len(m.rows[0], 0.133_333_34);
        m.set_pv_v(MV + 0xa0, v);
        m.set_pv_f(MV + 0xa8, m.pv_f(MV + 0xa8) + 0.138_888_9);
        return;
    }
    if s != -0.65 || 3.0 <= f { return; }
    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
    back(w, id);
}

/// State 17 (module docs).
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        if seq_a(w, id, ctx) == 12 || seq_b(w, id, ctx) == 12 { w.mobys[id].mode &= !mode::TARGET; }
        let m = &mut w.mobys[id];
        if m.prev_state == 14 { m.set_pv_v(REACT + 0x80, scale(m.pv_v(REACT + 0x80), 3.0)); }
    }
    let flags = ai::flight(w, id, ctx);
    let a = seq_a(w, id, ctx);
    if w.mobys[id].prev_state.wrapping_sub(14) < 2 && a == 11 && 16.0 < ctx.host.frame_time(&w.mobys[id]) {
        w.mobys[id].set_state(0xf, -1);
    }
    let a = seq_a(w, id, ctx);
    let f = ctx.host.frame_time(&w.mobys[id]);
    if flags & 1 != 0 || (a == 12 && 15.0 < f && ctx.game.rng.randi(15) == 0) {
        drop_and_die(w, id, ctx);
        return;
    }
    if flags & 2 != 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(PREY, 0);
        let sub = m.sub_state;
        m.set_state(0xc, 2);
        if sub == 1 { m.set_state(7, -1); }
    }
}

/// Facing its yaw this tick (`0x347ab8` = L00 `0x32ceb8`), else held (`0x347a98` = L00 `0x32ce98`).
fn face_or_hold(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.pv_s(FACING) != 0 {
        let yaw = m.pv_f(FACE);
        ai::turn_in_place(w, id, ctx, yaw);
    } else {
        ai::hold(w, id, ctx);
    }
}

/// Its rise held to 1/300 on the floor.
fn hold_rise(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.pv_f(MV + 0x90) <= m.pos[2] && 0.003_333_333_6 < m.pv_f(MV + 0xa8) { m.set_pv_f(MV + 0xa8, 0.003_333_333_6); }
}

fn fall(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0xa8, m.pv_f(MV + 0xa8) - m.pv_f(MV + 0x2c));
}

/// `0x3f8118` (module docs).
fn movement(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !2);
    let a = seq_a(w, id, ctx);
    let state = w.mobys[id].state;
    match a {
        0..=3 | 6..=8 | 10 => {
            if state == 16 && a == 2 {
                let f = ctx.host.frame_time(&w.mobys[id]);
                if 4.0 < f && f < 22.0 {
                    let v = w.mobys[id].pv_v(MV + 0xa0);
                    ai::move_by(w, id, ctx, v);
                    fall(w, id);
                    return;
                }
                if speed(w, id, ctx) != -1.0 || 4.0 < f { return; }
                ai::hold(w, id, ctx);
                return;
            }
            if state == 14 {
                let m = &mut w.mobys[id];
                if m.timer == 0 { return; }
                let goal = m.pv_v(GOAL);
                let d = set_len(sub(goal, m.pos), m.pv_f(RUN_SPEED) * 0.016_666_668);
                let yaw = atan2(goal[0] - m.pos[0], goal[1] - m.pos[1]);
                let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
                turn_ease(yaw, m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c), &mut r, &mut v, 0);
                m.rot[2] = r;
                m.set_pv_f(MV + 0xb4, v);
                ai::move_by(w, id, ctx, d);
                return;
            }
            if state == 9 { return; }
            face_or_hold(w, id, ctx);
        }
        4 => {
            let (o, goal) = (w.mobys[id].pv_f(OFFSET), w.mobys[id].pv_v(GOAL));
            ai::move_toward(w, id, ctx, o, goal);
            hold_rise(w, id);
            let m = &mut w.mobys[id];
            if m.state.wrapping_sub(0xb) > 1 || m.pv_i(MV + 0x80) < 0x10 { return; }
            if m.pv_f(MV + 0xa8) < 0.001 && m.pv_i(MV + 0x84) == 0 { m.set_state(0x10, -1); }
        }
        5 => {
            let f = ctx.host.frame_time(&w.mobys[id]);
            let m = &w.mobys[id];
            if 10.0 < f && f < 17.0 && m.pv_i(MV + 0x94) & 0xf == 0 && m.pv_i(MV + 0x80) == 0 && m.pv_s(MV + 0xc2) == 0 {
                let to = add(m.pos, scale(m.rows[0], 10.0));
                let yaw = atan2(to[0] - m.pos[0], to[1] - m.pos[1]);
                ai::move_step(w, id, ctx, yaw, to);
                let m = &w.mobys[id];
                if m.pos[2] < m.pv_f(MV + 0x90) { return; }
                hold_rise(w, id);
                return;
            }
            face_or_hold(w, id, ctx);
        }
        9 => {
            if state == 2 { return; }
            let m = &mut w.mobys[id];
            let r = m.pv_f(MV);
            let old = m.pv_v(MV + 0xa0);
            if m.pv_s(SETTLED) == 0 { m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 2); }
            m.set_pv_f(MV, r * m.scale / m.pv_f(SIZE));
            ai::move_by(w, id, ctx, old);
            let m = &mut w.mobys[id];
            m.set_pv_f(MV, r);
            let push = m.pv_v(MV + 0x70);
            if 0.0 < len(push) && m.pv_i(MV + 0x84) == 0 {
                m.set_pv_s(BOUNCES, m.pv_s(BOUNCES) + 1);
                m.pos = add(m.pos, push);
                // `0x309a98(v, old, n)`: reflected off the face (the push-out's direction, module docs).
                let n = set_len(push, 1.0);
                let v = scale(sub(old, scale(n, 2.0 * dot(old, n))), 0.8);
                m.set_pv_v(MV + 0xa0, v);
                m.pos = add(m.pos, v);
            }
            fall(w, id);
        }
        13 => {
            if state != 0xf { return; }
            fall(w, id);
            let v = w.mobys[id].pv_v(MV + 0xa0);
            ai::move_by(w, id, ctx, v);
        }
        _ => {}
    }
}

/// `0x3f8608` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    m.set_pv_s(FACING, 0);
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(5.0..=1018.0).contains(&c)) { die(w, id, ctx, false); }
    // `0x341e28` = L00 `0x327308`: the head aim.
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 2, HEAD_SCALE);
}

