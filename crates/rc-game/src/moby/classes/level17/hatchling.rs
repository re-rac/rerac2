//! Smolg's hatchlings and their pods (L17 addresses).
//!
//! **0x0b88** ([`update_0b88`], `0x3e47a8`, 88 placed; the decomp's v2.00 `func_US2_L17_003E47A8`, its helpers
//! v1.01 `func_L17_003E41A8` / `003E4560` / `003E47E8` at our `0x3e57b0` / `0x3e5b68` / `0x3e5df0`): a small critter
//! that waits in a pod, hatches when Ratchet comes near, runs at him and bites, backs off after each bite, and
//! otherwise wanders about its home. It is L04's skitter (crate::moby::classes::level04::skitter) grown: the same
//! framework (crate::moby::ai) and walker (crate::moby::crawler, block `+0x140`), its words `+0xf0` / `+0x100` on,
//! with a back-off, a turn to Ratchet out of the pod, and pods that hatch it. Health block `+0x20`, the flash
//! `+0x50`, reaction `+0x60` (a hurt goes to state 7, sub 3), the wander block `+0x110` (home, radius 0.5, speed
//! `+0x124`), target block `+0x190` (its moby `+0x1a0`).
//!
//! **Its words:** `+0x3d0` home (kept once set: `+0x3dc` 12439.4 marks it), `+0x3e0` the back-off's goal, `+0x3f8`
//! its rest (seconds) and `+0x3fc` the rest timer, `+0x400` / `+0x402` the wander and leap timers (s16), `+0x404`
//! found (s16), `+0x406` alarmed, `+0x408` its target (the target block's moby, else Ratchet's), `+0x40c` its run
//! speed a second, `+0x410` the turn speed, `+0x414` the heading offset and `+0x418` its timer, `+0x41c` the wait,
//! `+0x420` / `+0x424` the floor moby it rides and its fall, `+0x428` its pod (an index), `+0x42c` the moby it waits
//! for, `+0x430` its bite (the health block's damage byte), `+0x43c` its pvar group, `+0x440` a cool-down
//! (counted down), `+0x444` its fence path (−1 none), `+0x448` a bite that landed, `+0x450` its 0x0929 pod (told
//! about it every tick: the pod's `+0xa4`), `+0x454` hidden by a scene.
//!
//! **The set-up** (state 0, [`head`]): on the ground (0.5 over), its class's scale, level; in a pod hidden without
//! collision 1.4 over it; the reaction (state 7 sub 3, sequences 9 / 12 / 15, keys 0.4167 / 0.05, gravities 1/327 /
//! 1/180 / 1/1800, speeds 0.35 / 0.35 / 0.4); the walker (radius 0.5 `$gp+0x1c68`, steps 2 / 2.5, most its run
//! speed); its home and the wander's; the wander timer 120..700; Ratchet's colour words; its bite; idle (1, sub 0,
//! sequence 0).
//!
//! **Each tick:** nothing while a scene plays (drawn: hidden, `+0x454` set; shown again after). Its 0x0929 pod told.
//! Drawn within 24 of the camera its shadow (range 0x16); a hit (flags 0x230000) of the kind 8: 9 (`+0xbc` 4); the
//! frame's damage — the spin's (kind 5) knock 0.8378 up — alarms its group's hatchlings (`0x3e5e68`); the search
//! (15 across, 3 high, from where it is).
//!
//! **Subs** (`+0x95`): **0** calm, unless it waits (a pod `+0x428` ready to hatch, [`pod_ready`], and `+0x42c` a
//! live other moby): the wait down; found (and the cool-down over) the cool-down (`0x3e4708`, 87 ticks while its
//! pvar group has members) and out: from a pod (ready, else nothing) the pod hatches ([`pod_open`]) and it is shown
//! with collision — a 0x0929 rides it (8 sub 3, sequence 9 at once, the pod's heading), else 10 sub 2 (sequence 2);
//! with none 10 sub 2. Alarmed: resting (2, sub 1, 0.8..1.2 × its rest). **1** resting: found, the cool-down and
//! 10 sub 2 (sequence 2); rested, idle (1, sub 0, sequence 0). **2**: not found, one tick in 19, resting. **3** the
//! hurt (its first tick, a sequence playing backwards forward again): off a ride the flight; dead its bolts and the
//! fade; over: found the cool-down and the run (4, sub 2, sequence 9), else resting (the wander timer 120..700). More
//! than 10 over its home, under z 10 (or on level 11 under 75): its bolts, gone.
//!
//! **States** (`+0x20`): **1** / **2** wandering (`0x325a88`(0.35, 0.25) = L00 `0x323958`, crate::moby::ai::wander;
//! its speed toward 2.5 / 4 a second (`$gp+0x1c70` / `+0x1c74`) by 0.0019 a tick, none in the timer's last 20
//! ticks; walking sequence 9 (state 1 at 0.8); out of time an idle (1: 0..1, 2: 5, half of those 4), at their end
//! the timer (120..500 / 120..200) and the walk). **3** rearing (turned to its target, at the end 4). **4** the run
//! (the offset's timer 80..100 on its first tick; within 1 (`$gp+0x1c6c`) the bite: 5 sub 2, sequence 6 from frame 2,
//! no offset; at the offset timer's end a new offset ±15..30°, the timer 150..200, and the bite the same). **5** the
//! bite (turned; frame 8 its sound 3, frame 10 a 1 sphere about it of its bite (flags 0x4000001), landed into
//! `+0x448`; at the end or frame 11, him out of reach: the back-off (6, sequence 9) — the original's test for a
//! second bite (11) compares `(d == 0) as f32` with the reach and never passes). **6** the back-off (`0x32fb68` =
//! L02 `0x331440` with the options 0x10): on its first tick its goal 10 from its target through it, the sequence
//! backwards; at sequence 2's end sequence 9 forward; at sequence 9's end the run (4, sequence 9), or, a bite having
//! landed, 11 (sequence 7). **8** riding its pod (out 6 from home: 4). **9** drawn in ([`pulled`]). **10** out of the
//! pod (turning to Ratchet 6° a tick, `0x31f030`; at its sequence's end 4, sequence 9). **11** a taunt (at its end
//! 4, sequence 9). Not 6 / 8 / 9 it walks ([`walk`]).
//!
//! **0x0920** / **0x0921** ([`update_0920`], `0x3c71b0`, 11 placed; the decomp's `func_L17_003C5C28`): a pod. In
//! state 0, with `+0x90` set, it falls (`$gp+0x12e8` 30 / 3600 a tick²) to the ground 0.5 under its line (the floor
//! moby `+0x9c`). Opened (state 1, by its hatchling, sequence 1 at once): at sequence 1's end it becomes 0x0921
//! (its class's model and collision), sequence 0 at once, state 2, and with `+0x90` set it goes 600 ticks on. Every
//! tick its move into its platform block `+0x50` (crate::moby::platform, `0x324660` = L00 `0x322668`).
//!
//! The engine functions with the port's: `0x31c380` = L00 `0x31b8d0` (set_state), `0x31c0a0` = L00 `0x31b5f0`
//! (set_anim, crate::moby::classes::props::set_seq), `0x31bfc8` (the sequence at once), `0x32d7b8` / `0x32d7e0` =
//! L00 `0x32a7c8` / `0x32a7f0` (the first tick of the state / sub), `0x32a160` = L00 `0x327e38` (take_damage),
//! `0x32aea8` = L00 `0x328b80` (flight), `0x32dd28` = L00 `0x32ad38` (find_target), `0x31cbf0` = L00 `0x31c078`
//! (turn_to), `0x31d4d0` / `0x31da08` = L04 `0x33dda8` / `0x33e2e0` (crate::moby::crawler), `0x325d28` = L00
//! `0x323bf8` (reached), `0x31e3a8` = L00 `0x31d180` (attack_sphere), `0x31e048` = L00 `0x31ce20` (ground_z),
//! `0x346ac0` = L00 `0x343678` (fence), `0x321e48` = L00 `0x3201e8` (flash), `0x330be8` (L00 `0x32dcf0`, senses),
//! `0x31c450` / `0x31c490` / `0x31c4e8` (crate::rng), `0x2ef8d8` / `0x2ef908` (crate::moby::countdown), `0x3f2b58`
//! = L01 `0x3eb1d0` (crate::moby::classes::death_fade), `0x312188` = L00 `0x311688` (delete).
//!
//! Not ported: the global gate `0x1ca140`, the floor mobys it rides (`0x324050` / `0x324020` / `0x3257b8`, as the
//! skitter's), `+0x310` (→ `$gp+0x1c78`, read by nothing here), what
//! draws it in (state 9: the moby `0x1afb5c`; it goes at once, as the skitter's), and a 0x0929 pod's debris
//! (`0x3292e8` = L03 `0x327c38`, five of class 0x0931) when it hatches; 0x0929's own update (`0x3c87e8`) is not
//! here, and the words of it read ([`pod_ready`]: `+0x98`, `+0x9c`, `+0xa0`) are taken as it leaves them.

use crate::hero::physics::{add, add_rot, approach, approach_angle, atan2, dist2, set_len, sub, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, crawler, mode, platform, Ctx, MobyWorld};

pub const HATCHLING: i16 = 0x0b88;
pub const POD: i16 = 0x0920;
pub const POD_OPEN: i16 = 0x0921;
/// The other pod (not ported here: `0x3c87e8`).
const POD_B: i16 = 0x0929;

const FLASH: usize = 0x50;
const WANDER: usize = 0x110;
const WALK: usize = 0x140;
/// The head aim (crate::moby::joint_aim::head).
const HEAD_AIM: usize = 0x350;
const HOME: usize = 0x3d0;
const BACK: usize = 0x3e0;
const REST: usize = 0x3f8;
const T_REST: usize = 0x3fc;
const T_WANDER: usize = 0x400;
const T_LEAP: usize = 0x402;
const FOUND: usize = 0x404;
const ALARMED: usize = 0x406;
const RUN: usize = 0x40c;
const TURN_V: usize = 0x410;
const OFFSET: usize = 0x414;
const T_OFFSET: usize = 0x418;
const WAIT: usize = 0x41c;
const FLOOR: usize = 0x420;
const FALL: usize = 0x424;
const SEAT: usize = 0x428;
const WAIT_FOR: usize = 0x42c;
const BITE: usize = 0x430;
const GROUP: usize = 0x43c;
const COOL: usize = 0x440;
const FENCE: usize = 0x444;
const LANDED: usize = 0x448;
const POD_LINK: usize = 0x450;
const SCENE: usize = 0x454;
const PVARS: usize = 0x458;
/// `$gp+0x1c68`: its radius; `+0x1c6c` its reach; `+0x1c70` / `+0x1c74` its wander speeds (a second).
const RADIUS: f32 = 0.5;
const REACH: f32 = 1.0;
const WANDER_1: f32 = 2.5;
const WANDER_2: f32 = 4.0;
/// The home marker (`+0x3dc`).
const MARK: f32 = 12439.4;

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(FOUND) == 1 }
fn cool(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(COOL) == 0 }
fn tick_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_i(off);
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(off, t);
    out
}
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_s(off) as i32;
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
    out
}
fn live(w: &MobyWorld, k: i32) -> Option<usize> {
    usize::try_from(k).ok().filter(|&k| w.mobys.get(k).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}
/// `mode & 0xffbe` and its class's collision.
fn show(w: &mut MobyWorld, id: usize) {
    let has = w.class(HATCHLING).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode &= 0xffbe;
    m.has_collision = has;
}
/// `0x327928` (L00 `0x32a7f0`): the first time since the last sub-state change.
fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}
/// Its target's place (`+0x408`): the target block's moby, else Ratchet's.
fn target_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    let m = &w.mobys[id];
    ai::link(m, ai::target(m) + 0x10).or(ctx.hero_moby).and_then(|k| w.mobys.get(k)).map_or(ctx.hero.pos, |t| t.pos)
}
/// Resting: 2 sub 1, the rest timer 0.8..1.2 × its rest seconds.
fn rest(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 2, 1);
    let f = w.mobys[id].pv_f(REST);
    let t = ctx.game.rng.randf(f * 0.8, f * 1.2);
    w.mobys[id].set_pv_i(T_REST, (t * 60.0) as i32);
}
/// `0x3e4708`: the cool-down, 87 ticks while its pvar group (`+0x43c`) has a member.
fn cool_down(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].pv_i(GROUP);
    if g & 0xff == 0xff { return; }
    if usize::try_from(g).ok().and_then(|g| w.groups.get(g)).is_some_and(|v| !v.is_empty()) { w.mobys[id].set_pv_i(COOL, 0x57); }
}
/// `0x3e5e68`: its group's live hatchlings alarmed.
fn alarm(w: &mut MobyWorld, id: usize) {
    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    for k in members {
        if let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80 && m.o_class == HATCHLING) { m.set_pv_s(ALARMED, 1); }
    }
}

/// `0x3c70f0`: a pod ready to hatch — never while Ratchet is off the ground (`0x18a160`); a 0x0920 always; a 0x0929
/// without another pod's moby on it (`+0x9c`, as a link), still (`+0x98` 0), and without a live moby at `+0xa0`.
pub(super) fn pod_ready(w: &MobyWorld, ctx: &Ctx, p: usize) -> bool {
    if ctx.hero.grounded_ticks == 0 { return false; }
    let m = &w.mobys[p];
    match m.o_class {
        POD => true,
        POD_B => {
            if let Some(o) = ai::link(m, 0x9c).and_then(|k| w.mobys.get(k)) {
                if o.o_class == POD || o.o_class == POD_OPEN || o.o_class == m.o_class { return false; }
            }
            m.pv_f(0x98) == 0.0 && live(w, m.pv_i(0xa0)).is_none()
        }
        _ => false,
    }
}

/// `0x3c7080`: the pod opens — a 0x0920 in state 0 to 1 (sequence 1 at once); a 0x0929 in state 0 hatches
/// (`0x3c8738`: its sound 0, without collision, hidden, state 1; its debris not ported).
pub(super) fn pod_open(w: &mut MobyWorld, ctx: &mut Ctx, p: usize) {
    match (w.mobys[p].o_class, w.mobys[p].state) {
        (POD, 0) => {
            w.mobys[p].state = 1;
            ctx.host.set_anim(&w.mobys[p], 1, 0, 0);
        }
        (POD_B, 0) => {
            ctx.host.play_sound(&w.mobys[p], 0, 0);
            let m = &mut w.mobys[p];
            m.mode |= 0x41;
            m.has_collision = false;
            m.state = 1;
        }
        _ => {}
    }
}

/// `0x3e57b0` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].alpha = 0x80;
    if w.mobys[id].state == 0 {
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let scale = w.class(HATCHLING).map(|c| c.scale);
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.pos[2] = z;
        if let Some(s) = scale { m.scale = s; }
        m.rot = [0.0; 3];
        if let Some(s) = usize::try_from(m.pv_i(SEAT)).ok().filter(|&s| s < w.mobys.len()) {
            let sz = w.mobys[s].pos[2];
            let m = &mut w.mobys[id];
            m.mode |= 0x41;
            m.has_collision = false;
            m.pos[2] = sz + f32::from_bits(0x3fb3_3333);
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(FLOOR, 0);
        m.set_pv_f(FALL, 0.0);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 4);
        for (k, v) in [(0x80, 0x3d4c_ccceu32), (0x84, 0x3b48_3fb8), (0x7c, 0x3ed5_5556), (0x88, 0x3bb6_0b61), (0x8c, 0x3a11_a2b4), (0x94, 0x3eb3_3333), (0x9c, 0x3ecc_cccd), (0x90, 0x3eb3_3333)] { m.set_pv_f(k, f32::from_bits(v)); }
        for (k, v) in [(0x29, 1u8), (0x70, 7), (0x71, 9), (0x76, 12), (0x77, 15), (0xd2, 6), (0xd3, 0xb), (0xcc, 6), (0xcd, 0xb), (0xce, 6), (0xcf, 0xb)] { m.set_pv_b(k, v); }
        m.set_pv_i(0xa8, 3);
        crawler::init(w, id, WALK);
        let m = &mut w.mobys[id];
        m.set_pv_f(WALK + 0xc, 2.5);
        m.set_pv_f(WALK + 8, 2.0);
        m.set_pv_i(WALK, (RADIUS * 1024.0) as i32);
        m.set_pv_f(WALK + 4, RADIUS);
        m.set_pv_f(WALK + 0x24, m.pv_f(RUN) * f32::from_bits(0x3c88_8889));
        if m.pv_f(HOME + 0xc) != MARK {
            let p = m.pos;
            m.set_pv_v(HOME, p);
            m.set_pv_f(HOME + 0xc, MARK);
        }
        let h = m.pv_v(HOME);
        m.set_pv_v(WANDER, h);
        m.set_pv_f(WANDER + 0xc, MARK);
        m.set_pv_f(WANDER + 0x10, 0.5);
        let t = ctx.game.rng.rand_range(0x78, 700);
        let m = &mut w.mobys[id];
        m.set_pv_i(WAIT, 0);
        m.set_pv_s(T_WANDER, t as i16);
        if let Some(hm) = ctx.hero_moby.and_then(|h| w.mobys.get(h)) { let (l, a) = (hm.light, hm.ambient); let m = &mut w.mobys[id]; m.light = l; m.ambient = a; }
        let m = &mut w.mobys[id];
        let (scale, h30) = (m.scale, m.pv_f(0x30));
        m.set_pv_b(0x28, 0);
        m.set_pv_i(T_OFFSET, 0);
        m.set_pv_f(0x340, scale);
        m.set_pv_f(0x330, h30);
        m.set_pv_f(BITE, m.pv_b(0x34) as f32);
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 0, 0, 0);
        let s = w.mobys[id].pv_i(SEAT);
        w.mobys[id].set_pv_i(POD_LINK, s);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if let Some(h) = w.hit_message(id) {
        if h.tmpl.flags & 0x23_0000 == 0 {
            w.clear_hit(id);
        } else if h.tmpl.kind & 0xff == 8 {
            let m = &mut w.mobys[id];
            m.state = 9;
            m.timer = 4;
        }
    }
    let k = ai::take_damage(w, id, ctx, false);
    if k != 0 {
        if k == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_f(0xf0, 0.0);
            m.set_pv_f(0xf4, f32::from_bits(0x3f56_7752));
        }
        alarm(w, id);
    }
    let f = ai::find_target(w, id, ctx, 15.0, 3.0, -1, None, false);
    w.mobys[id].set_pv_s(FOUND, f as i16);
}

/// Its bolts (all of them when it carries some, else the drop without the bank's share) and gone.
fn drop_and_go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let flags = if w.mobys[id].bolts != 0 { 0 } else { 0x1000 };
    crate::moby::bolts::drop(w, id, flags, 3, ctx);
    w.delete(id);
}

/// 0x0b88 (module docs).
pub fn update_0b88(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE {
        if w.mobys[id].drawn == 0 { return; }
        w.mobys[id].set_pv_i(SCENE, 1);
        let m = &mut w.mobys[id];
        m.has_collision = false;
        m.mode = (m.mode | 0x41) & 0xefff;
        return;
    }
    if w.mobys[id].pv_i(SCENE) != 0 {
        let has = w.class(HATCHLING).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[id];
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = has;
        m.set_pv_i(SCENE, 0);
    }
    head(w, id, ctx);
    tick_i(w, id, COOL);
    if let Some(p) = usize::try_from(w.mobys[id].pv_i(POD_LINK)).ok().filter(|&p| w.mobys.get(p).is_some_and(|q| q.o_class == POD_B)) {
        ai::set_link(&mut w.mobys[p], 0xa4, Some(id));
    }
    match w.mobys[id].sub_state {
        0 => calm(w, id, ctx),
        1 => {
            if found(w, id) && cool(w, id) {
                cool_down(w, id);
                st(w, id, 10, 2);
                set_seq(w, id, ctx, 2, 0, 0xe);
            } else if w.mobys[id].pv_i(T_REST) == 0 {
                st(w, id, 1, 0);
                set_seq(w, id, ctx, 0, 0, 0xc);
            }
        }
        2 => {
            if !found(w, id) && ctx.game.rng.randi(0x13) == 0 {
                rest(w, id, ctx);
                w.mobys[id].set_pv_s(T_WANDER, 0);
            }
        }
        3 => {
            if first_sub(&mut w.mobys[id]) && ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.speed < 0.0) {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if w.mobys[id].state != 8 {
                let r = ai::flight(w, id, ctx);
                if r & 1 != 0 {
                    w.mobys[id].set_pv_s(0x98, 0);
                    let flags = if w.mobys[id].bolts != 0 { 0 } else { 0x1000 };
                    crate::moby::bolts::drop(w, id, flags, 3, ctx);
                    crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None);
                    return;
                }
                if r & 2 != 0 {
                    if found(w, id) && cool(w, id) {
                        cool_down(w, id);
                        st(w, id, 4, 2);
                        set_seq(w, id, ctx, 9, 0, 0xe);
                    } else {
                        rest(w, id, ctx);
                        let t = ctx.game.rng.rand_range(0x78, 700);
                        w.mobys[id].set_pv_s(T_WANDER, t as i16);
                    }
                }
            }
        }
        _ => {}
    }
    let z = w.mobys[id].pos[2];
    if 10.0 < z - w.mobys[id].pv_f(HOME + 8) || z < 10.0 || (z < 75.0 && ctx.game.level == 0xb) {
        let m = &mut w.mobys[id];
        m.set_pv_s(0x98, 0);
        m.rot = [0.0; 3];
        return drop_and_go(w, id, ctx);
    }
    if states(w, id, ctx) { return drop_and_go(w, id, ctx); }
    if !matches!(w.mobys[id].state, 6 | 8 | 9) { walk(w, id, ctx); }
    tail(w, id, ctx);
}

/// Sub 0 (module docs).
fn calm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (seat, wait_for) = (m.pv_i(SEAT), m.pv_i(WAIT_FOR));
    let waiting = seat != -1 && wait_for != -1 && wait_for as usize != id && live(w, wait_for).is_some()
        && usize::try_from(seat).ok().filter(|&s| s < w.mobys.len()).is_some_and(|s| pod_ready(w, ctx, s));
    if waiting { return; }
    tick_i(w, id, WAIT);
    if found(w, id) && cool(w, id) {
        cool_down(w, id);
        match usize::try_from(seat).ok().filter(|&s| s < w.mobys.len()) {
            Some(s) => {
                if !pod_ready(w, ctx, s) { return; }
                pod_open(w, ctx, s);
                show(w, id);
                w.mobys[id].set_pv_i(SEAT, -1);
                if w.mobys[s].o_class == POD_B {
                    st(w, id, 8, 3);
                    ctx.host.set_anim(&w.mobys[id], 9, 0, 0);
                    w.mobys[id].rot[2] = w.mobys[s].rot[2];
                } else {
                    st(w, id, 10, 2);
                    set_seq(w, id, ctx, 2, 0, 0xe);
                    show(w, id);
                }
            }
            None => {
                st(w, id, 10, 2);
                set_seq(w, id, ctx, 2, 0, 0xe);
                show(w, id);
            }
        }
    } else if w.mobys[id].pv_s(ALARMED) != 0 {
        st(w, id, 2, 1);
        w.mobys[id].set_pv_s(ALARMED, 0);
        let f = w.mobys[id].pv_f(REST);
        let t = ctx.game.rng.randf(f * 0.8, f * 1.2);
        let m = &mut w.mobys[id];
        m.set_pv_i(T_REST, (t * 60.0) as i32);
        m.set_pv_s(T_WANDER, 0);
    }
}

/// Its wander speed toward `speed` a second, none in the timer's last 20 ticks.
fn wander_speed(w: &mut MobyWorld, id: usize, speed: f32) {
    let want = if 20 < w.mobys[id].pv_s(T_WANDER) { speed * (1.0 / 60.0) } else { 0.0 };
    let mut v = w.mobys[id].pv_f(WANDER + 0x14);
    approach(want, f32::from_bits(0x3afe_dcbb), &mut v);
    w.mobys[id].set_pv_f(WANDER + 0x14, v);
}

/// Turned to its target with its offset (`0x31cbf0`(0.05, 0.3, 6°)).
fn turn(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let t = target_pos(w, id, ctx);
    let m = &mut w.mobys[id];
    let yaw = add_rot(atan2(t[0] - m.pos[0], t[1] - m.pos[1]), m.pv_f(OFFSET));
    ai::turn_to(m, yaw, f32::from_bits(0x3d4c_cccd), f32::from_bits(0x3e99_999a), f32::from_bits(0x3dd6_7752), TURN_V);
}

/// `0x32fb68(m, p, 0x10, yaw, −1, −1)` (L02 `0x331440`): the move to `p` with the movement options 0x10 for the
/// tick.
fn back_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) {
    let q = w.mobys[id].pos;
    let yaw = atan2(p[0] - q[0], p[1] - q[1]);
    let o = ai::mv(&w.mobys[id]) + 0x38;
    let kept = w.mobys[id].pv_i(o);
    w.mobys[id].set_pv_i(o, 0x10);
    ai::move_step(w, id, ctx, yaw, p);
    w.mobys[id].set_pv_i(o, kept);
}

/// The states (module docs); true when it is drawn in and goes.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].state {
        1 => {
            tick_s(w, id, T_WANDER);
            let t = w.mobys[id].pv_s(T_WANDER);
            if seq(w, id, ctx) == 0 && t != 0 { set_seq(w, id, ctx, 9, 0, 0xe); }
            wander_speed(w, id, WANDER_1);
            let s = seq(w, id, ctx);
            if s == 9 {
                ctx.host.set_anim_speed(&w.mobys[id], 0.8);
                if t == 0 && s as i32 != ctx.game.rng.randi(2) {
                    let n = ctx.game.rng.randi(2) as u8;
                    ctx.host.set_anim(&w.mobys[id], n, 0, 0xc);
                }
                ai::wander(w, id, ctx, f32::from_bits(0x3eb3_3333), 0.25, WANDER);
            } else if (s == 0 || s == 1) && wrapped(w, id, ctx) {
                let t = ctx.game.rng.rand_range(0x78, 500);
                w.mobys[id].set_pv_s(T_WANDER, t as i16);
                set_seq(w, id, ctx, 9, 0, 0xc);
            }
        }
        2 => {
            tick_s(w, id, T_WANDER);
            if !matches!(seq(w, id, ctx), 9 | 5 | 4) { set_seq(w, id, ctx, 9, 0, 0xe); }
            wander_speed(w, id, WANDER_2);
            let s = seq(w, id, ctx);
            if s == 9 {
                if w.mobys[id].pv_s(T_WANDER) == 0 {
                    ctx.host.set_anim(&w.mobys[id], 5, 0, 0xc);
                    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 4, 0, 0xc); }
                }
                ai::wander(w, id, ctx, f32::from_bits(0x3eb3_3333), 0.25, WANDER);
            } else if (s == 4 || s == 5) && wrapped(w, id, ctx) {
                let t = ctx.game.rng.rand_range(0x78, 200);
                w.mobys[id].set_pv_s(T_WANDER, t as i16);
                set_seq(w, id, ctx, 9, 0, 0xc);
            }
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
        }
        10 => {
            if wrapped(w, id, ctx) {
                st(w, id, 4, -1);
                set_seq(w, id, ctx, 9, 0, 0xe);
            }
            let (p, h) = (w.mobys[id].pos, ctx.hero.pos);
            let m = &mut w.mobys[id];
            approach_angle(atan2(h[0] - p[0], h[1] - p[1]), f32::from_bits(0x3dd6_7752), &mut m.rot[2], 0);
        }
        4 => {
            if w.mobys[id].first() {
                let t = ctx.game.rng.rand_range(80, 100);
                w.mobys[id].set_pv_i(T_OFFSET, t);
            }
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            let t = target_pos(w, id, ctx);
            if dist2(w.mobys[id].pos, t) <= REACH {
                st(w, id, 5, 2);
                set_seq(w, id, ctx, 6, 2, 6);
                w.mobys[id].set_pv_f(OFFSET, 0.0);
            } else if tick_i(w, id, T_OFFSET) {
                let f = ctx.game.rng.randf(f32::from_bits(0x3e86_0a93), f32::from_bits(0x3f06_0a93));
                let f = if ctx.game.rng.randi(2) == 0 { -f } else { f };
                w.mobys[id].set_pv_f(OFFSET, f);
                let t = ctx.game.rng.rand_range(150, 200);
                w.mobys[id].set_pv_i(T_OFFSET, t);
                st(w, id, 5, 2);
                set_seq(w, id, ctx, 6, 2, 6);
            }
        }
        5 => {
            let sa = |w: &MobyWorld, ctx: &Ctx| ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa(w, &*ctx) != 6 { return false; }
            if ai::reached(w, id, ctx, 8.0) { ctx.host.play_sound(&w.mobys[id], 3, 0); }
            if sa(w, &*ctx) != 6 { return false; }
            if ai::reached(w, id, ctx, 10.0) {
                let (d, pos) = (w.mobys[id].pv_f(BITE), w.mobys[id].pos);
                let n = ai::attack_sphere(w, id, ctx, 1.0, d, 1.0, pos, 0x400_0001, 0, 1, 0);
                w.mobys[id].set_pv_i(LANDED, (0 < n) as i32);
            } else if sa(w, &*ctx) == 6 && (wrapped(w, id, ctx) || ai::reached(w, id, ctx, 11.0)) {
                let t = target_pos(w, id, ctx);
                if REACH < dist2(w.mobys[id].pos, t) {
                    // The original's `(d == 0 ? 1 : 0) > reach`: with the reach 1, never.
                    let v = if dist2(w.mobys[id].pos, t) == 0.0 { 1.0 } else { 0.0 };
                    if REACH < v {
                        if 0 < ctx.game.rng.rand_range(0, 7) {
                            st(w, id, 11, 2);
                            set_seq(w, id, ctx, 7, 0, 0xe);
                        }
                    } else {
                        st(w, id, 6, 2);
                        set_seq(w, id, ctx, 9, 0, 0xe);
                    }
                }
            }
        }
        11 => {
            if wrapped(w, id, ctx) {
                st(w, id, 4, 2);
                set_seq(w, id, ctx, 9, 0, 0);
            }
        }
        6 => {
            ai::senses(&mut w.mobys[id], f32::from_bits(0x3ba3_d70a), f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3c8e_fa36), f32::from_bits(0x3f86_0a93));
            if w.mobys[id].first() {
                let t = target_pos(w, id, ctx);
                let p = w.mobys[id].pos;
                let mut d = sub(p, t);
                d[2] = 0.0;
                let goal = add(set_len(d, 10.0), p);
                w.mobys[id].set_pv_v(BACK, goal);
                ctx.host.set_anim_speed(&w.mobys[id], -1.0);
            }
            let goal = w.mobys[id].pv_v(BACK);
            back_to(w, id, ctx, goal);
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa == 2 && wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 9, 0, 10);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa == 9 && wrapped(w, id, ctx) {
                if w.mobys[id].pv_i(LANDED) == 0 {
                    st(w, id, 4, 2);
                    set_seq(w, id, ctx, 9, 0, 6);
                } else {
                    st(w, id, 11, 2);
                    set_seq(w, id, ctx, 7, 0, 0xe);
                    w.mobys[id].set_pv_i(LANDED, 0);
                }
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        8 => {
            if 6.0 < dist2(w.mobys[id].pos, w.mobys[id].pv_v(HOME)) { st(w, id, 4, 2); }
        }
        3 => {
            turn(w, id, ctx);
            if wrapped(w, id, ctx) {
                st(w, id, 4, -1);
                set_seq(w, id, ctx, 9, 0, 0xf);
            }
        }
        9 => return pulled(w, id, ctx),
        _ => {}
    }
    false
}

/// `0x3e5ee8`: drawn in by the moby `0x1afb5c` (none in the port: it goes at once).
fn pulled(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    set_seq(w, id, ctx, 1, 0, 10);
    true
}

/// `0x3e5b68`: in 4 on its run sequence (9) turned to its target and walked along its facing (the walker at its
/// radius), kept in its fence; in 5 turned. Then it falls (1/180 a tick²) to the ground 0.5 over it.
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
    match w.mobys[id].state {
        4 if sa == 9 => {
            turn(w, id, ctx);
            let m = &mut w.mobys[id];
            m.set_pv_i(WALK, (RADIUS * 1024.0) as i32);
            m.set_pv_f(WALK + 4, RADIUS);
            crawler::ahead(w, id, ctx, WALK);
            let f = w.mobys[id].pv_i(FENCE);
            if let Some(p) = usize::try_from(f).ok().and_then(|f| ctx.level.paths.get(f)) {
                if let Some(q) = ai::fence(p, RADIUS, w.mobys[id].pos) { w.mobys[id].pos = q; }
            }
        }
        5 => turn(w, id, ctx),
        _ => {}
    }
    let m = &mut w.mobys[id];
    let f = m.pv_f(FALL) + f32::from_bits(0x3bb6_0b61);
    m.set_pv_f(FALL, f);
    m.pos[2] -= f - 2.0;
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    let m = &mut w.mobys[id];
    m.pos[2] -= 2.0;
    if g <= m.pos[2] { return; }
    m.set_pv_i(FLOOR, 0);
    m.pos[2] = g;
    m.set_pv_f(FALL, 0.0);
}

/// `0x3e5df0`: the flash, the rest and leap timers down, its ticks in the state, its head aim (`0x329158` = L00
/// `0x327308` on `+0x350`, list 1, 2.5: crate::moby::joint_aim::head).
fn tail(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    ai::flash(&mut w.mobys[id], FLASH);
    tick_i(w, id, T_REST);
    tick_s(w, id, T_LEAP);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 1, 2.5);
}

/// 0x0920 / 0x0921 (module docs).
pub fn update_0920(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    match w.mobys[id].state {
        0 => if w.mobys[id].pv_i(0x90) != -1 {
            let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            let m = &mut w.mobys[id];
            if g < m.pos[2] {
                let f = m.pv_f(0x98) + 30.0 * f32::from_bits(0x3991_a2b4);
                m.set_pv_f(0x98, f);
                m.pos[2] -= f;
            } else {
                m.set_pv_f(0x98, 0.0);
                m.pos[2] = g;
            }
        },
        1 => {
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa == 1 && wrapped(w, id, ctx) {
                let c = w.class(POD_OPEN).map(|c| (c.slot, c.has_collision));
                let m = &mut w.mobys[id];
                m.o_class = POD_OPEN;
                if let Some((slot, has)) = c {
                    m.slot = slot;
                    m.has_collision = has;
                }
                ctx.host.set_anim(&w.mobys[id], 0, 0, 0);
                let m = &mut w.mobys[id];
                m.state = 2;
                if m.pv_i(0x90) != -1 { m.set_pv_i(0x94, 600); }
            }
        }
        2 => if w.mobys[id].pv_i(0x90) != -1 && tick_i(w, id, 0x94) {
            return w.delete(id);
        },
        _ => {}
    }
    let m = &mut w.mobys[id];
    let mv = sub(m.pos, old);
    platform::write(m, 0x50, mv, rot, rot);
}
