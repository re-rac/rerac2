//! Grelbin's frozen beast **0x0f57** (`0x4173c0`, 33 placed; the decomp's v2.00 `func_US2_L19_004173C0`, its head
//! v1.01 `func_L19_004170D8` at our `0x4183f8` (no C), its tail `func_L19_00417A00` at `0x418d20`): it waits frozen
//! in a block of ice (0x1178, its own or the placed moby `+0x2dc`) until Ratchet comes into its search, breaks out,
//! floats toward him at a height by his distance, and breathes a freezing beam at him; a lurker
//! (crate::moby::classes::level19::lurker) just 15..16 away across it eats instead. Killed, it leaves the ice's
//! remains (0x1266) and its uid is counted, so it stays gone.
//!
//! **Its pvars:** health `+0x20` (most `+0x24`, the hit pause `+0x26`, status `+0x2e`: 2 kills it, its damage byte
//! `+0x34`, `+0x30` 16), movement `+0x50` (radius 0, height 0.2, steps 0.75 / 0.75, slope 90°, turn left 30°, stop
//! 0.5; only its turn limits are used), reaction `+0x140` (only its colour flash at `+0x140`: 200 red), `+0x1e8` the
//! damage factor for hits from its own class, target block `+0x1f0` (`+0x200` the moby, `+0x204` found), `+0x210` /
//! `+0x214` the first search's area and the second's range (× 1.5), `+0x218` / `+0x21c` timers (`+0x218` 480..600
//! after an alarm; read by nothing here), `+0x21e` / `+0x220` the alarm's countdown and the alarm, `+0x228` /
//! `+0x22c` the charge glows' sizes, `+0x230` 1, `+0x234` the beam's width, `+0x238` its length (× 60), `+0x2c0` the
//! beam's end, `+0x2d0` / `+0x2d4` the height it eases to and its speed, `+0x2d8` its home height, `+0x2dc` the
//! placed ice (−1: its own), `+0x2e0` the beam's aim, `+0x2f0` the ice, `+0x2f4` the second search's area (−1: Ratchet
//! within 32 across instead), `+0x2f8` / `+0x2fc` the second and first searches' results, `+0x304` broken out,
//! `+0x308` a group it waits on, `+0x30c` a moby it waits on (−1 none), `+0x310` / `+0x314` its camera tweak.
//!
//! **Each tick** ([`update_0f57`]): nothing with Ratchet in 0x6f; outside the loaded variant (`0x31f3f0` = L00
//! `0x321a78`) its update distance its draw distance + 15 and nothing more. The ice shares its mode bit 0x80. Out of
//! sub 2..3, with Ratchet in 0x31 or no health, the camera tweak is let go (`0x2d4e48` = L00 `0x2d7508`).
//!
//! **The head** ([`head`], `0x4183f8`). The set-up (state 0): without collision, hidden and untargeted, its ice made
//! (0x1178 at it, `0x327880` = L00 `0x32a748`; its distances, mode 0x100) or taken, home its height, distances 0x5a,
//! the blocks, state 1 (sub 0), sequence 7. Counted in the save already: the ice turns to the broken 0x1208 and it
//! goes. Status 2: its bolts (`0x318780` = L00 `0x31adc0`, limit 3), the remains ([`leave_remains`]) and it goes.
//! Else its flash ticks (`0x31de18` = L00 `0x3201e8`); a hit (`0x31b918` = L00 `0x31dce8`, mask 0x10000) past the
//! pause (`0x2ec100` = L00 `0x2eeae0`) takes its damage (`0x31b978` = L00 `0x31dd48`) and flashes it
//! (`0x31dd58` = L00 `0x320128`); none left, sequence 5 and state 12 (sub 3), a lurker it was eating gone (one
//! attacker less) and Ratchet its target. Out of 11 the searches: the first (`0x327e70` = L00 `0x32ad38`: 36 across,
//! 10 high, area `+0x210`) and, out of sub 0, the second. Then, out of states 4 / 8 / 9 and alive, a lurker out of
//! hiding 15..16 across with `+0x254` set becomes its target (in 7 after 15 ticks none is looked for); with its target
//! such a lurker 15..16 across and out of sub 0 and state 11, it starts eating: its height home, sequence 6, state 11,
//! the lurker held (state 0x17, sub 4, sequence 6).
//!
//! **Sub-states** (`+0x95`): **0** frozen: with no group member `+0x308` alive (when it has a group) and the moby
//! `+0x30c` gone, found by the first search, it breaks out (4, sub 2, sequence 0); else an alarm counted out wakes it
//! (2, sub 1). It faces its target. **1** awake: found by either search (out of 11), 7 (sub 2, sequence 2). **2**
//! attacking: its camera tweak (priority 5: distance 10, pivot 0.1, pitch 0); facing its target; out of 4 its height
//! eased (`0x31c9a8` = L00 `0x31ed78`, 7/1800 a tick, at most 0.7) to home less 10 / 7 / 4 for Ratchet within 12 /
//! 26 / farther (out of 4 and 11); found by neither and out of 4 / 7 / 8 / 9 / 11, 2 (sub 1, sequence 1). **4**
//! (unused here): as 2's facing and height, faster (0.389), then as 3. **3** dying: at its sequence's end its bolts
//! and it goes; at frame 45 with no health the remains, once.
//!
//! **States** (`+0x20`): **1** frozen. **2** / **3** idle (2: at the end sequence 3 or 4 and 3; 3: at the end
//! sequence 1 and 2). **4** breaking out (sequence 0): from frame 4, once and blended in, the camera shakes (slot 0:
//! 0.25 for 38 ticks), it is shown, solid and targeted, five of the ice's pieces fly (`0x2d96f0` = L00 `0x2dbdb0`
//! mode 1: 180 / 45, up to 1.25 up) and the ice becomes the broken 0x1208; at its end its height home less 7 and 7.
//! **5** at its end 7. **7** the charge (sequence 2): its sound 4 at frame 13; past frame 17 and blended in, its speed
//! 0.5 and [`draw_charge`]; at its end the aim its target's point, 8 (sequence 4 from frame 20). **8**
//! [`draw_gather`]; the glow grown to 1, its sound 3 and 9. **9** the beam: [`draw_beam`]; at the beam's end ten pairs
//! of frost puffs (`0x33e538` = L00 `0x340830`); the beam a hit (`0x31ac40` = L00 `0x31d158`: its damage byte, flags
//! 0x10001) along the line from its mouth (`0x2dc1b8` = L00 `0x2de878`); it widens and lengthens for 40 ticks, then
//! thins out to 10 (sequence 3). **10** at its end 7. **11** eating: the lurker turned to face it; at frame 11
//! blended in the lurker is gone (one attacker less) and Ratchet its target; at its end its height home less 4 and 7
//! or 2 by the searches. **12** dying (sub 3).
//!
//! **The tail** (`0x418d20`): its two timers (`0x2ec0d0` / `0x2ec100` = L00 `0x2eeab0` / `0x2eeae0`), its state ticks.
//!
//! The engine functions with the port's: `0x319290` = L00 `0x31b8d0` (crate::moby::Moby::set_state), `0x318fb0` = L00
//! `0x31b5f0` (set_anim), `0x30f200` = L00 `0x311880` (the frame time), `0x321608` = L00 `0x323bf8`
//! (crate::moby::ai::reached), `0x32aa68` (L01 `0x323560`, the facing eased), `0x32aad0` = L00 `0x32dcf0`
//! (crate::moby::ai::senses), `0x31a248` = L00 `0x31c8e0` (crate::moby::classes::props::group_alive), `0x35e400` =
//! L00 `0x3605f0` (the class sound), `0x3100e8` = L00 `0x312728` (the joint's frame), `0x2eb598` = L00 `0x2edf78`
//! (crate::fx::sprite), `0x2e9510` = L00 `0x2eb998` (a quad), `0x319538` = L00 `0x31bb48` ([`shell`]), `0x3195b0` =
//! L00 `0x31bbc0` (crate::rng::Rng::rand_vec), `0x2ec748` = L00 `0x2ef138` (set_len), `0x30eca0` / `0x30f008` = L00
//! `0x311320` / `0x311688` (create / delete), `0x311780` = L00 `0x313dc0` (rebuild), `0x31dab0` = L00 `0x31fe80`
//! (Ratchet's light). `$gp+0x2cd0` (`0x1b1cc0`) is the lurkers' attacker pool (crate::moby::classes::level19::Globals).
//!
//! The draws are the frame's first list's (`0x2e4fd0` = L00 `0x2e7718`, crate::fx), one of three by the state that
//! registered it: [`draw`] tells them by the state (or the one before, when it changed this tick).
//!
//! The port's differences: the charge's sparks ([`draw_charge`]'s) are made by the update when it registers it (the
//! draws have no particles), and the beam's end ([`draw_beam`]'s line `0x2dc1b8`) is found by the update after its
//! tick (the draws have no collision), with the joint as it is then.
//!
//! Not ported: the ice's rows turned to the ground under it (`0x2dc1b8` down 11 and `0x3240e8` = L00 `0x3268c8` on the
//! line's normal); the level's state table `0x21ee44` as a second save test; the moby list `$gp+0x39b8` the lurkers
//! are looked for in (every 0x10d0 alive, in slot order, here).

use crate::hero::physics::{add, atan2, dist2, ease, set_len, sub, V3};
use crate::moby::classes::props::{group_alive, set_seq, wrapped};
use crate::moby::classes::shot::polar;
use crate::moby::{ai, bolts, countdown, mode, shards, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x0f57;
const ICE: i16 = 0x1178;
const ICE_BROKEN: i16 = 0x1208;
const REMAINS: i16 = 0x1266;
const LURKER: i16 = super::lurker::CLASS;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const PAUSE: usize = 0x26;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const SAME_CLASS: usize = 0x1e8;
const T_BLOCK: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const T_FOUND: usize = 0x204;
const AREA1: usize = 0x210;
const RANGE2: usize = 0x214;
const TIMER: usize = 0x218;
const TIMER2: usize = 0x21c;
const ALARM_T: usize = 0x21e;
const ALARM: usize = 0x220;
const GLOW: usize = 0x228;
const GATHER: usize = 0x22c;
const G230: usize = 0x230;
const WIDTH: usize = 0x234;
const LENGTH: usize = 0x238;
const BEAM_END: usize = 0x2c0;
const HEIGHT: usize = 0x2d0;
const HEIGHT_V: usize = 0x2d4;
const HOME: usize = 0x2d8;
const PLACED_ICE: usize = 0x2dc;
const AIM: usize = 0x2e0;
const ICE_AT: usize = 0x2f0;
const AREA2: usize = 0x2f4;
const FOUND2: usize = 0x2f8;
const FOUND1: usize = 0x2fc;
const BROKEN: usize = 0x304;
const WAIT_GROUP: usize = 0x308;
const WAIT_MOBY: usize = 0x30c;
const TWEAK: usize = 0x310;
const TWEAK_SLOT: usize = 0x314;

/// `0x32aad0(1/300, 0.1, 0.4°, 48°)`, the lurker's run's.
const SENSES: [u32; 4] = [0x3b5a_740e, 0x3dcc_ccce, 0x3be4_c38a, 0x3f56_7752];
/// `$gp+0x2ba8` (`0x1b1b98`): the beam's frost puffs' frame.
const PUFF_FRAME: usize = 6;
/// `$gp+0x2bac..+0x2bb4` (`0x1b1b9c..`): the gather glow's colour, FX and size (× `+0x22c`).
const GATHER_RGBA: u32 = 0x6080_8080;
const GATHER_FX: usize = 0x2a;
const GATHER_SIZE: f32 = 28.5;
/// The glows' push toward the camera (0.08).
const PUSH: u32 = 0x3da3_d70a;

fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }

/// `0x319538` (L00 `0x31bb48(a, b)`): a vector of length `randf(a, b)` a random way (yaw, then pitch).
fn shell(rng: &mut crate::rng::Rng, a: f32, b: f32) -> V3 {
    let yaw = rng.rand_angle();
    let pitch = rng.rand_angle();
    polar(rng.randf(a, b), yaw, pitch)
}

fn alive(w: &MobyWorld, k: usize) -> bool { w.mobys.get(k).is_some_and(|o| o.state < 0x80) }

fn ice(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], ICE_AT).filter(|&k| k < w.mobys.len()) }

fn target_pos(w: &MobyWorld, id: usize) -> V3 {
    let m = &w.mobys[id];
    ai::link(m, T_MOBY).and_then(|k| w.mobys.get(k)).map_or(m.pv_v(T_BLOCK), |t| t.pos)
}

/// `0x32aa68(yaw, m)` (L01 `0x323560`): moby `id`'s facing eased toward `yaw` at its movement block's turn limits.
fn face(w: &mut MobyWorld, id: usize, yaw: f32) {
    let m = &mut w.mobys[id];
    let (a, b, most) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    crate::hero::physics::turn_ease(yaw, a, b, most, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
}

fn face_target(w: &mut MobyWorld, id: usize) {
    let (t, p) = (target_pos(w, id), w.mobys[id].pos);
    face(w, id, atan2(t[0] - p[0], t[1] - p[1]));
}

/// `0x31c9a8` (L00 `0x31ed78`): its height eased toward `+0x2d0`.
fn ease_height(m: &mut Moby, rate: f32) {
    let (mut z, mut v) = (m.pos[2], m.pv_f(HEIGHT_V));
    ease(m.pv_f(HEIGHT), rate, rate, f32::from_bits(0x3f33_3334), &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(HEIGHT_V, v);
}

fn release_tweak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pv_i(TWEAK) == 0 { return; }
    let slot = m.pv_i(TWEAK_SLOT);
    if 0 < slot { ctx.game.cam_tweaks.release(slot as usize - 1); }
    m.set_pv_i(TWEAK, 0);
}

/// The ice turned to the broken 0x1208 (at it, its rows and angles, this one's distances); the old ice goes.
fn break_ice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(old) = ice(w, id) else { return };
    let n = w.create(ICE_BROKEN);
    w.adopt(n, Some(old), &*ctx.host);
    let (ud, dd) = (w.mobys[id].update_dist, w.mobys[id].draw_dist);
    let (rows, rot) = (w.mobys[old].rows, w.mobys[old].rot);
    let m = &mut w.mobys[n];
    (m.draw_dist, m.update_dist, m.rows, m.rot) = (dd, ud, rows, rot);
    w.delete(old);
    ai::set_link(&mut w.mobys[id], ICE_AT, Some(n));
    w.changed = true;
}

/// The ice's remains 0x1266 (its uid counted): alpha 0x50, mode 0x800, at the ice 1 higher, Ratchet's light,
/// sequence 1 at 0.2, state 2.
fn leave_remains(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = ice(w, id).map_or(w.mobys[id].pos, |k| w.mobys[k].pos);
    let uid = w.mobys[id].uid;
    ctx.game.count_uid(uid);
    let n = w.create(REMAINS);
    let m = &mut w.mobys[n];
    (m.alpha, m.update_dist, m.draw_dist, m.drawn, m.state, m.timer) = (0x50, 0xff, 0xff, 1, 0, 0);
    m.mode |= 0x800;
    m.pos = add(at, [0.0, 0.0, 1.0]);
    w.rebuild(n, ctx.host);
    if let Some(h) = ctx.hero_moby.and_then(|h| w.mobys.get(h)) {
        let (light, ambient) = (h.light, h.ambient);
        (w.mobys[n].light, w.mobys[n].ambient) = (light, ambient);
    }
    w.mobys[n].set_pv_f(0, 0.1);
    ctx.host.set_anim(&w.mobys[n], 1, 0, 20);
    ctx.host.set_anim_speed(&w.mobys[n], 0.2);
    w.mobys[n].state = 2;
    w.changed = true;
}

/// 0x0f57 (module docs).
pub fn update_0f57(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state == 0x6f { return; }
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) {
        let m = &mut w.mobys[id];
        m.update_dist = (m.draw_dist as f32 + 15.0) as i32 as u8;
        return;
    }
    if let Some(k) = ice(w, id) {
        let on = w.mobys[id].mode & 0x80 != 0;
        let c = &mut w.mobys[k];
        c.mode = if on { c.mode | 0x80 } else { c.mode & 0xff7f };
    }
    let m = &w.mobys[id];
    if !(2..=3).contains(&m.sub_state) || ctx.hero.state == 0x31 || m.pv_f(HEALTH) <= 0.0 { release_tweak(w, id, ctx); }
    if head(w, id, ctx) {
        release_tweak(w, id, ctx);
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        0 => {
            let m = &w.mobys[id];
            let mut ok = true;
            let grouped = usize::try_from(m.group).ok().and_then(|g| w.groups.get(g)).is_some_and(|l| !l.is_empty());
            if grouped && group_alive(w, m.pv_i(WAIT_GROUP)) != 0 { ok = false; }
            let k = m.pv_i(WAIT_MOBY);
            if k != -1 && usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_some_and(|o| o.state != 0xfe && o.state != 0xfd) { ok = false; }
            if ok {
                if w.mobys[id].pv_i(FOUND1) == 1 {
                    w.mobys[id].set_state(4, 2);
                    set_seq(w, id, ctx, 0, 0, 10);
                } else if w.mobys[id].pv_i(ALARM) != 0 && w.mobys[id].count_s(ALARM_T) {
                    let m = &mut w.mobys[id];
                    m.set_state(2, 1);
                    m.set_pv_i(ALARM, 0);
                    if m.pv_i(TIMER) < 300 {
                        let t = ctx.game.rng.rand_range(480, 600);
                        w.mobys[id].set_pv_i(TIMER, t);
                    }
                }
            }
            face_target(w, id);
        }
        1 => {
            let m = &w.mobys[id];
            if (m.pv_i(FOUND1) == 1 || m.pv_i(FOUND2) == 1) && m.state != 11 {
                w.mobys[id].set_state(7, 2);
                set_seq(w, id, ctx, 2, 0, 10);
            }
        }
        2 => {
            if w.mobys[id].pv_i(TWEAK) == 0 && ctx.hero.state != 0x31 {
                if let Some(s) = ctx.game.cam_tweaks.request(5) {
                    let tw = &mut ctx.game.cam_tweaks.slots[s];
                    (tw.dist, tw.pivot_h, tw.pitch) = (10.0, 0.1, 0.0);
                    w.mobys[id].set_pv_i(TWEAK_SLOT, s as i32 + 1);
                }
                w.mobys[id].set_pv_i(TWEAK, 1);
            }
            face_target(w, id);
            let hero = ctx.hero.pos;
            let m = &mut w.mobys[id];
            if m.state != 4 { ease_height(m, f32::from_bits(0x3b7e_dcbb)); }
            if m.state != 4 && m.state != 11 {
                let d = dist2(m.pos, hero);
                let drop = if d < 12.0 { 10.0 } else if d < 26.0 { 7.0 } else { 4.0 };
                m.set_pv_f(HEIGHT, m.pv_f(HOME) - drop);
            }
            if m.pv_i(FOUND1) != 1 && m.pv_i(FOUND2) != 1 && ![4, 7, 8, 9, 11].contains(&m.state) {
                m.set_state(2, 1);
                set_seq(w, id, ctx, 1, 0, 10);
            }
        }
        s @ (3 | 4) => {
            if s == 4 {
                face_target(w, id);
                let m = &mut w.mobys[id];
                if m.state != 4 { ease_height(m, f32::from_bits(0x3ec7_1c72)); }
            }
            if wrapped(w, id, ctx) {
                bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
                return;
            }
            let m = &w.mobys[id];
            if m.pv_f(HEALTH) <= 0.0 && ai::reached(w, id, ctx, 45.0) && ctx.game.uid_count(m.uid) == 0 { leave_remains(w, id, ctx); }
        }
        _ => {}
    }
    states(w, id, ctx);
    // `0x418d20`: its two timers and its ticks.
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(TIMER);
    countdown(&mut t);
    m.set_pv_i(TIMER, t);
    m.count_s(TIMER2);
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = m.state_ticks.wrapping_add(1); }
    w.changed = true;
}

/// `0x4183f8` (module docs): true when it is to go.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        setup(w, id, ctx);
        if ctx.game.uid_count(w.mobys[id].uid) != 0 {
            break_ice(w, id, ctx);
            return true;
        }
    }
    if w.mobys[id].pv_b(STATUS) == 2 {
        bolts::drop(w, id, 0, 3, ctx);
        leave_remains(w, id, ctx);
        return true;
    }
    ai::flash(&mut w.mobys[id], REACT);
    let mut hit = w.hit_message(id);
    if hit.is_some_and(|h| h.tmpl.flags & 0x1_0000 == 0) {
        w.clear_hit(id);
        hit = None;
    }
    if !w.mobys[id].count_s(PAUSE) { hit = None; }
    if let Some(mut h) = hit {
        let k = w.mobys[id].pv_f(SAME_CLASS);
        if k != 0.0 && h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == CLASS) { h.tmpl.damage *= k; }
        let dmg = ai::damage_of(w, id, Some(&h), false);
        w.clear_hit(id);
        let m = &mut w.mobys[id];
        m.set_pv_b(REACT + 7, 200);
        m.set_pv_f(HEALTH, m.pv_f(HEALTH) - dmg);
        ai::flash_start(m, REACT);
        if m.pv_f(HEALTH) <= 0.0 {
            set_seq(w, id, ctx, 5, 0, 0x14);
            w.mobys[id].set_state(12, 3);
            if let Some(l) = ai::link(&w.mobys[id], T_MOBY).filter(|&l| w.mobys.get(l).is_some_and(|o| o.o_class == LURKER)) {
                w.level19.attackers -= 1;
                w.delete(l);
                let aim = ctx.hero.aim_point;
                let m = &mut w.mobys[id];
                m.set_pv_i(T_FOUND, 1);
                ai::set_link(m, T_MOBY, ctx.hero_moby);
                m.set_pv_v(T_BLOCK, aim);
            }
        }
    }
    if w.mobys[id].state != 11 {
        let (a1, a2, r2) = (w.mobys[id].pv_i(AREA1), w.mobys[id].pv_i(AREA2), w.mobys[id].pv_f(RANGE2));
        let f1 = ai::find_target(w, id, ctx, 36.0, 10.0, a1, None, false);
        w.mobys[id].set_pv_i(FOUND1, f1 as i32);
        if w.mobys[id].sub_state != 0 {
            let f2 = ai::find_target(w, id, ctx, r2 * 1.5, 0.0, a2, None, false);
            let f2 = if a2 == -1 { dist2(w.mobys[id].pos, ctx.hero.pos) < 32.0 } else { f2 };
            w.mobys[id].set_pv_i(FOUND2, f2 as i32);
        }
        if ctx.hero.state == 0x31 {
            let p = ctx.hero.pos;
            w.mobys[id].set_pv_v(T_BLOCK, p);
        }
        look_for_lurkers(w, id);
    }
    let Some(t) = ai::link(&w.mobys[id], T_MOBY).filter(|&t| t < w.mobys.len()) else { return false };
    if w.mobys[t].o_class != LURKER { return false; }
    let d = dist2(w.mobys[id].pos, w.mobys[t].pos);
    if 16.0 <= d || d <= 15.0 || w.mobys[id].state == 11 || w.mobys[id].sub_state == 0 { return false; }
    let home = w.mobys[id].pv_f(HOME);
    w.mobys[id].set_pv_f(HEIGHT, home);
    set_seq(w, id, ctx, 6, 0, 0x13);
    w.mobys[id].set_state(11, -1);
    let l = &mut w.mobys[t];
    (l.state, l.sub_state) = (0x17, 4);
    ctx.host.set_anim(&w.mobys[t], 6, 0, 0xf);
    false
}

/// The lurkers looked through (module docs): one 15..16 across, out of hiding and with `+0x254` set, its target.
fn look_for_lurkers(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    if [4, 8, 9].contains(&m.state) || m.pv_f(HEALTH) <= 0.0 { return; }
    let lurkers: Vec<usize> = (0..w.mobys.len()).filter(|&k| w.mobys[k].o_class == LURKER && alive(w, k)).collect();
    if lurkers.is_empty() || (m.state == 7 && 0xf < m.state_ticks as u16) { return; }
    let pos = m.pos;
    for k in lurkers {
        let l = &w.mobys[k];
        let d = dist2(pos, l.pos);
        if d < 16.0 && 15.0 < d && l.sub_state != 0 && l.pv_i(0x254) != 0 {
            let p = l.pos;
            let m = &mut w.mobys[id];
            ai::set_link(m, T_MOBY, Some(k));
            m.set_pv_i(T_FOUND, 1);
            m.set_pv_v(T_BLOCK, p);
            return;
        }
    }
}

/// The head's set-up (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 1;
    m.set_pv_i(BROKEN, 0);
    let placed = m.pv_i(PLACED_ICE);
    if placed == -1 {
        let n = w.create(ICE);
        w.adopt(n, Some(id), &*ctx.host);
        let (ud, dd) = (w.mobys[id].update_dist, w.mobys[id].draw_dist);
        let c = &mut w.mobys[n];
        (c.draw_dist, c.update_dist) = (dd, ud);
        c.mode |= mode::KEEP_ROWS;
        ai::set_link(&mut w.mobys[id], ICE_AT, Some(n));
        w.changed = true;
    } else {
        ai::set_link(&mut w.mobys[id], ICE_AT, usize::try_from(placed).ok());
    }
    let m = &mut w.mobys[id];
    let z = m.pos[2];
    m.set_pv_f(HEIGHT, z);
    m.set_pv_f(HOME, z);
    (m.update_dist, m.draw_dist) = (0x5a, 0x5a);
    m.set_pv_f(LENGTH, 0.1);
    m.set_pv_b(REACT + 0x10, 12);
    m.set_pv_i(REACT + 0x48, 3);
    m.set_pv_b(REACT + 0x16, 5);
    m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3e19_999a));
    m.set_pv_f(0x30, 16.0);
    m.set_pv_b(STATUS, 1);
    m.set_pv_f(GLOW, 0.1);
    m.set_pv_f(GATHER, 0.0);
    m.set_pv_f(G230, 1.0);
    m.set_pv_f(WIDTH, 0.1);
    m.set_pv_f(REACT + 0x30, 0.5);
    m.set_pv_f(REACT + 0x34, 0.5);
    m.set_pv_b(REACT + 0x77, 1);
    if m.pv_s(MOST) == 0 {
        m.set_pv_f(HEALTH, 1.0);
        m.set_pv_s(MOST, 1);
    }
    m.set_pv_f(MV, 0.0);
    m.set_pv_f(MV + 0x34, 0.5);
    m.set_pv_f(MV + 4, f32::from_bits(0x3e4c_cccd));
    m.set_pv_f(MV + 0xc, 0.75);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc9_0fdc));
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
    m.set_pv_f(MV + 8, 0.75);
    ai::senses(m, f32::from_bits(SENSES[0]), f32::from_bits(SENSES[1]), f32::from_bits(SENSES[2]), f32::from_bits(SENSES[3]));
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 2);
    m.set_state(1, 0);
    set_seq(w, id, ctx, 7, 0, 0);
}

/// Its mouth: joint 0 and 0.25 along its first row (`0x3100e8`).
fn mouth(host: &dyn crate::moby::MobyHost, m: &Moby) -> V3 {
    let (rows, at) = host.joint_matrix(m, 0).unwrap_or((m.rows, m.pos));
    add(at, set_len(rows[0], 0.25))
}

/// The state (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        4 => {
            let m = &w.mobys[id];
            if 4.0 <= ctx.host.frame_time(m) && m.pv_i(BROKEN) == 0 && seq_a(w, id, ctx) == 0 { break_out(w, id, ctx); }
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.set_pv_f(HEIGHT, m.pv_f(HOME) - 7.0);
                m.set_state(7, -1);
                set_seq(w, id, ctx, 2, 0, 10);
            }
        }
        2 => if wrapped(w, id, ctx) {
            let n = crate::moby::classes::props::seq(w, id, ctx);
            if n as i32 != ctx.game.rng.randi(2) + 3 {
                let s = ctx.game.rng.randi(2) + 3;
                ctx.host.set_anim(&w.mobys[id], s as u8, 0, 2);
            }
            w.mobys[id].set_state(3, -1);
        },
        3 => if wrapped(w, id, ctx) {
            set_seq(w, id, ctx, 1, 0, 4);
            w.mobys[id].set_state(2, -1);
        },
        5 | 10 => if wrapped(w, id, ctx) {
            let blend = if w.mobys[id].state == 5 { 6 } else { 10 };
            w.mobys[id].set_state(7, -1);
            set_seq(w, id, ctx, 2, 0, blend);
        },
        7 => {
            if ai::reached(w, id, ctx, 13.0) { ctx.host.play_sound(&w.mobys[id], 4, 0); }
            if 17.0 < ctx.host.frame_time(&w.mobys[id]) && seq_a(w, id, ctx) == 2 {
                ctx.host.set_anim_speed(&w.mobys[id], 0.5);
                w.register_draw_first(id);
                charge_sparks(w, id, ctx);
                if wrapped(w, id, ctx) {
                    let m = &mut w.mobys[id];
                    let p = m.pv_v(T_BLOCK);
                    m.set_pv_v(AIM, p);
                    m.set_state(8, -1);
                    set_seq(w, id, ctx, 4, 20, 20);
                }
            }
        }
        8 => {
            w.register_draw_first(id);
            if 1.0 <= w.mobys[id].pv_f(GATHER) {
                ctx.host.play_sound(&w.mobys[id], 3, 0);
                w.mobys[id].set_state(9, -1);
            }
        }
        9 => beam(w, id, ctx),
        11 => {
            let t = ai::link(&w.mobys[id], T_MOBY).filter(|&t| w.mobys.get(t).is_some_and(|o| o.o_class == LURKER));
            if let Some(t) = t {
                let (p, q) = (w.mobys[id].pos, w.mobys[t].pos);
                face(w, t, atan2(p[0] - q[0], p[1] - q[1]));
            }
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.set_pv_f(HEIGHT, m.pv_f(HOME) - 4.0);
                if m.pv_i(FOUND1) != 1 && m.pv_i(FOUND2) != 1 {
                    m.set_state(7, 2);
                    set_seq(w, id, ctx, 2, 0, 10);
                } else {
                    m.set_state(2, 1);
                    set_seq(w, id, ctx, 1, 0, 10);
                }
            }
            if ai::reached(w, id, ctx, 11.0) && seq_a(w, id, ctx) == 6 {
                // The port's guard: never Ratchet's moby (the original deletes whatever `+0x200` names).
                if let Some(t) = ai::link(&w.mobys[id], T_MOBY).filter(|&t| Some(t) != ctx.hero_moby) {
                    w.level19.attackers -= 1;
                    w.delete(t);
                }
                let aim = ctx.hero.aim_point;
                let m = &mut w.mobys[id];
                m.set_pv_i(T_FOUND, 1);
                ai::set_link(m, T_MOBY, ctx.hero_moby);
                m.set_pv_v(T_BLOCK, aim);
            }
        }
        _ => {}
    }
}

/// State 4's break-out (module docs).
fn break_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.shakes.ask(0, 0.25, 0x26);
    let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = solid;
    m.set_pv_i(BROKEN, 1);
    if let Some(k) = ice(w, id) {
        for j in 0..5u8 {
            let mut v = ctx.game.rng.rand_vec(f32::from_bits(0x3e91_1112), f32::from_bits(0x3eb3_3334));
            v[2] = (v[2].abs() * 6.5).min(f32::from_bits(0x3fa0_0001));
            let c = &w.mobys[k];
            let (class, pos, rot) = (c.o_class, c.pos, c.rot);
            shards::spawn_ball(w, ctx, class, Some(k), pos, rot, 180.0, 45.0, j, v, 0xff);
        }
    }
    break_ice(w, id, ctx);
}

/// [`draw_charge`]'s sparks (`0x418d78`'s, made here: crate docs): while its glow is under 1.15 after this tick's
/// growth, twenty drawn in from 2.5..3 around its mouth (2 up) at 0.05 a tick (`0x33e538`: 0.2..0.5 × 210000 toward
/// 126000, 30 ticks, 0x804020, alpha 0 → 0x7f, frame 5).
fn charge_sparks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if f32::from_bits(0x3f93_3333) <= w.mobys[id].pv_f(GLOW) + f32::from_bits(0x3cf5_c28f) { return; }
    let at = add(mouth(&*ctx.host, &w.mobys[id]), [0.0, 0.0, 2.0]);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..20 {
        let spin = g.randf_signed(2.0, 5.0) as i32 as u8;
        let p = add(shell(g, 2.5, 3.0), at);
        let v = set_len(sub(at, p), f32::from_bits(0x3d4c_ccce));
        let size = g.randf(f32::from_bits(0x3e4c_cccd), 0.5) * f32::from_bits(0x484d_1400);
        crate::particles::burst(sys, g, size, f32::from_bits(0x47f6_1801), 0.0, p, v, 30, 0x80_4020, spin, 0, 0x7f, 5);
    }
}

/// State 9's beam (module docs).
fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.register_draw_first(id);
    let at = w.mobys[id].pv_v(BEAM_END);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let g = &mut ctx.game.rng;
        let fall = f32::from_bits(0x3b84_e478);
        for _ in 0..10 {
            let mut v = shell(g, f32::from_bits(0x3e6e_eef0), f32::from_bits(0x3e88_8889));
            if v[2] < 0.0 { v[2] = -v[2]; }
            let spin = g.randf_signed(2.0, 5.0) as i32 as u8;
            let f = g.randf(0.6, 1.0);
            crate::particles::burst(sys, g, f * f32::from_bits(0x484d_1400), 2100.0, fall, at, v, 20, 0x80_4020, spin, 0x7f, 0x7f, PUFF_FRAME);
            let spin = g.randf_signed(2.0, 5.0) as i32 as u8;
            let f = g.randf(0.6, 1.0);
            if let Some(i) = crate::particles::burst(sys, g, f * 630_000.0, 4200.0, fall, at, v, 20, 0x80_4040, spin, 0x30, 0x30, 0) {
                sys.pool.recs[i][2] = sys.def_first(0x35);
            }
        }
    }
    let m = &w.mobys[id];
    let from = mouth(&*ctx.host, m);
    let to = add(from, set_len(sub(m.pv_v(AIM), from), m.pv_f(LENGTH) * 60.0));
    let t = crate::moby::hits::HitTemplate::new(m.pv_b(DAMAGE) as f32, Some(id), 0x1_0001, [0.0; 4]);
    if let Some(o) = w.coll_line(ctx, from, to, 0, Some(id)) {
        if let Some(hit) = o.moby { w.deliver_hit_any(hit, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    }
    let m = &mut w.mobys[id];
    if 40 < m.state_ticks as u16 {
        m.set_pv_f(WIDTH, m.pv_f(WIDTH) - 0.05);
        if m.pv_f(WIDTH) <= 0.0 {
            for (o, v) in [(GATHER, 0.0), (G230, 1.0), (GLOW, 0.1), (WIDTH, 0.1), (LENGTH, 0.1)] { m.set_pv_f(o, v); }
            m.set_state(10, -1);
            set_seq(w, id, ctx, 3, 0, 10);
        }
    } else {
        m.set_pv_f(WIDTH, (m.pv_f(WIDTH) + 0.1).min(1.0));
        m.set_pv_f(LENGTH, (m.pv_f(LENGTH) + 0.2).min(1.0));
    }
    // [`draw_beam`]'s end (crate docs): from its mouth 1 up, as far as the world lets it (`0x2dc1b8(.., 2, m)`).
    let m = &w.mobys[id];
    let from = add(mouth(&*ctx.host, m), [0.0, 0.0, 1.0]);
    let to = add(from, set_len(sub(m.pv_v(AIM), from), m.pv_f(LENGTH) * 60.0));
    let end = w.coll_line(ctx, from, to, 2, Some(id)).map_or(to, |o| o.point);
    w.mobys[id].set_pv_v(BEAM_END, end);
}

/// Two glows of FX 0x31 at `p`, pushed 0.08 toward the camera at random rolls: `+0x228` × 2 in 0x60804020 and × 1.5
/// in 0x80808080.
fn glow_pair(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, p: V3, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let g = w.mobys[id].pv_f(GLOW);
    let push = f32::from_bits(PUSH);
    crate::fx::sprite(out, g + g, push, rng.rand_angle(), p, cam, 0x31, 0x6080_4020, true);
    crate::fx::sprite(out, g * 1.5, push, rng.rand_angle(), p, cam, 0x31, 0x8080_8080, true);
}

/// `0x418d78` (state 7's): the glow pair at its mouth 2 up ([`glow_pair`]); the glow grows 0.03 (its sparks:
/// [`charge_sparks`]).
pub fn draw_charge(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let at = add(mouth(host, &w.mobys[id]), [0.0, 0.0, 2.0]);
    glow_pair(w, id, rng, at, cam, out);
    let m = &mut w.mobys[id];
    m.set_pv_f(GLOW, m.pv_f(GLOW) + f32::from_bits(0x3cf5_c28f));
}

/// `0x418fc0` (state 8's): at its mouth 1 up a glow of FX 0x2a (`$gp+0x2bb0`) of 28.5 × `+0x22c` in 0x60808080 and
/// × 1.2 in 0xff804020 (no roll); `+0x22c` grows 0.12; then the glow pair there and at two random points 0.25..0.45
/// off it (a third drawn and not used).
pub fn draw_gather(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let at = add(mouth(host, &w.mobys[id]), [0.0, 0.0, 1.0]);
    let push = f32::from_bits(PUSH);
    let s = GATHER_SIZE * w.mobys[id].pv_f(GATHER);
    crate::fx::sprite(out, s, push, 0.0, at, cam, GATHER_FX, GATHER_RGBA, true);
    crate::fx::sprite(out, GATHER_SIZE * w.mobys[id].pv_f(GATHER) * f32::from_bits(0x3f99_999a), push, 0.0, at, cam, GATHER_FX, 0xff80_4020, true);
    let m = &mut w.mobys[id];
    m.set_pv_f(GATHER, m.pv_f(GATHER) + f32::from_bits(0x3df5_c28f));
    let _ = shell(rng, 0.25, f32::from_bits(0x3ee6_6666));
    glow_pair(w, id, rng, at, cam, out);
    for _ in 0..2 {
        let p = add(shell(rng, 0.25, f32::from_bits(0x3ee6_6666)), at);
        glow_pair(w, id, rng, p, cam, out);
    }
}

/// `0x4192a8` (state 9's): the glow pair at its mouth 1 up and at two random points 0.25..0.45 off it (a third drawn
/// and not used); the beam two
/// quads of FX 0xe (`0x2e4a70(0xe)`, additive) from there to its end (`+0x2c0`, crate docs), as wide as `+0x234` × 2
/// across the camera's view of each end, in 0x80804020 and 0x80806060.
pub fn draw_beam(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::cross;
    let at = add(mouth(host, &w.mobys[id]), [0.0, 0.0, 1.0]);
    let _ = shell(rng, 0.25, f32::from_bits(0x3ee6_6666));
    glow_pair(w, id, rng, at, cam, out);
    for _ in 0..2 {
        let p = add(shell(rng, 0.25, f32::from_bits(0x3ee6_6666)), at);
        glow_pair(w, id, rng, p, cam, out);
    }
    let m = &w.mobys[id];
    let end = m.pv_v(BEAM_END);
    let width = m.pv_f(WIDTH) * 2.0;
    let side = |p: V3| set_len(cross(set_len(sub(p, cam), 1.0), [0.0, 0.0, 1.0]), width);
    let (a, b) = (side(at), side(end));
    let corners = [sub(at, a), add(at, a), sub(end, b), add(end, b)];
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    for rgba in [0x8080_4020, 0x8080_6060] {
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners, st, rgba: [rgba; 4] });
    }
}

/// The frame's first list's callback (crate::fx): the one the state that registered it gave (the state before, when
/// it changed this tick: [`update_0f57`]'s tail has counted its ticks to 1).
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let state = if m.state_ticks == 1 { m.prev_state } else { m.state };
    match state {
        7 => draw_charge(w, host, id, rng, cam, out),
        8 => draw_gather(w, host, id, rng, cam, out),
        9 => draw_beam(w, host, id, rng, cam, out),
        _ => {}
    }
}
