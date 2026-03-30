mod heap;
use heap::Heap;

use ::rand::Rng;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use macroquad::prelude::*;

const AGENT_COUNT: usize = 20;

struct Agent {
    vm: ChimeraVM,
    pos: Vec2,
    vel: Vec2,
    energy: f32,
    color: Color,
    target: Option<usize>, // ID of the node being hunted
}

impl Agent {
    fn new(x: f32, y: f32) -> Self {
        let dna = generate_random_dna(16);
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100;

        Self {
            vm,
            pos: vec2(x, y),
            vel: vec2(0.0, 0.0),
            energy: 100.0,
            color: Color::new(1.0, 0.0, 1.0, 0.8), // Purple foragers
            target: None,
        }
    }

    fn update(&mut self, heap: &mut Heap, reachable_nodes: &[usize]) {
        // Execute a step of their genetic code
        let _ = self.vm.step();

        // Interpret VM state as movement/behavior
        let speed = 2.0;

        // Pop values from stack to use as velocity, if available
        let mut vx = 0.0;
        let mut vy = 0.0;
        if let Some(chimera_lang::vm::Value::Int(v)) = self.vm.stack.pop() {
            vx = (v as f32 % 10.0 - 5.0) / 5.0;
        }
        if let Some(chimera_lang::vm::Value::Int(v)) = self.vm.stack.pop() {
            vy = (v as f32 % 10.0 - 5.0) / 5.0;
        }

        // They try to find a target if they don't have one
        if self.target.is_none() || !heap.nodes.contains_key(&self.target.unwrap()) {
            let mut closest_dist = f32::MAX;
            let mut best_target = None;
            for (id, node) in &heap.nodes {
                if !node.alive {
                    continue;
                }
                let dist = self.pos.distance(node.pos);
                if dist < closest_dist {
                    closest_dist = dist;
                    best_target = Some(*id);
                }
            }
            self.target = best_target;
        }

        // Move towards target or wander based on VM output
        if let Some(target_id) = self.target {
            if let Some(target_node) = heap.nodes.get(&target_id) {
                let dir = (target_node.pos - self.pos).normalize_or_zero();
                // Blend wandering VM output with target seeking
                self.vel = (dir * speed * 0.8) + (vec2(vx, vy) * speed * 0.2);
            }
        } else {
            self.vel = vec2(vx, vy) * speed;
        }

        self.pos += self.vel;
        self.energy -= 0.1;

        // Interaction with heap nodes
        if let Some(target_id) = self.target {
            if let Some(target_node) = heap.nodes.get_mut(&target_id) {
                if self.pos.distance(target_node.pos) < target_node.size {
                    // Consume it
                    target_node.alive = false;

                    // If it was reachable (live), take damage! Bad reaper!
                    if reachable_nodes.contains(&target_id) {
                        self.energy -= 50.0;
                        self.color = RED;
                    } else {
                        // Dead matter is food! Good reaper!
                        self.energy += 30.0;
                        self.color = GREEN;
                    }

                    self.target = None;
                }
            }
        }

        // Keep on screen
        self.pos.x = self.pos.x.clamp(0.0, screen_width());
        self.pos.y = self.pos.y.clamp(0.0, screen_height());
    }
}

fn generate_random_dna(length: usize) -> Dna {
    let mut rng = ::rand::thread_rng();
    let mut genes = Vec::new();
    let opcodes = [
        OpCode::Push,
        OpCode::Add,
        OpCode::Sub,
        OpCode::Mul,
        OpCode::Jump,
        OpCode::GRead,
        OpCode::GWrite,
        OpCode::Dup,
        OpCode::Swap,
        OpCode::Nop,
    ];

    for _ in 0..length {
        let op = opcodes[rng.gen_range(0..opcodes.len())].clone();
        genes.push(Gene { op, args: vec![Nucleotide::Number(rng.gen_range(0..10))] });
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Chimera Reaper")]
async fn main() {
    let mut heap = Heap::new();
    let mut agents = Vec::new();
    let mut rng = ::rand::thread_rng();

    for _ in 0..AGENT_COUNT {
        agents.push(Agent::new(
            rng.gen_range(0.0..800.0),
            rng.gen_range(0.0..600.0),
        ));
    }

    let mut frame = 0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        // Randomly allocate memory
        if frame % 30 == 0 {
            let root = heap.roots[rng.gen_range(0..heap.roots.len())];
            if let Some(root_node) = heap.nodes.get(&root) {
                let pos = root_node.pos;
                let id = heap.allocate(pos);
                heap.add_reference(root, id);
            }
        }

        // Randomly drop references (simulating dead objects)
        if frame % 100 == 0 && heap.nodes.len() > 5 {
            let ids: Vec<usize> = heap.nodes.keys().copied().collect();
            let parent_id = ids[rng.gen_range(0..ids.len())];
            if let Some(parent) = heap.nodes.get(&parent_id) {
                if !parent.children.is_empty() {
                    let child_id = parent.children[0];
                    heap.remove_reference(parent_id, child_id);
                }
            }
        }

        heap.update();

        let reachable = heap.determine_reachability();

        // Update agents
        for agent in &mut agents {
            agent.update(&mut heap, &reachable);
        }

        // Reproduce / Die
        let mut new_agents = Vec::new();
        agents.retain(|agent| agent.energy > 0.0);
        for agent in &agents {
            if agent.energy > 150.0 && rng.gen_bool(0.05) {
                let mut child = Agent::new(agent.pos.x, agent.pos.y);
                child.vm = ChimeraVM::new(agent.vm.dna.clone());
                // Simple mutation
                if !child.vm.dna.helix.strands.is_empty() && !child.vm.dna.helix.strands[0].genes.is_empty() {
                    let mut genes = child.vm.dna.helix.strands[0].genes.clone();
                    let idx = rng.gen_range(0..genes.len());
                    genes[idx].args = vec![Nucleotide::Number(rng.gen_range(0..10))];
                    child.vm.dna.helix.strands[0].genes = genes;
                }
                new_agents.push(child);
            }
        }
        agents.extend(new_agents);

        // Draw heap
        for node in heap.nodes.values() {
            if !node.alive {
                continue;
            }
            let is_reachable = reachable.contains(&node.id);
            let is_root = heap.roots.contains(&node.id);

            let color = if is_root {
                GREEN
            } else if is_reachable {
                WHITE
            } else {
                GRAY // Orphaned memory
            };

            for &child_id in &node.children {
                if let Some(child) = heap.nodes.get(&child_id) {
                    if child.alive {
                        draw_line(node.pos.x, node.pos.y, child.pos.x, child.pos.y, 2.0, color);
                    }
                }
            }

            draw_circle(node.pos.x, node.pos.y, node.size, color);
        }

        // Draw agents
        for agent in &agents {
            draw_circle(agent.pos.x, agent.pos.y, 5.0, agent.color);
        }

        frame += 1;
        next_frame().await;
    }
}
