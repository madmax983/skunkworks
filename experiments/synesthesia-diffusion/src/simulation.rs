use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::{RenderAssets, RenderAssetUsages},
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel, RenderGraphApp},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
    core_pipeline::core_2d::graph::{Core2d, Node2d},
};
use crate::audio::AudioSpectrum;
use bytemuck::{Pod, Zeroable};

// --- Constants ---
const WORKGROUP_SIZE: u32 = 8;
const TEXTURE_SIZE: (u32, u32) = (1024, 1024);

// --- Resources ---

#[derive(Resource, Clone, ExtractResource)]
pub struct DiffusionConfig {
    pub feed: f32,
    pub kill: f32,
    pub du: f32,
    pub dv: f32,
    pub dt: f32,
    pub mouse_pos: Vec2,
    pub mouse_pressed: f32,
}

impl Default for DiffusionConfig {
    fn default() -> Self {
        Self {
            feed: 0.055,
            kill: 0.062,
            du: 1.0,
            dv: 0.5,
            dt: 1.0,
            mouse_pos: Vec2::ZERO,
            mouse_pressed: 0.0,
        }
    }
}

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct SimulationImage(pub Handle<Image>);

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct SimulationImageBind(pub Handle<Image>);

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct DiffusionShader(pub Handle<Shader>);

#[derive(Resource)]
struct DiffusionBindGroup(BindGroup);

#[derive(Resource)]
struct DiffusionPipeline {
    pipeline_id: CachedComputePipelineId,
    bind_group_layout: BindGroupLayout,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct DiffusionLabel;

// --- Plugin ---

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<DiffusionConfig>::default())
           .add_plugins(ExtractResourcePlugin::<SimulationImage>::default())
           .add_plugins(ExtractResourcePlugin::<SimulationImageBind>::default())
           .add_plugins(ExtractResourcePlugin::<DiffusionShader>::default())
           .init_resource::<DiffusionConfig>()
           .add_systems(Startup, setup_simulation)
           .add_systems(Update, (update_parameters_from_audio, handle_interaction, swap_textures));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(Render, (
                queue_pipeline.in_set(RenderSet::Prepare),
                prepare_bind_group.in_set(RenderSet::Prepare).after(queue_pipeline)
            ))
            .add_render_graph_node::<DiffusionNode>(Core2d, DiffusionLabel)
            .add_render_graph_edge(
                Core2d,
                DiffusionLabel,
                Node2d::StartMainPass,
            );
    }
}

// --- Systems ---

fn setup_simulation(mut commands: Commands, mut images: ResMut<Assets<Image>>, asset_server: Res<AssetServer>) {
    // Load shader
    let shader = asset_server.load("shaders/diffusion.wgsl");
    commands.insert_resource(DiffusionShader(shader));

    // Create textures
    let mut image_a = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE.0,
            height: TEXTURE_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image_a.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let mut image_b = image_a.clone();

    // Seed initial state
    let center_x = TEXTURE_SIZE.0 / 2;
    let center_y = TEXTURE_SIZE.1 / 2;
    let radius = 20;

    let data_len = (TEXTURE_SIZE.0 * TEXTURE_SIZE.1 * 16) as usize;
    let mut data = vec![0u8; data_len];
    for y in 0..TEXTURE_SIZE.1 {
        for x in 0..TEXTURE_SIZE.0 {
            let idx = ((y * TEXTURE_SIZE.0 + x) * 16) as usize;
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;

            let u: f32 = 1.0;
            let mut v: f32 = 0.0;

            if dx*dx + dy*dy < radius*radius {
                v = 1.0;
            }

            if rand::random::<f32>() > 0.999 {
                v = 1.0;
            }

            let u_bytes = u.to_ne_bytes();
            let v_bytes = v.to_ne_bytes();

            data[idx..idx+4].copy_from_slice(&u_bytes);
            data[idx+4..idx+8].copy_from_slice(&v_bytes);
            data[idx+12..idx+16].copy_from_slice(&1.0f32.to_ne_bytes());
        }
    }

    image_a.data = data.clone();
    image_b.data = data;

    let handle_a = images.add(image_a);
    let handle_b = images.add(image_b);

    commands.insert_resource(SimulationImage(handle_a));
    commands.insert_resource(SimulationImageBind(handle_b));
}

fn update_parameters_from_audio(
    mut config: ResMut<DiffusionConfig>,
    spectrum: Res<AudioSpectrum>,
) {
    if !spectrum.data.is_empty() {
        let low = spectrum.data.get(2).unwrap_or(&0.0);
        let mid = spectrum.data.get(10).unwrap_or(&0.0);

        let target_feed = 0.055 + (low * 0.02);
        let target_kill = 0.062 + (mid * 0.02);

        config.feed = config.feed + (target_feed - config.feed) * 0.1;
        config.kill = config.kill + (target_kill - config.kill) * 0.1;

        config.feed = config.feed.clamp(0.01, 0.1);
        config.kill = config.kill.clamp(0.01, 0.1);
    }
}

fn handle_interaction(
    mut config: ResMut<DiffusionConfig>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Some(window) = windows.iter().next() {
        if let Some(pos) = window.cursor_position() {
            // Map cursor pos (window space) to texture space
            // Assuming texture fills window for now, or 1:1 mapping if resized
            // Our texture is 1024x1024. Window might be different.
            // Let's assume window is 1024x1024 or scale it.
            let size = Vec2::new(window.width(), window.height());
            let uv = pos / size;
            let tex_pos = uv * Vec2::new(TEXTURE_SIZE.0 as f32, TEXTURE_SIZE.1 as f32);
            // Flip Y because Bevy UI vs Texture coords?
            // Bevy window Y is down. Texture Y is down (usually) or up depending on API.
            // WGSL textureLoad: (0,0) is usually top-left or bottom-left.
            // wgpu: top-left origin for textures usually.

            config.mouse_pos = tex_pos;
        }
    }

    if buttons.pressed(MouseButton::Left) {
        config.mouse_pressed = 1.0;
    } else {
        config.mouse_pressed = 0.0;
    }

    // Toggle Ghost Mode? Handled in Audio? No, handle 'G' here
    // Actually Audio plugin handles logic, but input is here.
    // Audio plugin logic is automatic.
    // If I want manual toggle, I need a flag in AudioConfig or something.
    // Skipping for now as Ghost Mode is automatic.
}

fn swap_textures(
    mut image_a: ResMut<SimulationImage>,
    mut image_b: ResMut<SimulationImageBind>,
) {
    std::mem::swap(&mut image_a.0, &mut image_b.0);
}

// --- Render World ---

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
struct ParamsUniform {
    feed: f32,
    kill: f32,
    du: f32,
    dv: f32,
    dt: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_pressed: f32,
}

fn queue_pipeline(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    render_device: Res<RenderDevice>,
    shader_handle: Res<DiffusionShader>,
    diffusion_pipeline: Option<ResMut<DiffusionPipeline>>,
) {
    if diffusion_pipeline.is_some() {
        return;
    }

    let bind_group_layout = render_device.create_bind_group_layout(
        "diffusion_bind_group_layout",
        &[
            // Input Texture
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Output Texture
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::WriteOnly,
                    format: TextureFormat::Rgba32Float,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            // Uniforms
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    );

    let pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("diffusion_pipeline".into()),
        layout: vec![bind_group_layout.clone()],
        shader: shader_handle.0.clone(),
        shader_defs: vec![],
        entry_point: "update".into(),
        push_constant_ranges: vec![],
    });

    commands.insert_resource(DiffusionPipeline {
        pipeline_id,
        bind_group_layout,
    });
}

fn prepare_bind_group(
    mut commands: Commands,
    diffusion_pipeline: Option<Res<DiffusionPipeline>>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    image_a: Res<SimulationImage>,
    image_b: Res<SimulationImageBind>,
    config: Res<DiffusionConfig>,
    render_device: Res<RenderDevice>,
) {
    let diffusion_pipeline = if let Some(p) = diffusion_pipeline { p } else { return };
    let gpu_image_a = if let Some(i) = gpu_images.get(&image_a.0) { i } else { return };
    let gpu_image_b = if let Some(i) = gpu_images.get(&image_b.0) { i } else { return };

    let params = ParamsUniform {
        feed: config.feed,
        kill: config.kill,
        du: config.du,
        dv: config.dv,
        dt: config.dt,
        mouse_x: config.mouse_pos.x,
        mouse_y: config.mouse_pos.y,
        mouse_pressed: config.mouse_pressed,
    };

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("diffusion_params_buffer"),
        contents: bytemuck::bytes_of(&params),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = render_device.create_bind_group(Some("diffusion_bind_group"), &diffusion_pipeline.bind_group_layout, &[
        BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&gpu_image_a.texture_view),
        },
        BindGroupEntry {
            binding: 1,
            resource: BindingResource::TextureView(&gpu_image_b.texture_view),
        },
        BindGroupEntry {
            binding: 2,
            resource: buffer.as_entire_binding(),
        },
    ]);

    commands.insert_resource(DiffusionBindGroup(bind_group));
}

#[derive(Default)]
struct DiffusionNode;

impl Node for DiffusionNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();

        if let Some(diffusion_pipeline) = world.get_resource::<DiffusionPipeline>() {
            if let Some(bind_group) = world.get_resource::<DiffusionBindGroup>() {
                 if let Some(pipeline) = pipeline_cache.get_compute_pipeline(diffusion_pipeline.pipeline_id) {
                    let mut pass = render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor::default());

                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, &bind_group.0, &[]);

                    let workgroup_count = (TEXTURE_SIZE.0 / WORKGROUP_SIZE, TEXTURE_SIZE.1 / WORKGROUP_SIZE, 1);
                    pass.dispatch_workgroups(workgroup_count.0, workgroup_count.1, workgroup_count.2);
                }
            }
        }

        Ok(())
    }
}
