use crate::fish::{Fish, PondParams};
use rand::Rng;
use ratatui::style::Color;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct Pond {
    pub fish: Vec<Fish>,
    pub params: PondParams,
    pub system: System,
}

impl Pond {
    pub fn new(width: f64, height: f64, count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut fish = Vec::with_capacity(count);
        let colors = [
            Color::Red,
            Color::Yellow,
            Color::Indexed(208), // Orange
            Color::White,
            Color::Cyan, // Special "Spirit" Koi
        ];

        for _ in 0..count {
            fish.push(Fish::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
                colors[rng.gen_range(0..colors.len())],
            ));
        }

        let params = PondParams {
            width,
            height,
            ..Default::default()
        };

        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );

        Self {
            fish,
            params,
            system,
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();

        // Calculate Global CPU Load (avg of all cores)
        let cpus = self.system.cpus();
        let cpu_usage: f32 = if !cpus.is_empty() {
            cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
        } else {
            0.0
        };

        // Map CPU (0-100) to Turbulence (0.0 - 1.0)
        // A usage of 50% is quite high turbulence.
        self.params.turbulence = (cpu_usage / 100.0).clamp(0.0, 1.0) as f64;

        // Map Memory to Visuals (handled in UI, but we can store state here if needed)
        // For now, fish behavior is mostly driven by turbulence.

        // Update Fish
        // We need to clone the fish to pass as read-only flock to update
        // Or implement a double-buffer approach.
        // For simplicity, we'll clone the vector of minimal data or update in place with slight lag.
        // Standard Boids usually takes a snapshot.

        let old_fish = self.fish.clone(); // Optimize later if needed
        for fish in self.fish.iter_mut() {
            // Pass slice of all *other* fish? Or just all fish (including self is usually fine if distance > 0 checks handle it)
            // passing `old_fish` is correct.
            fish.update(&old_fish, &self.params);
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.params.width = width;
        self.params.height = height;
    }

    pub fn add_food(&mut self, x: f64, y: f64) {
        // For now, food acts as a momentary attractor or just spawns a new fish?
        // Let's spawn a new fish for now as "food turned into fish" or just "stocking the pond"
        self.fish.push(Fish::new(x, y, Color::Green));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pond_initialization() {
        let pond = Pond::new(100.0, 100.0, 10);
        assert_eq!(pond.fish.len(), 10);
        assert_eq!(pond.params.width, 100.0);
    }

    #[test]
    fn test_pond_update() {
        let mut pond = Pond::new(100.0, 100.0, 5);
        let initial_x = pond.fish[0].x;

        // Force some velocity
        pond.fish[0].vx = 1.0;

        pond.update();

        assert_ne!(pond.fish[0].x, initial_x, "Fish should move after update");
    }
}
