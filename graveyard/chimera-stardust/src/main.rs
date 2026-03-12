use ::rand::prelude::*;
use chimera_lang::prelude::*;
use image::RgbaImage;
use macroquad::prelude::*;
use stardust_compiler::stego;

const GRID_WIDTH: u32 = 800;
const GRID_HEIGHT: u32 = 600;

struct Agent {
    vm: ChimeraVM,
    x: u32,
    y: u32,
    color: Color,
}

impl Agent {
    fn new(x: u32, y: u32) -> Self {
        let mut rng = ::rand::thread_rng();

        // Genes:
        // 1. Read Sensor (Red at [8][8], Green at [8][9], Blue at [8][10])
        // 2. Mutate color slightly (add random value)
        // 3. Write back to [1][0], [1][1], [1][2]
        // 4. Move randomly (write to [0][0], [0][1])

        let mut genes = Vec::new();

        // --- READ RED ---
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)],
        }); // X
        genes.push(Gene {
            op: OpCode::GRead,
            args: vec![],
        }); // Stack: [Red]

        // Modify Red (Add 10)
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        });
        genes.push(Gene {
            op: OpCode::Add,
            args: vec![],
        });

        // Write Red ([1][0])
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // X
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        });

        // --- MOVE ---
        // DX (Random -1..1) - simulated by just pushing a number for now,
        // normally we'd want logic but hardcoding movement into genes makes them "drift" in specific directions
        let dx: i64 = rng.gen_range(-1..=1);
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(dx)],
        });
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // X
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        });

        let dy: i64 = rng.gen_range(-1..=1);
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(dy)],
        });
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }); // X
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        });

        // Loop
        genes.push(Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        });

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.chaos_mode = true; // Allow mutations

        Self {
            vm,
            x,
            y,
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
        }
    }

    fn update(&mut self, img: &mut RgbaImage) {
        // 1. Sense Environment (Pixel Color)
        let pixel = img.get_pixel(self.x, self.y);
        let r = pixel[0] as i64;
        let g = pixel[1] as i64;
        let b = pixel[2] as i64;

        // Inject into VM
        self.vm.grid[8][8] = Value::Int(r);
        self.vm.grid[8][9] = Value::Int(g);
        self.vm.grid[8][10] = Value::Int(b);

        // 2. Execute DNA
        for _ in 0..10 {
            self.vm.step();
            if self.vm.halted {
                break;
            }
        }

        // 3. Read Actuators
        // Movement
        let dx = match self.vm.grid[0][0] {
            Value::Int(n) => n.clamp(-1, 1),
            _ => 0,
        };
        let dy = match self.vm.grid[0][1] {
            Value::Int(n) => n.clamp(-1, 1),
            _ => 0,
        };

        // Color Modification
        let mut new_r = r;
        if let Value::Int(v) = self.vm.grid[1][0] {
            new_r = v.clamp(0, 255);
        }

        let mut new_g = g;
        if let Value::Int(v) = self.vm.grid[1][1] {
            new_g = v.clamp(0, 255);
        }

        let mut new_b = b;
        if let Value::Int(v) = self.vm.grid[1][2] {
            new_b = v.clamp(0, 255);
        }

        // Apply changes to environment
        img.put_pixel(
            self.x,
            self.y,
            image::Rgba([new_r as u8, new_g as u8, new_b as u8, 255]),
        );

        // Move
        if dx != 0 || dy != 0 {
            let new_x = (self.x as i64 + dx).rem_euclid(GRID_WIDTH as i64) as u32;
            let new_y = (self.y as i64 + dy).rem_euclid(GRID_HEIGHT as i64) as u32;
            self.x = new_x;
            self.y = new_y;
        }

        // Random Mutation (Simulated via drift for now to ensure activity)
        let mut rng = ::rand::thread_rng();
        if rng.gen_bool(0.05) {
            let mx = rng.gen_range(-1..=1);
            let my = rng.gen_range(-1..=1);
            let new_x = (self.x as i64 + mx).rem_euclid(GRID_WIDTH as i64) as u32;
            let new_y = (self.y as i64 + my).rem_euclid(GRID_HEIGHT as i64) as u32;
            self.x = new_x;
            self.y = new_y;
        }
    }
}

#[macroquad::main("Chimera Stardust")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    // 1. Generate Nebula (Environment)
    // Using a random seed for unique nebula each run
    let mut nebula_img = stego::generate_nebula(GRID_WIDTH, GRID_HEIGHT, rng.gen());

    // 2. Spawn Agents (ChimeraVMs)
    let mut agents = Vec::new();
    for _ in 0..100 {
        let x = rng.gen_range(0..GRID_WIDTH);
        let y = rng.gen_range(0..GRID_HEIGHT);
        agents.push(Agent::new(x, y));
    }

    loop {
        // Update Agents
        for agent in &mut agents {
            agent.update(&mut nebula_img);
        }

        // Convert Image to Texture for Rendering
        // Note: Creating a new texture every frame is expensive, but for 800x600 it might be okay for a demo.
        // Optimization: Update an existing texture.
        // Macroquad's Texture2D::from_image creates a new OpenGL texture.
        // A better way is using `update` if available, or just re-creating.
        // Let's re-create for simplicity first.
        let texture = Texture2D::from_image(&macroquad::texture::Image {
            width: nebula_img.width() as u16,
            height: nebula_img.height() as u16,
            bytes: nebula_img.as_raw().clone(),
        });

        clear_background(BLACK);

        // Draw Nebula
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Draw Agents as bright sparks
        for agent in &agents {
            draw_circle(agent.x as f32, agent.y as f32, 2.0, agent.color);
        }

        // UI
        draw_text("Chimera Stardust", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", agents.len()),
            20.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Agents are reading/writing pixels as DNA IO",
            20.0,
            90.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
