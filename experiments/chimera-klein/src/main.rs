use chimera_lang::prelude::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

const MAX_AGENTS: usize = 32;

/// Parametric equation for a Figure-8 Klein Bottle
/// u in [0, 2PI], v in [0, 2PI]
fn parametric_klein(u: f32, v: f32) -> Vec3 {
    let r = 2.5;
    let cos_u = u.cos();
    let sin_u = u.sin();
    let cos_u2 = (u / 2.0).cos();
    let sin_u2 = (u / 2.0).sin();
    let sin_v = v.sin();
    let sin_2v = (2.0 * v).sin();

    // Figure-8 Immersion logic
    let term = r + cos_u2 * sin_v - sin_u2 * sin_2v;

    let x = term * cos_u;
    let y = term * sin_u;
    let z = sin_u2 * sin_v + cos_u2 * sin_2v;

    // Rotate to make it look nicer (Vertical Figure-8)
    vec3(x, z, y)
}

struct Agent {
    vm: ChimeraVM,
    pos: Vec2, // (u, v) in [0, 2PI]
    vel: Vec2,
    handedness: bool, // true = Right, false = Left (flipped)
    color: Color,
}

impl Agent {
    fn new() -> Self {
        // DNA: Random movement using Migrate
        // We push dy, dx then Migrate
        // Since Migrate pops dx (top) then dy (next)

        let genes = vec![
            // Move North (dy=-1, dx=0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Migrate, args: vec![] },

            // Move East (dy=0, dx=1)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Migrate, args: vec![] },

            // Move South (dy=1, dx=0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Migrate, args: vec![] },

            // Move West (dy=0, dx=-1)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] },
            Gene { op: OpCode::Migrate, args: vec![] },

            // Loop
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        let u = macroquad::rand::gen_range(0.0, 2.0 * PI);
        let v = macroquad::rand::gen_range(0.0, 2.0 * PI);

        Self {
            vm,
            pos: vec2(u, v),
            vel: vec2(macroquad::rand::gen_range(-0.01, 0.01), macroquad::rand::gen_range(-0.01, 0.01)),
            handedness: true,
            color: GREEN,
        }
    }

    fn update(&mut self) {
        if self.vm.energy > 0 {
            // Track Grid Position Change
            let old_loc = self.vm.context_loc;

            // Execute VM step
            self.vm.step();

            let new_loc = self.vm.context_loc;

            // Calculate Grid Movement
            let dy = (new_loc.0 as isize - old_loc.0 as isize) as f32;
            let dx = (new_loc.1 as isize - old_loc.1 as isize) as f32;

            // Influence Velocity (VM drives movement)
            if dy != 0.0 || dx != 0.0 {
                self.vel.x += dx * 0.005;
                self.vel.y += dy * 0.005;
            } else {
                // Brownian noise if no grid movement
                self.vel.x += macroquad::rand::gen_range(-0.0005, 0.0005);
                self.vel.y += macroquad::rand::gen_range(-0.0005, 0.0005);
            }

            // Decay velocity
            self.vel *= 0.99;

            // Apply velocity to position
            self.pos += self.vel;

            // Topological Wrap & Twist
            let two_pi = 2.0 * PI;

            // U-axis wrap (Tube cross-section)
            if self.pos.x < 0.0 {
                self.pos.x += two_pi;
            } else if self.pos.x >= two_pi {
                self.pos.x -= two_pi;
            }

            // V-axis wrap (Mobius Loop)
            // If we cross the V boundary, we flip U and Handedness
            if self.pos.y < 0.0 {
                self.pos.y += two_pi;
                // Twist!
                self.apply_twist(two_pi);
            } else if self.pos.y >= two_pi {
                self.pos.y -= two_pi;
                // Twist!
                self.apply_twist(two_pi);
            }

            // Update color
            self.color = if self.handedness { GREEN } else { ORANGE };
        } else {
            self.color = GRAY;
        }
    }

    fn apply_twist(&mut self, two_pi: f32) {
        // Flip U coordinate: u' = 2PI - u
        self.pos.x = two_pi - self.pos.x;

        // Flip U velocity (cross-section direction reverses relative to the surface normal)
        self.vel.x = -self.vel.x;

        // Flip Handedness
        self.handedness = !self.handedness;
    }
}

#[macroquad::main("Chimera Klein")]
async fn main() {
    let mut agents: Vec<Agent> = (0..MAX_AGENTS).map(|_| Agent::new()).collect();

    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 10.0f32;

    loop {
        // Update
        for agent in &mut agents {
            agent.update();
        }

        // Camera Input
        if is_key_down(KeyCode::Left) { cam_angle_y += 0.02; }
        if is_key_down(KeyCode::Right) { cam_angle_y -= 0.02; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 0.02; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 0.02; }
        if is_key_down(KeyCode::W) { cam_dist -= 0.1; }
        if is_key_down(KeyCode::S) { cam_dist += 0.1; }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        clear_background(BLACK);

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Klein Bottle Mesh (Wireframe-ish)
        let steps = 40;
        let two_pi = 2.0 * PI;
        for i in 0..steps {
            for j in 0..steps {
                let u = (i as f32 / steps as f32) * two_pi;
                let v = (j as f32 / steps as f32) * two_pi;
                let p = parametric_klein(u, v);

                // Draw points or small lines
                draw_cube(p, vec3(0.02, 0.02, 0.02), None, DARKGRAY);
            }
        }

        // Draw Agents
        for agent in &agents {
            let p = parametric_klein(agent.pos.x, agent.pos.y);
            // Draw sphere at position
            draw_sphere(p, 0.1, None, agent.color);
        }

        set_default_camera();

        draw_text("Chimera Klein", 10.0, 20.0, 30.0, WHITE);
        draw_text("Green = Right Handed | Orange = Left Handed", 10.0, 50.0, 20.0, GRAY);
        draw_text(&format!("Agents: {}", agents.len()), 10.0, 70.0, 20.0, WHITE);

        next_frame().await
    }
}
