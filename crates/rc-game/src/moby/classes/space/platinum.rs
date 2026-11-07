//! The space levels' platinum bolt, class 0x126c (Feltzin L05 `0x42db38`, Hrugis L10 `0x42ef30`, Gorn L15 `0x438bb8`:
//! one code, the decomp's `func_US2_L05_0042DB38`): bolt `[0]` of the level (the save's byte `0x19b278 + 4·level + k`,
//! crate::game::GameState::platinum), taken by the ship. None is placed: the gate race makes bolt 0 at its last gate
//! when no gate was missed ([`make`], `0x42e6b0`; [`super::mission`]).
//!
//! * **[`make`]** (`0x42e6b0(pos, k, a, b)`, the decomp's `func_L05_0042C928`): none for k ≥ 4 or found; else at
//!   `pos`, its update and draw distances 0x40, drawn, `[0]` k, `[4]` / `[8]` a / b, its home `[0x10]` where it is
//!   (`[0x1c]` its yaw), its angles at random, the turns a tick in `[0x60..0x68]`, state 1, its ambient 0x82 grey
//!   (`0x32b6f0`); the ship found (`$gp+0x2730`: the last 0xfbf of the moby list).
//! * **0** (as a placed one would start): found already (or no number) it goes; else its home, yaw, random angles and
//!   turns as above, its draw distance at least 0x40, 1.
//! * **1**: its scale the class's (× `$gp+0x26e8` = 1); it bobs 0.3 about its home's height (`$gp+0x26ec` = 0 above
//!   it, `+0x26f0`: the sine of its fourth angle), turning 120°, 40° and 20° a second about x, y and z and its bob
//!   90° (`$gp+0x26f8..+0x2704`); a sparkle 1 tick in 40 (`0x333d40(40)`; the sparkle `0x42e578` is not ported).
//!   The ship within 1.5 takes it: `$gp+0x3100` = 180, the message 0x124b, found, the level's count `$gp+0x272c`,
//!   the autosave (`0x325a28(0, −1)`, crate::game::GameState::autosave; all through
//!   [`crate::moby::classes::platinum`]'s found), its sound 1, deleted.
//!
//! Hrugis's and Gorn's numbers are Feltzin's (`$gp+0x26b4..` / `+0x27c0..`).
//!
//! Not ported: its four glow rings (`0x42e018` set-up, `0x42e0f0` each tick: the L01 bolt's `0x397950` /
//! `0x397a28`, not ported there either) and sparkles (`0x42e578`), the light word `+0x38` copied from the hero's
//! moby, the word `$gp+0x3100` (read by the engine at `0x312f80`) and the count `$gp+0x272c`; its state 2 (the L01
//! bolt's show on foot, `0x42dd98`: nothing sets it on these levels; its own autosave there).

use crate::hero::physics::{add_rot, dist, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x126c;

/// `$gp+0x26f0`: the bob.
const BOB: f32 = 0.3;
/// `$gp+0x26f8..+0x2704`: the turns in degrees a second (x, y, z, the bob's phase).
const TURN: [f32; 4] = [120.0, 40.0, 20.0, 90.0];
const DEG_S: f32 = 0.000_290_888_25;
/// The ship's reach.
const REACH: f32 = 1.5;

/// The home, yaw, angles and turns (state 0 and [`make`]).
fn set_home(m: &mut crate::moby::Moby, rng: &mut crate::rng::Rng) {
    let (pos, yaw) = (m.pos, m.rot[2]);
    m.set_pv_v(0x10, pos);
    m.set_pv_f(0x1c, yaw);
    let a: [f32; 4] = std::array::from_fn(|_| rng.rand_angle());
    m.rot = [a[0], a[1], a[2]];
    // Its fourth angle (`+0xfc`, the bob's phase) kept at `[0x7c]`, as the L01 bolt's.
    m.set_pv_f(0x7c, a[3]);
    m.set_pv_f(0x68, TURN[0] * DEG_S);
    m.set_pv_f(0x64, TURN[1] * DEG_S);
    m.set_pv_f(0x60, TURN[2] * DEG_S);
}

fn new(ctx: &Ctx, k: i32) -> bool { k != -1 && super::super::platinum::slot(ctx, k).is_some_and(|i| ctx.game.platinum[i] == 0) }

/// `0x42e6b0(pos, k, a, b)`: bolt k made at `pos` (module docs); its moby.
pub fn make(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, k: i32, a: i32, b: i32) -> Option<usize> {
    if !(0..4).contains(&k) || !new(ctx, k) || w.class(CLASS).is_none() { return None; }
    let id = w.create(CLASS);
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.update_dist = 0x40;
    m.draw_dist = 0x40;
    m.drawn = 1;
    m.set_pv_i(0, k);
    set_home(m, &mut ctx.game.rng);
    m.set_pv_i(8, b);
    m.set_pv_i(4, a);
    m.state = 1;
    m.ambient = [0x82; 3];
    Some(id)
}

/// `0x42db38`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if !new(ctx, w.mobys[id].pv_i(0)) { return w.delete(id); }
            let m = &mut w.mobys[id];
            set_home(m, &mut ctx.game.rng);
            if m.draw_dist < 0x40 { m.draw_dist = 0x40; }
            m.state = 1;
        }
        1 => {
            let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.scale = class_scale;
            let phase = m.pv_f(0x7c);
            m.pos[2] = m.pv_f(0x18) + BOB * phase.sin();
            for (r, t) in m.rot.iter_mut().zip(TURN) { *r = add_rot(*r, t * DEG_S); }
            m.set_pv_f(0x7c, add_rot(phase, TURN[3] * DEG_S));
            // `0x333d40(40)`: the sparkle's chance (the sparkle not ported).
            let _ = ctx.game.rng.randi(40);
            let me = w.mobys[id].pos;
            let ship = w.mobys.iter().rposition(|s| s.o_class == crate::ship::SHIP_CLASS && s.state < 0x80);
            if !ship.is_some_and(|s| dist(w.mobys[s].pos, me) < REACH) { return; }
            super::super::platinum::found(w, id, ctx);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            w.delete(id);
        }
        _ => {}
    }
}
