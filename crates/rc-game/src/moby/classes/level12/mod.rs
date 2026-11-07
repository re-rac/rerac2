//! Level 0x0c's own classes (level 12; L12 addresses, the engine functions they call named by their L12 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! Not ported yet: the launcher system — the critters 0x0bb8 (`0x3e0d78`, 10 placed: set-up `0x3e1890`, tail
//! `0x3e1b88`, the walkers 0x0fb4 given them `0x3e2f00`; launched by `0x3e1c30` from a cuboid's group, then along their
//! cuboid waypoints on the framework, counted dead toward the help texts 0xa2d..0xa30 and skill point 0x0a) and their
//! carrier 0x0fa5 (`0x41a510`, 1 placed: three paths, its child's pad `0x41ba60` / `0x41bb00`, its exhaust `0x41a188`).

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

pub mod breakables;
pub mod debris;
pub mod fence;
pub mod field;
pub mod grid;
pub mod head;
pub mod hints;
pub mod lifts;
pub mod mine;
pub mod misc;
pub mod props;
pub mod robot;
pub mod shot;
pub mod towers;
pub mod trader;
pub mod tractor;
pub mod trooper;
pub mod walker;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        0x110c => Some(breakables::update_110c),
        // The hover pad: Level 9's code (`0x408a28` = L09 `0x411ae8`).
        super::level09::hover_pad::CLASS => Some(super::level09::hover_pad::update_0e84),
        0x1307 => Some(breakables::update_1307),
        0x1114 => Some(breakables::update_1114),
        0x0852 | 0x0884 => Some(breakables::update_0852),
        0x0f69 => Some(props::update_0f69),
        0x087f => Some(props::update_087f),
        0x115f => Some(props::update_115f),
        0x1351 => Some(props::update_1351),
        0x0fc4 | 0x0fc5 => Some(tractor::update_0fc4),
        0x0fbc => Some(lifts::update_0fbc),
        0x0fd8 => Some(lifts::update_0fd8),
        0x0fd9 => Some(lifts::update_0fd9),
        0x0fc6 | 0x0fc7 | 0x0fc8 => Some(tractor::update_0fc6),
        0x0f6a => Some(tractor::update_0f6a),
        0x0fb8 => Some(tractor::update_0fb8),
        0x0fb9 => Some(tractor::update_0fb9),
        0x0f00 => Some(field::update_0f00),
        0x086a => Some(head::update_086a),
        0x117b => Some(props::update_117b),
        0x0ee8 => Some(props::update_0ee8),
        0x0424 => Some(props::update_0424),
        0x0fab => Some(mine::update_0fab),
        0x0fb4 => Some(walker::update_0fb4),
        0x0f31 => Some(trooper::update_0f31),
        0x114b => Some(trooper::update_114b),
        0x0d66 => Some(debris::update_0d66),
        0x03a6 => Some(shot::update_03a6),
        fence::CLASS => Some(fence::update_0fae),
        grid::CLASS => Some(grid::update_0f9f),
        robot::CLASS => Some(robot::update_1169),
        towers::CLASS => Some(towers::update_12b1),
        hints::CLASS => Some(hints::update),
        trader::CLASS => Some(trader::update),
        0x0f1f => Some(misc::update_0f1f),
        0x0e6a => Some(misc::update_0e6a),
        0x1109 => Some(misc::update_1109),
        // `0x38b400`: `jr ra`, as level 4's U293 classes.
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        0x0424 => {
            props::draw_0424(w, id, game.level, cam, out);
            true
        }
        0x0fab => {
            mine::draw(w, id, out);
            true
        }
        fence::CLASS => {
            fence::draw(w, id, out);
            true
        }
        grid::CLASS => {
            grid::draw(w, id, cam, out);
            true
        }
        _ => false,
    }
}
