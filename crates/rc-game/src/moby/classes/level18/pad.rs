//! The step pad **0x0ed0** (L18 `0x42c4f8`, 1 placed; L19 `0x40efd0`, 5; L20 `0x432580`, 10; the decomp's
//! `func_US2_L18_0042C4F8`, the same code on the three levels): a pad that pulses grey until Ratchet stands on it (or
//! its uid or mission says it was set off before), then sets off what it is tied to and glows orange.
//!
//! **Its pvars:** `+0x00` a moby to hook (−1 none), `+0x04` the pulse's phase, `+0x08` its sound played, `+0x09` the
//! body back to Ratchet when set off (`0x2d0050`), `+0x0a` the timer byte for its group (0xff none), `+0x0b` its group
//! hooked, `+0x0c` the group (−1 none), `+0x10` its uid not counted, `+0x11` its uid's record not read, `+0x12` its
//! mission done and the checkpoint where Ratchet stands, `+0x13` set off once its mission is done.
//!
//! **Each tick** the phase turns 6° (`0x30a650` = L00 `0x2efe60`, wrapped).
//! * **0**: touched — Ratchet on it (`+0x33c` / `+0x340`: crate::hero::Hero::ground_moby, grounded) within half its
//!   size (its scale over its class's) across of its joint 0 (`0x32d808` = L00 `0x3127c8`; `0x309820` = L00
//!   `0x2ef050`, the xy distance): its uid counted (crate::game::GameState::count_uid) unless `+0x10`, its mission done
//!   (`0x32e2e8` = L00 `0x3132a8`) with `+0x12`. Set off when touched, when (with a mission and not `+0x11`) its uid is
//!   gone for good (crate::game::GameState::uid_dead) or counted, or with `+0x13` its mission done: state 1 and its
//!   timer byte 1; the moby `+0x00` hooked (`0x3458f0` = L00 `0x32a818`); with `+0x09` the body back
//!   (crate::hero::Hero::body_back); its group hooked (`0x337928` = L02 `0x320b50`) with `+0x0b` and given its timer
//!   byte (`0x3378d8` = L00 `0x31c970`) unless 0xff; touched with `+0x12`, the checkpoint at Ratchet (`0x375e08` =
//!   L00 `0x359b20`). Its glow (`+0x78`) a grey pulse: `(sin(phase)·4 − 3)·128` (`0x309c50` / `0x30a7a8` = L00
//!   `0x2ef460` / `0x2effb8`) held to 32..128.
//! * **Else**: on its first tick its sound (class 0x08a1's sound 0, `0x37c980` = L00 `0x360698`); its glow
//!   0x80208010.
//!
//! The checkpoint is at Ratchet's own moby (`+0xc18`): with the Hypnomatic's body playing (crate::hero::hypno) where
//! he stands, not the creature.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0ed0;
/// The class whose sound 0 it plays when set off.
const SOUND_CLASS: i16 = 0x08a1;

pub fn update_0ed0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return; }
    let phase = crate::hero::physics::add_rot(w.mobys[id].pv_f(4), 0.104_719_77);
    w.mobys[id].set_pv_f(4, phase);
    let flag = |w: &MobyWorld, k: usize| w.mobys[id].pvars.get(k).copied().unwrap_or(0);
    if w.mobys[id].state != 0 {
        if flag(w, 8) == 0 {
            ctx.host.play_sound_of(&w.mobys[id], SOUND_CLASS, 0, 0);
            w.mobys[id].pvars[8] = 1;
        }
        w.mobys[id].glow = 0x8020_8010;
        return;
    }
    let m = &w.mobys[id];
    let p = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let hp = ctx.hero.pos;
    let d = ((hp[0] - p[0]).powi(2) + (hp[1] - p[1]).powi(2)).sqrt();
    w.mobys[id].pvars[8] = 0;
    let m = &w.mobys[id];
    let class_scale = w.class(m.o_class).map_or(1.0, |c| c.scale);
    let (uid, mission) = (m.uid, m.mission);
    let touched = ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 && d < m.scale * 0.5 / class_scale;
    if touched {
        if flag(w, 0x10) == 0 { ctx.game.count_uid(uid); }
        if flag(w, 0x12) != 0 { ctx.game.mission_done(mission); }
    }
    let before = mission != 0xff && flag(w, 0x11) == 0
        && (ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&v| v != 0) || ctx.game.uid_count(uid) != 0);
    let done = flag(w, 0x13) != 0
        && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|&b| b & 0x80 != 0);
    if before || done || touched {
        let m = &mut w.mobys[id];
        m.timer = 1;
        m.state = 1;
        let hook = m.pv_i(0);
        if let Some(t) = usize::try_from(hook).ok().and_then(|k| w.mobys.get_mut(k)) { t.once |= 4; }
        if flag(w, 9) != 0 { ctx.hero.body_back(ctx.game.scenes.mode == crate::scene::SCENE); }
        let group = w.mobys[id].pv_i(0xc);
        if group != -1 {
            if flag(w, 0xb) != 0 { crate::moby::classes::maktar::hook_group(w, group, true); }
            let t = flag(w, 0xa);
            if t != 0xff { w.poke_group(group, t); }
        }
        if flag(w, 0x12) != 0 && touched {
            let h = &ctx.hero;
            let (p, r) = if h.mode == 4 { (h.hypno.ratchet_pos, h.hypno.ratchet_rot) } else { (h.pos, h.rot) };
            ctx.game.save_checkpoint(p, r);
        }
    }
    let v = (((phase.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(32, 128) as u32;
    w.mobys[id].glow = 0x8000_0000 | v << 16 | v << 8 | v;
}
