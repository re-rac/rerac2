//! Todano's one-placed controls (L12 addresses):
//!
//! * **0x0f1f** ([`update_0f1f`], `0x3892a0`; the decomp's `func_L12_00388038`): in the scene mode (`0x1a8f00` = 2)
//!   with scene 5 loaded (`0x1bdab0`, set by `0x3090e8` = L00 `0x305690`), every moby of the level load's list
//!   (`$gp+0x3e38`, linked by `+0x28`: built by `0x332f80` = L00 `0x32e290` from the mobys alive and not mode 2 then;
//!   here the placed ones alive and not mode 2) without collision and hidden, not a target (mode `| 0x41 & !0x1000`).
//! * **0x0e6a** ([`update_0e6a`], `0x404250`, `func_US2_L12_00404250`): a pickup of item 0x10. Each tick (in state 0,
//!   the only one) its angles (0°, 180°, 0°: `$gp+0x2288..0x2290`), its bob 0.3 at 45°/s by the change (`0x327c78` =
//!   L00 `0x324430`, its phase and last offset the level's `0x1b3b9c` / `0x1b3ba0`: past its pvars here); its uid
//!   marked (`0x21f204[uid]` or its count in `0x19b4a8`): deleted. Then its sparkle ([`sparkle_0e6a`], `0x403fa0`) and,
//!   Ratchet within 1.5 across (`0x2f2718` = L00 `0x2ef050`): the message 0x1246 for 500 ticks (`0x32a8f0` = L00
//!   `0x327448`), item 0x10 his (`0x2f7dc8` = L00 `0x2f46e0`) and asked for (`0x3261f0` = L00 `0x3227d8`), the moby
//!   `+0x0` hooked (`0x32eb20` = L00 `0x32a818`), its uid counted, deleted.
//! * **0x1109** ([`update_1109`], `0x421ad0`, `func_L12_00420660`): a glow switch. Its pvars: the cuboid `+0x0` (−1
//!   none), the lamp `+0x4`, the moby `+0x8` it hooks (−1 none). **0**: 1. **1**: the lamp drawn, its glow pulses from
//!   `0x802020a0` to `0x80202050` every 60 ticks (`0x32a698` = L01 `0x31bc68`, crate::moby::classes::electrolyzer::
//!   pulse_colour, by `(frame % 60)·2π/60 − π`); Clank's moby (`0x1b08bc`) in the cuboid, or without it the upgraded
//!   Spiderbot (`$gp+0x2330`, the class 0x0e79, `0x405bc0`: alive and not in its states 5, 6, 8) in it: the lamp
//!   `0x80a02020`, 2, the moby `+0x8` hooked.
//!
//! Not ported: 0x1109's Clank (the port has no Clank mode on this level, so the Spiderbot's test always runs).

use crate::hero::physics::{add, dist2, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// The lamp's colours (`0x421b78..`).
const PULSE: [u32; 2] = [0x8020_20a0, 0x8020_2050];
const LIT: u32 = 0x80a0_2020;
/// `0x3c8efa36`: degrees to radians; `0x3998825c`: °/s to rad/tick.
const DEG: f32 = f32::from_bits(0x3c8e_fa36);
const DEG_S: f32 = f32::from_bits(0x3998_825c);
const DT: f32 = f32::from_bits(0x3c88_8889);
/// 0x0e6a's level globals, kept past its pvars (`0x1b3b9c` / `0x1b3ba0`).
const BOB_PHASE: usize = 0x10;
const BOB_LAST: usize = 0x14;

/// 0x0f1f (module docs).
pub fn update_0f1f(w: &mut MobyWorld, _id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode != crate::scene::SCENE || ctx.game.scenes.player.as_ref().map(|p| p.id) != Some(5) { return; }
    let n = w.first_created.min(w.mobys.len());
    for m in w.mobys[..n].iter_mut() {
        if 0x80 <= m.state || m.mode & crate::moby::mode::NO_UPDATE != 0 { continue; }
        m.has_collision = false;
        m.mode = (m.mode | 0x41) & 0xefff;
    }
}

fn uid_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let uid = w.mobys[id].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// `0x403fa0`: the pickup's sparkle. At (0.1, −0.5, 0) in its frame (`$gp+0x22d0`): one tick in four a glint (size 1,
/// `0x50804030`, 60 ticks) drifting a random way at 1/60..2/60 a tick, and three more (size 0.6, `0x40807f4b`) each
/// drifting faster by a random 0..0.333/60 (`0x3407b8` = L00 `0x33b5e0`, crate::particles::glint). Then four sparkles
/// (`0x340640` = L00 `0x33b468`: frame 0x35, soft, 2 ticks) stepped 0.1 apart from 0.3 behind it away from the camera
/// (`0x1b9d80`), sized `(2 + 0.5·i)` by its scale over its class's, coloured between `0x30803070` and `0x1080504b` by
/// `|0.5 − (255 − ((frame + 20·i) & 255))/255|`, their angle the frame's low byte (doubled for i & 2, from 255 for i & 1).
fn sparkle_0e6a(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let r = m.rows;
    let off = [f32::from_bits(0x3dcc_cccd), -0.5, 0.0];
    let pos: V3 = std::array::from_fn(|k| off[0] * r[0][k] + off[1] * r[1][k] + off[2] * r[2][k] + m.pos[k]);
    let hero = ctx.hero.pos;
    if ctx.game.rng.randi(4) == 0 {
        let mut v = ctx.game.rng.rand_vec(DT, 2.0 * DT);
        let rot = ctx.game.rng.randi(0xff) as u8;
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glint(sys, 1.0, pos, v, 0x5080_4030, 0x3c, rot, false, hero); }
        for _ in 0..3 {
            let d = ctx.game.rng.rand_vec(0.0, f32::from_bits(0x3eaa_7efa) * DT);
            v = add(v, d);
            let rot = ctx.game.rng.randi(0xff) as u8;
            if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glint(sys, f32::from_bits(0x3f19_999a), pos, v, 0x4080_7f4b, 0x3c, rot, false, hero); }
        }
    }
    let toward = set_len(sub(ctx.host.camera_pos(), pos), f32::from_bits(0xbe99_999a));
    let step = set_len(toward, f32::from_bits(0x3dcc_cccd));
    let mut at = add(toward, pos);
    let class_scale = w.class(m.o_class).map_or(1.0, |c| c.scale);
    let size_k = m.scale / class_scale;
    let frame = ctx.game.frame;
    for i in 0..4i32 {
        let x = (255 - (frame.wrapping_add(i * 20) & 0xff)) as f32 * f32::from_bits(0x3b80_8081);
        let rgba = crate::hud::lerp_rgba((0.5 - x).abs(), 0x3080_3070, 0x1080_504b);
        let mut a = frame & 0xff;
        if i & 2 != 0 { a *= 2; }
        if i & 1 != 0 { a = 255 - a; }
        let size = (2.0 + i as f32 * 0.5) * size_k;
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::sparkle(sys, size, at, rgba, a as u8, 0x35, true, 2, false, hero); }
        at = add(at, step);
    }
}

/// 0x0e6a (module docs).
pub fn update_0e6a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 0 { return; }
    let m = &mut w.mobys[id];
    m.rot = [0.0, 180.0 * DEG, 0.0];
    crate::moby::classes::props::bob(m, f32::from_bits(0x3e99_999a), 45.0 * DEG_S, 0.0, BOB_PHASE, BOB_LAST);
    if uid_done(w, id, ctx) { return w.delete(id); }
    sparkle_0e6a(w, id, ctx);
    if dist2(w.mobys[id].pos, ctx.hero.pos) < 1.5 {
        ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1246, ticks: Some(0x1f4) });
        ctx.game.get_item(0x10);
        if let Some(k) = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&k| k < w.mobys.len()) { w.mobys[k].once |= 4; }
        let held = ctx.game.held;
        ctx.hero.items.ask(held, 0x10);
        let uid = w.mobys[id].uid;
        ctx.game.count_uid(uid);
        w.delete(id);
    }
}

/// `0x405bc0`: the upgraded Spiderbot out (alive, not in its states 5, 6, 8).
fn bot_out(w: &MobyWorld) -> Option<usize> {
    w.mobys.iter().position(|m| m.o_class == crate::moby::classes::spiderbot::PLUS_BOT && m.state < 0x80 && !matches!(m.state, 5 | 6 | 8))
}

/// 0x1109 (module docs).
pub fn update_1109(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let lamp = usize::try_from(w.mobys[id].pv_i(4)).ok().filter(|&k| k < w.mobys.len());
            if let Some(l) = lamp.filter(|&l| w.mobys[l].drawn != 0) {
                let phase = (ctx.game.frame.rem_euclid(60)) as f32 * f32::from_bits(0x40c9_0fdb) / 60.0 - f32::from_bits(0x4049_0fdb);
                w.mobys[l].glow = crate::moby::classes::electrolyzer::pulse_colour(phase, PULSE[0], PULSE[1]);
            }
            let cub = w.mobys[id].pv_i(0);
            if cub == -1 { return; }
            let Some(b) = bot_out(w) else { return };
            if !ctx.level.in_cuboid(cub, w.mobys[b].pos) { return; }
            if let Some(l) = lamp { w.mobys[l].glow = LIT; }
            w.mobys[id].set_state(2, -1);
            if let Some(k) = usize::try_from(w.mobys[id].pv_i(8)).ok().filter(|&k| k < w.mobys.len()) { w.mobys[k].once |= 4; }
        }
        _ => {}
    }
}
