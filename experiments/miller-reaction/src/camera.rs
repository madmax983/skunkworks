use cgmath::*;
use winit::event::*;
use winit::keyboard::{KeyCode, PhysicalKey};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

pub struct CameraController {
    speed: f32,
    rotation_speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_rotate_left_pressed: bool,
    is_rotate_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32, rotation_speed: f32) -> Self {
        Self {
            speed,
            rotation_speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
            is_rotate_left_pressed: false,
            is_rotate_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &KeyEvent) -> bool {
        let is_pressed = event.state == ElementState::Pressed;
        match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.is_forward_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.is_left_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.is_backward_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.is_right_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::Space) => {
                self.is_up_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::ShiftLeft) => {
                self.is_down_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyQ) => {
                self.is_rotate_left_pressed = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyE) => {
                self.is_rotate_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        let forward_norm = if forward_mag > 0.0 {
            forward / forward_mag
        } else {
            Vector3::unit_z()
        };
        let right_norm = forward_norm.cross(camera.up).normalize();

        // Movement
        let mut delta = Vector3::zero();
        if self.is_forward_pressed {
            delta += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            delta -= forward_norm * self.speed;
        }
        if self.is_right_pressed {
            delta += right_norm * self.speed;
        }
        if self.is_left_pressed {
            delta -= right_norm * self.speed;
        }
        if self.is_up_pressed {
            delta += camera.up * self.speed;
        }
        if self.is_down_pressed {
            delta -= camera.up * self.speed;
        }

        // Apply movement to both eye and target to keep direction
        camera.eye += delta;
        camera.target += delta;

        // Rotation (Yaw)
        let mut rotation_angle = Rad(0.0);
        if self.is_rotate_left_pressed {
            rotation_angle += Rad(self.rotation_speed);
        }
        if self.is_rotate_right_pressed {
            rotation_angle -= Rad(self.rotation_speed);
        }

        if rotation_angle.0 != 0.0 {
            // Rotate forward vector around up axis
            let rotation = Quaternion::from_axis_angle(camera.up, rotation_angle);
            let new_forward = rotation.rotate_vector(forward_norm);
            camera.target = camera.eye + new_forward * forward_mag;
        }
    }
}
