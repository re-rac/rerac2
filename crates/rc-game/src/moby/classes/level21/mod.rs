//! The Insomniac Museum's own classes (`G/LEVEL21.WAD`, the game's level 30; L21 addresses as the census has them,
//! the decomp's `L30`). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod commentary;
pub mod emitter;
pub mod exhibits;
pub mod flung;
pub mod toys;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        exhibits::EXHIBIT => Some(exhibits::update_134e),
        toys::STRUCK => Some(toys::update_0bd5),
        toys::PAD => Some(toys::update_08d7),
        emitter::CLASS => Some(emitter::update_1317),
        flung::CLASS => Some(flung::update_0c46),
        commentary::CLASS => Some(commentary::update_130e),
        // 0x0b84 (`0x3da5d8`, L00 `0x4450c0`): only a debug print (`0x2a48f0`).
        0x0b84 => Some(super::level04::misc::update_empty),
        _ => None,
    }
}
