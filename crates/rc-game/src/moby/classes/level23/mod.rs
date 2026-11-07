//! Level 0x17's own classes (Damosel's orbit; L23 addresses, the engine functions they call named by their L23
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own. The classes this level shares with Dobbo's orbit (the same code at its own places) are
//! crate::moby::classes::level22's.

pub mod effects;
pub mod targets;

/// The words the opponent 0x0f7f keeps for the fight's other classes (its update `0x417888` is not ported: they stay
/// as the level starts them) (crate::moby::MobyWorld::level23).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x2388` / `$gp+0x238c`: the 0x0f7e's health and its floor; `$gp+0x468c`: the 0x120f's health
    /// ([`targets`]).
    pub f7e_health: f32,
    pub f7e_floor: f32,
    pub t120f_health: f32,
    /// `$gp+0x2654` / `$gp+0x2658`: the guard's count and its top ([`targets`], [`effects`]).
    pub hero_guard: i32,
    pub hero_guard_top: i32,
    /// `0x1b3694`: the opponent's hurt sound's wait.
    pub hurt_wait: i32,
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        // Giant Clank's body (L23 `0x3ae5c8`).
        super::level22::giant::BODY => Some(super::level22::giant::update_027d),
        targets::F7E => Some(targets::update_0f7e),
        targets::T120F => Some(targets::update_120f),
        effects::C1251 => Some(effects::update_1251),
        effects::C1252 => Some(effects::update_1252),
        _ => None,
    }
}
