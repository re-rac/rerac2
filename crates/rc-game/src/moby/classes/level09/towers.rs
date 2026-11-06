//! Dobbo's arc towers and their beams (L09 addresses):
//!
//! * **0x0d12** ([`update_0d12`], `0x3fc960`, 1 placed; the decomp's `func_L09_003FB4D0`): the beam manager,
//!   crate::fx::beams ([`beams::Kind::Dobbo`]); its pvars `[0]` / `[1]` the glow's and the strip's FX textures less 63.
//! * **0x0cea** ([`update_0cea`], `0x3f1940`, 4 placed; `func_L09_003F04D0`): a tower that throws pairs of linked
//!   nodes, an arc between each pair.
//!   * **Its pvars**: health `+0x20` (f32; `+0x26` its hold-off, s16), the flash `+0x70`, the target block `+0x50`
//!     (`+0x64` found), the area it watches `+0x8c` (from `+0xa4` while it fires, `+0xa8` while it waits), the
//!     timer `+0x92` (s16) and `+0x90`, the nodes' reach `+0x80`, spread `+0x94`, speed `+0x98` (a second), the
//!     reload `+0x9c` and rest `+0x9e` (u16), the lives `+0xa0 · k + +0xa2` (s16), their damage `+0x34` (byte).
//!   * **Each tick** (`0x3f17e8` first): in sub 0 armed (`0x3f1740`: health 10 (`+0x24` 10), `+0x28` 3, `+0x30` 1,
//!     `+0x2f` 1, the area `+0xa4`; sub 1); else a hit with the flags 0x10000 (`0x31a158`) while its hold-off is under
//!     1 and its damage (`0x31a1b8`, crate::moby::ai::damage_of) above 0 comes off its health: none left, state 6
//!     (sub 4); else `+0x77` `ftoi(damage · 20) + 100`, its flash started (`0x31c598`), sequence 4 (blend 1), state 5;
//!     its hit cleared. Then the flash (`0x31c658`) and the target (`0x3275b8`, crate::moby::ai::find_target: Ratchet
//!     in the area `+0x8c`).
//!   * **Sub 1**, found: the area `+0xa4`, state 1 sub 2. **Sub 2**, lost in state 1: the area `+0xa8`, sequence 0
//!     (blend 3), state 0 sub 1.
//!   * **States**: 1 the timer out: sequence 1 (3), 2. **2** its sequence wrapped: the timer the reload, sequence 2,
//!     its class sound 2, and at its joints (0, 1), (2, 3), (4, 5) a pair of nodes ([`throw`]) moving along its facing
//!     at its speed, living `+0xa0 · k + +0xa2` ticks; the second linked to the first and the pair given a beam slot
//!     (crate::fx::beams claim, light bits 1, follow 0), or the first deleted when the second is not made; 3. **3** the
//!     timer out: the timer the rest, sequence 3, 4. **4** wrapped: sequence 0, 1. **5** (hurt) wrapped: the area
//!     `+0xa4`, the timer the rest, sequence 3 (blend 1), 4. **6**: its bolts (crate::moby::bolts::drop), the
//!     explosion at its joint 6 (crate::moby::blast::explode: 10 embers, 3 billows, 16 flares, flashes 4 and 2 past
//!     9, size 1, light 15, the shake, a debris, flags 0x810001), its class sound 3, deleted.
//!   * At the end (`0x3f17a0`): its ticks, the timers `+0x92` and `+0x90` down.
//! * **0x0d09** ([`update_0d09`], `0x3f7fb8`, made by [`throw`] `0x3f7cb0`; `func_L09_003F6B28`): a node, hidden
//!   (mode | 0x2201), update and draw distances 0xff, a random spin. **Its pvars**: `+0x00` the way it flies, `+0x10`
//!   its velocity, `+0x20` / `+0x24` the tower and its partner (links), `+0x28` the spread, `+0x2c` the damage, `+0x30`
//!   its timer, `+0x34` its reach, `+0x38` its life (f32), `+0x3c` its hit timer. The first of a pair (no partner)
//!   has its byte `+0xbc` 1.
//!   * Each tick its angles turn by `randf(−90°, 90°)`. **0** → 1. **1**: on each wrap of its sequence sequence 1 (10)
//!     at speed 2; spread at `0.03 · (1 − timer / life)²` ([`spread`], `0x3f7e30`); the timer out: 2; the tower
//!     gone: 3. **2 / 3**: spread at 0.05, flying ([`fly`], `0x3f7f48`: the velocity eased 0.04 toward the way, the
//!     place moved by it); farther across than its reach from the tower, or the tower gone: 3. In 3 it fades (alpha −2
//!     a tick, its scale × 0.98) and, under 2, gives its slot back and is deleted.
//!   * Within `spread · 2 + 1` across of Ratchet, with the tower (0x0cea) and its partner (0x0d09) alive: the first of
//!     a pair hits along the line to its partner (both jittered 0.1; crate::moby::ai::attack_line from the tower,
//!     flags 0x210001, its damage); one tick in four within 2, a sphere of `scale · 0.4 / class scale` at it
//!     (crate::moby::ai::attack_sphere); a hit on Ratchet sets the hit timer (60) once it is out.
//!   * Its alpha up 5 a tick to 100 (but in 3). The first of a pair with its partner alive reports the beam
//!     (crate::fx::beams, `0x3fcd98`): rows `unit(partner − it)`, up, their cross, at it.
//!
//! The engine functions with the port's: `0x317af8` = L00 `0x31b8d0` (set_state), `0x317818` = L00 `0x31b5f0`
//! (set_anim), `0x31a158` / `0x31a1b8` = L00 `0x31dce8` / `0x31dd48` (hit_message / damage_of), `0x31c598` / `0x31c658`
//! = L00 `0x320128` / `0x3201e8` (flash_start / flash), `0x3275b8` = L00 `0x32ad38` (find_target), `0x319698` = L01
//! `0x3132c8` (attack_line), `0x3194a8` = L00 `0x31d180` (attack_sphere).
//!
//! Not ported: the level's height gate (`0x1c9f80`, as Tabora's `0x1ca080`: with its mode set the towers and the
//! nodes past its heights are not updated, the nodes deleted; nothing the port has sets it).

use crate::fx::beams::{self, Kind};
use crate::hero::physics::{add, add_rot, cross, dist2, dot, scale, set_len, sub, V3};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

pub const MANAGER: i16 = 0x0d12;
pub const TOWER: i16 = 0x0cea;
pub const NODE: i16 = 0x0d09;

const HEALTH: usize = 0x20;
const FOUND: usize = 0x64;
const FLASH: usize = 0x70;
const AREA: usize = 0x8c;
const TIMER: usize = 0x92;
const TIMER_B: usize = 0x90;
const REACH: usize = 0x80;
const SPREAD: usize = 0x94;
const SPEED: usize = 0x98;
const RELOAD: usize = 0x9c;
const REST: usize = 0x9e;
const LIFE_STEP: usize = 0xa0;
const LIFE: usize = 0xa2;
const AREA_ON: usize = 0xa4;
const AREA_OFF: usize = 0xa8;
const DAMAGE: usize = 0x34;

// The node's pvars.
const N_WAY: usize = 0x00;
const N_VEL: usize = 0x10;
const N_TOWER: usize = 0x20;
const N_PARTNER: usize = 0x24;
const N_SPREAD: usize = 0x28;
const N_DAMAGE: usize = 0x2c;
const N_TIMER: usize = 0x30;
const N_REACH: usize = 0x34;
const N_LIFE: usize = 0x38;
const N_HIT: usize = 0x3c;
/// `$gp+0x1ea0..`: the spread's ease while flying and while it grows, the velocity's ease, the alpha's steps up and
/// down and its most, the spin, the fade's shrink, the height over the tower.
const SPREAD_FLY: f32 = 0.05;
const SPREAD_GROW: f32 = 0.03;
const VEL_EASE: f32 = 0.04;
const ALPHA_UP: u8 = 5;
const ALPHA_DOWN: u8 = 2;
const ALPHA_MOST: u8 = 100;
const SPIN: (f32, f32) = (-90.0, 90.0);
const SHRINK: f32 = 0.98;
const RISE: f32 = 0.8;

/// `0x3fc960`.
pub fn update_0d12(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { beams::step(w, id, ctx, Kind::Dobbo); }

fn down_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(off) as u16 as i32;
    let r = countdown(&mut t);
    m.set_pv_s(off, t as i16);
    r
}

fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
}

/// `0x3f17e8` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].sub_state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_f(HEALTH, 10.0);
        m.set_pv_s(HEALTH + 4, 10);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x30, 1.0);
        m.set_pv_b(0x2f, 1);
        m.set_pv_i(AREA, m.pv_i(AREA_ON));
        m.set_state(0, 1);
    } else {
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
        let hit = if w.mobys[id].pv_s(HEALTH + 6) < 1 { hit } else { None };
        let dmg = ai::damage_of(w, id, hit.as_ref(), false);
        if hit.is_some() && 0.0 < dmg {
            let m = &mut w.mobys[id];
            let hp = m.pv_f(HEALTH) - dmg;
            m.set_pv_f(HEALTH, hp);
            if hp <= 0.0 {
                m.set_state(6, 4);
            } else {
                m.set_pv_b(FLASH + 7, ((dmg * 20.0) as i32 + 100) as u8);
                ai::flash_start(m, FLASH);
                anim(w, id, ctx, 4, 1);
                w.mobys[id].set_state(5, -1);
            }
        }
        w.clear_hit(id);
    }
    ai::flash(&mut w.mobys[id], FLASH);
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
}

/// `0x3f1940` (module docs).
pub fn update_0cea(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    head(w, id, ctx);
    let found = w.mobys[id].pv_i(FOUND) == 1;
    match w.mobys[id].sub_state {
        1 if found => {
            let m = &mut w.mobys[id];
            m.set_pv_i(AREA, m.pv_i(AREA_ON));
            m.set_state(1, 2);
        }
        2 if !found && w.mobys[id].state == 1 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(AREA, m.pv_i(AREA_OFF));
            if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 3); }
            w.mobys[id].set_state(0, 1);
        }
        _ => {}
    }
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].pv_s(TIMER) == 0 {
                anim(w, id, ctx, 1, 3);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.set_pv_s(TIMER, m.pv_s(RELOAD));
                anim(w, id, ctx, 2, 3);
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                for i in (0..6).step_by(2) {
                    let m = &w.mobys[id];
                    let way = set_len(m.rows[0], m.pv_f(SPEED) * (1.0 / 60.0));
                    let life = (i / 2) as i32 * m.pv_s(LIFE_STEP) as i32 + m.pv_s(LIFE) as i32;
                    let (spread, damage, reach) = (m.pv_f(SPREAD), m.pv_b(DAMAGE) as f32, m.pv_f(REACH));
                    let Some(p) = ctx.host.joint(m, i) else { continue };
                    let Some(first) = throw(w, ctx, p, way, id, None, life, spread, damage, reach) else { continue };
                    let Some(p) = ctx.host.joint(&w.mobys[id], i + 1) else {
                        w.delete(first);
                        continue;
                    };
                    let second = throw(w, ctx, p, way, id, Some(first), life, spread, damage, reach);
                    ai::set_link(&mut w.mobys[first], N_PARTNER, second);
                    match second {
                        Some(s) => { w.beams.claim(Kind::Dobbo, first, Some(s), 1, 0); }
                        None => w.delete(first),
                    }
                }
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            if w.mobys[id].pv_s(TIMER) == 0 {
                let m = &mut w.mobys[id];
                m.set_pv_s(TIMER, m.pv_s(REST));
                anim(w, id, ctx, 3, 3);
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => {
            if wrapped(w, id, ctx) {
                if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 3); }
                w.mobys[id].set_state(1, -1);
            }
        }
        5 => {
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.set_pv_i(AREA, m.pv_i(AREA_ON));
                m.set_pv_s(TIMER, m.pv_s(REST));
                anim(w, id, ctx, 3, 1);
                w.mobys[id].set_state(4, -1);
            }
        }
        6 => {
            crate::moby::bolts::drop(w, id, 0, 3, ctx);
            let at = ctx.host.joint(&w.mobys[id], 6);
            let e = crate::moby::blast::Explosion { embers: 10, billows: 3, flares: 16, flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, sound: -1, light: 15.0, shake: true, debris: 1, lod: -1, colour: 0, flags: 0x81_0001, ..Default::default() };
            crate::moby::blast::explode(w, ctx, Some(id), at, &e);
            ctx.host.play_sound(&w.mobys[id], 3, 0);
            w.delete(id);
            return;
        }
        _ => {}
    }
    // `0x3f17a0`.
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = m.state_ticks.wrapping_add(1); }
    down_s(w, id, TIMER);
    down_s(w, id, TIMER_B);
}

/// `0x3f7cb0(pos, way, tower, partner, life, spread, damage, reach)`: a node (module docs); None when none is made.
#[allow(clippy::too_many_arguments)]
pub fn throw(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, way: V3, tower: usize, partner: Option<usize>, life: i32, spread: f32, damage: f32, reach: f32) -> Option<usize> {
    let class = w.class(NODE)?.scale;
    let n = w.create(NODE);
    let g = &mut ctx.game.rng;
    let (r2, r1, r0) = (g.rand_angle(), g.rand_angle(), g.rand_angle());
    let m = &mut w.mobys[n];
    (m.update_dist, m.draw_dist, m.drawn) = (0xff, 0xff, 1);
    m.rot = [r0, r1, r2];
    m.scale = class;
    if m.pvars.len() < 0x40 { m.pvars.resize(0x40, 0); }
    ai::set_link(m, N_TOWER, Some(tower));
    m.mode |= 0x2200;
    m.pos = pos;
    m.set_pv_v(N_WAY, way);
    m.set_pv_v(N_VEL, [0.0; 3]);
    m.set_pv_f(N_SPREAD, spread);
    m.set_pv_f(N_DAMAGE, damage);
    ai::set_link(m, N_PARTNER, partner);
    m.set_pv_i(N_TIMER, life);
    m.set_pv_f(N_LIFE, life as f32);
    m.set_pv_f(N_REACH, reach);
    m.alpha = 0;
    if partner.is_none() { m.timer = 1; }
    if seq(w, n, ctx) != 0 { ctx.host.set_anim(&w.mobys[n], 0, 0, 1); }
    let m = &mut w.mobys[n];
    m.drawn = 0;
    ctx.host.set_anim_speed(&w.mobys[n], 0.5);
    let m = &mut w.mobys[n];
    m.mode |= 1;
    m.set_state(0, -1);
    w.rebuild(n, &*ctx.host);
    w.changed = true;
    Some(n)
}

fn alive(w: &MobyWorld, k: Option<usize>) -> Option<usize> { k.filter(|&k| w.mobys.get(k).is_some_and(|m| m.state != 0xfe && m.state != 0xfd && m.state < 0x80)) }

/// `0x3f7e30(m, f)`: across the tower's facing, the node eased `f` of the way out to the spread on its side; its
/// height `f` of the way to 0.8 over the tower.
fn spread(w: &mut MobyWorld, id: usize, f: f32) {
    let Some(t) = ai::link(&w.mobys[id], N_TOWER).and_then(|t| w.mobys.get(t)) else { return };
    let (tp, rows) = (t.pos, t.rows);
    let m = &mut w.mobys[id];
    let v = sub(m.pos, tp);
    let mut l = [dot(rows[0], v), dot(rows[1], v), dot(rows[2], v)];
    let r = m.pv_f(N_SPREAD);
    l[1] += (l[1] / l[1].abs()) * (r - l[1].abs()) * f;
    let v = add(add(scale(rows[0], l[0]), scale(rows[1], l[1])), scale(rows[2], l[2]));
    m.pos = add(v, tp);
    m.pos[2] += (tp[2] + RISE - m.pos[2]) * f;
}

/// `0x3f7f48`.
fn fly(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let v = m.pv_v(N_VEL);
    let v = add(v, scale(sub(m.pv_v(N_WAY), v), VEL_EASE));
    m.set_pv_v(N_VEL, v);
    m.pos = add(m.pos, v);
}

fn on_wrap(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !wrapped(w, id, ctx) { return; }
    if seq(w, id, ctx) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 10); }
    ctx.host.set_anim_speed(&w.mobys[id], 2.0);
}

/// `0x3f7fb8` (module docs).
pub fn update_0d09(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for k in 0..3 {
        let d = ctx.game.rng.randf(SPIN.0 * 0.017_453_294, SPIN.1 * 0.017_453_294);
        let m = &mut w.mobys[id];
        m.rot[k] = add_rot(m.rot[k], d);
    }
    let tower = alive(w, ai::link(&w.mobys[id], N_TOWER));
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            on_wrap(w, id, ctx);
            let m = &w.mobys[id];
            let k = 1.0 - m.pv_i(N_TIMER) as f32 / m.pv_f(N_LIFE);
            spread(w, id, SPREAD_GROW * (k * k));
            let mut t = w.mobys[id].pv_i(N_TIMER);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(N_TIMER, t);
            if out { w.mobys[id].set_state(2, -1); }
            if tower.is_none() { w.mobys[id].set_state(3, -1); }
        }
        2 | 3 => {
            on_wrap(w, id, ctx);
            spread(w, id, SPREAD_FLY);
            fly(w, id);
            let far = tower.is_none_or(|t| w.mobys[id].pv_f(N_REACH) < dist2(w.mobys[id].pos, w.mobys[t].pos));
            if far { w.mobys[id].set_state(3, -1); }
            if w.mobys[id].state == 3 {
                let m = &mut w.mobys[id];
                if m.alpha < ALPHA_DOWN {
                    beams::Beams::release(w, Kind::Dobbo, id);
                    w.delete(id);
                    return;
                }
                m.scale *= SHRINK;
                m.alpha -= ALPHA_DOWN;
            }
        }
        _ => {}
    }
    let m = &w.mobys[id];
    let d = dist2(m.pos, ctx.hero.pos);
    let partner = alive(w, ai::link(m, N_PARTNER));
    if d < m.pv_f(N_SPREAD) * 2.0 + 1.0 {
        let owner = tower.filter(|&t| w.mobys[t].o_class == TOWER);
        if let (Some(t), Some(p)) = (owner, partner.filter(|&p| w.mobys[p].o_class == NODE)) {
            let mut hit = false;
            let damage = w.mobys[id].pv_f(N_DAMAGE);
            if w.mobys[id].timer == 1 {
                let mut a = w.mobys[id].pos;
                let mut b = w.mobys[p].pos;
                crate::moby::classes::level08::tabora_ai::jitter(&mut ctx.game.rng, &mut a, 0.1);
                crate::moby::classes::level08::tabora_ai::jitter(&mut ctx.game.rng, &mut b, 0.1);
                hit = ai::attack_line(w, t, ctx, a, b, 0x21_0001, damage, 1.0, 0).is_some_and(|h| Some(h) == ctx.hero_moby);
            }
            if ctx.game.rng.randi(4) == 0 && d < 2.0 {
                let m = &w.mobys[id];
                let class = w.class(NODE).map_or(1.0, |c| c.scale);
                let r = m.scale * 0.4 / class;
                let pos = m.pos;
                if 0 < ai::attack_sphere(w, t, ctx, r, damage, 1.0, pos, 0x21_0001, 0, 1, 0) { hit = true; }
            }
            let m = &mut w.mobys[id];
            let mut h = m.pv_i(N_HIT);
            if countdown(&mut h) && hit { h = 60; }
            m.set_pv_i(N_HIT, h);
        }
    }
    let m = &mut w.mobys[id];
    if m.state != 3 {
        m.alpha = m.alpha.wrapping_add(ALPHA_UP);
        if ALPHA_MOST < m.alpha { m.alpha = ALPHA_MOST; }
    }
    if m.timer == 1 {
        if let Some(p) = partner {
            let at = w.mobys[id].pos;
            let r0 = set_len(sub(w.mobys[p].pos, at), 1.0);
            // Row 1 is left unset by the game (stack garbage): world up here.
            let up = [0.0, 0.0, 1.0];
            beams::Beams::report_dobbo(w, id, [r0, up, cross(r0, up), at]);
        }
    }
}
