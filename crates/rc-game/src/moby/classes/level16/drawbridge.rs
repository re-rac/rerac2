//! Snivelak's swinging bridge **0x125d** ([`update_125d`], `0x420020`, 4 placed; the decomp's `func_L16_0041EBE8`):
//! hooked, it swings its group's mobys about a pivot until they come to rest at 40°.
//!
//! Its swing is the level's ([`Globals`]: the angle `$gp+0x2f74`, its speed `0x1b3a2c`), shared by every 0x125d.
//! * **0**: hooked (`0x322cc8` = L00 `0x32a838`, crate::moby::Moby::once bit 2): the angle and the speed 0; 1.
//! * **1**: the speed 20″ a tick more (`$gp+0x2f7c`, × 4.828e-6), the angle by it; at 40° (`$gp+0x2f78`) or past it
//!   the angle held there and the speed bounced back a third (`$gp+0x2f80`, −0.333, into `$gp+0x4a3c` = the speed);
//!   below 1′ a tick (`0x2e8158` the absolute value) it rests (2). Then the group swung ([`swing`], `0x420130`).

use crate::hero::physics::{add, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const BRIDGE: i16 = 0x125d;

/// The swing every 0x125d shares (crate::moby::MobyWorld::drawbridge).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x2f74`: the angle (radians).
    pub angle: f32,
    /// `0x1b3a2c` (`$gp+0x4a3c`): its speed a tick.
    pub speed: f32,
}

/// `$gp+0x2f90`: the pivot in its frame.
const PIVOT: V3 = [-6.25, 0.0, 0.0];

fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// 0x125d (module docs).
pub fn update_125d(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if w.mobys[id].once & 4 != 0 {
                w.drawbridge = Globals::default();
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            let g = &mut w.drawbridge;
            g.speed += 20.0 * f32::from_bits(0x36a2_ad2f);
            g.angle += g.speed;
            let most = 40.0 * f32::from_bits(0x3c8e_fa36);
            let mut rest = false;
            if most <= g.angle {
                g.speed *= f32::from_bits(0xbeaa_7efa);
                g.angle = most;
                rest = g.speed.abs() < f32::from_bits(0x3998_825c);
            }
            if rest { w.mobys[id].set_state(2, -1); }
            let a = w.drawbridge.angle;
            swing(w, id, a);
        }
        _ => {}
    }
}

/// `0x420130(angle, m)`: each moby of its group (`+0x21`) turned about the pivot ([`PIVOT`] through its rows, at its
/// place): its pitch (`+0xf4`) the angle (0x125d's own the angle's negative), its rows rebuilt (`0x2e8a68` = L00
/// `0x2ef6b8`), its place moved so the pivot keeps its place in its frame (`0x31e8e0` = L00 `0x326248`: into the
/// old frame, back out through the new).
pub fn swing(w: &mut MobyWorld, id: usize, angle: f32) {
    let m = &w.mobys[id];
    let pivot = add(through(PIVOT, &m.rows), m.pos);
    let Some(list) = usize::try_from(m.group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for k in list {
        let Some(e) = w.mobys.get_mut(k) else { break };
        let d = sub(pivot, e.pos);
        let local = [0, 1, 2].map(|i| e.rows[i][0] * d[0] + e.rows[i][1] * d[1] + e.rows[i][2] * d[2]);
        e.rot[1] = if e.o_class == BRIDGE { -angle } else { angle };
        e.rows = crate::moby::attach::rows_of(e.rot);
        let moved = sub(through(local, &e.rows), d);
        e.pos = sub(e.pos, moved);
    }
}
