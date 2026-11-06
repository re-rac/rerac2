//! Level 0x0e's director (the prison on Aranos; L14 addresses).

use crate::moby::director::{finish, gone, hold_tweak, let_tweak_go, link, mission_is_done, music_to, place_hero, place_hero_at, solid};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x10ff;

/// Its pvars (words).
mod pv {
    /// The camera tweak it holds (crate::moby::director::hold_tweak).
    pub const TWEAK: usize = 0x14;
    /// The cuboids: Clank's way out `+0x18`, Ratchet's way in `+0x1c`, the four of the tweak `+0x20..+0x2c`, the
    /// gate's place `+0x30` and the door's `+0x34`.
    pub const AT_EXIT: usize = 0x18;
    pub const AT_ENTER: usize = 0x1c;
    pub const AT_TWEAK: usize = 0x20;
    pub const GATE_PLACE: usize = 0x30;
    pub const DOOR_PLACE: usize = 0x34;
    /// The group it hooks.
    pub const GROUP: usize = 0x38;
    /// The mobys: Clank's body `+0x3c`, the panel `+0x40`, the beam `+0x44`, the guard `+0x48`, the switch `+0x4c`,
    /// the door `+0x50`.
    pub const CLANK: usize = 0x3c;
    pub const PANEL: usize = 0x40;
    pub const BEAM: usize = 0x44;
    pub const GUARD: usize = 0x48;
    pub const SWITCH: usize = 0x4c;
    pub const DOOR: usize = 0x50;
    /// The missions: the guard beaten `+0x54`, Clank out `+0x58`, Ratchet free `+0x5c`, the door open `+0x60`, the
    /// panel `+0x64`.
    pub const M_GUARD: usize = 0x54;
    pub const M_OUT: usize = 0x58;
    pub const M_FREE: usize = 0x5c;
    pub const M_DOOR: usize = 0x60;
    pub const M_PANEL: usize = 0x64;
    /// The fade's level `+0x68` (a float), its ticks `+0x6c`, the state after the wait `+0x70`, the wait `+0x74`, the
    /// fade's length `+0x78`.
    pub const LEVEL: usize = 0x68;
    pub const FADE_T: usize = 0x6c;
    pub const NEXT: usize = 0x70;
    pub const WAIT: usize = 0x74;
    pub const FADE_LEN: usize = 0x78;
    /// The music: Clank's `+0x7c`, Ratchet's `+0x80`, the end's `+0x84`, their stinger `+0x88`; the end's frames
    /// `+0x8c`.
    pub const MUSIC_CLANK: usize = 0x7c;
    pub const MUSIC_RATCHET: usize = 0x80;
    pub const MUSIC_END: usize = 0x84;
    pub const STINGER: usize = 0x88;
    pub const FRAMES: usize = 0x8c;
}

/// The tweak's distance and pivot height (`$gp+0x2768` / `+0x276c`).
const TWEAK_DIST: f32 = 2.0;
const TWEAK_PIVOT: f32 = 3.0;
/// The flag byte the guard's defeat sets (`0x1395e1`).
const GUARD_FLAG: usize = 0x29;

/// 0x10ff (`0x427b88`, the decomp's `func_L14_004265B8`): the prison's flow — Ratchet locked up, Clank's escape,
/// the guard, the panel, the gate and Ratchet freed. Ratchet's and Clank's turns are the body switch
/// (crate::hero::body: `0x2ba348` = L00 `0x2b2b48`, Clank's body in state 0x3f on `+0x3c`; back `0x2ba558` = L02
/// `0x2b60d8`).
/// * **0**: update distance 0xff, the fade's length 125. With Ratchet free (`+0x5c` done): in cuboid `+0x1c` he is put
///   at the door's place, Clank back (items 2 and 3 had, `0x1a7b02` / `0x1a7b03` and their copies `0x1a7b3a` /
///   `0x1a7b3b`; on his back, `0x18c31c` 0); with the door's mission `+0x60` the group `+0x38` and the door `+0x50`
///   hooked; then 7. Else Clank taken (the items 0, `0x18c31c` 1) and by the missions done: Clank out `+0x58` → the
///   group hooked, 5; the panel `+0x64` → hooked, 4; the guard `+0x54` → 3; none → 2.
/// * **2** (Clank in the cell): on its first tick Ratchet put on Clank's body in state 0x6f, the body switched to Clank
///   (0x3f), Clank's music `+0x7c`. The tweak ([`tweak`]). Once the guard `+0x48` is down ([`guard_down`]): the flag
///   `0x1395e1`, the guard's mission done and the checkpoint (`0x3642d0(m + 0xf0, m + 0xf0)`: its angles for both),
///   then the fade (6) into 3.
/// * **3** (Ratchet at the panel): on its first tick back to Ratchet, put at its place (state 0), the group hooked, his
///   health full (`0x1a7a08`), Ratchet's music `+0x80`. Once the panel `+0x40` is in its state 2 or 3: the panel's
///   mission done and the checkpoint at the guard, then the fade into 4.
/// * **4** (Clank to the gate): on its first tick the panel's `+0x84` 1, the guard deleted, the beam `+0x44` hidden
///   and passed through, Ratchet put at the guard in state 0x6f and switched to Clank, Clank's music. The tweak. In
///   cuboid `+0x18`: Clank's mission done and the checkpoint (`0x3642d0(&pos, panel + 0xf0)`, its place an unset
///   local here: Ratchet's), then the fade into 5.
/// * **5** (Ratchet at the gate): on its first tick Clank's body deleted, the beam shown and solid, back to Ratchet
///   (when Clank plays), put at the gate's place (state 0), the panel to its state 2, his health full, Ratchet's
///   music. Out of a scene, once the switch `+0x4c` is gone (state past 0x80): Ratchet's mission done, the
///   checkpoint at the door's place, Clank back, then 7.
/// * **6** (the wait): on its first tick the wait the fade's length, the fade's ticks twice it, Ratchet to state 0x6f
///   (`0x2cbee0` = L00 `0x2c4560`); the wait out → the state `+0x70`.
/// * **7**: from its sixth frame out of a scene, the end's music `+0x84` unless it plays.
/// * Each tick with the fade's ticks: one down, the level `+0x68` the sine (`0x2f7028` = L00 `0x2ef460`) of
///   ticks · π/2 / the length.
///
/// Not ported: the debug state 1 (with `0x1ff424` set: "DEBUG: Play as Clank" in the second box and the prompt's lease
/// in cuboid `+0x1c`, △ for 2); the fade's draw (`0x4283b8`, on the frame's second list: a screen fill of alpha
/// (1 − `+0x68`) · 128 in the blend `0x387180(0x42, 9)`, which crate::moby::MobyWorld::screen_fills, additive, cannot
/// take); the items' "ever had" copies (`0x1a7b38`, not typed).
pub fn update_10ff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    let done = |ctx: &Ctx, w: &MobyWorld, o: usize| mission_is_done(ctx.game, w.mobys[id].pv_i(o));
    let clank = link(w, p(w, pv::CLANK));
    let panel = link(w, p(w, pv::PANEL));
    let guard = link(w, p(w, pv::GUARD));
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_pv_i(pv::FADE_LEN, 0x7d);
            let next = if done(ctx, w, pv::M_FREE) {
                if ctx.level.in_cuboid(p(w, pv::AT_ENTER), ctx.hero.pos) {
                    place_hero_at(ctx, p(w, pv::DOOR_PLACE), 0);
                    clank_items(ctx, true);
                }
                if done(ctx, w, pv::M_DOOR) {
                    hook_group(w, id);
                    if let Some(d) = link(w, p(w, pv::DOOR)) { w.mobys[d].once |= 4; }
                }
                7
            } else {
                clank_items(ctx, false);
                if done(ctx, w, pv::M_OUT) {
                    hook_group(w, id);
                    5
                } else if done(ctx, w, pv::M_PANEL) {
                    hook_group(w, id);
                    4
                } else if done(ctx, w, pv::M_GUARD) {
                    3
                } else {
                    // `0x1ff424` (the debug flag) would take 1.
                    2
                }
            };
            w.mobys[id].set_state(next, -1);
        }
        2 => {
            if w.mobys[id].first() {
                if let Some(c) = clank {
                    let (pos, rot) = (w.mobys[c].pos, w.mobys[c].rot);
                    place_hero(ctx, pos, rot, 0x6f);
                    ctx.hero.switch_body(1, 0x3f, c);
                }
                music_to(ctx, p(w, pv::MUSIC_CLANK), p(w, pv::STINGER));
            }
            tweak(w, id, ctx);
            if guard_down(w, guard) {
                if let Some(b) = ctx.game.flags.get_mut(GUARD_FLAG) { *b = 1; }
                let rot = w.mobys[id].rot;
                finish(ctx, p(w, pv::M_GUARD), rot, rot);
                next_after_fade(w, id, 3);
            }
        }
        3 => {
            if w.mobys[id].first() {
                ctx.hero.body_back(ctx.game.scenes.mode == crate::scene::SCENE);
                let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
                place_hero(ctx, pos, rot, 0);
                hook_group(w, id);
                ctx.hero.hp = ctx.game.max_health;
                music_to(ctx, p(w, pv::MUSIC_RATCHET), p(w, pv::STINGER));
            }
            if panel.is_some_and(|k| matches!(w.mobys[k].state, 2 | 3)) {
                let (pos, rot) = guard.map_or((ctx.hero.pos, ctx.hero.rot), |g| (w.mobys[g].pos, w.mobys[g].rot));
                finish(ctx, p(w, pv::M_PANEL), pos, rot);
                next_after_fade(w, id, 4);
            }
        }
        4 => {
            if w.mobys[id].first() {
                if let Some(k) = panel { w.mobys[k].set_pv_i(0x84, 1); }
                let at = guard.map(|g| (w.mobys[g].pos, w.mobys[g].rot));
                if let Some(g) = guard { if !gone(w, g) { w.delete(g); } }
                if let Some(b) = link(w, p(w, pv::BEAM)) { solid(w, b, false); }
                if let Some((pos, rot)) = at { place_hero(ctx, pos, rot, 0x6f); }
                if let Some(c) = clank { ctx.hero.switch_body(1, 0x3f, c); }
                music_to(ctx, p(w, pv::MUSIC_CLANK), p(w, pv::STINGER));
            }
            tweak(w, id, ctx);
            if ctx.level.in_cuboid(p(w, pv::AT_EXIT), ctx.hero.pos) {
                let rot = panel.map_or(ctx.hero.rot, |k| w.mobys[k].rot);
                let pos = ctx.hero.pos;
                finish(ctx, p(w, pv::M_OUT), pos, rot);
                next_after_fade(w, id, 5);
            }
        }
        5 => {
            if w.mobys[id].first() {
                if let Some(c) = clank { w.delete(c); }
                if let Some(b) = link(w, p(w, pv::BEAM)) { solid(w, b, true); }
                if ctx.hero.mode == 1 { ctx.hero.body_back(ctx.game.scenes.mode == crate::scene::SCENE); }
                place_hero_at(ctx, p(w, pv::GATE_PLACE), 0);
                if let Some(k) = panel { w.mobys[k].set_state(2, -1); }
                ctx.hero.hp = ctx.game.max_health;
                music_to(ctx, p(w, pv::MUSIC_RATCHET), p(w, pv::STINGER));
            }
            if ctx.game.scenes.mode != crate::scene::SCENE && link(w, p(w, pv::SWITCH)).is_some_and(|k| 0x80 < w.mobys[k].state) {
                if let Some((pos, rot)) = ctx.level.cuboid_place(p(w, pv::DOOR_PLACE)) { finish(ctx, p(w, pv::M_FREE), pos, rot); }
                clank_items(ctx, true);
                w.mobys[id].set_state(7, -1);
            }
        }
        6 => {
            let m = &mut w.mobys[id];
            if m.first() {
                let len = m.pv_i(pv::FADE_LEN);
                m.set_pv_i(pv::WAIT, len);
                m.set_pv_i(pv::FADE_T, len * 2);
                ctx.hero.request_state(0x6f, true);
            }
            let mut t = m.pv_i(pv::WAIT);
            let out = countdown(&mut t);
            m.set_pv_i(pv::WAIT, t);
            if out {
                let next = m.pv_i(pv::NEXT);
                m.set_state(next as u8, -1);
            }
        }
        7 => {
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_i(pv::FRAMES, 0); }
            if ctx.game.scenes.mode != crate::scene::SCENE && 6 <= m.pv_i(pv::FRAMES) {
                let (track, stinger) = (m.pv_i(pv::MUSIC_END), m.pv_i(pv::STINGER));
                music_to(ctx, track, stinger);
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::FRAMES, m.pv_i(pv::FRAMES).wrapping_add(1));
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.pv_i(pv::FADE_T) != 0 {
        let mut t = m.pv_i(pv::FADE_T);
        countdown(&mut t);
        m.set_pv_i(pv::FADE_T, t);
        let len = m.pv_i(pv::FADE_LEN) as f32;
        m.set_pv_f(pv::LEVEL, (t as f32 * std::f32::consts::FRAC_PI_2 / len).sin());
    }
}

/// `+0x70` = `next`, then the wait (6).
fn next_after_fade(w: &mut MobyWorld, id: usize, next: i32) {
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::NEXT, next);
    m.set_state(6, -1);
}

/// `0x4282e8`: every member of the group `+0x38` hooked (`0x333698` = L00 `0x32a818`).
fn hook_group(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].pv_i(pv::GROUP);
    if g == -1 { return; }
    for k in crate::moby::director::members(w, g) { w.mobys[k].once |= 4; }
}

/// Clank with Ratchet (`on`: items 2 and 3 had, on his back) or taken away.
fn clank_items(ctx: &mut Ctx, on: bool) {
    for k in [2, 3] {
        if let Some(b) = ctx.game.items.get_mut(k) { *b = on as u8; }
    }
    ctx.game.clank = on;
    ctx.hero.body.clank_away = !on;
}

/// `0x4287f0(guard)`: the guard gone (deleted, or no moby) or its `+0x04` set.
fn guard_down(w: &MobyWorld, guard: Option<usize>) -> bool {
    match guard {
        Some(g) if !gone(w, g) => w.mobys[g].pv_i(4) != 0,
        _ => true,
    }
}

/// `0x428430`: Ratchet in one of the cuboids `+0x20..+0x2c` (−1 none) holds the camera tweak at 2 back, 3 up; in none
/// it is let go.
fn tweak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for k in 0..4 {
        let c = w.mobys[id].pv_i(pv::AT_TWEAK + 4 * k);
        if c != -1 && ctx.level.in_cuboid(c, ctx.hero.pos) {
            return hold_tweak(w, id, ctx, pv::TWEAK, [Some(TWEAK_DIST), Some(TWEAK_PIVOT), None]);
        }
    }
    if w.mobys[id].pv_i(pv::TWEAK) != 0 { let_tweak_go(w, id, ctx, pv::TWEAK); }
}
