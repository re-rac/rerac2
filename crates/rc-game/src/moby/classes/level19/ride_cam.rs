//! Grelbin's vehicle camera **0x1258** ([`update_1258`], L19 `0x423dc8`; the decomp's `func_L19_00422A98`): while
//! Ratchet rides (his state 0x31 with `0x18b650` set, crate::hero::vehicle) it keeps a camera tweak (crate::camera::
//! CamTweaks, priority 5, `0x2d4da8`) looking at 10 ahead of the vehicle along its yaw and half its pitch reversed
//! (`0x322b18`): for the vehicle 0x0905 the distance 7 (`$gp+0x2e44..`), the pitch and look height left alone;
//! for the others (the hovership 0x0eef, crate::moby::classes::hovership) the distance 6, the pitch 0.1
//! and the look height 0; the face-after-rest flag on, the way it faces the point less the camera's place. Off the
//! vehicle the tweak is let go (`0x2d4e48`).
//!
//! **0**: the tweak none, update distance 0xff; 1. **1**: as above (the tweak's slot kept in its `+0x10`, the port's:
//! the original keeps it in `$gp+0x2e5c`).
//!
//! Not ported: its cuboid (`+0`): with the camera inside it, `$gp+0x351c` = 2 (0 again once out while it was 2), the
//! first-person camera's veto (crate::first_person has none).

use crate::hero::physics::{add, sub};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1258;
/// The other vehicle.
const VEHICLE_0905: i16 = 0x0905;
/// `$gp+0x2e44..+0x2e58`: the look's reach; 0x0905's distance; the others' distance, pitch and look height.
const AHEAD: f32 = 10.0;
const DIST_0905: f32 = 7.0;
const DIST: f32 = 6.0;
const PITCH: f32 = f32::from_bits(0x3dcc_cccd);
const LOOK_H: f32 = 0.0;
/// The port's slot word (module docs).
const SLOT: usize = 0x10;

pub fn update_1258(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(SLOT, 0);
            m.update_dist = 0xff;
            m.set_state(1, -1);
        }
        1 => {
            let slot = usize::try_from(w.mobys[id].pv_i(SLOT) - 1).ok();
            let v = ctx.hero.riding().and_then(|v| w.mobys.get(v)).map(|v| (v.pos, v.rot, v.o_class));
            let Some((pos, rot, class)) = v else {
                if let Some(s) = slot { ctx.game.cam_tweaks.release(s); }
                w.mobys[id].set_pv_i(SLOT, 0);
                return;
            };
            let at = add(crate::moby::classes::shot::polar(AHEAD, rot[2], -rot[1] * 0.5), pos);
            let slot = match slot {
                Some(s) => s,
                None => {
                    let Some(s) = ctx.game.cam_tweaks.request(5) else { return };
                    w.mobys[id].set_pv_i(SLOT, s as i32 + 1);
                    s
                }
            };
            let cam = ctx.host.camera_pos();
            let t = &mut ctx.game.cam_tweaks.slots[slot];
            if class == VEHICLE_0905 {
                (t.dist, t.look_h, t.pitch) = (DIST_0905, -1.0, -1.0);
            } else {
                (t.dist, t.look_h, t.pitch) = (DIST, LOOK_H, PITCH);
            }
            t.face_rest = true;
            t.dir = sub(at, cam);
        }
        _ => {}
    }
}
