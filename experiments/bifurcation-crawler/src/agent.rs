use chimera_lang::prelude::*;
use macroquad::prelude::*;
use physics_pbd::PbdSystem;

/// Represents the physical and mental state of a simulated crawler agent.
///
/// The crawler consists of physical particle segments connected by muscular actuators.
/// Its brain is driven by a `ChimeraVM` that processes environmental chaos and dictates
/// muscular contraction logic.
pub struct Crawler {
    /// The virtual machine executing the crawler's genetic logic.
    pub vm: ChimeraVM,
    /// Indices referencing the agent's physical particles in the `PbdSystem`.
    pub particle_indices: Vec<usize>,
    /// Indices referencing the agent's muscular actuators (distance constraints) in the `PbdSystem`.
    pub actuator_indices: Vec<usize>,
    /// The rendering color of the crawler, dynamically indicating its health/energy.
    pub color: Color,
    /// The crawler's internal energy level (0.0 to 100.0). Depletes in chaotic zones.
    pub energy: f32,
    /// The total simulation time the crawler has been alive.
    pub age: f32,
}

impl Crawler {
    /// Constructs a new `Crawler` with a multi-segmented physics body.
    ///
    /// The body consists of 5 particles connected horizontally by 4 distance constraints
    /// acting as muscular actuators.
    ///
    /// # Arguments
    ///
    /// * `pos` - The initial 2D coordinate placement of the crawler's head.
    /// * `system` - A mutable reference to the active Position Based Dynamics physics system.
    /// * `dna` - The genetic code injected into the crawler's internal `ChimeraVM`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bifurcation_crawler::agent::Crawler;
    /// # use physics_pbd::PbdSystem;
    /// # use macroquad::prelude::*;
    /// # use chimera_lang::prelude::*;
    /// let mut physics = PbdSystem::new();
    /// let empty_dna = Dna::default();
    /// let crawler = Crawler::new(vec2(1.0, 5.0), &mut physics, empty_dna);
    ///
    /// assert_eq!(crawler.energy, 100.0);
    /// assert_eq!(crawler.particle_indices.len(), 5);
    /// ```
    pub fn new(pos: Vec2, system: &mut PbdSystem, dna: Dna) -> Self {
        let mut particle_indices = Vec::new();
        let mut actuator_indices = Vec::new();

        // Create a simple worm: 4 segments (5 particles)
        // Horizontal arrangement
        for i in 0..5 {
            let p_pos = vec3(pos.x + i as f32 * 0.05, pos.y, 0.0);
            // Mass 1.0 for head, slightly less for tail? Uniform is fine.
            let idx = system.add_particle(p_pos, 1.0);
            particle_indices.push(idx);
        }

        // Add Actuators (Muscles) between segments
        for i in 0..4 {
            let p1 = particle_indices[i];
            let p2 = particle_indices[i + 1];
            // Min/Max/Stiffness
            system.add_actuator_constraint(p1, p2, 0.02, 0.10, 0.8);
            let c_idx = system.constraints.len() - 1;
            actuator_indices.push(c_idx);
        }

        // Add Struts for stability? Not needed for a chain.
        // But maybe a "skin" constraint?

        Crawler {
            vm: ChimeraVM::new(dna),
            particle_indices,
            actuator_indices,
            color: WHITE,
            energy: 100.0,
            age: 0.0,
        }
    }

    /// Updates the crawler's internal virtual machine and applies its computational
    /// outputs to physically actuate its muscular constraints.
    ///
    /// The agent reads the `chaos_factor`, its current `energy`, and body `strain`
    /// onto its VM stack. After stepping the VM, popped values are mapped to
    /// constraint scaling factors, allowing the agent to "flex" its body to move.
    ///
    /// # Arguments
    ///
    /// * `system` - A mutable reference to the Position Based Dynamics physics system.
    /// * `chaos_factor` - The proximity distance to the nearest logistic map attractor.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bifurcation_crawler::agent::Crawler;
    /// # use physics_pbd::PbdSystem;
    /// # use macroquad::prelude::*;
    /// # use chimera_lang::prelude::*;
    /// let mut physics = PbdSystem::new();
    /// let mut crawler = Crawler::new(vec2(1.0, 5.0), &mut physics, Dna::default());
    ///
    /// // Simulate an update step with a 0.0 chaos factor (safe zone)
    /// crawler.update(&mut physics, 0.0);
    /// assert!(crawler.age > 0.0);
    /// ```
    pub fn update(&mut self, system: &mut PbdSystem, chaos_factor: f32) {
        self.age += 0.016;

        // Input to VM:
        // 1. Current Chaos Level (0-100)
        // 2. Internal Energy (0-100)
        // 3. Average Strain (0-100)

        let strain = self.calculate_strain(system);

        self.vm.stack.clear();
        self.vm
            .stack
            .push(Value::Int((chaos_factor * 100.0) as i64));
        self.vm.stack.push(Value::Int(self.energy as i64));
        self.vm.stack.push(Value::Int((strain * 100.0) as i64));

        // Run VM
        for _ in 0..50 {
            let _ = self.vm.step();
        }

        // Output: Muscle contractions
        // Expecting 4 values for 4 actuators? Or 1 value for all?
        // Let's pop up to 4 values.

        for (i, &c_idx) in self.actuator_indices.iter().enumerate() {
            let factor = if let Some(val) = self.vm.stack.pop() {
                match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                }
            } else {
                0.5 + (self.age * 5.0 + i as f32).sin() * 0.5 // Default: Peristaltic wave
            };

            // Apply to constraint
            if let physics_pbd::Constraint::Actuator {
                factor: ref mut f, ..
            } = &mut system.constraints[c_idx]
            {
                *f = factor;
            }
        }

        // Color reflects health/energy
        self.color = if self.energy > 50.0 { GREEN } else { RED };
        self.color.a = (self.energy / 100.0).clamp(0.2, 1.0);
    }

    fn calculate_strain(&self, system: &PbdSystem) -> f32 {
        let mut total = 0.0;
        for &c_idx in &self.actuator_indices {
            if let physics_pbd::Constraint::Actuator {
                p1, p2, max_len, ..
            } = &system.constraints[c_idx]
            {
                let pos1 = system.particles[*p1].pos;
                let pos2 = system.particles[*p2].pos;
                total += pos1.distance(pos2) / max_len;
            }
        }
        total / self.actuator_indices.len() as f32
    }
}
