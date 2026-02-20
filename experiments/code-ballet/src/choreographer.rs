use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::skeleton::{Dancer, DancerPart, Motor};
use crate::stage::{StageMap, Platform};

#[derive(Component, Default)]
pub struct Choreographer {
    pub state: DanceState,
    pub target_platform: Option<Entity>,
    pub timer: f32,
}

#[derive(Default, PartialEq, Debug)]
pub enum DanceState {
    #[default]
    Idle,
    Prepare,
    Jump,
    Airborne,
    Land,
}

pub struct ChoreographerPlugin;

impl Plugin for ChoreographerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ensure_choreographer, choreographer_logic));
    }
}

fn ensure_choreographer(
    mut commands: Commands,
    query: Query<Entity, (With<Dancer>, Without<Choreographer>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Choreographer::default());
    }
}

fn choreographer_logic(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Choreographer, &Transform, &Velocity), With<Dancer>>,
    mut motors: Query<(&mut Motor, &DancerPart)>,
    stage: Res<StageMap>,
    platforms: Query<(&Transform, &Platform)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    for (dancer_entity, mut choreo, transform, velocity) in query.iter_mut() {
        choreo.timer -= dt;

        match choreo.state {
            DanceState::Idle => {
                // If on ground (velocity low) and timer done, pick next target
                // Also check if we are actually touching something? Velocity check is simple proxy.
                if choreo.timer <= 0.0 && velocity.linvel.y.abs() < 1.0 {
                    // Find next platform
                    let current_x = transform.translation.x;
                    let mut best_target = None;
                    let mut min_dist = f32::MAX;

                    for platform_entity in &stage.platforms {
                        if let Ok((p_transform, _platform)) = platforms.get(*platform_entity) {
                            let dist = p_transform.translation.x - current_x;
                            if dist > 50.0 && dist < 600.0 && dist < min_dist { // Look forward
                                min_dist = dist;
                                best_target = Some(*platform_entity);
                            }
                        }
                    }

                    if let Some(target) = best_target {
                        choreo.target_platform = Some(target);
                        choreo.state = DanceState::Prepare;
                        choreo.timer = 0.3; // Prepare time
                    } else {
                         // End of stage? Turn around?
                         // For now, just reset timer
                         choreo.timer = 1.0;
                    }
                }
            }
            DanceState::Prepare => {
                // Crouch
                for (mut motor, part) in motors.iter_mut() {
                     if part.name.contains("Thigh") { motor.target_angle = 1.0; } // Flex hip
                     if part.name.contains("Shin") { motor.target_angle = 2.0; } // Flex knee
                }

                if choreo.timer <= 0.0 {
                    choreo.state = DanceState::Jump;
                    // Timer acts as a frame counter here effectively
                }
            }
            DanceState::Jump => {
                // Extend legs
                 for (mut motor, part) in motors.iter_mut() {
                     motor.stiffness = 80000.0;
                     if part.name.contains("Thigh") { motor.target_angle = -0.5; } // Extend hip
                     if part.name.contains("Shin") { motor.target_angle = 0.0; } // Extend knee
                }

                // Apply impulse for the jump
                // Scale impulse by target distance maybe?
                let impulse_x = 10000.0;
                let impulse_y = 35000.0;

                commands.entity(dancer_entity).insert(ExternalImpulse {
                     impulse: Vec2::new(impulse_x, impulse_y),
                     torque_impulse: 0.0,
                });

                choreo.state = DanceState::Airborne;
                choreo.timer = 0.5; // Min air time
            }
            DanceState::Airborne => {
                // Pose
                 for (mut motor, part) in motors.iter_mut() {
                     if part.name.contains("Arm") { motor.target_angle = 2.5; } // Arms up
                     if part.name.contains("Shin") { motor.target_angle = 1.0; } // Tuck knees
                }

                // Check landing
                if choreo.timer <= 0.0 && velocity.linvel.y < 0.0 {
                    // Raycast down? Or just velocity check?
                    // Simple logic: if moving down, prepare to land
                    choreo.state = DanceState::Land;
                    choreo.timer = 0.2;
                }
            }
            DanceState::Land => {
                // Absorb
                 for (mut motor, part) in motors.iter_mut() {
                     motor.stiffness = 5000.0; // Soft knees
                     if part.name.contains("Thigh") { motor.target_angle = 0.5; }
                     if part.name.contains("Shin") { motor.target_angle = 1.0; }
                }

                if choreo.timer <= 0.0 && velocity.linvel.y.abs() < 5.0 {
                    choreo.state = DanceState::Idle;
                    choreo.timer = 0.5;
                     // Reset stiffness
                     for (mut motor, _part) in motors.iter_mut() {
                         motor.stiffness = 50000.0;
                    }
                }
            }
        }
    }
}
