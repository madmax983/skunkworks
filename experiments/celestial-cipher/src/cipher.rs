use bevy::prelude::*;
use crate::physics::Gear;

#[derive(Resource, Default)]
pub struct CipherState {
    pub key: String,
    pub last_sun_angle: f32,
    pub generated_count: usize,
}

pub struct CipherPlugin;

impl Plugin for CipherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CipherState>()
           .add_systems(Update, read_cipher_system);
    }
}

fn read_cipher_system(
    mut cipher: ResMut<CipherState>,
    query: Query<(&Transform, &Gear)>,
) {
    let mut sun_angle = 0.0;
    let mut moon_angle = 0.0;
    let mut zodiac_angle = 0.0;
    let mut sun_found = false;
    let mut moon_gear = None;
    let mut zodiac_gear = None;

    for (transform, gear) in query.iter() {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2; // Z-rotation
        // Normalize to 0..2PI
        let norm_angle = (angle % (std::f32::consts::PI * 2.0) + std::f32::consts::PI * 2.0) % (std::f32::consts::PI * 2.0);

        if gear.name == "Sun" {
            sun_angle = norm_angle;
            sun_found = true;
        } else if gear.name == "Moon" {
            moon_angle = norm_angle;
            moon_gear = Some(gear);
        } else if gear.name == "Zodiac" {
            zodiac_angle = norm_angle;
            zodiac_gear = Some(gear);
        }
    }

    if !sun_found { return; }

    // Detect wrap-around (Sun passes 0)
    // If last > 5.0 (approx 2PI - 1) and current < 1.0
    // 2*PI = 6.28
    if cipher.last_sun_angle > 5.0 && sun_angle < 1.0 {
        // Wrapped!
        cipher.generated_count += 1;

        let moon_char = if let Some(g) = moon_gear {
            get_char_from_angle(moon_angle, g.teeth)
        } else { '?' };

        let zodiac_char = if let Some(g) = zodiac_gear {
            get_char_from_angle(zodiac_angle, g.teeth)
        } else { '?' };

        // Append to key
        cipher.key.push(moon_char);
        cipher.key.push(zodiac_char);

        // Keep key length manageable for display
        let current_len = cipher.key.len();
        if current_len > 64 {
            let kept = cipher.key.split_off(current_len - 64);
            cipher.key = kept;
        }

        info!("TICK {}: Moon={} Zodiac={} -> Key: {}", cipher.generated_count, moon_char, zodiac_char, cipher.key);
    }

    cipher.last_sun_angle = sun_angle;
}

fn get_char_from_angle(angle: f32, teeth: usize) -> char {
    // Map angle to tooth index
    let step = (std::f32::consts::PI * 2.0) / (teeth as f32);
    // Align so that index 0 is at angle 0.
    // Round to nearest tooth center.
    let index = (angle / step).round() as usize % teeth;

    // Map index to a character set
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let char_idx = index % charset.len();
    charset.chars().nth(char_idx).unwrap_or('?')
}
