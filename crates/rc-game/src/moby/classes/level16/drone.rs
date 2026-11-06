//! Snivelak's drone **0x0f35** (L16 `0x40dc68`, 4 placed; the decomp's v2.00 `func_US2_L16_0040DC68`, its helpers
//! v1.01 `func_L16_0040CD88` / `0040CF98` / `0040D258` at our `0x40e1b0` / `0x40e3c0` / `0x40e680`) and its shot
//! **0x0f7a** (`0x414c78`, made by `0x415138`): a craft hovering at its height on five exhaust jets that waits for
//! Ratchet in its area, then fires at him from its joint 2 every time its sequence comes round, flying its path, and
//! rests between bursts. On the framework (crate::moby::ai): health `+0x20` (most `+0x24`, 1 by default), movement
//! `+0x50` (radius 1.868, no height, steps 0.5 / 0.5, slope 180°, the turn left 30°, options | 2), reaction `+0x140`
//! (a hurt to state 5, sub 3; radius and centre 0.5, bounce 0.15), target block `+0x1f0` (its point the shots' aim).
//!
//! **Its pvars:** `+0x230` / `+0x234` the first search's area and range, `+0x238` a timer counted down (480..600
//! after an alarm, 580..600 when it calms; read by nothing here) and `+0x23c` a second (s16), `+0x23e` the alarm's
//! wait, `+0x240` / `+0x354` the two searches' results (the second's area `+0x350`, no range), `+0x244` alarmed (by
//! its group: [`alarm`]), `+0x24c` its path (−1 none), `+0x250` / `+0x2d0` its joint aims (lists 0 and 1, toward
//! `+0x2b8` / `+0x334`), `+0x358` its height.
//!
//! **Each tick** ([`head`], `0x40e1b0`): in state 0 the set-up — the reaction and movement blocks, the senses
//! (`0x325d70` = L00 `0x32dcf0`: 1/450, 1/15, 0.025°, 2°), health, state 1 (sub 0), sequence 0, its height. Its
//! shadow within 32 (range 0x1e); the frame's damage (`0x320400` = L00 `0x327e38`, crate::moby::ai::take_damage)
//! alarms its group; the two searches (`0x3231c8` = L00 `0x32ad38`, height 0); its height kept.
//!
//! **Sub-states** (`+0x95`): **0** calm: its wait drawn 0..29 every tick; found and the wait out (`0x2e7e88`, a
//! countdown from what was just drawn), the attack (3, sub 2) at sequence 1; alarmed: on guard (2, sub 1). **1** on
//! guard: found by either, the attack. **2** attacking: found by neither, on guard. **3** hurt (`0x321148` = L00
//! `0x328b80`, crate::moby::ai::flight): dead — four of its own pieces as balls (`0x2d5088` = L00 `0x2dbdb0` mode 1,
//! crate::moby::shards::spawn_ball: 0.13..0.2 a random way, up at 1.5×, spin 30, gravity 15), its scorch 0x10ce on
//! the ground under it (0.5 up, `0x316510` = L00 `0x31ce20`) in Ratchet's light, a blue blast (size 1: 45 / 40 / 40
//! sprites, 40 puffs, 45 streaks, the flash ball; `0x402698`, crate::moby::blast::burst_blue), the camera shaken
//! (0.2 for 60 ticks, the roll 0.1 for 90), a wreck 0x0ecb thrown (mode 3, crate::moby::shards::spawn_whole_ball:
//! ±0.5..4 / ±0..4 across, 21..27 up a second, spin 180, gravity 25, Ratchet's light), its bolts (`0x314538` = L00
//! `0x31adc0`), gone; over: found by either the attack at sequence 1, else on guard.
//!
//! **States** (`+0x20`): **3** the attack: at frame 1 of its sequence a shot from its joint 2 at the target block's
//! point (25 a second, damage 2); along its path (`0x325dd8` = L09 `0x32a280`, crate::moby::ai::follow_path); after
//! 180 ticks the rest (4) at sequence 0. **4** the rest: along its path; after 180 ticks the attack at sequence 1.
//!
//! **The tail** (`0x40e3c0`, `0x40e680`): the timers, its ticks; the jets (crate::particles::trail, `0x32ad48` = L00
//! `0x332c10`): at each of its joints 5..7 one down (4.5..5.5 a second; 0x6000ffff → 0x80, 20..24 ticks, 350 base,
//! −10) and one orange (0xefff7f4f → 0xff0000, 10..15 ticks, 20..90 base shrinking as much), at its joints 3 and 4
//! the same backward along its facing (bases 100 and 10..25).
//!
//! **The shot 0x0f7a** ([`fire`], [`update_0f7a`]): life 240 ticks (then a 0.6 sphere of its damage, flags
//! 0x10001); each tick its up row set from the floor under it (the line 0.1 over to 3 under it along the row, flags
//! 2: its face's normal, else the row), its velocity kept (`0x2e8e88` = L00 `0x2efa98`: turned by an angle that comes
//! out 0); the line to its next place (flags 0, its hit on what it meets: flags 1) or a 0.3 sphere (flags 0): the
//! world, or a moby not its owner, a 0.6 sphere (its damage, flags 0x10001, along its way ×3 / ×1.5) and it ends —
//! the next tick a small blast (`0x3e8da0` = L00 `0x3f8348`, crate::moby::blast::burst: 0.25, 25 / 20 / 20 sprites,
//! 20 puffs, 45 streaks), gone. At its firing two twinkles at it (`0x3324d0` = L00 `0x33a6a0`).
//!
//! Not ported: the two joint aims (`0x40e3c0`: `0x31d6e8` = L00 `0x324208` on `+0x250` / `+0x2d0`, the head aims no
//! class of the port has), the jets' other colours while `0x1b25c8` bit 0 is set (taken as clear: no class of the port
//! sets it), the shot's draw (`0x4152f0`) and its own model (at a scale of 0.0001), so in flight it is not seen.

use crate::hero::physics::{add, cross, dot, scale, set_len, sub, unit, V3};
use crate::moby::classes::props::{seq, set_seq};
use crate::moby::{ai, blast, bolts, countdown, mode, shards, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x0f35;
pub const SHOT: i16 = 0x0f7a;
/// Its scorch (a model; no update of its own).
const SCORCH: i16 = 0x10ce;
/// Its wreck (crate::moby::classes::level16::copter's rider).
const WRECK: i16 = super::copter::RIDER;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const T_POINT: usize = 0x1f0;
const AREA1: usize = 0x230;
const RANGE1: usize = 0x234;
const TIMER: usize = 0x238;
const TIMER2: usize = 0x23c;
const T_ALARM: usize = 0x23e;
const FOUND1: usize = 0x240;
pub(super) const ALARM: usize = 0x244;
const PATH: usize = 0x24c;
const AREA2: usize = 0x350;
const FOUND2: usize = 0x354;
const HEIGHT: usize = 0x358;
const PVARS: usize = 0x360;

/// The shot's speed a tick (`0x3ed55556`: 25 a second).
const SHOT_SPEED: f32 = f32::from_bits(0x3ed5_5556);

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn tick_i(m: &mut Moby, off: usize) -> bool {
    let mut t = m.pv_i(off);
    let out = countdown(&mut t);
    m.set_pv_i(off, t);
    out
}
fn tick_s(m: &mut Moby, off: usize) -> bool {
    let mut t = m.pv_s(off) as i32;
    let out = countdown(&mut t);
    m.set_pv_s(off, t as i16);
    out
}
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(FOUND1) == 1 || w.mobys[id].pv_i(FOUND2) == 1 }
fn seq0(w: &MobyWorld, id: usize, ctx: &mut Ctx) { if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); } }
fn seq1(w: &MobyWorld, id: usize, ctx: &mut Ctx) { set_seq(w, id, ctx, 1, 0, 10); }
/// Ratchet's light (`0x319798` = L00 `0x31fe80`).
fn hero_light(w: &mut MobyWorld, k: usize, ctx: &Ctx) {
    if let Some((l, a)) = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient)) { (w.mobys[k].light, w.mobys[k].ambient) = (l, a); }
}

/// 0x0f35 (module docs).
pub fn update_0f35(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    head(w, id, ctx);
    match w.mobys[id].sub_state {
        0 => {
            let t = ctx.game.rng.randi(30);
            let m = &mut w.mobys[id];
            m.set_pv_s(T_ALARM, t as i16);
            if m.pv_i(FOUND1) == 1 && tick_s(m, T_ALARM) {
                seq1(w, id, ctx);
                st(w, id, 3, 2);
            } else if w.mobys[id].pv_i(ALARM) != 0 {
                st(w, id, 2, 1);
                let m = &mut w.mobys[id];
                m.set_pv_i(ALARM, 0);
                if m.pv_i(TIMER) < 300 {
                    let t = ctx.game.rng.rand_range(480, 600);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        1 => {
            if found(w, id) { st(w, id, 3, 2); }
        }
        2 => {
            if !found(w, id) {
                st(w, id, 2, 1);
                let t = ctx.game.rng.rand_range(580, 600);
                w.mobys[id].set_pv_i(TIMER, t);
            }
        }
        3 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                die(w, id, ctx);
                return;
            }
            if f & 2 != 0 {
                if found(w, id) {
                    seq1(w, id, ctx);
                    st(w, id, 3, 2);
                } else {
                    st(w, id, 2, 1);
                    let t = ctx.game.rng.rand_range(580, 600);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        3 => {
            if ai::reached(w, id, ctx, 1.0) {
                if let Some(p) = ctx.host.joint(&w.mobys[id], 2) {
                    let v = set_len(sub(w.mobys[id].pv_v(T_POINT), p), SHOT_SPEED);
                    fire(w, ctx, 2.0, id, v, p);
                }
            }
            let path = w.mobys[id].pv_i(PATH);
            if path != -1 { ai::follow_path(w, id, ctx, path); }
            if 180 < w.mobys[id].state_ticks as u16 {
                seq0(w, id, ctx);
                st(w, id, 4, -1);
            }
        }
        4 => {
            let path = w.mobys[id].pv_i(PATH);
            if path != -1 { ai::follow_path(w, id, ctx, path); }
            if 180 < w.mobys[id].state_ticks as u16 {
                seq1(w, id, ctx);
                st(w, id, 3, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    tick_i(m, TIMER);
    tick_s(m, TIMER2);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    jets(w, id, ctx);
    w.changed = true;
}

/// `0x40e1b0` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_f(MV + 4, 0.0);
        m.set_pv_f(MV + 0x2c, 0.0);
        m.set_pv_b(REACT + 0x10, 5);
        m.set_pv_i(REACT + 0x48, 3);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3e19_999a));
        m.set_pv_f(MV, f32::from_bits(0x3fef_1aa0));
        m.set_pv_f(MV + 0xc, 0.5);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x4049_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(REACT + 0x30, 0.5);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(MV + 8, 0.5);
        ai::senses(m, f32::from_bits(0x3b11_a2b4), f32::from_bits(0x3d88_8889), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3d0e_fa37));
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 2);
        if m.pv_s(MOST) == 0 {
            m.set_pv_s(MOST, 1);
            m.set_pv_f(HEALTH, 1.0);
        }
        st(w, id, 1, 0);
        seq0(w, id, ctx);
        let m = &mut w.mobys[id];
        // `0x2e93a8` (L00 `0x2effb8`): 18 and 16 as bytes.
        m.set_pv_b(0x48, 18);
        m.set_pv_b(0x4a, 16);
        let z = m.pos[2];
        m.set_pv_f(HEIGHT, z);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 32.0, 0x1e);
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    let m = &w.mobys[id];
    let (a1, r1, a2) = (m.pv_i(AREA1), m.pv_f(RANGE1), m.pv_i(AREA2));
    let f1 = ai::find_target(w, id, ctx, r1, 0.0, a1, None, false);
    w.mobys[id].set_pv_i(FOUND1, f1 as i32);
    let f2 = ai::find_target(w, id, ctx, 0.0, 0.0, a2, None, false);
    w.mobys[id].set_pv_i(FOUND2, f2 as i32);
    let m = &mut w.mobys[id];
    m.pos[2] = m.pv_f(HEIGHT);
}

/// `0x40e5b0`: its group alarmed — each 0x0f35 `+0x244`, each 0x0ea3 `+0x24c` (a class the port has not).
fn alarm(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].group;
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for k in list {
        let Some(m) = w.mobys.get_mut(k) else { return };
        let off = match m.o_class { CLASS => ALARM, 0x0ea3 => 0x24c, _ => continue };
        if off + 4 <= m.pvars.len() { m.set_pv_i(off, 1); }
    }
}

/// Sub 3, dead (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, rot, o_class) = (w.mobys[id].pos, w.mobys[id].rot, w.mobys[id].o_class);
    for k in 0..4u8 {
        let mut v = ctx.game.rng.rand_vec(f32::from_bits(0x3e08_8889), f32::from_bits(0x3e4c_ccce));
        v[2] = v[2].abs() * 1.5;
        shards::spawn_ball(w, ctx, o_class, Some(id), pos, rot, 30.0, 15.0, k, v, 0xff);
    }
    // `0x322bd8` (L00 `0x32a748`): at its place, angles and light; then on the ground.
    let (light, amb) = (w.mobys[id].light, w.mobys[id].ambient);
    let k = w.create(SCORCH);
    let s = &mut w.mobys[k];
    (s.update_dist, s.draw_dist, s.drawn) = (0xff, 0xff, 1);
    s.parent = Some(id);
    (s.pos, s.rot) = (pos, rot);
    (s.light, s.ambient) = (light, amb);
    s.mode &= 0xfff8;
    s.pos[2] = ai::ground_z(ctx, 0.5, pos, 0);
    hero_light(w, k, ctx);
    w.rebuild(k, &*ctx.host);
    let b = blast::Blast { size: 1.0, bursts: [45, 40, 40], puffs: 40, streaks: 45, light: true, shake: false };
    blast::burst_blue(w, ctx, pos, &b);
    let roll = &mut ctx.game.shakes.0[2];
    (roll.amp, roll.timer) = (f32::from_bits(0x3dcc_cccd), 90);
    ctx.game.shakes.ask(0, f32::from_bits(0x3e4c_cccd), 60);
    let rng = &mut ctx.game.rng;
    let x = rng.randf_signed(0.5, 4.0) * 0.016_666_668;
    let y = rng.randf_signed(0.0, 4.0) * 0.016_666_668;
    let z = rng.randf(7.0, 9.0) * 0.016_666_668;
    let v = [x, y, z.abs() * 3.0];
    if let Some(k) = shards::spawn_whole_ball(w, ctx, WRECK, Some(id), pos, rot, 180.0, 25.0, v, 0xff) { hero_light(w, k, ctx); }
    bolts::drop(w, id, 0, 3, ctx);
    w.delete(id);
}

/// `0x40e680` (module docs).
fn jets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let back = w.mobys[id].rows[0];
    for j in [5usize, 6, 7, 3, 4] {
        let s = ctx.game.rng.randf(4.5, 5.5) * 0.016_666_668;
        let v = if 5 <= j { [0.0, 0.0, -s] } else { set_len(back, -s) };
        let Some(p) = ctx.host.joint(&w.mobys[id], j) else { continue };
        let (life, base) = (ctx.game.rng.rand_range(20, 24), if 5 <= j { 350 } else { 100 });
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::trail(sys, &mut ctx.game.rng, p, v, 0x6000_ffff, 0x80, life, base, -10, true); }
        let n = ctx.game.rng.rand_range(if 5 <= j { 20 } else { 10 }, if 5 <= j { 90 } else { 25 });
        let life = ctx.game.rng.rand_range(10, 15);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::trail(sys, &mut ctx.game.rng, p, v, 0xefff_7f4f, 0x00ff_0000, life, n as i16, -n as i16, true); }
    }
}

// The shot 0x0f7a.

const S_VEL: usize = 0x00;
const S_OWNER: usize = 0x10;
const S_DAMAGE: usize = 0x14;
const S_LIFE: usize = 0x24;
const S_PVARS: usize = 0x30;

/// `0x415138(damage, owner, vel, pos)` (module docs): update and draw distances 0xff, drawn, state 0, hidden (mode
/// | 0x41), the owner's light and angles; mode | 0x308; scale `$gp+0x2948` (0.0001); the two twinkles.
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, owner: usize, vel: V3, pos: V3) -> usize {
    let k = w.create(SHOT);
    let (light, amb, rot) = (w.mobys[owner].light, w.mobys[owner].ambient, w.mobys[owner].rot);
    let m = &mut w.mobys[k];
    if m.pvars.len() < S_PVARS { m.pvars.resize(S_PVARS, 0); }
    m.update = Some(update_0f7a);
    m.mode &= !mode::NO_UPDATE;
    (m.update_dist, m.draw_dist, m.drawn, m.state) = (0xff, 0xff, 1, 0);
    (m.light, m.ambient) = (light, amb);
    m.pos = pos;
    m.mode |= 0x41;
    m.set_pv_v(S_VEL, vel);
    m.set_pv_f(S_DAMAGE, damage);
    m.set_pv_f(0x20, f32::from_bits(0x3e19_999a));
    ai::set_link(m, S_OWNER, Some(owner));
    m.mode |= 0x208;
    m.rot = rot;
    m.rows = crate::hero::physics::euler_rows(rot);
    m.alpha = 0x80;
    m.mode |= mode::KEEP_ROWS;
    m.scale = f32::from_bits(0x38d1_b717);
    m.set_pv_i(S_LIFE, 0);
    let s = ctx.game.rng.randf(8.0, 10.0);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.3, s, 0.0, pos, 15, 0x8010_3080, 0, 10, [0.0; 3], 0);
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.04, s * 0.4, 0.0, pos, 15, 0xffff_ffff, 1, -10, [0.0; 3], 0);
    }
    k
}

/// 0x0f7a (`0x414c78`, module docs).
pub fn update_0f7a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let life = m.pv_i(S_LIFE) + 1;
    m.set_pv_i(S_LIFE, life);
    let dmg = m.pv_f(S_DAMAGE);
    if 0xf0 < life {
        let pos = w.mobys[id].pos;
        ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f19_999a), dmg, 0.0, pos, 0x1_0001, 0, 1, 0);
        st(w, id, 1, -1);
    }
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, vel, up, owner) = (m.pos, m.pv_v(S_VEL), m.rows[2], ai::link(m, S_OWNER));
            let back = scale(up, -1.0);
            let speed = crate::hero::physics::len(vel);
            let (a, b) = (add(set_len(up, 0.1), pos), add(set_len(up, -3.0), pos));
            let n = match w.coll_line(ctx, a, b, 2, Some(id)) {
                Some(o) => set_len(o.normal, 1.0),
                None => set_len(up, -1.0),
            };
            let n = scale(n, -1.0);
            let v = unit(vel);
            let c = cross(back, n);
            let h = -dot(back, c) * 0.5;
            let q = { let s = set_len(c, 1.0); let k = h.sin(); [s[0] * k, s[1] * k, s[2] * k, h.cos()] };
            let vel = set_len(crate::quat::rotate(q, v), speed);
            let m = &mut w.mobys[id];
            m.set_pv_v(S_VEL, vel);
            m.rows[2] = scale(c, -1.0);
            let next = add(pos, vel);
            let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 1, [vel[0], vel[1], vel[2], 0.0]);
            t.kind = (SHOT as u16 as u32) << 16;
            let (coll, scene) = (ctx.coll, ctx.scene);
            let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, pos, next, crate::collision::QueryFlags(0), Some(id)));
            if let Some(o) = line.as_ref() {
                if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
            }
            let met = match line {
                Some(o) => Some(o.moby),
                None => coll.and_then(|c| crate::collision::coll_sphere_m(c, scene, pos, f32::from_bits(0x3e99_999a), crate::collision::QueryFlags(0), Some(id))).map(|o| o.moby),
            };
            let push = match met {
                Some(None) => Some(3.0),
                Some(Some(k)) if Some(k) != owner => Some(1.5),
                _ => None,
            };
            if let Some(k) = push {
                let d = scale(vel, k);
                let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, [d[0], d[1], 1.0, 5_627.925]);
                t.kind = (SHOT as u16 as u32) << 16 | 1 << 8;
                w.sphere_hit(scene, pos, f32::from_bits(0x3f19_999a), 0, Some(id), Some(&t));
                st(w, id, 1, -1);
                return;
            }
            w.mobys[id].pos = next;
        }
        1 => {
            let pos = w.mobys[id].pos;
            let b = blast::Blast { size: 0.25, bursts: [25, 20, 20], puffs: 20, streaks: 45, light: false, shake: false };
            blast::burst(w, ctx, pos, &b);
            w.delete(id);
            return;
        }
        _ => {}
    }
    w.changed = true;
}
