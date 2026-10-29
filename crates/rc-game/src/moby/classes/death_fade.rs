//! The dying mobys' fade (L01 `0x3eb1d0`, the same code on every level with it): a dead creature, frozen, fades out
//! over a second toward cyan and goes (or comes back through a callback). The fades are kept in one list
//! (`0x2502c0`, at most 64, the count `$gp + 0x4280`) that a manager moby of class 0xc41 runs as its update
//! (`0x3eacd8`; `0x3eb1d0` makes the manager when there is none, update distance 0xff).
//!
//! * **Start** (`0x3eb1d0(m, rate, done)`, [`start`]): the moby frozen (mode |= 2, its playback speed `+0x48` and
//!   blend `+0x44` 0), its ambient kept, the factor 1; with the list full it goes at once (or `done` runs).
//! * **Tick** (`0x3eacd8`, [`update`]): each fade's factor steps toward 0 by 1/60 (`$gp + 0x1b44` = 1.0 a second; the
//!   start's rate is kept but not read), its moby's alpha `f·96` and its ambient between its own (f = 1) and
//!   [`FADE_TO`] (`0x3eb0f8`); at 0 it is deleted, or with `done` its colour, alpha 0x80 and updates come back and
//!   `done` runs. A finished fade's slot takes the list's last (which then waits a tick).
//!
//! Not ported: the puffs each fading moby gives off (`0x3eae90`: up to 32 staged puffs, crate::particles::staged, on
//! points of its model `0x3141a0`).

use crate::moby::{mode, Ctx, MobyWorld, UpdateFn};

/// The manager's class.
pub const CLASS: i16 = 0x0c41;
/// The list's size.
pub const MAX: usize = 64;
/// `0x1b0b28` (L01): the colour a fading moby goes toward.
pub const FADE_TO: u32 = 0x0080_8000;

/// One fade (`0x2502c0 + 0x20·k`: the moby, its ambient, the rate, the factor, the callback).
#[derive(Clone, Copy, Debug)]
pub struct Fade {
    pub moby: usize,
    pub ambient: [u8; 3],
    pub rate: f32,
    pub f: f32,
    pub done: Option<UpdateFn>,
}

/// `0x3eb1d0(m, rate, done)` (module docs).
pub fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, rate: f32, done: Option<UpdateFn>) {
    let live = w.fade_manager.filter(|&k| w.mobys.get(k).is_some_and(|m| m.o_class == CLASS && m.state < 0x80));
    if live.is_none() {
        let k = w.create(CLASS);
        let m = &mut w.mobys[k];
        m.update = Some(update);
        m.mode &= !mode::NO_UPDATE;
        m.update_dist = 0xff;
        w.fade_manager = Some(k);
    }
    ctx.host.set_anim_speed(&w.mobys[id], 0.0);
    w.mobys[id].mode |= mode::NO_UPDATE;
    if w.fades.len() < MAX {
        let ambient = w.mobys[id].ambient;
        w.fades.push(Fade { moby: id, ambient, rate, f: 1.0, done });
    } else if let Some(d) = done {
        d(w, id, ctx);
    } else {
        w.delete(id);
    }
}

/// `0x3eacd8`, the manager's update (module docs).
pub fn update(w: &mut MobyWorld, _id: usize, ctx: &mut Ctx) {
    let mut k = 0;
    while k < w.fades.len() {
        let f = w.fades[k];
        if f.f == 0.0 {
            match f.done {
                None => w.delete(f.moby),
                Some(d) => {
                    let m = &mut w.mobys[f.moby];
                    m.ambient = f.ambient;
                    m.alpha = 0x80;
                    m.mode &= !mode::NO_UPDATE;
                    d(w, f.moby, ctx);
                }
            }
            let last = w.fades.pop();
            if k < w.fades.len() { if let Some(l) = last { w.fades[k] = l; } }
        } else {
            let mut v = f.f;
            crate::hero::physics::approach(0.0, 1.0 / 60.0, &mut v);
            w.fades[k].f = v;
            // `0x3eb0f8`: the alpha and the colour.
            let saved = f.ambient[0] as u32 | (f.ambient[1] as u32) << 8 | (f.ambient[2] as u32) << 16;
            let c = crate::particles::tween_color(v.to_bits(), FADE_TO, saved);
            let m = &mut w.mobys[f.moby];
            m.alpha = (v * 96.0) as i32 as u8;
            m.set_ambient_word(c);
        }
        k += 1;
    }
}
