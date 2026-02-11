use cgmath::{InnerSpace, MetricSpace, Point3, Vector3, Zero};
use rand::prelude::*;
use rayon::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Solid,
    Liquid,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Point3<f32>,
    pub vel: Vector3<f32>,
    pub lattice_pos: Point3<f32>,
    pub temperature: f32,
    pub state: State,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub pos: Point3<f32>,
    pub vel: Vector3<f32>,
    pub vm: ChimeraVM,
    pub energy: f32,
    pub generation: u32,
    pub last_action: String,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub agents: Vec<Agent>,
    pub grid_size: u32,
    pub spacing: f32,
}

impl Simulation {
    pub fn new(grid_size: u32) -> Self {
        let mut particles = Vec::with_capacity((grid_size * grid_size * grid_size) as usize);
        let spacing = 1.5;
        let offset = (grid_size as f32 * spacing) / 2.0;

        for x in 0..grid_size {
            for y in 0..grid_size {
                for z in 0..grid_size {
                    let lx = (x as f32) * spacing - offset;
                    let ly = (y as f32) * spacing - offset;
                    let lz = (z as f32) * spacing - offset;
                    let pos = Point3::new(lx, ly, lz);

                    particles.push(Particle {
                        pos,
                        vel: Vector3::zero(),
                        lattice_pos: pos,
                        temperature: 0.05, // Start cold
                        state: State::Solid,
                    });
                }
            }
        }

        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        let num_agents = 200; // Reduced from 500 for performance with VM

        for _ in 0..num_agents {
            let x = rng.gen_range(-offset..offset);
            let y = rng.gen_range(-offset..offset);
            let z = rng.gen_range(-offset..offset);

            let dna = generate_random_dna();
            let vm = ChimeraVM::new(dna);

            agents.push(Agent {
                pos: Point3::new(x, y, z),
                vel: Vector3::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize()
                    * 5.0,
                vm,
                energy: 100.0,
                generation: 1,
                last_action: "Born".to_string(),
            });
        }

        Self {
            particles,
            agents,
            grid_size,
            spacing,
        }
    }

    pub fn update(&mut self, dt: f32, global_temp_mod: f32) {
        let spring_k = 10.0; // Spring constant for solids
        let liquid_k = 0.5; // Weak spring for liquids (viscosity/tension)
        let damping = 0.90;

        // Update Agents
        let limit = (self.grid_size as f32 * self.spacing) / 2.0;

        // Find nearest particle temperature for each agent (approximation)
        // Optimization: Just check a random sample or assume ambient + local effects?
        // Let's iterate agents and find nearest particle.
        // Or better, particles affect agents in the particle loop, but agents need inputs BEFORE they act.

        for agent in &mut self.agents {
            // 1. Inputs
            // Find temp at current pos
            // Brute force nearest neighbor for now (slow O(N*M))?
            // Or just use global temp + local variation.
            // Let's simulate local temp as agent's own internal temp or energy.

            // Push sensor data to VM
            agent.vm.stack.push(Value::Int(agent.energy as i64));
            // Push normalized position
            agent.vm.stack.push(Value::Int((agent.pos.x * 10.0) as i64));
            agent.vm.stack.push(Value::Int((agent.pos.y * 10.0) as i64));
            agent.vm.stack.push(Value::Int((agent.pos.z * 10.0) as i64));

            // 2. Step VM
            // Run for 10 ticks or untill halted
            for _ in 0..10 {
                if !agent.vm.halted {
                    agent.vm.step();
                }
            }

            // 3. Outputs
            // Check stack for action command
            // Format: [ActionID, Param]
            // Actions:
            // 0: Move (Param: Direction 0-6)
            // 1: Heat (Cost energy)
            // 2: Cool (Cost energy)
            // 3: Reproduce (Cost high energy)

            let mut action = -1;
            let mut param = 0;

            if let Some(val) = agent.vm.stack.last() {
                if let Value::Int(v) = val {
                     action = *v;
                }
            }

            // Consume action if valid
            if action >= 0 {
                agent.vm.stack.pop(); // Pop action
                if let Some(Value::Int(p)) = agent.vm.stack.pop() {
                    param = p;
                }
            }

            match action {
                0 => { // Move
                    let dir = match param % 6 {
                        0 => Vector3::new(1.0, 0.0, 0.0),
                        1 => Vector3::new(-1.0, 0.0, 0.0),
                        2 => Vector3::new(0.0, 1.0, 0.0),
                        3 => Vector3::new(0.0, -1.0, 0.0),
                        4 => Vector3::new(0.0, 0.0, 1.0),
                        _ => Vector3::new(0.0, 0.0, -1.0),
                    };
                    // Smooth turn
                    agent.vel += dir * dt * 10.0;
                    agent.energy -= 0.1;
                    agent.last_action = "Move".to_string();
                },
                1 => { // Heat
                    // Effect applied in particle loop
                    // Just mark state or verify energy
                    if agent.energy > 5.0 {
                        agent.energy -= 1.0;
                        agent.last_action = "Heat".to_string();
                    } else {
                        action = -1; // Failed
                    }
                },
                2 => { // Cool
                     if agent.energy > 5.0 {
                        agent.energy -= 1.0;
                        agent.last_action = "Cool".to_string();
                     } else {
                        action = -1;
                     }
                },
                3 => { // Reproduce
                    // Handled later
                    agent.last_action = "Reproduce".to_string();
                }
                _ => {
                    agent.last_action = "Idle".to_string();
                }
            }

            // Physics update
            if agent.vel.magnitude() > 10.0 {
                agent.vel = agent.vel.normalize() * 10.0;
            }

            agent.pos += agent.vel * dt;

            // Bounce
            if agent.pos.x.abs() > limit { agent.vel.x *= -1.0; agent.pos.x = agent.pos.x.signum() * limit; }
            if agent.pos.y.abs() > limit { agent.vel.y *= -1.0; agent.pos.y = agent.pos.y.signum() * limit; }
            if agent.pos.z.abs() > limit { agent.vel.z *= -1.0; agent.pos.z = agent.pos.z.signum() * limit; }

            // Starvation
            agent.energy -= dt; // Base metabolism
            if agent.vm.halted {
                agent.vm.ip = (0,0);
                agent.vm.halted = false; // Reset loop
                agent.vm.energy = 50; // Reset VM internal energy
            }
        }

        // Reproduction & Death
        let mut new_agents = Vec::new();
        let mut rng = rand::thread_rng();

        self.agents.retain(|a| a.energy > 0.0);

        for agent in &mut self.agents {
             // Check if action was reproduce
             if agent.last_action == "Reproduce" && agent.energy > 60.0 {
                 agent.energy -= 30.0;
                 let mut child_dna = agent.vm.dna.clone();
                 mutate_dna(&mut child_dna);
                 let mut child_vm = ChimeraVM::new(child_dna);

                 new_agents.push(Agent {
                     pos: agent.pos + Vector3::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)),
                     vel: agent.vel * -0.5,
                     vm: child_vm,
                     energy: 30.0,
                     generation: agent.generation + 1,
                     last_action: "Born".to_string(),
                 });
             }
        }

        self.agents.append(&mut new_agents);

        // Cap population
        if self.agents.len() > 300 {
            // Remove oldest/weakest? Or just random?
            // Remove low energy
            self.agents.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap());
            self.agents.truncate(300);
        }

        if self.agents.len() < 50 {
            // Repopulate if extinction event
             for _ in 0..10 {
                let dna = generate_random_dna();
                let vm = ChimeraVM::new(dna);
                self.agents.push(Agent {
                    pos: Point3::new(0.0, 0.0, 0.0),
                    vel: Vector3::zero(),
                    vm,
                    energy: 50.0,
                    generation: 0,
                    last_action: "Respawns".to_string(),
                });
             }
        }

        // Parallel update for particles
        // Note: agents vector is now potentially resized, but we process particles relative to current agents.
        // We cannot use self.agents inside par_iter because of borrow rules if we mutate self.particles?
        // Actually we can slice it.

        let agents_ref = &self.agents; // Immutable borrow
        let spacing = self.spacing;

        self.particles.par_iter_mut().for_each(|p| {
            let mut rng = rand::thread_rng();

            // Check for nearby agents
            for agent in agents_ref {
                let dist_sq = p.pos.distance2(agent.pos);
                if dist_sq < (spacing * spacing * 4.0) {
                    // Effect based on last action
                    match agent.last_action.as_str() {
                        "Heat" => p.temperature += 10.0 * dt,
                        "Cool" => p.temperature -= 10.0 * dt,
                        _ => {}
                    }

                    // Feeding: if in liquid (high temp), gain energy?
                    // Or if maintaining stable temp?
                    // Let's say:
                    // Liquid provides energy?
                    // Or phase transitions provide energy?
                }
            }

            // Agents gain energy from environment?
            // This needs mutable access to agents, which we can't do in parallel particle loop easily.
            // Simplified: Agents lose energy for actions, gain energy if they are in "Liquid" (simulating finding food/water).
            // But we can't update agent energy here.
            // We'll do it in the agent loop next frame, approximating particle state?
            // Or just do a separate pass.

            // Natural cooling/warming towards global ambient
            let ambient = global_temp_mod;
            p.temperature += (ambient - p.temperature) * dt * 0.5;
            p.temperature = p.temperature.max(0.0);

            // Phase transition
            if p.temperature > 1.0 {
                p.state = State::Liquid;
            } else {
                p.state = State::Solid;
            }

            // Dynamics based on state
            let k = match p.state {
                State::Solid => spring_k,
                State::Liquid => liquid_k,
            };

            // Force towards lattice position
            let displacement = p.pos - p.lattice_pos;
            let spring_force = -k * displacement;

            // Thermal noise (Random walk)
            let noise_mag = p.temperature * 5.0;
            let random_force = Vector3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ) * noise_mag;

            let force = spring_force + random_force;

            // Euler integration
            p.vel += force * dt;
            p.vel *= damping;
            p.pos += p.vel * dt;
        });

        // Post-particle update: Agents gain energy from liquid
        // We need spatial query.
        // Naive O(N*M) again?
        // Let's just assume agents in liquid gain energy.
        // But we don't know particle state at agent pos without searching.
        // We can approximate: if global temp high? No.
        // Let's skip detailed environment feeding for now, or just give baseline energy.
        // Or: agents gain energy if they successfully Heat/Cool (doing work on environment).
        // Let's say: "Thermo-synthesis". If they Heat a cold particle -> Gain. If they Cool a hot particle -> Gain.
        // Implemented simplified: if last_action was Heat/Cool, small chance to gain energy.
         for agent in &mut self.agents {
             if agent.last_action == "Heat" || agent.last_action == "Cool" {
                 // Assume they found a gradient
                 agent.energy += 1.5; // Net gain (spent 1.0, gained 1.5)
             }
         }
    }
}

fn generate_random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let mut genes = Vec::new();
    let ops = [
        OpCode::Push, OpCode::Dup, OpCode::Swap, OpCode::Add, OpCode::Sub,
        OpCode::Jump, OpCode::Brz, OpCode::Drop
    ];

    for _ in 0..20 {
        let op = ops[rng.gen_range(0..ops.len())].clone();
        let args = match op {
            OpCode::Push => vec![Nucleotide::Number(rng.gen_range(0..5))], // Small numbers for actions
            OpCode::Jump | OpCode::Brz => vec![Nucleotide::Number(rng.gen_range(0..5))], // Jump back/forward small amount
            _ => vec![],
        };
        genes.push(Gene { op, args });
    }

    // Ensure loop at end
    genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] });

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

fn mutate_dna(dna: &mut Dna) {
    let mut rng = rand::thread_rng();
    if let Some(strand) = dna.helix.strands.get_mut(0) {
        if !strand.genes.is_empty() {
             let idx = rng.gen_range(0..strand.genes.len());
             // Simple mutation: change arg or op
             if rng.gen_bool(0.5) && !strand.genes[idx].args.is_empty() {
                 strand.genes[idx].args[0] = Nucleotide::Number(rng.gen_range(0..10));
             } else {
                 // Change op (simplified)
                 let ops = [OpCode::Push, OpCode::Dup, OpCode::Swap, OpCode::Add, OpCode::Sub];
                 strand.genes[idx].op = ops[rng.gen_range(0..ops.len())].clone();
                 if strand.genes[idx].op == OpCode::Push {
                     strand.genes[idx].args = vec![Nucleotide::Number(rng.gen_range(0..5))];
                 } else {
                     strand.genes[idx].args.clear();
                 }
             }
        }
    }
}
