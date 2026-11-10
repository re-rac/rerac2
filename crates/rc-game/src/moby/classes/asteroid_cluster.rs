//! The orbits' asteroid cluster, class **0x0d3d** (L22 `0x3f5a78`, 124 placed; L23 `0x3fcc40`, 120 placed; the
//! decomp's `func_L22_003F45D8`; L22 addresses below). The moby is a hidden carrier: what shows is the tie group it
//! sits on, which it carries along (crate::moby::attach, its frame its rows), wrapped in a column of shield layers that
//! take the hits. Level 22's skill point watcher 0x12db counts them (crate::moby::classes::level22::props::update_12db).
//! Updated from any distance once set up.
//!
//! * **0** ([`set_up`]): the tie group with a tie nearest it in xy (under 2, else it goes) and that tie's class
//!   (`+0xa4`); its rows from its angles; the shrub group of the same index (−1 past the shrub groups). Its layers
//!   (`+0x14`): (the group's highest tie over the height centre − its own + 4.5) / 4, cut to an integer (its height
//!   `+0xa0` = 4 each); its up row kept (`+0x00`). Each layer, class [`LAYER_C4C`] over a tie group of kind 0xc4c,
//!   else [`LAYER`], at its place 4 more up its up row than the last, its angles and rows (the first two scaled 1.08
//!   more each layer), mode |= 0x4100 (its rows kept, hit by lines), rebuilt (`+0x18 + 4k`). It takes up to 90
//!   members, the ties then the shrubs (`0x329728(m, +0xd0, 90, −1, ties, shrubs, 7, 0)`), its frame its rows
//!   (`+0x11d`); health (`+0xac`) 50; then 1.
//! * **1** ([`run`]): each layer's hit with the flag 0x80000 ([`HIT`]; one without it let go); outside the cool-down
//!   (`+0xa8`) and short of killing it, the first of the group's ties of the kinds 0xc4a, 0xc4c, 0xc4f, 0xc50, 0xc55
//!   or 0xf5f within 2 of the layer turns to debris ([`convert`]). The cool-down out and a layer hit, the health
//!   down by its damage: at 0 the cluster breaks (2); else it starts to sway ([`sway_start`]) and the cool-down is
//!   40 ticks. The layers' hits let go; the sway ([`sway`]); with `+0xc8` set (its placed pvars: nothing here writes
//!   it) its rows turn ½° a tick about their up; its ties hidden or shown by the camera's side ([`hide`]); swaying or
//!   turning, its members carried (`0x329c38`).
//! * **2** ([`destroy`]): the level's sound 0x10 past the split (`0x364178(0x10, 0, 0)`), its dust ([`dust`]), its
//!   layers and debris deleted, its ties hidden; then 3, which does nothing.
//!
//! **The debris** ([`convert`]): a layer without debris yet makes the tie kind's first class ([`debris_of`]); one with
//! debris of another class than the kind's second replaces it with the second (the carrier's member that was the old
//! debris now the new); one already the second makes none. The new debris takes the tie's member in the carrier's
//! block (`+0x4c` 1 → 0, `+0x48` the moby's slot), the layer's place, angles and light (`0x32efc0` = L00 `0x32a748`),
//! the tie's rows (mode |= 0x100), and the tie is hidden. (The switch also lists 0xc51, 0xf61, 0xf83, 0xf84, 0xf8a,
//! 0xf8b, 0xf8d, 0xf9a, 0xf9b, 0xffd..0x1000 → 0xdc9, 0x1218..0x1223, which the kind test before it never lets
//! through.)
//!
//! pvars: `+0x00` its up row at set-up, `+0x10` the tie group, `+0x14` the layer count, `+0x16` the shrub group,
//! `+0x18` the layers, `+0x50` their debris, `+0xa0` the height, `+0xa4` the nearest tie's class, `+0xa8` the
//! cool-down, `+0xac` the health, `+0xb0` the sway's heading, `+0xbc` its size, `+0xc0` its phase step, `+0xc4` its
//! phase, `+0xc8` turning, `+0xcc` the ties hidden, `+0xd0` the carrier's block (crate::moby::MobyWorld::attached).
//!
//! The engine functions (L22): `0x3168d0` / `0x316c38` create / delete, `0x319320` rebuild, `0x2f5048` =
//! crate::moby::attach::rows_of, `0x2f49e0` = crate::hero::physics::dist2 (xy), `0x2f49a8` = dist, `0x2f48f0` = dot,
//! `0x3231c0` the hit with a mask (L00 `0x31dce8`), `0x2f4458` the timer (L00 `0x2eeab0`), `0x323ff0` =
//! crate::hero::physics::approach, `0x2f5390` = crate::moby::attach::turn, `0x329728` / `0x329c38` =
//! crate::moby::MobyWorld::attach / carry, `0x32b130(m, v, out, 0)` a vector into its frame, `0x32b440(p, out, 1)` =
//! crate::gravity::Gravity::up_at, `0x323db0` the lerp (L00 `0x31e8d8`), `0x364178` =
//! crate::moby::MobyHost::play_level_sound_after (L00 `0x3607e8`), `0x335c80` = crate::particles::staged, `0x2f5990`
//! = crate::hud::lerp_rgba. The camera `0x1b9c00` (L23 `0x1ba5c0`) is crate::moby::MobyHost::camera_pos; the height
//! centre `0x1b1d10` is crate::gravity::Gravity::height_centre. The L23 copies read the same values from `$gp+0x50`
//! further on.
//!
//! **The rocks** it throws when it breaks ([`throw_rocks`], `0x3f6778`, L23 `0x3fd940`, before the dust):
//! (60 / the camera's distance; above 2.2, 2.7)² · height · 0.55 of them, at most the free moby slots (`0x1b1fc0`,
//! crate::moby::MobyWorld::free_slots) · 0.15. Each one is a 0x0a80 (`0x3d1e68`, L23 `0x3d9518`,
//! crate::moby::classes::thrown_rock::spawn) at its share of the height up the column, ±(that factor plus a tenth
//! of its height) across, with random angles.
//!
//! Not ported:
//! * the level's explosion at it (L22 `0x3a4f80` → `0x3a4df8`: a pool of 50 a kind of three effect classes,
//!   `0x3fc6f8` / `0x3fdc80` / `0x3fe6e8`; L23 its own `0x41dcd8`): no port of either;
//! * its shrubs hidden and shown with its ties (`0x1b25f8 + 0x20·i` `+0x16` bit 0): the port has no hidden shrubs;
//! * the list of tie kinds the clusters sit on (`0x255750`, its count `$gp+0x19b8`; a new kind appended at set-up):
//!   nothing ported reads it;
//! * the ties' update period the carrier draws (`7`; crate::moby::attach keeps none).
//!
//! Uncertain: the port loads no level sphere (crate::gravity), so the height centre is the origin on these levels
//! unless something sets it, and the layer count follows from that.

use crate::gravity::{to_local, to_world};
use crate::hero::physics::{add, add_rot, approach, atan2, dist, dist2, dot, scale, sub, V3};
use crate::moby::attach::{rows_of, turn, Kind};
use crate::moby::hits::HitRecord;
use crate::moby::{ai, mode, Ctx, Moby, MobyWorld};

pub const CLUSTER: i16 = 0x0d3d;
/// The shield layers: over a tie group of kind 0xc4c, and the rest.
pub const LAYER_C4C: i16 = 0x0dcb;
pub const LAYER: i16 = 0x0dcc;

mod pv {
    pub const BASIS: usize = 0x00;
    pub const TIES: usize = 0x10;
    pub const COUNT: usize = 0x14;
    pub const SHRUBS: usize = 0x16;
    pub const LAYERS: usize = 0x18;
    pub const DEBRIS: usize = 0x50;
    pub const HEIGHT: usize = 0xa0;
    pub const KIND: usize = 0xa4;
    pub const COOL: usize = 0xa8;
    pub const HEALTH: usize = 0xac;
    pub const HEADING: usize = 0xb0;
    pub const AMP: usize = 0xbc;
    pub const STEP: usize = 0xc0;
    pub const PHASE: usize = 0xc4;
    pub const TURNING: usize = 0xc8;
    pub const HIDDEN: usize = 0xcc;
}

/// The hit flag the layers answer.
const HIT: u32 = 0x8_0000;
/// The nearest tie's xy distance it must be within.
const NEAR: f32 = 2.0;
/// A tie within this of a hit layer turns to debris.
const REACH: f32 = 2.0;
/// The layers' gap up the up row and the scale of each over the last.
const GAP: f32 = 4.0;
const GROW: f32 = 1.08;
/// The carrier's members at most.
const MEMBERS: usize = 90;
const HEALTH: f32 = 50.0;
const COOL_DOWN: i32 = 40;
/// The turn about its up a tick with `+0xc8` set (0x3c0efa37, ½°).
const TURN: f32 = f32::from_bits(0x3c0e_fa37);
/// `$gp+0x19d4` / `+0x19d8` / `+0x19dc`: the sway's size, its fade a second, its phase step in arcminutes a tick
/// (× π/10800, 0x3998825c).
const SWAY_SIZE: f32 = 0.025;
const SWAY_FADE: f32 = 0.025;
const SWAY_RATE: f32 = 720.0;
const ARCMIN: f32 = f32::from_bits(0x3998_825c);
/// `$gp+0x19bc..+0x19c8`: the hiding threshold's ends (one layer → fourteen) on the camera's up · its up; in the
/// game's mode 4 (`0x1a8f00`) the second pair.
const HIDE: [(f32, f32); 2] = [(-0.15, -0.47), (-0.3, -0.7)];
/// `$gp+0x19e0`: the dust's density; `$gp+0x19e8..`: its colours (two pairs lerped), `$gp+0x19f8` its two sizes,
/// `$gp+0x1a08` its fade-in at most, `$gp+0x1a0c` / `+0x1a10` its ease, `$gp+0x1a04` its frame (−1).
const DUST_DENSITY: f32 = 0.3;
const DUST_COLOURS: [u32; 4] = [0x8080_8080, 0x8040_6080, 0x2080_8080, 0x2040_6080];
const DUST_SIZE: [f32; 2] = [6.0, 5.0];
const DUST_IN: i32 = 5;
const DUST_EASE: (i32, i32) = (60, 180);
/// The dust column's radius at the bottom.
const DUST_WIDTH: f32 = 2.7;

fn tie_pos(m: &[[f32; 4]; 4]) -> V3 { [m[3][0], m[3][1], m[3][2]] }
fn tie_rows(m: &[[f32; 4]; 4]) -> [V3; 3] { std::array::from_fn(|r| [m[r][0], m[r][1], m[r][2]]) }
fn tie_class(w: &MobyWorld, t: usize) -> i32 { w.tie_classes.get(t).copied().unwrap_or(-1) }
fn group(w: &MobyWorld, g: i32) -> Option<Vec<u16>> { usize::try_from(g).ok().and_then(|g| w.tie_groups.get(g)).cloned() }
fn count(m: &Moby) -> usize { m.pv_s(pv::COUNT).max(0) as usize }

/// `0x3231c0(m, mask, 0)`: its hit record when it has one of `mask`'s flags; one without them is let go.
fn hit_with(w: &mut MobyWorld, id: usize, mask: u32) -> Option<HitRecord> {
    let h = w.hit_message(id)?;
    if h.tmpl.flags & mask == 0 {
        w.clear_hit(id);
        return None;
    }
    Some(h)
}

/// `0x2f4458(&t)`: true when the timer is out, else one down.
fn timer_out(m: &mut Moby, off: usize) -> bool {
    let t = m.pv_i(off);
    if t == 0 { return true; }
    m.set_pv_i(off, t - 1);
    false
}

/// The debris classes of a tie kind: the first hit's and the second's.
pub fn debris_of(class: i32) -> Option<(i16, i16)> {
    Some(match class {
        0x0c4a => (0x0dc2, 0x0dc3),
        0x0c4c => (0x0dc4, 0x0dc5),
        0x0c4f => (0x0dc6, 0x0dc7),
        0x0c50 => (0x0dc8, 0x0dc8),
        0x0c55 => (0x0dca, 0x0dca),
        0x0f5f => (0x1217, 0x1217),
        _ => return None,
    })
}

/// A group's ties hidden or shown (the runtime tie's `+0x1e` bit 2).
fn ties_hidden(w: &mut MobyWorld, g: i32, on: bool) {
    let Some(list) = group(w, g) else { return };
    for t in list {
        if on { w.hidden_ties.insert(t as usize); } else { w.hidden_ties.remove(&(t as usize)); }
    }
}

pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => set_up(w, id, ctx),
        1 => run(w, id, ctx),
        2 => destroy(w, id, ctx),
        _ => {}
    }
}

/// State 0 (module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let (mut best, mut nearest, mut kind) = (9999.0f32, -1i32, -1i32);
    for (g, list) in w.tie_groups.iter().enumerate() {
        for &t in list {
            let Some(m) = w.ties.get(t as usize) else { break };
            let d = dist2(tie_pos(m), pos);
            if d < best {
                (best, nearest, kind) = (d, g as i32, tie_class(w, t as usize));
            }
        }
    }
    if nearest == -1 || NEAR < best {
        w.delete(id);
        return;
    }
    let rows = rows_of(w.mobys[id].rot);
    let shrubs = if (nearest as usize) < w.shrub_groups.len() { nearest as i16 } else { -1 };
    // The highest tie over the height centre (the kinds 0xca4, 0xca5 and 0xca7 left out).
    let centre = ctx.hero.gravity.height_centre;
    let mut top = 0.0f32;
    for t in group(w, nearest).unwrap_or_default() {
        let t = t as usize;
        let Some(m) = w.ties.get(t) else { break };
        if matches!(tie_class(w, t), 0x0ca4 | 0x0ca5 | 0x0ca7) { continue; }
        let d = dist(tie_pos(m), centre);
        if top < d { top = d; }
    }
    let n = (((top - dist(pos, centre)) + 4.5) * 0.25) as i32 as i16;
    let (rot, light, ambient) = {
        let m = &mut w.mobys[id];
        m.rows = rows;
        m.set_pv_i(pv::TIES, nearest);
        m.set_pv_i(pv::KIND, kind);
        m.set_pv_s(pv::SHRUBS, shrubs);
        m.set_pv_s(pv::COUNT, n);
        m.set_pv_f(pv::HEIGHT, n as f32 * 4.0);
        m.set_pv_v(pv::BASIS, rows[2]);
        (m.rot, m.light, m.ambient)
    };
    let (mut grow, mut offset) = (1.0f32, 0.0f32);
    for k in 0..n.max(0) as usize {
        let c = w.create(if kind == 0x0c4c { LAYER_C4C } else { LAYER });
        {
            let l = &mut w.mobys[c];
            l.pos = add(pos, to_world([0.0, 0.0, offset], &rows));
            l.rot = rot;
            (l.light, l.ambient) = (light, ambient);
            l.rows = [scale(rows[0], grow), scale(rows[1], grow), rows[2]];
            l.mode |= mode::KEEP_ROWS | mode::LINE;
        }
        offset += GAP;
        w.rebuild(c, &*ctx.host);
        ai::set_link(&mut w.mobys[id], pv::LAYERS + 4 * k, Some(c));
        grow *= GROW;
    }
    w.attach(id, MEMBERS, -1, nearest, shrubs as i32);
    w.attach_use_rows(id);
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::HEALTH, HEALTH);
    m.update_dist = 0xff;
    m.state = 1;
}

/// State 1 (module docs).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = count(&w.mobys[id]);
    let mut hit = None;
    for k in 0..n {
        let Some(layer) = ai::link(&w.mobys[id], pv::LAYERS + 4 * k) else { continue };
        let Some(h) = hit_with(w, layer, HIT) else { continue };
        hit = Some(h);
        let m = &w.mobys[id];
        if m.pv_i(pv::COOL) != 0 || !(h.tmpl.damage < m.pv_f(pv::HEALTH)) { continue; }
        convert(w, id, ctx, k, layer);
    }
    let out = timer_out(&mut w.mobys[id], pv::COOL);
    if let (true, Some(h)) = (out, hit) {
        let m = &mut w.mobys[id];
        let mut health = m.pv_f(pv::HEALTH);
        approach(0.0, h.tmpl.damage, &mut health);
        m.set_pv_f(pv::HEALTH, health);
        if health == 0.0 {
            m.state = 2;
            return;
        }
        sway_start(w, id, ctx);
        w.mobys[id].set_pv_i(pv::COOL, COOL_DOWN);
    }
    for k in 0..n {
        if let Some(layer) = ai::link(&w.mobys[id], pv::LAYERS + 4 * k) { w.clear_hit(layer); }
    }
    sway(&mut w.mobys[id]);
    let turning = w.mobys[id].pv_i(pv::TURNING) != 0;
    if turning {
        let m = &mut w.mobys[id];
        m.rows = turn(&m.rows, &rows_of([0.0, 0.0, TURN]));
    }
    hide(w, id, ctx);
    if w.mobys[id].pv_f(pv::AMP) != 0.0 || turning { w.carry(id, &*ctx.host); }
}

/// Layer `k`'s hit near a tie of the group (module docs, "The debris"): the first matching tie within reach turns.
fn convert(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: usize, layer: usize) {
    let Some(list) = group(w, w.mobys[id].pv_i(pv::TIES)) else { return };
    let at = w.mobys[layer].pos;
    let slot = pv::DEBRIS + 4 * k;
    for t in list {
        let t = t as usize;
        let Some(tie) = w.ties.get(t).copied() else { break };
        let Some((first, second)) = debris_of(tie_class(w, t)) else { continue };
        if REACH < dist(tie_pos(&tie), at) { continue; }
        let made = match ai::link(&w.mobys[id], slot) {
            None => Some(w.create(first)),
            Some(old) if w.mobys[old].o_class == second => None,
            Some(old) => {
                let c = w.create(second);
                w.delete(old);
                if let Some(car) = w.attached.get_mut(&id) {
                    for m in car.members.iter_mut().filter(|m| m.kind == Kind::Moby && m.index == old) { m.index = c; }
                }
                ai::set_link(&mut w.mobys[id], slot, Some(c));
                Some(c)
            }
        };
        if let Some(c) = made {
            if let Some(m) = w.attached.get_mut(&id).and_then(|car| car.members.iter_mut().find(|m| m.kind == Kind::Tie && m.index == t)) {
                (m.kind, m.index) = (Kind::Moby, c);
            }
            w.adopt(c, Some(layer), &*ctx.host);
            ai::set_link(&mut w.mobys[id], slot, Some(c));
            let d = &mut w.mobys[c];
            d.rows = tie_rows(&tie);
            d.mode |= mode::KEEP_ROWS;
            // The port's: a carrier moves it (crate::moby::Moby::drive).
            d.drive = true;
            w.hidden_ties.insert(t);
        }
        break;
    }
}

/// `0x3f6958` (L23 `0x3fdb20`): the sway starts, across the way to the camera in its frame (±90° at random from it),
/// [`SWAY_SIZE`] big, its phase 0.
fn sway_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cam = ctx.host.camera_pos();
    let side = if ctx.game.rng.randi(2) == 0 { -std::f32::consts::FRAC_PI_2 } else { std::f32::consts::FRAC_PI_2 };
    let m = &mut w.mobys[id];
    let l = to_local(sub(cam, m.pos), &m.rows);
    m.set_pv_f(pv::HEADING, add_rot(side, atan2(l[0], l[1])));
    m.set_pv_f(pv::AMP, SWAY_SIZE);
    m.set_pv_f(pv::PHASE, 0.0);
    m.set_pv_f(pv::STEP, SWAY_RATE * ARCMIN);
}

/// `0x3f6a20` (L23 `0x3fdbe8`): swaying, its up row the kept one plus (cos, sin) of the heading · size · sin(phase)
/// through its rows; the phase on by its step, the size down [`SWAY_FADE`] a second. Still, its up row put back once.
fn sway(m: &mut Moby) {
    let size = m.pv_f(pv::AMP);
    if size == 0.0 {
        if m.pv_f(pv::STEP) != 0.0 {
            m.rows[2] = m.pv_v(pv::BASIS);
            m.set_pv_f(pv::STEP, 0.0);
        }
        return;
    }
    let s = size * m.pv_f(pv::PHASE).sin();
    let h = m.pv_f(pv::HEADING);
    let off = to_world([h.cos() * s, h.sin() * s, 0.0], &m.rows);
    m.rows[2] = add(m.pv_v(pv::BASIS), off);
    m.set_pv_f(pv::PHASE, add_rot(m.pv_f(pv::PHASE), m.pv_f(pv::STEP)));
    let mut size = size;
    approach(0.0, SWAY_FADE * 0.016_666_668, &mut size);
    m.set_pv_f(pv::AMP, size);
}

/// `0x3f6d98` (L23 `0x3fdf60`): its ties hidden once its up turns from the camera's (the dot of the two under the
/// threshold lerped by layers / 14 between [`HIDE`]'s ends), shown again once it is past it.
fn hide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let up = ctx.hero.gravity.up_at(ctx.host.camera_pos());
    let m = &w.mobys[id];
    let d = dot(m.rows[2], up);
    let f = m.pv_s(pv::COUNT) as f32 * 0.071_428_575;
    let (a, b) = HIDE[(ctx.game.scenes.mode == 4) as usize];
    let thr = a + (b - a) * f;
    let (hidden, ties) = (m.pv_i(pv::HIDDEN) != 0, m.pv_i(pv::TIES));
    if hidden && thr < d {
        w.mobys[id].set_pv_i(pv::HIDDEN, 0);
        ties_hidden(w, ties, false);
    } else if !hidden && d < thr {
        w.mobys[id].set_pv_i(pv::HIDDEN, 1);
        ties_hidden(w, ties, true);
    }
}

/// State 2 (module docs).
fn destroy(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_level_sound_after(None, 0x10, 0);
    throw_rocks(w, id, ctx);
    dust(w, id, ctx, true);
    let n = count(&w.mobys[id]);
    for base in [pv::LAYERS, pv::DEBRIS] {
        for k in 0..n {
            if let Some(c) = ai::link(&w.mobys[id], base + 4 * k) { w.delete(c); }
        }
    }
    let ties = w.mobys[id].pv_i(pv::TIES);
    if 0 <= ties { ties_hidden(w, ties, true); }
    w.mobys[id].state = 3;
}

/// `0x3f6778` (L23 `0x3fd940`): the rocks of its break (module docs, "The rocks").
fn throw_rocks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut f = 60.0 / dist(ctx.host.camera_pos(), w.mobys[id].pos);
    if f32::from_bits(0x400c_cccd) < f { f = f32::from_bits(0x402c_cccd); }
    let height = w.mobys[id].pv_f(pv::HEIGHT);
    let n = f * f * height * f32::from_bits(0x3f0c_cccd);
    let step = height / n;
    let n = n.min(w.free_slots() as f32 * f32::from_bits(0x3e19_999a));
    let mut i = 0;
    while (i as f32) < n {
        let up = i as f32 * step;
        let across = f + up * 0.1;
        let g = &mut ctx.game.rng;
        let (x, y) = (g.randf_signed(0.0, across), g.randf_signed(0.0, across));
        let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
        let (rows, pos) = (w.mobys[id].rows, w.mobys[id].pos);
        let at = add(to_world([x, y, up], &rows), pos);
        crate::moby::classes::thrown_rock::spawn(w, ctx, id, at, rot);
        i += 1;
    }
}

/// `0x3f6b28(m, full)` (L23 `0x3fdcf0`): height · 7.29 · [`DUST_DENSITY`] puffs (a fifth of them without `full`) up
/// its column, each at its share of the height up its up row and ±(2.7 + a tenth of that) across, still; its colours
/// lerped at random within [`DUST_COLOURS`]' pairs, one of the two sizes, in 1..5 ticks, eased 60..180, out 60.
fn dust(w: &MobyWorld, id: usize, ctx: &mut Ctx, full: bool) {
    let m = &w.mobys[id];
    let (pos, rows, height) = (m.pos, m.rows, m.pv_f(pv::HEIGHT));
    let mut n = height * 7.290_000_4 * DUST_DENSITY;
    if !full { n *= 0.2; }
    let step = height / n;
    let mut i = 0;
    while (i as f32) < n {
        let up = i as f32 * step;
        let across = up * 0.1 + DUST_WIDTH;
        let g = &mut ctx.game.rng;
        let (x, y) = (g.randf_signed(0.0, across), g.randf_signed(0.0, across));
        let at = add(to_world([x, y, up], &rows), pos);
        let c0 = crate::hud::lerp_rgba(g.randf(0.0, 1.0), DUST_COLOURS[0], DUST_COLOURS[1]);
        let c1 = crate::hud::lerp_rgba(g.randf(0.0, 1.0), DUST_COLOURS[2], DUST_COLOURS[3]);
        let size = DUST_SIZE[g.randi(2) as usize];
        let fade_in = g.rand_range(1, DUST_IN);
        let ease = g.rand_range(DUST_EASE.0, DUST_EASE.1);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::staged(sys, &mut ctx.game.rng, at, [0.0, 0.0, 0.0, size], [0.0, 0.0, 0.0, size], c0, c1, fade_in as i16, ease as i16, 60, -1);
        }
        i += 1;
    }
}
