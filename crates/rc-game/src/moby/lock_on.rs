//! The Lock On mod (Slim Cognito's slot 2, crate::vendor::mods: the vendor's "on" bit 5 of `0x1a7a88[item]`): the
//! lock `0x18b270` the weapons aim at while L2 and R2 are held.
//!
//! **The lock's update** (`0x2cb2e0`, at the head of the hero's transitions `0x2cb650`; the port runs it at the head
//! of the item pass, after the hero's tick): no hand item (`0x18b068` < 0), the item's mod off, or Ratchet in the
//! state 0x31 — no lock. Without one, L2 and R2 held (`0x138320 & 3`) with one of them pressed this tick
//! ([`search`]) picks the moby nearest the view's yaw (`0x1b8d58`) within 25, 35° across and 70° up or down, else
//! nearest Ratchet's facing. Held, it is dropped beyond 30 of Ratchet (`0x189ea0`), when L2 or R2 lets go, or when
//! it leaves the target list (mode 0x1000).
//!
//! **Its readers.** The aim search's wrapper `0x325f90` (crate::moby::aim::search_lock) and the weapons that test it
//! themselves ([`live`]: not deleted (states 0xfe / 0xfd) with health left; the Chopper's blades only [`held`]) take it
//! before any search; the weapons' updates copy it into their aim of the tick `0x18aa18` ([`mirror`]) — with a lock held the posture's aim turn
//! (`0x2abc18`) leaves Ratchet's spine alone and his head looks about as when nothing is aimed at.
//!
//! Not ported: the kill count of a lock that leaves the target list (`0x18c900` / `0x18c904` / `0x18c908`, the
//! weapons' statistics), the gravity frames' search (`0x2ae668`, with `0x18c0e3` set), the update's skip on a tick
//! the hero takes a hit (`0x2b2dd0`), the lock's reticle and health bar (`0x2dae30`, sprites 0x33..0x35), the
//! camera's turn to the lock (`0x4239d0` at `0x4240c4`: its counter D+0x288 to 400, `0x426468`), the Comet-Strike's
//! aim at it (`0x41fe58`, its own search unported) and the aim at it of the lobbing weapons (`0x32b4d0`: the Lava Gun,
//! the Meteor Gun, the Plasma Coil, the Hoverbomb Gun — they only mirror it).

use super::{aim, mode, Ctx, MobyWorld};
use crate::hero::physics::{atan2, dist};

/// The vendor's "on" bit of slot 2.
pub const LOCK_ON: u8 = 1 << 5;
/// The search's reach and cones (`0x41c80000`, `0x3f1c61ab`, `0x3f9c61ab`).
const RANGE: f32 = 25.0;
const YAW_CONE: f32 = f32::from_bits(0x3f1c_61ab);
const PITCH_CONE: f32 = f32::from_bits(0x3f9c_61ab);
/// The lock's leash from Ratchet.
const LEASH: f32 = 30.0;

/// `0x2cb2e0` (module docs).
pub fn update(w: &MobyWorld, ctx: &mut Ctx) {
    let it = ctx.hero.items;
    let on = usize::try_from(it.item).ok().and_then(|i| ctx.game.mods_bought.get(i)).is_some_and(|&b| b & LOCK_ON != 0);
    if !on || ctx.hero.state == 0x31 {
        ctx.hero.items.lock = None;
        return;
    }
    let both = it.held & 3 == 3;
    match it.lock {
        None => {
            if both && it.pressed & 3 != 0 {
                let f = ctx.host.camera_rows()[0];
                let view_yaw = atan2(f[0], f[1]);
                let lock = search(w, ctx, view_yaw).or_else(|| search(w, ctx, ctx.hero.rot[2]));
                ctx.hero.items.lock = lock;
            }
        }
        Some(l) => {
            let keep = w.mobys.get(l).is_some_and(|m| dist(ctx.hero.pos, m.pos) <= LEASH && both && m.mode & mode::TARGET != 0);
            if !keep { ctx.hero.items.lock = None; }
        }
    }
}

/// `0x2ae900(25, yaw, 35°, 70°, .., class type 5, line)`: the lock's search (aim::hero_search, the line tested).
fn search(w: &MobyWorld, ctx: &Ctx, yaw: f32) -> Option<usize> { aim::hero_search(w, ctx, RANGE, yaw, YAW_CONE, PITCH_CONE, true) }

/// The lock while it is not deleted (states 0xfe / 0xfd): the Chopper's blades' test.
pub fn held(w: &MobyWorld, ctx: &Ctx) -> Option<usize> {
    ctx.hero.items.lock.filter(|&l| w.mobys.get(l).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}

/// The lock when a weapon may take it: [`held`] and with health left (`0x31fa20`).
pub fn live(w: &MobyWorld, ctx: &Ctx) -> Option<usize> {
    let l = held(w, ctx)?;
    aim::health(w, l).filter(|&(hp, _)| 0.0 < hp).map(|_| l)
}

/// The weapons' `if (0x18b270) 0x18aa18 = 0x18b270`: the lock as the aim of the tick (its aim height with it).
pub fn mirror(w: &MobyWorld, ctx: &mut Ctx) {
    if let Some(l) = ctx.hero.items.lock.filter(|&l| w.mobys.get(l).is_some()) {
        ctx.hero.items.aim_target = Some((l, aim::health(w, l).map_or(0.0, |(_, a)| a)));
    }
}
