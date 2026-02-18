use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice, RenderQueue},
        texture::ImageSampler,
    },
    utils::HashMap,
};
use std::borrow::Cow;
use bytemuck::{Pod, Zeroable};

// Fluid Parameters Uniform
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct FluidParams {
    dt: f32,
    grid_width: u32,
    grid_height: u32,
    decay: f32,
    viscosity: f32,
    _padding: [f32; 3], // Align to 16 bytes? struct is 20 bytes (5*4). Needs padding to 32?
    // WGSL align rules:
    // f32 (4), u32 (4), u32 (4), f32 (4), f32 (4). Total 20.
    // Uniform buffers need 16-byte alignment usually?
    // 20 is not multiple of 16. Padding to 32 is safe.
    // 32 - 20 = 12 bytes = 3 f32s.
}

#[derive(Resource)]
pub struct FluidContext {
    pipeline: ComputePipeline,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,

    // Resources
    density_in: TextureView,
    density_out: TextureView,
    velocity_in: TextureView,
    velocity_out: TextureView,
    force_tex: TextureView,
    params_buffer: Buffer,

    // Texture Handles for resizing or recreating?
    // For now fixed size.
    pub density_image_handle: Handle<Image>, // For rendering
}

pub struct FluidPlugin;

impl Plugin for FluidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fluid)
           .add_systems(Update, update_fluid);
    }
}

const GRID_SIZE: u32 = 256;

fn setup_fluid(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let size = Extent3d {
        width: GRID_SIZE,
        height: GRID_SIZE,
        depth_or_array_layers: 1,
    };

    // Helper to create texture
    let create_texture = |label: &str, format: TextureFormat, usage: TextureUsages| {
        render_device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        })
    };

    // 1. Textures
    // Density In/Out (RGBA32Float for high precision, or RGBA16Float)
    let format = TextureFormat::Rgba32Float;
    let usage = TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::COPY_SRC;

    let den_in = create_texture("density_in", format, usage);
    let den_out = create_texture("density_out", format, usage);
    let vel_in = create_texture("velocity_in", format, usage);
    let vel_out = create_texture("velocity_out", format, usage);
    // Force texture needs to be writable from CPU (COPY_DST) and readable in shader (TEXTURE_BINDING)
    let force = create_texture("force", format, usage);

    let den_in_view = den_in.create_view(&TextureViewDescriptor::default());
    let den_out_view = den_out.create_view(&TextureViewDescriptor::default());
    let vel_in_view = vel_in.create_view(&TextureViewDescriptor::default());
    let vel_out_view = vel_out.create_view(&TextureViewDescriptor::default());
    let force_view = force.create_view(&TextureViewDescriptor::default());

    // Create an Image for rendering that points to density_in (or out?)
    // Actually, we can just create a Bevy Image from the texture, but raw wgpu texture conversion to Bevy Image is complex manually?
    // Easier: Create Bevy Image normally, then extract its texture view.
    // But Bevy Images are usually 8-bit rgba.
    // We want Rgba32Float.

    // Let's stick to using the texture directly in a custom Material.
    // But to pass it to Material, we need a Handle<Image>.
    // So we should create the Image via Assets<Image> and let Bevy manage the texture.

    // Revised Strategy:
    // Create 5 Images in Assets<Image> with Rgba32Float format.
    // Get their GpuImage (TextureView) in the update system?
    // But `RenderDevice` access in Main World is fine, but extracting `GpuImage` from `Handle<Image>` is tricky because `GpuImage` is in Render Assets.

    // OK, "Moonshot" Hack:
    // Use `render_device` to create textures (as above).
    // Create a dummy `Image` with the same size/format and put it in Assets.
    // Manually inject the `wgpu::Texture` into the `Image`? No, `Image` stores CPU data.

    // Best way:
    // Just use `render_device` for simulation textures.
    // For visualization, copy `density_out` to a `TEXTURE_BINDING` texture that is managed by a `Handle<Image>`.

    // Let's proceed with custom `FluidContext` resource holding raw wgpu objects.

    // 2. Uniform Buffer
    let params = FluidParams {
        dt: 0.1,
        grid_width: GRID_SIZE,
        grid_height: GRID_SIZE,
        decay: 0.99,
        viscosity: 0.0, // Not used yet
        _padding: [0.0; 3],
    };
    let params_buffer = render_device.create_buffer_init(&BufferInitDescriptor {
        label: Some("fluid_params"),
        contents: bytemuck::bytes_of(&params),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    // 3. Bind Group Layout
    let bind_group_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("fluid_bind_group_layout"),
        entries: &[
            // 0: density_in (texture)
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
            // 1: density_out (storage)
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::WriteOnly,
                    format: format,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            // 2: velocity_in (texture)
            BindGroupLayoutEntry {
                binding: 2,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // 3: velocity_out (storage)
            BindGroupLayoutEntry {
                binding: 3,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::WriteOnly,
                    format: format,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
            // 4: force_tex (texture)
            BindGroupLayoutEntry {
                binding: 4,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: false },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // 5: params (uniform)
            BindGroupLayoutEntry {
                binding: 5,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    // 4. Bind Group
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("fluid_bind_group"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry { binding: 0, resource: BindingResource::TextureView(&den_in_view) },
            BindGroupEntry { binding: 1, resource: BindingResource::TextureView(&den_out_view) },
            BindGroupEntry { binding: 2, resource: BindingResource::TextureView(&vel_in_view) },
            BindGroupEntry { binding: 3, resource: BindingResource::TextureView(&vel_out_view) },
            BindGroupEntry { binding: 4, resource: BindingResource::TextureView(&force_view) },
            BindGroupEntry { binding: 5, resource: params_buffer.as_entire_binding() },
        ],
    });

    // 5. Pipeline
    // Load shader module. This is tricky in main world synchronously.
    // But `setup_fluid` is Startup system. Assets might not be loaded.
    // We should use `AssetServer` to load shader, but then we have to wait.
    // Hack: Include shader source as string using `include_str!` for instant load.

    let shader_source = include_str!("../assets/shaders/fluid.wgsl");
    let shader = render_device.create_shader_module(ShaderModuleDescriptor {
        label: Some("fluid_shader"),
        source: ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
    });

    let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("fluid_pipeline_layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = render_device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("fluid_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "update",
    });

    // Create a handle for visualization
    // We create a Bevy Image that *wraps* our density_in texture?
    // No, we can't easily wrap an existing wgpu texture into a Handle<Image> without unsafe hacks in Bevy.
    // Instead, we will create a Bevy Image normally, and copy our simulation result to it every frame.

    let mut viz_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255], // Initial data
        TextureFormat::Rgba8Unorm, // Render shader expects standard format? Or can use Float?
        // Let's use Rgba32Float for visualization too if possible, but Rgba8Unorm is safer for display.
        // Copying Float texture to Unorm texture requires a render pass (blit), copy_texture_to_texture only works for same format.
        // So let's make the Viz Image Rgba32Float too.
    );
    viz_image.texture_descriptor.format = TextureFormat::Rgba32Float;
    viz_image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;

    let viz_handle = images.add(viz_image);

    commands.insert_resource(FluidContext {
        pipeline,
        bind_group_layout, // Keep alive
        bind_group,
        density_in: den_in_view,
        density_out: den_out_view,
        velocity_in: vel_in_view,
        velocity_out: vel_out_view,
        force_tex: force_view,
        params_buffer,
        density_image_handle: viz_handle,
    });

    // Also store the raw textures in the context to copy between them
    // (Wait, TextureView doesn't support copy, need Texture object.
    // I dropped the Texture objects `den_in`, etc.
    // I need to store them in FluidContext.
}

// Need to update FluidContext to store Textures
// ...

fn update_fluid(
    ctx: Res<FluidContext>,
    render_queue: Res<RenderQueue>,
    // Needed to copy to viz image:
    images: Res<Assets<Image>>,
    render_device: Res<RenderDevice>,
) {
    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor { label: Some("fluid_encoder") });

    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor { label: Some("fluid_pass"), timestamp_writes: None });
        pass.set_pipeline(&ctx.pipeline);
        pass.set_bind_group(0, &ctx.bind_group, &[]);
        pass.dispatch_workgroups(GRID_SIZE / 8, GRID_SIZE / 8, 1);
    }

    // Copy out -> in for next frame
    // We need the Texture objects, not just Views.
    // (Assuming I fixed FluidContext to have them)

    // And copy density -> viz image
    // Get GPU texture of viz image
    if let Some(viz_gpu_image) = images.get_gpu_image(&ctx.density_image_handle) { // wait, get_gpu_image isn't on Assets<Image>
        // It's in RenderAssets<Image>, which is in Render World.
        // We are in Main World.
        // We can't access GPU resource of the Handle easily here.
        // This is the tricky part of Bevy Main World compute.
    }
}
