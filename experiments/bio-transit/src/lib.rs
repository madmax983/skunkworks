//! # Bio-Transit
//!
//! A procedural simulation mapping the behavior of *Physarum polycephalum* (slime mold)
//! onto urban transit network design.
//!
//! This crate provides the foundational simulation data structures:
//! * [`Agent`]: The individual "Commuter" driven by pheromone trails and destination bias.
//! * [`TrailMap`]: The spatial grid storing pheromone concentration.
//! * [`Simulation`]: The overarching environment that steps the logic over time.
//!
//! Agents deposit pheromones on the `TrailMap` as they travel between randomly
//! assigned "Home" and "Work" [`City`] points, creating emergent paths that subsequent
//! agents follow.

use macroquad::prelude::*;
use rayon::prelude::*;

/// Represents the daily phase of an [`Agent`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgentState {
    /// Agent is currently moving towards its work city.
    CommutingToWork,
    /// Agent is currently moving towards its home city.
    CommutingHome,
}

/// A simulated Physarum commuter.
///
/// The agent traverses the space using a combination of local pheromone sensing
/// (following paths laid by others) and a global "GPS" pull towards its target
/// (either home or work).
///
/// # Examples
///
/// ```
/// use bio_transit::{Agent, AgentState};
/// use macroquad::math::Vec2;
///
/// let home = Vec2::new(10.0, 10.0);
/// let work = Vec2::new(90.0, 90.0);
///
/// // Create a new agent starting at home, facing right (angle 0.0)
/// let agent = Agent::new(home, 0.0, home, work);
///
/// assert_eq!(agent.state, AgentState::CommutingToWork);
/// ```
#[derive(Clone, Copy)]
pub struct Agent {
    /// The current 2D spatial position.
    pub pos: Vec2,
    /// The current facing direction in radians.
    pub angle: f32,
    /// The designated "Home" coordinates.
    pub home: Vec2,
    /// The designated "Work" coordinates.
    pub work: Vec2,
    /// The current commuter phase (heading home or to work).
    pub state: AgentState,
    /// Mask used to distinguish agent types (currently unused).
    pub species_mask: u32,
}

impl Agent {
    /// Spawns a new agent.
    ///
    /// The agent defaults to the [`AgentState::CommutingToWork`] phase.
    ///
    /// # Arguments
    ///
    /// * `pos` - The initial starting position.
    /// * `angle` - Initial facing direction in radians.
    /// * `home` - The coordinate of the home city.
    /// * `work` - The coordinate of the work city.
    pub fn new(pos: Vec2, angle: f32, home: Vec2, work: Vec2) -> Self {
        Self {
            pos,
            angle,
            home,
            work,
            state: AgentState::CommutingToWork,
            species_mask: 1,
        }
    }

    /// Steps the agent forward in time based on sensor readings and target bias.
    ///
    /// The agent senses the [`TrailMap`] at three points (left, center, right) relative
    /// to its current heading. It turns toward the strongest pheromone concentration,
    /// while continuously nudging its angle toward its current target destination
    /// to prevent infinite loops.
    ///
    /// # Arguments
    ///
    /// * `map` - The grid providing the pheromone trails to sense.
    /// * `settings` - The global simulation settings defining sensor range and turn angles.
    ///
    /// # Examples
    ///
    /// ```
    /// use bio_transit::{Agent, TrailMap, Settings};
    /// use macroquad::math::Vec2;
    ///
    /// let mut agent = Agent::new(Vec2::new(50.0, 50.0), 0.0, Vec2::new(10.0, 10.0), Vec2::new(90.0, 90.0));
    /// let map = TrailMap::new(100, 100);
    /// let settings = Settings::default();
    ///
    /// let old_pos = agent.pos;
    /// agent.update(&map, &settings);
    ///
    /// assert_ne!(old_pos, agent.pos);
    /// ```
    pub fn update(&mut self, map: &TrailMap, settings: &Settings) {
        let sensor_angle = settings.sensor_angle;
        let sensor_dist = settings.sensor_dist;
        let turn_angle = settings.turn_angle;

        // Sensing (Physarum Logic)
        let sense = |angle_offset: f32| -> f32 {
            let angle = self.angle + angle_offset;
            let dir = Vec2::new(angle.cos(), angle.sin());
            let sensor_pos = self.pos + dir * sensor_dist;

            // Wrap coordinates
            let w = map.width as f32;
            let h = map.height as f32;
            let x = (sensor_pos.x.rem_euclid(w)) as usize;
            let y = (sensor_pos.y.rem_euclid(h)) as usize;

            // Safe access
            if x < map.width && y < map.height {
                map.grid[y * map.width + x]
            } else {
                0.0
            }
        };

        let left = sense(-sensor_angle);
        let center = sense(0.0);
        let right = sense(sensor_angle);

        // Turn based on sensors
        let mut turned = false;
        if center > left && center > right {
            // Stay straight
        } else if center < left && center < right {
            // Randomly turn left or right
            let mut rng = ::rand::thread_rng();
            use ::rand::Rng;
            if rng.gen_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
            turned = true;
        } else if left > right {
            self.angle -= turn_angle;
            turned = true;
        } else if right > left {
            self.angle += turn_angle;
            turned = true;
        }

        // Target Bias (Commuter Logic)
        let target = match self.state {
            AgentState::CommutingToWork => self.work,
            AgentState::CommutingHome => self.home,
        };

        // If close to target, switch state
        if self.pos.distance(target) < 10.0 {
            self.state = match self.state {
                AgentState::CommutingToWork => AgentState::CommutingHome,
                AgentState::CommutingHome => AgentState::CommutingToWork,
            };
            // Turn around 180 degrees roughly
            self.angle += std::f32::consts::PI;
        }

        // Apply bias towards target
        // If we just turned based on trail, apply less bias to respect the trail
        let bias_strength = if turned { 0.02 } else { 0.05 };

        let to_target = target - self.pos;
        let target_angle = to_target.y.atan2(to_target.x);
        let angle_diff = (target_angle - self.angle + std::f32::consts::PI)
            .rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;

        // Nudge
        self.angle += angle_diff * bias_strength;

        // Move
        let dir = Vec2::new(self.angle.cos(), self.angle.sin());
        self.pos += dir * settings.move_speed;

        // Wrap Position
        self.pos.x = self.pos.x.rem_euclid(map.width as f32);
        self.pos.y = self.pos.y.rem_euclid(map.height as f32);
    }
}

/// A 2D spatial grid storing pheromone concentration.
///
/// Think of this as the "canvas" the agents draw on. It handles depositing
/// pheromones and the global diffusion/decay step that blurs the trails over time.
///
/// # Examples
///
/// ```
/// use bio_transit::{TrailMap, Settings};
///
/// let mut map = TrailMap::new(100, 100);
///
/// // Deposit pheromone at (50, 50)
/// map.deposit(50, 50, 1.0);
///
/// // Step the diffusion
/// let settings = Settings::default();
/// map.diffuse_and_decay(&settings);
///
/// // The pheromone will have decayed and spread slightly
/// assert!(map.grid[50 * 100 + 50] < 1.0); // Center decayed
/// assert!(map.grid[50 * 100 + 51] > 0.0); // Spread to right neighbor
/// ```
pub struct TrailMap {
    /// The width of the simulation grid.
    pub width: usize,
    /// The height of the simulation grid.
    pub height: usize,
    /// The flattened 1D array representing the 2D grid of concentrations (0.0 to 1.0).
    pub grid: Vec<f32>,
}

impl TrailMap {
    /// Creates a new, empty map initialized with zero pheromones.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![0.0; width * height],
        }
    }

    /// Deposits pheromones at a specific coordinate.
    ///
    /// Concentration is clamped to a maximum of 1.0. Safe against out-of-bounds coords.
    ///
    /// # Arguments
    ///
    /// * `x` - The grid X coordinate.
    /// * `y` - The grid Y coordinate.
    /// * `amount` - How much pheromone to add.
    pub fn deposit(&mut self, x: usize, y: usize, amount: f32) {
        let idx = y * self.width + x;
        if idx < self.grid.len() {
            self.grid[idx] = (self.grid[idx] + amount).min(1.0);
        }
    }

    /// Applies a 3x3 box blur to spread pheromones, followed by global decay.
    ///
    /// This runs in parallel using `rayon`.
    ///
    /// # Arguments
    ///
    /// * `settings` - Global tuning containing the `decay_rate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bio_transit::{TrailMap, Settings};
    ///
    /// let mut map = TrailMap::new(10, 10);
    /// map.deposit(5, 5, 1.0);
    /// map.diffuse_and_decay(&Settings::default());
    ///
    /// assert!(map.grid[55] < 1.0); // Pheromone decayed
    /// ```
    pub fn diffuse_and_decay(&mut self, settings: &Settings) {
        let mut next_grid = vec![0.0; self.grid.len()];
        let w = self.width;
        let h = self.height;
        let decay = settings.decay_rate;

        // Parallel update using Rayon
        next_grid.par_iter_mut().enumerate().for_each(|(i, cell)| {
            let x = i % w;
            let y = i / w;

            // 3x3 Box Blur
            let mut sum = 0.0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                    // Standard indexing is safe (panics on OOB)
                    sum += self.grid[ny * w + nx];
                }
            }

            *cell = (sum / 9.0) * decay;
        });

        self.grid = next_grid;
    }
}

/// A randomly placed point of interest representing Home/Work destinations.
pub struct City {
    /// The location on the map.
    pub pos: Vec2,
    /// The visual radius (for rendering).
    pub radius: f32,
    /// The visual color (for rendering).
    pub color: Color,
}

/// Global parameters driving the Physarum logic.
///
/// # Examples
///
/// ```
/// use bio_transit::Settings;
///
/// let settings = Settings::default();
/// assert_eq!(settings.move_speed, 1.0);
/// ```
pub struct Settings {
    /// Angle in radians separating the three sensors.
    pub sensor_angle: f32,
    /// Distance in pixels the sensors reach forward.
    pub sensor_dist: f32,
    /// Angle in radians an agent turns when it detects a trail.
    pub turn_angle: f32,
    /// Units an agent moves forward each tick.
    pub move_speed: f32,
    /// Factor by which pheromones diminish each tick.
    pub decay_rate: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sensor_angle: std::f32::consts::PI / 4.0,
            sensor_dist: 9.0,
            turn_angle: std::f32::consts::PI / 4.0,
            move_speed: 1.0,
            decay_rate: 0.9,
        }
    }
}

/// The overarching simulation environment.
///
/// Handles orchestrating the parallel updates of all agents and the diffusion
/// of the `TrailMap`.
///
/// # Examples
///
/// ```
/// use bio_transit::Simulation;
///
/// // Create a small simulation with 10 agents
/// let mut sim = Simulation::new(100, 100, 10);
///
/// // Step the simulation forward one tick
/// sim.step();
/// ```
pub struct Simulation {
    /// The active commuters.
    pub agents: Vec<Agent>,
    /// The spatial grid.
    pub map: TrailMap,
    /// The available destination nodes.
    pub cities: Vec<City>,
    /// Tuning parameters for the logic.
    pub settings: Settings,
}

impl Simulation {
    /// Instantiates a new environment, randomly scattering cities and assigning
    /// agents to them.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the simulation space in pixels.
    /// * `height` - The height of the simulation space in pixels.
    /// * `num_agents` - Number of commuters to spawn.
    ///
    /// # Panics
    ///
    /// While it attempts to bound parameters, it assumes the `width` and `height`
    /// are reasonable enough to place at least 5 randomly generated cities safely.
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let map = TrailMap::new(width, height);
        let mut agents = Vec::with_capacity(num_agents);
        let mut cities = Vec::new();

        use ::rand::Rng;
        let mut rng = ::rand::thread_rng();

        let w = width as f32;
        let h = height as f32;
        // Ensure bounds are safe
        let padding = 50.0f32.min(w * 0.4).min(h * 0.4);

        // Spawn Cities
        let num_cities = 5;
        for _ in 0..num_cities {
            cities.push(City {
                pos: vec2(
                    rng.gen_range(padding..(w - padding)),
                    rng.gen_range(padding..(h - padding)),
                ),
                radius: rng.gen_range(10.0..30.0),
                color: Color::new(rng.r#gen(), rng.r#gen(), rng.r#gen(), 1.0),
            });
        }

        // Spawn Agents
        for _ in 0..num_agents {
            // Pick Home and Work
            let home_idx = rng.gen_range(0..cities.len());
            let mut work_idx = rng.gen_range(0..cities.len());
            while work_idx == home_idx {
                work_idx = rng.gen_range(0..cities.len());
            }

            let home = cities[home_idx].pos;
            let work = cities[work_idx].pos;

            // Start at home
            // Add some jitter
            let start_pos = home + vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
            let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);

            agents.push(Agent::new(start_pos, angle, home, work));
        }

        Self {
            agents,
            map,
            cities,
            settings: Settings::default(),
        }
    }

    /// Advances the simulation by a single tick.
    ///
    /// 1. Updates agent positions in parallel.
    /// 2. Deposits agent trails sequentially.
    /// 3. Diffuses and decays the `TrailMap` in parallel.
    pub fn step(&mut self) {
        // Update agents in parallel
        let map = &self.map;
        let settings = &self.settings;
        self.agents.par_iter_mut().for_each(|agent| {
            agent.update(map, settings);
        });

        // Deposit pheromones (Serial for now)
        let w = self.map.width;
        let h = self.map.height;
        for agent in &self.agents {
            let x = agent.pos.x as usize;
            let y = agent.pos.y as usize;
            if x < w && y < h {
                self.map.deposit(x, y, 1.0);
            }
        }

        // Diffuse
        self.map.diffuse_and_decay(&self.settings);
    }
}
