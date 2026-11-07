//! The museum's blast exhibit, class **0x11e4** ([`update_11e4`], `0x415710`, 1 placed; the decomp's
//! `func_US2_L30_00415710`): a blast at its place every 45 ticks, all of it set by its pvars.
//!
//! Its pvars (the editor's fields): `+0x04` the size (a float), `+0x08` the camera shake (0 / 1), `+0x0c` / `+0x10` /
//! `+0x14` the three sets of blast sprites, `+0x18` the puffs, `+0x1c` the streaks, `+0x20` / `+0x24` / `+0x28` its
//! r / g / b, `+0x30` the screen flash's facing test, `+0x34` the flash ball, `+0x38` a word the blast moby keeps
//! (`+0x1c`, for its draws), `+0x3c` the cuboid that offers the editor (−1 none). The placed one: 0.5, no shake,
//! 25 / 20 / 20, 20 puffs, 45 streaks, colour 0x3064ff, the flash ball.
//!
//! * **0** (set-up): the timer (`+0x1c`, a float; the port's pvar `+0x80`) 0; then 1.
//! * **1**: Ratchet in its cuboid (`0x327920` = L00 `0x321858`): the prompt 0x1338 held (`0x32c268(1, 0x1338, 1)` =
//!   L00 `0x327ae0`, crate::prompt::Prompt::force).
//! * **Each tick**: at 45 the timer back to 0 and the blast (`0x415a58` → `0x415f30`, the museum's copy of L00's
//!   `0x3f8348` in its one colour: crate::moby::blast::burst_tinted); then the timer up by 1.
//!
//! Not ported: the editor (state 2: L2 + R2 held in state 1, `0x138320 & 3`; its menu moby 0x118a, `0x413b08` /
//! `0x413ba0`), as the emitter 0x1317's (crate::moby::classes::level21::emitter); the blast moby 0x11e5 itself
//! (`0x317ad8(0x11e5)`: its update `0x415d58` only lives out its 60 ticks, its draws — the screen flash and its facing
//! test `+0x18`, `+0x1c` — are not ported, as the 0x13a0's in crate::moby::blast).

use crate::moby::{blast, Ctx, MobyWorld};

pub const CLASS: i16 = 0x11e4;

const SIZE: usize = 0x04;
const SHAKE: usize = 0x08;
const BURSTS: usize = 0x0c;
const PUFFS: usize = 0x18;
const STREAKS: usize = 0x1c;
const RGB: usize = 0x20;
const LIGHT: usize = 0x34;
const CUBOID: usize = 0x3c;
/// `+0x1c` of the moby (its place's w), kept in a pvar.
const TIMER: usize = 0x80;

pub fn update_11e4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_pv_f(TIMER, 0.0);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let c = w.mobys[id].pv_i(CUBOID);
            if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) { ctx.game.prompt.force(1, 0x1338, 1); }
        }
        _ => {}
    }
    if 45.0 <= w.mobys[id].pv_f(TIMER) {
        let m = &mut w.mobys[id];
        m.set_pv_f(TIMER, 0.0);
        let c = (m.pv_i(RGB + 8) as u32) << 16 | (m.pv_i(RGB + 4) as u32) << 8 | m.pv_i(RGB) as u32;
        let b = blast::Blast {
            size: m.pv_f(SIZE),
            bursts: [m.pv_i(BURSTS), m.pv_i(BURSTS + 4), m.pv_i(BURSTS + 8)],
            puffs: m.pv_i(PUFFS),
            streaks: m.pv_i(STREAKS),
            light: m.pv_i(LIGHT) != 0,
            shake: m.pv_i(SHAKE) != 0,
        };
        let at = m.pos;
        blast::burst_tinted(w, ctx, at, &b, c);
    }
    let t = w.mobys[id].pv_f(TIMER);
    w.mobys[id].set_pv_f(TIMER, t + 1.0);
}
