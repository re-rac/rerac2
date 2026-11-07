//! Yeedil's biter **0x119e** (`0x440b08`, 20 placed; the decomp's v2.00 `func_US2_L20_00440B08`, v1.01
//! `func_L20_0043F570`): a small creature kept hidden in a pool (state 6) until the level's thrower 0x11e0 throws one
//! out (`0x4416d8`, state 7); it lands, circles Ratchet and lunges at him with a bite. On the framework
//! (crate::moby::ai): health `+0x20` (most `+0x24`, its status byte `+0x2e`: 2 sends it back to the pool), movement
//! `+0x50` (radius 0.5, height 0.3, steps 0.4 / 2, slope 90°, the turn left 45°, stop 0.05, the fall kill; its weave
//! `+0x12c` ±18..22° flipped every 120..180 ticks), reaction `+0x140` (a hurt to state 5, sub 2; radius 0.5, centre
//! 0.6, bounce 0.15, its kill height the level's death height), target block `+0x1f0` (`+0x200` the moby, `+0x204`
//! found; the search watches the area `+0x270`).
//!
//! **Its pvars:** `+0x210` the flight's body (a break piece's block, crate::moby::shards: `+0x210` its velocity),
//! `+0x230` where it stands up from the flight (in its frame), `+0x280` attacking (counted by [`may_attack`]),
//! `+0x284` its circling's side, `+0x288` the flight's spin (degrees a second), then the circling's offset.
//!
//! **Each tick** ([`head`], `0x441208`): the set-up in state 0 ([`setup`]); its kill height; out of the pool, status 2
//! sends it back ([`stow`]), the status 1; in the flight with health its hits taken off it (flags 0x10000, dead: no
//! longer attacking, its bolts, the fade with [`stow`] after), else the frame's damage (`0x33d250` = L00 `0x327e38`,
//! crate::moby::ai::take_damage); the target search (`0x340018` = L00 `0x32ad38`, 64, the area); nothing found, its
//! target is Ratchet anyway. A scene (`0x1a8f00` = 2) stops it. Then the sub-state: sub 1 on its first tick picks
//! the circling's side; sub 2 the hurt's flight (`0x33df98` = L00 `0x328b80`, crate::moby::ai::flight): dead, as a
//! death in the flight, knocked out of it to 3 (sub 1).
//!
//! **States** (`+0x20`): **1** standing up: on its first tick shown, with collision and targeted, moved by `+0x230`,
//! on the ground, facing Ratchet, sequence 3; at its end 2 (sub 1). **2** circling: sequence 2, its speed 7 a second;
//! offset 60° (120° within 4 of Ratchet, back to 60° past 8) on its side, swapped every 60 stuck ticks; on its frame
//! slice (one in 16) it attacks when [`may_attack`] lets it (3). **3** the run in: sequence 2, speed 5; within 3, 4.
//! **4** the bite: sequence 4 from frame 2; between its frames 9 and 17 the lunge at 60 a second (120 a second²) to
//! Ratchet, else facing him; from frame 14 the bite (a 0.333 sphere 1.5 ahead and 0.5 up: 2 damage, flags
//! 0x4000001); at its end, past 3 of him, 3. **5** hurt. **6** in the pool (hidden, no collision). **7** thrown: the
//! flight's body (the throw's spin turning its velocity about the up); a 0.5 sphere about it hits for 5 (flags
//! 0x4000000); once its speed across falls under 0.05 it slows by 1/360 a tick to a stop: 1 (sub 1); under its kill
//! height back to 6.
//!
//! **The tail** (`0x441690`): its state ticks; the head aim (`0x33c3d8` = L00 `0x327308` on `+0x2b0`, list 2,
//! `$gp+0x2fe4` = 2.5: crate::moby::joint_aim::head).
//!
//! The engine functions with the port's: `0x330820` = L00 `0x31b8d0` (crate::moby::Moby::set_state), `0x330540` =
//! L00 `0x31b5f0` (set_anim), `0x33faa8` / `0x33fad0` = L00 `0x32a7c8` / `0x32a7f0` (the first tick of the state /
//! sub), `0x341c98` = L00 `0x32cc30` (crate::moby::ai::move_toward), `0x341b68` (L01 `0x322450`: the move to a point,
//! crate::moby::ai::move_step), `0x342dd0` (L01 `0x323560`: the facing eased at the turn limits), `0x338e28` = L00
//! `0x323bf8` (crate::moby::ai::reached), `0x332230` = L00 `0x31d180` (crate::moby::ai::attack_sphere), `0x3267e0` =
//! L00 `0x311880` (the frame time), `0x32fd10` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x40b020` = L01
//! `0x3eb1d0` (crate::moby::classes::death_fade), `0x331ed0` = L00 `0x31ce20` (crate::moby::ai::ground_z),
//! `0x333c58` = L00 `0x31eb18` (approach), `0x3309e8` = L00 `0x31ba98` (randf_signed), `0x3308f0` = L00 `0x31b9a0`.
//!
//! **The throw** ([`throw`], `0x4416d8`, from the boss 0x11e0's state 12, crate::moby::classes::level20::boss): the
//! first of the group in the pool, at the throw's place facing Ratchet, shown with its collision; sequence 7 cut in,
//! state 7 (sub 3); its body (`0x2efff8` = L00 `0x2dbff8` mode 3 on `+0x210`, crate::moby::shards::whole_ball: gravity
//! 15, the throw's velocity, a spin of sin 6° about `up × velocity`, no sound) and the throw's spin `+0x288`; its
//! collision back and targeted. In state 7 the body flies (`0x2f01f0` = L00 `0x2dc1f0`, crate::moby::shards::ball_at).
//!
//! Not ported: in a scene the deletion once the loaded part (`0x1bdc70`, set by `0x319ba8`) is 5 or more; the moby word `+0x9c` = 1 of the throw. The frame slice uses its
//! index for the moby's address `>> 8`.

use crate::hero::physics::{add, approach, atan2, dist2, euler_rows, len2, set_len2, through, V3};
use crate::moby::classes::death_fade;
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, bolts, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x119e;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const STATUS: usize = 0x2e;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const T_BLOCK: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const FOUND: usize = 0x204;
const BODY: usize = 0x210;
const STAND: usize = 0x230;
const AREA: usize = 0x270;
const ATTACKING: usize = 0x280;
const SIDE: usize = 0x284;
const SPIN: usize = 0x288;
/// The head aim's record (crate::moby::joint_aim).
const HEAD_AIM: usize = 0x2b0;
/// `$gp+0x2fe4`: the head's scale with the enemies' big heads.
const HEAD_SCALE: f32 = 2.5;

/// `$gp+0x2fa0` / `+0x2fa4` / `+0x2fa8`: the circling's speed and the run's (a second), the walk's gain (a second²).
const CIRCLE_SPEED: f32 = 7.0;
const SPEED: f32 = 5.0;
const ACCEL: f32 = 20.0;
/// `$gp+0x2fb0` / `0x1b1fa4`: within this of Ratchet it circles wide (120°); past that, back to 60°.
const NEAR: f32 = 4.0;
const FAR: f32 = 8.0;
/// `$gp+0x2fb8`: the bite's reach.
const REACH: f32 = 3.0;
/// `$gp+0x2fbc`: the bite sequence's first frame.
const BITE_FRAME: f32 = 2.0;
/// `$gp+0x2fc0` / `+0x2fc4`: the lunge's speed (a second) and gain (a second²).
const LUNGE_SPEED: f32 = 60.0;
const LUNGE_ACCEL: f32 = 120.0;
/// `$gp+0x2fd0`: the bite's centre in its frame.
const BITE_AT: V3 = [1.5, 0.0, 0.5];
/// `$gp+0x2fe0`: at most this many of its group attacking, or nearer Ratchet, before it may attack.
const MOST_AHEAD: i32 = 5;
const DEG60: f32 = f32::from_bits(0x3f86_0a93);
const DEG120: f32 = f32::from_bits(0x4006_0a93);

/// The playing sequence (`+0x43`), and whether it is not blending (`+0x42` = `+0x43`).
fn seq_b(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { seq(w, id, ctx) }
fn settled(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == a.seq_b) }

/// Hidden, without collision, untargeted.
fn hide(m: &mut crate::moby::Moby) {
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x41;
}

/// `0x4419a0`: back into the pool — the reaction's options cleared, full health, hidden, state 6 (sub 3). Also the
/// fade's callback.
fn stow(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_s(REACT + 0x38, 0);
    let most = m.pv_s(MOST) as f32;
    m.set_pv_f(HEALTH, most);
    hide(m);
    m.set_state(6, 3);
    w.changed = true;
}

/// Dead: no longer attacking, its bolts (`0x32fd10(m, 0, 3)`), the fade and back to the pool.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(ATTACKING, 0);
    bolts::drop(w, id, 0, 3, ctx);
    death_fade::start(w, id, ctx, 0.0, Some(stow as crate::moby::UpdateFn));
}

/// `0x441208`'s set-up (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let weave = ctx.game.rng.randf_signed(18.0, 22.0) * 0.017_453_294;
    let m = &mut w.mobys[id];
    m.set_pv_f(MV, 0.5);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc9_0fdc));
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3f49_0fdc));
    m.set_pv_f(REACT + 0x30, 0.5);
    m.set_pv_b(REACT + 0x10, 5);
    m.set_pv_i(REACT + 0x48, 2);
    m.set_pv_f(REACT + 0x34, 0.6);
    m.set_pv_f(MV + 0x34, 0.05);
    m.set_pv_f(MV + 4, 0.3);
    m.set_pv_f(MV + 8, 0.4);
    m.set_pv_f(MV + 0xc, 2.0);
    m.set_pv_f(REACT + 0x3c, 0.15);
    m.set_pv_s(MV + 0xe2, 0x78);
    m.set_pv_s(MV + 0xe4, 0xb4);
    m.set_pv_f(MV + 0xdc, weave);
    m.set_pv_f(MV + 0x28, SPEED * 0.016_666_668);
    m.set_pv_f(MV + 0x24, ACCEL * 0.000_277_777_78);
    m.set_pv_f(MV + 0x20, ACCEL * 0.000_277_777_78);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x200);
    // `0x304128` (L00 `0x2effb8`): 8 and 12 as bytes.
    m.set_pv_b(0x48, 8);
    m.set_pv_b(0x4a, 12);
    m.set_pv_b(0x29, 1);
    if m.pv_s(MOST) == 0 {
        m.set_pv_s(MOST, 3);
        m.set_pv_f(HEALTH, 3.0);
    }
    hide(m);
    m.set_state(6, 3);
    if seq_b(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
}

/// `0x441208` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { setup(w, id, ctx); }
    let kill = ctx.game.death_z.unwrap_or(f32::MIN);
    let m = &mut w.mobys[id];
    m.set_pv_f(REACT + 0x40, kill);
    if m.state == 6 { return; }
    if m.pv_b(STATUS) == 2 {
        hide(m);
        m.set_state(6, 3);
    }
    let m = &mut w.mobys[id];
    m.set_pv_b(STATUS, 1);
    if m.state == 7 && 0.0 < m.pv_f(HEALTH) {
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
        let dmg = ai::damage_of(w, id, hit.as_ref(), false);
        let m = &mut w.mobys[id];
        let mut hp = m.pv_f(HEALTH);
        approach(0.0, dmg, &mut hp);
        m.set_pv_f(HEALTH, hp);
        if hp <= 0.0 {
            die(w, id, ctx);
            w.clear_hit(id);
        }
    } else {
        ai::take_damage(w, id, ctx, false);
    }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 64.0, 0.0, area, None, false);
    let m = &mut w.mobys[id];
    if m.pv_i(FOUND) == 0 {
        ai::set_link(m, T_MOBY, ctx.hero_moby);
        m.set_pv_v(T_BLOCK, ctx.hero.pos);
        m.set_pv_i(FOUND, 1);
    }
}

/// `0x441530`: it may attack — fewer than [`MOST_AHEAD`] others of its group (out of the pool) attacking, and fewer
/// nearer Ratchet than it.
fn may_attack(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let Some(list) = usize::try_from(m.group).ok().and_then(|g| w.groups.get(g)) else { return true };
    let d = dist2(m.pos, ctx.hero.pos);
    let (mut busy, mut nearer) = (0, 0);
    for &k in list {
        let Some(o) = w.mobys.get(k) else { break };
        if k == id || o.state == 6 { continue; }
        if o.pv_i(ATTACKING) != 0 { busy += 1; }
        if MOST_AHEAD <= busy { return false; }
        if dist2(o.pos, ctx.hero.pos) < d { nearer += 1; }
        if MOST_AHEAD <= nearer { return false; }
    }
    true
}

fn yaw_to_hero(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 {
    let (p, h) = (w.mobys[id].pos, ctx.hero.pos);
    atan2(h[0] - p[0], h[1] - p[1])
}

fn set_walk(w: &mut MobyWorld, id: usize, speed: f32, accel: f32) {
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0x20, accel * 0.000_277_777_78);
    m.set_pv_f(MV + 0x28, speed * 0.016_666_668);
    m.set_pv_f(MV + 0x24, accel * 0.000_277_777_78);
}

/// `0x4416d8(spin, group, pos, vel)`: the first of `group` in the pool (state 6) thrown out (module docs); None when
/// none is.
pub fn throw(w: &mut MobyWorld, ctx: &mut Ctx, group: i32, pos: V3, vel: V3, spin: f32) -> Option<usize> {
    let list = usize::try_from(group).ok().filter(|_| group & 0xff != 0xff).and_then(|g| w.groups.get(g))?;
    let id = list.iter().copied().take_while(|&k| k < w.mobys.len()).find(|&k| w.mobys[k].state == 6)?;
    let h = ctx.hero.pos;
    let coll = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.rot[2] = atan2(h[0] - pos[0], h[1] - pos[1]);
    m.mode &= 0xefbe;
    m.has_collision = coll;
    ctx.host.set_anim(&w.mobys[id], 7, 0, 0);
    w.mobys[id].set_state(7, 3);
    w.rebuild(id, &*ctx.host);
    // `0x302f98`: up (`vf0.zwxy`), across the velocity; `0x336d50`: the quaternion of sin 6° about it.
    let axis = crate::hero::physics::cross([0.0, 0.0, 1.0], vel);
    let q = crate::quat::axis_quat(f32::from_bits(0x3dd6_7752), axis);
    crate::moby::shards::whole_ball(w, id, BODY, 15.0, vel, q, 0xff);
    let m = &mut w.mobys[id];
    m.set_pv_f(SPIN, spin);
    m.has_collision = coll;
    m.mode |= mode::TARGET;
    w.changed = true;
    Some(id)
}

/// 0x119e (module docs).
pub fn update_119e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    head(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    if ctx.game.scenes.mode == crate::scene::SCENE { return; }
    match w.mobys[id].sub_state {
        1 => if w.mobys[id].first_sub() {
            let s = ctx.game.rng.randi(2);
            w.mobys[id].set_pv_i(SIDE, s);
        },
        2 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                die(w, id, ctx);
            } else if f & 2 != 0 {
                w.mobys[id].set_state(3, 1);
            }
        }
        _ => {}
    }
    if 0x80 <= w.mobys[id].state { return; }
    let hero = ctx.hero.pos;
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() {
                let c = w.class(CLASS);
                let (solid, cmode) = (c.is_some_and(|c| c.has_collision), c.map_or(0, |c| c.mode));
                let m = &mut w.mobys[id];
                m.has_collision = solid;
                m.mode = cmode | mode::TARGET;
                let off = through(&m.rows, m.pv_v(STAND));
                let p = add(m.pos, off);
                m.pos = p;
                let z = ai::ground_z(ctx, 0.5, p, 0);
                w.mobys[id].pos[2] = z;
                let yaw = yaw_to_hero(w, id, ctx);
                w.mobys[id].rot = [0.0, 0.0, yaw];
                if seq_b(w, id, ctx) != 3 { ctx.host.set_anim(&w.mobys[id], 3, 0, 5); }
            }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(2, 1); }
        }
        2 => {
            if w.mobys[id].first() {
                if seq_b(w, id, ctx) != 2 { ctx.host.set_anim(&w.mobys[id], 2, 0, 0x14); }
                let m = &mut w.mobys[id];
                m.set_pv_f(SPIN, DEG60);
                m.set_pv_f(MV + 0x28, CIRCLE_SPEED * 0.016_666_668);
            }
            let m = &mut w.mobys[id];
            let d = dist2(m.pos, hero);
            if d < NEAR { m.set_pv_f(SPIN, DEG120); } else if FAR < d { m.set_pv_f(SPIN, DEG60); }
            if m.pv_i(MV + 0x80) == 60 {
                m.set_pv_i(MV + 0x80, 0);
                m.set_pv_i(SIDE, (m.pv_i(SIDE) + 1) & 1);
            }
            let off = if m.pv_i(SIDE) != 0 { m.pv_f(SPIN) } else { -m.pv_f(SPIN) };
            ai::move_toward(w, id, ctx, off, hero);
            if (ctx.game.frame & 15) as usize == id & 15 && may_attack(w, id, ctx) {
                w.mobys[id].set_pv_i(ATTACKING, 1);
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            if w.mobys[id].first() {
                if seq_b(w, id, ctx) != 2 { ctx.host.set_anim(&w.mobys[id], 2, 0, 10); }
                w.mobys[id].set_pv_f(MV + 0x28, SPEED * 0.016_666_668);
            }
            ai::move_toward(w, id, ctx, 0.0, hero);
            if w.mobys[id].pv_f(MV + 0xcc) < REACH { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if w.mobys[id].first() && seq_b(w, id, ctx) != 4 { ctx.host.set_anim(&w.mobys[id], 4, BITE_FRAME as u8, 10); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if settled(w, id, ctx) && (9.0..=17.0).contains(&f) {
                set_walk(w, id, LUNGE_SPEED, LUNGE_ACCEL);
                let yaw = yaw_to_hero(w, id, ctx);
                ai::move_step(w, id, ctx, yaw, hero);
            } else {
                let yaw = yaw_to_hero(w, id, ctx);
                crate::moby::ai::face_at(&mut w.mobys[id], MV, yaw);
            }
            set_walk(w, id, SPEED, ACCEL);
            if settled(w, id, ctx) && ai::reached(w, id, ctx, 14.0) {
                let m = &w.mobys[id];
                let c = add(through(&m.rows, BITE_AT), m.pos);
                ai::attack_sphere(w, id, ctx, f32::from_bits(0x3eaa_7efa), 2.0, 1.0, c, 0x400_0001, 0, 1, 0);
            }
            if wrapped(w, id, ctx) && REACH < dist2(w.mobys[id].pos, hero) { w.mobys[id].set_state(3, -1); }
        }
        7 => {
            // `0x2f01f0` on `+0x210` (its result unused).
            crate::moby::shards::ball_at(w, id, ctx, BODY);
            let p = w.mobys[id].pos;
            ai::attack_sphere(w, id, ctx, 0.5, 5.0, 1.0, p, 0x400_0000, 0, 1, 0);
            let m = &mut w.mobys[id];
            let rows = euler_rows([0.0, 0.0, m.pv_f(SPIN) * 0.016_666_668]);
            let v = through(&rows, m.pv_v(BODY));
            m.set_pv_v(BODY, v);
            let mut l = len2(v);
            if l < 0.050_000_004 {
                approach(0.0, f32::from_bits(0x3b36_0b61), &mut l);
                if 0.0 < l { m.set_pv_v(BODY, set_len2(v, l)); } else { m.set_state(1, 1); }
            }
            let m = &mut w.mobys[id];
            if m.pos[2] < m.pv_f(REACT + 0x40) {
                hide(m);
                m.set_state(6, -1);
            }
        }
        _ => {}
    }
    // `0x441690`: its ticks, its head aim.
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 2, HEAD_SCALE);
    w.changed = true;
}
