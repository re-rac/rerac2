//! The surface under the feet (L00 `0x2aca80`, the update's step after the move): the flags of the surface id the
//! ground probe left (`0x18b2b0`), and the states they force — the lava hop 0x38 (surface 1; with no health the
//! deaths), the slide 0x30 (4), the sinking 0x7c (0xd, below the liquid's surface `0x18a150`), the quicksand 0x65 /
//! 0x78 (3, below `0x18a154`), the pit 0x78 / 0x74 (0xb).
//!
//! Surface ids: 0 / 0xe water (0xe: a puddle 0.2 deep), 1 lava, 2 a magnetic walkway (with the Gravity Boots
//! `0x1a7b13`: `0x18a168` = 1), 3 quicksand, 4 a slide, 7 ice, 8 / 0xc mud (`0x18b2ba`), 0xb a pit, 0xd a deep
//! liquid.
//!
//! The liquid heights `0x18a150` / `0x18a154` are the ground probe's (`0x2b4a08`, crate::hero::physics; the water's
//! height query `0x31d7a0` in crate::hero::water's docs). Nothing is done with the hero's updates off (`0x18c0dd`).
//!
//! On the lava, Clank's body (`0x18c0d4` = 1) dies with 0x7a; Ratchet with no health left with
//! crate::hero::damage's `lava_death` (the electric 0x7d in Maktar's arena, else 0x79, or 0x39 for a class without
//! its sequence).
//!
//! **Per level.** Each level overlay carries its own copy of the step, and ten of them drop the surfaces that level
//! has no faces of ([`kinds`], read from each overlay's copy: the flags `0x18b2b2..0x18b2bc` it sets). The
//! port's step is the full copy (L00's, shared by 17 levels) with each level's set: a surface a level's copy does not
//! know does nothing there. The one place that matters on the disc: Grelbin (19) has 1275 ice faces (surface 7, flat
//! at 295.1) its copy never flags, so they are not slippery.
//!
//! **The hazard surfaces** (the collision faces' surface id; every level's, `work/scratch/water_inventory.tsv` and the
//! survey in the docs of [`kinds`]): the lava 1 (Maktar's moat, Endako, Dobbo, Boldan (13), the Jamming Array (26):
//! the lava hop 0x38 and its damage, the deaths with no health); the pit 0xb, a death liquid Ratchet sinks into
//! (Tabora's lava streams at z 42..67, Boldan's, the Aranos prison's (14) and Snivelak's (16) at 68.6: no ground under
//! it (crate::hero::physics's probe), the sinking death 0x78, on level 0xd the fall 0x74); quicksand 3 (Oozla: the
//! sinking 0x65, 0x78 with no health); the deep liquid 0xd (Siberius (7): 0x7c). Maktar arena's electric floor is its
//! lava with the arena on (0x7d). Besides these the step has no hazard: the fall out of a level is the death height
//! (`0x1b1a68`, GameState::death_z), the other hurts are mobys' hits (crate::hero::damage's `hit_intake`). The first
//! water to swim in is level 6's.

use super::physics::*;
use super::Hero;
use crate::hero::anim::AnimCtl;

/// The flags (`0x18b2b0..0x18b2bd`, cleared every tick) and the liquid heights.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceBlock {
    /// `0x18b2b4` water, `0x18b2b2` ice, `0x18b2b5` lava, `0x18b2b6` quicksand, `0x18b2bb` a pit, `0x18b2b3` a slide,
    /// `0x18b2bc` a deep liquid, `0x18b2ba` mud, `0x18b2b7` magnetic.
    pub water: bool,
    pub ice: bool,
    pub hot: bool,
    pub sand: bool,
    pub pit: bool,
    pub slide: bool,
    pub deep: bool,
    pub mud: bool,
    pub magnet: bool,
    /// `0x18a168`: on a magnetic walkway with the Gravity Boots.
    pub boots: u8,
    /// `0x18c0d9`: wading (water 0.25..0.85 deep).
    pub wading: bool,
    /// `0x18a150` / `0x18a154`: the liquid's and the quicksand's surface (−∞: none); `0x18c2e4`: the water's depth.
    pub water_z: f32,
    pub sand_z: f32,
    pub depth: f32,
}

impl Default for SurfaceBlock {
    fn default() -> Self {
        SurfaceBlock {
            water: false, ice: false, hot: false, sand: false, pit: false, slide: false, deep: false, mud: false, magnet: false,
            boots: 0, wading: false, water_z: f32::NEG_INFINITY, sand_z: f32::NEG_INFINITY, depth: 0.0,
        }
    }
}

/// The surfaces a level's copy of `0x2aca80` reads ([`kinds`]): a bit per surface id, and whether Clank's body has
/// its own lava death 0x7a there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceKinds {
    pub ids: u32,
    pub clank_lava: bool,
}

impl SurfaceKinds {
    pub fn knows(self, id: i32) -> bool { (0..32).contains(&id) && self.ids & 1 << id != 0 }
}

const fn bits(ids: &[i32]) -> u32 {
    let mut m = 0;
    let mut i = 0;
    while i < ids.len() {
        m |= 1 << ids[i];
        i += 1;
    }
    m
}
/// The full copy (L00 `0x2aca80`): water 0 / 0xe, lava 1, magnetic 2, quicksand 3, slide 4, ice 7, mud 8 / 0xc, pit
/// 0xb, deep 0xd.
const ALL: u32 = bits(&[0, 0xe, 1, 2, 3, 4, 7, 8, 0xc, 0xb, 0xd]);

/// Level `level`'s copy of the surface step (its address; the flags it sets). The full copy is L00's (levels 0, 2, 5,
/// 6, 10, 12..15, 17, 20..26, 30). The others (surfaces the level has faces of in brackets, from its collision):
/// * 1 `0x2adf40`: slide, quicksand, magnetic, mud, pit (2, 3, 4, 9, 0xa, 0xc);
/// * 3 `0x2b2910`: lava, magnetic, mud, Clank's 0x7a (1, 9, 0xa, 0xc);
/// * 4 `0x2d3a60`: ice, water, magnetic, mud, deep (9, 0xa, 0xc);
/// * 7 `0x2ac0f0`: slide, quicksand, magnetic, mud, pit, deep (9, 0xc, 0xd);
/// * 8 `0x2bea60`: ice, water, quicksand, magnetic, mud, pit, deep — no lava, no slide (0xb, 0xc);
/// * 9 `0x2afd88`: water, lava, quicksand, magnetic, mud, pit (0, 1, 6, 8, 0xa, 0xc);
/// * 11 `0x2c5348`: lava, quicksand, magnetic, mud, pit, Clank's 0x7a (9, 0xa, 0xc);
/// * 16 `0x2ad398`: lava, quicksand, magnetic, mud, pit (2, 9, 0xa, 0xb, 0xc);
/// * 18 `0x2ca3c8`: ice, water, magnetic, mud, no states (9, 0xa, 0xc);
/// * 19 `0x2b01c0`: water, lava, magnetic, mud, deep — no ice (7, 9, 0xc).
pub fn kinds(level: i32) -> SurfaceKinds {
    let (ids, clank_lava) = match level {
        1 => (bits(&[4, 3, 2, 8, 0xc, 0xb]), false),
        3 => (bits(&[1, 2, 8, 0xc]), true),
        4 => (bits(&[7, 0, 0xe, 2, 8, 0xc, 0xd]), false),
        7 => (bits(&[4, 3, 2, 8, 0xc, 0xb, 0xd]), false),
        8 => (bits(&[7, 0, 0xe, 3, 2, 8, 0xc, 0xb, 0xd]), false),
        9 => (bits(&[0, 0xe, 1, 3, 2, 8, 0xc, 0xb]), false),
        11 => (bits(&[1, 3, 2, 8, 0xc, 0xb]), true),
        16 => (bits(&[1, 3, 2, 8, 0xc, 0xb]), false),
        18 => (bits(&[7, 0, 0xe, 2, 8, 0xc]), false),
        19 => (bits(&[0, 0xe, 1, 2, 8, 0xc, 0xd]), false),
        _ => (ALL, true),
    };
    SurfaceKinds { ids, clank_lava }
}

impl Hero {
    /// `0x2aca80` (each level's copy: [`kinds`]). Returns whether it set a state.
    pub fn surface_check(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> bool {
        let (water_z, sand_z) = (self.surf.water_z, self.surf.sand_z);
        self.surf = SurfaceBlock { water_z, sand_z, ..Default::default() };
        let id = self.surface;
        if self.no_update { return false; }
        let on = self.grounded_ticks != 0 || (self.state == 0x7b && self.height < 0.4);
        if id == -1 { return false; }
        let kinds = kinds(env.level);
        let s = &mut self.surf;
        let id = if kinds.knows(id) { id } else { -1 };
        match id {
            2 => {
                // The flag only near the floor (`0x18b2b7` is set in the condition's chain).
                if self.land_lag == 0 && (self.f16c == 0 || self.height < 2.5) {
                    s.magnet = true;
                    if self.mode == 0 && env.boots { s.boots = 1; }
                }
            }
            0xe => {
                s.water = true;
                s.water_z = self.ground_z + 0.2;
                s.depth = 0.2;
            }
            0 => {
                s.depth = s.water_z - self.ground_z;
                if s.depth < 0.85 && 0.25 < s.depth { s.wading = true; }
                s.water = true;
            }
            3 => s.sand = true,
            0xb => s.pit = true,
            4 => s.slide = true,
            7 => s.ice = true,
            0xd => s.deep = true,
            1 => s.hot = true,
            8 | 0xc => s.mud = true,
            _ => {}
        }
        let s = self.surf;
        let mut next = None;
        if s.slide && on && !matches!(self.group, 0x10 | 0x14 | 7) {
            next = Some(0x30);
        } else if s.hot && on && self.mode == 1 && self.state != 0x7a && kinds.clank_lava {
            // Clank's body: the lava death 0x7a.
            next = Some(0x7a);
        } else if s.hot && on && (self.state != 0x38 || self.jump.descending != 0) {
            next = Some(if self.hp == 0 { self.lava_death(env, anim) } else { 0x38 });
        } else if s.deep && self.state != 0x7c && (s.water_z - (self.pos[2] + 0.25)).abs() < 1.0 && 0.0 < s.water_z - self.pos[2] && (self.disp[2] < 0.0 || self.state == 0x7b) {
            next = Some(0x7c);
        } else if s.sand && !matches!(self.state, 0x65 | 0x78) && (s.sand_z - (self.pos[2] + 0.25)).abs() < 1.0 && 0.0 < s.sand_z - self.pos[2]
            && !(self.state == 0x66 && self.jump.descending == 0) && (self.disp[2] < 0.0 || self.state == 0x7b)
        {
            next = Some(if self.hp != 0 { 0x65 } else { 0x78 });
        } else if s.pit && !matches!(self.state, 0x78 | 0x74) && (s.sand_z - (self.pos[2] + 0.25)).abs() < 1.0 && 0.0 < s.sand_z - self.pos[2] && (self.disp[2] < 0.0 || self.state == 0x7b) {
            // Level 0xd's pits are the fall out of the level.
            next = Some(if env.level == 0xd { 0x74 } else { 0x78 });
        }
        match next {
            Some(n) => self.set_state(anim, env, n, true),
            None => false,
        }
    }
}
