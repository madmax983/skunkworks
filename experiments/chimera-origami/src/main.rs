use macroquad::prelude::*;
use physics_pbd::PbdSystem;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use chimera_lang::vm::ChimeraVM;
use chimera_lang::ast::{Dna, Gene};
use chimera_lang::opcode::OpCode;

struct MeshAgent {
    system: PbdSystem,
    mesh_indices: Vec<u16>,
    actuator_indices: Vec<usize>,
    brain: ChimeraVM,
}

impl MeshAgent {
    pub fn new() -> Self {
        let grid_size = (10, 10);
        let params = MiuraParams {
            a: 2.0,
            b: 2.0,
            gamma: 80.0f32.to_radians(),
            orientation: Orientation::Horizontal,
        };

        // Create initial fully expanded grid
        let vertices_pos = generate_miura_grid(params, grid_size, 1.0);

        // --- Lineage: origami (Parent B) ---
        let mut system = PbdSystem::new();
        for pos in &vertices_pos {
            system.add_particle(*pos, 1.0);
        }

        // Add structural constraints and actuators
        let (cols, rows) = grid_size;
        let v_cols = cols + 1;
        let mut actuator_indices = Vec::new();

        for j in 0..rows {
            for i in 0..cols {
                let p00 = (j * v_cols + i) as usize;
                let p10 = (j * v_cols + (i + 1)) as usize;
                let p01 = ((j + 1) * v_cols + i) as usize;
                let p11 = ((j + 1) * v_cols + (i + 1)) as usize;

                // Edges
                system.add_distance_constraint(p00, p10, 0.5);
                system.add_distance_constraint(p00, p01, 0.5);
                system.add_distance_constraint(p10, p11, 0.5);
                system.add_distance_constraint(p01, p11, 0.5);

                // Diagonals (Cross struts for stability)
                system.add_distance_constraint(p00, p11, 0.5);
                system.add_distance_constraint(p10, p01, 0.5);

                // Actuators (Diagonal contraction)
                system.add_actuator_constraint(p00, p11, 1.0, 5.0, 0.5);
                actuator_indices.push(system.constraints.len() - 1);
            }
        }

        let mut mesh_indices = Vec::new();
        for j in 0..rows {
            for i in 0..cols {
                let p00 = (j * v_cols + i) as u16;
                let p10 = (j * v_cols + (i + 1)) as u16;
                let p01 = ((j + 1) * v_cols + i) as u16;
                let p11 = ((j + 1) * v_cols + (i + 1)) as u16;

                mesh_indices.extend_from_slice(&[p00, p10, p01, p10, p11, p01]);
            }
        }

        // --- Lineage: chimera-lang (Parent A) ---
        let default_dna = Dna::from_genes(vec![Gene::from(OpCode::Nop)]);
        let brain = ChimeraVM::new(default_dna);

        Self {
            system,
            mesh_indices,
            actuator_indices,
            brain,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Step the brain
        self.brain.step();

        // --- Lineage: chimera-lang (Parent A) ---
        // Convert the stack output to actuator contractions
        // E.g. we read the top of the stack, or just use brain's energy/state
        let contraction = if self.brain.stack.is_empty() {
            1.0 // Relaxed
        } else {
            0.5 // Contracted
        };

        // --- Lineage: origami (Parent B) ---
        // Smoothly apply to actuators
        for &idx in &self.actuator_indices {
            if let physics_pbd::Constraint::Actuator { factor, .. } = &mut self.system.constraints[idx] {
                *factor += (contraction - *factor) * 0.1;
            }
        }

        self.system.step(dt, 5);
    }

    pub fn draw(&self) {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: self.mesh_indices.clone(),
            texture: None,
        };

        for p in &self.system.particles {
            mesh.vertices.push(Vertex {
                position: p.pos,
                uv: Vec2::ZERO,
                color: Color::new(0.5, 0.8, 0.9, 1.0).into(),
                normal: vec4(0.0, 1.0, 0.0, 1.0),
            });
        }

        draw_mesh(&mesh);
    }
}

#[macroquad::main("Chimera Origami")]
async fn main() {
    let mut mesh_agent = MeshAgent::new();

    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(0.0, -30.0, 30.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        draw_grid(20, 5.0, Color::new(0.2, 0.2, 0.2, 1.0), GRAY);

        mesh_agent.update(0.016);
        mesh_agent.draw();

        set_default_camera();

        draw_text("CHIMERA ORIGAMI", 10.0, 30.0, 30.0, WHITE);

        next_frame().await
    }
}
