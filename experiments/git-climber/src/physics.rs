use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::climber::{Limb, HandSensor, Climber};
use crate::cliff::CommitLedge;

#[derive(Component)]
pub struct GrabbingLedge {
    pub ledge_entity: Entity,
    pub grab_point_local: Vec2, // Point on ledge relative to ledge center
    pub timer: Timer,
}

pub fn climb_control(
    mut commands: Commands,
    sensors: Query<(Entity, &Parent, &GlobalTransform), With<HandSensor>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut limbs: Query<(Entity, &mut ExternalImpulse, &mut ExternalForce, &GlobalTransform), With<Limb>>,
    ledges: Query<(Entity, &GlobalTransform), With<CommitLedge>>,
    mut grabbing_limbs: Query<(Entity, &mut GrabbingLedge, &GlobalTransform)>,
    _climber: Query<(Entity, &GlobalTransform), With<Climber>>,
    time: Res<Time>,
) {
    // 1. Handle New Grabs
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (sensor_entity, ledge_entity) = if sensors.contains(*e1) && ledges.contains(*e2) {
                (*e1, *e2)
            } else if sensors.contains(*e2) && ledges.contains(*e1) {
                (*e2, *e1)
            } else {
                continue;
            };

            // Get Limb
            if let Ok((_, parent, sensor_transform)) = sensors.get(sensor_entity) {
                let limb_entity = parent.get();

                // If not already grabbing
                if !grabbing_limbs.contains(limb_entity) {
                    // Calculate grab point on ledge
                    if let Ok((_, ledge_transform)) = ledges.get(ledge_entity) {
                         let grab_point_global = sensor_transform.translation().truncate();
                         let ledge_pos = ledge_transform.translation().truncate();
                         let grab_point_local = grab_point_global - ledge_pos;

                         commands.entity(limb_entity).insert(GrabbingLedge {
                             ledge_entity,
                             grab_point_local,
                             timer: Timer::from_seconds(2.0, TimerMode::Once), // Hold for 2 seconds
                         });
                    }
                }
            }
        }
    }

    // 2. Apply Magnetic Force for Grabbing Limbs
    for (limb_entity, mut grabbing, limb_transform) in grabbing_limbs.iter_mut() {
        grabbing.timer.tick(time.delta());

        if grabbing.timer.finished() {
            commands.entity(limb_entity).remove::<GrabbingLedge>();
            if let Ok((_, _, mut ext_force, _)) = limbs.get_mut(limb_entity) {
                ext_force.force = Vec2::ZERO; // Reset force on release
            }
            continue;
        }

        if let Ok((_, ledge_transform)) = ledges.get(grabbing.ledge_entity) {
            let target_pos = ledge_transform.translation().truncate() + grabbing.grab_point_local;
            let current_pos = limb_transform.translation().truncate();

            // Vector to target
            let delta = target_pos - current_pos;
            let distance = delta.length();

            if distance > 0.0 {
                let force_dir = delta.normalize();
                let force_mag = 5000.0 * distance.min(10.0); // Spring force

                // Set force to Limb
                if let Ok((_, _, mut ext_force, _)) = limbs.get_mut(limb_entity) {
                    ext_force.force = force_dir * force_mag;
                }
            }
        } else {
            // Ledge gone?
            commands.entity(limb_entity).remove::<GrabbingLedge>();
        }
    }

    // 3. Reach Up (if not grabbing)
    // Set upward force to all limbs periodically
    for (limb_entity, _, mut ext_force, _) in limbs.iter_mut() {
        if !grabbing_limbs.contains(limb_entity) {
             // Flail upwards
             let noise = (time.elapsed_seconds() * 5.0 + limb_entity.index() as f32).sin();
             // Base lift + noise
             ext_force.force = Vec2::new(noise * 200.0, 500.0 + noise * 100.0);
        }
    }
}
