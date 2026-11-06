//! Level 0x12's director (Damosel; L18 addresses).

use crate::hero::physics::euler_rows;
use crate::moby::director::{gone, link, members, mission_is_done, place_hero};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f01;

/// Its pvars.
mod pv {
    /// The four streamed groups (words), whose members it owns (`+0xb8`).
    pub const STREAMS: usize = 0x00;
    /// The wave cuboids: the wave `w` (−1 before the first) ends in `+0x40 + 4·(w + 1)`; `+0x44` takes wave 5 back to
    /// wave 1.
    pub const AT_WAVE: usize = 0x40;
    pub const AT_BACK: usize = 0x44;
    /// The eight wave groups (words).
    pub const WAVE_GROUPS: usize = 0x60;
    /// The camera tweak (crate::camera::CamTweaks, slot + 1).
    pub const TWEAK: usize = 0x80;
    /// Ratchet in this cuboid as it starts: wave 5.
    pub const AT_START: usize = 0x84;
    /// The wave (−1 none).
    pub const WAVE: usize = 0x88;
    /// Half-words: `+0xa8` 1 at the start, the stream's wait `+0xaa`, the camera cut's ticks `+0xac` and its index
    /// `+0xae`.
    pub const A8: usize = 0xa8;
    pub const CUT_T: usize = 0xac;
    pub const CUT_K: usize = 0xae;
    /// The missions: `+0xb0` the way in, `+0xb4` the turret reached, `+0xb8` the turret's, `+0xbc` the way out.
    pub const M_IN: usize = 0xb0;
    pub const M_TURRET: usize = 0xb4;
    pub const M_DONE: usize = 0xb8;
    pub const M_OUT: usize = 0xbc;
    /// The mobys: the one it pokes `+0xc0`, the turret's door `+0xc4`, the gates `+0xc8` / `+0xcc`, the one it hooks
    /// `+0xd0` (`+0xd4` tested, `+0xd0` hooked again), the two whose `+0x74` it arms `+0xd8` / `+0xdc`, Ratchet's place at
    /// the turret `+0xe4`.
    pub const POKE: usize = 0xc0;
    pub const DOOR: usize = 0xc4;
    pub const GATE_A: usize = 0xc8;
    pub const GATE_B: usize = 0xcc;
    pub const HOOK: usize = 0xd0;
    pub const HOOK_B: usize = 0xd4;
    pub const ARMED: [usize; 2] = [0xd8, 0xdc];
    pub const HOME: usize = 0xe4;
    /// The group that must stand (`+0xe0`), the end's mission (`+0xe8`), the four streamed groups the end counts
    /// (`+0xf0..`).
    pub const END_GROUP: usize = 0xe0;
    pub const M_END: usize = 0xe8;
    pub const END_STREAMS: usize = 0xf0;
    /// The music: the first wave's `+0x100`, the end's `+0x104`.
    pub const MUSIC_A: usize = 0x100;
    pub const MUSIC_B: usize = 0x104;
}

/// The level's globals it keeps (`$gp+0x14b0`: the dialogue scene's step — 1 asked for, 2 playing).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    pub scene_step: i32,
}

/// The dialogue scene.
const SCENE: i32 = 0x95;
/// The skill point at the end (`0x1a7a83`).
const POINT: usize = 0x1b;
/// The flag bytes (`0x1395b8 +`): Ratchet to the fixed place with the scene (`+0x9f`), the item 5's gift (`+0xa1`).
const F_SCENE: usize = 0x9f;
const F_GIFT: usize = 0xa1;
/// The fixed place (`$gp+0x14c0` / `+0x14d0`).
const SCENE_POS: [f32; 3] = [508.3, 188.5, 110.5];
const SCENE_ROT: [f32; 3] = [0.0, 0.0, std::f32::consts::FRAC_PI_2];
/// The gift: item 5 and its message.
const GIFT: i32 = 5;
const GIFT_TEXT: i32 = 0x129f;
/// The camera tweak's fields by mode (`$gp+0x13f0..`): the distance, the pivot height, the look height.
const TWEAKS: [[f32; 3]; 3] = [[-1.0, -1.0, -1.0], [4.0, 2.0, 0.0], [8.0, 4.0, 4.0]];
/// The wave groups each wave shows (`0x3fb5f0`).
const WAVE_SHOWS: [&[usize]; 5] = [&[0, 1], &[2], &[3, 5, 6], &[4], &[7]];

/// 0x0f01 (`0x3fa510`, the decomp's v2.00 `func_US2_L18_003FA510`): Damosel's turret battle.
/// * **The start** (state 0): its draw distance 0, update distance 0xff; the members of the four streamed groups its
///   own (`+0xb8`, `0x3fa190`); `+0xa8` 1, the wave −1, `+0xae` −1, `+0xac` 0; state 1. The eight wave groups hidden
///   (`0x3fbc30`: mode | 0x43, & ~0x1000, no collision), the door `+0xc4` and gate `+0xc8` off (mode | 3, no
///   collision). The turret reached but not done: Ratchet put at `+0xe4` (state 0). Flag `0x1395b8 + 0x9f`: Ratchet
///   put at the fixed place, the flag cleared, the scene asked for. Ratchet in cuboid `+0x84`: wave 5.
/// * **The scene** (from the second tick): asked for in the level's first 10 frames, scene 0x95 queued and started
///   with the cut in (`0x38dc48` = L00 `0x371b60`); while it plays the director hidden in the scene mode; at its end
///   the body's idle (`0x2e4310(0)`: Ratchet 0, body 4 0x8d), without item 5 (`0x1a7b05`) and with flag `+0xa1` the item
///   given (`0x30eed0` = L00 `0x2f46e0`), asked for in the hand (`0x33d640` = L00 `0x3227d8`) and its message 0x129f,
///   the checkpoint where Ratchet stands.
/// * **The waves**: Ratchet in the cuboid that ends the wave takes the next: the sixth resets them all ([`reset`])
///   with the end's music `+0x104`; the first starts with the music `+0x100`; each shows its groups ([`show_wave`]).
///   In wave 5, cuboid `+0x44` resets them and takes wave 1 again.
/// * **The camera tweak** ([`tweak`]): by the wave — 2 far (8, 4, 4), 0 / 1 / 3 / 4 near (4, 2, 0), else none (−1).
/// * **The gates**: with the way out done, or the way in but not the turret: the gate `+0xc8` (hidden) on and `+0xcc`
///   (shown) off. The turret reached but not done: `+0xc8` off and the two armed (`+0x74` −1, `0x3ff480`). Not done
///   and the director hooked (`0x345910` = L00 `0x32a838`): its timer byte 1, the hook cleared, the turret's mission
///   done, back from the turret (`0x2d0050(0)`), the door on, and with `+0xc0`: `+0xd0` hooked (twice: the second test
///   is `+0xd4`'s), `+0xc0`'s timer byte 1. Its timer byte set and the way out done: the byte 0, `+0xc8` on, the door
///   off, `+0xd0` hooked, the two disarmed (`+0x74` 0, `0x3ff490`).
/// * **The end** (skill point 0x1b not had): the group `+0xe0` all standing and shown, and none of the four streamed
///   groups `+0xf0..` standing and shown: the skill point (crate::skill_points) and the mission `+0xe8` done.
///
/// Not ported: the streamed spawns (the stream cuboids `+0x30..`, their flags `+0xa0..`, the wait `+0xaa` of 0..180,
/// `0x3fbd90`: a member of a group put into play at a point out of view — 1324 bytes and the classes' own entries);
/// the camera cuts (`0x3fb2c0` / `0x3fb360`, cuboids `+0x90..` and the bits `0x1395b8 + 0x94`: the scripted camera
/// `0x335438` / `0x335370` / `0x3353c8` and its sound); the draw callback `0x3fc2c0` (eleven beams between two fixed
/// lists of points); the turret body (4: `0x2d0050`'s own part, the hero's turret mode is not ported — the return
/// is crate::hero::Hero::body_back while a body plays); `0x384740` (`0x152c26` = 1) before each placing; the
/// NPC-busy word `0x1b24e8` (crate::moby::talk); the tweak's gate `0x1cb9c0` (read as 0); the autosave after the
/// scene (`0x327c10` = L00 `0x30d658(0, −1)`); the skill point's reward flag `0x1a903c`.
pub fn update_0f01(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    let done = |w: &MobyWorld, ctx: &Ctx, o: usize| mission_is_done(ctx.game, w.mobys[id].pv_i(o));
    if w.mobys[id].state == 0 {
        start(w, id, ctx);
    } else {
        scene(w, id, ctx);
        waves(w, id, ctx);
    }

    // The gates.
    if done(w, ctx, pv::M_OUT) || (done(w, ctx, pv::M_IN) && !done(w, ctx, pv::M_TURRET)) {
        if let (Some(a), Some(b)) = (link(w, p(w, pv::GATE_A)), link(w, p(w, pv::GATE_B))) {
            if w.mobys[a].mode & 1 != 0 && w.mobys[b].mode & 1 == 0 {
                on(w, a);
                off(w, b);
            }
        }
    }
    if done(w, ctx, pv::M_TURRET) && !done(w, ctx, pv::M_DONE) {
        if let Some(a) = link(w, p(w, pv::GATE_A)) {
            off(w, a);
            for o in pv::ARMED { if let Some(k) = link(w, p(w, o)) { w.mobys[k].set_pv_i(0x74, -1); } }
        }
    }
    if !done(w, ctx, pv::M_DONE) && w.mobys[id].once & 4 != 0 {
        let m = &mut w.mobys[id];
        m.timer = 1;
        m.once &= !4;
        ctx.game.mission_done(p(w, pv::M_DONE) as u8);
        if ctx.hero.mode != 0 { ctx.hero.body_back(ctx.game.scenes.mode == crate::scene::SCENE); }
        if let Some(d) = link(w, p(w, pv::DOOR)) { on(w, d); }
        if let Some(k) = link(w, p(w, pv::POKE)) {
            if let Some(h) = link(w, p(w, pv::HOOK)) { w.mobys[h].once |= 4; }
            if p(w, pv::HOOK_B) != -1 { if let Some(h) = link(w, p(w, pv::HOOK)) { w.mobys[h].once |= 4; } }
            w.mobys[k].timer = 1;
        }
    }
    if w.mobys[id].timer != 0 && done(w, ctx, pv::M_OUT) {
        w.mobys[id].timer = 0;
        if let Some(a) = link(w, p(w, pv::GATE_A)) { on(w, a); }
        if let Some(d) = link(w, p(w, pv::DOOR)) { off(w, d); }
        if let Some(h) = link(w, p(w, pv::HOOK)) { w.mobys[h].once |= 4; }
        if p(w, pv::HOOK_B) != -1 { if let Some(h) = link(w, p(w, pv::HOOK)) { w.mobys[h].once |= 4; } }
        for o in pv::ARMED { if let Some(k) = link(w, p(w, o)) { w.mobys[k].set_pv_i(0x74, 0); } }
    }

    end(w, id, ctx);
}

/// The start (above).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    let m = &mut w.mobys[id];
    m.draw_dist = 0;
    m.update_dist = 0xff;
    for i in 0..4 {
        for k in members(w, p(w, pv::STREAMS + 4 * i)) { w.mobys[k].parent = Some(id); }
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::A8, 1);
    m.state = 1;
    m.set_pv_i(pv::WAVE, -1);
    m.set_pv_s(pv::CUT_K, -1);
    m.set_pv_s(pv::CUT_T, 0);
    // `0x3fbc30`: the wave groups hidden.
    for i in 0..8 {
        for k in members(w, p(w, pv::WAVE_GROUPS + 4 * i)) {
            let m = &mut w.mobys[k];
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x43;
        }
    }
    for o in [pv::DOOR, pv::GATE_A] { if let Some(k) = link(w, p(w, o)) { off(w, k); } }
    if mission_is_done(ctx.game, p(w, pv::M_TURRET)) && !mission_is_done(ctx.game, p(w, pv::M_DONE)) {
        if let Some(h) = link(w, p(w, pv::HOME)) {
            let (pos, rot) = (w.mobys[h].pos, w.mobys[h].rot);
            place_hero(ctx, pos, rot, 0);
        }
    }
    if ctx.game.flags.get(F_SCENE).is_some_and(|&b| b != 0) {
        place_hero(ctx, SCENE_POS, SCENE_ROT, 0);
        ctx.game.flags[F_SCENE] = 0;
        w.level18.scene_step = 1;
    }
    if ctx.level.in_cuboid(p(w, pv::AT_START), ctx.hero.pos) { w.mobys[id].set_pv_i(pv::WAVE, 5); }
}

/// The dialogue scene (above).
fn scene(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.frame < 10 && w.level18.scene_step == 1 {
        let g = &mut *ctx.game;
        if g.scenes.queue.push(SCENE) {
            g.scenes.start_queue(ctx.hero.hp);
            crate::moby::talk::cut_in(g);
            w.level18.scene_step = 2;
        }
    } else if w.level18.scene_step == 2 {
        if ctx.game.scenes.in_scene() {
            if ctx.game.scenes.mode == crate::scene::SCENE {
                let m = &mut w.mobys[id];
                m.drawn = 0;
                m.mode |= 1;
            }
        } else {
            // `0x2e4310(0)`: the body's idle.
            match ctx.hero.mode {
                0 => ctx.hero.request_state(0, true),
                4 => ctx.hero.request_state(0x8d, true),
                _ => {}
            }
            w.level18.scene_step = 0;
            let had = ctx.game.items.get(GIFT as usize).is_some_and(|&b| b != 0);
            if !had && ctx.game.flags.get(F_GIFT).is_some_and(|&b| b != 0) {
                ctx.game.get_item(GIFT);
                let held = ctx.game.held;
                ctx.hero.items.ask(held, GIFT);
                ctx.game.messages.push(crate::msgbox::Request::Text { text: GIFT_TEXT, ticks: None });
            }
            ctx.game.save_checkpoint(ctx.hero.pos, ctx.hero.rot);
        }
    }
}

/// The waves and the camera tweak (above).
fn waves(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    let wave = p(w, pv::WAVE);
    let ends = p(w, pv::AT_WAVE.wrapping_add((wave + 1) as usize * 4));
    if ctx.level.in_cuboid(ends, ctx.hero.pos) {
        let wave = wave + 1;
        w.mobys[id].set_pv_i(pv::WAVE, wave);
        if wave == 6 {
            reset(w, id);
            ctx.host.music_request(p(w, pv::MUSIC_B), -1);
        } else {
            if wave == 0 { ctx.host.music_request(p(w, pv::MUSIC_A), -1); }
            show_wave(w, id, wave);
        }
    }
    if p(w, pv::WAVE) == 5 && ctx.level.in_cuboid(p(w, pv::AT_BACK), ctx.hero.pos) {
        reset(w, id);
        w.mobys[id].set_pv_i(pv::WAVE, 1);
        show_wave(w, id, 1);
    }
    let mode = match p(w, pv::WAVE) { 2 => 2, 0 | 1 | 3 | 4 => 1, _ => 0 };
    tweak(w, id, ctx, mode);
}

/// `0x3fb210(mode)`: the tweak `+0x80` asked for (`0x2d7468(5)` = L00 `0x2d7468`; its first tick only that), then
/// its fields by the mode.
fn tweak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, mode: usize) {
    let s = w.mobys[id].pv_i(pv::TWEAK);
    if s == 0 {
        if let Some(i) = ctx.game.cam_tweaks.request(5) { w.mobys[id].set_pv_i(pv::TWEAK, i as i32 + 1); }
        return;
    }
    let f = TWEAKS[mode];
    let t = &mut ctx.game.cam_tweaks.slots[s as usize - 1];
    (t.dist, t.pivot_h, t.look_h) = (f[0], f[1], f[2]);
}

/// `0x3fb5f0(wave)`: the wave's groups put into play (mode & ~0x43 | 0x1001, their class's collision, `+0x31` 0).
fn show_wave(w: &mut MobyWorld, id: usize, wave: i32) {
    let Some(groups) = usize::try_from(wave).ok().and_then(|k| WAVE_SHOWS.get(k)) else { return };
    for &g in groups.iter() {
        for k in members(w, w.mobys[id].pv_i(pv::WAVE_GROUPS + 4 * g)) {
            let coll = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[k];
            m.mode = (m.mode & 0xffbc) | 0x1001;
            m.has_collision = coll;
            m.drawn = 0;
        }
    }
}

/// `0x3fb420`: every wave group's members home and off (mode | 3, no collision), the wave −1. The four walkers
/// (0x0d5c, 0x11cc, 0x0d9b, 0x11cf: `0x3fde28`) back at their start (`+0x60` / `+0x70`), mode & ~0x41 | 0x1000, state
/// 0; the breeder 0x0b69 (`0x3f8df8`) its `+0x2e` 1, `+0x20` its `+0x24`, no drops, back at `+0x210` / `+0x220`,
/// state 0.
fn reset(w: &mut MobyWorld, id: usize) {
    for i in 0..8 {
        for k in members(w, w.mobys[id].pv_i(pv::WAVE_GROUPS + 4 * i)) {
            let m = &mut w.mobys[k];
            match m.o_class {
                0x0d5c | 0x11cc | 0x0d9b | 0x11cf => {
                    m.mode = (m.mode & 0xffbe) | 0x1000;
                    (m.pos, m.rot) = (m.pv_v(0x60), m.pv_v(0x70));
                    m.rows = euler_rows(m.rot);
                    m.set_state(0, -1);
                }
                super::breeder::CLASS => {
                    m.set_pv_b(0x2e, 1);
                    m.set_pv_f(0x20, m.pv_s(0x24) as f32);
                    m.drops = 0;
                    (m.pos, m.rot) = (m.pv_v(0x210), m.pv_v(0x220));
                    m.rows = euler_rows(m.rot);
                    m.set_state(0, -1);
                }
                _ => {}
            }
            m.has_collision = false;
            m.mode |= 3;
        }
    }
    w.changed = true;
    w.mobys[id].set_pv_i(pv::WAVE, -1);
}

/// The end (above).
fn end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.skill_points[POINT] != 0 { return; }
    let shown = |w: &MobyWorld, k: usize| !gone(w, k) && w.mobys[k].mode & 1 == 0;
    if !members(w, w.mobys[id].pv_i(pv::END_GROUP)).into_iter().all(|k| shown(w, k)) { return; }
    for i in 0..4 {
        if members(w, w.mobys[id].pv_i(pv::END_STREAMS + 4 * i)).into_iter().any(|k| shown(w, k)) { return; }
    }
    ctx.game.award_skill_point(POINT);
    ctx.game.mission_done(w.mobys[id].pv_i(pv::M_END) as u8);
}

/// On: shown and solid (mode & ~3, its class's collision).
fn on(w: &mut MobyWorld, k: usize) {
    let coll = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[k];
    m.mode &= !3;
    m.has_collision = coll;
}

/// Off: hidden and passed through (mode | 3, no collision).
fn off(w: &mut MobyWorld, k: usize) {
    let m = &mut w.mobys[k];
    m.mode |= 3;
    m.has_collision = false;
}
