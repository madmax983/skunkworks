use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};
use crossbeam::channel::{unbounded, Receiver};
use sysinfo::System;

mod audio;
mod fluid;
mod rhythm;

use audio::AudioEngine;
use fluid::{FluidImage, FluidParams, FluidPlugin, ForceImage};
use rhythm::{Conductor, MusicianState, RhythmEvent};

// Resource to hold the receiver for Bevy system
#[derive(Resource)]
struct RhythmReceiver(Receiver<RhythmEvent>);

// Resource to hold system info
#[derive(Resource)]
struct SysMonitor {
    sys: System,
}

fn main() {
    // 1. Setup Channels
    let (audio_tx, audio_rx) = unbounded();
    let (rhythm_tx, rhythm_rx) = unbounded();

    // 2. Start Audio Engine
    let audio_engine = AudioEngine::new(audio_rx).expect("Failed to init audio");

    // 3. Start Conductor
    let mut conductor = Conductor::new(audio_tx, rhythm_tx);
    // Add musicians (Polyrhythms)
    // Id, Loop(ms), Hold(ms), Freq
    conductor.add_musician(1, 400, 80, 220.0); // A3
    conductor.add_musician(2, 500, 80, 277.18); // C#4
    conductor.add_musician(3, 600, 80, 329.63); // E4
    conductor.add_musician(4, 700, 80, 440.0); // A4
    conductor.add_musician(5, 1100, 150, 110.0); // A2 Bass

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FluidPlugin)
        .insert_resource(RhythmReceiver(rhythm_rx))
        .insert_resource(SysMonitor {
            sys: System::new_all(),
        })
        .insert_non_send_resource(audio_engine) // Keep it alive
        .insert_non_send_resource(conductor) // Keep threads alive
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_rhythm_forces, update_viscosity))
        .run();
}

const GRID_SIZE: u32 = 256;

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Init Fluid Image (Density)
    let mut fluid_img = Image::new_fill(
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
    fluid_img.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::COPY_SRC;
    let fluid_handle = images.add(fluid_img);
    commands.insert_resource(FluidImage(fluid_handle.clone()));

    // Init Force Image
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

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sprite
    commands.spawn(SpriteBundle {
        texture: fluid_handle,
        transform: Transform::from_scale(Vec3::splat(3.0)),
        ..default()
    });
}

fn apply_rhythm_forces(
    receiver: Res<RhythmReceiver>,
    mut images: ResMut<Assets<Image>>,
    force_res: Res<ForceImage>,
    time: Res<Time>,
) {
    let force_image = images.get_mut(&force_res.0).unwrap();
    let data: &mut [f32] = bytemuck::cast_slice_mut(&mut force_image.data);

    // Clear previous forces
    data.fill(0.0);

    // Read all pending events
    while let Ok(event) = receiver.0.try_recv() {
        let RhythmEvent::StateChange(id, state) = event;
        if state == MusicianState::Playing {
            // Determine position based on ID
            // Map IDs 1..5 to locations
            let (x, y) = match id {
                1 => (GRID_SIZE / 2, GRID_SIZE / 2),         // Center
                2 => (GRID_SIZE / 4, GRID_SIZE / 4),         // Top Left
                3 => (GRID_SIZE / 4 * 3, GRID_SIZE / 4),     // Top Right
                4 => (GRID_SIZE / 4, GRID_SIZE / 4 * 3),     // Bottom Left
                5 => (GRID_SIZE / 4 * 3, GRID_SIZE / 4 * 3), // Bottom Right
                _ => (GRID_SIZE / 2, GRID_SIZE / 2),
            };

            let idx = ((y as u32 * GRID_SIZE + x as u32) * 4) as usize;

            if idx + 4 < data.len() {
                // Inject Density (Color)
                // R channel = density
                data[idx] = 10.0; // Strong puff

                // Inject Velocity
                // Random or directional?
                let angle = (time.elapsed_seconds() * 10.0 + id as f32) % 6.28;
                let speed = 50.0;
                data[idx + 1] = angle.cos() * speed; // G = Vel X
                data[idx + 2] = angle.sin() * speed; // B = Vel Y
            }
        }
    }
}

fn update_viscosity(mut sys_mon: ResMut<SysMonitor>, mut params: ResMut<FluidParams>) {
    // Refresh CPU
    sys_mon.sys.refresh_cpu();
    let usage = sys_mon.sys.global_cpu_info().cpu_usage(); // 0..100

    // Map usage to viscosity
    let target_viscosity = (usage / 1000.0).clamp(0.0, 0.2);

    // Smooth transition
    params.viscosity = params.viscosity * 0.95 + target_viscosity * 0.05;

    let target_decay = 0.99 + (usage / 10000.0);
    params.decay = params.decay * 0.9 + target_decay.clamp(0.9, 0.999) * 0.1;
}
