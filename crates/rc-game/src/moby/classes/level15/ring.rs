//! Gorn's floating rings 0x1279 (L15 `0x43ae60`, 10 placed; the decomp's `func_L15_004390A0`): each carries the tie
//! group nearest it, its moby group (`+4`) and shrub group (`+8`) up and down (crate::moby::attach), the group's ties
//! of class 0x0d34 and 0x0d33 turning in it.
//!
//! * **State 0**: the tie group with the tie nearest it (`0x1a9014`, the tie's place); none, it waits. Its bob (`0x2aee70`
//!   = L00 `0x324430`, crate::moby::classes::props's bob): 12 high (`+0x28`) at 30..37° a second (`+0x2c`) from a
//!   random phase (`+0x1c`; the last offset `+0xc` its sine × 12), relative to its place; updated from anywhere; the
//!   carrier takes up to 20 members (`0x33ccd8` = L00 `0x3246c0`: the moby group, that tie group, the shrub group)
//!   — none, it is gone (`0x29f338`). The group's ties of class 0x0d34 take the angles `+0x680` as their own turn,
//!   those of 0x0d33 `+0x6a0` (`0x33d6d8` = L00 `0x3250c0`); the carrier's frame its rows (`+0x8d`), those from its
//!   angles, kept (mode |= 0x104); the two turns' steps 27..52 and 25..37° a second of random signs (`0x2a78d8`),
//!   opposite. → 1.
//! * **State 1**: the bob, each turn's z by its step (`+0x688` by `+0x38`, `+0x6a8` by `+0x3c`), the members put
//!   back (`0x33d1e8` = L00 `0x324bd0`).

use crate::moby::attach::{rows_of, Kind};
use crate::moby::{mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1279;

const TIES: usize = 0x0;
const MOBYS: usize = 0x4;
const SHRUBS: usize = 0x8;
const LAST: usize = 0xc;
const PHASE: usize = 0x1c;
const AMP: usize = 0x28;
const SPEED: usize = 0x2c;
const STEP_A: usize = 0x38;
const STEP_B: usize = 0x3c;
const TURN_A: usize = 0x680;
const TURN_B: usize = 0x6a0;
/// Degrees a second to radians a tick.
const K: f32 = 0.000_290_888_245;

fn bob(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (amp, step) = (m.pv_f(AMP), m.pv_f(SPEED) * K);
    super::super::props::bob(m, amp, step, -1.0, PHASE, LAST);
}

/// The members' turns into the carrier's block, by their ties' classes.
fn turns(w: &mut MobyWorld, id: usize) {
    let (a, b) = (w.mobys[id].pv_v(TURN_A), w.mobys[id].pv_v(TURN_B));
    let ties: Vec<usize> = w.attached.get(&id).map_or(Vec::new(), |c| c.members.iter().filter(|m| m.kind == Kind::Tie).map(|m| m.index).collect());
    for t in ties {
        match w.tie_classes.get(t).copied() {
            Some(0x0d34) => w.attach_member_turn(id, Kind::Tie, t, a),
            Some(0x0d33) => w.attach_member_turn(id, Kind::Tie, t, b),
            _ => {}
        }
    }
}

/// `0x43ae60`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let at = w.mobys[id].pos;
            let mut best: Option<(usize, f32)> = None;
            for (g, list) in w.tie_groups.iter().enumerate() {
                for &t in list {
                    let Some(m) = w.ties.get(t as usize) else { break };
                    let d = crate::hero::physics::dist([m[3][0], m[3][1], m[3][2]], at);
                    if best.is_none_or(|(_, b)| d < b) && d < 9999.0 { best = Some((g, d)); }
                }
            }
            let Some((group, _)) = best else { return };
            let g = &mut ctx.game.rng;
            let (speed, phase) = (g.randf(30.0, 37.0), g.rand_angle());
            {
                let m = &mut w.mobys[id];
                m.set_pv_i(TIES, group as i32);
                m.set_pv_f(AMP, 12.0);
                m.set_pv_f(SPEED, speed);
                m.set_pv_f(PHASE, phase);
                m.set_pv_f(LAST, phase.sin() * 12.0);
            }
            bob(w, id);
            w.mobys[id].update_dist = 0xff;
            let (mobys, shrubs) = (w.mobys[id].pv_i(MOBYS), w.mobys[id].pv_i(SHRUBS));
            w.attach(id, 20, mobys, group as i32, shrubs);
            if !w.attached.contains_key(&id) { return w.delete(id); }
            turns(w, id);
            w.attach_use_rows(id);
            let g = &mut ctx.game.rng;
            let a = g.randf_signed(27.0, 52.0) * K;
            let mut b = g.randf_signed(25.0, 37.0) * K;
            if 0.0 <= a * b { b = -b; }
            let m = &mut w.mobys[id];
            m.mode |= mode::KEEP_ROWS | mode::NO_MATRIX;
            m.rows = rows_of(m.rot);
            m.set_pv_f(STEP_A, a);
            m.set_pv_f(STEP_B, b);
            m.state = 1;
            w.changed = true;
        }
        1 => {
            bob(w, id);
            let m = &mut w.mobys[id];
            let a = crate::hero::physics::add_rot(m.pv_f(TURN_A + 8), m.pv_f(STEP_A));
            let b = crate::hero::physics::add_rot(m.pv_f(TURN_B + 8), m.pv_f(STEP_B));
            m.set_pv_f(TURN_A + 8, a);
            m.set_pv_f(TURN_B + 8, b);
            turns(w, id);
            w.carry(id, &*ctx.host);
            w.rebuild(id, &*ctx.host);
        }
        _ => {}
    }
}
