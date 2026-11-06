//! 0x0e0b, Dobbo's orbit's opponent (L22 `0x39eb38`, asm only in the decomp; its set-up and frame `0x3a2a00`, the
//! decomp's `func_US2_L22_003A2A00`; one placed). Ported: what the fight's other systems need of it — the set-up, the
//! globals it keeps and the pickups it runs (crate::moby::classes::level22::pickups).
//!
//! * **The set-up** (`0x3a2a00` in state 0): its health from its pvar `+0x326` (3600) — × 0.9 past 90000 ticks of
//!   fighting (`$gp+0x2a8`, counted while no intro runs; `$gp+0x2ac`), × 0.6 past 126000 (`$gp+0x2b0`) — kept as its
//!   most `0x1b2a28`, `+0x20` (a float) and `+0x24`; it is the level's opponent (`$gp+0x2d4`, found by class in the
//!   moby table); Giant Clank's health `+0x324` (1200); the planetoid's radius `0x1b2a24` its distance from the
//!   sphere's centre (`0x189ed0`); mode | 0x4100, update and draw distance 0xff, speed 1; the pickups set up
//!   (`0x3a4ad0`).
//! * **Each tick** (`0x39eca0` on): its scale the class's × 1.45 (`$gp+0x35c`), it stands up on the sphere (`+0x200`
//!   the down); the fight's ticks counted; set up, the damages kept from its pvars (`0x1b2a14..0x1b2a20` = `+0x330`
//!   the shots' 4, `+0x334` 6, `+0x338` the special's 48, `+0x33c` 0.35); the pickups' tick (`0x3a5180`).
//!
//! Not ported: its wait in state 0x27 while a scene trigger 0x0f6e is out; the intro (`$gp+0x2c4` counted from 1:
//! Giant Clank held in a landing lag of 10 and his gun put away until ✕ or △ — `0x138324 & 0x50` — starts the
//! fight in state 5 with a lag of 20; past 200 ticks `0x3a58c0`): the fight starts at once here (state 5); Giant
//! Clank put at the body 0x027d with the angles (0.00161, 0.29459, 1.57105) (the spawn stands him there); the seed
//! from the clock; its AI (its states, movement on the paths `0x3a3db8` / `0x3a3bb0` / `0x3a3fc0`, its shots and
//! specials, its hits `0x3231c0` and health, its tuning words `+0x70..+0x4ec`, the scan `0x3a5d68`), its parts
//! 0x0e0a / 0x0e0c / 0x0f8f, its reaction to Giant Clank's death (state 2), the difficulty words `0x1b2a8c` /
//! `0x1b2a90`, the help mobys' flags (0x133a mode | 0x41).

use super::pickups;
use crate::moby::frames::upright;
use crate::moby::{Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x0e0b;

/// `$gp+0x35c`: its scale over the class's.
const SCALE: f32 = f32::from_bits(0x3fb9_999a);

/// 0x0e0b (`0x39eb38`, module docs).
pub fn update_0e0b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { set_up(w, id, ctx); }
    let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    let down = super::planet::down_at(ctx, w.mobys[id].pos);
    let m = &mut w.mobys[id];
    m.scale = class_scale * SCALE;
    m.rows = turn_rows(&m.rows, upright(&m.rows, crate::hero::physics::scale(down, -1.0)));
    let damage = [m.pv_f(0x330), m.pv_f(0x334), m.pv_f(0x338), m.pv_f(0x33c)];
    let g = &mut w.level22;
    g.fight_ticks += 1;
    g.damage = damage;
    pickups::tick(w, id, ctx);
}

/// `0x3a2a00`'s set-up (module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ticks = w.level22.fight_ticks;
    let m = &mut w.mobys[id];
    let full = m.pv_s(0x326);
    let most = if 126_000 < ticks {
        (full as f32 * f32::from_bits(0x3f19_999a)) as i32
    } else if 90_000 < ticks {
        (full as f32 * f32::from_bits(0x3f66_6666)) as i32
    } else {
        full as i32
    };
    ctx.hero.giant.hp = m.pv_s(0x324) as i32;
    m.mode |= 0x4100;
    (m.update_dist, m.draw_dist) = (0xff, 0xff);
    ctx.host.set_anim_speed(m, 1.0);
    m.set_pv_f(0x20, most as f32);
    m.set_pv_s(0x24, most as f32 as i32 as i16);
    let pos = m.pos;
    let g = &mut w.level22;
    g.boss = Some(id);
    g.most = most;
    g.radius = crate::hero::physics::dist(pos, ctx.hero.gravity.centre);
    pickups::set_up(w, id, ctx);
    // The intro, not ported, ends at once (`0x31b8d0(m, 5, 0)`).
    w.mobys[id].set_state(5, 0);
}
