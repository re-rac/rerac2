//! Level 7's **0x0c29** (`0x379b08`, 1 placed; the decomp's `func_L07_00378898`): up to three tie groups turning
//! about z on the ground chunk.
//!
//! Its pvars: `[0..3]` the tie groups (−1 none), `[3..6]` their yaws. **0**: updated from any distance (`+0x30` 0xff); 1. **1**: with chunk 0 in
//! place (`0x1c9700` = 0, crate::game::GameState::variant), each group's yaw on by its [`RATES`] (`0x2efe60` =
//! crate::hero::physics::add_rot) and each of its ties' rows made the yaw's (`0x2e5778`) with the first two
//! scaled by its first row's length (`0x2e5060`, `0x2e51f8`; the third is left unit), its place kept, the tie updated
//! (`0x361f80`, crate::moby::MobyWorld::moved_ties).

use crate::hero::physics::{add_rot, len};
use crate::moby::attach::rows_of;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c29;

/// `$gp−0x440`: the three groups' turns, degrees a second.
const RATES: [f32; 3] = [5.0, -10.0, 15.0];

/// `0x379b08` (module docs).
pub fn update_0c29(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            w.mobys[id].set_state(1, -1);
        }
        1 if ctx.game.variant == 0 => {
            for (i, rate) in RATES.iter().enumerate() {
                let g = w.mobys[id].pv_i(4 * i);
                if g == -1 { continue; }
                let m = &mut w.mobys[id];
                let yaw = add_rot(m.pv_f(0xc + 4 * i), rate * 0.000_290_888_25);
                m.set_pv_f(0xc + 4 * i, yaw);
                let Some(list) = usize::try_from(g).ok().and_then(|g| w.tie_groups.get(g)).cloned() else { continue };
                let rows = rows_of([0.0, 0.0, yaw]);
                for t in list {
                    let t = t as usize;
                    let Some(mat) = w.ties.get_mut(t) else { continue };
                    let s = len([mat[0][0], mat[0][1], mat[0][2]]);
                    for (r, row) in rows.iter().enumerate() {
                        let k = if r < 2 { s } else { 1.0 };
                        for c in 0..3 { mat[r][c] = row[c] * k; }
                    }
                    if !w.moved_ties.contains(&t) { w.moved_ties.push(t); }
                }
            }
        }
        _ => {}
    }
}
