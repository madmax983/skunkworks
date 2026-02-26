//! 🧬 Experiment: chimera-enigma
//!
//! # Lineage
//! - **Parent A**: `experiments/hyper-enigma` (4D Hypercube Cryptography)
//! - **Parent B**: `experiments/chimera-lang` (Evolutionary Virtual Machine)
//!
//! # Novel Trait: Evolutionary Cryptography (Bio-Encryption)
//! The encryption key is not static; it is the living state of a population of ChimeraVM agents
//! inhabiting a 4D Hypercube.
//!
//! - **The Machine**: A Tesseract where each of the 16 vertices holds a "Rotor" value.
//! - **The Key**: The sum of all Rotor values at any given instant.
//! - **The Operators**: ChimeraVM agents ("Codebreakers" or "Keepers") that crawl along the edges
//!   of the hypercube. When they reach a vertex, they execute their genetic code to modify the
//!   local Rotor value.
//!
//! The encryption algorithm itself evolves as the agents mutate. To decrypt a message, one would
//! need the exact biological state of the simulation at the moment of encryption—effectively
//! making the key time-dependent and biologically unique.

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::value::Value;
use chimera_lang::vm::ChimeraVM;
use hyper_system::math::{Vec3, Vec4};
use macroquad::prelude::*;
use std::collections::VecDeque;

// Helper to convert hyper_system Vec3 to macroquad Vec3
fn to_mq(v: Vec3) -> macroquad::math::Vec3 {
    macroquad::math::vec3(v.x, v.y, v.z)
}

trait Vec4Ext {
    fn rotate_xy(&self, theta: f32) -> Vec4;
    fn rotate_xz(&self, theta: f32) -> Vec4;
    fn rotate_xw(&self, theta: f32) -> Vec4;
    fn rotate_yz(&self, theta: f32) -> Vec4;
    fn rotate_yw(&self, theta: f32) -> Vec4;
    fn rotate_zw(&self, theta: f32) -> Vec4;
}

impl Vec4Ext for Vec4 {
    fn rotate_xy(&self, theta: f32) -> Vec4 {
        let (sin, cos) = theta.sin_cos();
        Vec4 {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
            w: self.w,
        }
    }

    fn rotate_xz(&self, theta: f32) -> Vec4 {
        let (sin, cos) = theta.sin_cos();
        Vec4 {
            x: self.x * cos - self.z * sin,
            y: self.y,
            z: self.x * sin + self.z * cos,
            w: self.w,
        }
    }

    fn rotate_xw(&self, theta: f32) -> Vec4 {
        let (sin, cos) = theta.sin_cos();
        Vec4 {
            x: self.x * cos - self.w * sin,
            y: self.y,
            z: self.z,
            w: self.x * sin + self.w * cos,
        }
    }

    fn rotate_yz(&self, theta: f32) -> Vec4 {
        let (sin, cos) = theta.sin_cos();
        Vec4 {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
            w: self.w,
        }
    }

    fn rotate_yw(&self, theta: f32) -> Vec4 {
        let (sin, cos) = theta.sin_cos();
        Vec4 {
            x: self.x,
            y: self.y * cos - self.w * sin,
            z: self.z,
            w: self.y * sin + self.w * cos,
        }
    }

    fn rotate_zw(&self, theta: f32) -> Vec4 {
        let (sin, cos) = theta.sin_cos();
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z * cos - self.w * sin,
            w: self.z * sin + self.w * cos,
        }
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

struct Agent {
    vm: ChimeraVM,
    position: Vec4,
    target_idx: usize,
    speed: f32,
    color: Color,
}

impl Agent {
    fn new(start_idx: usize, vertices: &[Vec4]) -> Self {
        // Create random DNA
        let mut genes = Vec::new();
        // A simple genome that tries to produce a number
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rand::gen_range(0, 100))] });
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rand::gen_range(0, 100))] });
        genes.push(Gene { op: OpCode::Add, args: vec![] }); // Sum
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] });
        genes.push(Gene { op: OpCode::Mod, args: vec![] }); // Mod 10
        genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }); // Loop

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        Self {
            vm: ChimeraVM::new(dna),
            position: vertices[start_idx],
            target_idx: start_idx,
            speed: rand::gen_range(0.5, 2.0),
            color: Color::from_rgba(rand::gen_range(100, 255), rand::gen_range(100, 255), rand::gen_range(100, 255), 255),
        }
    }

    fn update(&mut self, dt: f32, vertices: &[Vec4], neighbors: &[Vec<usize>], rotors: &mut [f32]) {
        let target_pos = vertices[self.target_idx];
        let diff = target_pos - self.position;
        let dist = diff.length();

        if dist < 0.1 {
            // Arrived at vertex
            self.position = target_pos;

            // Execute VM
            self.vm.step();

            // Interact with Rotor
            if let Some(val) = self.vm.stack.last() {
                if let Value::Int(n) = val {
                    // Modulate rotor based on output
                    // Small change based on gene output
                    let delta = (*n as f32) * 0.01;
                    rotors[self.target_idx] += delta;
                }
            }

            // Pick new target (neighbor)
            if let Some(adj) = neighbors.get(self.target_idx) {
                if !adj.is_empty() {
                    let idx = rand::gen_range(0, adj.len());
                    self.target_idx = adj[idx];
                }
            }
        } else {
            // Move
            let dir = diff.scale(1.0 / dist);
            self.position = self.position + dir.scale(self.speed * dt);
        }
    }
}

struct EnigmaHypercube {
    rotors: [f32; 16], // One rotor per vertex
    history: VecDeque<(char, char)>,
}

impl EnigmaHypercube {
    fn new() -> Self {
        Self {
            rotors: [0.0; 16],
            history: VecDeque::new(),
        }
    }

    fn encrypt(&mut self, c: char) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }

        let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
        let idx = (c as u8 - base) as i32;

        // Calculate shift from Bio-State
        let mut total_shift = 0.0;
        for r in &self.rotors {
            total_shift += *r;
        }

        let shift_int = (total_shift.abs() as i32) % 26;
        let out_idx = (idx + shift_int) % 26;
        let out_char = (base + out_idx as u8) as char;

        self.history.push_back((c, out_char));
        if self.history.len() > 10 {
            self.history.pop_front();
        }

        out_char
    }
}

#[macroquad::main("Chimera Enigma")]
async fn main() {
    let (base_verts, edges) = generate_tesseract_base();

    // Build adjacency list for agents
    let mut neighbors = vec![Vec::new(); 16];
    for &(i, j) in &edges {
        neighbors[i].push(j);
        neighbors[j].push(i);
    }

    let mut enigma = EnigmaHypercube::new();
    let mut agents = Vec::new();

    // Spawn agents
    for _ in 0..8 {
        let start = rand::gen_range(0, 16);
        agents.push(Agent::new(start, &base_verts));
    }

    // Camera vars
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 5.0f32;

    loop {
        let dt = get_frame_time();

        // Input
        if let Some(c) = get_char_pressed() {
            enigma.encrypt(c);
        }

        // Camera Control
        if is_key_down(KeyCode::Left) { cam_angle_y += 2.0 * dt; }
        if is_key_down(KeyCode::Right) { cam_angle_y -= 2.0 * dt; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 2.0 * dt; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 2.0 * dt; }
        if is_key_down(KeyCode::W) { cam_dist -= 5.0 * dt; }
        if is_key_down(KeyCode::S) { cam_dist += 5.0 * dt; }

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

        // Update Agents
        for agent in &mut agents {
            agent.update(dt, &base_verts, &neighbors, &mut enigma.rotors);
        }

        // Transform Vertices based on system load? Or just rotate constantly?
        // Let's rotate based on total rotor energy to show "activity"
        let total_energy: f32 = enigma.rotors.iter().sum();
        let rotation_speed = total_energy * 0.01;

        let transform = |v: Vec4| -> macroquad::math::Vec3 {
            let mut v = v;
            // Rotate in 4D
            v = v.rotate_xw(rotation_speed * get_time() as f32);
            v = v.rotate_yw(rotation_speed * get_time() as f32 * 0.5);
            to_mq(v.project_to_3d(3.0))
        };

        // Draw Edges
        for &(i, j) in &edges {
            let v1 = base_verts[i];
            let v2 = base_verts[j];
            let p1 = transform(v1);
            let p2 = transform(v2);

            let rotor_i = enigma.rotors[i];
            let rotor_j = enigma.rotors[j];
            let intensity = ((rotor_i + rotor_j).abs() * 0.1).clamp(0.2, 1.0);

            draw_line_3d(p1, p2, Color::new(0.0, intensity, 0.0, 0.5));
        }

        // Draw Vertices
        for (i, v) in base_verts.iter().enumerate() {
            let p = transform(*v);
            let r_val = enigma.rotors[i];
            let size = 0.05 + (r_val.abs() * 0.01).clamp(0.0, 0.2);
            draw_sphere(p, size, None, WHITE);
        }

        // Draw Agents
        for agent in &agents {
            let p = transform(agent.position);
            draw_sphere(p, 0.08, None, agent.color);
        }

        set_default_camera();

        // UI
        draw_text("CHIMERA ENIGMA", 20.0, 30.0, 40.0, WHITE);
        draw_text("Bio-Encryption Active", 20.0, 60.0, 20.0, GREEN);
        draw_text(&format!("Total Key Energy: {:.2}", total_energy), 20.0, 80.0, 20.0, GRAY);

        let mut y = 120.0;
        for (din, dout) in &enigma.history {
            draw_text(&format!("{} -> {}", din, dout), 20.0, y, 30.0, GREEN);
            y += 30.0;
        }

        next_frame().await
    }
}
