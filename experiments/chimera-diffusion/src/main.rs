use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::value::Value;
use chimera_lang::vm::ChimeraVM;
use gray_scott::GrayScott;
use macroquad::prelude::*;

#[allow(dead_code)]
struct Agent {
    pos: Vec2,
    vel: Vec2,
    dna: Dna,
    vm: ChimeraVM,
    health: f32,
}

impl Agent {
    fn new(x: f32, y: f32, dna: Dna) -> Self {
        let mut vm = ChimeraVM::new(dna.clone());
        // Initialize stack with something so it doesn't immediately fail
        vm.stack.push(Value::Int(1));
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(
                macroquad::rand::gen_range(-1.0, 1.0),
                macroquad::rand::gen_range(-1.0, 1.0),
            )
            .normalize(),
            dna,
            vm,
            health: 100.0,
        }
    }

    fn update(&mut self, width: usize, height: usize, gs: &mut GrayScott) {
        // Sense the chemical environment
        let grid_x = self.pos.x.clamp(0.0, width as f32 - 1.0) as usize;
        let grid_y = self.pos.y.clamp(0.0, height as f32 - 1.0) as usize;
        let idx = gs.get_index(grid_x, grid_y);

        let u = gs.u()[idx];
        let v = gs.v()[idx];

        // Push sensory data
        self.vm.stack.push(Value::Int((u * 100.0) as i64));
        self.vm.stack.push(Value::Int((v * 100.0) as i64));

        // Run a few steps of the VM
        for _ in 0..5 {
            let _ = self.vm.step();
        }

        // Output: use stack top to decide action
        let output = match self.vm.stack.pop().unwrap_or(Value::Int(0)) {
            Value::Int(v) => v,
            _ => 0,
        } % 4;
        let mut force = Vec2::ZERO;

        match output {
            0 => force.x += 1.0,
            1 => force.x -= 1.0,
            2 => force.y += 1.0,
            3 => force.y -= 1.0,
            _ => {}
        }

        self.vel = (self.vel + force * 0.1).normalize();
        self.pos += self.vel * 2.0;

        // Wrap around
        if self.pos.x < 0.0 {
            self.pos.x += width as f32;
        }
        if self.pos.x >= width as f32 {
            self.pos.x -= width as f32;
        }
        if self.pos.y < 0.0 {
            self.pos.y += height as f32;
        }
        if self.pos.y >= height as f32 {
            self.pos.y -= height as f32;
        }

        // Interaction: V kills, U heals
        let grid_x = self.pos.x.clamp(0.0, width as f32 - 1.0) as usize;
        let grid_y = self.pos.y.clamp(0.0, height as f32 - 1.0) as usize;
        let idx = gs.get_index(grid_x, grid_y);

        // Agent modifies the grid! It secretes V chemical to defend itself if V is low,
        // or consumes U to live. Let's make it a "V-farmer".
        // If agent is healthy, it excretes V
        if self.health > 50.0 {
            gs.v_mut()[idx] = (gs.v()[idx] + 0.1).min(1.0);
            self.health -= 0.5; // Costs health to secrete
        } else {
            // Eat U to regain health
            let u_val = gs.u()[idx];
            if u_val > 0.1 {
                gs.u_mut()[idx] = u_val - 0.1;
                self.health += 1.0;
            }
        }

        // V is toxic in high amounts
        if gs.v()[idx] > 0.8 {
            self.health -= 1.0;
        }

        self.health = self.health.clamp(0.0, 100.0);
    }
}

fn generate_random_dna() -> Dna {
    let mut genes = Vec::new();
    for _ in 0..10 {
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(macroquad::rand::gen_range(1, 10) as i64)],
        });
        genes.push(Gene {
            op: if macroquad::rand::gen_range(0, 2) == 0 {
                OpCode::Add
            } else {
                OpCode::Sub
            },
            args: vec![],
        });
    }
    // Loop
    genes.push(Gene {
        op: OpCode::Jump,
        args: vec![Nucleotide::Number(0)],
    });

    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[macroquad::main("Chimera Diffusion")]
async fn main() {
    let width = 200;
    let height = 200;

    // Scale up rendering
    let scale = 4.0;

    request_new_screen_size(width as f32 * scale, height as f32 * scale);

    let mut gs = GrayScott::new(width, height);

    // Initial chemical seed
    for i in (width / 2 - 10)..(width / 2 + 10) {
        for j in (height / 2 - 10)..(height / 2 + 10) {
            let idx = gs.get_index(i, j);
            gs.v_mut()[idx] = 1.0;
        }
    }

    let mut agents = Vec::new();
    for _ in 0..50 {
        agents.push(Agent::new(
            macroquad::rand::gen_range(0.0, width as f32),
            macroquad::rand::gen_range(0.0, height as f32),
            generate_random_dna(),
        ));
    }

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    let feed = 0.055;
    let kill = 0.062;
    let dt = 1.0;

    loop {
        // Step simulation
        for _ in 0..10 {
            gs.update(feed, kill, dt);
        }

        // Update agents
        for agent in &mut agents {
            agent.update(width, height, &mut gs);
        }

        // Replace dead agents
        let width_f = width as f32;
        let height_f = height as f32;
        agents.retain(|a| a.health > 0.0);
        while agents.len() < 50 {
            agents.push(Agent::new(
                macroquad::rand::gen_range(0.0, width_f),
                macroquad::rand::gen_range(0.0, height_f),
                generate_random_dna(),
            ));
        }

        // Draw chemicals
        for y in 0..height {
            for x in 0..width {
                let idx = gs.get_index(x, y);
                let u = gs.u()[idx];
                let v = gs.v()[idx];

                // Map U to blue, V to red/orange
                let r = (v * 255.0).clamp(0.0, 255.0) as u8;
                let g = (v * 150.0).clamp(0.0, 255.0) as u8;
                let b = (u * 255.0).clamp(0.0, 255.0) as u8;

                image.set_pixel(x as u32, y as u32, Color::from_rgba(r, g, b, 255));
            }
        }

        texture.update(&image);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(width as f32 * scale, height as f32 * scale)),
                ..Default::default()
            },
        );

        // Draw agents
        for agent in &agents {
            draw_circle(
                agent.pos.x * scale,
                agent.pos.y * scale,
                3.0,
                if agent.health > 50.0 { GREEN } else { YELLOW },
            );
        }

        next_frame().await
    }
}
