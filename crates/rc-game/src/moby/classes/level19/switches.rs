//! Grelbin's one-off controllers (L19 addresses).
//!
//! * **0x1205** ([`update_1205`], `0x423b40`, 1 placed; the decomp's v1.01 `func_L19_00422810`): a clearer. Set off by
//!   its timer byte, a hook (`0x327970` = L00 `0x32a838`), its moby `+0x00` gone (or none) or its mission done, it
//!   deletes every member of its groups `+0x04..+0x0c` (−1 none), hooks every member of its groups `+0x20..+0x2c`
//!   (`0x31a390`, L02 `0x31f858`: L00 `0x3294f0(m, 1)`, crate::moby::Moby::once bit 2), counts its uid and goes.
//! * **0x11f4** ([`update_11f4`], `0x422220`, 1 placed; the decomp's v1.01 `func_L19_00420EF0`): a spinning carrier,
//!   a twin of Aranos's 0x0c66 (crate::moby::classes::carriers) without the bob and sway. It takes its tie group
//!   `+0x00`, moby group `+0x04` and shrub group `+0x08` (at most 0x32, `0x322118` = L00 `0x3246c0`; none: it goes),
//!   its frame its rows (mode 0x104); while the level's variant (`0x1c9fc0`) is its `+0x0c`, its rows turn by
//!   `+0x10..+0x18` °/s and its members' by `+0x1c..+0x24` °/s (`0x2ed010` = L00 `0x2efa00`), carried (`0x322628` =
//!   L00 `0x324bd0`).

use crate::moby::attach::{rows_of, turn};
use crate::moby::{mode, Ctx, MobyWorld};

pub const CLEARER: i16 = 0x1205;
pub const SPINNER: i16 = 0x11f4;

/// °/s as radians a tick.
const DEG_S: f32 = 0.000_290_888_25;

/// 0x1205 (module docs).
pub fn update_1205(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.timer == 0 && m.once & 4 == 0 {
        let first = m.pv_i(0);
        let alive = usize::try_from(first).ok().and_then(|k| w.mobys.get(k)).is_some_and(|o| o.state != 0xfe && o.state != 0xfd);
        let done = ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
        if first != -1 && alive && !done { return; }
    }
    for k in 0..3 {
        let g = w.mobys[id].pv_i(4 + 4 * k);
        if g == -1 || g & 0xff == 0xff { continue; }
        for i in usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() { w.delete(i); }
    }
    for k in 0..4 {
        let g = w.mobys[id].pv_i(0x20 + 4 * k);
        if g == -1 { continue; }
        for i in usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() {
            if let Some(o) = w.mobys.get_mut(i) { o.once |= 4; }
        }
    }
    ctx.game.count_uid(w.mobys[id].uid);
    w.delete(id);
}

/// 0x11f4 (module docs).
pub fn update_11f4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            if m.pv_i(0) == -1 && m.pv_i(4) == -1 { return; }
            m.update_dist = 0xff;
            let (tie, moby, shrub) = (m.pv_i(0), m.pv_i(4), m.pv_i(8));
            w.attach(id, 0x32, moby, tie, shrub);
            if !w.attached.contains_key(&id) { return w.delete(id); }
            w.attach_use_rows(id);
            let m = &mut w.mobys[id];
            m.mode |= mode::KEEP_ROWS | mode::NO_MATRIX;
            m.rows = rows_of(m.rot);
            m.state = 1;
        }
        1 => {
            if ctx.game.variant != m.pv_i(0xc) { return; }
            let spin = rows_of([m.pv_f(0x10) * DEG_S, m.pv_f(0x14) * DEG_S, m.pv_f(0x18) * DEG_S]);
            m.rows = turn(&m.rows, &spin);
            let members = rows_of([m.pv_f(0x1c) * DEG_S, m.pv_f(0x20) * DEG_S, m.pv_f(0x24) * DEG_S]);
            w.attach_turn(id, &members);
            w.carry(id, &*ctx.host);
        }
        _ => {}
    }
}

pub const PLANET_SCENE: i16 = 0x1267;
/// The planet 0x1267 unlocks (and is gone once it is).
const PLANET: i32 = 0x14;

/// 0x1267 (`0x424dc8`, 1 placed; the decomp's v2.00 `func_US2_L19_00424DC8`): the scenes that give the next planet.
/// * **0**: without collision, hidden and untargeted; with planet 0x14 unlocked already it goes; else its shadow's
///   slab 0 / 0.2 (`0x31b670`) and 1.
/// * **1**: Ratchet in its cuboid `+0x120` (`0x31f178` = L00 `0x321858`): the scene 0x96, the movie 0x14 and the
///   scene 0x97 queued (the first refused, it tries again; `0x2f24f8` = L00 `0x2f4ee8`) and started (`0x2f2640` =
///   L00 `0x2f5030`), 2.
/// * **2**: hidden while a scene shows; once they are over planet 0x14 is unlocked (`0x320160` = L00 `0x322730`) with
///   its banner (`0x322c00` = L00 `0x325270`), Ratchet is put at the cuboid's place (`0x2bc0e0` = L00 `0x2b9e68`, state
///   0), it is shown, its mission done (`0x310c68` = L00 `0x3132a8`) and the checkpoint made at Ratchet (`0x357930` =
///   L00 `0x359b20`) when it has one; it goes.
///
/// At the scenes' start its talk record is marked (`0x32d3f8` = L01 `0x3259b0`, crate::moby::talk::mark), the level's
/// NPC busy word `0x1b26a8` is held through them (crate::moby::talk::busy), and the autosave follows at the end
/// (`0x30af90` = L00 `0x30d658(0, −1)`, crate::game::GameState::autosave).
///
/// Not ported: its head `0x424ce8` (the objective marker on the class in the list `0x1bd6c0` with the option
/// `0x1a7a48`).
pub fn update_1267(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const AT: usize = 0x120;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & !mode::TARGET;
            if ctx.game.planets.get(PLANET as usize).is_some_and(|&b| b != 0) { return w.delete(id); }
            m.set_state(1, -1);
            m.shadow_slab = [0.0, f32::from_bits(0x3e4c_cccd)];
        }
        1 => {
            if !ctx.level.in_cuboid(w.mobys[id].pv_i(AT), ctx.hero.pos) { return; }
            let g = &mut *ctx.game;
            if !g.scenes.queue.push(0x96) { return; }
            g.scenes.queue.push(0x14);
            g.scenes.queue.push(0x97);
            crate::moby::talk::mark(w, id, ctx);
            ctx.game.scenes.start_queue(ctx.hero.hp);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            crate::moby::talk::busy(ctx, true);
            if ctx.game.scenes.mode == crate::scene::SCENE {
                let m = &mut w.mobys[id];
                m.drawn = 0;
                m.mode |= 1;
            }
            if ctx.game.scenes.in_scene() { return; }
            crate::moby::talk::busy(ctx, false);
            ctx.game.unlock_planet(PLANET);
            crate::travel::new_planet_banner(ctx.game, PLANET);
            if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(AT)) {
                ctx.hero.teleport(p, r);
                ctx.hero.request_state(0, true);
            }
            let m = &mut w.mobys[id];
            m.mode &= 0xfffe;
            m.drawn = 1;
            if m.mission != 0xff {
                ctx.game.mission_done(m.mission);
                ctx.game.save_checkpoint(ctx.hero.pos, ctx.hero.rot);
            }
            ctx.game.autosave(-1);
            w.delete(id);
        }
        _ => {}
    }
}
