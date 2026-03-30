use macroquad::prelude::*;

mod graph;
use graph::Graph;

use chimera_lang::ast::{Dna, Gene};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use chimera_lang::vm::Value;

struct Agent {
    vm: ChimeraVM,
    current_node: usize,
    x: f32,
    y: f32,
}

const AGENT_COUNT: usize = 100;

fn create_random_dna() -> Dna {
    let mut genes = Vec::new();
    for _ in 0..10 {
        let r = rand::gen_range(0, 4);
        let op = match r {
            0 => OpCode::Push, // Just push something
            1 => OpCode::Drop,
            2 => OpCode::Add,
            _ => OpCode::Nop,
        };
        // Just empty args for simple ops
        genes.push(Gene::new(op, vec![]));
    }
    Dna::from_genes(genes)
}

#[macroquad::main("Mnemonic Chimera")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Scan the codebase to build the graph
    graph.scan_directory("experiments/mnem-chimera/src");
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    if graph.nodes.is_empty() {
        panic!("No nodes found to build the graph.");
    }

    let mut agents = Vec::new();

    for _ in 0..AGENT_COUNT {
        let node_id = rand::gen_range(0, graph.nodes.len());
        let pos = graph.nodes[node_id].pos;

        let dna = create_random_dna();
        let vm = ChimeraVM::new(dna);

        agents.push(Agent {
            vm,
            current_node: node_id,
            x: pos.x,
            y: pos.y,
        });
    }

    let mut entropy_timer = 0.0;

    // Camera state
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    loop {
        let dt = get_frame_time();

        // --- Controls ---
        let mouse = vec2(mouse_position().0, mouse_position().1);
        if is_mouse_button_pressed(MouseButton::Left) {
            dragging = true;
            last_mouse = mouse;
        }
        if is_mouse_button_released(MouseButton::Left) {
            dragging = false;
        }
        if dragging {
            let delta = mouse - last_mouse;
            offset += delta / zoom;
            last_mouse = mouse;
        }
        let scroll = mouse_wheel().1;
        if scroll > 0.0 {
            zoom *= 1.1;
        } else if scroll < 0.0 {
            zoom /= 1.1;
        }

        // --- Graph Update ---
        entropy_timer += dt;
        if entropy_timer > 0.1 {
            entropy_timer = 0.0;
            // Decay health based on randomness
            for node in &mut graph.nodes {
                if rand::gen_range(0.0, 1.0) < 0.05 {
                    node.health = (node.health - 0.02).max(0.0); // Minimum health is 0.0
                }
            }
        }

        // --- Agent Update ---
        for i in (0..agents.len()).rev() {
            let agent = &mut agents[i];

            // Read environment: Push node health to stack
            let node_health = graph.nodes[agent.current_node].health;
            agent.vm.stack.push(Value::Int((node_health * 255.0) as i64));

            // Read environment: Push edges count
            let edges: Vec<_> = graph.edges.iter().filter(|e| e.from == agent.current_node || e.to == agent.current_node).collect();
            agent.vm.stack.push(Value::Int(edges.len() as i64));

            // Execute VM
            agent.vm.step();

            if node_health > 0.5 {
                // healthy node heals agent
                agent.vm.energy += 5;
            } else {
                // rotting node hurts agent, but agent heals node
                agent.vm.energy -= 2;
                graph.nodes[agent.current_node].health = (node_health + 0.005).min(1.0);
            }

            // Interpret top of stack as a move command if it's an Int
            if let Some(Value::Int(val)) = agent.vm.stack.last() {
                if *val > 0 && !edges.is_empty() {
                    let e = edges[rand::gen_range(0, edges.len())];
                    agent.current_node = if e.from == agent.current_node { e.to } else { e.from };
                    agent.vm.stack.pop(); // consume the value
                }
            }

            // Lerp physical position to logical node position
            let target_pos = graph.nodes[agent.current_node].pos;
            let current_pos = vec2(agent.x, agent.y);
            let new_pos = current_pos.lerp(target_pos, 0.1);
            agent.x = new_pos.x;
            agent.y = new_pos.y;

            if agent.vm.halted || agent.vm.energy <= 0 {
                 // Dead
                 agents.remove(i);
            }
        }

        // Repopulate if all dead
        if agents.is_empty() {
            for _ in 0..AGENT_COUNT {
                let node_id = rand::gen_range(0, graph.nodes.len());
                let pos = graph.nodes[node_id].pos;
                let dna = create_random_dna();
                let vm = ChimeraVM::new(dna);
                agents.push(Agent {
                    vm,
                    current_node: node_id,
                    x: pos.x,
                    y: pos.y,
                });
            }
        }

        // --- Render ---
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let cam_transform = |p: Vec2| -> Vec2 { (p + offset) * zoom + vec2(screen_width() / 2.0, screen_height() / 2.0) };

        // Draw edges
        for edge in &graph.edges {
            let p1 = cam_transform(graph.nodes[edge.from].pos);
            let p2 = cam_transform(graph.nodes[edge.to].pos);
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, Color::new(0.2, 0.2, 0.2, 0.5));
        }

        // Draw nodes
        for node in &graph.nodes {
            let p = cam_transform(node.pos);
            // Color based on health: Green -> Red
            let c = Color::new(1.0 - node.health, node.health, 0.1, 1.0);
            draw_circle(p.x, p.y, 4.0 * zoom, c);
        }

        // Draw agents
        for agent in &agents {
            let p = cam_transform(vec2(agent.x, agent.y));
            // Energy dictates brightness
            let alpha = (agent.vm.energy as f32 / 100.0).clamp(0.1, 1.0);
            draw_circle(p.x, p.y, 3.0 * zoom, Color::new(0.5, 0.5, 1.0, alpha));
        }

        // UI
        draw_text(
            &format!("Agents: {} | Zoom: {:.2}", agents.len(), zoom),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
