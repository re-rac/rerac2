//! The splash moby **0x0307** (L00 `0x39f938` makes it, `func_00312D58`; its update [`update`], L00 `0x39f9e0`, the
//! decomp's `func_L00_0039E5B0`: both on every level but level 1, whose table lacks the update): a ring on the
//! water that fades as it grows.
//!
//! Its maker's callers the port has: level 6's falling pole (crate::moby::classes::level06::pole). Not ported: the
//! others (the fall into water `0x3a2c20` of crate::moby::knock, the Mini-Turret's in crate::moby::classes::turret):
//! their notes say no level's class table has 0x0307, so the make returns nothing there.

use crate::hero::physics::V3;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0307;

/// `0x39f938(pos, scale)` (L06 `0x3e25e0`): a splash at `pos` (the class missing: none) with Ratchet's colour words
/// (`0x2ab650`), ambient (0x28, 0x28, 0x46), `scale` times its class's scale, update distance 0xff, draw distance 0x40,
/// a random yaw; [`update`].
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, scale: f32) -> Option<usize> {
    w.class(CLASS)?;
    let k = w.create(CLASS);
    let light = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| h.light);
    let yaw = ctx.game.rng.rand_angle();
    let m = &mut w.mobys[k];
    if let Some(l) = light { m.light = l; }
    m.ambient = [0x28, 0x28, 0x46];
    m.update = Some(update);
    m.mode &= !crate::moby::mode::NO_UPDATE;
    m.update_dist = 0xff;
    m.draw_dist = 0x40;
    m.scale *= scale;
    m.pos = pos;
    m.rot[2] = yaw;
    w.rebuild(k, &*ctx.host);
    Some(k)
}

/// 0x0307 (L06 `0x3e2688`): the splash fades 3 a tick, growing 2.5%; under 4 gone.
pub fn update(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.alpha = m.alpha.wrapping_sub(3);
    m.scale *= 1.025;
    if m.alpha < 4 { w.delete(id); }
}
