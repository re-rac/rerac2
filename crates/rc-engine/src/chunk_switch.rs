//! The level chunk switch (L02 `0x3082e0`, L01 `0x2f9bd8`): a level in chunks (Maktar's resort and arena) swaps its
//! world — the tfrags and the collision — for chunk k's, keeping its mobys. The game asks for it through
//! rc_game::game::GameState::switch_chunk; after the tick [`apply`] reads chunk k's world from the level file
//! (crate::level_wad::chunk_world), puts it in the level, rebuilds the tfrag draw (crate::tfrag_render, its LOD state
//! with it) and, through [`ChunkGeneration`], the world lights' lists.
//!
//! The switched-in tfrags have no occlusion mapping (the switch resets the tfrags' lists, `0x372c80`): all always
//! visible. Not ported: the chunk's sound bank (bank k + 1, `0x334fe0`; crate::level_wad::chunk_sounds loads chunk
//! 0's only) and its ambient (`0x335630`); the hiding of what lies in the other chunk (`0x3081c8`, also at the level
//! load: the ties' `+0x1e` bit 4, the shrubs' `+0x16` bit 0 and the mobys' `+0x34` bit 0x80 set unless the variant
//! planes `0x308138` put them in chunk k); the chunk tfrags' occlusion words (`0x3080a0`: `+0x3a` through the table
//! `0x1caa94` the occlusion load `0x305380` fills from all 1259 tfrag mappings on Maktar); the other resets
//! (`0x368320`, `0x335998`, `0x383fd8`, `0x373960`).

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::storage::ShaderBuffer;
use crate::tfrag_render::TfragMaterial;

/// +1 per chunk switch: what was built from the level's tfrags rebuilds when it changes.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkGeneration(pub u32);

pub struct ChunkSwitchPlugin;

impl Plugin for ChunkSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGeneration>().add_systems(Update, apply);
    }
}

/// The switch asked for this tick (module docs).
#[allow(clippy::too_many_arguments)]
fn apply(
    mut commands: Commands,
    play: Option<ResMut<crate::play::Play>>,
    mut level: ResMut<crate::Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<TfragMaterial>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    drawn: Query<Entity, (With<MeshMaterial3d<TfragMaterial>>, Without<RenderLayers>)>,
    mut generation: ResMut<ChunkGeneration>,
) {
    let Some(mut p) = play else { return };
    let Some(k) = p.game.chunk_request.take() else { return };
    let index = crate::level_load::level_index();
    let world = usize::try_from(k).ok().map(|k| crate::level_wad::chunk_world(&crate::level_load::extracted_root(), index, k));
    let (tfrags, collision) = match world {
        Some(Ok(Some(w))) => w,
        Some(Ok(None)) | None => { eprintln!("chunk switch: level {index} has no chunk {k}"); return; }
        Some(Err(e)) => { eprintln!("chunk switch: level {index} chunk {k} not read ({e:#})"); return; }
    };
    let lv = &mut level.0;
    lv.occlusion.objects.tfrag = vec![rc_formats::occlusion::OcclBits::ALWAYS; tfrags.len()];
    lv.tfrags = tfrags;
    lv.collision = Some(collision);
    for e in &drawn { commands.entity(e).despawn(); }
    let s = crate::tfrag_render::spawn_tfrags(&mut commands, lv, &mut meshes, &mut images, &mut materials, &mut buffers);
    generation.0 += 1;
    println!("chunk switch: level {index} chunk {k}: {} tfrags, {} LOD-0 triangles", s.tfrags, s.triangles);
}
