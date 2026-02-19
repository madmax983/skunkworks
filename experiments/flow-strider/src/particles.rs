use bevy::prelude::*;
use crate::strider::Limb;
use rand::prelude::*;

#[derive(Component)]
pub struct FlowParticle {
    pub t: f32,
    pub speed: f32,
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_particles, update_particles));
    }
}

fn spawn_particles(
    mut commands: Commands,
    limbs: Query<(Entity, &Limb)>,
) {
    let mut rng = thread_rng();
    for (limb_entity, _) in limbs.iter() {
        if rng.gen_bool(0.2) {
            let speed = rng.gen_range(0.8..1.5);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.7, 0.9, 1.0, 0.7),
                        custom_size: Some(Vec2::new(4.0, 4.0)),
                        ..default()
                    },
                    ..default()
                },
                FlowParticle {
                    t: 0.0,
                    speed,
                },
            )).set_parent(limb_entity);
        }
    }
}

fn update_particles(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Transform, &Parent, &mut FlowParticle)>,
    limbs: Query<(&Limb, &Parent)>,
    transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (entity, mut transform, parent_limb, mut particle) in particles.iter_mut() {
        if let Ok((limb, parent_body)) = limbs.get(parent_limb.get()) {
            if let Ok(body_transform) = transforms.get(parent_body.get()) {
                let body_pos = body_transform.translation().truncate();
                let local_foot = limb.foot_pos - body_pos;
                let knee = limb.knee_pos;

                particle.t += particle.speed * time.delta_seconds();

                if particle.t > 1.0 {
                    commands.entity(entity).despawn();
                } else {
                    let t = particle.t;
                    let p0 = Vec2::ZERO;
                    let p1 = knee;
                    let p2 = local_foot;

                    let pos = (1.0 - t).powi(2) * p0 + 2.0 * (1.0 - t) * t * p1 + t.powi(2) * p2;
                    transform.translation = pos.extend(2.0); // Z=2.0 to be on top of limb
                }
            } else {
                 commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
