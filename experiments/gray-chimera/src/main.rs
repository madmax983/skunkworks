use chimera_lang::ast::{Dna, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};
use gray_scott::GrayScott;
use locus::Vec2;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;
const AGENT_COUNT: usize = 500;
const AGENT_SPEED: f32 = 40.0;
const CELL_SIZE: f32 = 4.0;

struct Agent {
    pos: Vec2,
    vel: Vec2,
    dna: Dna,
    vm: ChimeraVM,
    health: f32,
    color: Color,
}

impl Agent {
    fn new(pos: Vec2, dna: Dna) -> Self {
        let vm = ChimeraVM::new(dna.clone());
        Self {
            pos,
            vel: Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize(),
            dna,
            vm,
            health: 1.0,
            color: GREEN,
        }
    }

    fn update(&mut self, dt: f32, gs: &mut GrayScott) {
        if self.health <= 0.0 {
            return;
        }

        // Clamp to grid
        let x_idx = (self.pos.x as f64 / CELL_SIZE as f64).clamp(0.0, (GRID_WIDTH - 1) as f64) as usize;
        let y_idx = (self.pos.y as f64 / CELL_SIZE as f64).clamp(0.0, (GRID_HEIGHT - 1) as f64) as usize;

        let gs_idx = gs.get_index(x_idx, y_idx);

        // Read U and V concentrations
        let u_val = gs.u()[gs_idx];
        let v_val = gs.v()[gs_idx];

        // The agents are poisoned by the 'V' kill chemical
        if v_val > 0.4 {
            self.health -= 0.1 * dt;
            self.color = RED;
        } else if u_val > 0.8 {
            // They feed on 'U'
            self.health = (self.health + 0.05 * dt).min(1.0);
            self.color = GREEN;
        } else {
            self.color = YELLOW;
        }

        // Push sensory data onto VM stack (scaled to i64)
        self.vm.stack.push(Value::Int((u_val * 1000.0) as i64));
        self.vm.stack.push(Value::Int((v_val * 1000.0) as i64));

        // Step VM
        self.vm.step();

        // Pop steering intent
        let mut steer_x = self.vel.x;
        let mut steer_y = self.vel.y;

        if let Some(Value::Int(i)) = self.vm.stack.pop() {
            steer_y = i as f64 / 1000.0;
        }
        if let Some(Value::Int(i)) = self.vm.stack.pop() {
            steer_x = i as f64 / 1000.0;
        }

        let steer = Vec2::new(steer_x, steer_y);
        if steer.magnitude_squared() > 0.01 {
            self.vel = (self.vel + steer.normalize() * 0.1).normalize();
        }

        // Move
        self.pos += self.vel * (AGENT_SPEED as f64) * (dt as f64);

        // Wrap around screen
        let w = GRID_WIDTH as f64 * CELL_SIZE as f64;
        let h = GRID_HEIGHT as f64 * CELL_SIZE as f64;
        self.pos.x = self.pos.x.rem_euclid(w);
        self.pos.y = self.pos.y.rem_euclid(h);

        // Deposit a bit of 'V' chemical where they traverse, accelerating reaction
        gs.add_chemical(x_idx, y_idx, 0.05 * dt);
    }
}

fn random_dna() -> Dna {
    let mut genes = Vec::new();
    for _ in 0..10 {
        genes.push(Gene::new(
            match rand::gen_range(0, 4) {
                0 => OpCode::Add,
                1 => OpCode::Sub,
                2 => OpCode::Mul,
                _ => OpCode::Push,
            },
            vec![Nucleotide::from(rand::gen_range(0, 16) as i64)],
        ));
    }
    Dna::from_genes(genes)
}

fn mutate_dna(mut dna: Dna) -> Dna {
    if rand::gen_range(0.0, 1.0) < 0.1 {
        if !dna.helix.strands.is_empty() {
            let strand_idx = rand::gen_range(0, dna.helix.strands.len());
            let strand = &mut dna.helix.strands[strand_idx];
            if !strand.genes.is_empty() {
                let gene_idx = rand::gen_range(0, strand.genes.len());
                strand.genes[gene_idx] = Gene::new(
                    match rand::gen_range(0, 4) {
                        0 => OpCode::Add,
                        1 => OpCode::Sub,
                        2 => OpCode::Mul,
                        _ => OpCode::Push,
                    },
                    vec![Nucleotide::from(rand::gen_range(0, 16) as i64)],
                );
            }
        }
    }
    dna
}

#[macroquad::main("gray-chimera")]
async fn main() {
    let mut gs = GrayScott::new(GRID_WIDTH, GRID_HEIGHT);

    // Initial chemical seed
    for y in (GRID_HEIGHT / 2 - 10)..(GRID_HEIGHT / 2 + 10) {
        for x in (GRID_WIDTH / 2 - 10)..(GRID_WIDTH / 2 + 10) {
            gs.add_chemical(x, y, 1.0);
        }
    }

    let mut agents: Vec<Agent> = (0..AGENT_COUNT)
        .map(|_| {
            Agent::new(
                Vec2::new(
                    rand::gen_range(0.0, (GRID_WIDTH as f64) * (CELL_SIZE as f64)),
                    rand::gen_range(0.0, (GRID_HEIGHT as f64) * (CELL_SIZE as f64)),
                ),
                random_dna(),
            )
        })
        .collect();

    let gs_texture = Texture2D::empty();
    gs_texture.set_filter(FilterMode::Nearest);

    let mut pixels = vec![0u8; GRID_WIDTH * GRID_HEIGHT * 4];

    loop {
        clear_background(BLACK);
        let dt = get_frame_time().min(0.05);

        // Advance Gray-Scott (Coral/Spots pattern)
        let f = 0.055;
        let k = 0.062;
        gs.update(f, k, 1.0); // 1 timestep per frame

        // Draw Gray-Scott
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = gs.get_index(x, y);
                let u = gs.u()[idx];
                let v = gs.v()[idx];

                let r = (v * 255.0).clamp(0.0, 255.0) as u8;
                let b = (u * 255.0).clamp(0.0, 255.0) as u8;

                let px_idx = (y * GRID_WIDTH + x) * 4;
                pixels[px_idx] = r;
                pixels[px_idx + 1] = 0;
                pixels[px_idx + 2] = b;
                pixels[px_idx + 3] = 255;
            }
        }

        let image = Image {
            width: GRID_WIDTH as u16,
            height: GRID_HEIGHT as u16,
            bytes: pixels.clone(),
        };

        gs_texture.update(&image);

        draw_texture_ex(
            &gs_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    GRID_WIDTH as f32 * CELL_SIZE,
                    GRID_HEIGHT as f32 * CELL_SIZE,
                )),
                ..Default::default()
            },
        );

        // Update and draw agents
        for agent in &mut agents {
            agent.update(dt, &mut gs);
            if agent.health > 0.0 {
                draw_circle(agent.pos.x as f32, agent.pos.y as f32, 2.0, agent.color);
            }
        }

        // Reproduction/Mutation for dead agents (Evolutionary loop)
        let mut new_dnas = Vec::new();
        let mut num_dead = 0;

        for agent in &agents {
            if agent.health <= 0.0 {
                num_dead += 1;
            }
        }

        if num_dead > 0 {
            // Find a healthy parent
            if let Some(parent) = agents.iter().find(|a| a.health > 0.5) {
                for _ in 0..num_dead {
                    // Slight mutation
                    let child_dna = parent.dna.clone();
                    new_dnas.push(mutate_dna(child_dna));
                }
            } else {
                 for _ in 0..num_dead {
                     new_dnas.push(random_dna());
                 }
            }

            // Re-spawn
            let mut dna_iter = new_dnas.into_iter();
            for agent in &mut agents {
                if agent.health <= 0.0 {
                    if let Some(dna) = dna_iter.next() {
                         *agent = Agent::new(
                            Vec2::new(
                                rand::gen_range(0.0, (GRID_WIDTH as f64) * (CELL_SIZE as f64)),
                                rand::gen_range(0.0, (GRID_HEIGHT as f64) * (CELL_SIZE as f64)),
                            ),
                            dna,
                        );
                    }
                }
            }
        }

        draw_text("Morphogenetic Genetics (gray-chimera)", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 30.0, WHITE);

        next_frame().await;
    }
}
