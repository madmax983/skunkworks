use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

mod fluid;
mod system_monitor;

use fluid::{FluidImage, FluidPlugin, ForceImage};
use system_monitor::{SystemMonitorPlugin, SystemStats};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SystemMonitorPlugin)
        .add_plugins(FluidPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_forces)
        .run();
}

const GRID_SIZE: u32 = 256;

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Create Fluid Image (Density)
    // Must be Rgba32Float for compute shader compatibility (we copy Rgba32Float -> Rgba32Float)
    let mut fluid_img = Image::new_fill(
        Extent3d {
            width: GRID_SIZE,
            height: GRID_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0], // Initial clear
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    // Important: Usage must include STORAGE_BINDING (for compute write) and COPY_DST (for copy from aux)
    fluid_img.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::COPY_SRC;

    let fluid_handle = images.add(fluid_img);
    commands.insert_resource(FluidImage(fluid_handle.clone()));

    // 2. Create Force Image (Input)
    // Rgba32Float: R=Density, G=VelX, B=VelY
    let mut force_img = Image::new_fill(
        Extent3d {
            width: GRID_SIZE,
            height: GRID_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba32Float,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    force_img.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;

    let force_handle = images.add(force_img);
    commands.insert_resource(ForceImage(force_handle));

    // 3. Camera
    commands.spawn(Camera2dBundle::default());

    // 4. Visualization Quad
    // We use a custom shader? Or just use ColorMaterial with the texture?
    // Using ColorMaterial means standard sprite shader.
    // Standard sprite shader handles Rgba32Float textures usually? Yes.
    // But we want "Heatmap" coloring.
    // So we should use a custom Material2d.
    // For "Moonshot" MVP, let's just use the raw texture first. It will look black/white/red depending on channels.
    // R channel is density.
    // If we want color mapping, we need custom shader.
    // Let's stick to simple Sprite first to verify it works.

    commands.spawn(SpriteBundle {
        texture: fluid_handle,
        transform: Transform::from_scale(Vec3::splat(2.0)), // Scale up 2x (512x512)
        ..default()
    });
}

fn update_forces(
    stats: Res<SystemStats>,
    mut images: ResMut<Assets<Image>>,
    force_res: Res<ForceImage>,
    time: Res<Time>,
) {
    let force_image = images.get_mut(&force_res.0).unwrap();

    // Clear forces
    // We need to write to the data buffer.
    // Format is Rgba32Float (4 * f32 = 16 bytes per pixel).

    // Reuse buffer logic
    // We can map `force_image.data` as `&mut [f32]`.
    let data: &mut [f32] = bytemuck::cast_slice_mut(&mut force_image.data);

    // Reset to 0
    data.fill(0.0);

    // Inject forces based on system stats
    // Total CPU -> Global Turbulence
    // Top Processes -> Local Injection

    let _rng = rand::thread_rng();

    for (pid, _name, cpu, mem) in &stats.top_processes {
        // Map PID to position
        // Hash PID to get stable position
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        pid.hash(&mut hasher);
        let hash = hasher.finish();

        let x = (hash % (GRID_SIZE as u64)) as usize;
        let y = ((hash / (GRID_SIZE as u64)) % (GRID_SIZE as u64)) as usize;

        let idx = (y * GRID_SIZE as usize + x) * 4;

        if idx + 4 < data.len() {
            // R = Density (Heat) based on CPU
            let heat = *cpu / 100.0; // 0..1

            // G, B = Velocity (Random direction based on hash time?)
            // We want swirling.
            let angle = (time.elapsed_seconds() * 2.0 + (pid % 100) as f32) as f32;
            let speed = (*mem as f32 / 1024.0 / 1024.0 / 100.0).clamp(0.0, 5.0); // MB -> speed factor

            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;

            // Write to pixel and neighbors (splat)
            // Just single pixel for now
            data[idx] += heat * 5.0;     // Density
            data[idx + 1] += vx;   // Vel X
            data[idx + 2] += vy;   // Vel Y
            // Alpha unused
        }
    }

    // Also add mouse interaction if needed?
    // Not implemented yet.
}
