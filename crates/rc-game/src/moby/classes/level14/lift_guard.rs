//! The prison's lift guards **0x112c** (5 placed; `0x42a448`, the decomp's `func_L14_00428E68`; its set-up and checks
//! `0x42abb0`, its tail `0x42aed0`, its death `0x42aff8`): 0x0b0b's body (its two spinning parts at `+0x300` /
//! `+0x304`, crate::moby::classes::enemies, and its glows, crate::fx's `0x3dc220`) on the framework
//! ([`crate::moby::ai`]): health `+0x20`, movement `+0x50`, reaction `+0x140`, target `+0x210`, head aim `+0x230`.
//! Each keeps a prison lift (crate::moby::classes::level14::lift, driven: [`lift::drive`] / [`lift::set_end`]) and
//! rides it after Ratchet.
//!
//! Its words: `+0x2b0` its area, `+0x2b4` its sight's range (4), `+0x2b8` its lift (a moby index, −1 none), `+0x2bc`
//! an area it waits for Ratchet in (−1 none), `+0x2d0` its last place, `+0x2e0` home, `+0x2f0` it sees him, `+0x2fc`
//! its parts' spin, `+0x30c` / `+0x310` timers, `+0x31c` it is up (at home, 1 at the start), `+0x320` its swipe still
//! armed.
//!
//! **Set-up** (state 0): its limits (as the module's pvars below), its turn's weave ±15° at random (`$gp+0x28b8`),
//! home its place, its range 4, its parts made (`0x42dfd0`), health 3 (of 3); 0.5 up and put on the floor under it
//! (`0x32e258`, crate::moby::ai::floor_under). With no waiting area idle (1, sub 0, sequence 2), else waiting (7, sub
//! 3, sequence 0). Its lift taken over (`0x3e0b68`).
//!
//! **Each tick, first** (`0x42abb0`): its shadow probed within 24; the frame's damage (`0x32fcf0`) with no health
//! left: its death; asked to go (`+0x2e` = 2), its parts gone; it goes. Else Ratchet sought (`0x333c18`: its range,
//! 1.5 high, its area).
//!
//! **Sub-states**: 0 seeing Ratchet while idle (1): the swipe (5, sub 1). 1 its first tick the limits (turn 20/3600 a
//! tick², 6/60 most; walk 720″, 1440·1/3438); losing him: idle (1, sub 0), `+0x30c` `rand_range(580, 600)`. 2 the
//! hurt's flight (`0x330a38`): dead → its death; over → the swipe if it sees him, else idle as above.
//!
//! **States**: 1 idle (sequence 6; at a sequence's end one of 6 / 7 / 2 / 0x11, `$gp+0x28a8`): with a lift held
//! still (`0x335f98`); one tick in 16 (its slot) with a lift: up and Ratchet more than 2 under it within 16 across →
//! 2; down and he is more than 2 over it within 8 → 3; the other ticks, more than 45° off facing him → 4. 4 (sequence
//! 8): turning to face him (`0x336de8`); within 3° → 1. 2 (sequence 8): its first tick its lift sent down; toward the
//! lift (`0x335c00`) until there (flag 0x80): the lift sent up, down (`+0x31c` 0), 1. 3 (sequence 8): its first tick
//! its lift sent down; home until there: up, 1. 5 the swipe (sequence 0x10 from frame 7, blend 7): before frame 19
//! facing its target, armed; frames 19..22 while armed spheres of 0.57 (damage 2, flags 1) at its two parts (push
//! 1.5) and its joint 6 (push 2): one of a 0x0964 robot's hit by the last disarms it. 7 waiting until Ratchet is in
//! its area `+0x2bc` → 8 (sequence 1, blend 2) to its end → idle (1, sub 0).
//!
//! **The tail** (`0x42aed0`): its timers, its state's ticks, its glows (`0x42e200`), its parts spun (`0x42e0b0`: 12°
//! a tick in sub-state 1, else 2/3°), its head aim (`0x32f270`: list 7, 2.0, crate::moby::joint_aim::head).
//!
//! **Its death** (`0x42aff8`): its parts gone, its bolts (3), a blast 0.75 up its frame (0.6: bursts 0x19 / 0x14 /
//! 0x14, 20 puffs, 45 streaks, the flash), its sound 6 and its pieces (mask 0x3f8, 3..6 / 3..6, along its knock at
//! 0.05).
//!
//! The L14 engine functions: `0x333670` = L00 `0x32a7f0` (the sub-state's first tick), `0x336f28` = L00 `0x32dcf0`
//! (ai::senses), `0x330a38` = L00 `0x328b80` (ai::flight), `0x335f98` = L00 `0x32ce98` (ai::hold), `0x335c00` = L01
//! `0x322450` (ai::move_step toward a point), `0x336de8` = L01 `0x323560` (ai::face), `0x324b00` = L00 `0x31d180`
//! (ai::attack_sphere), `0x31a198` = L00 `0x3127c8` (the joint's place), `0x3336c8` = L00 `0x32a848` (in the area),
//! `0x32fcf0` = L00 `0x327e38` (ai::take_damage), `0x333c18` = L00 `0x32ad38` (ai::find_target), `0x42dfd0` /
//! `0x42e0b0` / `0x42e190` = L00 `0x3dbed0` / `0x3dbfb0` / `0x3dc090` (enemies' parts), `0x322740` = L00 `0x31adc0`
//! (crate::moby::bolts::drop), `0x405740` = L00 `0x3f8348` (crate::moby::blast::burst), `0x32f400` = L00 `0x327498`
//! (crate::moby::shards::break_into).
//!
//! Not ported: its parts' sparks while it hunts (`0x42e0b0`'s one in six, `0x3258c0`; as 0x0b0b's), the group stamp
//! of its glows (`0x42e200`'s `+0x308`).

use crate::hero::physics::{add, atan2, dist2, rot_dist, scale, set_len, V3};
use crate::moby::classes::enemies;
use crate::moby::classes::level14::lift;
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x112c;

const MV: usize = 0x50;
const TARGET: usize = 0x210;
const HEAD_AIM: usize = 0x230;
const AREA: usize = 0x2b0;
const RANGE: usize = 0x2b4;
const LIFT: usize = 0x2b8;
const WAIT_AREA: usize = 0x2bc;
const LAST: usize = 0x2d0;
const HOME: usize = 0x2e0;
const SEEN: usize = 0x2f0;
const SPIN: usize = 0x2fc;
const PARTS: usize = 0x300;
const TIMER: usize = 0x30c;
const UP: usize = 0x31c;
const ARMED: usize = 0x320;

/// `$gp+0x28a8` (`0x1b1898`): its idles.
const IDLES: [u8; 4] = [6, 7, 2, 0x11];
/// The robot its swipe stops at (crate::moby::classes::endako_d::ROBOT_964).
const ROBOT: i16 = 0x0964;

fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let q = w.mobys[id].pos;
    atan2(p[0] - q[0], p[1] - q[1])
}

/// Its lift, when it has one.
fn lift_of(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(LIFT)).ok().filter(|&l| l < w.mobys.len())
}

/// The sequence `s` from `frame` unless it plays.
fn play(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: u8) {
    if seq(w, id, ctx) != s { set_seq(w, id, ctx, s, frame, blend); }
}

/// `0x42abb0` (module docs). Returns whether it is to go.
fn checks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let weave = if ctx.game.rng.randi(2) != 0 { -15.0 } else { 15.0 } * 0.017_453_294;
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 9);
        m.set_pv_b(0x150, 6);
        m.set_pv_f(0x17c, 0.15);
        m.set_pv_i(0x188, 2);
        m.set_pv_f(0x170, 1.0);
        m.set_pv_f(0x174, 1.0);
        m.set_pv_f(0x180, m.pos[2] - 3.0);
        m.set_pv_f(MV + 4, 0.3);
        m.set_pv_f(MV, 1.0);
        m.set_pv_f(MV + 0x28, f32::from_bits(0x3dcc_ccce));
        m.set_pv_f(MV + 0x24, f32::from_bits(0x3ba3_d70a));
        m.set_pv_f(MV + 0x18, f32::from_bits(0x3b64_c38a));
        m.set_pv_f(MV + 0x10, std::f32::consts::PI);
        m.set_pv_f(MV + 0x20, f32::from_bits(0x3ba3_d70a));
        m.set_pv_f(MV + 0x14, f32::from_bits(0x3b64_c38a));
        m.set_pv_f(MV + 0xdc, weave);
        m.set_pv_s(MV + 0xe2, 30);
        m.set_pv_b(0x2e, 1);
        m.set_pv_s(MV + 0xe4, 90);
        let home = m.pos;
        m.set_pv_v(HOME, home);
        m.set_pv_i(UP, 1);
        m.set_pv_f(RANGE, 4.0);
        enemies::make_parts(w, id, id, PARTS, ctx);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x20, 3.0);
        m.set_pv_s(0x24, 3);
        m.pos[2] += 0.5;
        let p = m.pos;
        w.mobys[id].pos = ai::floor_under(ctx, id, p);
        if w.mobys[id].pv_i(WAIT_AREA) == -1 {
            w.mobys[id].set_state(1, 0);
            play(w, id, ctx, 2, 0, 0);
        } else {
            w.mobys[id].set_state(7, 3);
            play(w, id, ctx, 0, 0, 0);
        }
        if let Some(l) = lift_of(w, id) { lift::drive(w, l, ctx); }
        w.changed = true;
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if ai::take_damage(w, id, ctx, false) != 0 && w.mobys[id].pv_f(0x20) <= 0.0 {
        die(w, id, ctx);
        return true;
    }
    if w.mobys[id].pv_b(0x2e) == 2 {
        enemies::delete_parts(w, id, PARTS);
        return true;
    }
    let (range, area) = (w.mobys[id].pv_f(RANGE), w.mobys[id].pv_i(AREA));
    let seen = ai::find_target(w, id, ctx, range, 1.5, area, None, false);
    w.mobys[id].set_pv_i(SEEN, seen as i32);
    false
}

/// `0x42aff8` (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    enemies::delete_parts(w, id, PARTS);
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    let m = &w.mobys[id];
    let at = add(m.pos, scale(m.rows[2], 0.75));
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3f19_999a), bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false };
    crate::moby::blast::burst(w, ctx, at, &b);
    ctx.host.play_sound(&w.mobys[id], 6, 0);
    let v = set_len(w.mobys[id].pv_v(0x1c0), f32::from_bits(0x3d4c_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3f8, v, 0xff);
}

/// Idle (1, sub 0) with `+0x30c` `rand_range(580, 600)`.
fn idle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(1, 0);
    let t = ctx.game.rng.rand_range(0x244, 600);
    w.mobys[id].set_pv_i(TIMER, t);
}

/// 0x112c (module docs).
pub fn update_112c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if checks(w, id, ctx) {
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(SEEN) == 1 && m.state == 1 { m.set_state(5, 1); }
        }
        1 => {
            if w.mobys[id].first_sub() {
                ai::senses(&mut w.mobys[id], 20.0 * 0.000_277_777_78, 6.0 * 0.016_666_668, 720.0 * 4.848_137_5e-6, 1440.0 * 0.000_290_888_25);
            }
            if w.mobys[id].pv_i(SEEN) != 1 { idle(w, id, ctx); }
        }
        2 => {
            let res = ai::flight(w, id, ctx);
            if res & 1 != 0 {
                die(w, id, ctx);
                w.delete(id);
                return;
            }
            if res & 2 != 0 {
                if w.mobys[id].pv_i(SEEN) != 1 { idle(w, id, ctx) } else { w.mobys[id].set_state(5, 1) }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    tail(w, id, ctx);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hero = ctx.hero.pos;
    let next = match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { play(w, id, ctx, 6, 0, 0x14); }
            if wrapped(w, id, ctx) {
                let r = IDLES[ctx.game.rng.randi(4) as usize];
                if seq(w, id, ctx) != r {
                    let s = IDLES[ctx.game.rng.randi(4) as usize];
                    ctx.host.set_anim(&w.mobys[id], s, 0, 0x14);
                }
            }
            let lift = lift_of(w, id);
            if lift.is_some() { ai::hold(w, id, ctx); }
            if ctx.game.frame & 0xf == id as i32 & 0xf {
                if lift.is_none() { return; }
                let m = &w.mobys[id];
                let (z, d) = (m.pos[2], dist2(m.pos, hero));
                if m.pv_i(UP) != 0 {
                    if hero[2] + 2.0 < z && d < 16.0 { 2 } else { return }
                } else if z < hero[2] - 2.0 && d < 8.0 {
                    3
                } else {
                    return;
                }
            } else {
                let yaw = yaw_to(w, id, hero);
                if rot_dist(w.mobys[id].rot[2], yaw) <= 0.785_398_24 { return; }
                4
            }
        }
        4 => {
            if w.mobys[id].first() { play(w, id, ctx, 8, 0, 0x14); }
            let yaw = yaw_to(w, id, hero);
            ai::face(&mut w.mobys[id], yaw);
            if 0.052_359_883 <= rot_dist(w.mobys[id].rot[2], yaw) { return; }
            1
        }
        2 => {
            let Some(l) = lift_of(w, id) else { return };
            if w.mobys[id].first() {
                play(w, id, ctx, 8, 0, 0x14);
                lift::set_end(w, l, 0);
            }
            let g = w.mobys[l].pos;
            let yaw = yaw_to(w, id, g);
            if ai::move_step(w, id, ctx, yaw, g) & 0x80 == 0 { return; }
            lift::set_end(w, l, 1);
            w.mobys[id].set_pv_i(UP, 0);
            1
        }
        3 => {
            let lift = lift_of(w, id);
            if w.mobys[id].first() {
                play(w, id, ctx, 8, 0, 0x14);
                if let Some(l) = lift { lift::set_end(w, l, 0); }
            }
            let g = w.mobys[id].pv_v(HOME);
            let yaw = yaw_to(w, id, g);
            if ai::move_step(w, id, ctx, yaw, g) & 0x80 == 0 { return; }
            w.mobys[id].set_pv_i(UP, 1);
            1
        }
        5 => {
            if w.mobys[id].first() { play(w, id, ctx, 0x10, 7, 7); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if f < 19.0 {
                let t = w.mobys[id].pv_v(TARGET);
                let yaw = yaw_to(w, id, t);
                ai::face(&mut w.mobys[id], yaw);
                w.mobys[id].set_pv_i(ARMED, 1);
            } else if f <= 22.0 && w.mobys[id].pv_i(ARMED) != 0 {
                let r = f32::from_bits(0x3f11_eb85);
                for p in enemies::parts(w, id, PARTS).into_iter().flatten() {
                    let at = w.mobys[p].pos;
                    ai::attack_sphere(w, id, ctx, r, 2.0, 1.5, at, 1, 0, 1, 0);
                }
                if let Some(at) = ctx.host.joint(&w.mobys[id], 6) {
                    let n = ai::attack_sphere(w, id, ctx, r, 2.0, 2.0, at, 1, 0, 1, 0);
                    // The hit list `0x1c3940` the last sphere left.
                    let robot = 0 < n && ctx.scene.is_some_and(|sc| {
                        crate::collision::mobys::coll_sphere_mobys(sc, at, r, crate::collision::QueryFlags(0), Some(id)).into_iter().any(|o| w.mobys.get(o).is_some_and(|o| o.o_class == ROBOT))
                    });
                    if robot { w.mobys[id].set_pv_i(ARMED, 0); }
                }
            }
            return;
        }
        7 => {
            if !ctx.level.in_area(w.mobys[id].pv_i(WAIT_AREA), hero) { return; }
            8
        }
        8 => {
            if w.mobys[id].first() { play(w, id, ctx, 1, 0, 2); }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(1, 0); }
            return;
        }
        _ => return,
    };
    w.mobys[id].set_state(next, -1);
}

/// `0x42aed0` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for o in [TIMER, TIMER + 4] {
        let mut t = m.pv_i(o);
        crate::moby::countdown(&mut t);
        m.set_pv_i(o, t);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    // `0x42e200` → `0x2e7920`: its glows (crate::fx).
    w.register_draw(id);
    let m = &mut w.mobys[id];
    let pos = m.pos;
    m.set_pv_v(LAST, pos);
    let step = if m.sub_state == 1 { f32::from_bits(0x3e56_7752) } else { f32::from_bits(0x3c3e_a2f4) };
    let a = crate::hero::physics::add_rot(m.pv_f(SPIN), step);
    m.set_pv_f(SPIN, a);
    enemies::spin_parts(w, id, id, PARTS, ctx, a);
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 7, 2.0);
    w.changed = true;
}
