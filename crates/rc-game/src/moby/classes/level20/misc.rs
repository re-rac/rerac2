//! Yeedil's smaller classes (L20 addresses; the engine functions by their L20 places with the L00 ones the port knows
//! them by).
//!
//! * **0x11ef** ([`update_11ef`], `0x446c80`, 2 placed; the decomp's `func_US2_L20_00446C80`): a slab that slides
//!   along x once its mission is done. Its pvars: `+0x00` how far, `+0x08` / `+0x0c` its ease's rate and most speed,
//!   `+0x10` its x at the start, `+0x14` its speed. **0**: its x kept; 1. **1**: its mission (`+0xb0`) done, 2.
//!   **2**: on its first tick still and its class sound 0 (`0x3766c0` = L00 `0x3605f0`); it eases to `+0x00` short of
//!   its start (`0x333eb8` = L00 `0x31ed78`, crate::camera::approach). **3**: hooked (`0x33fb18` = L00 `0x32a838`), 4.
//!   **4**: the same back to its start.
//! * **0x1236** ([`update_1236`], `0x4483b8`, 2 placed; the decomp's `func_US2_L20_004483B8`): 0x11ef's code without
//!   its two sounds (the same pvars and states).
//! * **0x1246** ([`update_1246`], `0x4488f8`, 3 placed; the decomp's `func_L20_004473B0`): a lift. Its pvars: `+0x20`
//!   its platform block (crate::moby::platform, flags | 1), `+0x60` its place at the start, `+0x70` how high, `+0x74`
//!   the trip's ticks, `+0x78` / `+0x7c` / `+0x80` the waits at the top, before the trip down, at the bottom, `+0x84`
//!   the wait's countdown, `+0x88` its speed, `+0x8c` above 0: the breeders of its group put down. **0**: its start
//!   kept, hooked (`0x33faf8` = L00 `0x32a818`), 4. **1**: its trip up (the wait `+0x74`; eased to `+0x70` above its
//!   start at `4·h / t²` a tick², at most that times `t`), then 2. **2**: the wait `+0x78`, 3. **3**: the trip down
//!   (the wait `+0x7c`), 4. **4**: the wait `+0x80`, 1. Not hooked with `+0x8c` above 0, each live breeder 0x0b69 of
//!   its group (`+0x21`) gives its bolts and dies (`0x448b88`: `0x32fd10(e, 0, 1)` and L18 `0x3f8928`,
//!   crate::moby::classes::level18::breeder::drop_and_die), `+0x8c` −1000. Its move into its platform block (`0x3378b8`
//!   = L00 `0x322668`) and Ratchet carried.
//! * **0x131a** ([`update_131a`], `0x3a16e8`, 2 placed; the decomp's `func_L20_003A0320`): a breakable. **0**: 1.
//!   **1**: a hit with flags 0x4010000 (`0x332e28` = L00 `0x31dce8`) while `+0x00` is 0 breaks it; its hit let go;
//!   the moby `+0x04` (above 0) gone or in its state 3 breaks it too. **2**: drawn (and the frame's load under 0.9,
//!   which the port's always is), the general explosion 3 up its scaled frame (`0x335678` = L00 `0x320538`,
//!   crate::moby::blast::explode: 10 embers, 3 billows, 16 flares, flashes 1 / 0.5 past 9, 1.3 its scale, light 15,
//!   debris); its pieces (`0x33c568` = L00 `0x327498`, crate::moby::shards::break_into: 3..6 / 3..6), the level's bolts
//!   (`0x33c458` = L00 `0x327388`, crate::moby::bolts::level_drop), its class sound 0, 3. **3**: hidden without
//!   collision, deleted 120 ticks on (`0x3265e8` = L00 `0x311688`).
//! * **0x04f7** ([`update_04f7`], `0x3c12c0`, 2 placed; the decomp's `func_L20_003BFEB8`): a trigger. Its pvars:
//!   `+0x00..+0x20` eight groups (−1 none), `+0x20` the state it goes to (−1 none; 0xff: its uid gone, crate::game::
//!   GameState::uid_gone, and deleted), `+0x24` its timer byte (−1 none), `+0x28` a cuboid Ratchet must be in (−1
//!   any), `+0x2c` a group it hooks (`0x331910` = L02 `0x320b50`, crate::moby::classes::maktar::hook_group). Each tick,
//!   while one of the groups has a member alive (`0x331830` = L00 `0x31c8e0`, crate::moby::classes::props::
//!   group_alive; the game's walk stays on a −1 slot, so the groups after one are not tested), Ratchet is in his state
//!   0x31 or out of the cuboid, nothing; else the timer byte, then the state (or the uid and gone), then the hook.
//! * **0x0f88** ([`update_0f88`], `0x43ab48`, one placed; the decomp's `func_L20_004395B0`): a box the Tractor Beam
//!   drags (crate::moby::classes::tractor). **0**: a target (mode 0x1000), its hold distance 7 (`+0x84`), its platform
//!   flags | 1 (`+0xdc`), its range `+0x88` the record's `+0xe4`; 1. **1**: the drag (`0x33b0e0` = L01 `0x31bd40`,
//!   its block `+0x50`, radius 1 (`$gp+0x2dcc`; the heights `+0x2dc8` / `+0x2dd0` 1 / 6, which the port's drag does not
//!   take), its path `+0xe0`). Its move into its platform block `+0xa0` and Ratchet carried.
//! * **0x1209** ([`update_1209`], `0x446dd8`, one placed; the decomp's `func_US2_L20_00446DD8`): a pad Ratchet stands
//!   on to set things off. Its pvars: `+0x00..+0x10` four mobys (−1 none) and `+0x10..+0x20` how each is set off (≠ 0
//!   hooked, `0x33faf8` = L00 `0x32a818`; else its timer byte 1), `+0x20` its pulse's phase, `+0x24` set off, `+0x28`
//!   a mission done at it (`0x328248` = L00 `0x3132a8`), `+0x2c` the hero's form switched back at it. Its pulse: 6° a
//!   tick, its shade `(sin·4 − 3)·128` held to 32..128. **0**: the glow on (mode 0x10); 1. **1**: Ratchet standing on
//!   it (`0x18a15c`, `0x18a160`) within half its scale over its class's of its joint 0 across (`0x3031a0` = L00
//!   `0x2ef050`) sets it off: 2, its mobys, the mission. Its glow the shade in red. **2**: its glow the shade in green
//!   and blue.
//! * **0x11e8** ([`update_11e8`], `0x4461f0`, one placed; the decomp's `func_L20_00444CA8`): a drawbridge that falls
//!   when its hook (the moby `+0x00`, kept 5.7 up its frame and 0.35 along its second row) is pulled. Its pvars: `+0x04`
//!   its fall's speed, `+0x08` a moby it crushes (the port keeps its index at `+0x10`; the game a pointer). Its pitch
//!   (`+0xf0`) eases each state (`0x333eb8` = L00 `0x31ed78`, crate::camera::approach):
//!   * **0**: the crushed moby kept; 1.
//!   * **1**: its hook hooked (`0x33fb18` = L00 `0x32a838`): its class sound 0, 2. Its mission done: a quarter turn
//!     down at once (`0x304018` = L00 `0x2efea8`) and 6.
//!   * **2**: down to −90° (0.000533 a tick², at most 290 a tick); past −45° the crushed moby, a live 0x0cbc, blows up
//!     (`0x4130f8` = L09 `0x3ec3d0`: its class sound 2, its bolts, the blast of size 1 with the flash and the shake,
//!     deleted); at −90°, its class sound 1 and 3.
//!   * **3** / **5**: a bounce up to −75° / −85°, then 4 / 6. **4** / **6**: down again to −90° (on 6's first tick the
//!     troopers of its group woken, [`wake`], `0x446620`), its class sound 1, then 5 / 7.
//!
//! Not ported: 0x1209's switch back of the hero's form (`0x2c5b50(0)`, with `+0x2c`; level 20's own, which the port
//! lacks).

use crate::hero::physics::{add_rot, dist2, sub};
use crate::moby::blast::{self, Explosion};
use crate::moby::{bolts, shards, Ctx, MobyWorld};

/// The level's mission `mission` done (`0x19b2e8 + level·0x10`, bit 7).
pub(super) fn mission_done(ctx: &Ctx, mission: u8) -> bool {
    let g = &ctx.game;
    mission != 0xff && g.missions.get(g.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0)
}

pub(super) fn idx(w: &MobyWorld, k: i32) -> Option<usize> { usize::try_from(k).ok().filter(|&k| k < w.mobys.len()) }

/// Its move into its platform block at `b` and Ratchet carried.
pub(super) fn ride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, b: usize, old: [f32; 3], old_rot: [f32; 3]) {
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, b, sub(pos, old), old_rot, rot);
    let flags = m.pv_i(b + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
}

pub const SLAB: i16 = 0x11ef;
pub const SLAB_QUIET: i16 = 0x1236;

/// 0x11ef (module docs).
pub fn update_11ef(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { slide(w, id, ctx, true) }

/// 0x1236 (module docs): 0x11ef without its sounds.
pub fn update_1236(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { slide(w, id, ctx, false) }

fn slide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, sound: bool) {
    let state = w.mobys[id].state;
    match state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x10, m.pos[0]);
            m.set_state(1, -1);
        }
        1 => {
            if mission_done(ctx, w.mobys[id].mission) { w.mobys[id].set_state(2, -1); }
        }
        2 | 4 => {
            if w.mobys[id].first() {
                if state == 2 { w.mobys[id].set_pv_f(0x14, 0.0); }
                if sound { ctx.host.play_sound(&w.mobys[id], 0, 0); }
                if state == 4 { w.mobys[id].set_pv_f(0x14, 0.0); }
            }
            let m = &mut w.mobys[id];
            let to = if state == 2 { m.pv_f(0x10) - m.pv_f(0) } else { m.pv_f(0x10) };
            let (a, most) = (m.pv_f(8), m.pv_f(0xc));
            let (mut x, mut v) = (m.pos[0], m.pv_f(0x14));
            crate::camera::approach(to, a, a, most, &mut x, &mut v);
            m.pos[0] = x;
            m.set_pv_f(0x14, v);
        }
        3 => {
            if w.mobys[id].once & 4 != 0 { w.mobys[id].set_state(4, -1); }
        }
        _ => {}
    }
}

pub const LIFT: i16 = 0x1246;

/// 0x1246 (module docs).
pub fn update_1246(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let m = &mut w.mobys[id];
    // The wait `wait` set on the state's first tick (and the speed cleared with a trip), then its countdown.
    let wait = |m: &mut crate::moby::Moby, wait: usize, trip: bool| {
        if m.first() {
            if trip { m.set_pv_i(0x88, 0); }
            m.set_pv_i(0x84, m.pv_i(wait));
        }
        let mut k = m.pv_i(0x84);
        let out = crate::moby::countdown(&mut k);
        m.set_pv_i(0x84, k);
        out
    };
    let trip = |m: &mut crate::moby::Moby, to: f32| {
        let t = m.pv_i(0x74);
        let a = m.pv_f(0x70) * 4.0 / t.wrapping_mul(t) as f32;
        let (mut z, mut v) = (m.pos[2], m.pv_f(0x88));
        crate::camera::approach(to, a, a, a * t as f32, &mut z, &mut v);
        m.pos[2] = z;
        m.set_pv_f(0x88, v);
    };
    match m.state {
        0 => {
            let p = m.pos;
            m.set_pv_v(0x60, p);
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            m.once |= 4;
            m.set_state(4, -1);
        }
        1 => {
            if wait(&mut *m, 0x74, true) { m.set_state(2, -1); }
            let to = m.pv_f(0x68) + m.pv_f(0x70);
            trip(&mut *m, to);
        }
        2 => {
            if wait(&mut *m, 0x78, false) { m.set_state(3, -1); }
        }
        3 => {
            if wait(&mut *m, 0x7c, true) { m.set_state(4, -1); }
            let to = m.pv_f(0x68);
            trip(&mut *m, to);
        }
        4 => {
            if wait(&mut *m, 0x80, false) { m.set_state(1, -1); }
        }
        _ => {}
    }
    if w.mobys[id].once & 4 == 0 && 0.0 < w.mobys[id].pv_f(0x8c) {
        let group = w.mobys[id].group as u8;
        if group != 0xff {
            let members = w.groups.get(group as usize).cloned().unwrap_or_default();
            for e in members {
                let live = w.mobys.get(e).is_some_and(|k| k.o_class == crate::moby::classes::level18::breeder::CLASS && k.state != 0xfe && k.state != 0xfd);
                if live { crate::moby::classes::level18::breeder::drop_and_die(w, e, ctx); }
            }
        }
        w.mobys[id].set_pv_f(0x8c, -1000.0);
    }
    ride(w, id, ctx, 0x20, old, old_rot);
}

pub const CRATE: i16 = 0x131a;

/// 0x131a (module docs).
pub fn update_131a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
                return;
            }
            w.clear_hit(id);
            let k = w.mobys[id].pv_i(4);
            if 0 < k {
                if let Some(o) = idx(w, k) {
                    let s = w.mobys[o].state;
                    if s == 0xfe || s == 0xfd || s == 3 { w.mobys[id].set_state(2, -1); }
                }
            }
        }
        2 => {
            let m = &w.mobys[id];
            let r = m.rows;
            let at: [f32; 3] = std::array::from_fn(|k| r[2][k] * 3.0 * m.scale + m.pos[k]);
            if m.drawn != 0 {
                let e = Explosion {
                    flash: 1.0, flash2: 0.5, near: 9.0, size: m.scale * 1.3, light: 15.0, embers: 10, billows: 3, flares: 0x10, sound: -1,
                    shake: false, debris: 1, lod: -1, ..Default::default()
                };
                blast::explode(w, ctx, Some(id), Some(at), &e);
            }
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 1, [0.0; 3], 0xff);
            bolts::level_drop(w, id, 0.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            let m = &mut w.mobys[id];
            if m.first() {
                m.mode |= 1;
                m.has_collision = false;
                m.drawn = 0;
                m.set_pv_i(8, 0x78);
            }
            let mut k = m.pv_i(8);
            let out = crate::moby::countdown(&mut k);
            m.set_pv_i(8, k);
            if out { w.delete(id); }
        }
        _ => {}
    }
}

pub const TRIGGER: i16 = 0x04f7;

/// 0x04f7 (module docs).
pub fn update_04f7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let mut slot = 0;
    for _ in 0..8 {
        let g = m.pv_i(slot * 4);
        if g != -1 {
            if crate::moby::classes::props::group_alive(w, g) != 0 { return; }
            slot += 1;
        }
    }
    if ctx.hero.state == 0x31 { return; }
    let c = m.pv_i(0x28);
    if c != -1 && !ctx.level.in_cuboid(c, ctx.hero.pos) { return; }
    let (timer, to, hook) = (m.pv_i(0x24), m.pv_i(0x20), m.pv_i(0x2c));
    let m = &mut w.mobys[id];
    if timer != -1 { m.timer = timer as u8; }
    if to == 0xff {
        let (uid, mission) = (m.uid, m.mission);
        ctx.game.uid_gone(uid, mission);
        w.delete(id);
        return;
    }
    if to != -1 { m.state = to as u8; }
    if hook != -1 { crate::moby::classes::maktar::hook_group(w, hook, true); }
}

pub const BOX: i16 = 0x0f88;
/// `$gp+0x2dcc`: its drag's radius.
const BOX_R: f32 = 1.0;
const BOX_HOLD: f32 = 7.0;

/// 0x0f88 (module docs).
pub fn update_0f88(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.mode |= crate::moby::mode::TARGET;
            m.set_pv_f(0x84, BOX_HOLD);
            m.set_pv_i(0xdc, m.pv_i(0xdc) | 1);
            m.set_pv_f(0x88, m.pv_f(0xe4));
            m.set_state(1, -1);
        }
        1 => {
            let path = m.pv_i(0xe0);
            crate::moby::classes::tractor::drag(w, id, ctx, BOX_R, path);
        }
        _ => {}
    }
    ride(w, id, ctx, 0xa0, old, old_rot);
}

pub const PAD: i16 = 0x1209;

/// 0x1209 (module docs).
pub fn update_1209(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(0x20), 0.104_719_77);
    m.set_pv_f(0x20, a);
    let shade = (((a.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(32, 128) as u32;
    match m.state {
        0 => {
            m.mode |= crate::moby::mode::GLOW;
            m.set_state(1, -1);
        }
        1 => {
            let joint = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
            let d = dist2(ctx.hero.pos, joint);
            let class_scale = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.set_pv_i(0x24, 0);
            let on = ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0;
            if on && d < m.scale * 0.5 / class_scale {
                m.set_state(2, -1);
                for k in 0..4 {
                    let o = w.mobys[id].pv_i(4 * k);
                    if o == -1 { continue; }
                    let hook = w.mobys[id].pv_i(0x10 + 4 * k) != 0;
                    if let Some(o) = idx(w, o) {
                        if hook { w.mobys[o].once |= 4 } else { w.mobys[o].timer = 1 }
                    }
                }
                let event = w.mobys[id].pv_i(0x28) as u8;
                ctx.game.mission_done(event);
            }
            w.mobys[id].glow = shade | 0x80 << 24;
        }
        2 => {
            if m.pv_i(0x24) == 0 { m.set_pv_i(0x24, 1); }
            m.glow = shade << 16 | shade << 8 | 0x80 << 24;
        }
        _ => {}
    }
}

pub const BRIDGE: i16 = 0x11e8;
/// The crushed moby's class.
const CRUSHED: i16 = 0x0cbc;

/// `0x446620`: the troopers 0x0d0f of its group (`+0x21`) held in reserve at tier 1 (their `+0x302` bit 0, `+0x420`
/// 1) woken: `+0x302` 2, their update on (mode & !2).
fn wake(w: &mut MobyWorld, id: usize) {
    let group = w.mobys[id].group as u8;
    if group == 0xff { return; }
    let members = w.groups.get(group as usize).cloned().unwrap_or_default();
    for e in members {
        let Some(m) = w.mobys.get_mut(e) else { break };
        if m.o_class == crate::moby::classes::trooper::CLASS && m.pv_b(0x302) & 1 != 0 && m.pv_i(0x420) == 1 {
            m.set_pv_b(0x302, 2);
            m.mode &= !2;
        }
    }
}

/// 0x11e8 (module docs).
pub fn update_11e8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use std::f32::consts::FRAC_PI_2;
    let target = idx(w, w.mobys[id].pv_i(0));
    // Its pitch eased toward `to` (rate, brake, most); true once at or past `to` the way `down` says.
    let ease = |w: &mut MobyWorld, to: f32, a: f32, b: f32, most: f32, down: bool| {
        let m = &mut w.mobys[id];
        let (mut x, mut v) = (m.rot[0], m.pv_f(4));
        crate::camera::approach(to, a, b, most, &mut x, &mut v);
        m.rot[0] = x;
        m.set_pv_f(4, v);
        let there = if down { x <= to } else { to <= x };
        if there { m.set_pv_f(4, 0.0); }
        there
    };
    let (fall, bounce, settle) = ((0.000_533_295_1, 174.532_94, 290.888_28), (0.017_453_294, 0.000_436_332_37, 0.872_664_75), (0.000_436_332_37, 0.174_532_95, 2.908_882_6));
    match w.mobys[id].state {
        0 => {
            let o = w.mobys[id].pv_i(8);
            w.mobys[id].set_pv_i(0x10, if 0 <= o { o } else { -1 });
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if target.is_some_and(|t| w.mobys[t].once & 4 != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
            }
            if mission_done(ctx, w.mobys[id].mission) {
                let m = &mut w.mobys[id];
                m.rot[0] = crate::hero::physics::sub_rot(m.rot[0], FRAC_PI_2);
                m.set_state(6, -1);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let (mut x, mut v) = (m.rot[0], m.pv_f(4));
            crate::camera::approach(-FRAC_PI_2, fall.0, fall.1, fall.2, &mut x, &mut v);
            m.rot[0] = x;
            m.set_pv_f(4, v);
            if x <= -std::f32::consts::FRAC_PI_4 {
                if let Some(k) = idx(w, w.mobys[id].pv_i(0x10)) {
                    let o = &w.mobys[k];
                    if o.o_class == CRUSHED && o.state != 0xfe && o.state != 0xfd {
                        // `0x4130f8` (L09 `0x3ec3d0`): the gunship's death.
                        crate::moby::classes::level09::gunship::die(w, k, ctx);
                    }
                }
            }
            if w.mobys[id].rot[0] <= -FRAC_PI_2 {
                w.mobys[id].set_pv_f(4, 0.0);
                w.mobys[id].set_state(3, -1);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        3 => {
            if ease(&mut *w, -1.308_997, bounce.0, bounce.1, bounce.2, false) { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if ease(&mut *w, -FRAC_PI_2, settle.0, settle.1, settle.2, true) {
                w.mobys[id].set_state(5, -1);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        5 => {
            if ease(&mut *w, -1.483_53, bounce.0, bounce.1, bounce.2, false) { w.mobys[id].set_state(6, -1); }
        }
        6 => {
            if w.mobys[id].first() { wake(w, id); }
            if ease(&mut *w, -FRAC_PI_2, settle.0, settle.1, settle.2, true) {
                w.mobys[id].set_state(7, -1);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        _ => {}
    }
    if let Some(t) = target {
        let m = &w.mobys[id];
        let p = crate::hero::physics::add(crate::hero::physics::add(m.pos, crate::hero::physics::set_len(m.rows[2], 5.7)), crate::hero::physics::set_len(m.rows[1], 0.35));
        w.mobys[t].pos = p;
    }
}
