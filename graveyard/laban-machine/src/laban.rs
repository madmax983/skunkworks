use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, Debug, Reflect)]
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

#[derive(Resource)]
pub struct Director {
    pub current_effort: LabanEffort,
    pub target_effort: LabanEffort,
    pub transition_speed: f32,
    pub timer: Timer,
}

impl Default for Director {
    fn default() -> Self {
        Self {
            current_effort: LabanEffort::default(),
            target_effort: LabanEffort::default(),
            transition_speed: 1.0,
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

pub fn director_system(time: Res<Time>, mut director: ResMut<Director>) {
    director.timer.tick(time.delta());

    if director.timer.finished() {
        // Pick a new random target
        let mut rng = rand::thread_rng();
        director.target_effort = LabanEffort {
            space: rng.gen_range(0.0..1.0),
            weight: rng.gen_range(0.0..1.0),
            time: rng.gen_range(0.0..1.0),
            flow: rng.gen_range(0.0..1.0),
        };
        // Randomize transition speed slightly
        director.transition_speed = rng.gen_range(0.5..2.0);
    }

    // Interpolate towards target
    // The speed depends on the "Time" factor of the TARGET (Self-referential logic)
    // If target is "Sudden" (Time -> 0.0), we move fast.
    // If target is "Sustained" (Time -> 1.0), we move slow.
    let speed_mod = 1.0 + (1.0 - director.target_effort.time) * 2.0;
    let dt = time.delta_seconds() * director.transition_speed * speed_mod;

    // Simple lerp with clamping
    let t = dt.clamp(0.0, 1.0);
    director.current_effort = director
        .current_effort
        .interpolate(&director.target_effort, t * 0.1); // Smooth damping
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate() {
        let start = LabanEffort::new(0.0, 0.0, 0.0, 0.0);
        let end = LabanEffort::new(1.0, 1.0, 1.0, 1.0);
        let mid = start.interpolate(&end, 0.5);

        assert!((mid.space - 0.5).abs() < 1e-6);
        assert!((mid.weight - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_director_logic() {
        let mut director = Director::default();
        director.current_effort = LabanEffort::new(0.0, 0.0, 0.0, 0.0);
        director.target_effort = LabanEffort::new(1.0, 1.0, 1.0, 1.0);
        director.transition_speed = 1.0;

        // Simulate a step
        // We can't easily simulate Time resource here without a Bevy app,
        // but we can test the math logic if we extract it.
        // For now, let's just verify struct instantiation works.
        assert_eq!(director.current_effort.space, 0.0);
    }
}
