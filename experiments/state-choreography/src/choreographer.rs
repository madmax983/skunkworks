use bevy::prelude::*;
use crate::components::*;

pub struct ChoreographerPlugin;

impl Plugin for ChoreographerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Choreographer {
            current_state: LabanState::Idle,
            next_state: LabanState::ReachHigh,
            transition_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            state_start_time: 0.0,
        });
        app.add_systems(Update, update_choreography);
    }
}

fn update_choreography(
    time: Res<Time>,
    mut choreographer: ResMut<Choreographer>,
    mut target_query: Query<&mut Transform>,
    chain_query: Query<(&IKChain, &Limb)>,
) {
    choreographer.transition_timer.tick(time.delta());

    // Normalize t based on timer
    // Timer is Repeating, so it goes 0..duration then wraps.
    // We want 0..1 during the transition.
    // If we are transitioning A -> B, we use fraction.
    // Once finished, we become B -> B (or B -> C).

    let t = choreographer.transition_timer.fraction();
    // Smoothstep for natural motion
    let smooth_t = t * t * (3.0 - 2.0 * t);

    for (chain, limb) in chain_query.iter() {
        if let Ok(mut target_transform) = target_query.get_mut(chain.target) {
            let start_pos = get_target_pos(choreographer.current_state, *limb);
            let end_pos = get_target_pos(choreographer.next_state, *limb);

            let new_pos = if choreographer.current_state == choreographer.next_state {
                end_pos
            } else {
                 start_pos.lerp(end_pos, smooth_t)
            };

            target_transform.translation = new_pos;
        }
    }

    if choreographer.transition_timer.just_finished() {
        // Commit the transition
        choreographer.current_state = choreographer.next_state;

        // Pick new random state for demo?
        // Or specific sequence.
        // Let's cycle: Idle -> ReachHigh -> Crouch -> TPose -> Arabesque -> Jump -> Idle
        let next = match choreographer.current_state {
            LabanState::Idle => LabanState::ReachHigh,
            LabanState::ReachHigh => LabanState::Crouch,
            LabanState::Crouch => LabanState::TPose,
            LabanState::TPose => LabanState::Arabesque,
            LabanState::Arabesque => LabanState::Jump,
            LabanState::Jump => LabanState::Idle,
        };
        choreographer.next_state = next;
    }
}

fn get_target_pos(state: LabanState, limb: Limb) -> Vec3 {
    match state {
        LabanState::Idle => match limb {
            Limb::LeftArm => Vec3::new(-30.0, -50.0, 0.0),
            Limb::RightArm => Vec3::new(30.0, -50.0, 0.0),
            Limb::LeftLeg => Vec3::new(-20.0, -100.0, 0.0),
            Limb::RightLeg => Vec3::new(20.0, -100.0, 0.0),
            Limb::Head => Vec3::new(0.0, 50.0, 0.0),
        },
        LabanState::ReachHigh => match limb {
            Limb::LeftArm => Vec3::new(-50.0, 100.0, 0.0),
            Limb::RightArm => Vec3::new(50.0, 100.0, 0.0),
            Limb::LeftLeg => Vec3::new(-20.0, -100.0, 0.0),
            Limb::RightLeg => Vec3::new(20.0, -100.0, 0.0),
            Limb::Head => Vec3::new(0.0, 60.0, 0.0),
        },
        LabanState::Crouch => match limb {
            Limb::LeftArm => Vec3::new(-40.0, -80.0, 0.0),
            Limb::RightArm => Vec3::new(40.0, -80.0, 0.0),
            Limb::LeftLeg => Vec3::new(-30.0, -50.0, 0.0),
            Limb::RightLeg => Vec3::new(30.0, -50.0, 0.0),
            Limb::Head => Vec3::new(0.0, 0.0, 0.0),
        },
        LabanState::TPose => match limb {
            Limb::LeftArm => Vec3::new(-100.0, 0.0, 0.0),
            Limb::RightArm => Vec3::new(100.0, 0.0, 0.0),
            Limb::LeftLeg => Vec3::new(-20.0, -100.0, 0.0),
            Limb::RightLeg => Vec3::new(20.0, -100.0, 0.0),
            Limb::Head => Vec3::new(0.0, 50.0, 0.0),
        },
        LabanState::Arabesque => match limb {
            Limb::LeftArm => Vec3::new(-80.0, 50.0, 0.0),
            Limb::RightArm => Vec3::new(80.0, 20.0, 0.0),
            Limb::LeftLeg => Vec3::new(0.0, -100.0, 0.0),
            Limb::RightLeg => Vec3::new(80.0, 0.0, 0.0),
            Limb::Head => Vec3::new(0.0, 60.0, 0.0),
        },
        LabanState::Jump => match limb {
            Limb::LeftArm => Vec3::new(-50.0, 80.0, 0.0),
            Limb::RightArm => Vec3::new(50.0, 80.0, 0.0),
            Limb::LeftLeg => Vec3::new(-20.0, -40.0, 0.0),
            Limb::RightLeg => Vec3::new(20.0, -40.0, 0.0),
            Limb::Head => Vec3::new(0.0, 100.0, 0.0),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use bevy::transform::TransformPlugin;

    #[test]
    fn test_choreographer_moves_target() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(TransformPlugin);
        app.add_plugins(ChoreographerPlugin);

        let target = app.world_mut().spawn(TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO))).id();
        let _root = app.world_mut().spawn(TransformBundle::default()).id();
        let joint = app.world_mut().spawn(TransformBundle::default()).id();
        let effector = app.world_mut().spawn(TransformBundle::default()).id();

        app.world_mut().spawn((
            IKChain {
                joints: vec![joint],
                effector,
                target,
                iterations: 1,
            },
            Limb::RightArm
        ));

        // Set state to ReachHigh
        if let Some(mut c) = app.world_mut().get_resource_mut::<Choreographer>() {
            c.current_state = LabanState::ReachHigh;
            c.next_state = LabanState::ReachHigh; // No interpolation needed
        }

        app.update();

        let target_tf = app.world().get::<Transform>(target).unwrap();

        // Assert it moved from ZERO towards ReachHigh RightArm target (50, 100, 0)
        assert!(target_tf.translation != Vec3::ZERO, "Target should have moved from ZERO");
        assert!(target_tf.translation.distance(Vec3::new(50.0, 100.0, 0.0)) < 1.0, "Target should be at ReachHigh position");
    }
}
