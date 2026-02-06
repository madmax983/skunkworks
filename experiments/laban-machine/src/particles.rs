use crate::actor::Actor;
use crate::laban::Director;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

#[derive(Component)]
pub struct ParticleEmitter {
    pub timer: Timer,
}

impl Default for ParticleEmitter {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.01, TimerMode::Repeating), // 100 per second
        }
    }
}

pub fn particle_emitter_system(
    mut commands: Commands,
    time: Res<Time>,
    director: Res<Director>,
    mut query: Query<(&Transform, &mut ParticleEmitter), With<Actor>>,
) {
    let effort = &director.current_effort;

    for (transform, mut emitter) in query.iter_mut() {
        emitter.timer.tick(time.delta());

        if emitter.timer.finished() {
            // Spawn multiple particles per tick based on Time (Sudden = Burst, Sustained = Stream)
            let count = if effort.time < 0.3 { 10 } else { 2 };

            for _ in 0..count {
                let mut rng = rand::thread_rng();

                // Spread based on Space (Direct = 0.0, Indirect = 1.0)
                let spread = 5.0 + (effort.space * 50.0);
                let offset = Vec2::new(
                    rng.gen_range(-spread..spread),
                    rng.gen_range(-spread..spread),
                );

                let start_pos = transform.translation.truncate() + offset;

                // Initial velocity also affected by Space
                let vel_spread = 10.0 + (effort.space * 100.0);
                let velocity = Vec2::new(
                    rng.gen_range(-vel_spread..vel_spread),
                    rng.gen_range(-vel_spread..vel_spread),
                );

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                            custom_size: Some(Vec2::new(4.0, 4.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(start_pos.extend(-1.0)), // Behind actor
                        ..default()
                    },
                    Particle {
                        velocity,
                        lifetime: 1.0 + effort.time, // Time affects duration
                        max_lifetime: 1.0 + effort.time,
                    },
                ));
            }
        }
    }
}

pub fn particle_update_system(
    mut commands: Commands,
    time: Res<Time>,
    director: Res<Director>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    let effort = &director.current_effort;
    let dt = time.delta_seconds();

    for (entity, mut transform, mut sprite, mut particle) in query.iter_mut() {
        // Update lifetime
        particle.lifetime -= dt;
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Apply Forces

        // 1. Weight (Gravity)
        // Heavy (0.0) -> Gravity down. Light (1.0) -> Float up/neutral.
        // Map 0.0 -> -500.0 (Down), 1.0 -> 50.0 (Up)
        let gravity_y = -500.0 + (effort.weight * 600.0);
        particle.velocity.y += gravity_y * dt;

        // 2. Flow (Turbulence)
        // Bound (0.0) -> No noise. Free (1.0) -> High noise.
        if effort.flow > 0.1 {
            let mut rng = rand::thread_rng();
            let noise_mag = effort.flow * 500.0 * dt;
            particle.velocity.x += rng.gen_range(-noise_mag..noise_mag);
            particle.velocity.y += rng.gen_range(-noise_mag..noise_mag);
        }

        // Update position
        transform.translation += particle.velocity.extend(0.0) * dt;

        // Fade out alpha
        let alpha = particle.lifetime / particle.max_lifetime;

        // Color Change based on Effort
        // Strong (Weight 0) -> Red. Light (Weight 1) -> Blue.
        // Sudden (Time 0) -> Yellow. Sustained (Time 1) -> Green.
        let r = 1.0 - effort.weight;
        let b = effort.weight;
        let g = effort.time;

        sprite.color = Color::srgba(r, g, b, alpha);

        // Scale down
        transform.scale = Vec3::splat(alpha);
    }
}
