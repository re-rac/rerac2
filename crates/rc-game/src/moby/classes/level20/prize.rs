//! Yeedil's prize **0x12d3** ([`update_12d3`], `0x44ad98`, one placed; the decomp's `func_L20_00449850`): the bomb
//! the boss (crate::moby::classes::level20::boss) brings out and eats for its attack 11, which Ratchet can drag with
//! the Tractor Beam (crate::moby::classes::tractor, its drag `0x33b0e0` = L01 `0x31bd40`) into it. The boss puts it
//! out (`0x44af30`, in crate::moby::classes::level20::boss: its life `+0xa8` 6 seconds, the boss `+0xa4`, `+0xac` 0,
//! a target, half a unit up, its drag's velocity cleared, 2) and hides it when it eats it.
//!
//! Its pvars: `+0x20` 1.0 and the short `+0x24` 1 (no reader here), `+0x50` its drag block (flags `+0x70` | 6: no fall,
//! the quick pull; the hold distance `+0x84` 8 and the range `+0x88` 16, `$gp+0x33a0` / `+0x33a4`), `+0x60` its place
//! kept, `+0xa0` its fence path, `+0xa4` the boss, `+0xa8` its life, `+0xac` taken (the beam has held it).
//!
//! * In a scene (`0x1a8f00` = 2) and not in 1: put away ([`stow`], `0x44afc0`: not a target, 1, no collision, mode |
//!   0x41).
//! * **0**: the words above; put away; nothing more.
//! * **1** (put away): nothing.
//! * **2** (out): the drag (radius 0.5, its block `+0x50`, its fence `+0xa0`).
//! * **After** (out of 0 and 1): held by the beam (`0x18a3e0`) it is taken. It blows up ([`blow`], `0x44b030`) on its
//!   frame slice of 16 (the level's frame `$gp+0x3a58` against the moby's address `>> 8`, the port's slot index) more
//!   than 5 above or below the ground under it (`0x331ed0` = L00 `0x31ce20`, crate::moby::ai::ground_z), or taken with
//!   its life out (`0x302c30`, crate::moby::countdown); else [`keep`] (`0x44b0f8`).
//! * **The blast** ([`blow`]): the big explosion facing the camera (`0x421d88` = L00 `0x3fabe8`,
//!   crate::moby::classes::explosion::start, its height less 0.5 the ground), a sphere of 6 at it (`0x332230` = L00
//!   `0x31d180`, crate::moby::ai::attack_sphere: damage 100 `$gp+0x33ac`, push 2, flags 0x10000), put away.
//! * **Kept** ([`keep`]): taken, on its frame slice of 8, a boss 0x11e0 within 1.5 of it (`0x2f54b0` = L00
//!   `0x2e14b0`, itself left out) blows it up. Else it keeps 4 (`$gp+0x33b0`) from the boss's pivot (`0x446160`: 5 up
//!   its frame, crate::moby::classes::level20::boss's tilt pivot), its place kept (`+0x60`), and within its fence (0.5,
//!   `0x3596f0` = L00 `0x343678`, crate::moby::ai::fence).

use crate::hero::physics::{add, len, set_len, sub};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12d3;
const BOSS: i16 = super::boss::CLASS;

/// `$gp+0x33a0` / `+0x33a4` / `+0x33ac` / `+0x33b0`.
const HOLD: f32 = 8.0;
const RANGE: f32 = 16.0;
const DAMAGE: f32 = 100.0;
const KEEP_OFF: f32 = 4.0;

/// `0x44afc0`: put away.
fn stow(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.mode &= 0xefff;
    m.set_state(1, -1);
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & 0xefff;
}

/// `0x44b030`: the blast (module docs).
fn blow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let dir = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
    crate::moby::classes::explosion::start(w, ctx, pos[2] - 0.5, pos, dir);
    ai::attack_sphere(w, id, ctx, 6.0, DAMAGE, 2.0, pos, 0x1_0000, 0, 1, 0);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & 0xefff;
    m.set_state(1, -1);
}

/// `0x44b0f8`: kept (module docs).
fn keep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(0xac) != 0 && (ctx.game.frame & 7) as usize == id & 7 {
        let pos = w.mobys[id].pos;
        let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, 1.5, crate::collision::QueryFlags(0), Some(id))).unwrap_or_default();
        if near.iter().any(|&k| w.mobys.get(k).is_some_and(|o| o.o_class == BOSS)) {
            blow(w, id, ctx);
            return;
        }
    }
    let Some(boss) = ai::link(&w.mobys[id], 0xa4).filter(|&b| b < w.mobys.len()) else { return };
    let b = &w.mobys[boss];
    let pivot: [f32; 3] = std::array::from_fn(|k| b.pos[k] + 5.0 * b.rows[2][k]);
    let off = sub(w.mobys[id].pos, pivot);
    if len(off) < KEEP_OFF {
        let p = add(set_len(off, KEEP_OFF), pivot);
        let m = &mut w.mobys[id];
        m.pos = p;
        m.set_pv_v(0x60, p);
        let fence = usize::try_from(m.pv_i(0xa0)).ok().and_then(|i| ctx.level.paths.get(i));
        if let Some(q) = fence.and_then(|pts| ai::fence(pts, 0.5, p)) { w.mobys[id].pos = q; }
    }
}

/// 0x12d3 (module docs).
pub fn update_12d3(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE && w.mobys[id].state != 1 { stow(w, id); }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, 1.0);
            m.set_pv_s(0x24, 1);
            m.set_pv_i(0x70, m.pv_i(0x70) | 6);
            m.set_pv_f(0x84, HOLD);
            m.set_pv_f(0x88, RANGE);
            stow(w, id);
            return;
        }
        1 => return,
        2 => {
            let path = w.mobys[id].pv_i(0xa0);
            crate::moby::classes::tractor::drag(w, id, ctx, 0.5, path);
        }
        _ => {}
    }
    if crate::moby::classes::tractor::held(w) == Some(id) { w.mobys[id].set_pv_i(0xac, 1); }
    let p = w.mobys[id].pos;
    let far = (ctx.game.frame & 0xf) as usize == id & 0xf && 5.0 < (p[2] - ai::ground_z(ctx, 0.5, p, 0)).abs();
    let spent = w.mobys[id].pv_i(0xac) != 0 && {
        let mut t = w.mobys[id].pv_i(0xa8);
        let out = countdown(&mut t);
        w.mobys[id].set_pv_i(0xa8, t);
        out
    };
    if far || spent { blow(w, id, ctx) } else { keep(w, id, ctx) }
}
