//! Hrugis's space ships (L10 addresses; 0xf93 is also Gorn's, the same code at L15 `0x421538`): the space-AI
//! library's ships (crate::moby::classes::space, its `update_fighter` with these numbers) and the level's still
//! ones. The library here: the ready flag `0x37c8e0`, the AI block `0x37ccf8`, the leaders' list `0x37ceb8`, the
//! push and hits `0x37e600` (the level's handler `0x4252b0`), the range test `0x37ea10`, the attack's start
//! `0x37e950` / `0x37e798`, the AI tick `0x37f228`.
//!
//! * **0x0f8e** ([`FIGHTER`], `0x419d08`, 30 placed; its set-up `0x419f78`): health 10, scale 0.04, at a random point
//!   of the sphere; in range within 176; fires in the cone (`0x41a078`: `0x37cff8(0.8, 2, 1, 1.3, m, 0x60000080, …)`
//!   every 30 ticks, its sound 1 two shots in three); at its death its engine's loop stopped and its sound 2; the
//!   engine's loop its sound 0 (`+0x26c`).
//! * **0x0f93** ([`WINGMAN`], `0x41a328`, 20 placed, Gorn's 32; `0x41a690`): health 10, scale 0.02, left where it
//!   was placed (`0x380228`: a wander goal from there); in range within 176; fires in range and the cone
//!   (`0x41a788`: `(0.6, 5, 1, 1, m, 0x60800000 / 0x60800080 one shot in two)` every 25 ticks, its sound 0 one shot
//!   in three); at its death the engine's loop stopped and its sound 1; the engine's loop its sound 3.
//! * **0x0f8d** ([`STATION`], [`update_station`], `0x4199a0`, 8 placed; `0x419a90`): health 2, scale 0.16, left
//!   where it is, active (`0x37df20(m, 1)`), drawn from anywhere; each tick the hits alone (`0x37e600(m, 0, 1, 1)`)
//!   and its engine's loop (its sound 0, `+0x268`).
//! * **0x110a** ([`BEACON`], [`update_beacon`], `0x4201b0`, 32 placed; `0x4202f0`): health 1, its class's scale
//!   ×4.1, left where it is, active; each tick the push and the hits (`0x37e600(m, 0, 0, 0)`) and its glow
//!   (`+0x78`) pulsing from 0x20202020 to white (`0x30aa40((sin p + 1)/2, …)`, `p` += 6° a tick from a random
//!   angle, `+0x268`).
//!
//! Not ported: 0xf93's state 2 (scale 0.08, hunting the nearest active 0x0cbe within 80 with its 0x60000080 shots
//! `(0.9, 13, 4.5, 1)`: the mission puts it there) and the hits only its shots give 0x0cbe; 0x110a's lock-on bit
//! (`+0x34` 0x1000 on level 10 in the mission's phase 2, `0x4252a8`) and its group's glow sprites (its leaders'
//! callback `0x4203a0`: FX 0x31 at each active member, 0.25 along its facing); the exhaust (crate::particles has
//! not their descriptors: 0xf8e `0x41a140` −0.45 behind 0.25 up ×0.5, `$gp+0x1f30..` (0xcfff0000, 0x00cf0000, −10,
//! 120, 10), (0x6000ffff, 0x80, −10, 120, 12), (0xefff7f4f, 0x00ff0000, −20, 80, 8); 0xf93 `0x41a8e8` −0.15 behind
//! 0.06 up ×0.5, `$gp+0x1f60..` the same with 0xcf0000ff / 0x000000cf first; 0xf8d `0x419b28` −2.1 behind ×0.8,
//! `$gp+0x1f00..` (0xcfff0000, 0x00cf0000, −80, 240, 10), (0x6000ffff, 0x80, −80, 240, 12), (0xefff7f4f,
//! 0x00ff0000, −160, 160, 8)); the hit flash.

use crate::hero::physics::V3;
use crate::moby::classes::space::{self, Spec, FIGHTER_SPEC};
use crate::moby::{Ctx, MobyWorld};

pub const FIGHTER: i16 = 0x0f8e;
pub const WINGMAN: i16 = 0x0f93;
pub const STATION: i16 = 0x0f8d;
pub const BEACON: i16 = 0x110a;

/// No exhaust (its descriptors not in crate::particles).
const NONE: &[V3] = &[];

/// The ships' numbers.
pub(crate) fn spec(o_class: i16) -> Option<Spec> {
    let base = Spec { target: 0x260, leads: 0x264, gun: 0x268, flyby: 0, engine: None, death: None, gun_sound: None, nozzles: NONE, ..FIGHTER_SPEC };
    Some(match o_class {
        FIGHTER => Spec { health: 10.0, scale: 0.04, fire_in_range: false, lead: 2.0, wait: 0x1e, shot: (0.8, 1.3, 0x6000_0080), engine: Some((FIGHTER, 0)), death: Some((FIGHTER, 2)), gun_sound: Some((FIGHTER, 1, true)), ..base },
        WINGMAN => Spec { health: 10.0, scale: 0.02, scatter: false, lead: 5.0, wait: 0x19, shot: (0.6, 1.0, 0x6080_0000), shot_alt: Some(0x6080_0080), engine: Some((WINGMAN, 3)), death: Some((WINGMAN, 1)), gun_sound: Some((WINGMAN, 0, false)), ..base },
        STATION => Spec { health: 2.0, scale: 0.16, scatter: false, engine: Some((STATION, 0)), ..base },
        BEACON => Spec { health: 1.0, scatter: false, ..base },
        // 0x0cbe (crate::moby::classes::level10::tethered): its leader word `+0x268`.
        super::tethered::CLASS => Spec { health: 2.0, scatter: false, leads: 0x268, ..base },
        _ => return None,
    })
}

/// `0x4199a0`.
pub fn update_station(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(space::STARTED) == 0 {
        // `0x419a90` (`0x37df20(m, 1)`, then drawn and updated from anywhere).
        space::start(w, id, ctx);
    } else {
        space::hits_with(w, id, ctx, false, true, true);
    }
    space::engine(w, id, ctx);
}

/// The beacon's glow phase (`+0x268`).
const PHASE: usize = 0x268;

/// `0x4201b0`.
pub fn update_beacon(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(space::STARTED) == 0 {
        // `0x4202f0`: active (`0x37df20(m, 1)`), its scale its class's ×4.1; the phase a random angle.
        space::start(w, id, ctx);
        let s = w.class(BEACON).map_or(1.0, |c| c.scale) * 4.1;
        let p = ctx.game.rng.rand_angle();
        let m = &mut w.mobys[id];
        m.scale = s;
        m.set_pv_f(PHASE, p);
        return;
    }
    space::hits_with(w, id, ctx, false, false, false);
    let m = &mut w.mobys[id];
    let p = crate::hero::physics::add_rot(m.pv_f(PHASE), 0.104_719_76);
    m.set_pv_f(PHASE, p);
    m.glow = crate::hud::lerp_rgba((p.sin() + 1.0) * 0.5, 0x2020_2020, 0xffff_ffff);
}
