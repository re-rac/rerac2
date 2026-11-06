//! The museum's teleporter pad **0x0f51** (L13 `0x41cb00`, 3 placed; L21 `0x40d208`, 1; the decomp's
//! `func_US2_L13_0041CAFC` / `func_US2_L30_0040D208`, one code): a pad without arms that sends Ratchet to its partner
//! pad, to the Insomniac Museum (level 21) or back from it. Its pvars: the partner `+0`, the other partner `+4` once
//! the mission `+0xc` is done, the mission `+8` it waits for (−1: none), the prompt `+0x10`, two eased values `+0x20`
//! / `+0x24` with their speeds `+0x28` / `+0x2c` (read by its draw), the beam `+0x34`, and the words `+0x38` on (it
//! does nothing without it), `+0x3c` it sends to the museum, `+0x40` it sends back from it.
//!
//! * **0** 1 (the museum's word from the menu, below, taken).
//! * **1** with its mission done (or none), something to send to, and Ratchet on it (his ground moby, grounded): its
//!   prompt (owner 0xb: `+0x10` 0 0x1bc6, 1 0x130b, else 0x12e5; `0x41cf28`); △ with the prompt his: its sound 0, he
//!   walks to its middle facing its yaw (`0x2def40` = L02 `0x2d7940`), a 1.5 sphere pushes things off it (20,
//!   `0x3273f8`), 2.
//! * **2** Ratchet's walk (`0x2def90` = L02 `0x2d7990`): there, the beam full (1.95), 3; not walking, 4.
//! * **3** the beam (drawn) down by 1/15 a tick; out: to a partner, Ratchet on it 0.2 up facing its way (`0x2c4e70` =
//!   L02 `0x2bd368`), the partner powering down (4: its beam from 0, its values 1, its sound 1), this pad back at 1
//!   (its beam full, its values 0); else the level change (`0x2ff7c0` = L00 `0x2f50f8`, with the save): back from the
//!   museum to level 13, to it level 21.
//! * **4** the beam (drawn) back up to 1.95 by 1/15, then `+0x24`, then `+0x20` eased down to 0 (`$gp+0x2688..0x2690`,
//!   L21 `$gp+0x27c0..`: 12/3600, 12/3600, at most 32/60), 1.
//!
//! Not ported: its draw (L13 `0x41cfa8`, the decomp's `func_L13_0041BAE0`); the view lock `0x18c0d5` it sets when it
//! sends Ratchet (as crate::moby::classes::telepad); the return to the level the special menu came from (`0x1a7904`
//! = 1 with the level `0x1a7908`, set by `0x2d1f28`, which the port does not have: always level 13).

use crate::hero::physics::{approach, ease};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f51;

mod pv {
    pub const PARTNER: usize = 0;
    pub const PARTNER2: usize = 4;
    pub const MISSION: usize = 8;
    pub const MISSION2: usize = 0xc;
    pub const TEXT: usize = 0x10;
    pub const A: usize = 0x20;
    pub const B: usize = 0x24;
    pub const A_V: usize = 0x28;
    pub const B_V: usize = 0x2c;
    pub const BEAM: usize = 0x34;
    pub const ON: usize = 0x38;
    pub const OUT: usize = 0x3c;
    pub const BACK: usize = 0x40;
}

const BEAM_FULL: f32 = 1.95;
const BEAM_STEP: f32 = 0.066_666_67;
/// The museum (our level 21) and the level its pad sends back to.
const MUSEUM: i32 = 0x15;
const HOME: i32 = 0xd;

fn mission_done(ctx: &Ctx, mission: i32) -> bool {
    let g = &*ctx.game;
    usize::try_from(mission).ok().and_then(|k| g.missions.get(g.level.max(0) as usize * 0x10 + k)).is_some_and(|b| b & 0x80 != 0)
}

/// 0x0f51 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x44 { w.mobys[id].pvars.resize(0x44, 0); }
    if w.mobys[id].pv_i(pv::ON) == 0 { return; }
    match w.mobys[id].state {
        0 => w.mobys[id].state = 1,
        1 => {
            let m = &w.mobys[id];
            let mission = m.pv_i(pv::MISSION);
            if 0 <= mission && !mission_done(ctx, mission) { return; }
            if m.pv_i(pv::PARTNER) == -1 && m.pv_i(pv::BACK) == 0 && m.pv_i(pv::OUT) == 0 { return; }
            if ctx.hero.ground_moby != Some(id) || ctx.hero.air_ticks != 0 { return; }
            let text = match m.pv_i(pv::TEXT) { 0 => 0x1bc6, 1 => 0x130b, _ => 0x12e5 };
            ctx.game.prompt.request(0xb, text, 1);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE == 0 || ctx.game.prompt.owner != 0xb { return; }
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let m = &mut w.mobys[id];
            m.state = 2;
            m.set_pv_i(pv::A_V, 0);
            m.set_pv_i(pv::B_V, 0);
            let (yaw, at) = (m.rot[2], m.pos);
            ctx.hero.script_walk_to(yaw, at, false);
            ai::attack_sphere(w, id, ctx, 1.5, 20.0, 1.0, at, 0x10000, 0, 1, 0);
        }
        2 => {
            // `0x2def90`: walking there (0x62..0x63), there (100), else not.
            match ctx.hero.state {
                100 => {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(pv::BEAM, BEAM_FULL);
                    m.state = 3;
                }
                0x62..=0x63 => {}
                _ => w.mobys[id].state = 4,
            }
        }
        3 => {
            let m = &w.mobys[id];
            if m.pv_f(pv::BEAM) != 0.0 {
                let m = &mut w.mobys[id];
                let mut b = m.pv_f(pv::BEAM);
                approach(0.0, BEAM_STEP, &mut b);
                m.set_pv_f(pv::BEAM, b);
                return;
            }
            if m.pv_i(pv::OUT) != 0 || m.pv_i(pv::BACK) != 0 {
                ctx.game.go_to_level(if m.pv_i(pv::BACK) != 0 { HOME } else { MUSEUM }, true);
                return;
            }
            let mut to = m.pv_i(pv::PARTNER);
            if m.pv_i(pv::PARTNER2) != -1 && mission_done(ctx, m.pv_i(pv::MISSION2)) { to = m.pv_i(pv::PARTNER2); }
            if let Some(p) = usize::try_from(to).ok().filter(|&p| p < w.mobys.len()) {
                let (at, rot) = (w.mobys[p].pos, w.mobys[p].rot);
                ctx.hero.teleport([at[0], at[1], at[2] + 0.2], rot);
                ctx.hero.request_state(0, true);
                let pm = &mut w.mobys[p];
                pm.state = 4;
                for o in [pv::BEAM, pv::A_V, pv::B_V] { pm.set_pv_i(o, 0); }
                pm.set_pv_f(pv::A, 1.0);
                pm.set_pv_f(pv::B, 1.0);
                ctx.host.play_sound(&w.mobys[p], 1, 0);
            }
            let m = &mut w.mobys[id];
            m.state = 1;
            m.set_pv_f(pv::BEAM, BEAM_FULL);
            for o in [pv::A, pv::B, pv::A_V, pv::B_V] { m.set_pv_i(o, 0); }
        }
        4 => {
            let m = &mut w.mobys[id];
            if m.pv_f(pv::BEAM) < BEAM_FULL {
                let mut b = m.pv_f(pv::BEAM);
                approach(BEAM_FULL, BEAM_STEP, &mut b);
                m.set_pv_f(pv::BEAM, b);
                return;
            }
            for (x, v) in [(pv::B, pv::A_V), (pv::A, pv::B_V)] {
                if 0.0 < m.pv_f(x) {
                    let (mut p, mut s) = (m.pv_f(x), m.pv_f(v));
                    ease(0.0, 12.0 * 0.000_277_777_78, 12.0 * 0.000_277_777_78, 32.0 * 0.016_666_668, &mut p, &mut s);
                    m.set_pv_f(x, p);
                    m.set_pv_f(v, s);
                    return;
                }
            }
            m.state = 1;
        }
        _ => {}
    }
}
