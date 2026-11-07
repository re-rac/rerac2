//! The line sparks: one shared function (L14 `0x4288b8`, L16 `0x3c4b48`, L26 `0x3c5d08`; the decomp's
//! `func_L14_004272D8`) on each level's own data ([`Sparks`]): a glint (L26 `0x33b050` = L00 `0x33b5e0`,
//! crate::particles::glint) at one end of a line in the moby's frame (`0x31bb58(2)` picks the end), running to the
//! other over its life (`0x31bbf0(0, 1)` under the big share: the big kind; its life `0x31bbb0(lo, hi)`), its roll
//! `0x31bb58(0xff)`.
//!
//! Users: crate::moby::classes::level26::node (0x0a79, also L16's copy, none placed there) and
//! crate::moby::classes::level14::guard (0x1100).

use crate::hero::physics::{add, scale, sub, V3};
use crate::moby::{Ctx, MobyWorld};

/// A level's sparks data (its `$gp` block): the two kinds' sizes, colours and life ranges, the big kind's share, and
/// the two ends of the line (in the moby's frame).
pub struct Sparks {
    pub size: [f32; 2],
    pub rgba: [u32; 2],
    pub life: [(i32, i32); 2],
    pub big: f32,
    pub ends: [V3; 2],
}

/// L14 `$gp+0x2790..0x27d0` (`0x1b1780`).
pub const L14: Sparks = Sparks {
    size: [0.25, 1.0],
    rgba: [0x6080_6000, 0x6090_4000],
    life: [(0x2d, 0x3c), (0x46, 0x5a)],
    big: 0.2,
    ends: [[0.0, 0.0, 0.25], [0.0, 0.0, 1.75]],
};

/// L26 `$gp+0x1000..0x1040`.
pub const L26: Sparks = Sparks {
    size: [0.5, 2.0],
    rgba: [0x6000_6080, 0x6000_4090],
    life: [(0x2d, 0x3c), (0x46, 0x5a)],
    big: 0.2,
    ends: [[0.0, 0.0, -4.0], [0.0, 0.0, -1.5]],
};

/// One spark of `s` from moby `id` (module docs).
pub fn emit(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: &Sparks) {
    let m = &w.mobys[id];
    let (pos, rows, hero) = (m.pos, m.rows, ctx.hero.pos);
    let rng = &mut ctx.game.rng;
    let side = rng.randi(2) as usize;
    let kind = usize::from(rng.randf(0.0, 1.0) < s.big);
    let n = rng.rand_range(s.life[kind].0, s.life[kind].1);
    let through = |v: V3| -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) };
    let at = add(through(s.ends[side]), pos);
    let step = scale(through(sub(s.ends[(side + 1) & 1], s.ends[side])), 1.0 / n as f32);
    let rot = rng.randi(0xff) as u8;
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::glint(sys, s.size[kind], at, step, s.rgba[kind], (n & 0xff) as u16, rot, false, hero);
    }
}
