//! The NPCs who talk and sell: the pieces every one of them is built from (G-UI-010). The game has no single NPC
//! routine; each NPC class carries the same blocks inline or calls the same small functions, and the port keeps
//! those here once.
//!
//! * **Can talk** ([`can_talk`], L01 `0x3258c0`, L04 `0x3530b8`, L06 `0x3709c0`, L08 `0x33e270`, L09 `0x32c7d8`, L17
//!   `0x332ee0`, L18 `0x34b1c8`, L19 `0x32d308`; the decomp's `func_L01_003245E0(m, n, r)`): Ratchet near enough and
//!   free to talk.
//! * **The offer** ([`offer`]): can talk (`n` = 10, as every caller passes) → the second message box with the text
//!   formatted (L01 `0x325830`, L08 `0x33e1e0`, L17 `0x332e50`, L18 `0x34b138`; `func_L01_00324550(text, ...)`: the
//!   string by id, `vsprintf` into a buffer, `0x2ed8b8` → L00 `0x30eff8`) and the talk record marked (L01 `0x3259b0`,
//!   `func_L01_003246D0`: not ported, below).
//! * **The scene hide** ([`hide_in_scene`], [`show`]): while a talk scene the NPC asked for plays (the game in the scene
//!   mode, `0x1a8f00` = 2) it is hidden (`+0x31` 0, mode | 1), and shown again when the scene is over (`0x371ba8`,
//!   crate::scene::Scenes::in_scene).
//! * **Ratchet put back** ([`put_hero`], `0x2b9e68(cuboid + 0x30, cuboid + 0x70, 0, 1)`): after the first talk scene
//!   Ratchet stands at the NPC's cuboid's place and angles, idle.
//! * **The cut in** ([`cut_in`], L00 `0x371b60`, L09 `0x36ee10`; the decomp's `func_002F5CC8`): a scene asked for
//!   starts without the blackout's ramp (`0x1a8fcc` = 1.2), behind a 6-frame blocking fade when the screen is not
//!   black already (`0x1b1860` < 1: `0x2e7c90(6)`).
//! * **The idle** ([`idle`]): in any state but 0, in its looking sequence and with its idle timer (a half-word) run
//!   out, the timer set to 1200..2400 ticks again and its fidget sequence started (10 ticks' blend; Smolg's seller and
//!   Oozla's have none); else, its sequence having wrapped (`+0x60 & 2`) and not the looking one, the looking one again.
//! * **The look** ([`look`]): in its looking sequence it turns its head (joint list 0) and body (list 1) toward a
//!   point: Ratchet's centre (`0x189f00`, a unit over his feet) while he is in front of it (within 8 across, within
//!   90° of its facing) and moving (`0x189f30` over 0.01; standing still the timer runs down), and for 120 ticks after;
//!   else a point 6 away at a random way (±90° of its facing, 0..30° up) picked every 180..300 ticks. The turn asked
//!   for from 2 over its feet: the yaw off its facing within ±90° (the head 0.6 of it, the body 0.4), the pitch within
//!   ±30° (Oozla's seller ±10°) on the head; aimed (crate::moby::joint_aim::aim) at 0.04 while it follows Ratchet,
//!   else 0.02, damped 0.3. With the actors' big heads (cheat byte 0, `0x1a7a48`) the head's record asks for 2.75.
//!   The pvars: the head's aim record `+0x00`, the body's `+0x80`, the point `+0x100`, then the two timers (L08 / L17 /
//!   L18 / L01: words `+0x114` / `+0x118`; L09: half-words `+0x110` / `+0x112`) and the idle timer (`+0x11e`; L09
//!   `+0x114`).
//! * **The actors' heads** ([`actor_heads`], L04 `0x424898` = L09 `0x40b6f8` = L17 `0x41f7b8` = L18 `0x441b28`, 2.75;
//!   L08 `0x436ff8` = L19 `0x42aec0`, 2.3): in a scene with cheat byte 0, the level's one head node (`0x1b2510`, L08
//!   `0x1b22e0`) hung on joint list 0 of the first scene actor of the NPC's class (`0x312f80`), at the scale.
//!
//! The consumers: L04 crate::moby::classes::level04::talkers (0x0c38, 0x0d3e, 0x0da8), L06
//! crate::moby::classes::level06::host (0x0ccf), L08 level08::buyer, L09 level09::talker, L11 level11::sellers
//! (0x0e95, 0x0b7c), L17 / L18 their sellers, Oozla's 0x0ace (crate::moby::classes::oozla::update_0ace).
//!
//! Not ported: the talk record (`0x3259b0` → `0x325980(0x3259d8(m))`, G-CLS-008: the moby's place in the level's
//! talk list `0x1c5d20` from `0x28ead0[level]`, its record `0x139a34 + 0x10·i` set to 1; the missions' kinds 4 / 5 read
//! it and are not ported either); the level's "NPC busy" word each sets while its scene plays (L04 `0x1b2228`, L06
//! `0x1b2568`, L08 `0x1b2728`, L09 `0x1b2528`, L11 `0x1b3368`, L17 `0x1b2968`, L18 `0x1b24e8`; read by L00 `0x2fa968`,
//! `0x30df80`, `0x30e200`, none ported); the autosave after a sale (`0x30d658(0, −1)`, as everywhere in the port).

use super::{countdown, joint_aim, Ctx, MobyWorld};
use crate::hero::physics::{add, add_rot, atan2, dist, dist2, len, len2, rot_dist, sub, sub_rot, V3};

/// The level globals the NPCs keep (reset with the level).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    /// L09 `$gp+0x2570` (`0x1b1560`): Dobbo's 0x0e5e has asked for its arrival scene 0x66 this visit, or will not
    /// (crate::moby::classes::level09::talker).
    pub arrival_done: bool,
    /// L21 `$gp−0x3b0` (`0x1aec40`): the museum's commentary kinds heard, a bit each (cleared by its set-up;
    /// crate::moby::classes::level21::commentary).
    pub museum_heard: i32,
}

/// A moby index in the pvars at `off` (as the level data gives them: −1 none).
pub fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&k| k < w.mobys.len()) }

/// The game's flag byte `0x1395b8 + k` set / to 1.
pub fn flag(ctx: &Ctx, k: usize) -> bool { ctx.game.flags.get(k).is_some_and(|&b| b != 0) }
pub fn set_flag(ctx: &mut Ctx, k: usize) { if let Some(b) = ctx.game.flags.get_mut(k) { *b = 1; } }

/// Scene `n` seen (the bit `0x139768 + (n & !3)`, `1 << (n & 31)`).
pub fn seen(ctx: &Ctx, n: usize) -> bool { ctx.game.scenes_seen.get(n).copied().unwrap_or(false) }

/// `0x3258c0(r, moby, n)`: Ratchet can talk to `id`: not in states 0x1c / 0x31, with health, on the ground (`0x18a160`
/// ticks, no air ticks `0x18a16e`), not in movement group 3, the game in play (`$gp−0x60b0`), within 2·r of it (3D),
/// the level running `n` frames or more (`0x1b2088`, L00's `0x1b1948`).
pub fn can_talk(w: &MobyWorld, id: usize, ctx: &Ctx, r: f32, n: i32) -> bool {
    let h = &*ctx.hero;
    if h.state == 0x1c || h.state == 0x31 || h.hp == 0 || h.grounded_ticks == 0 || h.air_ticks != 0 || h.group == 3 { return false; }
    if ctx.game.scenes.mode != crate::scene::PLAY { return false; }
    if r + r < dist(w.mobys[id].pos, h.pos) { return false; }
    n <= w.frame
}

/// `0x325830(text, args.., −1)`: the text by id formatted with `args` into the second message box for a tick.
pub fn say(ctx: &mut Ctx, text: i32, args: &[i32]) {
    use crate::msgbox::Request;
    let r = match *args {
        [a, b, ..] => Request::SecondFormatted2 { text, n: [a, b], ticks: Some(1) },
        [a] => Request::SecondFormatted { text, n: a, ticks: Some(1) },
        [] => Request::SecondFormatted { text, n: 0, ticks: Some(1) },
    };
    ctx.game.messages.push(r);
}

/// While Ratchet can talk to `id` ([`can_talk`] within `r`, from the tenth frame): the text in the second box
/// ([`say`]) and the talk record marked (`0x3259b0`, not ported). True when he can.
pub fn offer(w: &MobyWorld, id: usize, ctx: &mut Ctx, r: f32, text: i32, args: &[i32]) -> bool {
    if !can_talk(w, id, ctx, r, 10) { return false; }
    say(ctx, text, args);
    true
}

/// Shown (`+0x31` 1, mode bit 1 off).
pub fn show(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.drawn = 1;
    m.mode &= !1;
}

/// Hidden (`+0x31` 0, mode bit 1).
pub fn hide(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.drawn = 0;
    m.mode |= 1;
}

/// Hidden while the game is in the scene mode (`0x1a8f00` = 2).
pub fn hide_in_scene(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE { hide(w, id); }
}

/// `0x2b9e68(cuboid + 0x30, cuboid + 0x70, 0, 1)`: Ratchet put at cuboid `c`'s place and angles, idle.
pub fn put_hero(ctx: &mut Ctx, c: i32) {
    if let Some((pos, rot)) = ctx.level.cuboid_place(c) {
        ctx.hero.teleport(pos, rot);
        ctx.hero.request_state(0, true);
    }
}

/// L00 `0x371b60`: the scene asked for starts at once (module docs).
pub fn cut_in(g: &mut crate::game::GameState) {
    g.scenes.zoom = f32::from_bits(0x3f99_999a);
    if g.scenes.fade < 1.0 { g.blocking_fade(6); }
}

/// A timer in the pvars: a word or a half-word.
#[derive(Clone, Copy, Debug)]
pub enum Timer {
    Word(usize),
    Half(usize),
}

impl Timer {
    fn get(self, w: &MobyWorld, id: usize) -> i32 {
        match self { Timer::Word(o) => w.mobys[id].pv_i(o), Timer::Half(o) => w.mobys[id].pv_s(o) as i32 }
    }
    fn set(self, w: &mut MobyWorld, id: usize, v: i32) {
        match self { Timer::Word(o) => w.mobys[id].set_pv_i(o, v), Timer::Half(o) => w.mobys[id].set_pv_s(o, v as i16) }
    }
    /// `0x2eeab0` / `0x2eeae0`: one down; true when it was 0 or reaches it.
    fn run(self, w: &mut MobyWorld, id: usize) -> bool {
        let mut v = self.get(w, id);
        let done = countdown(&mut v);
        self.set(w, id, v);
        done
    }
}

/// [`idle`] and [`look`]'s layout: the looking sequence, the fidget sequence (none: the timer only restarts), the
/// idle timer (a half-word), the two look timers, the pitch's limit (±30° but Oozla's seller's ±10°: [`PITCH`]).
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub look_seq: u8,
    pub fidget: Option<u8>,
    pub idle: usize,
    pub seen: Timer,
    pub glance: Timer,
    pub pitch: f32,
}

/// The pitch's usual limit (`0x3f060a93`, 30°).
pub const PITCH: f32 = 0.523_598_85;

/// The random point's distance (`0x325100(6, ..)`) and a degree.
const REACH: f32 = 6.0;
const DEG: f32 = 0.017_453_294;

/// The idle (module docs). Not in state 0.
pub fn idle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, l: &Layout) { idle_on(w, id, ctx, l, Timer::Half(l.idle)) }

/// [`idle`] with its idle timer a word (L04's 0x0c38 / 0x0da8: `+0x11c`, `0x30f6e8`).
pub fn idle_word(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, l: &Layout) { idle_on(w, id, ctx, l, Timer::Word(l.idle)) }

fn idle_on(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, l: &Layout, timer: Timer) {
    use crate::moby::classes::props::{seq, wrapped};
    if w.mobys[id].state == 0 { return; }
    let s = seq(w, id, ctx);
    if s == l.look_seq && timer.run(w, id) {
        let t = ctx.game.rng.randf(1200.0, 2400.0) as i32;
        timer.set(w, id, t);
        if let Some(f) = l.fidget { if s != f { ctx.host.set_anim(&w.mobys[id], f, 0, 10); } }
    } else if wrapped(w, id, ctx) && s != l.look_seq {
        ctx.host.set_anim(&w.mobys[id], l.look_seq, 0, 10);
    }
}

/// The look (module docs): the asks set while it is in its looking sequence, then both aims.
pub fn look(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, l: &Layout) { look_with(w, id, ctx, l, true, true) }

/// [`look`] with the pitch's ask left alone (`pitch_ask` false: L04's 0x0c38 works it out but never stores it) or no
/// asks at all (`track` false: L04's 0x0d3e before its first scene), the aims still run.
pub fn look_with(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, l: &Layout, pitch_ask: bool, track: bool) {
    use std::f32::consts::FRAC_PI_2;
    const POINT: usize = 0x100;
    const HEAD_SCALE: usize = 0x70;
    let mut speed = f32::from_bits(0x3ca3_d70a);
    if track && crate::moby::classes::props::seq(w, id, ctx) == l.look_seq {
        let (pos, yaw) = (w.mobys[id].pos, w.mobys[id].rot[2]);
        let hero = ctx.hero.pos;
        let centre = [hero[0], hero[1], hero[2] + 1.0];
        if dist2(pos, hero) < 8.0 && rot_dist(yaw, atan2(centre[0] - pos[0], centre[1] - pos[1])) < FRAC_PI_2 {
            if 0.01 < len(ctx.hero.disp) { l.seen.set(w, id, 120); } else { l.seen.run(w, id); }
        } else if l.seen.get(w, id) != 0 {
            l.seen.set(w, id, 0);
            w.mobys[id].set_pv_v(POINT, centre);
        }
        if l.glance.run(w, id) {
            let t = ctx.game.rng.randf(180.0, 300.0) as i32;
            l.glance.set(w, id, t);
            let a = add_rot(yaw, ctx.game.rng.randf(-90.0, 90.0) * DEG);
            let p = ctx.game.rng.randf(0.0, 30.0) * DEG;
            w.mobys[id].set_pv_v(POINT, add(crate::moby::classes::shot::polar(REACH, a, p), pos));
        }
        let target = if l.seen.get(w, id) != 0 {
            speed = f32::from_bits(0x3d23_d70a);
            centre
        } else {
            w.mobys[id].pv_v(POINT)
        };
        let d: V3 = sub(target, [pos[0], pos[1], pos[2] + 2.0]);
        let lim = f32::from_bits(0x3fc9_0fdc);
        let turn = sub_rot(atan2(d[0], d[1]), yaw).clamp(-lim, lim);
        let pitch = (-atan2(len2(d), d[2])).clamp(-l.pitch, l.pitch);
        let m = &mut w.mobys[id];
        if pitch_ask { m.set_pv_f(joint_aim::ASK + 4, pitch); }
        m.set_pv_f(joint_aim::ASK + 8, turn * 0.6);
        m.set_pv_f(0x80 + joint_aim::ASK + 8, turn * 0.4);
    }
    if ctx.game.cheat(crate::cheats::cheat::ACTOR_HEADS) { w.mobys[id].set_pv_f(HEAD_SCALE, 2.75); }
    let damp = f32::from_bits(0x3e99_999a);
    joint_aim::aim(w, id, 0, id, 0, speed, damp);
    joint_aim::aim(w, id, 0x80, id, 1, speed, damp);
}

/// The actors' heads (module docs): in a scene with cheat byte 0, the first actor of `id`'s class takes the level's
/// head node on its joint list 0 at `scale`, once.
pub fn actor_heads(w: &mut MobyWorld, id: usize, ctx: &Ctx, scale: f32) {
    if ctx.game.scenes.mode != crate::scene::SCENE || !ctx.game.cheat(crate::cheats::cheat::ACTOR_HEADS) { return; }
    let Some(p) = ctx.game.scenes.player.as_ref() else { return };
    let class = w.mobys[id].o_class;
    let Some(&(a, _)) = p.actors.iter().find(|&&(a, _)| a != id && w.mobys.get(a).is_some_and(|m| m.o_class == class)) else { return };
    let m = &mut w.mobys[a];
    if !m.joint_scales.iter().any(|j| j.0 == 0) { m.joint_scales.push((0, scale)); }
}
