//! Level 0x06's own classes (level 6; L06 addresses, the engine functions they call named by their L06 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! The seller of planet 7's coordinates 0x0ccf: [`host`]. The spinning breakables 0x0fc1 / 0x0fc0: [`spinners`]. The
//! floor button 0x08a1: crate::moby::classes::button. The barrier 0x08a0 and its strand field 0x0431:
//! crate::moby::classes::strands. The water sheet 0x093e (`0x404680` / `0x403ac0`): crate::water::ocean.
//! The liquid grid 0x0fb2 / 0x0fd6 / 0x0fce / 0x0fcf / 0x0fe4 (`0x4581c8`): crate::water::grid.
//!
//! The core 0x0fc2 (`0x458a00`) with its eight rays (class 0x13ba: crate::moby::classes::ray) and its pieces:
//! [`power_core`]. 0x0eb1 (`0x451ad0`, the challenge mode's flag 0x18): [`props`]. The creature in the water that
//! swallows Ratchet, 0x0947 (`0x404c60`; L09, L12 too): crate::moby::classes::swimmer.
//!
//! Not ported (level 6's other classes):
//! * 0x0fe6 (`0x45bbc8`, the decomp's `func_L06_0045A5B0`, 5 placed: 6024 bytes and its level helpers; it hunts
//!   Ratchet within 30 inside the polygons of its paths `+0x118` / `+0x138`, states 0..0x15) and 0x0bc7 (`0x422070`,
//!   `func_L06_00420AE8`, 4 placed, 3996 bytes), both on the line test (G-CLS-005).
//! * Ratchet's: the ferry 0x0330 (`0x3e5460`, L11 `0x3b7340`: a ride on paths with its own controls), 0x0b57
//!   (`0x41b200`, 3 placed: Ratchet's state 0x37 and the level's scene functions `0x479xxx`).
//! * The hints 0x0cdd (`0x4321e8`: help 0x9df / 0x9e0 / 0x9e1 / 0x9e2 / 0x9db / 0x9b5 by cuboids, on the help records
//!   crate::game::GameState::help; it reads Ratchet's `0x18c2d4`, the byte `0x1a7b27`, the uids' save bits `0x19b4a8`
//!   and the uids gone for good `0x21ecc4`, not mapped).

pub mod arc_shot;
pub mod barrel;
pub mod brawler;
pub mod breakables;
pub mod power_core;
pub mod gate;
pub mod glowbug;
pub mod host;
pub mod lifts;
pub mod pole;
pub mod props;
pub mod spinners;
pub mod traffic;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// The level's globals its classes share.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `0x1b3760`: the brawlers' arc shots' six slots taken (crate::moby::classes::level06::arc_shot).
    pub shot_slots: [u8; 6],
    /// `0x1b3790` / `0x1b37a0`: the six arcs' timers and alphas (one set for every slot).
    pub arc_timers: [i16; 6],
    pub arc_alpha: [i16; 6],
    /// `0x25ffc0`: each slot's six arcs of four points.
    pub arcs: [[[V3; 4]; 6]; 6],
    /// `0x1b375c`: the core's pieces' glow (crate::moby::classes::level06::power_core).
    pub core_glow: i32,
    /// `0x1b3758`: the core gone.
    pub core_gone: i32,
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x12c8 => Some(breakables::update_12c8),
        0x1183 => Some(breakables::update_1183),
        0x11b1 => Some(breakables::update_11b1),
        0x0fe8 => Some(breakables::update_0fe8),
        0x0fb5 => Some(breakables::update_0fb5),
        0x0fea => Some(breakables::update_0fea),
        0x0912 => Some(breakables::update_0912),
        0x0faa => Some(breakables::update_0faa),
        0x0fd2 | 0x0fd4 | 0x0fe9 => Some(breakables::update_0fd2),
        0x0dda => Some(props::update_0dda),
        0x086b | 0x086d => Some(traffic::update_086b),
        0x089d => Some(props::update_089d),
        0x0948 => Some(props::update_0948),
        0x0fe2 => Some(props::update_0fe2),
        0x02ee => Some(props::update_02ee),
        0x07ef => Some(props::update_07ef),
        0x0c4b => Some(props::update_0c4b),
        0x0e8e => Some(props::update_0e8e),
        0x0feb => Some(props::update_0feb),
        0x1240 => Some(props::update_1240),
        0x08c9 => Some(lifts::update_08c9),
        0x0849 => Some(lifts::update_0849),
        glowbug::CLASS => Some(glowbug::update_0db0),
        brawler::CLASS => Some(brawler::update_0bba),
        brawler::SWITCH_CLASS => Some(brawler::update_0fdc),
        arc_shot::CLASS => Some(arc_shot::update_0fe5),
        pole::CLASS => Some(pole::update_12c4),
        gate::CLASS => Some(gate::update_031a),
        0x0307 => Some(pole::update_0307),
        barrel::CLASS => Some(barrel::update_08b8),
        host::CLASS => Some(host::update_0ccf),
        spinners::CLASS_0FC1 => Some(spinners::update_0fc1),
        spinners::CLASS_0FC0 => Some(spinners::update_0fc0),
        power_core::CLASS => Some(power_core::update_0fc2),
        0x0eb1 => Some(props::update_0eb1),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        glowbug::CLASS => { glowbug::draw(w, id, host, cam, out); true }
        arc_shot::CLASS => { arc_shot::draw(w, id, game, cam, out); true }
        _ => false,
    }
}
