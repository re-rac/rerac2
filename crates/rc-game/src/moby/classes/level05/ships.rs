//! Feltzin's space ships (L05 addresses): the space-AI library's ships (crate::moby::classes::space, its
//! `update_fighter` with these numbers) and the carrier that sends them.
//!
//! Each ship's update is the Wupash fighter's (`0x3ca118`) pattern: its start (the set-up, `0x37c130(m, 1)` active,
//! at a random point of the fight's sphere, wandering), then every tick while active (`+0x110`) the AI tick
//! (`0x37d3d0`), the push and the hits (`0x37c7a8(m, 0, 0, 0)`: the level's handler `0x421c18`), the range test
//! (`0x37cbb8`), its disc target (`+0x260`), the gun, the attack's start (`0x37caf8` / `0x37c940`), the gun's wait
//! (`+0x268`), the exhaust and the engine's loop.
//!
//! * **0x0f73** ([`FIGHTER`], `0x416b40`, 10 placed; its set-up `0x416d50`): health 20 (`+0x180`; the AI's `+0x148`
//!   20), scale 0.02; in range within 32 and on screen; fires in the cone (`0x416e50`: `0x37b210(0.75, 5, 1, 1, m,
//!   0x60800000, …)`: the shot 5 ahead of the target, every 20 ticks); the fly-by (one in 400 within 10, the ship's
//!   0xe); the exhaust (`0x416ef8`: −0.31 behind, ×0.4, `$gp+0x1e98..`: the Wupash fighter's descriptors); the
//!   engine's loop its sound 0 (`+0x26c`).
//! * **0x0f76** ([`CRUISER`], `0x4170d8`, 5 placed; `0x417348`): health 5, scale 0.02; in range within 32 and on
//!   screen; fires in range and the cone (`0x4174b8`: `(0.75, 5, 1, 1, m, 0x60408020)` every 15 ticks, its sound 0
//!   two shots in three); the engine's loop its sound 1 (`+0x274`).
//! * **0x10f5** ([`FLAMER`], `0x41de00`, 30 placed; `0x41e010`): health 5, scale 0.08; in range within 64 and on
//!   screen; in range and the cone the flame every 4 ticks (crate::moby::classes::space's flame: `0x41e108`); no
//!   engine's loop. 0x111a sends it again once it is put away ([`update_carrier`]).
//! * **0x111a** ([`CARRIER`], [`update_carrier`], `0x424108`, 12 placed; `0x424250`): health 10, left where it is
//!   (`0x37c7a8(m, 0, 1, 1)`: no push, the hits only). Every 120 ticks (`+0x26c`) one of its group's active
//!   ships at random (`0x37c610`: the group's count; with fewer than 2 itself) launches a 0x10f5 (`0x4242c8`): the
//!   first put-away 0x10f5 of its group (`0x37c6b0(0x10f5, 1)`: the group of the leaders' list's first 0x10f5,
//!   activated, `0x37c130`) at one of the launcher's joints 0..4 (`0x32abb8` / `0x32ab18`), facing along it, the
//!   left row across it and the level's up, wandering from there (`0x37e480`).
//!
//! Not ported: the exhaust of 0x0f76 (`0x417570`: −0.241 behind ×0.4, `$gp+0x1ec8..` — (0x60904090, 0x00904090,
//! −20, 60, 10), (0x00204090, 0x80, −20, 60, 12), (0xefff7f4f, 0x00ff0000, −40, 40, 8)) and 0x10f5 (`0x41e3a0`, on
//! screen: −0.8 behind, (7.5..10)/60 ×0.5, `$gp+0x22c0..` — (0x290000ff, 0xcf, −5, 60, 40), (0x190080ff, 0x80, −5,
//! 60, 48), (0x29ff7f4f, 0x00ff8000, −10, 40, 32)): crate::particles has no descriptors of theirs; 0x0f76's two
//! engines 0x0f81 (`0x418308`, made at its start) and its beams (`0x417750`, the pool `0x265960`: from its joint 0
//! to the target, hitting the ship for 1); 0x10f5's flame puffs and its draw callback (`0x41e730` → `0x41e588`), its
//! sequence 1 and its speed 0.9..1.1 (`+0x48`); the hit flash.

use crate::hero::physics::{cross, unit, V3};
use crate::moby::classes::space::{self, Spec, FIGHTER_SPEC};
use crate::moby::{Ctx, MobyWorld};

pub const FIGHTER: i16 = 0x0f73;
pub const CRUISER: i16 = 0x0f76;
pub const FLAMER: i16 = 0x10f5;
pub const CARRIER: i16 = 0x111a;

/// No exhaust (its descriptors not in crate::particles).
const NONE: &[V3] = &[];

/// The ships' numbers.
pub(crate) fn spec(o_class: i16) -> Option<Spec> {
    let base = Spec { scatter: true, target: 0x260, leads: 0x264, gun: 0x268, flyby: 0, engine: None, death: None, gun_sound: None, ..FIGHTER_SPEC };
    Some(match o_class {
        FIGHTER => Spec { health: 20.0, scale: 0.02, range: 32.0, on_screen: true, fire_in_range: false, lead: 5.0, wait: 0x14, shot: (0.75, 1.0, 0x6080_0000), flyby: 400, engine: Some((FIGHTER, 0)), nozzles: &[[-0.31, 0.0, 0.0]], puff: (30.0, 40.0, 0.4), desc: 0, exhaust_on_screen: false, ..base },
        CRUISER => Spec { health: 5.0, scale: 0.02, range: 32.0, on_screen: true, lead: 5.0, wait: 0xf, shot: (0.75, 1.0, 0x6040_8020), engine: Some((CRUISER, 1)), gun_sound: Some((CRUISER, 0, true)), nozzles: NONE, ..base },
        FLAMER => Spec { health: 5.0, scale: 0.08, range: 64.0, on_screen: true, flame: true, wait: 4, nozzles: NONE, ..base },
        CARRIER => Spec { health: 10.0, scatter: false, nozzles: NONE, ..base },
        _ => return None,
    })
}

/// `0x424108`.
pub fn update_carrier(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(space::STARTED) == 0 {
        // `0x424250`: health 10, active (`0x37c130(m, 1)`), the launch's wait 0.
        w.mobys[id].set_pv_i(space::STARTED, 1);
        w.mobys[id].state = 1;
        w.mobys[id].mode |= crate::moby::mode::KEEP_ROWS;
        space::activate(w, id, ctx);
        w.mobys[id].set_pv_i(0x26c, 0);
        return;
    }
    space::hits_with(w, id, ctx, false, true, true);
    if w.mobys[id].pv_i(0x264) == 0 { return; }
    let mut t = w.mobys[id].pv_i(0x26c);
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(0x26c, t);
    if !out { return; }
    let members = group(w, id);
    let active: Vec<usize> = members.iter().copied().filter(|&m| space::ai_i(w, m, space::a::ACTIVE) == 1).collect();
    let from = if 1 < active.len() {
        // `0x333d80(0, n − 1)`: the k-th of the group's list (as the game counts, the list's k-th member).
        let k = ctx.game.rng.rand_range(0, active.len() as i32 - 1) as usize;
        members.get(k).copied()
    } else {
        Some(id)
    };
    if let Some(f) = from { launch(w, f, ctx); }
    w.mobys[id].set_pv_i(0x26c, 0x78);
}

/// The moby's group (`0x1fb1a0[+0x21]`), its members.
fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x4242c8`: a put-away 0x10f5 of the first one's group (`0x37c6b0`) activated at a random joint (0..4) of `from`.
fn launch(w: &mut MobyWorld, from: usize, ctx: &mut Ctx) {
    let Some(first) = w.mobys.iter().position(|m| m.o_class == FLAMER && m.pv_i(0x264) != 0) else { return };
    let Some(ship) = group(w, first).into_iter().find(|&m| w.mobys[m].o_class == FLAMER && w.mobys[m].pv_i(space::STARTED) != 0 && space::ai_i(w, m, space::a::ACTIVE) == 0) else { return };
    space::activate(w, ship, ctx);
    let j = ctx.game.rng.rand_range(0, 4) as usize;
    let (rows, at) = ctx.host.joint_matrix(&w.mobys[from], j).unwrap_or((w.mobys[from].rows, w.mobys[from].pos));
    let fwd = rows[0];
    let left = unit(cross([0.0, 0.0, 1.0], fwd));
    let m = &mut w.mobys[ship];
    m.pos = at;
    m.rows = [fwd, left, cross(fwd, left)];
    space::wander(w, ship, ctx);
}
