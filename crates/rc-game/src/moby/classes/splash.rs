//! The splash moby **0x0307** (L00 `0x39f938` makes it, `func_00312D58`; its update [`update`], L00 `0x39f9e0`, the
//! decomp's `func_L00_0039E5B0`: both on every level but level 1, whose table lacks the update): a ring on the
//! water that fades as it grows.
//!
//! Its maker's callers (L00 places unless named): level 6's falling pole (L06 `0x466558`,
//! crate::moby::classes::level06::pole), the knock's fall into water (L06 `0x35fde0` = L08 `0x32d388` = L17 `0x320fc8`,
//! crate::moby::knock), the Mini-Turret's landing in water (`0x387ee8` / `0x38a9a8`, L03 / L04 / L14 `0x38d190` /
//! `0x3ac9d8` / `0x393708`, crate::moby::classes::turret). Not ported: the hero's big splash (`0x2aac58`, every level:
//! crate::hero's `splash` has no moby world), the fall into water of the classes 0x0079 (`0x38eb50`, each level its own
//! copy; made by 0x00c0's update `0x399290`) and 0x081f (`0x3afd40`), the hoverbike race's sprays (L04 `0x40d260` = L11
//! `0x3fcf08`), level 1's own (`0x3a7428`, `0x3b5608`, `0x3cd130`, `0x3da060`, `0x3dd160`). Every level's class list has
//! the class (with its model); every update table but level 1's has its update.

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
