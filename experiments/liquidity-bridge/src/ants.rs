use rand::Rng;
use crate::market::{Grid, Terrain};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Foraging,
    Bridging,
    Returning,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub state: State,
}

impl Ant {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            state: State::Foraging,
        }
    }
}

pub struct AntColony {
    pub ants: Vec<Ant>,
    pub width: usize,
    pub height: usize,
}

impl AntColony {
    pub fn new(width: usize, height: usize, count: usize) -> Self {
        let mut ants = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();

        // Spawn ants at the bottom (Bid side)
        for _ in 0..count {
            ants.push(Ant::new(
                rng.gen_range(0..width),
                rng.gen_range(height - 10..height),
            ));
        }

        Self {
            ants,
            width,
            height,
        }
    }

    pub fn update(&mut self, grid: &mut Grid, volatility: f32) {
        let mut rng = rand::thread_rng();

        for ant in self.ants.iter_mut() {
            match ant.state {
                State::Bridging => {
                    // Volatility might break the bridge
                    if rng.gen_bool(volatility as f64) {
                        ant.state = State::Foraging;
                        grid.set_terrain(ant.x, ant.y, Terrain::Gap);
                    } else {
                        // Reinforce
                         grid.set_terrain(ant.x, ant.y, Terrain::Bridge);
                    }
                }
                State::Foraging => {
                    // Random walk with bias towards the Gap (Upwards)
                    let dx = rng.gen_range(-1..=1);
                    let dy = rng.gen_range(-1..=0); // Bias Up

                    let nx = (ant.x as isize + dx).clamp(0, self.width as isize - 1) as usize;
                    let ny = (ant.y as isize + dy).clamp(0, self.height as isize - 1) as usize;

                    let target_terrain = grid.get_terrain(nx, ny);

                    match target_terrain {
                        Terrain::Gap => {
                            // Found a gap. Should we bridge?
                            // Chance increases if we are close to other bridges or edges?
                            // For now, simple probability.
                            if rng.gen_bool(0.1) {
                                ant.state = State::Bridging;
                                ant.x = nx;
                                ant.y = ny;
                                grid.set_terrain(nx, ny, Terrain::Bridge);
                            }
                        }
                        Terrain::Bridge | Terrain::Solid => {
                            // Move freely
                            ant.x = nx;
                            ant.y = ny;
                        }
                    }
                }
                State::Returning => {
                    // Placeholder for future logic
                    ant.state = State::Foraging;
                }
            }
        }
    }
}
