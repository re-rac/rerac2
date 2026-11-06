//! Yeedil's shooter **0x12ef** (`0x44b710`, 8 placed; the decomp's `func_L20_0044A1C8`): a small flier the boss
//! 0x11e0 (crate::moby::classes::level20::boss) spits out of its pool in its attack 9 ([`launch`]). It glides out,
//! hovers, then picks the fighters 0x12f3 (crate::moby::classes::level20::fighter) or Ratchet and fires class 0x12f2
//! shots at them (`0x39d908`, [`fire`]). Its pvars: health `+0x20` (most `+0x24`; its shot's damage byte `+0x34`),
//! target `+0x140` (`+0x150` the moby; the search's area `+0x160`), `+0x170` the way it was spat, `+0x180` where it
//! flies (its place without the bob), `+0x190` its goal (`+0x198` its height), `+0x1a0` the bob's phase, `+0x1a4`
//! its turn speed, `+0x1a8` / `+0x1ac` the bank and pitch it eases to, `+0x1b0` its pick (a link), `+0x1b4` the
//! shots' walk along the ground (from −4..−6 short of the target to 4 past), `+0x1b8` its loop sound. Its owner
//! (`+0xb8`) the boss.
//!
//! **Each tick**: the head ([`head`], `0x44bba0`): the set-up in state 0 (hidden, no collision, 1; its place kept;
//! scaled × 3); nothing more in 1. A hit from a 0x12f2 or a 0x12ef does nothing; the frame's damage off its health;
//! none left: its end ([`die`]), hidden, 1. Out of 5 the target: its pick (half a unit up), else the search (1024,
//! height 37, its area), else Ratchet's aim point. In a scene it is gone. **States**: **2** spat out (its loop sound
//! 0; the boss's collision off for its move): to its goal ([`fly`]), within 0.1 → 3. **3** 60 ticks → 4. **4**
//! sequence 0; its pick (the boss's, [`boss::pick_for`], one time in two none); a goal 5..15 from the target at the
//! camera's yaw + ±20..70°; there → 5. **5** sequence 1, the walk from 0 (a pick) or −4..−6, its class sound 1; on its
//! frame slice (one in 16) a shot from `(0.5, 0.55, 0.8)` in its frame at 40 a second toward the ground the walk
//! has reached (the target's height off the ground), the walk on by 1 to 4 → 4. Then the tail (`0x44bd80`): its
//! ticks; drawn, it bobs 0.075 at 600°/s about its place with a 0.3 shadow; shown, it faces its target (720°/s²,
//! 720°/s) and eases its bank and pitch (four times that a tick).
//!
//! The engine functions with the port's: `0x330820` (set_state), `0x330540` (set_anim), `0x33faa8` (first),
//! `0x340018` = L00 `0x32ad38` (ai::find_target), `0x332e28` / `0x332e88` = L00 `0x31dce8` / `0x31dd48` (the hit,
//! ai::damage_of), `0x3345f0` = L00 `0x31f4b0` (physics::ease_speed), `0x341f40` (L11 `0x33fa98`, ai::move_by),
//! `0x334370` = L00 `0x31f230` (physics::turn_ease), `0x332a70` = L00 `0x31d930` (crate::shadows::request),
//! `0x41b550` = L00 `0x3f8348` (crate::moby::blast::burst), `0x339450` = L00 `0x3241d8` (the ambient word),
//! `0x3766c0` / `0x3763c0` (the class sounds).
//!
//! Not ported: the shot 0x12f2's own update (no port; [`fire`] makes it with its owner, place, velocity, damage and
//! life); its place over the ground in 4 from Ratchet's `+0x2d8` (`0x18a0f8`, grown by 1.5..2.5 when set — the port
//! keeps the goal's last height); the moby word `+0x9c`.

use super::boss;
use crate::hero::physics::{add, add_rot, approach, atan2, ease_speed, len, scale, set_len, set_len2, sub, turn_ease, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::props::seq;
use crate::moby::{ai, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12ef;
/// Its shot.
pub const SHOT: i16 = 0x12f2;

const HP: usize = 0x20;
const MAX: usize = 0x24;
const DAMAGE: usize = 0x34;
const MV: usize = 0x50;
const AIM: usize = 0x140;
const AIM_MOBY: usize = 0x150;
const AREA: usize = 0x160;
const WAY: usize = 0x170;
const HOME: usize = 0x180;
const GOAL: usize = 0x190;
const BOB: usize = 0x1a0;
const TURN_V: usize = 0x1a4;
const BANK: usize = 0x1a8;
const PITCH: usize = 0x1ac;
const PICK: usize = 0x1b0;
const WALK: usize = 0x1b4;
const VOICE: usize = 0x1b8;

const DEG_S: f32 = 0.000_290_888_25;
const DEG_S2: f32 = 4.848_137_5e-6;
/// `$gp+0x33d0` / `+0x33d4`: the bob's height and rate (°/s); `+0x33d8` its scale; `+0x33dc` its shadow.
const BOB_AMP: f32 = f32::from_bits(0x3d99_999a);
const BOB_RATE: f32 = 600.0;
const SCALE: f32 = 3.0;
const SHADOW: f32 = f32::from_bits(0x3e99_999a);
/// `$gp+0x33e0`: the shot's speed (a second); `+0x33f0` its muzzle in its frame.
const SHOT_SPEED: f32 = 40.0;
const MUZZLE: V3 = [0.5, f32::from_bits(0x3f0c_cccd), f32::from_bits(0x3f4c_cccd)];
/// `$gp+0x3400`: the ambient word it is given as it is spat.
const SPAT_AMBIENT: u32 = 0x8000_0000;
/// `$gp+0x3404` (/3600) / `+0x3408` (/60): its speed's ease; `+0x340c` the bank by its sideways speed; `+0x3410` /
/// `+0x3414` its turn (°/s², °/s).
const ACCEL: f32 = 40.0;
const TOP: f32 = 20.0;
const BANK_K: f32 = 2.0;
const TURN: (f32, f32) = (720.0, 720.0);
const QUARTER: f32 = f32::from_bits(0x3f49_0fdb);
/// Its end's blast (`0x3f8348(0.666, 20, at, 0, 25, 20, 20, 20, 45, 0, 1, …)`).
const END: Blast = Blast { size: f32::from_bits(0x3f2a_7efa), bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false };

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&k| k < w.mobys.len()) }
fn hide(m: &mut crate::moby::Moby) {
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & 0xefff;
}

/// `0x44c180(boss, g, pos, vel)`: the first of group `g` in state 1 spat out at `pos` along `vel` — shown with its
/// collision, sequence 0, the ambient word, mirrored one time in two, its place `pos`, `vel` kept, the boss its owner,
/// 2; its goal 20..30 × `vel` on (at `pos`'s height), full health, facing along `vel`, the bob's phase at random.
pub fn launch(w: &mut MobyWorld, boss: usize, ctx: &mut Ctx, g: i32, pos: V3, vel: V3) -> Option<usize> {
    let id = boss::members(w, g).into_iter().find(|&k| w.mobys[k].state == 1)?;
    let c = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = c;
    if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0xf); }
    let mirror = ctx.game.rng.randi(2) != 0;
    let m = &mut w.mobys[id];
    m.set_ambient_word(SPAT_AMBIENT);
    if mirror { m.mode |= mode::MIRROR; } else { m.mode &= !mode::MIRROR; }
    m.pos = pos;
    m.set_pv_v(HOME, pos);
    m.set_pv_v(WAY, vel);
    m.parent = Some(boss);
    m.set_state(2, -1);
    let k = ctx.game.rng.randf(20.0, 30.0);
    let m = &mut w.mobys[id];
    let g = add(scale(vel, k), pos);
    m.set_pv_v(GOAL, [g[0], g[1], pos[2]]);
    m.set_pv_f(HP, m.pv_s(MAX) as f32);
    m.rot[2] = atan2(vel[0], vel[1]);
    let a = ctx.game.rng.rand_angle();
    w.mobys[id].set_pv_f(BOB, a);
    w.changed = true;
    Some(id)
}

/// `0x39d908(owner, pos, vel, damage)`: a shot 0x12f2 at `pos` — its owner, update distance 0xff, its velocity, damage
/// and a life of 300 (its own update is not ported). None without the class.
pub fn fire(w: &mut MobyWorld, owner: usize, pos: V3, vel: V3, damage: f32) -> Option<usize> {
    w.class(SHOT)?;
    let k = w.create(SHOT);
    let m = &mut w.mobys[k];
    m.parent = Some(owner);
    m.pos = pos;
    m.update_dist = 0xff;
    m.set_pv_v(0x10, vel);
    m.set_pv_f(0x2c, damage);
    m.set_pv_i(0x20, 0x12c);
    w.changed = true;
    Some(k)
}

/// `0x44c0b0`: its end — a blast 0.3 up its frame, its loop sound let go.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at = add([m.rows[2][0] * 0.3, m.rows[2][1] * 0.3, m.rows[2][2] * 0.3], m.pos);
    blast::burst(w, ctx, at, &END);
    let h = w.mobys[id].pv_i(VOICE);
    if h != -1 {
        ctx.host.stop_sound(h);
        w.mobys[id].set_pv_i(VOICE, -1);
    }
}

/// `0x44bba0` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let s = w.class(CLASS).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        m.set_state(1, -1);
        hide(m);
        let p = m.pos;
        m.set_pv_v(HOME, p);
        m.scale = s * SCALE;
        m.set_pv_i(VOICE, -1);
    }
    if w.mobys[id].state == 1 { return; }
    let mut hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    if hit.as_ref().and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == SHOT || s.o_class == CLASS) { hit = None; }
    let f = ai::damage_of(w, id, hit.as_ref(), false);
    let m = &mut w.mobys[id];
    let mut hp = m.pv_f(HP);
    approach(0.0, f, &mut hp);
    m.set_pv_f(HP, hp);
    w.clear_hit(id);
    if hp == 0.0 {
        die(w, id, ctx);
        w.clear_hit(id);
        let m = &mut w.mobys[id];
        m.set_state(1, -1);
        hide(m);
    }
    if w.mobys[id].state == 5 { return; }
    if let Some(p) = link(w, id, PICK) {
        let at = w.mobys[p].pos;
        let m = &mut w.mobys[id];
        ai::set_link(m, AIM_MOBY, Some(p));
        m.set_pv_v(AIM, [at[0], at[1], at[2] + 0.5]);
        return;
    }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 1024.0, 37.0, area, None, false);
    if link(w, id, AIM_MOBY).is_none() {
        let m = &mut w.mobys[id];
        ai::set_link(m, AIM_MOBY, ctx.hero_moby);
        m.set_pv_v(AIM, ctx.hero.aim_point);
    }
}

/// `0x44be38`: a tick of the flight from its place to its goal — its speed eased toward the way left (40/3600,
/// 20/60 at most) along it; the move (`0x341f40`) from its place without the bob; the bank and pitch by its velocity
/// in its frame (× 2: the bank ±45°, the pitch 0..45°). Returns the way left.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> f32 {
    let m = &mut w.mobys[id];
    let d = sub(m.pv_v(GOAL), m.pv_v(HOME));
    let mut speed = len(m.pv_v(MV + 0xa0));
    let f = len(d);
    ease_speed(f, ACCEL * 0.000_277_777_8, ACCEL * 0.000_277_777_8, TOP * 0.016_666_668, &mut speed);
    let v = set_len(d, speed);
    m.set_pv_v(MV + 0xa0, v);
    m.pos = m.pv_v(HOME);
    ai::move_by(w, id, ctx, v);
    let m = &mut w.mobys[id];
    let p = m.pos;
    m.set_pv_v(HOME, p);
    let r = m.rows;
    let local = [crate::hero::physics::dot(v, r[0]), crate::hero::physics::dot(v, r[1])];
    m.set_pv_f(BANK, (local[1] * -BANK_K).clamp(-QUARTER, QUARTER));
    m.set_pv_f(PITCH, (local[0] * BANK_K).clamp(0.0, QUARTER));
    f
}

/// `0x44bd80`: the tail (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.state_ticks = m.state_ticks.wrapping_add(1);
    if m.drawn != 0 {
        m.pos = m.pv_v(HOME);
        let a = add_rot(m.pv_f(BOB), BOB_RATE * DEG_S);
        m.set_pv_f(BOB, a);
        m.pos[2] += BOB_AMP * a.sin();
        crate::shadows::request(w, ctx, SHADOW, id);
    }
    if w.mobys[id].mode & 1 == 0 {
        // `0x44bfd0`.
        let m = &mut w.mobys[id];
        let t = m.pv_v(AIM);
        let yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
        let (c, d) = (TURN.0 * DEG_S2, TURN.1 * DEG_S);
        let (mut r, mut v) = (m.rot[2], m.pv_f(TURN_V));
        turn_ease(yaw, c, c, d, &mut r, &mut v, 0);
        m.rot[2] = r;
        m.set_pv_f(TURN_V, v);
        let (bank, pitch) = (m.pv_f(BANK), m.pv_f(PITCH));
        approach(bank, d * 4.0, &mut m.rot[0]);
        approach(pitch, d * 4.0, &mut m.rot[1]);
        m.set_pv_f(BANK, 0.0);
        m.set_pv_f(PITCH, 0.0);
    }
    w.changed = true;
}

/// The boss's collision off (`+0xb8`'s `+0x98`) for `f`, then its class's back.
fn without_owner<R>(w: &mut MobyWorld, id: usize, f: impl FnOnce(&mut MobyWorld) -> R) -> R {
    let o = w.mobys[id].parent.filter(|&p| p < w.mobys.len());
    if let Some(o) = o { w.mobys[o].has_collision = false; }
    let r = f(w);
    if let Some(o) = o { w.mobys[o].has_collision = w.class(w.mobys[o].o_class).is_some_and(|c| c.has_collision); }
    r
}

/// 0x12ef (module docs).
pub fn update_12ef(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    head(w, id, ctx);
    if ctx.game.scenes.mode == crate::scene::SCENE {
        w.delete(id);
        return;
    }
    let ticks = w.mobys[id].state_ticks as u16;
    match w.mobys[id].state {
        2 => {
            if w.mobys[id].first() {
                let v = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(VOICE, v);
            }
            let f = without_owner(w, id, |w| fly(w, id, ctx));
            if f < f32::from_bits(0x3dcc_cccd) { w.mobys[id].set_state(3, -1); }
        }
        3 => if 60 < ticks { w.mobys[id].set_state(4, -1) },
        4 => {
            if w.mobys[id].first() {
                if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0xf); }
                let parent = w.mobys[id].parent.filter(|&p| p < w.mobys.len());
                let mut p = parent.and_then(|b| boss::pick_for(w, b, id));
                if ctx.game.rng.randi(2) != 0 { p = None; }
                ai::set_link(&mut w.mobys[id], PICK, p);
                if let Some(p) = p {
                    let at = w.mobys[p].pos;
                    let m = &mut w.mobys[id];
                    ai::set_link(m, AIM_MOBY, Some(p));
                    m.set_pv_v(AIM, at);
                }
                let rows = ctx.host.camera_rows();
                let a = add_rot(rows[0][1].atan2(rows[0][0]), ctx.game.rng.randf_signed(20.0, 70.0) * 0.017_453_294);
                let r = ctx.game.rng.randf(5.0, 15.0);
                let m = &mut w.mobys[id];
                let saved = m.pv_f(GOAL + 8);
                let t = m.pv_v(AIM);
                m.set_pv_v(GOAL, [a.cos() * r + t[0], a.sin() * r + t[1], saved]);
            }
            if fly(w, id, ctx) < f32::from_bits(0x3dcc_cccd) { w.mobys[id].set_state(5, -1); }
        }
        5 => {
            if w.mobys[id].first() {
                if seq(w, id, ctx) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 0xf); }
                let s = if link(w, id, PICK).is_some() { 0.0 } else { -ctx.game.rng.randf(4.0, 6.0) };
                w.mobys[id].set_pv_f(WALK, s);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
            if ctx.game.frame & 15 == (id & 15) as i32 {
                let m = &w.mobys[id];
                let (p, aim) = (m.pos, m.pv_v(AIM));
                let mut t = sub(aim, p);
                t[2] = 0.0;
                let x = (len(t) + m.pv_f(WALK)).max(1.0);
                t = add(set_len2(t, x), p);
                t[2] = ai::ground_z(ctx, 0.5, t, 0);
                if t[2] == 0.0 { t[2] = aim[2]; }
                let m = &w.mobys[id];
                let o = add(std::array::from_fn(|k| MUZZLE[0] * m.rows[0][k] + MUZZLE[1] * m.rows[1][k] + MUZZLE[2] * m.rows[2][k]), p);
                let v = set_len(sub(t, o), SHOT_SPEED * 0.016_666_668);
                let dmg = m.pv_b(DAMAGE) as f32;
                fire(w, id, o, v, dmg);
                let m = &mut w.mobys[id];
                let mut s = m.pv_f(WALK);
                approach(4.0, 1.0, &mut s);
                m.set_pv_f(WALK, s);
            }
            if 4.0 <= w.mobys[id].pv_f(WALK) { w.mobys[id].set_state(4, -1); }
        }
        _ => {}
    }
    tail(w, id, ctx);
}
