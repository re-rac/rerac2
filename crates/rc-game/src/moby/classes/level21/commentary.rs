//! The museum's commentary, class **0x130e** ([`update_130e`], `0x3856d8`; the decomp's `func_US2_L30_003856D8`, its
//! level helpers `func_L30_00384768` / `003848B8` / `00384980` at L21 `0x385978` / `0x385ac8` / `0x385b90`): standing
//! at an exhibit, △ plays the developers' two lines about it on the dialogue player (crate::audio::speech, the
//! player's state `0x1a63a8`) while its caption shows.
//!
//! Its pvars: `+0x00` the 27 exhibits' cuboids (−1 none), `+0x70` each exhibit's own line, `+0xe0` / `+0x10c` the 11
//! kinds' first and later introductions (`+0xe0 + 4k` the first time kind `k` is heard this visit, `+0x10c + 4k`
//! after; kind 6 has no "heard": exhibit 1 takes `+0xf8`, the others `+0x124`), `+0x140` the cuboid Ratchet is in,
//! `+0x144` its exhibit. Each exhibit's kind is the table [`KIND`] (`0x295b60`), its caption the text `0x295af0[i]`
//! (0x31dc + i). A line's stream id is `(pvar & 0x3fff) + 3000`. By state:
//! * **0**: updated at any distance (`+0x30` 0xff), no kind heard (`$gp−0x3b0`, crate::moby::talk::Mem::museum_heard),
//!   no cuboid, exhibit 0; 1.
//! * **1**: Ratchet in an exhibit's cuboid (the first in order, `0x385978`): that cuboid and exhibit kept; 2.
//! * **2**: out of it: 1. △ (`0x138324 & 0x10`): 3. Else the prompt 0x1bc7 held (`0x32c268(12, 0x1bc7, 1)` = L00
//!   `0x327ae0`, crate::prompt::Prompt::force).
//! * **3**: the dialogue free (`+0x68` 0, `+0x24` −1): the exhibit's introduction asked for (`+0x24` = id, `+0x28` = 0,
//!   `0x385ac8`; crate::moby::MobyHost::request_speech); 4.
//! * **4**: the line started and held (`+0x72` = 3, its id `+0x6c` below 6000): let go (`0x2b6d28` = L21 `0x32f3b8`,
//!   crate::moby::MobyHost::speech_go); 5.
//! * **5**: the dialogue free: the exhibit's own line asked for (`0x385b90`); 6.
//! * **6**: as 4; 7.
//! * **7**: 10 ticks in and the dialogue free: 2.
//! * **3..7**, after the state's step: the prompt lease held with no text (`0x32c268(1, 0, 1)`); △ with a line playing
//!   or asked for: the line stopped (`+0x72` = 5 unless 6 / 7 already, crate::moby::MobyHost::speech_stop), 2.
//! * Every tick its state's ticks (`+0x96`) + 1.
//!
//! **The caption** (the draw callback `0x385a08`, queued every tick of states 3..7 through `0x2ee6a0` = L08
//! `0x2f2d10`, a 64-entry list the frame's draw runs): Ratchet held (`0x18a000` = 5, his landing lag: the port sets
//! it in the update) and his items kept away (`0x18c0dc` = 1), and the exhibit's caption in the HUD's framed text box
//! (`0x314318` = L00 `0x30db60` → `0x3380b0` on the HUD object `+0x36f28`: the box `0x1aec48` {top 203, bottom 410,
//! left 23, right 485, centre (253, 347), line 16}, the text colour 0x80f0f0f0, style 1).
//!
//! Not ported: the caption's draw (no moby asks the HUD for a framed box a frame; crate::gui::TextBox is the vendor's
//! and the menus'), Ratchet's items kept away (`0x18c0dc`, not modelled), and the lines' sound itself where the
//! dialogue player has no stream for them (crate::audio::speech plays the level's track table's 3000.. range).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x130e;

/// The exhibits.
const N: usize = 27;
/// `0x295b60`: each exhibit's kind (0..10).
pub const KIND: [u8; N] = [5, 6, 0, 9, 4, 8, 3, 8, 6, 3, 2, 10, 10, 5, 10, 7, 7, 5, 0, 0, 0, 4, 7, 7, 1, 9, 2];
/// The pvars.
const LINES: usize = 0x70;
const FIRST: usize = 0xe0;
const AGAIN: usize = 0x10c;
const AT: usize = 0x140;
const INDEX: usize = 0x144;
/// The prompt's owner and text.
const PROMPT_OWNER: i32 = 12;
const PROMPT_TEXT: u32 = 0x1bc7;
/// The kind with no "heard" bit.
const ODD_KIND: u8 = 6;

/// `(pvar & 0x3fff) + 3000`: a line's stream id.
fn line(v: i32) -> i32 { (v & 0x3fff) + 3000 }

/// `0x385978`: the first exhibit whose cuboid Ratchet is in, kept (`+0x140`, `+0x144`).
fn find(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    for i in 0..N {
        let c = w.mobys[id].pv_i(4 * i);
        if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) {
            let m = &mut w.mobys[id];
            m.set_pv_i(AT, c);
            m.set_pv_i(INDEX, i as i32);
            return true;
        }
    }
    false
}

/// `0x385ac8`: with the dialogue free, the exhibit's introduction asked for (module docs).
fn intro(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if !ctx.host.speech_free() { return false; }
    let Ok(i) = usize::try_from(w.mobys[id].pv_i(INDEX)) else { return false };
    let Some(&kind) = KIND.get(i) else { return false };
    let k = kind as usize;
    let m = &w.mobys[id];
    let v = if kind == ODD_KIND {
        m.pv_i(if i == 1 { FIRST + 4 * k } else { AGAIN + 4 * k })
    } else if w.talk.museum_heard >> kind & 1 == 0 {
        w.talk.museum_heard |= 1 << kind;
        m.pv_i(FIRST + 4 * k)
    } else {
        m.pv_i(AGAIN + 4 * k)
    };
    ctx.host.request_speech(line(v));
    true
}

/// `0x385b90`: with the dialogue free, the exhibit's own line asked for.
fn own_line(w: &MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if !ctx.host.speech_free() { return false; }
    let i = w.mobys[id].pv_i(INDEX);
    if i == -1 { return false; }
    ctx.host.request_speech(line(w.mobys[id].pv_i(LINES + 4 * i.max(0) as usize)));
    true
}

/// The caption's callback `0x385a08` (module docs): Ratchet held.
fn caption(ctx: &mut Ctx) { ctx.hero.land_lag = 5; }

/// The update (module docs).
pub fn update_130e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let triangle = ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0;
    match w.mobys[id].state {
        0 => {
            w.talk.museum_heard = 0;
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_pv_i(AT, -1);
            m.set_pv_i(INDEX, 0);
            m.set_state(1, -1);
        }
        1 => {
            if find(w, id, ctx) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            if !ctx.level.in_cuboid(w.mobys[id].pv_i(AT), ctx.hero.pos) {
                w.mobys[id].set_state(1, -1);
            } else if triangle {
                w.mobys[id].set_state(3, -1);
            } else {
                ctx.game.prompt.force(PROMPT_OWNER, PROMPT_TEXT, 1);
            }
        }
        3 => {
            if intro(w, id, ctx) { w.mobys[id].set_state(4, -1); }
            caption(ctx);
        }
        4 | 6 => {
            if ctx.host.speech_held() {
                ctx.host.speech_go();
                let next = w.mobys[id].state + 1;
                w.mobys[id].set_state(next, -1);
            }
            caption(ctx);
        }
        5 => {
            if own_line(w, id, ctx) { w.mobys[id].set_state(6, -1); }
            caption(ctx);
        }
        7 => {
            if 10 <= w.mobys[id].state_ticks as u16 && ctx.host.speech_free() { w.mobys[id].set_state(2, -1); }
            caption(ctx);
        }
        _ => {}
    }
    if 3 <= w.mobys[id].state && w.mobys[id].state < 0x80 {
        ctx.game.prompt.force(1, 0, 1);
        if triangle && !ctx.host.speech_free() {
            ctx.host.speech_stop();
            w.mobys[id].set_state(2, -1);
        }
    }
    let m = &mut w.mobys[id];
    m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16;
}
