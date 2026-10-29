//! The casters' shadows in the engine (G-REN-001, rc_game::shadow_volumes): the frame's casters and their volumes
//! each tick, and a native screen-space shadow-volume pass. One general system: any class with a shadow block
//! (`rc_formats::moby_shadow`) whose update fills its slab casts, with no per-class code here. R&C1's pass
//! (`rerac:rc_engine::shadow_render`) on Going Commando's data.
//!
//! * **Tick** ([`collect`], at the end of the hero's part of the tick, crate::play): MobyProc's deferral (L00
//!   `0x318fd0`, the mode 0x400 branch) in the moby array's order: a live moby with mode 0x400 (not 0x800, not
//!   skipped by 0x81), drawn last frame (Ratchet: always), with a slab (`+0x74` > 0) and a class shadow block; its
//!   bounding sphere moved along its direction onto the slab's plane and grown by the slab's half thickness must pass
//!   the moby culls with the range `+0x6f` as the draw distance (crate::moby_lod); the size word by distance
//!   (`volume::shadow_size`); the 8,064-byte budget in list order (`volume::fitting`). Each caster is posed on its
//!   current skeleton pose (crate::mobys::Host::pose_joints, the pose it is drawn with) and turned into closed
//!   prisms (`rc_game::shadow_volumes::volume`), in Bevy axes, into [`ShadowVolumes`]; the casters' draws move after
//!   the pass (crate::moby_render `CasterTwin`: the created ones by crate::mobys `shadow_casters`, the placed ones
//!   by their class).
//! * **GPU** (render world, `Core3d` between the opaque and the transparent main passes, after crate::pre_shadow,
//!   [`shadow_pass`]): the prisms into an `R16Float` count target (cleared to 0, same size and samples as the view),
//!   depth-tested `GreaterEqual` against the scene depth (read only), additive `+1` front / `−1` back faces; then a
//!   full-screen triangle multiplies the colour target by [`SHADOW_FACTOR`] where the count is positive: the game's
//!   destination-alpha counter (`0x35a290` clears, `0x35a200` counts, TEST_1 0x51001) and its resolve (`0x35a438`:
//!   TEST_1 0x3d801, ALPHA_1 0x2000000064 = 0.75·Cd). Bevy's 3D depth format has no stencil, hence the count target.
//!
//! Not reproduced (hardware): the GS guard-band face drop, the destination-alpha counter's byte arithmetic. The
//! darkening is a result-level reproduction (hardware_fidelity_layers.md).

use crate::moby_lod::{self, ProcInput};
use crate::tfrag_render::game_to_bevy;
use bevy::core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy::core_pipeline::{Core3d, Core3dSystems, FullscreenShader};
use bevy::mesh::VertexBufferLayout;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_resource::binding_types::{texture_2d, texture_2d_multisampled, uniform_buffer};
use bevy::render::render_resource::{
    BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendComponent, BlendFactor, BlendOperation, BlendState,
    Buffer, BufferInitDescriptor, BufferUsages, CachedRenderPipelineId, ColorTargetState, ColorWrites, CompareFunction,
    DepthStencilState, Extent3d, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache, PrimitiveState,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, ShaderStages, StoreOp, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, VertexAttribute, VertexFormat, VertexState, VertexStepMode,
};
use bevy::render::renderer::{RenderContext, RenderDevice, ViewQuery};
use bevy::render::texture::{CachedTexture, TextureCache};
use bevy::render::view::{ExtractedView, ViewDepthTexture, ViewTarget};
use bevy::render::{Render, RenderApp, RenderStartup, RenderSystems};
use bevy::shader::{Shader, ShaderDefVal};
use rc_game::moby::MobyWorld;
use rc_game::shadow_volumes::{volume, MODE_CASTER};
use std::collections::HashSet;
use std::sync::Arc;

/// The darkening as one linear-light multiply on the sRGB target: the GS byte `Cd − ⌈Cd/4⌉` (0.75·Cd) is met
/// within ±2 on 244 of the 256 display bytes and −3 on 12 dark ones (12..40); 0.53 minimises that error
/// (`tests::factor_matches_the_gs_bytes`). Mirrored in `shadow_resolve.wgsl`.
#[cfg_attr(not(test), allow(dead_code))]
pub const SHADOW_FACTOR: f32 = 0.53;
/// The count target's format (blendable; counts the small integers exactly).
pub const COUNT_FORMAT: TextureFormat = TextureFormat::R16Float;

/// The tick's shadow volumes: triangles (3 points each) in Bevy world axes.
#[derive(Resource, Clone, Default, ExtractResource)]
pub struct ShadowVolumes {
    pub tris: Arc<Vec<[f32; 3]>>,
}

/// The render-world set of the shadow pass (crate::pre_shadow draws before it).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShadowPassSet;

/// The camera whose view gets the shadow pass (the world camera).
#[derive(Component, Clone, Copy, Default, ExtractComponent)]
pub struct ShadowCamera;

pub struct ShadowPlugin;

impl Plugin for ShadowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShadowVolumes>()
            .add_plugins((ExtractResourcePlugin::<ShadowVolumes>::default(), ExtractComponentPlugin::<ShadowCamera>::default()))
            .add_systems(Update, tag_camera)
            .add_systems(Last, sync);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app
            .init_resource::<ShadowGpu>()
            .add_systems(RenderStartup, init_pipelines)
            .add_systems(Render, prepare.in_set(RenderSystems::PrepareResources))
            .add_systems(Core3d, shadow_pass.after(main_opaque_pass_3d).before(main_transparent_pass_3d).in_set(Core3dSystems::MainPass).in_set(ShadowPassSet));
    }
}

/// The world camera, not yet tagged.
type Untagged = (With<crate::render_settings::WorldCamera>, With<Camera3d>, Without<ShadowCamera>);

fn tag_camera(mut commands: Commands, cams: Query<Entity, Untagged>) {
    for e in &cams { commands.entity(e).insert(ShadowCamera); }
}

/// The tick's volumes into the render world's copy (none without a level in play).
fn sync(play: Option<Res<crate::play::Play>>, mut out: ResMut<ShadowVolumes>) {
    let tris = play.as_ref().and_then(|p| p.mobys.as_ref()).map(|m| m.shadow_tris.clone()).unwrap_or_default();
    if !Arc::ptr_eq(&out.tris, &tris) { out.tris = tris; }
}

/// A caster that passed MobyProc's shadow tests this tick.
struct Deferred {
    id: usize,
    size: u16,
    bytes: usize,
}

/// The tick's casters and their volumes (module docs): the volumes in Bevy axes and the deferred mobys. `eye` /
/// `rows` / `tans`: the game camera (crate::moby_lod's view).
pub fn collect(world: &MobyWorld, host: &crate::mobys::Host, game: &rc_game::game::GameState, hero: Option<usize>, eye: Vec3, rows: &[Vec3; 3], tans: (f32, f32)) -> (Vec<[f32; 3]>, HashSet<usize>) {
    let mut list = Vec::new();
    for (id, m) in world.mobys.iter().enumerate() {
        if 0x80 <= m.state || m.mode & MODE_CASTER == 0 || m.mode & 0x800 != 0 || m.mode & 0x81 != 0 { continue; }
        let t = m.shadow_slab[1];
        if !(0.0 < t) || (m.drawn == 0 && Some(id) != hero) { continue; }
        let Some(block) = world.class(m.o_class).and_then(|c| c.shadow.as_ref()) else { continue };
        let (d, up) = (game.shadow.dir_of(m), m.rows[2]);
        let ud = up[0] * d[0] + up[1] * d[1] + up[2] * d[2];
        if ud == 0.0 { continue; }
        // The sphere onto the plane `up·x = up·pos + off` along the direction (integer units, ×1024).
        let plane = (up[0] * m.pos[0] + up[1] * m.pos[1] + up[2] * m.pos[2] + m.shadow_slab[0]) * 1024.0;
        let b = m.bsphere;
        let q = (plane - (up[0] * b[0] + up[1] * b[1] + up[2] * b[2])) / ud;
        let r = b[3] + t * 1024.0;
        let c = [b[0] + d[0] * q, b[1] + d[1] * q, b[2] + d[2] * q, r];
        let v = moby_lod::view_centre(c, eye, rows);
        let inp = ProcInput { position: m.pos, rows: [[0.0; 3]; 3], scale: m.scale, draw_distance: m.shadow_range as i32, lod_trans: 0xff, shine_distance: 0, alpha: 0x80 };
        if moby_lod::moby_proc_view(v, r, &inp, tans).is_err() { continue; }
        let size = volume::shadow_size(m.shadow_range, moby_lod::sphere_depth(v, r));
        list.push(Deferred { id, size, bytes: volume::caster_bytes(block) });
    }
    let n = volume::fitting(list.iter().map(|c| c.bytes));
    let deferred = list[..n].iter().map(|c| c.id).collect();
    let mut tris = Vec::new();
    for c in &list[..n] {
        let m = &world.mobys[c.id];
        let Some(block) = world.class(m.o_class).and_then(|k| k.shadow.as_ref()) else { continue };
        let top = block.prims.iter().flat_map(|p| p.joints()).max().unwrap_or(0) as usize + 1;
        let Some(joints) = host.pose_joints(m, top) else { continue };
        let pose = volume::CasterPose { joints: &joints, rows: m.rows, position: m.pos, scale: m.scale, size: c.size };
        let prims = volume::pose_prims(block, &pose);
        let up = m.rows[2];
        let plane = up[0] * m.pos[0] + up[1] * m.pos[1] + up[2] * m.pos[2] + m.shadow_slab[0];
        volume::caster_volume(&prims, game.shadow.dir_of(m), up, plane, m.shadow_slab[1], &mut tris);
    }
    for p in &mut tris { *p = game_to_bevy(*p).to_array(); }
    (tris, deferred)
}

// ---------------------------------------------------------------------------------------------------
// Render world

/// The frame's vertex buffer.
#[derive(Resource, Default)]
struct ShadowGpu {
    buffer: Option<Buffer>,
    vertices: u32,
}

#[derive(Resource)]
struct ShadowPipelines {
    view_layout: BindGroupLayoutDescriptor,
    count_layout: [BindGroupLayoutDescriptor; 2],
    volume_shader: Handle<Shader>,
    resolve_shader: Handle<Shader>,
    fullscreen: VertexState,
    ids: std::sync::Mutex<HashMap<(u8, u32, TextureFormat), CachedRenderPipelineId>>,
}

/// Per shadow view: the count target.
#[derive(Component)]
struct ShadowCount(CachedTexture);

fn init_pipelines(mut commands: Commands, assets: Res<AssetServer>, fullscreen: Res<FullscreenShader>) {
    commands.insert_resource(ShadowPipelines {
        view_layout: BindGroupLayoutDescriptor::new("shadow volume view", &BindGroupLayoutEntries::single(ShaderStages::VERTEX, uniform_buffer::<Mat4>(false))),
        count_layout: [
            BindGroupLayoutDescriptor::new("shadow count", &BindGroupLayoutEntries::single(ShaderStages::FRAGMENT, texture_2d(TextureSampleType::Float { filterable: false }))),
            BindGroupLayoutDescriptor::new(
                "shadow count (multisampled)",
                &BindGroupLayoutEntries::single(ShaderStages::FRAGMENT, texture_2d_multisampled(TextureSampleType::Float { filterable: false })),
            ),
        ],
        volume_shader: assets.load("shaders/shadow_volume.wgsl"),
        resolve_shader: assets.load("shaders/shadow_resolve.wgsl"),
        fullscreen: fullscreen.to_vertex_state(),
        ids: Default::default(),
    });
}

/// Pipeline kinds.
const VOLUME: u8 = 0;
const DARKEN: u8 = 1;

impl ShadowPipelines {
    fn id(&self, cache: &PipelineCache, kind: u8, samples: u32, format: TextureFormat) -> CachedRenderPipelineId {
        let mut ids = self.ids.lock().unwrap_or_else(|e| e.into_inner());
        *ids.entry((kind, samples, format)).or_insert_with(|| {
            let multisample = MultisampleState { count: samples, ..default() };
            if kind == VOLUME {
                let add = BlendComponent { src_factor: BlendFactor::One, dst_factor: BlendFactor::One, operation: BlendOperation::Add };
                cache.queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("shadow volumes".into()),
                    layout: vec![self.view_layout.clone()],
                    vertex: VertexState {
                        shader: self.volume_shader.clone(),
                        shader_defs: vec![],
                        entry_point: Some("vertex".into()),
                        buffers: vec![VertexBufferLayout {
                            array_stride: 12,
                            step_mode: VertexStepMode::Vertex,
                            attributes: vec![VertexAttribute { format: VertexFormat::Float32x3, offset: 0, shader_location: 0 }],
                        }],
                    },
                    fragment: Some(FragmentState {
                        shader: self.volume_shader.clone(),
                        shader_defs: vec![],
                        entry_point: Some("fragment".into()),
                        targets: vec![Some(ColorTargetState { format: COUNT_FORMAT, blend: Some(BlendState { color: add, alpha: add }), write_mask: ColorWrites::RED })],
                    }),
                    // Both faces (front +1, back −1); the scene depth read only, reverse Z (GS GEQUAL).
                    primitive: PrimitiveState { cull_mode: None, ..default() },
                    depth_stencil: Some(DepthStencilState {
                        format: bevy::core_pipeline::core_3d::CORE_3D_DEPTH_FORMAT,
                        depth_write_enabled: Some(false),
                        depth_compare: Some(CompareFunction::GreaterEqual),
                        stencil: default(),
                        bias: default(),
                    }),
                    multisample,
                    ..default()
                })
            } else {
                let ms = samples > 1;
                let shader_defs: Vec<ShaderDefVal> = if ms { vec!["MULTISAMPLED".into()] } else { vec![] };
                // Darken: dst·K (Zero / Src), alpha kept.
                let blend = Some(BlendState {
                    color: BlendComponent { src_factor: BlendFactor::Zero, dst_factor: BlendFactor::Src, operation: BlendOperation::Add },
                    alpha: BlendComponent { src_factor: BlendFactor::Zero, dst_factor: BlendFactor::One, operation: BlendOperation::Add },
                });
                cache.queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("shadow resolve".into()),
                    layout: vec![self.count_layout[ms as usize].clone()],
                    vertex: self.fullscreen.clone(),
                    fragment: Some(FragmentState {
                        shader: self.resolve_shader.clone(),
                        shader_defs,
                        entry_point: Some("darken".into()),
                        targets: vec![Some(ColorTargetState { format, blend, write_mask: ColorWrites::ALL })],
                    }),
                    multisample,
                    ..default()
                })
            }
        })
    }
}

/// Uploads the frame's triangles, queues the pipelines and gives every shadow view its count target.
#[allow(clippy::too_many_arguments)]
fn prepare(
    mut commands: Commands,
    volumes: Option<Res<ShadowVolumes>>,
    mut gpu: ResMut<ShadowGpu>,
    device: Res<RenderDevice>,
    cache: Res<PipelineCache>,
    pipes: Option<Res<ShadowPipelines>>,
    mut textures: ResMut<TextureCache>,
    views: Query<(Entity, &ExtractedCamera, &ViewTarget), With<ShadowCamera>>,
) {
    let Some(pipes) = pipes else { return };
    let tris = volumes.as_ref().map(|v| v.tris.clone()).unwrap_or_default();
    gpu.vertices = tris.len() as u32;
    gpu.buffer = (!tris.is_empty()).then(|| {
        let bytes: Vec<u8> = tris.iter().flat_map(|p| p.iter().flat_map(|x| x.to_le_bytes())).collect();
        device.create_buffer_with_data(&BufferInitDescriptor { label: Some("shadow volumes"), contents: &bytes, usage: BufferUsages::VERTEX })
    });
    for (e, cam, target) in &views {
        let samples = target.sampled_main_texture().map_or(1, |t| t.sample_count());
        let format = target.main_texture_format();
        // Queued every frame (cheap once cached), so the first shadow never waits for a compile of both.
        for kind in [VOLUME, DARKEN] { pipes.id(&cache, kind, samples, format); }
        let Some(size) = cam.physical_target_size else { continue };
        let count = textures.get(
            &device,
            TextureDescriptor {
                label: Some("shadow count"),
                size: Extent3d { width: size.x.max(1), height: size.y.max(1), depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: samples,
                dimension: TextureDimension::D2,
                format: COUNT_FORMAT,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
        );
        commands.entity(e).insert(ShadowCount(count));
    }
}

/// The shadow pass of a shadow view (module docs): volumes into the count target, then the resolve.
fn shadow_pass(
    view: ViewQuery<(&ExtractedCamera, &ExtractedView, &ViewTarget, &ViewDepthTexture, &ShadowCount), With<ShadowCamera>>,
    gpu: Res<ShadowGpu>,
    pipes: Option<Res<ShadowPipelines>>,
    cache: Res<PipelineCache>,
    mut ctx: RenderContext,
) {
    let (Some(buffer), Some(pipes)) = (gpu.buffer.as_ref(), pipes) else { return };
    let (camera, ev, target, depth, count) = view.into_inner();
    let samples = target.sampled_main_texture().map_or(1, |t| t.sample_count());
    let format = target.main_texture_format();
    let (Some(vol_pipe), Some(res_pipe)) = (
        cache.get_render_pipeline(pipes.id(&cache, VOLUME, samples, format)),
        cache.get_render_pipeline(pipes.id(&cache, DARKEN, samples, format)),
    ) else {
        return;
    };
    let clip_from_world = ev.clip_from_world.unwrap_or_else(|| ev.clip_from_view * ev.world_from_view.to_matrix().inverse());
    let device = ctx.render_device().clone();
    let bytes: Vec<u8> = clip_from_world.to_cols_array().iter().flat_map(|x| x.to_le_bytes()).collect();
    let uniform = device.create_buffer_with_data(&BufferInitDescriptor { label: Some("shadow view"), contents: &bytes, usage: BufferUsages::UNIFORM });
    let view_group = device.create_bind_group("shadow view", &cache.get_bind_group_layout(&pipes.view_layout), &BindGroupEntries::single(uniform.as_entire_binding()));
    let count_group = device.create_bind_group("shadow count", &cache.get_bind_group_layout(&pipes.count_layout[(samples > 1) as usize]), &BindGroupEntries::single(&count.0.default_view));
    let viewport = camera.viewport.clone();
    {
        let mut pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("shadow volumes"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &count.0.default_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations { load: LoadOp::Clear(Default::default()), store: StoreOp::Store },
            })],
            depth_stencil_attachment: Some(depth.get_attachment(StoreOp::Store)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        if let Some(v) = &viewport { pass.set_camera_viewport(v); }
        pass.set_render_pipeline(vol_pipe);
        pass.set_bind_group(0, &view_group, &[]);
        pass.set_vertex_buffer(0, buffer.slice(..));
        pass.draw(0..gpu.vertices, 0..1);
    }
    let mut pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some("shadow resolve"),
        color_attachments: &[Some(target.get_color_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    if let Some(v) = &viewport { pass.set_camera_viewport(v); }
    pass.set_render_pipeline(res_pipe);
    pass.set_bind_group(0, &count_group, &[]);
    pass.draw(0..3, 0..1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s2l(c: f32) -> f32 { if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) } }
    fn l2s(c: f32) -> f32 { if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 } }

    /// The resolve's linear multiply against the GS byte `Cd − ⌈Cd/4⌉` (ALPHA_1 0x2000000064: `(0 − Cd)·0x20 >> 7
    /// + Cd`, arithmetic shift), on every display byte.
    #[test]
    fn factor_matches_the_gs_bytes() {
        let (mut worst, mut over2) = (0i32, 0);
        for d in 0..=255i32 {
            let gs = d + ((-d * 0x20) >> 7);
            let port = (l2s(SHADOW_FACTOR * s2l(d as f32 / 255.0)) * 255.0).round() as i32;
            let e = port - gs;
            worst = worst.max(e.abs());
            if e.abs() > 2 {
                over2 += 1;
                assert!((12..=40).contains(&d) && e == -3, "byte {d}: {port} vs {gs}");
            }
        }
        assert_eq!((worst, over2), (3, 12));
    }

    #[test]
    fn shader_uses_the_same_factor() {
        let wgsl = include_str!("../assets/shaders/shadow_resolve.wgsl");
        assert!(wgsl.contains(&format!("const K: f32 = {SHADOW_FACTOR};")), "shadow_resolve.wgsl K");
    }
}
