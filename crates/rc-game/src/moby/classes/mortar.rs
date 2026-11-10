//! The mortar **0x0c1a** ([`update_0c1a`], L17 `0x3f1f48`, 4 placed; the same code as L08 `0x3f7ab0`, 10 placed;
//! the decomp's `func_US2_L08_003F7AB0`, its helpers v1.01 `func_L08_003F6348` / `003F6568` / `003F6018` at our L17
//! `0x3f1c80` / `0x3f1ea0` / `0x3f1950`; the addresses below are L17's): a pot that lobs the members of its group at Ratchet one by one — Smolg's
//! hatchlings (crate::moby::classes::level17::hatchling), Tabora's burrowers and breathers (crate::moby::classes::
//! level08::burrower / breather, which fly in along the knock it gives them, their start mode 2). Level 4's launcher
//! (crate::moby::classes::launcher) of another design: the pool, the target search, the landing cuboids.
//!
//! **Its pvars:** health `+0x20` (1), the flash `+0x50`, its knock block `+0x60` (crate::moby::knock), the target
//! block `+0xc0` (crate::moby::target_search: its moby `+0x100`, the result `+0x104`), the reach `+0x120`, the
//! cuboid `+0x124` and path `+0x128` the search keeps to (−1 none), the reload `+0x12c` and its timer `+0x12e`
//! (u16), the most out at once `+0x130` and how many are `+0x131`, `+0x132` ≠ 0 the landing cuboid nearest Ratchet
//! (else a random one), `+0x133` ≠ 0 its mobys at a tenth of their class's size, its two groups `+0x134` / `+0x138`
//! (their members, all deleted once the game runs, are its pool), `+0x140..+0x150` four landing cuboids (−1 none).
//!
//! **Each tick** the hit ([`hit`], `0x3f1c80`), then drawn within 24 of the camera its shadow (range 0x16). **States:**
//! **0** once the frame counter (`0x1b2748`) runs: its groups' live members deleted, 1; its health block's `+0x29` 1.
//! **1** each reload: one tick in seven the search (`0x323c88`: its cuboid as a list of one, else its path, within
//! the reach), else its target's place (a target deleted: none); with a target the count ([`count`], `0x3f1ea0`):
//! fewer out than its most and a dead member left, 2 (sequence 1 over 10 ticks). **2**: its sequence wrapped, 1
//! (sequence 0 over 10). At frame 12 (`0x325d28` = L00 `0x323bf8`, crate::moby::ai::reached) the throw: the landing
//! point — the landing cuboid nearest Ratchet (`+0x132`; none: half the way to the target, at most 3 with a reach over
//! 3, at least 3) or one from a random start, else a random point within its reach (at least 3, `0x31c6c8` = L02
//! `0x31fce0`) — a cuboid's place moved by its first two rows × −1..1 each, on the ground there (0.5 up, `0x31e048`);
//! the way there at `min(d / 90, 0.08)` a tick (`$gp+0x2054`), from its joint 0 (`0x3132c8` = L00 `0x3127c8`) 0.05 along
//! it with ±0.025 on each axis (`0x323438` = L00 `0x3217d8`), the rise that reaches the point under −0.002722 a tick²
//! (`0x31f330` = L00 `0x31e048`); the next dead member thrown ([`throw`], `0x3f1950`); its count up, the reload again.
//! **3** hurt: the knock's fall (`0x320fc8`); its sequence wrapped, 1 (sequence 0 over 20). **4** dying: the fall
//! until its sequence wraps, then the explosion (`0x322198` = L00 `0x320538`: flashes 8 and 4 past 18, size 1, light
//! 15, 30 embers, 8 billows, 20 flares, its class sound 1, the shake, one debris, flags 0x810001) and gone.
//!
//! The engine functions with the port's: `0x31ef28` / `0x31ef88` = L00 `0x31dce8` / `0x31dd48` (the hit, crate::moby::
//! ai::damage_of), `0x321d88` / `0x321e48` = L00 `0x320128` / `0x3201e8` (the flash), `0x31c0a0` = L00 `0x31b5f0`
//! (set_anim), `0x31b870` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x31dc00` / `0x31dab0` = L00 `0x31c9d8` /
//! `0x31c8e0` (a group's first dead or live member, its live count), `0x312188` = L00 `0x311688` (delete),
//! `0x2ef908` = L00 `0x2eeae0` (crate::moby::countdown), `0x31c450` / `0x31c490` / `0x31c4e8` (crate::rng),
//! `0x2efe58` = L00 `0x2ef018` (the distance), `0x31ec98` = L00 `0x31da58` (crate::shadow_volumes::probe_near).
//!
//! Not ported: the explosion's offset (`0x1ac5a0` / 6: crate::moby::blast::explode has no such argument), the thrown
//! moby's words `+0x36` (0x7f80), `+0x61` / `+0x62` / `+0x6c` / `+0x6d` (0xff) and `+0x6e` (0) the port has no
//! fields for, and the message `0x2a5e70` prints when its pool is empty.

use crate::hero::physics::{add, atan2, dist, len, scale, set_len, sub, V3};
use crate::moby::classes::props::{group_alive, set_seq, wrapped};
use crate::moby::{ai, knock, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c1a;
const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
const KNOCK: usize = 0x60;
const T_BLOCK: usize = 0xc0;
const T_MOBY: usize = 0x100;
const T_RESULT: usize = 0x104;
const REACH: usize = 0x120;
const CUBOID: usize = 0x124;
const PATH: usize = 0x128;
const RELOAD: usize = 0x12c;
const TIMER: usize = 0x12e;
const MOST: usize = 0x130;
const OUT: usize = 0x131;
const NEAREST: usize = 0x132;
const SMALL: usize = 0x133;
const POOLS: [usize; 2] = [0x134, 0x138];
const LANDINGS: usize = 0x140;
const PVARS: usize = 0x150;
/// `$gp+0x2054`: the distance a tick of the throw's speed covers.
const SPAN: f32 = 90.0;

fn members(w: &MobyWorld, g: i32) -> Vec<usize> { usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() }
fn dead(w: &MobyWorld, k: usize) -> bool { w.mobys.get(k).is_none_or(|m| 0x80 <= m.state) }

/// `0x3f1c80(m, p)`: unless dying, a hit (flags 0xc30000) that does damage takes it off the health: left alive, a
/// knock of 0.1333 across and 0.1167 up the hit's way (crate::moby::knock, sequence 3 over 1 tick), its flash's colour
/// 0x78, and a full one (damage 1 or more) hurts it (3, sequence 2 at once); none left, not a target any more, its
/// flash's colour 0xfa, 4, a knock of 0.1833 across and 0.1167 up, its keys 5 and 10, sequence 3, its bolts
/// (`0x31b870(m, 0, 3)`). Either way the flash starts. The record let go; the flash's tick.
fn hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 4 { return; }
    let h = w.hit_message(id).filter(|h| h.tmpl.flags & 0xc3_0000 != 0);
    let f = ai::damage_of(w, id, h.as_ref(), false);
    if 0.0 < f {
        let h = h.expect("damage without a hit");
        let hp = w.mobys[id].pv_f(HEALTH) - h.tmpl.damage;
        w.mobys[id].set_pv_f(HEALTH, hp);
        let (across, up) = if 0.0 < hp { (0x3e08_8889, 0x3dee_eef0) } else { (0x3e3b_bbbc, 0x3dee_eef0) };
        let m = &mut w.mobys[id];
        let (mut a, mut u) = (f32::from_bits(across), f32::from_bits(up));
        m.set_pv_f(KNOCK + 0x18, a);
        m.set_pv_f(KNOCK + 0x1c, u);
        let angle = knock::aim(h.tmpl.dir, &mut a, &mut u);
        m.set_pv_f(KNOCK + 0x18, a);
        m.set_pv_f(KNOCK + 0x1c, u);
        if 0.0 < hp {
            knock::start(w, id, ctx, KNOCK, 3, 1, 0, angle);
            w.mobys[id].set_pv_b(FLASH + 7, 0x78);
            if 1.0 <= f {
                w.mobys[id].state = 3;
                ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
            }
        } else {
            let m = &mut w.mobys[id];
            m.mode &= !mode::TARGET;
            m.set_pv_b(FLASH + 7, 0xfa);
            m.state = 4;
            knock::start(w, id, ctx, KNOCK, 3, 1, 0, angle);
            let m = &mut w.mobys[id];
            m.set_pv_f(KNOCK + 0x50, 5.0);
            m.set_pv_f(KNOCK + 0x54, 10.0);
            set_seq(w, id, ctx, 3, 0, 0);
            crate::moby::bolts::drop(w, id, 0, 3, ctx);
        }
        ai::flash_start(&mut w.mobys[id], FLASH);
    }
    w.clear_hit(id);
    ai::flash(&mut w.mobys[id], FLASH);
}

/// `0x3f1ea0(m, p)`: its count out (its groups' live members); the first dead member of its groups, if any.
fn count(w: &mut MobyWorld, id: usize) -> Option<usize> {
    let m = &w.mobys[id];
    let gs = POOLS.map(|o| m.pv_i(o));
    let n = gs.iter().map(|&g| group_alive(w, g)).sum::<usize>();
    w.mobys[id].set_pv_b(OUT, n as u8);
    gs.iter().find_map(|&g| members(w, g).into_iter().find(|&k| dead(w, k)))
}

/// `0x3f1950(m, at, dir)`: its groups' first dead member brought back at `at`, heading along `dir`: its count up;
/// state 0, its class's mode (glow `| 0x10` and shadow `| 0x400` when its class has them), scale, collision and
/// update; update and draw distances 0xff, drawn, no hit, its sequence from the start, its ticks in the state and
/// drops 0; its angles 0, 0 and `dir`'s heading; `dir` into its reaction block's velocity (`+0x80`, `0x320d38` = L00
/// `0x31fa50`); with `+0x133` a tenth of its class's size; its bolts out of the mortar's (no more than it has, the
/// mortar's down by them); its owner the mortar; Ratchet's light (`0x321ae0` = L00 `0x31fe80`); rebuilt
/// (`0x314870` = L00 `0x313dc0`); its health full.
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3, dir: V3) -> Option<usize> {
    let gs = POOLS.map(|o| w.mobys[id].pv_i(o));
    let k = gs.iter().find_map(|&g| members(w, g).into_iter().find(|&k| dead(w, k) && k < w.mobys.len()))?;
    let m = &mut w.mobys[id];
    m.set_pv_b(OUT, m.pv_b(OUT).wrapping_add(1));
    let (small, owner_bolts) = (m.pv_b(SMALL) != 0, m.bolts);
    let class = w.class(w.mobys[k].o_class).cloned();
    let (light, ambient) = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or((0, [0x80; 3]), |h| (h.light, h.ambient));
    let o = &mut w.mobys[k];
    o.pos = at;
    o.state = 0;
    o.timer = 0;
    if let Some(c) = class.as_ref() {
        o.mode = c.mode;
        o.scale = c.scale;
        if c.glow != 0 { o.mode |= mode::GLOW; }
        if c.shadow.is_some() { o.mode |= 0x400; }
        o.has_collision = c.has_collision;
        if c.update.is_some() { o.update = c.update; }
    }
    o.update_dist = 0xff;
    o.draw_dist = 0xff;
    o.drawn = 1;
    o.hit_slot = -1;
    o.state_ticks = 0;
    o.drops = 0;
    o.rot = [0.0, 0.0, atan2(dir[0], dir[1])];
    if let Some(r) = ai::react_of(o) { o.set_pv_v(r + 0x80, dir); }
    if small { o.scale *= 0.1; }
    if owner_bolts < o.bolts { o.bolts = owner_bolts; }
    let given = o.bolts;
    o.parent = Some(id);
    o.light = light;
    o.ambient = ambient;
    if let Some(h) = ai::health_of(o) {
        let most = o.pv_s(h + 4) as f32;
        o.set_pv_f(h, most);
        o.set_pv_i(h + 0x20, 0);
        o.set_pv_i(h + 0x24, 0);
    }
    let m = &mut w.mobys[id];
    m.bolts = owner_bolts.wrapping_sub(given).max(0);
    ctx.host.set_anim(&w.mobys[k], 0, 0, 0);
    w.rebuild(k, &*ctx.host);
    Some(k)
}

/// `0x31c6c8(r)` (L02 `0x31fce0`, as L04 `0x33cf18`): a random point across within `r` of the origin.
fn random_within(ctx: &mut Ctx, r: f32) -> V3 {
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let u = (((rng.rand() >> 16) & 0xfff) as f32) * 0.000_244_140_62;
    let l = (1.0 - u * u) * r;
    [a.cos() * l, a.sin() * l, 0.0]
}

/// `0x31f330(from, to, &ticks, speed, g)` (L00 `0x31e048`): the rise a tick that takes a throw at `speed` across
/// from `from` to `to` under the gravity `g` (0 when it has no way across).
fn rise(from: V3, to: V3, speed: f32, g: f32) -> f32 {
    let d = sub(to, from);
    let t = crate::hero::physics::len2(d) / speed;
    if t == 0.0 { return 0.0; }
    -((from[2] - to[2]) + g * t * t * 0.5) / t
}

/// 0x0c1a (module docs).
pub fn update_0c1a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    hit(w, id, ctx);
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    match w.mobys[id].state {
        0 => {
            if ctx.game.frame != 0 {
                for o in POOLS {
                    for k in members(w, w.mobys[id].pv_i(o)) {
                        if k != id && !dead(w, k) { w.delete(k); }
                    }
                }
                w.mobys[id].state = 1;
            }
            w.mobys[id].set_pv_b(HEALTH + 9, 1);
        }
        1 => aim(w, id, ctx),
        2 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].state = 1;
                set_seq(w, id, ctx, 0, 0, 10);
                return;
            }
            if ai::reached(w, id, ctx, 12.0) { fire(w, id, ctx); }
        }
        3 => {
            knock::fall(w, id, ctx, KNOCK);
            if wrapped(w, id, ctx) {
                w.mobys[id].state = 1;
                set_seq(w, id, ctx, 0, 0, 20);
            }
        }
        4 => {
            if knock::fall(w, id, ctx, KNOCK) & 0x40 != 0 {
                use crate::moby::blast::{explode, Explosion};
                let e = Explosion { flash: 8.0, flash2: 4.0, near: 18.0, size: 1.0, light: 15.0, embers: 30, billows: 8, flares: 20, sound: 1, shake: true, debris: 1, lod: -1, colour: 0, flags: 0x81_0001, ..Default::default() };
                explode(w, ctx, Some(id), None, &e);
                w.delete(id);
            }
        }
        _ => {}
    }
}

/// State 1 (module docs).
fn aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut t = w.mobys[id].pv_s(TIMER) as u16 as i32;
    let ready = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_s(TIMER, t as u16 as i16);
    if !ready { return; }
    if ctx.game.rng.randi(7) == 0 {
        let m = &w.mobys[id];
        let (reach, cub, path) = (m.pv_f(REACH), m.pv_i(CUBOID), m.pv_i(PATH));
        let one = [cub];
        let areas: &[i32] = if cub != -1 { &one } else { &[] };
        crate::moby::target_search::search(w, id, ctx, T_BLOCK, areas, path, reach);
    } else {
        match ai::link(&w.mobys[id], T_MOBY).filter(|&t| w.mobys.get(t).is_some_and(|o| o.state != 0xfe && o.state != 0xfd)) {
            Some(t) => {
                let p = w.mobys[t].pos;
                w.mobys[id].set_pv_v(T_BLOCK, p);
            }
            None => {
                let m = &mut w.mobys[id];
                ai::set_link(m, T_MOBY, None);
                m.set_pv_i(T_RESULT, 2);
            }
        }
    }
    if w.mobys[id].pv_i(T_RESULT) == 2 { return; }
    let free = count(w, id);
    let m = &w.mobys[id];
    if m.pv_b(OUT) < m.pv_b(MOST) && free.is_some() {
        w.mobys[id].state = 2;
        set_seq(w, id, ctx, 1, 0, 10);
    }
}

/// State 2's throw at frame 12 (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let pos = m.pos;
    let landings: Vec<i32> = (0..4).map(|k| m.pv_i(LANDINGS + 4 * k)).collect();
    let mut chosen: Option<usize> = None;
    let mut e;
    if m.pv_b(NEAREST) != 0 {
        let mut best = 100_000.0;
        for c in landings.iter().filter_map(|&c| usize::try_from(c).ok()) {
            let Some(v) = ctx.level.cuboids.get(c) else { continue };
            let d = dist(ctx.hero.pos, [v.matrix[3][0], v.matrix[3][1], v.matrix[3][2]]);
            if d < best {
                chosen = Some(c);
                best = d;
            }
        }
        e = scale(sub(m.pv_v(T_BLOCK), pos), 0.5);
        if 3.0 < m.pv_f(REACH) && 3.0 < len(e) { e = set_len(e, 3.0); }
        if len(e) < 3.0 { e = set_len(e, 3.0); }
        e = add(e, pos);
    } else {
        let k = ctx.game.rng.rand_range(0, 3);
        chosen = (0..4).map(|i| landings[((i + k) % 4) as usize]).find(|&c| c != -1).and_then(|c| usize::try_from(c).ok());
        e = [0.0; 3];
        if chosen.is_none() {
            let r = w.mobys[id].pv_f(REACH).max(3.0);
            e = add(random_within(ctx, r), pos);
        }
    }
    if let Some(c) = chosen.and_then(|c| ctx.level.cuboids.get(c)) {
        let (r0, r1) = ([c.matrix[0][0], c.matrix[0][1], c.matrix[0][2]], [c.matrix[1][0], c.matrix[1][1], c.matrix[1][2]]);
        e = [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]];
        let a = ctx.game.rng.randf(-1.0, 1.0);
        e = add(e, scale(r0, a));
        let b = ctx.game.rng.randf(-1.0, 1.0);
        e = add(e, scale(r1, b));
    }
    let g = ai::ground_z(ctx, 0.5, e, 0);
    if g != 0.0 { e[2] = g; }
    let d = sub(e, pos);
    let f = (len(d) / SPAN).min(0.08);
    let mut dir = set_len(d, f);
    let from = ctx.host.joint(&w.mobys[id], 0).unwrap_or(pos);
    let mut step = set_len(dir, 0.05);
    for s in step.iter_mut() { *s += ctx.game.rng.randf(-0.025, 0.025); }
    let from = add(from, step);
    dir[2] = rise(from, e, f, f32::from_bits(0xbb32_6750));
    throw(w, id, ctx, from, dir);
    let m = &mut w.mobys[id];
    m.set_pv_b(OUT, m.pv_b(OUT).wrapping_add(1));
    m.set_pv_s(TIMER, m.pv_s(RELOAD));
}
