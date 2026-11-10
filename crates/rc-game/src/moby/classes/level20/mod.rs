//! Level 0x14's own classes (Yeedil; L20 addresses, the engine functions they call named by their L20 addresses with
//! the L00 ones the port knows them by). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own.
//!
//! Shared with other levels: the fading field 0x119b (crate::moby::classes::fading_field, L19's 0x1204), the floating
//! platform 0x1257 (crate::moby::classes::floe, L19's 0x0f4b), the arc glow 0x1212 (crate::moby::classes::arc_glow,
//! L18), the breeding critter 0x0b69 (crate::moby::classes::breeder) and the step pad 0x0ed0
//! (crate::moby::classes::pads; L18, L19).

pub mod biter;
pub mod boss;
pub mod director;
pub mod effects;
pub mod fighter;
pub mod misc;
pub mod movers;
pub mod prize;
pub mod props;
pub mod shooter;
pub mod strafer;

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, _host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, _cam: crate::hero::physics::V3, id: usize, first: bool, _out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        director::CLASS if !first => director::draw(w, game, id),
        _ => return false,
    }
    true
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        biter::CLASS => Some(biter::update_119e),
        boss::CLASS => Some(boss::update_11e0),
        director::CLASS => Some(director::update_0c36),
        fighter::CLASS => Some(fighter::update_12f3),
        shooter::CLASS => Some(shooter::update_12ef),
        strafer::CLASS => Some(strafer::update_12f8),
        props::SLIDER => Some(props::update_124f),
        props::CRYSTAL => Some(props::update_1269),
        props::NEST => Some(props::update_11ec),
        // `0x446c78`: `jr ra`.
        misc::SLAB => Some(misc::update_11ef),
        misc::LIFT => Some(misc::update_1246),
        misc::CRATE => Some(misc::update_131a),
        misc::TRIGGER => Some(misc::update_04f7),
        misc::BOX => Some(misc::update_0f88),
        misc::PAD => Some(misc::update_1209),
        misc::BRIDGE => Some(misc::update_11e8),
        misc::SLAB_QUIET => Some(misc::update_1236),
        movers::TURNTABLE => Some(movers::update_1170),
        movers::BOBBER => Some(movers::update_1306),
        movers::GATE => Some(movers::update_1243),
        movers::RISER => Some(movers::update_1283),
        prize::CLASS => Some(prize::update_12d3),
        effects::SCENE_FX => Some(effects::update_0f2a),
        effects::RISING => Some(effects::update_0548),
        _ => None,
    }
}
