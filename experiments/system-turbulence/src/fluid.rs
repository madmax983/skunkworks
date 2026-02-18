use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraphApp, RenderLabel},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        Render, RenderSet, RenderApp,
    },
    core_pipeline::core_2d::graph as core_2d_graph,
};
use std::borrow::Cow;
use bytemuck::{Pod, Zeroable};
// use wgpu::util::DeviceExt; // Removed

// --- Resources (Main World) ---

#[derive(Resource, Clone, ExtractResource)]
pub struct FluidParams {
    pub dt: f32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub decay: f32,
    pub viscosity: f32,
    pub padding: [f32; 3],
}

impl Default for FluidParams {
    fn default() -> Self {
        Self {
            dt: 0.1,
            grid_width: 256,
            grid_height: 256,
            decay: 0.99,
            viscosity: 0.0,
            padding: [0.0; 3],
        }
    }
}

#[derive(Resource, Clone, ExtractResource)]
pub struct FluidImage(pub Handle<Image>);

#[derive(Resource, Clone, ExtractResource)]
pub struct ForceImage(pub Handle<Image>);

// --- Plugin ---

pub struct FluidPlugin;

impl Plugin for FluidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FluidParams>()
            .add_plugins(ExtractResourcePlugin::<FluidParams>::default())
            .add_plugins(ExtractResourcePlugin::<FluidImage>::default())
            .add_plugins(ExtractResourcePlugin::<ForceImage>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(Render, prepare_fluid_pipeline.in_set(RenderSet::Prepare))
            .add_systems(Render, prepare_bind_group.in_set(RenderSet::PrepareBindGroups))
            .add_render_graph_node::<FluidNode>(core_2d_graph::Core2d, FluidLabel)
            .add_render_graph_edge(
                core_2d_graph::Core2d,
                FluidLabel,
                core_2d_graph::Node2d::MainPass,
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct FluidLabel;

// --- Render World Resources ---

#[derive(Resource)]
struct FluidPipeline {
    bind_group_layout: BindGroupLayout,
    pipeline: ComputePipeline,
}

#[derive(Resource)]
struct FluidBindGroup(BindGroup);

#[derive(Resource)]
struct FluidAuxTextures {
    density_out: Texture,
    velocity_in: Texture,
    velocity_out: Texture,
}

// --- Systems ---

fn prepare_fluid_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Option<Res<FluidPipeline>>,
) {
    if pipeline.is_some() { return; }

    let bind_group_layout = render_device.create_bind_group_layout(
        Some("fluid_bind_group_layout"),
        &[
            // 0: density_in (texture)
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: false }, view_dimension: TextureViewDimension::D2, multisampled: false },
                count: None,
            },
            // 1: density_out (storage)
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture { access: StorageTextureAccess::WriteOnly, format: TextureFormat::Rgba32Float, view_dimension: TextureViewDimension::D2 },
                count: None,
            },
            // 2: velocity_in (texture)
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: false }, view_dimension: TextureViewDimension::D2, multisampled: false },
                count: None,
            },
            // 3: velocity_out (storage)
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture { access: StorageTextureAccess::WriteOnly, format: TextureFormat::Rgba32Float, view_dimension: TextureViewDimension::D2 },
                count: None,
            },
            // 4: force_tex (texture)
            BindGroupLayoutEntry {
                binding: 4,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture { sample_type: TextureSampleType::Float { filterable: false }, view_dimension: TextureViewDimension::D2, multisampled: false },
                count: None,
            },
            // 5: params (uniform)
            BindGroupLayoutEntry {
                binding: 5,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            },
        ]
    );

    let shader_source = include_str!("../assets/shaders/fluid.wgsl");
    let shader = render_device.create_shader_module(ShaderModuleDescriptor {
        label: Some("fluid_shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
    });

    let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = render_device.create_compute_pipeline(&RawComputePipelineDescriptor {
        label: Some("fluid_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "update",
    });

    commands.insert_resource(FluidPipeline {
        bind_group_layout,
        pipeline,
    });
}

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<FluidPipeline>,
    render_device: Res<RenderDevice>,
    fluid_image: Res<FluidImage>,
    force_image: Res<ForceImage>,
    gpu_images: Res<RenderAssets<Image>>,
    params: Res<FluidParams>,
    aux_opt: Option<Res<FluidAuxTextures>>,
) {
    let density_image = gpu_images.get(&fluid_image.0);
    let force_image_gpu = gpu_images.get(&force_image.0);

    if let (Some(den_img), Some(force_img)) = (density_image, force_image_gpu) {

        let aux = if let Some(aux) = aux_opt {
            aux
        } else {
            let create_tex = |label: &str| {
                render_device.create_texture(&TextureDescriptor {
                    label: Some(label),
                    size: den_img.texture.size(),
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Rgba32Float,
                    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST,
                    view_formats: &[],
                })
            };

            let den_out = create_tex("density_out");
            let vel_in = create_tex("velocity_in");
            let vel_out = create_tex("velocity_out");

            commands.insert_resource(FluidAuxTextures {
                density_out: den_out,
                velocity_in: vel_in,
                velocity_out: vel_out,
            });
            return;
        };

        let den_out_view = aux.density_out.create_view(&TextureViewDescriptor::default());
        let vel_in_view = aux.velocity_in.create_view(&TextureViewDescriptor::default());
        let vel_out_view = aux.velocity_out.create_view(&TextureViewDescriptor::default());

        #[repr(C)]
        #[derive(Copy, Clone, Pod, Zeroable)]
        struct ParamsUniform {
            dt: f32,
            grid_width: u32,
            grid_height: u32,
            decay: f32,
            viscosity: f32,
            padding: [f32; 3],
        }
        let p_uniform = ParamsUniform {
            dt: params.dt,
            grid_width: params.grid_width,
            grid_height: params.grid_height,
            decay: params.decay,
            viscosity: params.viscosity,
            padding: params.padding,
        };

        // Use Bevy's helper instead of DeviceExt
        let params_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("fluid_params_buffer"),
            contents: bytemuck::bytes_of(&p_uniform),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = render_device.create_bind_group(
            Some("fluid_bind_group"),
            &pipeline.bind_group_layout,
            &[
                BindGroupEntry { binding: 0, resource: BindingResource::TextureView(&den_img.texture_view) },
                BindGroupEntry { binding: 1, resource: BindingResource::TextureView(&den_out_view) },
                BindGroupEntry { binding: 2, resource: BindingResource::TextureView(&vel_in_view) },
                BindGroupEntry { binding: 3, resource: BindingResource::TextureView(&vel_out_view) },
                BindGroupEntry { binding: 4, resource: BindingResource::TextureView(&force_img.texture_view) },
                BindGroupEntry { binding: 5, resource: params_buffer.as_entire_binding() },
            ],
        );

        commands.insert_resource(FluidBindGroup(bind_group));
    }
}

#[derive(Default)]
struct FluidNode;

impl render_graph::Node for FluidNode {
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let pipeline = world.resource::<FluidPipeline>();
        let bind_group = match world.get_resource::<FluidBindGroup>() {
            Some(bg) => bg,
            None => return Ok(()),
        };
        let aux = match world.get_resource::<FluidAuxTextures>() {
            Some(a) => a,
            None => return Ok(()),
        };
        let fluid_image = world.resource::<FluidImage>();
        let gpu_images = world.resource::<RenderAssets<Image>>();
        let den_img = match gpu_images.get(&fluid_image.0) {
            Some(img) => img,
            None => return Ok(()),
        };

        {
            let mut pass = render_context.command_encoder().begin_compute_pass(&ComputePassDescriptor::default());
            pass.set_pipeline(&pipeline.pipeline);
            pass.set_bind_group(0, &bind_group.0, &[]);
            pass.dispatch_workgroups(32, 32, 1);
        }

        render_context.command_encoder().copy_texture_to_texture(
            ImageCopyTexture {
                texture: &aux.density_out,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            ImageCopyTexture {
                texture: &den_img.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            den_img.texture.size(),
        );

        render_context.command_encoder().copy_texture_to_texture(
            ImageCopyTexture {
                texture: &aux.velocity_out,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            ImageCopyTexture {
                texture: &aux.velocity_in,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            den_img.texture.size(),
        );

        Ok(())
    }
}
