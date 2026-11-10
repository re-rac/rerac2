//! Level 0x04's own classes (level 4; L04 addresses, the engine functions they call named by their L04 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! The talking NPCs 0x0c38, 0x0d3e and 0x0da8: [`talkers`]. The hint manager 0x0c26: [`hints`]. The liquid grid 0x0960 (`0x3e9320`): crate::water::grid.
//!
//! The hoverbike race (Ratchet's hoverbike 0x0bbc, the racers 0x0b2d, the boost jets 0x0474 / 0x0cef, shared with level
//! 11): crate::moby::classes::race.
//!
//! The quad fountain 0x02f8 (`0x3c4f80`, also L19) and its clock 0x0329 (`0x3c83c8`, also L17 / L19):
//! crate::fx::fountain. The ripple water 0x0b37 (`0x4084c0`) and 0x0b54 (`0x409440`): crate::water::ripple.
//!
//! Shared with other levels: the launcher 0x0a58 and its carrier 0x0e44 (crate::moby::classes::launcher), the
//! breakable 0x1293 (crate::moby::classes::breakables), the beacons 0x0a99 / 0x0ab8 (crate::moby::classes::beacon),
//! the light 0x0cfd (crate::moby::classes::hue_light).

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

pub mod ambush;
pub mod beast;
pub mod hints;
pub mod misc;
pub mod platforms;
pub mod rider;
pub mod skitter;
pub mod talkers;
pub mod tie_wave;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        rider::CLASS => Some(rider::update),
        beast::CLASS => Some(beast::update),
        skitter::CLASS => Some(skitter::update),
        0x0e8b | 0x0e8f | 0x0e90 => Some(misc::update_0e8b),
        0x0e8d => Some(misc::update_0e8d),
        0x1157 => Some(misc::update_1157),
        0x123f => Some(misc::update_123f),
        0x0d8f => Some(misc::update_0d8f),
        0x0d61 => Some(misc::update_0d61),
        0x0f9c => Some(misc::update_0f9c),
        0x0f71 => Some(misc::update_0f71),
        0x0c17 => Some(misc::update_0c17),
        0x09f0 => Some(ambush::update_09f0),
        0x0ce6 => Some(ambush::update_0ce6),
        0x1362 => Some(ambush::update_1362),
        0x1137 => Some(tie_wave::update),
        0x0f61 => Some(misc::update_0f61),
        0x0968 => Some(platforms::update_0968),
        0x09eb => Some(platforms::update_09eb),
        0x09ea => Some(platforms::update_09ea),
        0x09bd => Some(platforms::update_09bd),
        talkers::PLANET_GIVER => Some(talkers::update_0c38),
        talkers::SELLER => Some(talkers::update_0d3e),
        talkers::TALKER => Some(talkers::update_0da8),
        hints::CLASS => Some(hints::update_0c26),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        beast::CLASS => { beast::draw(w, id, cam, out); true }
        _ => false,
    }
}
