use ::rand::Rng;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::value::Value;
use chimera_lang::vm::ChimeraVM;
use macroquad::prelude::*;

mod erosion;
mod phonology;

use crate::erosion::{hydraulic_erosion, thermal_weathering};
use crate::phonology::{char_to_terrain, terrain_to_char, TerrainPoint};

const TERRAIN_SCALE_X: f32 = 15.0;
const TERRAIN_SCALE_Y: f32 = 150.0;
const BASELINE_Y: f32 = 400.0;
const AGENT_COUNT: usize = 50;

struct PolyglotAgent {
    vm: ChimeraVM,
    x: usize,
    color: Color,
    last_action: String,
}

impl PolyglotAgent {
    fn new(x: usize, dna: Dna) -> Self {
        let mut rng = ::rand::thread_rng();
        let color = Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0);
        Self {
            vm: ChimeraVM::new(dna),
            x,
            color,
            last_action: String::new(),
        }
    }
}

#[macroquad::main("Polyglot Chimera")]
async fn main() {
    let mut input_text = String::from("CHIMERA");
    let mut terrain: Vec<TerrainPoint> = text_to_terrain(&input_text);
    let mut agents: Vec<PolyglotAgent> = Vec::new();

    // Spawn initial population
    spawn_agents(&mut agents, terrain.len());

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Dark space

        // --- Input Handling ---
        let char_pressed = get_char_pressed();
        if let Some(c) = char_pressed {
            if c.is_alphabetic() || c == ' ' {
                input_text.push(c);
                terrain = text_to_terrain(&input_text); // Reset terrain on edit
                spawn_agents(&mut agents, terrain.len()); // Respawn agents
            }
        }

        if is_key_pressed(KeyCode::Back) && !input_text.is_empty() {
            input_text.pop();
            terrain = text_to_terrain(&input_text);
            spawn_agents(&mut agents, terrain.len());
        }

        if is_key_pressed(KeyCode::R) {
            spawn_agents(&mut agents, terrain.len());
        }

        // --- Simulation ---

        // Erosion (Natural)
        if is_key_down(KeyCode::E) {
            hydraulic_erosion(&mut terrain);
            thermal_weathering(&mut terrain);
        }

        // Agent Logic
        for agent in agents.iter_mut() {
            // Ensure agent is on terrain
            if agent.x >= terrain.len() {
                agent.x = terrain.len() - 1;
            }

            // 1. Sense Environment
            let point = terrain[agent.x];
            // Push Height (scaled 0-100) and Hardness (scaled 0-100)
            agent
                .vm
                .stack
                .push(Value::Int((point.hardness * 100.0) as i64));
            agent
                .vm
                .stack
                .push(Value::Int((point.height * 100.0) as i64));

            // 2. Step VM
            agent.vm.step();

            // 3. Act based on Stack Output
            // Expected: [ ... Command ]
            // If stack is empty, do nothing
            if let Some(val) = agent.vm.stack.pop() {
                if let Value::Int(cmd) = val {
                    match cmd {
                        1 => {
                            // Move Left
                            if agent.x > 0 {
                                agent.x -= 1;
                                agent.last_action = "L".to_string();
                            }
                        }
                        2 => {
                            // Move Right
                            if agent.x < terrain.len() - 1 {
                                agent.x += 1;
                                agent.last_action = "R".to_string();
                            }
                        }
                        3 => {
                            // Dig / Eat
                            // Reduce height based on hardness
                            let erosion = 0.05 * (1.0 - terrain[agent.x].hardness * 0.5);
                            terrain[agent.x].height = (terrain[agent.x].height - erosion).max(0.0);
                            agent.vm.energy += 5; // Gain energy
                            agent.last_action = "Eat".to_string();
                        }
                        _ => {
                            agent.last_action = "?".to_string();
                        }
                    }
                }
            }

            // Refill Energy if low (solar power?) to keep them alive for demo
            if agent.vm.energy < 10 {
                agent.vm.energy = 50;
            }

            // Random movement jitter if stuck (mutation/drift)
            let mut rng = ::rand::thread_rng();
            if rng.gen_bool(0.05) {
                if rng.gen_bool(0.5) && agent.x > 0 {
                    agent.x -= 1;
                } else if agent.x < terrain.len() - 1 {
                    agent.x += 1;
                }
            }
        }

        // --- Drawing ---

        // Draw Terrain
        draw_terrain(&terrain, BASELINE_Y, GREEN);

        // Draw Text Labels (Reconstruction)
        let reconstructed = reconstruct_text(&terrain);
        draw_text(
            &format!("Current Landscape Reads: {}", reconstructed),
            20.0,
            90.0,
            20.0,
            GOLD,
        );

        // Draw Agents
        for agent in &agents {
            let tx = agent.x as f32 * TERRAIN_SCALE_X + 50.0;
            let ty = BASELINE_Y - terrain[agent.x].height * TERRAIN_SCALE_Y;

            draw_circle(tx, ty - 5.0, 3.0, agent.color);
            // draw_text(&agent.last_action, tx, ty - 10.0, 10.0, WHITE);
        }

        // UI
        draw_text(
            "Polyglot Chimera: Agents eating Language",
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Input: {}", input_text),
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Agents: {}", agents.len()),
            20.0,
            120.0,
            20.0,
            WHITE,
        );

        draw_text(
            "Controls: Type to change terrain | E: Natural Erosion | R: Respawn Agents",
            20.0,
            screen_height() - 30.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}

fn spawn_agents(agents: &mut Vec<PolyglotAgent>, terrain_len: usize) {
    agents.clear();
    let mut rng = ::rand::thread_rng();

    // Simple Grazer DNA
    // Logic:
    // Stack has [Hardness, Height] (Height is top)
    // Dup -> [Hardness, Height, Height]
    // Push(20) -> [..., Height, 20] (Threshold)
    // Gt -> [..., 1 or 0]
    // Brz(3) -> Jump over Eat if 0 (Low height)
    // Push(3) -> Command Eat
    // Jump(2) -> Skip Move
    // Label Move:
    // Push(1) or Push(2) randomly?
    // Let's just alternate or random walk using math?
    // For simplicity: Push(1) (Left) if Hardness < 50, Push(2) (Right) else.
    // Or just Random: Push(1) Push(2) ...

    // Let's make a simple "Eater" gene
    let genes = vec![
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // Check Height
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        }, // Threshold
        Gene {
            op: OpCode::Gt,
            args: vec![],
        },
        Gene {
            op: OpCode::Brz,
            args: vec![Nucleotide::Number(3)],
        }, // Skip to Move
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(3)],
        }, // Eat
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(100)],
        }, // End/Restart
        // Move Logic
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // Drop Height
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Move Left
           // Note: Real random walk requires randomness in VM.
           // We rely on external jitter in main loop for true randomness if DNA is static.
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    for _ in 0..AGENT_COUNT {
        let x = rng.gen_range(0..terrain_len);
        agents.push(PolyglotAgent::new(x, dna.clone()));
    }
}

fn text_to_terrain(text: &str) -> Vec<TerrainPoint> {
    let mut terrain = Vec::new();

    // Padding
    for _ in 0..5 {
        terrain.push(TerrainPoint {
            height: 0.0,
            hardness: 0.0,
        });
    }

    for c in text.chars() {
        let center = char_to_terrain(c);

        // Interpolate / Spread the phoneme
        // Left shoulder
        terrain.push(TerrainPoint {
            height: center.height * 0.5,
            hardness: center.hardness * 0.8,
        });

        // Center
        terrain.push(center);

        // Right shoulder
        terrain.push(TerrainPoint {
            height: center.height * 0.5,
            hardness: center.hardness * 0.8,
        });

        // Gap
        terrain.push(TerrainPoint {
            height: 0.1,
            hardness: 0.1,
        });
    }

    // Padding
    for _ in 0..5 {
        terrain.push(TerrainPoint {
            height: 0.0,
            hardness: 0.0,
        });
    }

    terrain
}

fn reconstruct_text(terrain: &Vec<TerrainPoint>) -> String {
    let mut s = String::new();
    let mut i = 5;
    while i < terrain.len().saturating_sub(5) {
        let point = &terrain[i + 1];
        let c = terrain_to_char(point);
        if c != ' ' {
            s.push(c);
        }
        i += 4;
    }
    s
}

fn draw_terrain(terrain: &Vec<TerrainPoint>, base_y: f32, color: Color) {
    for i in 0..terrain.len().saturating_sub(1) {
        let x1 = i as f32 * TERRAIN_SCALE_X + 50.0;
        let y1 = base_y - terrain[i].height * TERRAIN_SCALE_Y;

        let x2 = (i + 1) as f32 * TERRAIN_SCALE_X + 50.0;
        let y2 = base_y - terrain[i + 1].height * TERRAIN_SCALE_Y;

        draw_line(x1, y1, x2, y2, 2.0, color);
        draw_line(
            x1,
            y1,
            x1,
            base_y,
            1.0,
            Color::new(color.r, color.g, color.b, 0.1),
        );
    }
}
