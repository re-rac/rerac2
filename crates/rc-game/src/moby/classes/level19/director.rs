//! Level 0x13's scene-driven NPCs (Grelbin; L19 addresses) on the shared NPC system crate::moby::talk, and the level's
//! census.

use crate::moby::director::{gone, solid, Tally};
use crate::moby::talk::{self, flag, link, seen, set_flag, Layout, Timer};
use crate::moby::{joint_aim, Ctx, MobyWorld};

pub const TALKER: i16 = 0x1254;
pub const TRADER: i16 = 0x1313;

/// The talker's idle and look: looking in sequence 1, fidgeting in 2.
const LOOK: Layout = Layout { look_seq: 1, fidget: Some(2), idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
const R: f32 = 2.5;

/// The scenes: the talker's (0x98, which also opens the trader) and the trader's two deals (0x9a, 0x99).
const SCENE_TALK: usize = 0x98;
const SCENE_DEAL_A: usize = 0x9a;
const SCENE_DEAL_B: usize = 0x99;
/// Its partner (`+0x124`, −1 none), deleted with it once scene 0x9a is seen.
const PARTNER: usize = 0x124;
/// The skill point the level's census gives (`0x1a7a6e`) and the classes it counts (and the trader's "left" counts):
/// the ice 0x11c7, 0x1266 and the frozen 0x0f57.
const POINT: usize = 6;
const COUNTED: [i16; 3] = [0x11c7, 0x1266, 0x0f57];
/// The class whose first scene actor the talker gives the level's head node (`0x1b2260`) with the actors' big heads,
/// and the scale (the trader its own class's, the same scale).
const HEAD_CLASS: i16 = 0x0e3e;
const HEAD_SCALE: f32 = f32::from_bits(0x3ff3_3333);

/// 0x1254 (`0x42be18`, the decomp's v2.00 `func_US2_L19_0042BE14`): an NPC who plays scene 0x98 when Ratchet comes
/// to it.
/// * **0**: update distance 0xff. Scene 0x9a seen (`0x139800` bit 26): its partner `+0x124` and it deleted; scene
///   0x98 seen (bit 24): it deleted. Else its shadow slab (0, 0.2; `0x31b670`) and 1.
/// * **1**: Ratchet can talk to it (crate::moby::talk::can_talk, within 2·2.5, from the tenth frame; the talk record
///   `0x32d3f8` not ported): scene 0x98 queued and the chain started (`0x2f24f8` / `0x2f2640` = L00 `0x2f4ee8` /
///   `0x2f5030`); then 2.
/// * **2**: hidden while the scene plays; once it is over (`0x36f938` = L00 `0x371ba8`), shown and deleted.
/// * Out of state 0 each tick: the level's census ([`census`]), the idle and the look (crate::moby::talk::idle /
///   look). In a scene with the actors' big heads, the first scene actor of 0x0e3e takes the head node at 1.9
///   (`0x42bd30`).
///
/// Not ported: the level's count of the census classes (`0x42bba0` into `$gp+0x2f68`) when `0x1b2488` is 5 and at its
/// set-up (the trader counts them live); the NPC-busy word `0x1b26a8` (crate::moby::talk); with the big heads its own
/// head asks 2.75 (talk::look) where the game asks 1.9 (`+0x70`).
pub fn update_1254(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    actor_heads_of(w, ctx, HEAD_CLASS);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            if seen(ctx, SCENE_DEAL_A) {
                if let Some(p) = link(w, id, PARTNER) { w.delete(p); }
                w.mobys[id].set_pv_i(PARTNER, -1);
                return w.delete(id);
            }
            if seen(ctx, SCENE_TALK) { return w.delete(id); }
            let m = &mut w.mobys[id];
            m.set_state(1, -1);
            m.shadow_slab = [0.0, f32::from_bits(0x3e4c_cccd)];
        }
        1 => {
            if talk::can_talk(w, id, ctx, R, 10) && ctx.game.scenes.queue.push(SCENE_TALK as i32) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            if !ctx.game.scenes.in_scene() {
                talk::show(w, id);
                return w.delete(id);
            }
        }
        _ => {}
    }
    if w.mobys[id].state != 0 {
        census(w, ctx);
        talk::idle(w, id, ctx, &LOOK);
    }
    talk::look(w, id, ctx, &LOOK);
}

/// The trader's prices (`0x28e988` / `0x28e990`; the other column `0x28e98c` / `0x28e994`, read with the byte
/// `0x1a7c97`, which is never 0, holds the same).
const PRICE_A: i32 = 9;
const PRICE_B: i32 = 16;
/// A moonstone's worth in bolts (10 times in the challenge mode, `0x2f2838` = L00 `0x2f51a0`: `0x1a7a12`).
const WORTH: i32 = 3000;
/// The flags (`0x1395b8 +`): the second deal done (`0x139641`); the two the help after it reads (`0x13963c` /
/// `0x13963e`).
const DEALT: usize = 0x89;
const F_84: usize = 0x84;
const F_86: usize = 0x86;
/// Its pvar: the prop it hides in the first deal's scene and deletes after it (`+0x80`, −1 none).
const PROP: usize = 0x80;

fn stones(ctx: &Ctx) -> i32 { ctx.game.moonstones }

/// `0x42bba0`'s count, live (as crate::moby::classes::level08::buyer's): the census classes left.
fn left(w: &MobyWorld) -> i32 { w.mobys.iter().filter(|m| COUNTED.contains(&m.o_class) && m.state != 0xfe && m.state != 0xfd).count() as i32 }

/// The bolts' sound: sound 0 of one of the bolt classes 13..17 (`0x35e4a8(0, 0, m, 0x3193d0(13, 17))`).
fn bolt_sound(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = ctx.game.rng.rand_range(13, 17) as i16;
    ctx.host.play_sound_of(&w.mobys[id], class, 0, 0);
}

fn triangle(ctx: &Ctx) -> bool { ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 }

/// 0x1313 (`0x42c4e0`, no decomp C): Grelbin's moonstone trader (the moonstones `0x1a7a24`,
/// crate::game::GameState::moonstones). After the talker's scene 0x98 it trades two deals (9 stones, then 16) for its
/// scenes, then buys the rest for bolts.
/// * **Each tick**, first: the actors' big heads of its class in a scene (`0x42c3f8`, 1.9).
/// * **0** (update distance 0xff; nothing while a scene is asked for or plays): with scene 0x9a seen it is shown and
///   solid (mode & ~0x41 | 0x1000, its class's collision) and goes on — the second deal not done: 4 short of 16, else 5;
///   done: 7 with no stones, else 8 — with its shadow slab (0, 0.2); with only 0x98 seen, shown and solid, 1 short of 9,
///   else 2; with neither, hidden and passed through (mode & ~0x1000 | 0x41, no collision).
/// * **1** (short; enough → 2): while Ratchet can talk to it (2.5) the text 0x132e with 9.
/// * **2** (enough; short → 1): while he can, the text 0x132f with 9; △ queues the deal's scene 0x9a and starts it; 3.
/// * **3**: hidden in the scene mode, and its prop `+0x80` too; after the scene: shown, 9 stones off, its group's timers
///   1 (`0x31a340`), the prop deleted, the autosave (`0x30af90` = L00 `0x30d658(0, −1)`, not ported); 4 with 16 or
///   fewer, else 5.
/// * **4** / **5**: as 1 / 2 with 16, the texts 0x1330 / 0x1331 and the scene 0x99; 6.
/// * **6**: hidden in the scene mode; after it: 16 stones off, shown, its group's timers 1, the flag `+0x89`, the
///   message 0x1294 (`0x3249b8` = L00 `0x327448`), the help text 0xa10 (slot 0x67) with both flags `+0x84` and `+0x86`,
///   else 0xa0f (slot 0x66); the autosave; 7 with no stones, else 8.
/// * **7** (none; some → 8): while he can talk to it the text 0x1334 with 0 when none of the census classes is left,
///   else 0x1332.
/// * **8** (some; none → 7): while he can, the text 0x1333 with the stones and their worth (3000 bolts each, 30 000 in
///   the challenge mode); △ pays them (the bolts, the bolt counter shown 180 ticks — `0x30b248` = L00 `0x30d910` — the
///   stones 0, in the challenge mode the bolt sound); 9.
/// * **9**: its ticks counted (to 0x7ff8), the bolt sound on every fourth of the ticks 41..89; while he can talk to it
///   the text 0x1334 with the census classes left; when he cannot, 7.
/// * Then the level's census ([`census`]) and its head's aim (list 0, at 0.02 damped 0.3; with the big heads `+0x70`
///   2.75): it asks no turn, so the head only springs back.
///
/// Not ported: the NPC-busy word `0x1b26a8` set while its scenes play; the talk record (`0x32d3f8`); the autosaves.
pub fn update_1313(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, HEAD_SCALE);
    trader(w, id, ctx);
    if gone(w, id) { return; }
    census(w, ctx);
    if ctx.game.cheat(crate::cheats::cheat::ACTOR_HEADS) { w.mobys[id].set_pv_f(0x70, 2.75); }
    joint_aim::aim(w, id, 0, id, 0, f32::from_bits(0x3ca3_d70a), f32::from_bits(0x3e99_999a));
}

fn trader(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let next = match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            if ctx.game.scenes.in_scene() { return; }
            if seen(ctx, SCENE_DEAL_A) {
                solid(w, id, true);
                let next = if !flag(ctx, DEALT) {
                    if stones(ctx) < PRICE_B { 4 } else { 5 }
                } else if stones(ctx) < 1 { 7 } else { 8 };
                let m = &mut w.mobys[id];
                m.set_state(next, -1);
                m.shadow_slab = [0.0, f32::from_bits(0x3e4c_cccd)];
                return;
            }
            if !seen(ctx, SCENE_TALK) { return solid(w, id, false); }
            solid(w, id, true);
            if stones(ctx) < PRICE_A { 1 } else { 2 }
        }
        1 => {
            if PRICE_A <= stones(ctx) { 2 } else { talk::offer(w, id, ctx, R, 0x132e, &[PRICE_A]); return; }
        }
        2 => {
            if stones(ctx) < PRICE_A { 1 } else {
                if !talk::offer(w, id, ctx, R, 0x132f, &[PRICE_A]) || !triangle(ctx) { return; }
                ctx.game.scenes.queue.push(SCENE_DEAL_A as i32);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                3
            }
        }
        3 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.mode == crate::scene::SCENE {
                if let Some(p) = link(w, id, PROP) { talk::hide(w, p); }
            }
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            ctx.game.moonstones -= PRICE_A;
            let group = w.mobys[id].group;
            if group != -1 { w.poke_group(group as i32, 1); }
            if let Some(p) = link(w, id, PROP) {
                w.delete(p);
                w.mobys[id].set_pv_i(PROP, -1);
            }
            if (stones(ctx) as u32) <= PRICE_B as u32 { 4 } else { 5 }
        }
        4 => {
            if PRICE_B <= stones(ctx) { 5 } else { talk::offer(w, id, ctx, R, 0x1330, &[PRICE_B]); return; }
        }
        5 => {
            if stones(ctx) < PRICE_B { 4 } else {
                if !talk::offer(w, id, ctx, R, 0x1331, &[PRICE_B]) || !triangle(ctx) { return; }
                ctx.game.scenes.queue.push(SCENE_DEAL_B as i32);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                6
            }
        }
        6 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            ctx.game.moonstones -= PRICE_B;
            talk::show(w, id);
            let group = w.mobys[id].group;
            if group != -1 { w.poke_group(group as i32, 1); }
            set_flag(ctx, DEALT);
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1294, ticks: None });
            let mut n = if flag(ctx, F_84) { 1 } else { 2 };
            if flag(ctx, F_86) { n -= 1; }
            if n < 1 { ctx.game.help(0xa10, 0x67); } else { ctx.game.help(0xa0f, 0x66); }
            if stones(ctx) < 1 { 7 } else { 8 }
        }
        7 => {
            if 0 < stones(ctx) { 8 } else {
                if left(w) == 0 { talk::offer(w, id, ctx, R, 0x1334, &[0]); } else { talk::offer(w, id, ctx, R, 0x1332, &[]); }
                return;
            }
        }
        8 => {
            let s = stones(ctx);
            if s < 1 { 7 } else {
                let each = if ctx.game.challenge { WORTH * 10 } else { WORTH };
                let worth = s.wrapping_mul(each);
                if !talk::offer(w, id, ctx, R, 0x1333, &[s, worth]) || !triangle(ctx) { return; }
                ctx.game.bolts = (ctx.game.bolts as i32).wrapping_add(worth) as u32;
                ctx.game.moonstones = 0;
                ctx.game.show_bolts = Some(0xb4);
                if ctx.game.challenge { bolt_sound(w, id, ctx); }
                9
            }
        }
        9 => {
            let m = &mut w.mobys[id];
            m.state_ticks = (m.state_ticks as u16).wrapping_add(1).min(0x7ff8) as i16;
            let t = m.state_ticks as u16;
            if (0x29..=0x59).contains(&t) && t & 3 == 0 { bolt_sound(w, id, ctx); }
            let n = left(w);
            if talk::offer(w, id, ctx, R, 0x1334, &[n]) { return; }
            7
        }
        _ => return,
    };
    w.mobys[id].set_state(next, -1);
}

/// `0x42bbf8`: the level's census (crate::moby::director::Census, its cursor and count in `$gp+0x2f6c` / `+0x2f70`):
/// the ice and frozen mobys left; a pass with none is skill point 6. True on that pass.
pub fn census(w: &mut MobyWorld, ctx: &mut Ctx) -> bool {
    if ctx.game.skill_points[POINT] != 0 { return false; }
    let mut c = w.level19.census;
    let empty = c.step(w, false, |m| if COUNTED.contains(&m.o_class) && m.state != 0xfe && m.state != 0xfd { Tally::Count } else { Tally::Pass });
    w.level19.census = c;
    if empty { ctx.game.award_skill_point(POINT); }
    empty
}

/// `0x42bd30`: in a scene with the actors' big heads (cheat byte 0), the first scene actor of `class` takes the
/// level's head node (`0x30fe40(actor, 0, 0x1b2260)`, unless the node is in use) at 1.9.
fn actor_heads_of(w: &mut MobyWorld, ctx: &Ctx, class: i16) {
    if ctx.game.scenes.mode != crate::scene::SCENE || !ctx.game.cheat(crate::cheats::cheat::ACTOR_HEADS) { return; }
    let Some(p) = ctx.game.scenes.player.as_ref() else { return };
    let Some(&(a, _)) = p.actors.iter().find(|&&(a, _)| w.mobys.get(a).is_some_and(|m| m.o_class == class)) else { return };
    let m = &mut w.mobys[a];
    if !m.joint_scales.iter().any(|j| j.0 == 0) { m.joint_scales.push((0, HEAD_SCALE)); }
}
