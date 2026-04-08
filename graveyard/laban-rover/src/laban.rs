use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct LabanEffort {
    /// Space: 0.0 (Direct) to 1.0 (Indirect/Flexible)
    pub space: f32,
    /// Weight: 0.0 (Strong/Heavy) to 1.0 (Light/Gentle)
    pub weight: f32,
    /// Time: 0.0 (Sudden/Quick) to 1.0 (Sustained/Slow)
    pub time: f32,
    /// Flow: 0.0 (Bound) to 1.0 (Free)
    pub flow: f32,
}

impl Default for LabanEffort {
    fn default() -> Self {
        Self {
            space: 0.5,
            weight: 0.5,
            time: 0.5,
            flow: 0.5,
        }
    }
}

impl LabanEffort {
    #[allow(dead_code)]
    pub fn new(space: f32, weight: f32, time: f32, flow: f32) -> Self {
        Self {
            space,
            weight,
            time,
            flow,
        }
    }

    pub fn interpolate(&self, target: &LabanEffort, t: f32) -> Self {
        Self {
            space: self.space + (target.space - self.space) * t,
            weight: self.weight + (target.weight - self.weight) * t,
            time: self.time + (target.time - self.time) * t,
            flow: self.flow + (target.flow - self.flow) * t,
        }
    }
}

pub struct Director {
    pub current_effort: LabanEffort,
    pub target_effort: LabanEffort,
    pub transition_speed: f32,
    // Removing Timer logic, will handle in main loop via delta time
}

impl Default for Director {
    fn default() -> Self {
        Self {
            current_effort: LabanEffort::default(),
            target_effort: LabanEffort::default(),
            transition_speed: 1.0,
        }
    }
}

impl Director {
    pub fn update(&mut self, dt_seconds: f32) {
        // Interpolate towards target
        // The speed depends on the "Time" factor of the TARGET (Self-referential logic)
        // If target is "Sudden" (Time -> 0.0), we move fast.
        // If target is "Sustained" (Time -> 1.0), we move slow.
        let speed_mod = 1.0 + (1.0 - self.target_effort.time) * 2.0;
        let dt = dt_seconds * self.transition_speed * speed_mod;

        // Simple lerp with clamping
        let t = dt.clamp(0.0, 1.0);
        self.current_effort = self
            .current_effort
            .interpolate(&self.target_effort, t * 0.1); // Smooth damping
    }

    pub fn set_target_from_environment(
        &mut self,
        size_factor: f32,
        age_factor: f32,
        depth_factor: f32,
    ) {
        // Map Environment to Laban Efforts

        // Weight: Heavy (0.0) if size is large, Light (1.0) if small
        // size_factor should be 0.0 (small) to 1.0 (large)
        self.target_effort.weight = 1.0 - size_factor.clamp(0.0, 1.0);

        // Time: Sudden (0.0) if new/urgent, Sustained (1.0) if old
        // age_factor 0.0 (now) to 1.0 (old)
        self.target_effort.time = age_factor.clamp(0.0, 1.0);

        // Space: Direct (0.0) if shallow/ordered, Indirect (1.0) if deep/messy
        self.target_effort.space = depth_factor.clamp(0.0, 1.0);

        // Flow: Bound (0.0) if restricted, Free (1.0) otherwise
        // For now, let's make it random or based on hidden files?
        // Let's randomize it slightly for "life"
        let mut rng = rand::thread_rng();
        self.target_effort.flow =
            (self.target_effort.flow + rng.gen_range(-0.05..0.05)).clamp(0.0, 1.0);
    }
}
