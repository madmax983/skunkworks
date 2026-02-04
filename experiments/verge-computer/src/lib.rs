use bevy::prelude::*;

pub mod cpu;
pub mod mechanism;
pub mod view;

use cpu::{cpu_tick_system, TickEvent, Program};

pub struct VergeComputerPlugin;

impl Plugin for VergeComputerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .init_resource::<Program>()
            .add_systems(Update, (cpu_tick_system, detect_tick_system));
    }
}

#[derive(Component)]
pub struct EscapeWheel {
    pub last_angle: f32,
    pub teeth: usize,
    pub cumulative_angle: f32,
}

#[derive(Component)]
pub struct Anchor;

fn detect_tick_system(
    mut events: EventWriter<TickEvent>,
    mut query: Query<(&Transform, &mut EscapeWheel)>,
) {
    for (transform, mut wheel) in &mut query {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2; // Z rotation

        // Handle wrapping -PI to PI
        let mut delta = angle - wheel.last_angle;

        if delta > std::f32::consts::PI {
            delta -= 2.0 * std::f32::consts::PI;
        } else if delta < -std::f32::consts::PI {
            delta += 2.0 * std::f32::consts::PI;
        }

        wheel.last_angle = angle;

        // We assume the wheel turns in one direction mostly, but escapement rocks back and forth slightly (recoil).
        // However, the net motion is forward.
        // We only care about net forward motion for a "tick".
        // If we just sum abs(delta), we count the recoil as progress.
        // We should sum delta (signed). If it's negative (clockwise), we count up.
        // Assuming clockwise rotation for the wheel.

        wheel.cumulative_angle += delta; // Signed sum

        // Check if we passed a tooth
        let tooth_angle = 2.0 * std::f32::consts::PI / (wheel.teeth as f32);

        // Assuming we are moving in negative Z (clockwise) direction
        if wheel.cumulative_angle.abs() >= tooth_angle {
            // Reset by one tooth worth (keeping the remainder)
            if wheel.cumulative_angle < 0.0 {
                wheel.cumulative_angle += tooth_angle;
            } else {
                wheel.cumulative_angle -= tooth_angle;
            }
            events.send(TickEvent);
            // info!("Tick!");
        }
    }
}
