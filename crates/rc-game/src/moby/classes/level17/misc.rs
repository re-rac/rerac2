//! Smolg's smaller classes (L17 addresses; the engine functions by their L17 places with the L00 ones the port knows
//! them by).
//!
//! * **0x0877 / 0x08ae / 0x08b0** ([`update_0877`], `0x3be088`, 4 each; the decomp's `func_L17_003BCBC8`): markers.
//!   Their glow flashes as the buoys' (crate::moby::classes::level17::floaters::flash_glow); their platform words set
//!   (`+0x8c` | 1, `+0x3e` | 8); their update distance 0.
//! * **0x120d** ([`update_120d`], `0x420138`, 2 placed; the decomp's `func_US2_L17_00420138`): an updraft pad, the
//!   sibling of L11's 0x08e2 (crate::moby::classes::level11::props::update_08e2). Its pvars: `+0x00` a fly-by camera
//!   (−1 none), `+0x04` the glow's phase, `+0x08` a cuboid (−1 none), `+0x0c` a moby it hooks (−1 none), `+0x10` the
//!   lift it gives (0: Ratchet's own). Its pulse: 6° a tick, its shade `sin·128` held to 32..128.
//!   * **0**: on its first tick the glow on (mode | 0x10); 1.
//!   * **1**: Ratchet's ground moby it (`0x18a15c`): with a camera, its uid not gone (crate::game::GameState::uid_dead,
//!     its save count 0), the camera armed (`0x437338` = L00 `0x42a4e0`, crate::cameras), its uid counted, its moby
//!     hooked (`0x32d808` = L00 `0x32a818`), the camera none. The prompt 0x1349 (`0x329e08` = L00 `0x327ae0`,
//!     crate::prompt::Prompt::force, owner 0xc); the glow its shade in green; the updraft's 2 ticks (`0x18a012`) and
//!     its lift (`0x18a3d8`) when it has one. Else the glow its shade in red.
//!   * Every tick, Ratchet floating (0x82) in its cuboid: the fall (`0x2c5778(6, 1)` = L00 `0x2c4560`).
//! * **0x1256** ([`update_1256`], `0x420b08`, 2 placed; the decomp's `func_L17_0041F478`): the barrier 0x1305's
//!   twin (crate::moby::classes::level17::gate; its wall `0x420810` the same code), behind a hook. With `+0x54` set
//!   its target is in its state 2 and, until it is hooked (`0x32d828` = L00 `0x32a838`), it is hidden without
//!   collision (mode | 0x41, drawn 0) and does nothing more; hooked, `+0x54` −1, the hook cleared, the target in its
//!   state 1. Its links count as 0x1305's but none while it is hooked, a link of its class gone from the save (its uid's
//!   count) counts as gone, and a gone link (or one in its gone state) puts the count back to 0 (the links after it
//!   still count).
//! * **0x1326** ([`update_1326`], `0x422498`, 2 placed; the decomp's `func_L17_00420E08`): a floor switch. Its pvars:
//!   `+0x00` a path, `+0x04` a marker, `+0x08` it follows its mission, `+0x0c` its pulse's phase.
//!   * **0**: its phase a random angle (`0x31c5b0` = L00 `0x31bb00`), 0.35 lower. Its mission done: 2, its timer byte
//!     2, the glow `0x80208020`; else 1.
//!   * **1**: its pulse (6° a tick; its shade `(sin·4 − 3)·128` held to 32..128, in grey). Ratchet standing on it
//!     (`0x18a15c`, `0x18a16e` 0), or with `+0x08` its mission done: 2, its timer byte 1, the glow `0x80208020`, its
//!     sound 0 (`0x363c80` = L00 `0x3605f0`).
//! * **0x1272** ([`update_1272`], `0x420f80`, one placed; the decomp's `func_L17_0041F8F0`): a music zone. Updated
//!   from any distance. Its pvars: `+0x00` a cuboid, `+0x04` the zone's cuboid, `+0x10` / `+0x18` the tracks inside /
//!   outside, `+0x14` their stinger. **0**: 1 with Ratchet in the zone (`0x3234b8` = L00 `0x321858`), else 2. **1**:
//!   on its first tick the inside's music (`0x331c08`, crate::moby::MobyHost::music_request); out of the zone 2. **2**:
//!   on its first tick the outside's music; in the zone 1.
//! * **0xf27** ([`update_0f27`], `0x387238`, one placed; the decomp's `func_L17_00385DE8`): a scene's effects.
//!   Updated from any distance. In a scene (`0x1a8f00` 2): the scene 6 hides every 0x0b88 and 0x1249 (no collision,
//!   mode | 0x43 without 0x1000); in the scene 4 at its ticks 301..359, and the scene 5 from its tick 531, its third
//!   actor (`0x1bd8c0 + 0x194`) of class 0x125e gives off two sparks a tick at its joint 0 (`0x3132c8` = L00
//!   `0x3127c8`), moving 0.05 down its up row (`0x338e70` = L00 `0x335bb8`, crate::particles::puff): one of
//!   100000 (0.1 jitter, 1..0.9 growth, ±0..15 spin, `0x7f204080`) and one of 60000 (0.05, 1..0.97, 16,
//!   `0x7fffffff`, a random roll), each 16 ticks with its tail (`+0x24` 2, `+0x2a` 0x7f, `+0x2b` 16) — the second only
//!   when the first was made.
//!
//! Not ported: 0x1326's markers (its path's points of w equal to `+0x04` set to 0 in its moves to 2; the port's paths
//! are the level's, read-only, and nothing here reads them); 0x1272's level words (`0x1b27cc` 2 and `$gp+0x3878`
//! 55 while Ratchet is in `+0x00`, `0x1b27cc` 0 out of it; on its state 1's first tick `0x13963d` 1 and `0x1b2868`
//! 75), which nothing in the port reads; 0x1256's debug prints (`0x2a5e70`).

use crate::hero::physics::{add_rot, set_len};
use crate::moby::{Ctx, MobyWorld};

pub const MARKER_A: i16 = 0x0877;
pub const MARKER_B: i16 = 0x08ae;
pub const MARKER_C: i16 = 0x08b0;
pub const UPDRAFT: i16 = 0x120d;
pub const BARRIER: i16 = 0x1256;
pub const SWITCH: i16 = 0x1326;
pub const MUSIC: i16 = 0x1272;
pub const SCENE_FX: i16 = 0x0f27;

/// 6° in radians.
const SIX_DEG: f32 = f32::from_bits(0x3dd6_7752);
const GREEN: u32 = 0x8020_8020;

/// 0x0877 / 0x08ae / 0x08b0 (module docs).
pub fn update_0877(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    super::floaters::flash_glow(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_i(0x8c, m.pv_i(0x8c) | 1);
    m.set_pv_s(0x3e, m.pv_s(0x3e) | 8);
    m.update_dist = 0;
}

/// The uid gone or counted in the save (`0x21f044 + uid`, `0x19b4a8`).
fn uid_done(ctx: &Ctx, uid: i16) -> bool {
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// 0x120d (module docs).
pub fn update_120d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let ph = add_rot(m.pv_f(4), SIX_DEG);
    m.set_pv_f(4, ph);
    let shade = ((ph.sin() * 128.0) as i32).clamp(32, 128) as u32;
    match m.state {
        0 => {
            if m.first() { m.mode |= 0x10; }
            m.set_state(1, -1);
        }
        1 => {
            m.first();
            if ctx.hero.ground_moby == Some(id) {
                let (cam, uid) = (m.pv_i(0), m.uid);
                if cam != -1 && !uid_done(ctx, uid) {
                    ctx.game.cameras.arm(cam);
                    ctx.game.count_uid(uid);
                    let link = w.mobys[id].pv_i(0xc);
                    if let Some(l) = usize::try_from(link).ok().and_then(|l| w.mobys.get_mut(l)) { l.once |= 4; }
                    w.mobys[id].set_pv_i(0, -1);
                }
                ctx.game.prompt.force(0xc, 0x1349, 1);
                let m = &mut w.mobys[id];
                m.glow = 0x8000_0000 | shade << 8;
                ctx.hero.gear.updraft = 2;
                let lift = m.pv_f(0x10);
                if lift != 0.0 { ctx.hero.gear.lift_store = lift; }
            } else {
                m.glow = 0x8000_0000 | shade;
            }
        }
        _ => {}
    }
    let c = w.mobys[id].pv_i(8);
    if 0 <= c && ctx.level.in_cuboid(c, ctx.hero.pos) && ctx.hero.state == 0x82 { ctx.hero.request_state(6, true); }
}

/// 0x1256's links counted (module docs).
fn links(w: &MobyWorld, id: usize, ctx: &Ctx) -> usize {
    let m = &w.mobys[id];
    let mut n = 0;
    for j in 0..5 {
        let l = 0x18 + j * 8;
        if m.pv_i(l) == -1 || m.once & 4 != 0 { continue; }
        let Some(o) = usize::try_from(m.pv_i(l)).ok().and_then(|k| w.mobys.get(k)) else { n = 0; continue };
        if o.o_class != m.pv_s(l + 4) || o.state == 0xfe || o.state == 0xfd || ctx.game.uid_count(o.uid) != 0 {
            n = 0;
            continue;
        }
        n += 1;
        let s = m.pv_b(l + 6) as i8;
        if s != -1 && o.state == s as u8 { n = 0; }
    }
    n
}

/// 0x1256 (module docs).
pub fn update_1256(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let target = usize::try_from(w.mobys[id].pv_i(0x10)).ok().filter(|&k| k < w.mobys.len());
    if w.mobys[id].pv_i(0x54) != -1 {
        if let Some(t) = target { w.mobys[t].state = 2; }
        let m = &mut w.mobys[id];
        if m.once & 4 == 0 {
            m.has_collision = false;
            m.drawn = 0;
            m.mode |= 0x41;
            return;
        }
        m.set_pv_i(0x54, -1);
        m.once &= !4;
        if let Some(t) = target { w.mobys[t].state = 1; }
    }
    let m = &w.mobys[id];
    let done = m.mission != 0xff
        && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
    let count = if done || m.state == 0 { 0 } else { links(w, id, ctx) };
    super::gate::run(w, id, ctx, count);
}

/// 0x1326 (module docs).
pub fn update_1326(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let lv = ctx.game.level.max(0) as usize * 0x10;
    let m = &w.mobys[id];
    let done = ctx.game.missions.get(lv + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
    match m.state {
        0 => {
            let a = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0xc, a);
            m.pos[2] -= f32::from_bits(0x3eb3_3333);
            if done {
                m.state = 2;
                m.timer = 2;
                m.glow = GREEN;
            } else {
                m.state = 1;
            }
        }
        1 => {
            let m = &mut w.mobys[id];
            let ph = add_rot(m.pv_f(0xc), SIX_DEG);
            m.set_pv_f(0xc, ph);
            let s = (((ph.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(32, 128) as u32;
            m.glow = 0x8000_0000 | s << 16 | s << 8 | s;
            let on = ctx.hero.ground_moby == Some(id) && ctx.hero.air_ticks == 0;
            if on || (m.pv_i(8) != 0 && done) {
                m.state = 2;
                m.timer = 1;
                m.glow = GREEN;
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
        }
        _ => {}
    }
}

/// 0x1272 (module docs).
pub fn update_1272(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].update_dist = 0xff;
    let zone = w.mobys[id].pv_i(4);
    let inside = |ctx: &Ctx| -> bool { ctx.level.in_cuboid(zone, ctx.hero.pos) };
    match w.mobys[id].state {
        0 => {
            let s = if inside(&*ctx) { 1 } else { 2 };
            w.mobys[id].set_state(s, -1);
        }
        1 => {
            if w.mobys[id].first() {
                let m = &w.mobys[id];
                ctx.host.music_request(m.pv_i(0x10), m.pv_i(0x14));
            }
            if !inside(&*ctx) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            if w.mobys[id].first() {
                let m = &w.mobys[id];
                ctx.host.music_request(m.pv_i(0x18), m.pv_i(0x14));
            }
            if inside(&*ctx) { w.mobys[id].set_state(1, -1); }
        }
        _ => {}
    }
}

/// 0xf27's sparks at the actor `a` (module docs).
fn sparks(w: &MobyWorld, a: usize, ctx: &mut Ctx) {
    let Some(at) = ctx.host.joint(&w.mobys[a], 0) else { return };
    let vel = set_len(w.mobys[a].rows[2], -0.05);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let n = g.randi(16);
    let spin = if g.randi(2) != 0 { -n } else { n };
    let first = crate::particles::puff(sys, g, 0.1, 1.0, f32::from_bits(0x3f66_6666), 100_000.0, at, spin, vel, 0x7f20_4080, 0);
    if let Some(i) = first { spark_tail(&mut sys.pool.recs[i][..], 16); }
    let second = crate::particles::puff(sys, g, 0.05, 1.0, f32::from_bits(0x3f78_51ec), 60_000.0, at, 16, vel, 0x7fff_ffff, 0);
    if let (Some(_), Some(i)) = (first, second) {
        let roll = g.randi(0xff) as u8;
        let r = &mut sys.pool.recs[i][..];
        spark_tail(r, 16);
        r[8] = roll;
    }
}

/// A spark's 16 ticks and its tail (`+0x24` 2, `+0x2a` 0x7f, `+0x2b` the first's life).
fn spark_tail(r: &mut [u8], life: u8) {
    r[0xa..0xc].copy_from_slice(&16i16.to_le_bytes());
    r[9] = 4 + 0x40;
    r[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
    r[0x2a] = 0x7f;
    r[0x2b] = life;
}

/// 0xf27 (module docs).
pub fn update_0f27(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.state = 1;
        }
        1 => {
            if ctx.game.scenes.mode != crate::scene::SCENE { return; }
            let Some((scene, tick, actor)) = ctx.game.scenes.player.as_ref().map(|p| (p.id, p.tick, p.actors.get(2).map(|a| a.0))) else { return };
            if scene == 6 {
                for m in w.mobys.iter_mut().filter(|m| m.state < 0x80 && (m.o_class == 0x0b88 || m.o_class == 0x1249)) {
                    m.has_collision = false;
                    m.mode = (m.mode | 0x43) & 0xefff;
                }
            }
            let Some(a) = actor.filter(|&a| w.mobys.get(a).is_some_and(|m| m.o_class == 0x125e)) else { return };
            if (scene == 4 && (301..=359).contains(&tick)) || (scene == 5 && 531 <= tick) { sparks(w, a, ctx); }
        }
        _ => {}
    }
}
