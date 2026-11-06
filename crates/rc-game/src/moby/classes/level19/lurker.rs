//! Grelbin's lurker **0x10d0** (`0x41c3a8`, 700 placed; the decomp's v2.00 `func_US2_L19_0041C3A8`, its helpers
//! v1.01 `func_L19_0041BFD0` / `func_L19_0041C2B8` at our `0x41d300` / `0x41d5e8`): it waits hidden (state 1) until
//! Ratchet is in one of its two searches and the level's pool of attackers has room, comes up out of hiding, runs at
//! him with leaps and bites, backs off after each, and once it loses him goes back home and hides again. Some wait in
//! a den (crate::moby::classes::level19::den), which releases them one at a time. On the framework
//! (crate::moby::ai): health `+0x20` (most `+0x24`, its status byte `+0x2e`: 2 deletes it; its damage byte `+0x34`),
//! movement `+0x50` (radius 1, height 0.8, steps 0.75 / 0.75, slope 90°, the turn left 30°, stop 0.5), reaction
//! `+0x140` (a hurt to state 25, sub 3; radius and centre 0.5, bounce 0.15), target block `+0x1f0` (`+0x200` the
//! moby, `+0x204` found).
//!
//! **Its pvars:** `+0x210` home (its place on the ground at the set-up, or the den it came out of), `+0x220` /
//! `+0x224` the second search's area and range and `+0x228` / `+0x22c` the first's (both 10 high, measured from home),
//! `+0x230` / `+0x234` timers counted down (`+0x230` 480..600 after an alarm, 580..600 at a flight; read by nothing
//! here), `+0x238` / `+0x240` the alarm's countdown and the alarm (set on its group's members when one is hurt,
//! [`alarm`], `0x41d658`), `+0x250` a group it waits on (255 none) and `+0x264` a moby it waits on (−1 none),
//! `+0x268` 0, `+0x26c` held in a den, `+0x270` the run's yaw offset (±5..25°), `+0x274` straight to the run (set by
//! the den), `+0x278` / `+0x27c` the two searches' results, `+0x280` the back-off's goal, `+0x290` the flight's goal,
//! `+0x2a0` the head aim.
//!
//! **Each tick** ([`head`], `0x41d300`): nothing outside the loaded variant (`0x31f3f0` = L00 `0x321a78`) or with
//! Ratchet in 0x6f. In state 0 the set-up: the pool emptied (`$gp+0x2cd0`), 1 in 2 mirrored, 1.45 times its size, on
//! the ground (`0x31a908` = L00 `0x31ce20`), hidden, the blocks, state 1 (sub 0), sequence 12. Status 2: one attacker
//! less and deleted. The frame's damage (`0x3250a8` = L00 `0x327e38`, crate::moby::ai::take_damage): hurt, its group
//! alarmed. From frame 6 on the two searches (`0x327e70` = L00 `0x32ad38`). In state 1 untargeted.
//!
//! **Sub-states** (`+0x95`): **0** hidden ([`waiting`]): found by the first search, not held and the pool open, it
//! comes out (state 10, sub 2, sequence 0; one attacker more) once the moby it waits on is gone or, with none, its
//! group has no member alive (or there is no group); otherwise an alarm counts down. **1** going home: found by
//! either search and the pool open, back to the run (14, sub 2; one more). **2** attacking: shown, solid and
//! targeted; found by neither and not mid-leap or bite, it flees ([`flee`]: state 7, sub 1, sequence 7, 1 in 3 6
//! else 1 in 2 13; one less). **3** hurt: the framework's flight (`0x325df0` = L00 `0x328b80`); dead, its bolts
//! (`0x318780` = L00 `0x31adc0`, limit 3), the fade (`0x3eaed0` = L01 `0x3eb1d0`) and one less; over, with its target
//! found and the pool not over, the run (14, sub 2), else the flight (8, sub 1, sequence 2).
//!
//! **States** (`+0x20`): **1** hidden. **3** into hiding (sequence 1): at its end sequence 12, state 1 (sub 0),
//! hidden, without collision, untargeted. **7** turning to go (at its end 8, sequence 2). **8** the flight home: slow
//! turns, to `+0x290` (home and 6..8 a random way); within 0.5 across, 3. **10** coming up (sequence 0): facing its
//! target; at frame 23 sequence 7 (1 in 2 6) and 11, or with `+0x274` (else 1 in 2) the run. **11** facing its target
//! to its sequence's end, then the run. **14** the run (sequence 3): solid; within 4..7 of its target and facing it
//! within 30°, the leap (15, sequence 4, faster turns); within 3 and facing, the bite (16, sequence 10); else the walk
//! to the target's point offset by `+0x270`. **15** the leap: between frames 4 and 15 the move to its target, until
//! frame 19 a 0.75 sphere at joint 0 (its damage byte, flags 1); at frame 30 the recovery (24, sequence 7, 1 in 3 13
//! from frame 41 else 6) and 4 in 5 the back-off (17, sequence 5). **16** the bite: until frame 14 the sphere; at 25
//! the recovery, 2 in 3 the back-off. **17** the back-off: on its first tick its goal 18 behind it; between frames 4
//! and 14 the move to it with the options 0x10; at 21 the recovery. **24** the recovery: at its end the run. **25**
//! hurt (its sub 3).
//!
//! **The tail** (`0x41d5e8`): its two timers, its state ticks.
//!
//! The engine functions with the port's: `0x319290` = L00 `0x31b8d0` (crate::moby::Moby::set_state), `0x318fb0` =
//! L00 `0x31b5f0` (set_anim, crate::moby::classes::props::set_seq), `0x327900` / `0x327928` = L00 `0x32a7c8` /
//! `0x32a7f0` (the first tick of the state / sub), `0x32aad0` = L00 `0x32dcf0` (crate::moby::ai::senses),
//! `0x329930` = L00 `0x32cc30` (crate::moby::ai::move_toward), `0x329800` (L01 `0x322450`, the move to a point,
//! crate::moby::ai::move_step), `0x329858` (L02 `0x331440`: the same with options and limits for the tick),
//! `0x32aa68` (L01 `0x323560`: the facing eased at the turn limits), `0x321608` = L00 `0x323bf8`
//! (crate::moby::ai::reached), `0x31adb0` = L00 `0x31d2c8` (crate::moby::ai::attack_joint), `0x30f200` = L00
//! `0x311880` (the frame time), `0x31a248` = L00 `0x31c8e0` (crate::moby::classes::props::group_alive), `0x2ec0d0`
//! = L00 `0x2eeab0` (crate::moby::countdown), `0x322b18` = L00 `0x325100` (crate::moby::classes::shot::polar),
//! `0x319390` / `0x3193d0` / `0x319428` / `0x319488` / `0x3194f0` = L00 `0x31b9a0` / `0x31b9e0` / `0x31ba38` /
//! `0x31ba98` / `0x31bb00` (crate::rng), `0x30f008` = L00 `0x311688` (delete). `$gp+0x3498` (`0x1b2488`) is the
//! frame count.
//!
//! Not ported: the head aim (`0x324930` = L00 `0x327308` on `+0x2a0`, `$gp+0x2cd8` = 2.5).

use crate::hero::physics::{add, add_rot, atan2, dist2, rot_dist, turn_ease, V3};
use crate::moby::classes::death_fade;
use crate::moby::classes::props::{group_alive, set_seq, wrapped};
use crate::moby::classes::shot::polar;
use crate::moby::{ai, bolts, countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x10d0;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const T_BLOCK: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const T_FOUND: usize = 0x204;
pub(super) const HOME: usize = 0x210;
const AREA2: usize = 0x220;
const RANGE2: usize = 0x224;
const AREA1: usize = 0x228;
pub(super) const RANGE1: usize = 0x22c;
const TIMER: usize = 0x230;
const TIMER2: usize = 0x234;
const ALARM_T: usize = 0x238;
const ALARM: usize = 0x240;
const WAIT_GROUP: usize = 0x250;
const WAIT_MOBY: usize = 0x264;
const G268: usize = 0x268;
pub(super) const HELD: usize = 0x26c;
const OFFSET: usize = 0x270;
pub(super) const STRAIGHT: usize = 0x274;
const FOUND1: usize = 0x278;
const FOUND2: usize = 0x27c;
const BACK: usize = 0x280;
const FLEE: usize = 0x290;

/// `$gp+0x2cd4`: at most this many of them attacking at once (the pool `$gp+0x2cd0`, [`Globals`](super::Globals)).
pub(super) const MOST_ATTACKERS: i32 = 4;
/// The run's limits (the set-up's too): `0x32aad0(1/300, 0.1, 0.4°, 48°)`.
const RUN_SENSES: [u32; 4] = [0x3b5a_740e, 0x3dcc_ccce, 0x3be4_c38a, 0x3f56_7752];
const FLEE_SENSES: [u32; 4] = [0x3ada_740e, 0x3d4c_ccce, 0x3be4_c38a, 0x3f56_7752];
const LEAP_SENSES: [u32; 4] = [0x3ca3_d70a, 0x3f19_999a, 0x3be4_c38a, 0x3f56_7752];
const BACK_SENSES: [u32; 4] = [0x3cbf_258c, 0x3f4c_ccce, 0x3be4_c38a, 0x3f56_7752];
/// 30°: the facing the leap and the bite need.
const FACING: f32 = 0.523_598_85;

fn senses(w: &mut MobyWorld, id: usize, s: [u32; 4]) {
    ai::senses(&mut w.mobys[id], f32::from_bits(s[0]), f32::from_bits(s[1]), f32::from_bits(s[2]), f32::from_bits(s[3]));
}

/// `0x327928` (L00 `0x32a7f0`): the first time since the last sub-state change.
fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

fn solid_of(w: &MobyWorld) -> bool { w.class(CLASS).is_some_and(|c| c.has_collision) }

/// Drawn, shown, its class's collision, targeted.
fn show(w: &mut MobyWorld, id: usize) {
    let has = solid_of(w);
    let m = &mut w.mobys[id];
    m.drawn = 1;
    m.mode &= !1;
    m.has_collision = has;
    m.mode |= mode::TARGET;
}

/// `mode & 0xffbe | 0x1000` and its class's collision.
fn solid(w: &mut MobyWorld, id: usize) {
    let has = solid_of(w);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = has;
}

/// Not drawn, without collision, hidden and untargeted.
fn stow(m: &mut crate::moby::Moby) {
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 1;
}

/// Its target's place: the target moby's (`+0x200`), else the block's point.
fn target_pos(w: &MobyWorld, id: usize) -> V3 {
    let m = &w.mobys[id];
    ai::link(m, T_MOBY).and_then(|k| w.mobys.get(k)).map_or(m.pv_v(T_BLOCK), |t| t.pos)
}

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let q = w.mobys[id].pos;
    atan2(p[0] - q[0], p[1] - q[1])
}

/// `0x32aa68(yaw, m)` (L01 `0x323560`): the facing eased toward `yaw` at the movement block's turn limits.
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let (a, b, most) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, most, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

/// `0x329800(m, p)` (L01 `0x322450`): the move to `p`.
fn go_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) {
    let yaw = yaw_to(w, id, p);
    ai::move_step(w, id, ctx, yaw, p);
}

/// `0x329858(yaw, −1, −1, m, p, opts)` (L02 `0x331440`): the move to `p` with the movement options `opts` for the
/// tick (the limits −1: kept).
fn go_to_opts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3, opts: i32) {
    let yaw = yaw_to(w, id, p);
    let kept = w.mobys[id].pv_i(MV + 0x38);
    w.mobys[id].set_pv_i(MV + 0x38, opts);
    ai::move_step(w, id, ctx, yaw, p);
    w.mobys[id].set_pv_i(MV + 0x38, kept);
}

/// 0x10d0 (module docs).
pub fn update_10d0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) || ctx.hero.state == 0x6f { return; }
    if head(w, id, ctx) {
        w.delete(id);
        return;
    }
    if w.mobys[id].state == 1 { w.mobys[id].mode &= !mode::TARGET; }
    match w.mobys[id].sub_state {
        0 => waiting(w, id, ctx),
        1 => {
            let m = &w.mobys[id];
            if (m.pv_i(FOUND1) == 1 || m.pv_i(FOUND2) == 1) && w.level19.attackers < MOST_ATTACKERS {
                set_seq(w, id, ctx, 3, 0, 10);
                w.mobys[id].set_state(14, 2);
                w.level19.attackers += 1;
            }
        }
        2 => {
            if first_sub(&mut w.mobys[id]) {
                show(w, id);
                solid(w, id);
            }
            let m = &w.mobys[id];
            if m.pv_i(FOUND1) != 1 && m.pv_i(FOUND2) != 1 && m.state != 15 && m.state != 16 {
                w.mobys[id].set_state(7, 1);
                set_seq(w, id, ctx, 7, 0, 10);
                if ctx.game.rng.randi(3) == 0 {
                    set_seq(w, id, ctx, 6, 0, 10);
                } else if ctx.game.rng.randi(2) != 0 {
                    set_seq(w, id, ctx, 13, 0, 10);
                }
                flee(w, id, ctx);
            } else {
                show(w, id);
                solid(w, id);
            }
        }
        3 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                bolts::drop(w, id, 0, 3, ctx);
                death_fade::start(w, id, ctx, 0.0, None);
                w.level19.attackers -= 1;
            } else if f & 2 != 0 {
                if w.mobys[id].pv_i(T_FOUND) == 1 && w.level19.attackers <= MOST_ATTACKERS {
                    set_seq(w, id, ctx, 3, 0, 10);
                    w.mobys[id].set_state(14, 2);
                } else {
                    w.mobys[id].set_state(8, 1);
                    set_seq(w, id, ctx, 2, 0, 10);
                    flee(w, id, ctx);
                }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    // `0x41d5e8`: its timers and ticks.
    let m = &mut w.mobys[id];
    for o in [TIMER, TIMER2] {
        let mut t = m.pv_i(o);
        countdown(&mut t);
        m.set_pv_i(o, t);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    w.changed = true;
}

/// `0x41d300` (module docs): true when it is to be deleted.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 { setup(w, id, ctx); }
    if w.mobys[id].pv_b(STATUS) == 2 {
        w.level19.attackers -= 1;
        return true;
    }
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    if ctx.game.frame < 6 { return false; }
    let m = &w.mobys[id];
    let (home, a1, r1, a2, r2) = (m.pv_v(HOME), m.pv_i(AREA1), m.pv_f(RANGE1), m.pv_i(AREA2), m.pv_f(RANGE2));
    let f1 = ai::find_target(w, id, ctx, r1, 10.0, a1, Some(home), false);
    w.mobys[id].set_pv_i(FOUND1, f1 as i32);
    let f2 = ai::find_target(w, id, ctx, r2, 10.0, a2, Some(home), false);
    w.mobys[id].set_pv_i(FOUND2, f2 as i32);
    false
}

/// `0x41d300`'s set-up (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.level19.attackers = 0;
    let mirror = ctx.game.rng.randi(2) != 0;
    let m = &mut w.mobys[id];
    // `0x2ed5c8` (L00 `0x2effb8`): 15 and 14.4 as bytes.
    m.set_pv_b(0x48, 15);
    m.set_pv_b(0x29, 1);
    m.set_pv_b(0x4a, 14);
    if mirror { m.mode |= mode::MIRROR; }
    m.scale *= f32::from_bits(0x3fb9_999a);
    let p = m.pos;
    let z = ai::ground_z(ctx, 0.5, p, 0);
    let m = &mut w.mobys[id];
    m.pos[2] = z;
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode | 1) & !mode::TARGET;
    m.set_pv_i(G268, 0);
    let o = ctx.game.rng.randf_signed(5.0, 25.0) * 0.017_453_294;
    let m = &mut w.mobys[id];
    m.set_pv_f(OFFSET, o);
    let p = m.pos;
    m.set_pv_v(HOME, p);
    m.set_pv_b(REACT + 0x10, 0x19);
    m.set_pv_i(REACT + 0x48, 3);
    m.set_pv_b(REACT + 0x16, 9);
    m.set_pv_b(REACT + 0x11, 0xe);
    m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3e19_999a));
    m.set_pv_f(0x30, 1.5);
    m.set_pv_b(STATUS, 1);
    m.set_pv_f(REACT + 0x30, 0.5);
    m.set_pv_f(REACT + 0x34, 0.5);
    m.set_pv_b(REACT + 0x77, 1);
    if m.pv_s(MOST) == 0 {
        m.set_pv_s(MOST, 1);
        m.set_pv_f(HEALTH, 1.0);
    }
    m.set_pv_f(MV + 0x34, 0.5);
    m.set_pv_f(MV + 4, f32::from_bits(0x3f4c_cccd));
    m.set_pv_f(MV, 1.0);
    m.set_pv_f(MV + 0xc, 0.75);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc9_0fdc));
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
    m.set_pv_f(MV + 8, 0.75);
    senses(w, id, RUN_SENSES);
    w.mobys[id].set_state(1, 0);
    set_seq(w, id, ctx, 12, 0, 0);
}

/// `0x41d658`: each member of its group alarmed (`+0x240`).
fn alarm(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].group;
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for k in list {
        let Some(m) = w.mobys.get_mut(k) else { break };
        if ALARM + 4 <= m.pvars.len() { m.set_pv_i(ALARM, 1); }
    }
}

/// Sub 0 (module docs).
fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.pv_i(FOUND1) == 1 && m.pv_i(HELD) == 0 && w.level19.attackers < MOST_ATTACKERS {
        let k = m.pv_i(WAIT_MOBY);
        let out = if k == -1 {
            let g = m.pv_i(WAIT_GROUP);
            let none = g == 0xff || usize::try_from(g).ok().and_then(|g| w.groups.get(g)).is_none_or(|l| l.is_empty());
            none || group_alive(w, g) == 0
        } else {
            usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_none_or(|e| e.state == 0xfe || e.state == 0xfd)
        };
        if out {
            show(w, id);
            w.mobys[id].set_state(10, 2);
            set_seq(w, id, ctx, 0, 0, 10);
            w.level19.attackers += 1;
        }
        return;
    }
    let m = &mut w.mobys[id];
    if m.pv_i(ALARM) == 0 { return; }
    let mut t = m.pv_i(ALARM_T);
    let done = countdown(&mut t);
    m.set_pv_i(ALARM_T, t);
    if !done { return; }
    m.set_pv_i(ALARM, 0);
    if m.pv_i(TIMER) < 300 {
        let t = ctx.game.rng.rand_range(480, 600);
        w.mobys[id].set_pv_i(TIMER, t);
    }
}

/// The flight's goal (`+0x290`: home and 6..8 a random way), `+0x230` 580..600, one attacker less.
fn flee(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let d = ctx.game.rng.randf(6.0, 8.0);
    let a = ctx.game.rng.rand_angle();
    let home = w.mobys[id].pv_v(HOME);
    w.mobys[id].set_pv_v(FLEE, add(home, polar(d, a, 0.0)));
    let t = ctx.game.rng.rand_range(580, 600);
    w.mobys[id].set_pv_i(TIMER, t);
    w.level19.attackers -= 1;
}

/// The recovery (24, sequence 7; 1 in 3 13 from frame 41, else 6).
fn recover(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    set_seq(w, id, ctx, 7, 0, 10);
    w.mobys[id].set_state(24, -1);
    if ctx.game.rng.randi(3) != 0 {
        set_seq(w, id, ctx, 6, 0, 10);
    } else if ctx.game.rng.randi(2) != 0 {
        set_seq(w, id, ctx, 13, 41, 10);
    }
}

/// The run (14, sequence 3 with `blend`).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8) {
    set_seq(w, id, ctx, 3, 0, blend);
    w.mobys[id].set_state(14, -1);
}

/// The sphere at joint 0 (`0x31adb0(0.75, damage, 1, m, 0, 1, 0, 1, 0)`).
fn strike(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let d = w.mobys[id].pv_b(DAMAGE) as f32;
    ai::attack_joint(w, id, ctx, 0.75, d, 1.0, 0, 1, 0, 1, 0);
}

/// Facing the target block's point within 30°.
fn facing(w: &MobyWorld, id: usize) -> bool {
    let p = w.mobys[id].pv_v(T_BLOCK);
    rot_dist(w.mobys[id].rot[2], yaw_to(w, id, p)) < FACING
}

/// The state (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        3 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 12, 0, 10);
                let m = &mut w.mobys[id];
                m.set_state(1, 0);
                stow(m);
            }
        }
        7 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(8, -1);
                set_seq(w, id, ctx, 2, 0, 13);
            }
        }
        8 => {
            if w.mobys[id].first() { senses(w, id, FLEE_SENSES); }
            let goal = w.mobys[id].pv_v(FLEE);
            go_to(w, id, ctx, goal);
            if dist2(w.mobys[id].pos, goal) < 0.5 {
                w.mobys[id].set_state(3, -1);
                set_seq(w, id, ctx, 1, 0, 10);
            }
        }
        10 => {
            let yaw = yaw_to(w, id, target_pos(w, id));
            face(w, id, yaw);
            if ai::reached(w, id, ctx, 23.0) {
                set_seq(w, id, ctx, 7, 1, 2);
                if ctx.game.rng.randi(2) != 0 { set_seq(w, id, ctx, 6, 0, 2); }
                w.mobys[id].set_state(11, -1);
                if w.mobys[id].pv_i(STRAIGHT) != 0 || ctx.game.rng.randi(2) == 0 { run(w, id, ctx, 6); }
            }
        }
        11 => {
            let yaw = yaw_to(w, id, target_pos(w, id));
            face(w, id, yaw);
            if wrapped(w, id, ctx) { run(w, id, ctx, 10); }
        }
        14 => {
            if w.mobys[id].first() {
                senses(w, id, RUN_SENSES);
                solid(w, id);
            }
            let d = dist2(w.mobys[id].pos, target_pos(w, id));
            if d < 7.0 && 4.0 < d && facing(w, id) {
                senses(w, id, LEAP_SENSES);
                set_seq(w, id, ctx, 4, 0, 10);
                w.mobys[id].set_state(15, -1);
            } else if d <= 3.0 && facing(w, id) {
                set_seq(w, id, ctx, 10, 0, 10);
                w.mobys[id].set_state(16, -1);
            } else {
                let (o, goal) = (w.mobys[id].pv_f(OFFSET), w.mobys[id].pv_v(T_BLOCK));
                ai::move_toward(w, id, ctx, o, goal);
            }
        }
        15 => {
            if w.mobys[id].first() { solid(w, id); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if 4.0 < f && f < 15.0 {
                let goal = w.mobys[id].pv_v(T_BLOCK);
                go_to(w, id, ctx, goal);
            }
            if ai::reached(w, id, ctx, 30.0) {
                recover(w, id, ctx);
                if ctx.game.rng.randi(5) != 0 {
                    set_seq(w, id, ctx, 5, 0, 10);
                    w.mobys[id].set_state(17, -1);
                }
            }
            if f < 19.0 { strike(w, id, ctx); }
        }
        16 => {
            if w.mobys[id].first() { solid(w, id); }
            if ai::reached(w, id, ctx, 25.0) {
                recover(w, id, ctx);
                if ctx.game.rng.randi(3) != 0 {
                    set_seq(w, id, ctx, 5, 0, 10);
                    w.mobys[id].set_state(17, -1);
                }
            }
            if ctx.host.frame_time(&w.mobys[id]) < 14.0 { strike(w, id, ctx); }
        }
        17 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                let a = add_rot(m.rot[2], f32::from_bits(0x4049_0fdc));
                let v = [a.cos() * 18.0, a.sin() * 18.0, 0.0];
                let p = add(v, m.pos);
                m.set_pv_v(BACK, p);
                senses(w, id, BACK_SENSES);
            }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if 4.0 < f && f < 14.0 {
                let goal = w.mobys[id].pv_v(BACK);
                go_to_opts(w, id, ctx, goal, 0x10);
            }
            if ai::reached(w, id, ctx, 21.0) { recover(w, id, ctx); }
        }
        24 => {
            if wrapped(w, id, ctx) { run(w, id, ctx, 10); }
        }
        _ => {}
    }
}
