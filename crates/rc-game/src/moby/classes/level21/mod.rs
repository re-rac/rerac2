//! The Insomniac Museum's own classes (`G/LEVEL21.WAD`, the game's level 30; L21 addresses as the census has them,
//! the decomp's `L30`). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod blaster;
pub mod commentary;
pub mod emitter;
pub mod exhibits;
pub mod flung;
pub mod hacking;
pub mod hovercraft;
pub mod ripples;
pub mod shooter;
pub mod toys;
pub mod zone;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        exhibits::EXHIBIT => Some(exhibits::update_134e),
        toys::STRUCK => Some(toys::update_0bd5),
        toys::PAD => Some(toys::update_08d7),
        emitter::CLASS => Some(emitter::update_1317),
        flung::CLASS => Some(flung::update_0c46),
        commentary::CLASS => Some(commentary::update_130e),
        zone::CLASS => Some(zone::update_12e7),
        // Also in the overlays of levels 8 and 19 (L08 `0x3c7f00`, L19 `0x3b7610`: the same code), placed on the museum only.
        hovercraft::EARLY => Some(hovercraft::update_085c),
        hovercraft::MUSEUM => Some(hovercraft::update_0f2f),
        ripples::CLASS => Some(ripples::update_1198),
        shooter::GUN => Some(shooter::update_11d4),
        shooter::SHOT => Some(shooter::update_11ea),
        blaster::CLASS => Some(blaster::update_11e4),
        hacking::CLASS => Some(hacking::update_1312),
        // 0x0b84 (`0x3da5d8`, L00 `0x4450c0`): only a debug print (`0x2a48f0`), so crate::moby::classes::props::update_empty.
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them (the pool 0x1198's, first list;
/// the shot 0x11ea's beam, second list: `0x417630` = L13 `0x40f2d0`, crate::fx::trails::shot_beam).
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, _host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        ripples::CLASS if first => ripples::draw(w, id, cam, out),
        shooter::SHOT if !first => crate::fx::trails::shot_beam(w, id, &mut game.rng, cam, out),
        _ => return false,
    }
    true
}
