//! The Hypnomatic (item 0x37, crate::hero::items::ROWS; its record on every level, L18 `0x26eca0`: attachment 1,
//! stance 1, use sequence 0x38 (the firing arm's 0x3e crouched, none moving), the use keeps the stance, icon 0xea84,
//! name 0x3305, no ammo): the hand moby **0x0950** (L00 `0x3ba758` through its class's `0x3bb100`, the same code on
//! every level; L18 `0x3dcd30`; the decomp's `func_L00_003B9310`). Held ○ throws its beam at a creature of the
//! capture (a gunner 0x0ee1 of endless lives, crate::moby::classes::level18::capturable) and the beam reaching it
//! hooks it: the creature's capture begins.
//!
//! **Its pvars** (`0x3b9310`'s record): `+0x00` the target (a link), `+0x04` its class, `+0x06` the lock's rest,
//! `+0x08` the target's aim height, `+0x0c` the candidate, `+0x10` the lock's ticks, `+0x14` first person, `+0x18`
//! set up, `+0x20` the beam's start, `+0x30` its way, `+0x40` / `+0x44` the beam's scroll and length (the draw
//! callback's), `+0x48` held, `+0x4c` the loop sound; the port's `+0x50` / `+0x54` the search's cone and reach as the
//! tick before left them (the globals `0x1b2860` / `0x1b2864`, read before they are set).
//!
//! **Each tick**: its first sets it up (no target, the lock's ticks 5, the beam off, no sound). The search's reach
//! is 35. In the look stance 0x1d with Ratchet hidden (first person) the beam starts at the camera, the cone 6°, the
//! search's angle 5° (and a target is dropped once on the way in); else from 0.75 up Ratchet's moby, its start the
//! hand's joint 1, both 15°. The lock's rest runs down; a target gone, of no health, not drawn, or with the arm raised
//! at a wall (`0x18c30c`) is dropped (else it is the hero's aim target `0x18aa18`). With the lock's rest out, no
//! landing lag and the arm not raised, the search along the camera's forward (flat in 0x1d): the Lock On mod's lock
//! (`0x18b270`, crate::moby::lock_on) while it lives with health; else the first of the aim search's list (`0x3257a0`: angle, cone, 35, 45°, 35°; twenty) that is a
//! gunner of endless lives with health, within its own reach (`+0x3e4`, 0: any); else what the line 35 ahead (flags
//! 0, Ratchet's moby left out) meets, the same test. The same candidate 5 ticks running is the target (the rest 5,
//! its aim height); none, no target. In 0x1d the reticle (sprite 0x26, `0x600fff0f`) at the screen's centre; a target
//! has the reticle on it (`0xff0fff0f`, its size `|0.9 − 0.0043·distance|` from the camera) and the beam's way is
//! toward it.
//!
//! **The use** (not in 0x1c or 0x31): from 1 the look stance 0x1d; on the ground, no item lock (`0x18c0dc`), ○ held
//! and in the hand 16 ticks: the use stance (`0x2af5e0`) and its class sound 0 looped (mode 4) on its first tick, a
//! new press the beam over again; the beam ([`beam`]) while it shows, ○ is held or he is off the ground; ○ let go
//! (after 10 ticks in use, or out of use, or off the ground) the sound stopped and the use over (`0x2af848`).
//!
//! **The beam** ([`beam`], the length and the hook of its draw callback `0x3b9938`): on while ○ is held, on the
//! ground and no landing lag; its reach the target's distance across (none, or not drawn: 2); its length grows 100
//! a second toward it while on, else falls 140 a second (not below 0), else is the reach; on and within 0.5 of the
//! target: the length and the scroll 0, Ratchet held 30 ticks (`0x18a000`), the target hooked (`0x3458f0` = L00
//! `0x32a818`), and Ratchet's checkpoint (`0x359b20`). The scroll runs 3 a second, wrapped at 1.4.
//!
//! Not ported: the beam's quads (the draw callback's ribbon of 0.7-spaced segments, its colours pulsing by three sines,
//! `0x3b9938`); the hand's barrel aim (`0x324208` on the record `0x23db20`); the classes the line test sees through
//! (`0x3ba5b8`: 0x1146, 0x1185, 0x119b, 0x11a4, 0x1204, 0x11fc, 0x120c, 0x1135, 0x1148, their collision `+0x98` off
//! for the search); the search with ○ held preferring the target (its own list's second pass is the same test); the
//! hold flag `0x1afc90` (0 on every level, never set); the body's branches (the beam from the body's joint 5 and its
//! gun's aim record `0x25be10`: the item pass does not run while a body plays).

use super::level18::capturable::{GUNNER, PVARS};
use crate::hero::physics::{add, dist, dist2, scale, set_len, sub, V3};
use crate::moby::{ai, aim, countdown, lock_on, Ctx, MobyWorld};

/// The item and its hand's class.
pub const ITEM: i32 = 0x37;
pub const HAND_CLASS: i16 = 0x950;

const TARGET: usize = 0x00;
const TARGET_CLASS: usize = 0x04;
const REST: usize = 0x06;
const HEIGHT: usize = 0x08;
const CANDIDATE: usize = 0x0c;
const LOCK: usize = 0x10;
const FIRST_PERSON: usize = 0x14;
const SET_UP: usize = 0x18;
const START: usize = 0x20;
const WAY: usize = 0x30;
const SCROLL: usize = 0x40;
const LENGTH: usize = 0x44;
const HELD: usize = 0x48;
const SOUND: usize = 0x4c;
const PREV_CONE: usize = 0x50;
const PREV_REACH: usize = 0x54;

/// `$gp+0xca8` / `+0xca4` (`0x1afc98` / `0x1afc94`): the ticks a candidate takes to lock, the rest after a lock.
const LOCK_TICKS: i32 = 5;
const REST_TICKS: i16 = 5;
/// The search's reach (`0x1b2864` = 35).
const REACH: f32 = 35.0;
/// `$gp+0xcb4` / `+0xcdc` / `+0xce0` / `+0xcc8` / `+0xcb8`: the beam's reach with no target, its growth and its fall a
/// second, its scroll a second, its segments' spacing.
const BARE: f32 = 2.0;
const GROW: f32 = 100.0;
const FALL: f32 = 140.0;
const SCROLL_SPEED: f32 = 3.0;
const SPACING: f32 = 0.7;
/// The capture's own pvars: its lives (0x80: endless), its health, its reach.
const LIVES: usize = 0x3b1;
const HEALTH: usize = 0x20;
const OWN_REACH: usize = 0x3e4;
const TICK: f32 = 0.016_666_668;
const RETICLE: u16 = 0x26;

fn gone(s: u8) -> bool { s == 0xfe || s == 0xfd }

/// A gunner of endless lives (with health when `health`), its own reach (`+0x3e4`, 0: any) beyond the distance
/// across `d(it)` (`0x3b8e58`'s test; the line's fallback measures from the beam's start to the hand).
fn capturable(w: &MobyWorld, k: usize, health: bool, d: impl Fn(&crate::moby::Moby) -> f32) -> bool {
    let Some(m) = w.mobys.get(k) else { return false };
    if gone(m.state) || m.o_class != GUNNER || m.pvars.len() < PVARS { return false; }
    if m.pv_b(LIVES) & 0x80 == 0 || (health && m.pv_f(HEALTH) <= 0.0) { return false; }
    let r = m.pv_f(OWN_REACH);
    r == 0.0 || d(m) < r
}

/// `0x3ba758` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < crate::moby::PVAR_BYTES { w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0); }
    if w.mobys[id].pv_i(SET_UP) == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_s(TARGET_CLASS, -1);
        m.set_pv_i(LOCK, LOCK_TICKS);
        m.set_pv_f(SCROLL, 0.0);
        m.set_pv_f(LENGTH, 0.0);
        ai::set_link(m, TARGET, None);
        m.set_pv_s(REST, 0);
        m.set_pv_f(HEIGHT, 0.0);
        m.set_pv_i(FIRST_PERSON, 0);
        m.set_pv_i(SOUND, -1);
        m.set_pv_i(SET_UP, 1);
    }
    let (reach, cone) = (w.mobys[id].pv_f(PREV_REACH), w.mobys[id].pv_f(PREV_CONE));
    w.mobys[id].set_pv_f(PREV_REACH, REACH);
    let hidden = ctx.hero_moby.and_then(|h| w.mobys.get(h)).is_some_and(|m| m.mode & 1 != 0);
    let first_person = ctx.hero.state == 0x1d && hidden;
    let origin;
    let angle;
    if first_person {
        origin = ctx.host.camera_pos();
        w.mobys[id].set_pv_v(START, origin);
        w.mobys[id].set_pv_f(PREV_CONE, 6.0 * 0.017_453_294);
        angle = 5.0 * 0.017_453_294;
        if w.mobys[id].pv_i(FIRST_PERSON) == 0 { ai::set_link(&mut w.mobys[id], TARGET, None); }
        w.mobys[id].set_pv_i(FIRST_PERSON, 1);
    } else {
        let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows[2], |m| m.rows[2]);
        origin = add(scale(up, 0.75), ctx.hero.pos);
        let start = ctx.host.joint(&w.mobys[id], 1).unwrap_or(w.mobys[id].pos);
        w.mobys[id].set_pv_v(START, start);
        angle = 15.0 * 0.017_453_294;
        w.mobys[id].set_pv_f(PREV_CONE, angle);
        w.mobys[id].set_pv_i(FIRST_PERSON, 0);
    }
    {
        let m = &mut w.mobys[id];
        let mut r = m.pv_s(REST) as i32;
        countdown(&mut r);
        m.set_pv_s(REST, r as i16);
    }
    // The target kept, or dropped.
    if let Some(t) = ai::link(&w.mobys[id], TARGET) {
        let health = aim::health(w, t);
        let keep = w.mobys.get(t).is_some_and(|m| !gone(m.state) && m.drawn != 0) && health.is_none_or(|(h, _)| 0.0 < h) && !ctx.hero.items.raised;
        if keep {
            ctx.hero.items.aim_target = Some((t, w.mobys[id].pv_f(HEIGHT)));
        } else {
            let m = &mut w.mobys[id];
            ai::set_link(m, TARGET, None);
            m.set_pv_s(TARGET_CLASS, -1);
            m.set_pv_f(HEIGHT, 0.0);
        }
    }
    if !ctx.hero.items.raised && w.mobys[id].pv_s(REST) == 0 && ctx.hero.land_lag == 0 {
        let mut dir = ctx.host.camera_rows()[0];
        if ctx.hero.state == 0x1d { dir[2] = 0.0; }
        let dir = set_len(dir, 1.0);
        w.mobys[id].set_pv_v(WAY, dir);
        let candidate = search(w, id, ctx, origin, dir, angle, cone, reach);
        let m = &mut w.mobys[id];
        match candidate {
            Some(c) => {
                if m.pv_i(CANDIDATE) - 1 == c as i32 {
                    let mut t = m.pv_i(LOCK);
                    let out = countdown(&mut t);
                    m.set_pv_i(LOCK, t);
                    if out {
                        let h = m.pv_i(SOUND);
                        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                        let height = aim::health(w, c).map_or(0.0, |(_, a)| a);
                        let class = w.mobys[c].o_class;
                        let m = &mut w.mobys[id];
                        ai::set_link(m, TARGET, Some(c));
                        m.set_pv_i(SOUND, -1);
                        m.set_pv_s(TARGET_CLASS, class);
                        m.set_pv_s(REST, REST_TICKS);
                        m.set_pv_f(HEIGHT, height);
                    }
                } else {
                    m.set_pv_i(LOCK, LOCK_TICKS);
                }
            }
            None => {
                m.set_pv_s(TARGET_CLASS, -1);
                m.set_pv_i(LOCK, LOCK_TICKS);
                ai::set_link(m, TARGET, None);
                m.set_pv_f(HEIGHT, 0.0);
            }
        }
        w.mobys[id].set_pv_i(CANDIDATE, candidate.map_or(0, |c| c as i32 + 1));
    }
    if ctx.hero.state == 0x1d { crate::hud::marker(&mut ctx.game.markers, 1.0, id, 0x600f_ff0f, None, RETICLE); }
    if let Some(t) = ai::link(&w.mobys[id], TARGET).filter(|&t| t < w.mobys.len()) {
        let at = aim::aim_point(w, t);
        let d = dist(ctx.host.camera_pos(), at);
        crate::hud::marker(&mut ctx.game.markers, (0.9 - d * 0.006_666_667 * 0.65).abs(), id, 0xff0f_ff0f, Some(at), RETICLE);
        let start = w.mobys[id].pv_v(START);
        w.mobys[id].set_pv_v(WAY, set_len(sub(at, start), 1.0));
    }
    if matches!(ctx.hero.state, 0x1c | 0x31) { return; }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let it = ctx.hero.items;
    let grounded = ctx.hero.grounded_ticks != 0 && ctx.hero.air_ticks == 0;
    let on = it.held & it.trigger != 0;
    if grounded && it.no_use == 0 && on && 16 <= it.item_ticks {
        if w.mobys[id].pv_i(HELD) == 0 {
            ctx.hero.items.use_start(ctx.hero.group);
            loop_sound(w, id, ctx);
            w.mobys[id].set_pv_i(HELD, 1);
        }
        if it.pressed & it.trigger != 0 {
            w.mobys[id].set_pv_f(SCROLL, 0.0);
            w.mobys[id].set_pv_f(LENGTH, 0.0);
            loop_sound(w, id, ctx);
        }
    }
    if 0.0 < w.mobys[id].pv_f(SCROLL) || on || !grounded { beam(w, id, ctx); }
    let it = ctx.hero.items;
    if !on && (10 <= it.use_ticks || !it.using || !grounded) {
        let h = w.mobys[id].pv_i(SOUND);
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
        let idle = ctx.hero.idle_seq();
        ctx.hero.items.use_end(ctx.hero.state, ctx.hero_view.seq_b, idle);
        w.mobys[id].set_pv_i(HELD, 0);
    }
}

/// Its class sound 0 looped (mode 4) unless it plays it already.
fn loop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    if h != -1 && ctx.host.sound_playing(h) { return; }
    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
    w.mobys[id].set_pv_i(SOUND, h);
}

/// `0x3ba2a0` (module docs): the creature the beam would take.
#[allow(clippy::too_many_arguments)]
fn search(w: &MobyWorld, id: usize, ctx: &Ctx, origin: V3, dir: V3, angle: f32, cone: f32, reach: f32) -> Option<usize> {
    // The Lock On mod's lock (`0x18b270`) while it lives (crate::moby::lock_on::live).
    if let Some(t) = lock_on::live(w, ctx) { return Some(t); }
    let c = aim::Cone { yaw: angle, pitch: cone, range: reach, near_pitch: std::f32::consts::FRAC_PI_4, near_yaw: f32::from_bits(0x3f1c_61aa) };
    let list = aim::search_list(w, ctx, c, origin, dir, false, 20);
    if let Some(&k) = list.iter().find(|&&k| capturable(w, k, true, |m| dist2(origin, m.pos))) { return Some(k); }
    let to = add(scale(dir, REACH), origin);
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, origin, to, crate::collision::QueryFlags(0), ctx.hero_moby))?;
    let k = hit.moby?;
    let at = w.mobys[id].pos;
    capturable(w, k, false, |_| dist2(origin, at)).then_some(k)
}

/// The beam's length and its hook (module docs).
fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let it = ctx.hero.items;
    let grounded = ctx.hero.grounded_ticks != 0 && ctx.hero.air_ticks == 0;
    let on = it.held & it.trigger != 0 && grounded && ctx.hero.land_lag == 0;
    let target = ai::link(&w.mobys[id], TARGET).filter(|&t| w.mobys.get(t).is_some_and(|m| m.drawn != 0));
    let reach = target.map_or(BARE, |t| dist2(w.mobys[id].pos, w.mobys[t].pos));
    let (grow, fall) = (GROW * TICK, FALL * TICK);
    let m = &mut w.mobys[id];
    let mut l = m.pv_f(LENGTH);
    if l < reach - grow && on {
        l += grow;
    } else if reach + fall < l || !on {
        l = (l - fall).max(0.0);
    } else {
        l = reach;
    }
    let mut scroll = m.pv_f(SCROLL);
    if let (true, Some(t)) = (on && reach - 0.5 < l, ai::link(m, TARGET)) {
        l = 0.0;
        scroll = 0.0;
        ctx.hero.land_lag = 30;
        if let Some(c) = w.mobys.get_mut(t) { c.once |= 4; }
        if ctx.hero.mode == 0 { ctx.game.save_checkpoint(ctx.hero.pos, ctx.hero.rot); }
    }
    let m = &mut w.mobys[id];
    if reach < l { l = reach; }
    scroll += SCROLL_SPEED * TICK;
    if SPACING + SPACING < scroll { scroll -= SPACING + SPACING; }
    m.set_pv_f(LENGTH, l);
    m.set_pv_f(SCROLL, scroll);
}
