//! The prison's launch plates **0x0f4d** (2 placed; `0x3cc728`): a plate on a hinge that, zapped by a 0x0a59 robot
//! (Clank's command 7, crate::moby::classes::endako_d, which hooks it), swings up and throws what stands on it — the
//! robots of its group and Clank — over to its cuboid.
//!
//! **Its pvars:** `+0x20` its platform block (crate::moby::platform), `+0x60` the cuboid it throws to, `+0x64` its
//! group of prisoners, `+0x70` its hinge (its place 0.581 up, `$gp+0xecc`), `+0x80` / `+0x84` the swing's angle and
//! speed.
//!
//! **0** the set-up: the block's flag 1, the hinge; a second 0x0f4d made at its place (`0x318cf0` + `0x3335c8`: the
//! base — no update, never updated (`+0x30` 0), at its draw distance, its piece 0 alone (`+0x7c` = 0x8001), its
//! collision primitive 0 and bit 31 off (`+0x9c` = 0x80000001)), its own primitives 1 and 2 off (`+0x9c` = 6); 1.
//! **1** waiting: its first tick the block written still (`0x32a0a0` = L00 `0x322668`); with Clank standing on it
//! (`0x18a15c`) its prisoners kept on it ([`keep`], `0x3ccad8`); hooked, it lets the hook go, throws ([`throw`],
//! `0x3ccd38`) and swings (2). **2** the swing: its first tick at 360° a second, slowed by 90° a second²; past 40° held
//! there, or under 0 held at 0, its speed turned back at half (−0.5) with its class sound 0 (`0x36ada0`) — at 0 and no
//! faster than 3° a second back to 1. The help 0x60 off (`0x18cdd8`). The angle its pitch about the hinge
//! ([`swing`], `0x3cc9a8`), the move and the turn into its block.
//!
//! **Its prisoners kept on it** ([`keep`], `0x3ccad8`): each living member of its group but the 0x0a59s, following
//! (sub-state 1, `0x3caa18`), more than 0.8 off the plate's middle (2 behind it) along or across in its frame, is sent
//! (crate::moby::classes::endako_d::send_to, `0x3cab28`) to the nearest place on it within 1.5..2.5 behind and 0.5
//! across.
//!
//! **The throw** ([`throw`], `0x3ccd38`): each living member of its group within 1 of the middle along and across is
//! launched (crate::moby::classes::endako_d::launch, `0x3cabb0`, 1/180 a tick² up and down) to its place on the
//! plate taken through the cuboid's matrix, over a top 1..2 over the cuboid's height (`+0x38`); Clank on it on the
//! ground, or less than 0.5 over the ground, and within the same square, likewise into the state 0x44 (his hop,
//! crate::hero::clank) with the velocity that takes him there over a top 1.5 over it.

use crate::hero::physics::{add, euler_rows, scale, sub, through, V3};
use crate::moby::classes::endako_d;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = endako_d::PLATE;

const BLOCK: usize = 0x20;
const CUBOID: usize = 0x60;
const GROUP: usize = 0x64;
const HINGE: usize = 0x70;
const ANGLE: usize = 0x80;
const SPEED: usize = 0x84;
/// `$gp+0xecc`: the hinge's height over its place.
const PIVOT: f32 = f32::from_bits(0x3f14_bc6a);
/// `$gp+0xebc` / `+0xeb8` / `+0xec4` / `+0xec0` / `+0xec8`: the swing's start (360° a second), its slowing (90° a
/// second²), its top (40°), its bounce (−0.5), its rest (3° a second).
const KICK: f32 = 360.0 * 0.000_290_888_25;
const PULL: f32 = 90.0 * 4.848_137_5e-6;
const TOP: f32 = 40.0 * 0.017_453_292;
const BOUNCE: f32 = -0.5;
const REST: f32 = 3.0 * 0.000_290_888_25;
/// The throw's gravity (a tick²) up and down.
const G: f32 = 0.005_555_555_7;
/// The help slot the swing turns off.
const HELP: usize = 0x60;

fn live(w: &MobyWorld, i: usize) -> bool { w.mobys.get(i).is_some_and(|m| m.state != 0xfe && m.state != 0xfd) }

/// Its group's members.
fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    let g = w.mobys[id].pv_i(GROUP);
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// 0x0f4d (module docs).
pub fn update_0f4d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => setup(w, id, ctx),
        1 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                let r = m.rot;
                crate::moby::platform::write(m, BLOCK, [0.0; 3], r, r);
            }
            if ctx.hero.ground_moby == Some(id) { keep(w, id, ctx); }
            if w.mobys[id].once & 4 != 0 {
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                m.once &= !4;
                throw(w, id, ctx);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_f(SPEED, KICK); }
            let v = m.pv_f(SPEED) - PULL;
            let a = m.pv_f(ANGLE) + v;
            m.set_pv_f(SPEED, v);
            m.set_pv_f(ANGLE, a);
            if TOP < a {
                m.set_pv_f(ANGLE, TOP);
                m.set_pv_f(SPEED, v * BOUNCE);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            } else if a < 0.0 {
                m.set_pv_f(ANGLE, 0.0);
                m.set_pv_f(SPEED, v * BOUNCE);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                if w.mobys[id].pv_f(SPEED) <= REST { w.mobys[id].set_state(1, -1); }
            }
            if let Some(c) = ctx.game.help_counts.get_mut(HELP) { *c = -1; }
            swing(w, id);
        }
        _ => {}
    }
}

/// The set-up (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_state(1, -1);
    m.set_pv_i(BLOCK + 0x3c, m.pv_i(BLOCK + 0x3c) | 1);
    let hinge = add(m.pos, [0.0, 0.0, PIVOT]);
    m.set_pv_v(HINGE, hinge);
    if w.class(CLASS).is_none() { return; }
    let c = w.create(CLASS);
    let (pos, rot, light, ambient, draw) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light, m.ambient, m.draw_dist) };
    let x = &mut w.mobys[c];
    x.parent = Some(id);
    x.pos = pos;
    x.rot = rot;
    x.light = light;
    x.ambient = ambient;
    w.rebuild(c, &*ctx.host);
    let x = &mut w.mobys[c];
    x.draw_dist = draw;
    x.mode |= crate::moby::mode::NO_UPDATE;
    x.coll_disable = 0x8000_0001;
    x.update_dist = 0;
    x.piece = Some(0);
    x.bangles = 0;
    w.mobys[id].coll_disable = 6;
    w.changed = true;
}

/// `0x3cc9a8`: the swing's angle its pitch (`+0xf4`); its rows again (`0x2f7280`), and its place moved so the hinge
/// stays where it was; the move and the turn into its platform block (`0x32a0a0`).
fn swing(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let old = m.rot;
    m.rot[1] = m.pv_f(ANGLE);
    m.rows = euler_rows(m.rot);
    let at = add(through(&m.rows, [0.0, 0.0, PIVOT]), m.pos);
    let d = sub(m.pv_v(HINGE), at);
    m.pos = add(m.pos, d);
    let new = m.rot;
    crate::moby::platform::write(m, BLOCK, d, old, new);
    w.changed = true;
}

/// `0x3ccad8` (module docs).
fn keep(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    for e in group(w, id) {
        if !live(w, e) || w.mobys[e].o_class == endako_d::ROBOT_A59 || !endako_d::sub_one(w, e) { continue; }
        let mut off = crate::gravity::to_local(sub(w.mobys[e].pos, pos), &rows);
        let mut clamped = false;
        if 0.8 < (off[0] + 2.0).abs() {
            clamped = true;
            off[0] = off[0].clamp(-2.5, -1.5);
        }
        if 0.8 < off[1].abs() {
            clamped = true;
            off[1] = off[1].clamp(-0.5, 0.5);
        }
        if clamped {
            let p = add(through(&rows, off), pos);
            endako_d::send_to(w, e, ctx, p);
        }
    }
}

/// `0x2ef340`: `p` (w 1) through the cuboid's matrix.
fn into_cuboid(m: &[[f32; 4]; 4], p: V3) -> V3 { std::array::from_fn(|k| p[0] * m[0][k] + p[1] * m[1][k] + p[2] * m[2][k] + m[3][k]) }

/// `0x3ccd38` (module docs).
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(cub) = usize::try_from(w.mobys[id].pv_i(CUBOID)).ok().and_then(|k| ctx.level.cuboids.get(k)).map(|c| c.matrix) else { return };
    let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    let on = |p: V3| {
        let mut off = crate::gravity::to_local(sub(p, pos), &rows);
        off[0] += 2.0;
        (off[0].abs() < 1.0 && off[1].abs() < 1.0).then_some(off)
    };
    for e in group(w, id) {
        if !live(w, e) { continue; }
        let Some(off) = on(w.mobys[e].pos) else { continue };
        let to = into_cuboid(&cub, off);
        let peak = cub[3][2] + ctx.game.rng.randf(1.0, 2.0);
        endako_d::launch(w, e, to, peak, G, G);
    }
    let h = &mut *ctx.hero;
    if !((h.ground_moby == Some(id) && h.grounded_ticks != 0) || h.height < 0.5) { return; }
    let Some(off) = on(h.pos) else { return };
    let to = into_cuboid(&cub, off);
    let a = cub[3][2] + 1.5;
    let b = (a - h.pos[2]) * 2.0;
    let t = (b / G).sqrt() + ((a - to[2]) * 2.0 / G).sqrt();
    let mut v = scale(sub(to, h.pos), 1.0 / t);
    v[2] = (b * G).sqrt();
    h.request_state(0x44, true);
    h.vel = v;
}
