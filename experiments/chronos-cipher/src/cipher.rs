use bevy::prelude::*;
use crate::mechanism::Gear;
use std::f32::consts::PI;

#[derive(Resource, Default)]
pub struct CipherState {
    pub key_stream: Vec<u8>,
    pub last_tick_time: f64,
}

pub fn read_cipher_system(
    mut cipher: ResMut<CipherState>,
    query: Query<(&Gear, &Transform)>,
    time: Res<Time>,
) {
    // Only sample every 0.2 seconds to simulate a "tick" rate or just based on physics speed
    if time.elapsed_seconds_f64() - cipher.last_tick_time < 0.2 {
        return;
    }
    cipher.last_tick_time = time.elapsed_seconds_f64();

    let mut combined_val: u32 = 0;
    let mut count = 0;

    // XOR the state of all gears
    for (gear, transform) in &query {
        // Z rotation
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let norm = (angle % (2.0 * PI) + 2.0 * PI) % (2.0 * PI);

        // Map angle to a byte value relative to tooth count
        // (angle / (2PI/teeth)) -> tooth index
        // fractional part -> phase

        let tooth_phase = (norm / (2.0 * PI) * (gear.teeth as f32) * 255.0) as u32;

        combined_val = combined_val.rotate_left(7) ^ tooth_phase;
        count += 1;
    }

    if count > 0 {
        let byte = (combined_val & 0xFF) as u8;
        cipher.key_stream.push(byte);
        if cipher.key_stream.len() > 32 {
            cipher.key_stream.remove(0);
        }
    }
}
