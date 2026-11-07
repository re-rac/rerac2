//! Snivelak's breakables (L16 addresses; the engine functions named by their L16 addresses with the L00 ones the port
//! knows them by).
//!
//! * **0x10f1** ([`update_10f1`], `0x418dc8`, 64 placed; the decomp's `func_L16_00417990`): a lamp with a glow at
//!   (13, 0, 15) in its scaled frame; shot, it breaks, explodes, leaves its stub (0x10f2) and smokes. Its pvars: `[0]`
//!   unbreakable (≠ 0), `[4]` its glow (particle + 1 here), `[8]` the smoke's wait.
//!   * **0** → 1.
//!   * **1**: a glow for two ticks every tick (`0x337380` = L00 `0x33f550`, crate::particles::glow: 3780000 × its
//!     scale, pulled 0.75, 0x28202080, far 0x60). A hit with the flags 0x4010000 (`0x317530`) while `[0]` is 0: 2. Its
//!     hit record let go.
//!   * **2** (the break): its pieces (`0x323a10` = L00 `0x32b580`, crate::moby::shards::throw_all: `$gp+0x2ba0` x
//!     2..5, y −3..3, z 1..7; spin 90..360 (`+0x2b98`), gravity 10..20 (`+0x2b90`); every piece a shard, sound 0,
//!     `$gp+0x2bc0` −1 skipped); 40 puffs of kind 1 (`0x2e61a8` = L00 `0x2ecfb0`, crate::particles::staged_burst:
//!     spread 0.2, speed 0.1167, size 1, fall 5, life 1) at (13, 0, 15) moving (1/6, 0, 0.1167); the explosion
//!     (`0x319e50`, crate::moby::blast::explode: no hit, the second flash 1 past 9, size 0.75, 10 embers, 3 billows,
//!     5 flares) at (13, 0, 14) in its scaled frame (`0x31e828` = L00 `0x326190`); its glow killed (`0x329078` = L00
//!     `0x330f40`); its stub 0x10f2 in its place (`0x31fe60` = L00 `0x327898`, update distance 0); itself without
//!     collision, not drawn, hidden; the smoke's wait 100; 3.
//!   * **3**: the smoke. While the wait is past 80, 4 puffs of kind 1 a tick (speed 0.05, size 1, fall 5) moving half
//!     that; each time it runs out two bursts of 3..6 puffs of kinds 1 and 0 (speed 0.05, size 0.5, fall 15) moving a
//!     third of it (its z a ninth), and the wait 30..80 again.

use crate::hero::physics::V3;
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::props::spawn_in_place;
use crate::moby::{countdown, shards, Ctx, MobyWorld};

pub const LAMP: i16 = 0x10f1;
const STUB: i16 = 0x10f2;

const GLOW: usize = 4;
const SMOKE: usize = 8;

/// The glow's and the puffs' point and the puffs' velocity in its frame.
const AT: V3 = [13.0, 0.0, 15.0];
const VEL: V3 = [f32::from_bits(0x3e2a_aaab), 0.0, f32::from_bits(0x3dee_eef0)];
/// `$gp+0x2ba0` / `+0x2b98` / `+0x2b90`.
const RANGE: [f32; 8] = [2.0, -3.0, 1.0, 0.0, 5.0, 3.0, 7.0, 0.0];
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);
/// The puffs' speed after the break (`0x3d4cccce`).
const SMOKE_SPEED: f32 = f32::from_bits(0x3d4c_ccce);

/// `0x2e61a8` on moby `id` (crate::particles::staged_burst) at [`AT`].
#[allow(clippy::too_many_arguments)]
fn puffs(w: &MobyWorld, id: usize, ctx: &mut Ctx, spread: f32, speed: f32, size: f32, fall: f32, vel: V3, count: i32, kind: u8) {
    let m = &w.mobys[id];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::staged_burst(sys, &mut ctx.game.rng, spread, speed, size, fall, 1.0, &m.rows, m.pos, m.scale, AT, vel, count, kind);
}

/// 0x10f1 (module docs).
pub fn update_10f1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let m = &w.mobys[id];
            let s = m.scale;
            let (offset, pos) = (AT.map(|x| x * s), m.pos);
            let g = ctx.parts.as_deref_mut().and_then(|sys| crate::particles::glow(sys, &mut ctx.game.rng, s * 3_780_000.0, 0.75, offset, id, pos, 0x60, false, 0x2820_2080, 2));
            w.mobys[id].set_pv_i(GLOW, g.map_or(0, |i| i as i32 + 1));
            let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0);
            if hit && w.mobys[id].pv_i(0) == 0 { w.mobys[id].set_state(2, -1); }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            puffs(w, id, ctx, 0.2, f32::from_bits(0x3dee_eef0), 1.0, 5.0, VEL, 0x28, 1);
            let at = crate::moby::ai::frame_point(w, id, [13.0, 0.0, 14.0]);
            let e = Explosion { flash2: 1.0, near: 9.0, size: 0.75, embers: 10, billows: 3, flares: 5, sound: -1, lod: -1, flags: 0x0081_0001, ..Default::default() };
            blast::explode(w, ctx, Some(id), Some(at), &e);
            let g = w.mobys[id].pv_i(GLOW);
            if let (Ok(i), Some(sys)) = (usize::try_from(g - 1), ctx.parts.as_deref_mut()) { sys.kill_part(i); }
            spawn_in_place(w, id, STUB, 0, &*ctx.host);
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.drawn = 0;
            m.mode |= 1;
            m.set_pv_i(SMOKE, 100);
            m.set_state(3, -1);
        }
        3 => {
            let mut vel = VEL;
            if 80 < w.mobys[id].pv_i(SMOKE) {
                vel = vel.map(|x| x * 0.5);
                puffs(w, id, ctx, 0.0, SMOKE_SPEED, 1.0, 5.0, vel, 4, 1);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(SMOKE);
            let out = countdown(&mut t);
            m.set_pv_i(SMOKE, t);
            if !out { return; }
            let third = f32::from_bits(0x3eaa_aaab);
            vel = vel.map(|x| x * third);
            vel[2] *= third;
            let n = ctx.game.rng.rand_range(3, 6);
            puffs(w, id, ctx, 0.0, SMOKE_SPEED, 0.5, 15.0, vel, n, 1);
            let n = ctx.game.rng.rand_range(3, 6);
            puffs(w, id, ctx, 0.0, SMOKE_SPEED, 0.5, 15.0, vel, n, 0);
            let t = ctx.game.rng.rand_range(0x1e, 0x50);
            w.mobys[id].set_pv_i(SMOKE, t);
        }
        _ => {}
    }
}
