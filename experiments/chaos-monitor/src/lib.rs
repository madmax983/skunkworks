use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct LorenzSystem {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl Default for LorenzSystem {
    fn default() -> Self {
        Self::new(0.1, 0.0, 0.0)
    }
}

impl LorenzSystem {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }

    // RK4 integration for better stability
    pub fn update_rk4(&mut self, dt: f64) {
        let (k1_x, k1_y, k1_z) = self.derivatives(self.x, self.y, self.z);
        let (k2_x, k2_y, k2_z) = self.derivatives(
            self.x + k1_x * dt * 0.5,
            self.y + k1_y * dt * 0.5,
            self.z + k1_z * dt * 0.5,
        );
        let (k3_x, k3_y, k3_z) = self.derivatives(
            self.x + k2_x * dt * 0.5,
            self.y + k2_y * dt * 0.5,
            self.z + k2_z * dt * 0.5,
        );
        let (k4_x, k4_y, k4_z) =
            self.derivatives(self.x + k3_x * dt, self.y + k3_y * dt, self.z + k3_z * dt);

        self.x += (k1_x + 2.0 * k2_x + 2.0 * k3_x + k4_x) * dt / 6.0;
        self.y += (k1_y + 2.0 * k2_y + 2.0 * k3_y + k4_y) * dt / 6.0;
        self.z += (k1_z + 2.0 * k2_z + 2.0 * k3_z + k4_z) * dt / 6.0;
    }

    fn derivatives(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        (dx, dy, dz)
    }
}

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        // Initialize with specific refresh kinds to be efficient
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn get_load_metrics(&self) -> (f64, f64, f64) {
        // CPU load (global)
        let cpu_usage = self.sys.global_cpu_info().cpu_usage() as f64 / 100.0;

        // Memory usage
        let total_mem = self.sys.total_memory() as f64;
        let used_mem = self.sys.used_memory() as f64;
        let mem_usage = if total_mem > 0.0 {
            used_mem / total_mem
        } else {
            0.0
        };

        // Swap usage
        let total_swap = self.sys.total_swap() as f64;
        let used_swap = self.sys.used_swap() as f64;
        let swap_usage = if total_swap > 0.0 {
            used_swap / total_swap
        } else {
            0.0
        };

        (cpu_usage, mem_usage, swap_usage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_update() {
        let mut system = LorenzSystem::new(1.0, 1.0, 1.0);
        let initial_x = system.x;
        system.update_rk4(0.01);
        assert_ne!(system.x, initial_x);
    }

    #[test]
    fn test_system_monitor_creation() {
        let monitor = SystemMonitor::new();
        // Just ensuring it doesn't panic
        let (cpu, mem, swap) = monitor.get_load_metrics();
        assert!(cpu >= 0.0);
        assert!(mem >= 0.0);
        assert!(swap >= 0.0);
    }
}
