//! Level 0x07's own classes (Siberius; L07 addresses, the engine functions they call named by their L07 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own. Level 4's light 0x0cfd (crate::moby::classes::level04::misc::update_0cfd) is this level's
//! `0x3e3698`.

pub mod bomb;
pub mod boss;
pub mod can;
pub mod dropship;
pub mod gunner;
pub mod hooks;
pub mod icicle;
pub mod laser;
pub mod mover;
pub mod prowler;
pub mod skill_target;
pub mod swarm;
pub mod tractor_block;
pub mod turret;
pub mod train;
pub mod snow;
pub mod spinner;
pub mod deleted;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        icicle::CLASS => Some(icicle::update_12a8),
        can::CLASS => Some(can::update_12a1),
        prowler::CLASS => Some(prowler::update_0d29),
        gunner::CLASS => Some(gunner::update_0d2a),
        gunner::SHOT => Some(gunner::update_0b7f),
        swarm::NEST => Some(swarm::update_0d1c),
        swarm::FLYER => Some(swarm::update_0d1d),
        turret::CLASS => Some(turret::update_0e4c),
        dropship::CLASS => Some(dropship::update_0d44),
        skill_target::CLASS => Some(skill_target::update_0f67),
        mover::CLASS => Some(mover::update_0eac),
        hooks::TRIGGER => Some(hooks::update_0d3c),
        hooks::BLAST => Some(hooks::update_0d3b),
        boss::CLASS => Some(boss::update_0e5f),
        boss::GLOW => Some(boss::update_0d45),
        laser::CLASS => Some(laser::update_0e28),
        bomb::CLASS => Some(bomb::update_0e52),
        tractor_block::CLASS => Some(tractor_block::update_0e30),
        train::LEAD..=train::CAR_LAST => Some(train::update_11d9),
        snow::CLASS => Some(snow::update_12ec),
        spinner::CLASS => Some(spinner::update_0c29),
        deleted::CLASS => Some(deleted::update_0d21),
        _ => None,
    }
}
