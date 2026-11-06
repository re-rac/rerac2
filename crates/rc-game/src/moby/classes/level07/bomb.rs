//! 0x0e52 (`0x3f7d00`, 3 placed; the decomp's `func_L07_003F6960`): the boss's homing bomb
//! (crate::moby::classes::level07::boss). It waits hidden (1) in the boss's group `+0x184` until the boss launches
//! it ([`launch`], `0x3f7fe0`), chases Ratchet at a hover height and blows up on touching anything but the boss.
//!
//! * Every tick but in state 1 ([`hurt`], `0x3f8090`): **0** sets it up (scale `$gp+0x1fa0` of its class's, state 1,
//!   hidden without collision); otherwise a hit's damage (`0x10000`) comes off its health (`+0x20`) and at none it
//!   blows up (3).
//! * **2**: its scale kept; the chase ([`chase`], `0x3f8160`); its trail ([`trail`], `0x3f82a0`); any moby within
//!   0.333 but itself and the boss → 3.
//! * **3**: the boss's class sound 6 at it; two blasts (`0x313428`: 0x10000 within 6 for 5, pushed 1; flag 1 within
//!   4 for its damage byte `+0x34`); the big explosion facing away from the camera (`0x3f0768`); the camera's shakes
//!   (slot 0: 0.2 for 60 ticks; slot 2: 0.1 for 90); hidden without collision, 1.
//!
//! pvars: `+0x20` health (`+0x24` its most), `+0x50` the drift, `+0x5c` the height's speed, `+0x60` the hover
//! height, `+0x64` the chase's slant (radians). The boss is its owner (`+0xb8`, the port's link at `+0x30`).

use crate::hero::physics::{add, set_len, sub, V3};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e52;
/// The boss: the bomb's sound 6 is its.
const BOSS: i16 = 0x0e5f;

/// `$gp+0x1fa0`: its scale against its class's.
const SCALE: f32 = 0.75;
/// `$gp+0x1fa8` / `+0x1fac`: the chase's slant, degrees either way.
const SLANT: (f32, f32) = (20.0, 40.0);
/// `$gp+0x1fb0`: the drift's change, a second² (/3600); `+0x1fb4` its speed a second.
const ACCEL: f32 = 8.0;
const SPEED: f32 = 3.0;
/// `$gp+0x1fb8`: the trail's sink, 0..1 a second; `+0x1fc0` / `+0x1fc4` the glints' sizes, `+0x1fc8` / `+0x1fcc`
/// their colours, `+0x1fd0..+0x1fdc` their lives, `+0x1fe0` how many fall.
const SINK: (f32, f32) = (0.0, 1.0);
const GLINT_SIZE: [f32; 2] = [1.5, 1.0];
const GLINT_RGBA: [u32; 2] = [0x6000_6080, 0x6000_4090];
const GLINT_LIFE: [(i32, i32); 2] = [(5, 10), (40, 90)];
const FALLING: i32 = 1;

const HIDE: u16 = 0x41;

/// `0x3f7f28(group)`: the first of the group waiting (state 1).
pub fn waiting(w: &MobyWorld, group: i32) -> Option<usize> {
    let g = w.groups.get(usize::try_from(group).ok()?)?;
    g.iter().map(|&i| i as usize).find(|&i| w.mobys.get(i).is_some_and(|m| m.state == 1))
}

/// `0x3f7fe0(bomb, owner, pos, height)`: launched from `pos` by `owner`, its drift stopped, health full, hovering at
/// `height`, slanted ±20..40°, shown with its class's collision, 2.
pub fn launch(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, owner: usize, pos: V3, height: f32) {
    let slant = ctx.game.rng.randf_signed(SLANT.0, SLANT.1) * 0.017_453_294;
    let has = w.class(CLASS).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    ai::set_link(m, 0x30, Some(owner));
    m.pos = pos;
    m.set_pv_v(0x50, [0.0; 3]);
    m.set_pv_f(0x20, m.pv_s(0x24) as f32);
    m.set_pv_f(0x60, height);
    m.set_pv_f(0x64, slant);
    m.mode = (m.mode & !HIDE) | 0x1000;
    m.has_collision = has;
    m.set_state(2, -1);
}

pub fn update_0e52(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state != 1 { hurt(w, id, ctx); }
    match w.mobys[id].state {
        2 => {
            set_scale(w, id);
            let hero = ctx.hero.pos;
            chase(w, id, hero);
            trail(w, id, ctx);
            let owner = ai::link(&w.mobys[id], 0x30);
            let pos = w.mobys[id].pos;
            let touched = ctx.scene.map_or(Vec::new(), |sc| crate::collision::mobys::coll_sphere_mobys(sc, pos, 0.333, crate::collision::QueryFlags(0), Some(id)));
            if touched.iter().any(|&o| o != id && Some(o) != owner) { w.mobys[id].set_state(3, -1); }
        }
        3 => {
            ctx.host.play_sound_of(&w.mobys[id], BOSS, 6, 0);
            let pos = w.mobys[id].pos;
            let damage = w.mobys[id].pv_b(0x34) as f32;
            ai::attack_sphere(w, id, ctx, 6.0, 5.0, 1.0, pos, 0x10000, 0, 1, 0);
            ai::attack_sphere(w, id, ctx, 4.0, damage, 1.0, pos, 1, 0, 1, 0);
            let dir = set_len(sub(pos, ctx.host.camera_pos()), 1.0);
            crate::moby::classes::explosion::start(w, ctx, 0.0, pos, dir);
            ctx.game.shakes.ask(2, 0.1, 0x5a);
            ctx.game.shakes.ask(0, 0.2, 0x3c);
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | HIDE) & 0xefff;
            m.set_state(1, -1);
        }
        _ => {}
    }
}

fn set_scale(w: &mut MobyWorld, id: usize) {
    let s = w.class(CLASS).map_or(1.0, |c| c.scale) * SCALE;
    w.mobys[id].scale = s;
}

/// `0x3f8090`: the set-up (state 0), else the hit's damage off its health; at none, 3; the hit let go.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        set_scale(w, id);
        let m = &mut w.mobys[id];
        m.set_state(1, -1);
        m.has_collision = false;
        m.mode = (m.mode | HIDE) & 0xefff;
        return;
    }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let d = ai::damage_of(w, id, hit.as_ref(), false);
    let mut health = w.mobys[id].pv_f(0x20);
    crate::hero::physics::approach(0.0, d, &mut health);
    w.mobys[id].set_pv_f(0x20, health);
    if health <= 0.0 { w.mobys[id].set_state(3, -1); }
    w.clear_hit(id);
}

/// `0x3f8160(m, target)`: its height eased to its hover height; the drift steered toward the target's way (flat,
/// [`SPEED`], turned by its slant) by at most [`ACCEL`]; moved by it.
fn chase(w: &mut MobyWorld, id: usize, target: V3) {
    let m = &mut w.mobys[id];
    let rate = ACCEL / 3600.0;
    let top = SPEED / 60.0;
    let mut off = sub(target, m.pos);
    off[2] = 0.0;
    let (mut z, mut vz) = (m.pos[2], m.pv_f(0x5c));
    crate::hero::physics::ease(m.pv_f(0x60), rate, rate, top, &mut z, &mut vz);
    m.pos[2] = z;
    m.set_pv_f(0x5c, vz);
    let v = set_len(off, top);
    let (s, c) = m.pv_f(0x64).sin_cos();
    let v = [v[0] * c - v[1] * s, v[0] * s + v[1] * c, v[2]];
    let mut drift = m.pv_v(0x50);
    let mut d = sub(v, drift);
    if rate < crate::hero::physics::len(d) { d = set_len(d, rate); }
    drift = add(drift, d);
    m.set_pv_v(0x50, drift);
    m.pos = add(m.pos, drift);
}

/// `0x3f82a0`: a glint at it (still), then [`FALLING`] more within 0.5 of it sinking.
fn trail(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let hero = ctx.hero.pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let life = g.rand_range(GLINT_LIFE[0].0, GLINT_LIFE[0].1) as u16;
    let rot = g.randi(0xff) as u8;
    crate::particles::glint(sys, GLINT_SIZE[0], pos, [0.0; 3], GLINT_RGBA[0], life, rot, false, hero);
    for _ in 0..FALLING {
        let p = add(g.rand_vec(0.0, 0.5), pos);
        let vz = -(g.randf(SINK.0, SINK.1) * 0.016_666_668);
        let life = g.rand_range(GLINT_LIFE[1].0, GLINT_LIFE[1].1) as u16;
        let rot = g.randi(0xff) as u8;
        crate::particles::glint(sys, GLINT_SIZE[1], p, [0.0, 0.0, vz], GLINT_RGBA[1], life, rot, false, hero);
    }
}
