use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use memory::SpectralMemory;
use num_complex::Complex;
use rand::Rng;

mod memory;

#[derive(Resource)]
struct MemoryStore(SpectralMemory);

#[derive(Resource)]
struct OriginalMemory(SpectralMemory);

#[derive(Resource)]
struct MemoryTexture(Handle<Image>);

#[derive(Resource)]
struct DecayConfig {
    phase_drift: f32,
    noise: f32,
    paused: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(DecayConfig {
            phase_drift: 10.0, // Increased drift for visibility
            noise: 0.0001,
            paused: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (decay_system, reconstruction_system, input_system))
        .run();
}

fn generate_pattern(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let cx = (x as f32 / width as f32) * 255.0;
            let cy = (y as f32 / height as f32) * 255.0;
            // XOR pattern combined with gradient
            let check = ((x / 16) ^ (y / 16)) % 2 == 0;
            let val = if check { cx } else { cy };
            data.push(val as u8);
        }
    }
    data
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let width = 256;
    let height = 256;

    let data = generate_pattern(width, height);

    // Initialize Memory
    let mut mem = SpectralMemory::new(width, height);
    mem.ingest(&data);

    // Store original and current state
    commands.insert_resource(OriginalMemory(mem.clone()));
    commands.insert_resource(MemoryStore(mem));

    // Create Image Asset
    let extent = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    // Create Rgba8 image (grayscale in RGB)
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
    for &p in &data {
        rgba_data.extend_from_slice(&[p, p, p, 255]);
    }

    let image = Image::new(
        extent,
        TextureDimension::D2,
        rgba_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let handle = images.add(image);
    commands.insert_resource(MemoryTexture(handle.clone()));

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    });

    // UI instructions
    commands.spawn(
        TextBundle::from_section(
            "Spectral Decay: Space to Pause, R to Reset",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn decay_system(mut store: ResMut<MemoryStore>, config: Res<DecayConfig>, time: Res<Time>) {
    if config.paused {
        return;
    }

    let dt = time.delta_seconds();
    let mem = &mut store.0;
    let mut rng = rand::thread_rng();

    let w = mem.width as usize;
    let h = mem.height as usize;

    // Parallel iter? No, simple loop is fine for 65k items.

    for y in 0..h {
        let fy = if y <= h / 2 { y } else { h - y };
        for x in 0..w {
            let fx = if x <= w / 2 { x } else { w - x };
            // Normalized frequency squared (0.0 to 1.0 approx if divided by N/2^2)
            let f_sq = (fx * fx + fy * fy) as f32;

            let idx = y * w + x;
            let c = &mut mem.buffer[idx];

            // Higher freq decays faster (Low Pass Filter)
            // e^(-k * f^2)
            let decay_factor = 1.0 - (f_sq * 0.000005 * dt * 60.0);
            *c = *c * decay_factor.max(0.99);

            // Phase drift proportional to freq
            let drift = (f_sq.sqrt() * config.phase_drift * dt) * rng.gen_range(-0.1..0.1);
            let rot = Complex::from_polar(1.0, drift);
            *c = *c * rot;

            // Noise (Cosmic Rays)
            if rng.gen_bool(config.noise as f64) {
                let spike = Complex::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0));
                *c = *c + spike;
            }
        }
    }
}

fn reconstruction_system(
    store: Res<MemoryStore>,
    tex_res: Res<MemoryTexture>,
    mut images: ResMut<Assets<Image>>,
) {
    let recalled = store.0.recall();

    if let Some(image) = images.get_mut(&tex_res.0) {
        // Convert u8 grayscale to RGBA
        for (i, &val) in recalled.iter().enumerate() {
            let idx = i * 4;
            if idx + 3 < image.data.len() {
                image.data[idx] = val;
                image.data[idx + 1] = val;
                image.data[idx + 2] = val;
                image.data[idx + 3] = 255;
            }
        }
    }
}

fn input_system(
    mut config: ResMut<DecayConfig>,
    keys: Res<ButtonInput<KeyCode>>,
    mut store: ResMut<MemoryStore>,
    original: Res<OriginalMemory>,
) {
    if keys.just_pressed(KeyCode::Space) {
        config.paused = !config.paused;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        store.0 = original.0.clone();
    }
}
