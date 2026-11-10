//! The decoy-aware target search `func_L04_003429F8` (L04 `0x343c18` = L06 `0x362ca8` = L08 `0x330678` = L17
//! `0x323c88`): the copy levels 4, 6, 8 and 17 carry for their mobys that lob things at Ratchet (crate::moby::classes::
//! launcher's 0x0a58, crate::moby::classes::level17::mortar's 0x0c1a). Level 8's ground enemies have their
//! own, wider search (crate::moby::classes::level08::tabora_ai::find_target, `0x330a10`).
//!
//! **The target:** Ratchet, unless he is in the movement group 0x18 or the state 0x6f; or (with `n` areas) in one of
//! the first `n − 1` cuboids of the area list (so one area tests nothing); or (with no areas) outside the path's
//! polygon (`0x31e190` = L00 `0x31cf68`). Then the nearest decoy across (`0x2efe90` = L00 `0x2ef050`, the xy
//! distance) — a 0x00cb or 0x076c in its state 3 nearer than the reach (shrinking to each one found) that passes the
//! same area or polygon test — takes his place. The result: 0 Ratchet, 1 a decoy, 2 none (also returned).
//!
//! **The block** (`b`): `+0x00` the place, `+0x10` the angles (the moby's `+0xf0`), `+0x20` / `+0x30` the aim
//! (Ratchet: his aim point `0x189ef0` and his centre `0x189f00`, a unit over his feet; a decoy: 0.6 over it, twice),
//! `+0x40` the moby, `+0x44` the result. No target: the block cleared.
//!
//! Not ported: the decoys' list the search reads (`0x1b2630` and its count `$gp+0x3d80`: every moby of those classes
//! is looked at, as tabora_ai's copy does).

use crate::hero::physics::{add, dist2, V3};
use crate::moby::ai::set_link;
use crate::moby::{Ctx, MobyWorld};

/// The decoys the search takes.
const DECOYS: [i16; 2] = [0x00cb, 0x076c];

/// `0x343c18(m, block, areas, n, path's points, count, reach)` (module docs): `areas` the area list (its length the
/// `n`), `path` the polygon tested without areas (−1 none). Returns the result.
pub fn search(w: &mut MobyWorld, id: usize, ctx: &Ctx, b: usize, areas: &[i32], path: i32, reach: f32) -> i32 {
    let has_path = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).is_some_and(|p| !p.is_empty());
    // The area or polygon test the target and the decoys pass.
    let passes = |p: V3| {
        if !areas.is_empty() {
            !areas[..areas.len() - 1].iter().any(|&c| ctx.level.in_cuboid(c, p))
        } else {
            !has_path || ctx.level.in_polygon(path, p)
        }
    };
    let mut target = ctx.hero_moby;
    let mut result = 0;
    if ctx.hero.group == 0x18 || ctx.hero.state == 0x6f || !passes(ctx.hero.pos) {
        target = None;
        result = 2;
    }
    let here = w.mobys[id].pos;
    let mut reach = reach;
    for (k, o) in w.mobys.iter().enumerate() {
        if !DECOYS.contains(&o.o_class) || o.state != 3 { continue; }
        let d = dist2(here, o.pos);
        if d < reach && passes(o.pos) {
            result = 1;
            target = Some(k);
            reach = d;
        }
    }
    let m = &mut w.mobys[id];
    set_link(m, b + 0x40, target);
    match target {
        None => for k in 0..4 { m.set_pv_v(b + 0x10 * k, [0.0; 3]); },
        Some(t) if Some(t) == ctx.hero_moby => {
            m.set_pv_v(b, ctx.hero.pos);
            m.set_pv_v(b + 0x10, ctx.hero.rot);
            m.set_pv_v(b + 0x20, ctx.hero.aim_point);
            m.set_pv_v(b + 0x30, add(ctx.hero.pos, [0.0, 0.0, 1.0]));
        }
        Some(t) => {
            let (p, r) = (w.mobys[t].pos, w.mobys[t].rot);
            let m = &mut w.mobys[id];
            m.set_pv_v(b, p);
            m.set_pv_v(b + 0x10, r);
            let up = add(p, [0.0, 0.0, 0.6]);
            m.set_pv_v(b + 0x30, up);
            m.set_pv_v(b + 0x20, up);
        }
    }
    w.mobys[id].set_pv_i(b + 0x44, result);
    result
}
