//! The escort 0x0f82 (Feltzin, Hrugis and Gorn: L05 `0x418668`, L10 `0x417600`, L15 `0x420eb0`; the decomp's
//! `func_L05_004170C8`): the friendly ship flying the mission's route, on the space-AI library ([`super`]).
//!
//! **Its start** (state 0: once the library's ready flag is set, `0x367720`, here taken as set) `0x4189e8`: state 1,
//! the rows kept, health 1 (`+0x148`, `+0x180`), active (`0x37c130(m, 1)`, [`super::start`]), at a random point of
//! the fight's sphere, scale 0.02, the fly-by's wait 120 (`+0x1c8`), the sounds' handles none (`+0x1c0` / `+0x1c4`),
//! the route not begun (`+0x1b8`). With its disc's leader word (`+0x1b4`) it joins the leaders' list with a callback
//! (`0x37b0d0(m, 0x418cd0, 0, 0)`: the route begun again and the done flag cleared, [`restart`], called by the
//! missions' group activation `0x37c4f0`, crate::moby::classes::space::activate_group) and the done flag
//! (`0x1b0f50`) is cleared.
//!
//! **Each tick** while active (`+0x110`): the route is the level's path `0x1fe380[0x221d48]` (the route the mission
//! picks, [`super::route`]; `{count, …}` then its points 16 bytes each). The AI state is 0xb (`0x37dff0(m, 0xb)`:
//! the tuning 0.02 / 0.1 / 0.5, no no-fly volumes). Not begun, it starts at the route's first point, its goal the
//! second (`+0x1bc` = 2, the 1-based goal), its velocity 1 toward it, the goal's timer 2000000. At each goal
//! (`0x37e7c8`, [`super::arrived`]) the next one, or past the last the done flag set. While not done the AI tick
//! (`0x37d3d0`, [`super::think`]) and the range test (`0x37cbb8(m, 1, 32)`: [`super::range`] by its [`super::Spec`]).
//! With its own target (disc `+0x1b0`, the player's ship): `+0x11c` 1 and `+0x120` that ship; within 15 of it, with
//! the ship facing it (its first row against the way to the escort, above 0.4) the ship's sound 0xe, facing away
//! (below −0.4) its sound 0xb, each when the fly-by's wait is out (again in 180..240 ticks). Then the exhaust
//! (`0x418af0`); its engine's loop (`0x36cc38(0, 6, m, 0xf82)`, `+0x1c0`) whenever it is not playing.
//!
//! It neither fires nor takes hits (no `0x37c7a8`).
//!
//! Not ported: the exhaust (`0x418af0`: −0.503 behind, three puffs 0.4·(30..40)/60 back, the descriptors
//! `0x1b0f58` / `0x1b0f68` / `0x1b0f78`, not in crate::particles).

use crate::hero::physics::{dist, dot, sub, unit};
use crate::moby::{Ctx, MobyWorld};

use super::{a, ai_i, ai_v, set_ai_i, set_ai_v, Spec, FIGHTER_SPEC};

pub const CLASS: i16 = 0x0f82;

/// The disc words: its own target, its leader word, the route begun, the 1-based goal, the engine's and the fly-by's
/// sound handles, the fly-by's wait.
const TARGET: usize = 0x1b0;
const LEADS: usize = 0x1b4;
const BEGUN: usize = 0x1b8;
const GOAL: usize = 0x1bc;
const FLYBY_WAIT: usize = 0x1c8;
/// The done flag (`0x1b0f50`, one for the level: here the escort's own port word).
const DONE: usize = 0x5c0;

/// Its numbers on the library ([`super::Spec`]): the range test's 32 and its screen check; the engine's loop its
/// sound 0. The fly-by is its own (above), so none from the library.
pub(crate) const SPEC: Spec = Spec {
    health: 1.0,
    scale: 0.02,
    range: 32.0,
    on_screen: true,
    target: TARGET,
    leads: LEADS,
    gun: BEGUN,
    flyby: 0,
    engine: Some((CLASS, 0)),
    death: None,
    gun_sound: None,
    nozzles: &[],
    ..FIGHTER_SPEC
};

/// `0x418cd0` (L10 / L15 the same; the decomp's `func_L05_00417730`): the leaders' callback of the group's
/// activation — the done flag cleared, the route not begun (`+0x1bc`, `+0x1b8`).
pub(crate) fn restart(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_i(DONE, 0);
    m.set_pv_i(GOAL, 0);
    m.set_pv_i(BEGUN, 0);
}

/// `0x42b730(escort)` (the gate races' test): the level's done flag (`0x1b0f50`: an escort past its route's end).
pub(crate) fn done(w: &MobyWorld) -> bool { w.mobys.iter().any(|m| m.o_class == CLASS && m.state < 0x80 && m.pv_i(DONE) != 0) }

/// `0x418668`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(super::STARTED) == 0 {
        // `0x4189e8`.
        super::start(w, id, ctx);
        let m = &mut w.mobys[id];
        m.set_pv_i(FLYBY_WAIT, 0x78);
        m.set_pv_i(0x1c0, -1);
        m.set_pv_i(0x1c4, -1);
        m.set_pv_i(BEGUN, 0);
        m.set_pv_i(DONE, 0);
        return;
    }
    if ai_i(w, id, a::ACTIVE) == 0 { return; }
    let path = super::route(w, ctx);
    set_ai_i(w, id, a::STATE, 0xb);
    if w.mobys[id].pv_i(BEGUN) == 0 {
        w.mobys[id].set_pv_i(BEGUN, 1);
        if let (Some(p0), Some(p1)) = (path.first(), path.get(1)) {
            w.mobys[id].pos = [p0[0], p0[1], p0[2]];
            w.mobys[id].set_pv_i(GOAL, 2);
            super::set_goal(w, id, ctx, [p1[0], p1[1], p1[2]]);
            let v = unit(sub(ai_v(w, id, a::GOAL), w.mobys[id].pos));
            set_ai_v(w, id, a::VEL, v);
            set_ai_i(w, id, a::TIMER, 2_000_000);
        }
    } else if super::arrived(w, id) {
        let next = w.mobys[id].pv_i(GOAL);
        if path.len() as i32 <= next {
            w.mobys[id].set_pv_i(DONE, 1);
        } else {
            w.mobys[id].set_pv_i(GOAL, next + 1);
            let p = path[next.max(0) as usize];
            super::set_goal(w, id, ctx, [p[0], p[1], p[2]]);
            set_ai_i(w, id, a::TIMER, 2_000_000);
        }
    }
    if w.mobys[id].pv_i(DONE) == 0 {
        super::think(w, id, ctx);
        super::range(w, id, ctx);
    }
    if ai_i(w, id, a::ACTIVE) == 1 {
        if let Some(t) = super::own_target(w, id) {
            set_ai_i(w, id, a::OWN_TARGET, 1);
            set_ai_i(w, id, a::TARGET, t as i32 + 1);
            let (me, ship) = (w.mobys[id].pos, w.mobys[t].pos);
            if dist(ship, me) < 15.0 {
                let facing = dot(unit(w.mobys[t].rows[0]), unit(sub(me, ship)));
                let mut wait = w.mobys[id].pv_i(FLYBY_WAIT);
                let sound = if 0.4 < facing {
                    crate::moby::countdown(&mut wait).then_some(0xe)
                } else if facing < -0.4 {
                    crate::moby::countdown(&mut wait).then_some(0xb)
                } else {
                    None
                };
                if let Some(s) = sound {
                    let h = ctx.host.play_sound_of(&w.mobys[id], crate::ship::SHIP_CLASS, s, 0);
                    if s == 0xe { w.mobys[id].set_pv_i(0x1c4, h); }
                    wait = ctx.game.rng.randf(180.0, 240.0) as i32;
                }
                w.mobys[id].set_pv_i(FLYBY_WAIT, wait);
            }
        }
    }
    super::engine(w, id, ctx);
    w.rebuild(id, &*ctx.host);
}
