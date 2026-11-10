//! Tabora's crystals' and Grelbin's moonstones' counter **0x09f3** ([`update_09f3`], L08 `0x3dcd50`, L19 `0x3c5b50`:
//! the same code and data; the decomp's `func_L08_003DB8E8`) and its HUD callback ([`draw`], L08 `0x3dc980`, L19
//! `0x3c5780`; `func_L08_003DB518`) in the HUD's slot 2 (crate::hud::HudSlots). L08 addresses below.
//!
//! **Its pvars**: `+0` the count shown, `+4` the icon's grow (a float, 0..1).
//!
//! **The count** (`0x3dc908`): on level 8 the crystals (`0x1a7a22`, a half word, crate::game::GameState::crystals),
//! on level 0x13 the moonstones (`0x1a7a24`), else none.
//!
//! **Each tick**: at Ratchet's place (`0x189ea0`). **0**: the count shown the count, the grow 0, update distance 0xff,
//! the draw's moby itself (`$gp+0x1448`); 1. Every tenth frame (`0x1b2508`) the count shown steps one toward the count,
//! a rise starting the grow over. With a count, out of the scene mode (`0x1a8f00` ≠ 2) and in 1 or 2: 2, the callback
//! in slot 2 (`0x31a090(0, 0x3dc980, 2)`); else in 2 with no count or in a scene: the slot emptied, 1.
//!
//! **The draw** ([`draw`]): the grow on by 0.1 a frame up to 1; the GUI sprites 0xac (`0x60442d00`) and 0xad
//! (`0x55f0c070`) at the screen's centre; the icon 0xeaa2 (frame 1, on level 0x13 frame 2) `32·grow` square in
//! `0x60f0f0b0`, its corner at `16 + o` across and `364 + int(o − 2 + 0.5)` down (`o = (1 − grow)·16`, the screen's
//! height less 52); the count shown (`"%d"`, font 1 centred, `0x80f0f0f0`) at (70, 376). It does not take the
//! counters' place.
//!
//! Not ported: the draw's packet head (`0x38b250`) and depth word (`0x300d58` / `0x300dc8` / `0x300d88`: 0.62 while
//! the sprites draw), the PAL sizes (`0x1b3240` / `0x1b3244`: 1 and the +4 rows here).

use crate::game::GameState;
use crate::hud::{Align, Draw, SlotCallback};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x09f3;

const SHOWN: usize = 0;
const GROW: usize = 4;
/// The HUD slot it draws in.
const SLOT: usize = 2;

/// `0x3dc908` (module docs).
fn count(game: &GameState) -> i32 {
    match game.level {
        8 => game.crystals as i16 as i32,
        0x13 => game.moonstones,
        _ => 0,
    }
}

/// 0x09f3 (`0x3dcd50`): see the module.
pub fn update_09f3(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 8 { w.mobys[id].pvars.resize(8, 0); }
    let n = count(ctx.game);
    let m = &mut w.mobys[id];
    m.pos = ctx.hero.pos;
    if m.state == 0 {
        m.state = 1;
        m.set_pv_i(SHOWN, n);
        m.set_pv_f(GROW, 0.0);
        m.update_dist = 0xff;
    }
    if w.frame % 10 == 0 {
        let m = &mut w.mobys[id];
        let shown = m.pv_i(SHOWN);
        if shown < n {
            m.set_pv_f(GROW, 0.0);
            m.set_pv_i(SHOWN, shown + 1);
        } else if n < shown {
            m.set_pv_i(SHOWN, shown - 1);
        }
    }
    let scene = ctx.game.scenes.mode == crate::scene::SCENE;
    let m = &mut w.mobys[id];
    if 0 < n && !scene && matches!(m.state, 1 | 2) {
        m.state = 2;
        ctx.game.hud_slots.set(Some(SlotCallback::Counter { moby: id }), SLOT);
        return;
    }
    if (n <= 0 || scene) && m.state == 2 {
        ctx.game.hud_slots.set(None, SLOT);
        m.state = 1;
    }
}

/// `0x3dc980`: the counter's HUD callback (module docs) for the moby `id`; false (the counters still draw).
pub fn draw(w: &mut MobyWorld, id: usize, game: &GameState, out: &mut Vec<Draw>) -> bool {
    let Some(m) = w.mobys.get_mut(id).filter(|m| m.o_class == CLASS && 8 <= m.pvars.len()) else { return false };
    let mut grow = m.pv_f(GROW) + 0.1;
    if 1.0 < grow { grow = 1.0; }
    m.set_pv_f(GROW, grow);
    let centre = [(crate::gui::SCREEN_W as i32 / 2) as f32, (crate::gui::SCREEN_H as i32 / 2) as f32];
    for (id, colour) in [(0xac, 0x6044_2d00), (0xad, 0x55f0_c070)] {
        out.push(Draw::Sprite { id, pos: centre, scale: [1.0, 1.0], frame: 0.0, colour });
    }
    let o = (1.0 - grow) * 32.0 * 0.5;
    let bottom = (crate::gui::SCREEN_H as i32 - (52.0f32 + 0.5) as i32) as f32;
    let frame = if game.level == 0x13 { 2 } else { 1 };
    let x = (16.0 + o) as i32;
    let y = (bottom + ((o - 2.0) + 0.5) as i32 as f32) as i32;
    let size = (32.0 * grow) as i32;
    let tall = (size as f32 + 0.5) as i32;
    out.push(Draw::Icon { id: 0xeaa2, frame, pos: [x as f32, y as f32], size: [size as f32, tall as f32], colour: 0x60f0_f0b0 });
    let text = format!("{}", m.pv_i(SHOWN));
    let at = [(16.0f32 + 54.0) as i32 as f32, (bottom as i32 + (12.0f32 + 0.5) as i32) as f32];
    out.push(Draw::Text { text, pos: at, scale: 1.0, colour: 0x80f0_f0f0, align: Align::Centre });
    false
}
