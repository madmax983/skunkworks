use crate::fluid::FluidSim;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AgentState {
    Seeking,
    Waiting,       // Waiting for lock (Contention)
    Holding(f32),  // Holding lock (Execution)
    Cooldown(f32), // Cooling down after release
}

pub struct Agent {
    pub id: usize,
    pub pos: Vec2,
    pub state: AgentState,
    pub target_lock_id: Option<u8>,
}

pub struct Lock {
    pub id: u8,
    pub pos: IVec2,
    pub owner: Option<usize>, // Agent ID
}

pub struct LockManager {
    pub agents: Vec<Agent>,
    pub locks: Vec<Lock>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            locks: Vec::new(),
        }
    }

    pub fn add_lock(&mut self, id: u8, x: usize, y: usize) {
        self.locks.push(Lock {
            id,
            pos: IVec2::new(x as i32, y as i32),
            owner: None,
        });
    }

    pub fn spawn_agent(&mut self, x: f32, y: f32) {
        let id = self.agents.len();
        self.agents.push(Agent {
            id,
            pos: vec2(x, y),
            state: AgentState::Cooldown(rand::gen_range(0.0f32, 2.0f32)),
            target_lock_id: None,
        });
    }

    pub fn update(&mut self, fluid: &mut FluidSim, dt: f32) {
        // Disjoint borrows
        let agents = &mut self.agents;
        let locks = &mut self.locks;

        for agent in agents.iter_mut() {
            // Apply boundary constraints
            agent.pos.x = agent.pos.x.clamp(0.0, fluid.width as f32 * 8.0);
            agent.pos.y = agent.pos.y.clamp(0.0, fluid.height as f32 * 8.0);

            match agent.state {
                AgentState::Cooldown(mut time) => {
                    time -= dt;
                    if time <= 0.0 {
                        // Pick a random lock target
                        if !locks.is_empty() {
                            let lock_idx = rand::gen_range(0, locks.len());
                            agent.target_lock_id = Some(locks[lock_idx].id);
                            agent.state = AgentState::Seeking;
                        } else {
                            agent.state = AgentState::Cooldown(1.0);
                        }
                    } else {
                        agent.state = AgentState::Cooldown(time);
                    }

                    // Wander
                    agent.pos +=
                        vec2(rand::gen_range(-10.0f32, 10.0f32), rand::gen_range(-10.0f32, 10.0f32)) * dt;
                }

                AgentState::Seeking => {
                    if let Some(target_id) = agent.target_lock_id {
                        if let Some(lock) = locks.iter().find(|l| l.id == target_id) {
                            let target_pos =
                                vec2(lock.pos.x as f32 * 8.0 + 4.0, lock.pos.y as f32 * 8.0 - 4.0);
                            let dir = target_pos - agent.pos;
                            let dist = dir.length();

                            if dist < 4.0 {
                                // Arrived!
                                // Check if lock is free RIGHT NOW
                                // Actually, we transition to Waiting to simulate checking.
                                // In Waiting state, we try to acquire.
                                agent.state = AgentState::Waiting;
                            } else {
                                agent.pos += dir.normalize_or_zero() * 100.0 * dt;
                            }
                        }
                    }
                }

                AgentState::Waiting => {
                    // Contention! Generate massive heat.
                    let cx = (agent.pos.x / 8.0) as usize;
                    let cy = (agent.pos.y / 8.0) as usize;
                    fluid.add_heat(cx, cy, 50.0 * dt);

                    // Try to acquire lock
                    if let Some(target_id) = agent.target_lock_id {
                        // We need mutable access to locks here
                        if let Some(lock) = locks.iter_mut().find(|l| l.id == target_id) {
                            if lock.owner.is_none() {
                                // Acquire!
                                lock.owner = Some(agent.id);
                                agent.state = AgentState::Holding(rand::gen_range(1.0f32, 3.0f32));
                            } else if lock.owner == Some(agent.id) {
                                agent.state = AgentState::Holding(1.0);
                            }
                            // If owner is someone else, we stay Waiting (and generating heat).
                        }
                    }
                }

                AgentState::Holding(mut time) => {
                    time -= dt;
                    // Execution! Generate moderate heat.
                    let cx = (agent.pos.x / 8.0) as usize;
                    let cy = (agent.pos.y / 8.0) as usize;
                    fluid.add_heat(cx, cy, 10.0 * dt);

                    if time <= 0.0 {
                        // Release lock
                        if let Some(target_id) = agent.target_lock_id {
                            if let Some(lock) = locks.iter_mut().find(|l| l.id == target_id) {
                                if lock.owner == Some(agent.id) {
                                    lock.owner = None;
                                }
                            }
                        }
                        agent.state = AgentState::Cooldown(rand::gen_range(2.0f32, 5.0f32));
                        agent.target_lock_id = None;
                    } else {
                        agent.state = AgentState::Holding(time);
                    }
                }
            }
        }
    }
}
