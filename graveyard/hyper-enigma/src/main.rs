use hyper_system::math::{Vec3, Vec4};
use macroquad::prelude::*;
use std::collections::VecDeque;

// Helper to convert hyper_system Vec3 to macroquad Vec3
fn to_mq(v: Vec3) -> macroquad::math::Vec3 {
    macroquad::math::vec3(v.x, v.y, v.z)
}

pub fn rotate_xy(v: &Vec4, theta: f32) -> Vec4 {
    let (sin, cos) = theta.sin_cos();
    Vec4 {
        x: v.x * cos - v.y * sin,
        y: v.x * sin + v.y * cos,
        z: v.z,
        w: v.w,
    }
}

pub fn rotate_xz(v: &Vec4, theta: f32) -> Vec4 {
    let (sin, cos) = theta.sin_cos();
    Vec4 {
        x: v.x * cos - v.z * sin,
        y: v.y,
        z: v.x * sin + v.z * cos,
        w: v.w,
    }
}

pub fn rotate_yz(v: &Vec4, theta: f32) -> Vec4 {
    let (sin, cos) = theta.sin_cos();
    Vec4 {
        x: v.x,
        y: v.y * cos - v.z * sin,
        z: v.y * sin + v.z * cos,
        w: v.w,
    }
}

fn generate_tesseract_base() -> (Vec<Vec4>, Vec<(usize, usize)>) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { 1.0 } else { -1.0 };
        let y = if i & 2 != 0 { 1.0 } else { -1.0 };
        let z = if i & 4 != 0 { 1.0 } else { -1.0 };
        let w = if i & 8 != 0 { 1.0 } else { -1.0 };
        verts.push(Vec4::new(x, y, z, w));
    }

    let mut edges = Vec::new();
    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                edges.push((i, j));
            }
        }
    }
    (verts, edges)
}

struct EnigmaState {
    // 6 Planes of rotation: XY, XZ, XW, YZ, YW, ZW
    rotors: [f32; 6],
    // Steps for each rotor (The "Machine Setting")
    steps: [f32; 6],
    history: VecDeque<(char, char)>, // (Input, Output)
}

impl EnigmaState {
    fn new() -> Self {
        Self {
            rotors: [0.0; 6],
            steps: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6], // Irregular steps for chaos
            history: VecDeque::new(),
        }
    }

    fn encrypt(&mut self, c: char) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }

        let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
        let idx = (c as u8 - base) as i32;

        // Step the rotors
        for i in 0..6 {
            self.rotors[i] += self.steps[i];
        }

        // Calculate shift based on hyper-rotational state
        // Use a chaotic mix of sines
        let mut shift = 0.0;
        shift += self.rotors[0].sin();
        shift += self.rotors[1].cos();
        shift += self.rotors[2].sin() * 2.0;
        shift += self.rotors[3].cos() * 3.0;
        shift += self.rotors[4].sin() * 5.0;
        shift += self.rotors[5].cos() * 7.0;

        // Map to 0-25 integer
        let shift_int = (shift.abs() * 100.0) as i32 % 26;

        // Encrypt
        let out_idx = (idx + shift_int) % 26;
        let out_char = (base + out_idx as u8) as char;

        self.history.push_back((c, out_char));
        if self.history.len() > 10 {
            self.history.pop_front();
        }

        out_char
    }
}

#[macroquad::main("Hyper Enigma")]
async fn main() {
    let (base_verts, edges) = generate_tesseract_base();
    let mut state = EnigmaState::new();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 6.0f32;

    loop {
        let dt = get_frame_time();

        // Input
        if let Some(c) = get_char_pressed() {
            if c.is_ascii_alphabetic() {
                state.encrypt(c);
            }
        }

        // Camera Control
        if is_key_down(KeyCode::Left) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 2.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 2.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 5.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0 * dt;
        }

        let cam_pos = macroquad::math::vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: macroquad::math::vec3(0.0, 0.0, 0.0),
            up: macroquad::math::vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Transform Vertices based on current Rotor State
        let transform = |v: Vec4| -> macroquad::math::Vec3 {
            let mut v = v;
            // Apply 6-plane rotation
            // 0: XY, 1: XZ, 2: XW, 3: YZ, 4: YW, 5: ZW
            v = rotate_xy(&v, state.rotors[0]);
            v = rotate_xz(&v, state.rotors[1]);
            v = v.rotate_xw(state.rotors[2]);
            v = rotate_yz(&v, state.rotors[3]);
            v = v.rotate_yw(state.rotors[4]);
            v = v.rotate_zw(state.rotors[5]);

            to_mq(v.project_to_3d(3.0))
        };

        // Draw Edges
        for &(i, j) in &edges {
            let v1 = base_verts[i];
            let v2 = base_verts[j];
            let p1 = transform(v1);
            let p2 = transform(v2);

            // Color shifts based on encryption state
            let r = (state.rotors[0].sin() + 1.0) * 0.5;
            let g = (state.rotors[1].cos() + 1.0) * 0.5;
            let b = (state.rotors[2].sin() + 1.0) * 0.5;

            draw_line_3d(p1, p2, Color::new(r, g, b, 0.8));
        }

        // Draw Vertices
        for v in &base_verts {
            let p = transform(*v);
            draw_sphere(p, 0.1, None, WHITE);
        }

        set_default_camera();

        // UI
        draw_text("HYPER ENIGMA", 20.0, 30.0, 40.0, WHITE);
        draw_text("Type to Encrypt", 20.0, 60.0, 20.0, GRAY);

        let mut y = 100.0;
        for (din, dout) in &state.history {
            draw_text(&format!("{} -> {}", din, dout), 20.0, y, 30.0, GREEN);
            y += 30.0;
        }

        // Visualize Rotors
        let rx = 500.0;
        let mut ry = 30.0;
        for i in 0..6 {
            draw_text(
                &format!("Rotor {}: {:.2}", i, state.rotors[i]),
                rx,
                ry,
                20.0,
                WHITE,
            );
            ry += 25.0;
        }

        next_frame().await
    }
}
