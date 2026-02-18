use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct FlyCam {
    pub speed: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for FlyCam {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: 0.003,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

pub fn fly_camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam::default(),
    ));
}

pub fn fly_camera_system(
    mut query: Query<(&mut Transform, &mut FlyCam)>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    if !window.cursor.visible {
        for (mut transform, mut cam) in query.iter_mut() {
            // Rotation
            for event in mouse_events.read() {
                cam.yaw -= event.delta.x * cam.sensitivity;
                cam.pitch -= event.delta.y * cam.sensitivity;

                // Clamp pitch
                cam.pitch = cam.pitch.clamp(-1.5, 1.5);
            }

            transform.rotation = Quat::from_axis_angle(Vec3::Y, cam.yaw)
                * Quat::from_axis_angle(Vec3::X, cam.pitch);

            // Movement
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0.0, local_z.z).normalize_or_zero();
            let right = Vec3::new(local_z.z, 0.0, -local_z.x).normalize_or_zero();
            let up = Vec3::Y;

            if key_input.pressed(KeyCode::KeyW) {
                velocity += forward;
            }
            if key_input.pressed(KeyCode::KeyS) {
                velocity -= forward;
            }
            if key_input.pressed(KeyCode::KeyD) {
                velocity += right;
            }
            if key_input.pressed(KeyCode::KeyA) {
                velocity -= right;
            }
            if key_input.pressed(KeyCode::Space) {
                velocity += up;
            }
            if key_input.pressed(KeyCode::ShiftLeft) {
                velocity -= up;
            }

            if velocity.length_squared() > 0.0 {
                velocity = velocity.normalize();
                transform.translation += velocity * cam.speed * time.delta_seconds();
            }
        }
    }
}

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
    }
}
