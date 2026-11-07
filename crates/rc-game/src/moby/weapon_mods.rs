//! The weapon mods' effects on a hit (Slim Cognito's mods, crate::vendor::mods: slot 0 Shock, slot 1 Acid): two
//! shared systems every weapon with the mod calls on a moby it hit, each a 30-slot table its manager moby runs.
//!
//! **Who may take one** (both starts, `0x3f9f70` / `0x4126b8(moby, item, owner)`): the item's base (its record's
//! `+0x3c`, else the item) with the mod on (`0x1a7a88[base]` bit 3 for Acid, bit 1 for Shock: the vendor's
//! "on" bit `2·slot + 1`); a moby of class type 5 in the target list (mode 0x1000), alive (`+0x20` ≥ 0, collision
//! `+0x98`), with a health block (`0x31fa20`) and health left, not in the table already (`0x3f9f18` / `0x412658`), a
//! free slot. The bank of the tuning (0 the base, 1 the Mega's): the base's current row (`0x139568`) a vendor row
//! (`−0x10`) of level 2 or 3, or 1 for the R&C1 weapons (crate::vendor::SPECIALS). The tuning is by the item the
//! weapon passes (its upgraded row's id for the upgraded weapon), [`ACID`] / [`SHOCK`]. The slot keeps the moby's
//! colour: its reaction block's (`+0x10`'s `+0x04..+0x06`, when its word `+0x00` ≠ 0), else its ambient (`0x313330`).
//!
//! **Acid** (`0x3f9f70`; the manager **0x0d04**'s update `0x3f9c50`, created at the level start `0x3fa2a8` →
//! `0x3fa268`): the slot (`0x253330`, 0x20 bytes) lasts the tuning's seconds and drains its rate a second
//! (`0x2536f0`: by 1/60 a tick). Each tick: the pulse phase on by 0.20944 (`0x2efe60`); the moby gone (`+0x20` < 0,
//! `+0x98` 0), without health or its time out (`0x2eeab0`) ends it; its health down by the rate, at 0 or below a
//! kill hit (0.001, flags 0x810000, backwards along its row 0 with the no-limit 5627.925, kind its own class
//! `<< 16 | 0x0105`, `0x31d378`) and the end. An end puts the colour back (`0x3f9e98`: the ambient and the reaction
//! block's). Else, past its first tick, the moby's ambient pulses between its colour and green `0x008000`
//! (`0x2f0000((cos phase + 1) / 2)`). Its draw callback (`0x3fa4e0`): two green puffs a tick about each moby
//! ([`acid_puffs`]).
//!
//! **Shock** (`0x4126b8`; the manager **0x1136**'s update `0x412228`, created at the level start `0x412ae8` →
//! `0x412aa8`): the slot (`0x2565a0`, 0x30 bytes) costs the moby the tuning's cost at once (`0x259240`; at 0 or below
//! the kill hit above and no slot) and lasts its seconds. Each tick: the phase, the end tests and the time as Acid's;
//! every `b` seconds (`0x2eeb40`, from its first tick) the first moby within 8 of it (`0x2e14b0`, flags 0, not itself)
//! of class type 5 in the target list and not deleted takes an arc that hits it for the tuning's `a`, and a second
//! arc only drawn; one tick in eleven (`rand() % 11`) an arc only drawn about the moby itself (two random points
//! 0.3..0.65 of its radius out, 5..15 ticks to reach, 15..30 to last); then the end or the pulse to blue `0xff0000`.
//!
//! **The arcs** (48 of 0xd0 bytes at `0x256b40`: made by `0x412da8`, run by `0x413030` after the slots, drawn by the
//! manager's draw callback `0x412cb8` → `0x413448`): from a moby's aim point (its place and up × aim height,
//! `0x326448`; its place without a health block) to another's, each end plus its offset; two wobbles of three angles
//! (±π, ±0.001..0.005 a tick — a hundred times that while it reaches — 3.25..4.75 out) give the control offsets; the
//! far end reaches the target over the grow ticks; reached, a hitting arc hits the target once (its `a`, flags
//! 0x210000, along the arc at 1/30, kind `0x1136 << 16 | 0x0305`, `0x31d378`) with ten sparks (`0x413e28(at, 10, 1)` =
//! crate::moby::classes::plasma_coil's sparks), and runs its life out. The draw ([`draw`]): 24 points along the
//! cubic (`0x413610`) jittered ±0.075, the last extrapolated (`0x413908`), as the strip `0x413970` (FX 0x2b, 0.05 and
//! 0.2 across, fixed alpha 0..0x40 in its last 10 ticks, else 0x40..0x80); a reached arc also two glows at its end
//! (`0x2edf78(0.5, 0.1, roll, end, 0x31, ..)`: `0x103080`, white, alpha 0x50..0x60).
//!
//! The port has no manager mobys: [`tick`] runs both updates after the moby loop and [`draw`] after the moby
//! callbacks. The kill and arc hits take the slot's owner as their source (the original's is the manager, its
//! `+0xb8` set to that owner first). The draw callback's acid puffs are made in the tick.
//!
//! Users (every L00 caller of `0x3f9f70` / `0x4126b8`): crate::moby::classes::{blitz, blitz_mega, bouncer, chopper,
//! gravity_bomb (with the Mini-Nuke's blast), hoverbomb, lava_gun, meteor, minirocket, plasma_coil, plasma_storm,
//! spiderbot, spiderbot_plus} (Acid) and {pulse_rifle, seeker, shot, synthenoid, synthenoid_bomb, turret, vaporizer}
//! (Shock). The third mod, Lock On (slot 2), is not an effect on a hit: these do not serve it.
//!
//! Not ported: the Heavy Lancer's shot 0x0c89 (`0x420358`: Shock as item 0x3c; its class is not ported). The weapons'
//! statistics counters (`0x18c8dc..`) are no part of these functions.

use crate::hero::physics::{add, scale, sub, V3};
use crate::moby::classes::plasma_coil::{self, Wobble};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, ai, countdown, mode, Ctx, MobyWorld};

/// The slots of each table.
pub const SLOTS: usize = 30;
/// The arcs (`0x256b40`).
pub const ARCS: usize = 48;
/// The managers' classes (the arcs' kind word names the Shock one).
pub const ACID_CLASS: i16 = 0x0d04;
pub const SHOCK_CLASS: i16 = 0x1136;
/// The vendor's "on" bits (`0x1a7a88[base]`): Shock slot 0, Acid slot 1.
const SHOCK_ON: u8 = 1 << 1;
const ACID_ON: u8 = 1 << 3;
/// The pulse colours (`r | g << 8 | b << 16`).
const ACID_TINT: u32 = 0x00_8000;
const SHOCK_TINT: u32 = 0xff_0000;
/// The phase's step a tick (`0x3e567752`).
const PHASE_STEP: f32 = f32::from_bits(0x3e56_7752);
/// The kill hit's damage (`0x3a83126f`) and the no-limit word (`0x45afdf66`).
const KILL_DAMAGE: f32 = f32::from_bits(0x3a83_126f);
const NO_LIMIT: f32 = f32::from_bits(0x45af_df66);

/// `0x2536f0` (set by `0x3fa2a8`): Acid's `(damage a second, seconds)` by bank and item (0 for the others).
pub fn acid_tuning(bank: usize, item: i32) -> (f32, i32) {
    match (bank, item) {
        (0, 0x2a | 0x47) => (1.0, 5),
        (0, 0x16 | 0x41 | 0x1a | 0x44) => (1.0, 3),
        (0, 0x1d | 0x3f | 0x25 | 0x4c | 0x1b | 0x45 | 0x1c | 0x3e | 0x20 | 0x4e | 0x19 | 0x46) => (2.0, 4),
        (1, 0x2a | 0x47) => (9.0, 8),
        (1, 0x16 | 0x41 | 0x1a | 0x44 | 0x19 | 0x46) => (3.0, 4),
        (1, 0x1d | 0x3f | 0x25 | 0x4c) => (3.0, 6),
        (1, 0x1b | 0x45) => (2.0, 5),
        (1, 0x1c | 0x3e | 0x20 | 0x4e) => (5.0, 4),
        _ => (0.0, 0),
    }
}

/// `0x259240` (set by `0x412ae8`): Shock's `(cost, arc damage, seconds, arc interval)` by bank and item (the
/// interval 1 for every item, the rest 0 for the others).
pub fn shock_tuning(bank: usize, item: i32) -> (f32, f32, i32, f32) {
    let (cost, a, secs) = match (bank, item) {
        (0, 0x17 | 0x42) => (10.0, 5.0, 2),
        (0, 0x18 | 0x43) => (2.0, 2.0, 2),
        (0, 0x1e | 0x3c) => (2.0, f32::from_bits(0x3f2b_851f), 4),
        (0, 0x1f | 0x3d) | (1, 0x1f | 0x3d) => (1.0, 1.0, 1),
        (0, 0x29 | 0x40) => (1.0, 2.0, 3),
        (1, 0x17 | 0x42) => (10.0, 8.0, 3),
        (1, 0x18 | 0x43) => (6.0, 2.0, 3),
        (1, 0x1e | 0x3c) => (10.0, 4.0, 6),
        (1, 0x29 | 0x40) => (2.0, 4.0, 3),
        _ => (0.0, 0.0, 0),
    };
    (cost, a, secs, 1.0)
}

/// An Acid slot (`0x253330 + 0x20·k`: `+0x00` its ticks, `+0x04` / `+0x08` the colours, `+0x0c` the phase, `+0x10`
/// the drain a tick, `+0x14` the moby, `+0x18` the owner, `+0x1c` the first tick's hold).
#[derive(Clone, Copy, Debug, Default)]
pub struct AcidSlot {
    pub ticks: i32,
    pub colour: u32,
    pub tint: u32,
    pub phase: f32,
    pub drain: f32,
    pub moby: Option<usize>,
    pub owner: Option<usize>,
    pub hold: i32,
}

/// A Shock slot (`0x2565a0 + 0x30·k`: `+0x00` its ticks, `+0x04` / `+0x08` the colours, `+0x0c` the phase, `+0x10`
/// the arcs' damage, `+0x14` their interval, `+0x18` the moby, `+0x1c` the interval's countdown, `+0x20` the owner,
/// `+0x24` the first tick's hold).
#[derive(Clone, Copy, Debug, Default)]
pub struct ShockSlot {
    pub ticks: i32,
    pub colour: u32,
    pub tint: u32,
    pub phase: f32,
    pub damage: f32,
    pub interval: f32,
    pub moby: Option<usize>,
    pub wait: f32,
    pub owner: Option<usize>,
    pub hold: i32,
}

/// An arc (`0x256b40 + 0xd0·k`: `+0x00` / `+0x10` the control offsets, `+0x20` / `+0x30` its ends, `+0x40` / `+0x50`
/// the ends' offsets, `+0x60` / `+0x84` the wobbles, `+0xa8` its life, `+0xac` its manager, `+0xb0` / `+0xb4` the
/// target and the source moby, `+0xb8` / `+0xbc` the reach's ticks and their count, `+0xc0` hit, `+0xc4` a hitting arc,
/// `+0xc8` its damage, `+0xcc` its slot).
#[derive(Clone, Copy, Debug, Default)]
pub struct Arc {
    pub live: bool,
    pub ctrl: [V3; 2],
    pub from: V3,
    pub to: V3,
    pub offsets: [V3; 2],
    pub halves: [Wobble; 2],
    pub life: i32,
    pub target: Option<usize>,
    pub source: Option<usize>,
    pub grow: i32,
    pub grow_total: i32,
    pub hit: bool,
    pub harms: bool,
    pub damage: f32,
    pub slot: usize,
}

/// The two tables and the arcs (MobyWorld::mods; a level's start empties them).
#[derive(Clone, Debug)]
pub struct Mods {
    pub acid: [AcidSlot; SLOTS],
    pub shock: [ShockSlot; SLOTS],
    pub arcs: Vec<Arc>,
}

impl Default for Mods {
    fn default() -> Self { Mods { acid: [AcidSlot::default(); SLOTS], shock: [ShockSlot::default(); SLOTS], arcs: vec![Arc::default(); ARCS] } }
}

/// The starts' common part: the bank when the item's base has the mod `on` and `target` may take it (module docs).
fn bank(w: &MobyWorld, ctx: &Ctx, target: usize, item: i32, on: u8) -> Option<usize> {
    let items = &ctx.level.items;
    let base = items.get(usize::try_from(item).ok()?).map_or(0, |r| r.base as i32);
    let base = if base == 0 { item } else { base };
    if ctx.game.mods_bought.get(usize::try_from(base).ok()?).is_none_or(|&b| b & on == 0) { return None; }
    let alt = items.get(ctx.game.item_row(base)).is_some_and(|r| {
        r.mega && (r.class == 2 || (r.class == 1 && crate::vendor::SPECIALS.contains(&base)) || r.class == 3)
    });
    let m = w.mobys.get(target)?;
    if w.class(m.o_class).map_or(0, |c| c.kind) != 5 || m.mode & mode::TARGET == 0 || 0x80 <= m.state || !m.has_collision {
        return None;
    }
    aim::health(w, target).filter(|&(hp, _)| 0.0 < hp)?;
    Some(alt as usize)
}

/// The colour a slot keeps (`r | g << 8 | b << 16`): the reaction block's, else the ambient (`0x313330`).
fn colour_of(w: &MobyWorld, id: usize) -> u32 {
    let m = &w.mobys[id];
    let rgb = match (m.mode & ai::AI != 0).then(|| ai::react_of(m)).flatten() {
        Some(r) if m.pv_s(r) != 0 => [m.pv_b(r + 4), m.pv_b(r + 5), m.pv_b(r + 6)],
        _ => m.ambient,
    };
    rgb[0] as u32 | (rgb[1] as u32) << 8 | (rgb[2] as u32) << 16
}

/// `0x3f9e98` / `0x4125d8`: the slot's colour back on the moby (its ambient, and its reaction block's colour when
/// it has one).
fn restore(w: &mut MobyWorld, id: usize, c: u32) {
    let Some(m) = w.mobys.get_mut(id) else { return };
    m.set_ambient_word(c);
    if m.mode & ai::AI == 0 { return; }
    let Some(r) = ai::react_of(m) else { return };
    if m.pv_s(r) == 0 { return; }
    m.set_pv_b(r + 4, c as u8);
    m.set_pv_b(r + 5, (c >> 8) as u8);
    m.set_pv_b(r + 6, (c >> 16) as u8);
}

/// The kill hit both mods deal when the health runs out: 0.001, flags 0x810000, backwards along its row 0, kind its
/// own class `<< 16 | 0x0105` (`0x31d158` + `0x31d378`).
fn kill(w: &mut MobyWorld, id: usize, source: Option<usize>) {
    let m = &w.mobys[id];
    let d = scale(m.rows[0], -1.0);
    let mut t = HitTemplate::new(KILL_DAMAGE, source, 0x81_0000, [d[0], d[1], d[2], NO_LIMIT]);
    t.kind = (m.o_class as u16 as u32) << 16 | 0x0105;
    w.deliver_hit_any(id, &t, [0.0; 4]);
}

/// The moby's health block offset (`0x31fa20`; mode 0x20).
fn health_at(w: &MobyWorld, id: usize) -> Option<usize> {
    w.mobys.get(id).filter(|m| m.mode & ai::AI != 0).map(ai::health)
}

/// `0x3f9f70(moby, item, owner)`: Acid on a moby the weapon `item` hit (module docs). Whether a slot was taken.
pub fn acid(w: &mut MobyWorld, ctx: &Ctx, target: usize, item: i32, owner: Option<usize>) -> bool {
    let Some(bank) = bank(w, ctx, target, item, ACID_ON) else { return false };
    if w.mods.acid.iter().any(|s| s.moby == Some(target)) { return false; }
    let Some(k) = w.mods.acid.iter().position(|s| s.moby.is_none()) else { return false };
    let (rate, secs) = acid_tuning(bank, item);
    let colour = colour_of(w, target);
    w.mods.acid[k] = AcidSlot { ticks: secs * 60, colour, tint: ACID_TINT, phase: 0.0, drain: rate * f32::from_bits(0x3c88_8889), moby: Some(target), owner, hold: 1 };
    true
}

/// `0x4126b8(moby, item, owner)`: Shock on a moby the weapon `item` hit (module docs). Whether a slot was taken
/// (also when its cost killed the moby).
pub fn shock(w: &mut MobyWorld, ctx: &Ctx, target: usize, item: i32, owner: Option<usize>) -> bool {
    let Some(bank) = bank(w, ctx, target, item, SHOCK_ON) else { return false };
    if w.mods.shock.iter().any(|s| s.moby == Some(target)) { return false; }
    let Some(k) = w.mods.shock.iter().position(|s| s.moby.is_none()) else { return false };
    let (cost, damage, secs, interval) = shock_tuning(bank, item);
    let Some(h) = health_at(w, target) else { return false };
    let hp = w.mobys[target].pv_f(h) - cost;
    w.mobys[target].set_pv_f(h, hp);
    let colour = colour_of(w, target);
    w.mods.shock[k] = ShockSlot { ticks: secs * 60, colour, tint: SHOCK_TINT, phase: 0.0, damage, interval, moby: Some(target), wait: 0.0, owner, hold: 1 };
    if hp <= 0.0 {
        kill(w, target, owner);
        w.mods.shock[k].moby = None;
    }
    true
}

/// `0x2eeb40`: a countdown in seconds, a frame (1/60) at a time; true when out.
fn count_secs(x: &mut f32) -> bool {
    if *x <= 1.0 / 60.0 {
        *x = 0.0;
        true
    } else {
        *x -= 1.0 / 60.0;
        false
    }
}

/// The pulse of a slot's moby: `0x2f0000((cos phase + 1) / 2, colour, tint)` as its ambient (`0x313300`).
fn pulse(w: &mut MobyWorld, id: usize, phase: f32, colour: u32, tint: u32) {
    let c = crate::hud::lerp_rgba((phase.cos() + 1.0) * 0.5, colour, tint);
    w.mobys[id].set_ambient_word(c);
}

/// `0x3f9c50` then `0x412228` (with `0x413030`): both managers' ticks (module docs), after the moby loop.
pub fn tick(w: &mut MobyWorld, ctx: &mut Ctx) {
    if w.mods.acid.iter().any(|s| s.moby.is_some()) { tick_acid(w, ctx); }
    if w.mods.shock.iter().any(|s| s.moby.is_some()) || w.mods.arcs.iter().any(|a| a.live) {
        tick_shock(w, ctx);
        tick_arcs(w, ctx);
    }
}

/// `0x3f9c50`: the Acid slots (module docs), then the puffs its draw callback makes.
fn tick_acid(w: &mut MobyWorld, ctx: &mut Ctx) {
    for k in 0..SLOTS {
        let mut s = w.mods.acid[k];
        let Some(id) = s.moby else { continue };
        s.phase = crate::hero::physics::add_rot(s.phase, PHASE_STEP);
        let m = &w.mobys[id];
        let mut end = !m.has_collision || 0x80 <= m.state;
        match health_at(w, id) {
            None => end = true,
            Some(h) => {
                let hp = w.mobys[id].pv_f(h) - s.drain;
                w.mobys[id].set_pv_f(h, hp);
                if hp <= 0.0 {
                    end = true;
                    kill(w, id, s.owner);
                }
            }
        }
        if countdown(&mut s.ticks) { end = true; }
        if end {
            restore(w, id, s.colour);
            s.moby = None;
        } else if s.hold == 0 {
            pulse(w, id, s.phase, s.colour, s.tint);
        } else {
            s.hold -= 1;
        }
        w.mods.acid[k] = s;
    }
    acid_puffs(w, ctx);
}

/// `0x3fa4e0` (the Acid manager's draw callback): for each slot's moby two puffs (`0x335bb8`: still, 1..1.01 growth,
/// 0.25..1 × 210000, turning 1..5, rising 1/60..1/40 a tick, colour word `0x5a182`) at a random point 0.5..0.75 of its
/// radius from its centre; each its frame 0x44 or 0x48 at random, `+0x09` 0x44, 35 ticks (`+0x0a`, `+0x2b`),
/// `+0x24` 3, `+0x3c` 5.0, `+0x2a` 0x40..0x80.
fn acid_puffs(w: &MobyWorld, ctx: &mut Ctx) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for s in &w.mods.acid {
        let Some(id) = s.moby else { continue };
        let m = &w.mobys[id];
        let centre = scale([m.bsphere[0], m.bsphere[1], m.bsphere[2]], f32::from_bits(0x3a7f_fbce));
        for _ in 0..2 {
            let rise = g.randf(f32::from_bits(0x3c88_8889), f32::from_bits(0x3ccc_ccce));
            let r = g.randf(0.5, 0.75);
            let (yaw, pitch) = (g.rand_angle(), g.rand_angle());
            let at = add(crate::moby::classes::shot::polar(m.bsphere[3] * r * f32::from_bits(0x3a7f_fbce), yaw, pitch), centre);
            let spin = g.rand_range(1, 5);
            let size = g.randf(0.25, 1.0) * 210_000.0;
            let Some(i) = crate::particles::puff(sys, g, 0.0, 1.0, f32::from_bits(0x3f81_47ae), size, at, spin, [0.0, 0.0, rise], 0x5_a182, 1) else { continue };
            let frame = if g.rand() % 2 == 0 { 0x44 } else { 0x48 };
            let life = g.rand_range(0x40, 0x80) as u8;
            let r = &mut sys.pool.recs[i];
            r[3] = frame;
            r[0xa..0xc].copy_from_slice(&0x23i16.to_le_bytes());
            r[9] = 0x44;
            r[0x3c..0x40].copy_from_slice(&5.0f32.to_le_bytes());
            r[0x24..0x28].copy_from_slice(&3u32.to_le_bytes());
            r[0x2a] = life;
            r[0x2b] = 0x23;
        }
    }
}

/// `0x412228`: the Shock slots (module docs).
fn tick_shock(w: &mut MobyWorld, ctx: &mut Ctx) {
    for k in 0..SLOTS {
        let mut s = w.mods.shock[k];
        let Some(id) = s.moby else { continue };
        s.phase = crate::hero::physics::add_rot(s.phase, PHASE_STEP);
        let mut end = 0x80 <= w.mobys[id].state || !w.mobys[id].has_collision;
        if countdown(&mut s.ticks) { end = true; }
        if count_secs(&mut s.wait) {
            s.wait = s.interval;
            let centre = w.mobys[id].pos;
            let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, centre, 8.0, crate::collision::QueryFlags(0), Some(id))).unwrap_or_default();
            w.hits.last_target = near.first().copied();
            let other = near.into_iter().find(|&o| {
                let m = &w.mobys[o];
                m.mode & mode::TARGET != 0 && m.state != 0xfe && m.state != 0xfd && w.class(m.o_class).map_or(0, |c| c.kind) == 5
            });
            if let Some(o) = other {
                start_arc(w, ctx, id, o, [[0.0; 3]; 2], true, 10, 30, k, s.damage);
                start_arc(w, ctx, id, o, [[0.0; 3]; 2], false, 10, 30, k, 0.0);
            }
        }
        if ctx.game.rng.rand() % 11 == 0 {
            let g = &mut ctx.game.rng;
            let radius = w.mobys[id].bsphere[3];
            let mut point = || {
                let r = g.randf(f32::from_bits(0x3e99_999a), f32::from_bits(0x3f26_6666)) * f32::from_bits(0x3a7f_fbce);
                let (yaw, pitch) = (g.rand_angle(), g.rand_angle());
                crate::moby::classes::shot::polar(radius * r, yaw, pitch)
            };
            let (a, b) = (point(), point());
            let grow = g.rand_range(5, 15);
            let life = g.rand_range(15, 30);
            start_arc(w, ctx, id, id, [a, b], false, grow, life, k, 0.0);
        }
        if end {
            restore(w, id, s.colour);
            s.moby = None;
        } else if s.hold == 0 {
            pulse(w, id, s.phase, s.colour, s.tint);
        } else {
            s.hold -= 1;
        }
        w.mods.shock[k] = s;
    }
}

/// `0x412da8(damage, manager, from, to, from offset, to offset, harms, grow, life, slot)`: the first free arc.
#[allow(clippy::too_many_arguments)]
fn start_arc(w: &mut MobyWorld, ctx: &mut Ctx, from: usize, to: usize, offsets: [V3; 2], harms: bool, grow: i32, life: i32, slot: usize, damage: f32) {
    let Some(k) = w.mods.arcs.iter().position(|a| !a.live) else { return };
    let g = &mut ctx.game.rng;
    let pi = std::f32::consts::PI;
    let mut halves = [Wobble::default(); 2];
    for h in halves.iter_mut() { for x in h.angles.iter_mut() { *x = g.randf(-pi, pi); } }
    for h in halves.iter_mut() { for x in h.speeds.iter_mut() { *x = g.randf_signed(f32::from_bits(0x3a83_126f), f32::from_bits(0x3ba3_d70a)); } }
    for h in halves.iter_mut() { for x in h.amps.iter_mut() { *x = g.randf(3.25, 4.75); } }
    w.mods.arcs[k] = Arc {
        live: true, ctrl: [[0.0; 3]; 2], from: [0.0; 3], to: [0.0; 3], offsets, halves, life, target: Some(to), source: Some(from),
        grow, grow_total: grow, hit: false, harms, damage, slot,
    };
}

/// An arc's end at moby `m`: its offset plus the moby's aim point (place + up × aim height, `0x326448`), or the
/// moby's place without a health block.
fn end_at(w: &MobyWorld, ctx: &Ctx, m: Option<usize>, offset: V3) -> V3 {
    let Some(i) = m.filter(|&i| i < w.mobys.len()) else { return offset };
    let x = &w.mobys[i];
    match aim::health(w, i) {
        Some((_, h)) => add(add(scale(ctx.hero.gravity.up_at(x.pos), h), x.pos), offset),
        None => x.pos,
    }
}

/// `0x413030`: the arcs (module docs).
fn tick_arcs(w: &mut MobyWorld, ctx: &mut Ctx) {
    for k in 0..ARCS {
        let mut a = w.mods.arcs[k];
        if !a.live { continue; }
        a.from = end_at(w, ctx, a.source, a.offsets[0]);
        let f = if a.grow == 0 { 1.0 } else { 100.0 };
        a.ctrl = [a.halves[0].step(f), a.halves[1].step(f)];
        let to = end_at(w, ctx, a.target, a.offsets[1]);
        let t = 1.0 - a.grow as f32 / a.grow_total as f32;
        a.to = std::array::from_fn(|i| a.from[i] + (to[i] - a.from[i]) * t);
        if countdown(&mut a.grow) {
            let alive = a.target.and_then(|t| w.mobys.get(t)).is_some_and(|m| m.has_collision);
            if alive && !a.hit && a.harms {
                let d = scale(sub(a.to, a.from), f32::from_bits(0x3d08_8889));
                let owner = w.mods.shock[a.slot].owner;
                let mut tm = HitTemplate::new(a.damage, owner, 0x21_0000, [d[0], d[1], d[2], 0.0]);
                tm.kind = (SHOCK_CLASS as u16 as u32) << 16 | 0x0305;
                if let Some(t) = a.target { w.deliver_hit_any(t, &tm, [0.0; 4]); }
                a.hit = true;
                plasma_coil::sparks(ctx, a.to, 10, 1);
            }
            if countdown(&mut a.life) { a.live = false; }
        }
        w.mods.arcs[k] = a;
    }
}

/// `0x413448` (the Shock manager's draw callback, by `0x412cb8`): every arc's strip (`0x413730`), and a reached arc's
/// two glows at its end (module docs).
pub fn draw(w: &MobyWorld, g: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    for a in w.mods.arcs.iter().filter(|a| a.live) {
        let (ca, cb) = (add(a.ctrl[0], a.from), add(a.ctrl[1], a.to));
        let mut points: Vec<V3> = (0..0x18)
            .map(|i| {
                let p = plasma_coil::cubic(i as f32 / 23.0, a.from, a.to, ca, cb);
                let v = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
                add(p, scale(v, f32::from_bits(0x3d99_999a)))
            })
            .collect();
        let fix = if a.life < 10 { g.rand_range(0, 0x40) } else { g.rand_range(0x40, 0x80) };
        plasma_coil::extrapolate(&mut points);
        plasma_coil::strip(out, &points, cam, [0x2b, 0x2b], [f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3e4c_cccd)], [0x7fff_ffff, 0x80ff_0000, 0x7f7f_7f7f, 0x7f7f_7f7f], [0x80ff_0000; 4], fix);
        if a.grow == 0 {
            for rgb in [0x10_3080u32, 0xff_ffff] {
                let alpha = g.rand_range(0x50, 0x60) as u32;
                let roll = g.rand_angle();
                crate::fx::sprite(out, 0.5, f32::from_bits(0x3dcc_cccd), roll, a.to, cam, 0x31, alpha << 24 | rgb, true);
            }
        }
    }
}
