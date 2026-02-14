use macroquad::prelude::*;
use crate::grid::MiuraGrid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AntState {
    Foraging,
    Jumping,
}

#[derive(Clone, Debug)]
pub struct Ant {
    pub u: f32, // Grid coordinate (0.0 .. cols)
    pub v: f32, // Grid coordinate (0.0 .. rows)
    pub target_u: Option<f32>,
    pub target_v: Option<f32>,
    pub state: AntState,
    pub jump_progress: f32, // 0.0 to 1.0
    pub jump_start_pos: Vec3,
    pub jump_end_pos: Vec3,
}

impl Ant {
    pub fn new(u: f32, v: f32) -> Self {
        Self {
            u,
            v,
            target_u: None,
            target_v: None,
            state: AntState::Foraging,
            jump_progress: 0.0,
            jump_start_pos: Vec3::ZERO,
            jump_end_pos: Vec3::ZERO,
        }
    }
}

pub struct Colony {
    pub ants: Vec<Ant>,
    pub pheromones: Vec<f32>, // Flattened grid (cols+1) * (rows+1)? No, faces or vertices? Let's use vertices.
    pub width: usize,
    pub height: usize,
}

impl Colony {
    pub fn new(width: usize, height: usize, count: usize) -> Self {
        let mut ants = Vec::with_capacity(count);
        for _ in 0..count {
            ants.push(Ant::new(
                rand::gen_range(0.0, width as f32),
                rand::gen_range(0.0, height as f32),
            ));
        }

        Self {
            ants,
            pheromones: vec![0.0; (width + 1) * (height + 1)],
            width,
            height,
        }
    }

    pub fn update(&mut self, _grid: &MiuraGrid, vertices: &[Vec3], dt: f32) {
        // Evaporate pheromones
        for p in &mut self.pheromones {
            *p *= 0.99;
        }

        // Precompute spatial index for wormhole detection?
        // Or just check random candidates?
        // Checking all pairs is O(N^2) on vertices, too slow.
        // Checking ant vs all vertices is O(A * V).
        // Let's just check local neighborhood of the ant in 3D?
        // No, we need to find "remote" vertices that are close.

        // Strategy: Ants only jump from vertices.
        // If ant is close to a vertex (integer u,v), check that vertex's 3D neighbors.
        // But we don't know who the 3D neighbors are without a spatial hash.

        // Simplified: Pick a random "wormhole candidate" for each ant?
        // Or: Just let them walk. If they cross a fold line, that's normal.
        // The "wormhole" concept relies on the fold bringing distant parts of the sheet together.
        // Let's implement a naive spatial check for a subset of vertices near the ant?
        // Too complex for this step.
        // Let's make them jump if they are "lucky".

        for ant in &mut self.ants {
            match ant.state {
                AntState::Foraging => {
                    // 1. Move towards target or pick new target
                    if ant.target_u.is_none() {
                        // Pick random neighbor
                        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
                        let dist = 1.0;
                        let tu = (ant.u + angle.cos() * dist).clamp(0.0, self.width as f32);
                        let tv = (ant.v + angle.sin() * dist).clamp(0.0, self.height as f32);
                        ant.target_u = Some(tu);
                        ant.target_v = Some(tv);
                    }

                    if let (Some(tu), Some(tv)) = (ant.target_u, ant.target_v) {
                        let speed = 2.0 * dt;
                        let du = tu - ant.u;
                        let dv = tv - ant.v;
                        let dist = (du * du + dv * dv).sqrt();

                        if dist < speed {
                            ant.u = tu;
                            ant.v = tv;
                            ant.target_u = None;
                            ant.target_v = None;

                            // Chance to find wormhole
                            // Calculate current 3D pos
                            let current_3d = bilinear_interp(ant.u, ant.v, self.width, vertices);

                            // Check a random vertex elsewhere on the grid
                            // Try 3 random attempts
                            for _ in 0..3 {
                                let ru = rand::gen_range(0, self.width);
                                let rv = rand::gen_range(0, self.height);

                                // Must be grid-distant
                                if (ru as f32 - ant.u).abs() > 5.0 || (rv as f32 - ant.v).abs() > 5.0 {
                                    let idx = rv * (self.width + 1) + ru;
                                    let other_3d = vertices[idx];

                                    if current_3d.distance(other_3d) < 2.0 { // Close in 3D
                                        // WORMHOLE FOUND!
                                        ant.state = AntState::Jumping;
                                        ant.jump_start_pos = current_3d;
                                        ant.jump_end_pos = other_3d;
                                        ant.jump_progress = 0.0;
                                        ant.target_u = Some(ru as f32);
                                        ant.target_v = Some(rv as f32);
                                        break;
                                    }
                                }
                            }

                        } else {
                            ant.u += du / dist * speed;
                            ant.v += dv / dist * speed;
                        }
                    }

                    // Deposit pheromone
                    let iu = ant.u.round() as usize;
                    let iv = ant.v.round() as usize;
                    if iu <= self.width && iv <= self.height {
                         let idx = iv * (self.width + 1) + iu;
                         self.pheromones[idx] = (self.pheromones[idx] + 0.5).min(1.0);
                    }
                }
                AntState::Jumping => {
                    ant.jump_progress += dt * 2.0; // Jump speed
                    if ant.jump_progress >= 1.0 {
                        ant.state = AntState::Foraging;
                        if let (Some(tu), Some(tv)) = (ant.target_u, ant.target_v) {
                            ant.u = tu;
                            ant.v = tv;
                        }
                        ant.target_u = None;
                        ant.target_v = None;
                    }
                }
            }
        }
    }
}

fn bilinear_interp(u: f32, v: f32, width_cols: usize, vertices: &[Vec3]) -> Vec3 {
    let iu = u.floor() as usize;
    let iv = v.floor() as usize;
    let fu = u - iu as f32;
    let fv = v - iv as f32;

    let w = width_cols + 1;
    let idx00 = iv * w + iu;
    let idx10 = iv * w + (iu + 1);
    let idx01 = (iv + 1) * w + iu;
    let idx11 = (iv + 1) * w + (iu + 1);

    // Bounds check roughly
    if idx11 >= vertices.len() {
        return Vec3::ZERO;
    }

    let p00 = vertices[idx00];
    let p10 = vertices[idx10];
    let p01 = vertices[idx01];
    let p11 = vertices[idx11];

    let p0 = p00.lerp(p10, fu);
    let p1 = p01.lerp(p11, fu);

    p0.lerp(p1, fv)
}
