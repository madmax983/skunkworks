use crate::model::LorenzParams;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct SystemMonitor {
    sys: System,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemMonitor {
    pub fn new() -> Self {
        // Only refresh CPU and Memory to save resources
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn poll(&mut self) -> (LorenzParams, f32, u64, u64) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpu_usage = self.sys.global_cpu_info().cpu_usage(); // Average usage
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();

        let mem_percent = if total_mem > 0 {
            (used_mem as f64 / total_mem as f64) * 100.0
        } else {
            0.0
        };

        // Map to Lorenz Params
        // Base: sigma=10, rho=28, beta=8/3

        // Sigma (Prandtl) linked to CPU. More CPU -> More turbulence/mixing?
        // Let's map 0-100% to 10.0 - 20.0
        let sigma = 10.0 + (cpu_usage as f64 / 100.0) * 10.0;

        // Rho (Rayleigh) linked to Memory.
        // Let's map 0-100% to 20.0 - 60.0.
        // Standard chaotic value is 28.
        let rho = 20.0 + (mem_percent / 100.0) * 40.0;

        let beta = 8.0 / 3.0;

        (
            LorenzParams { sigma, rho, beta },
            cpu_usage,
            used_mem,
            total_mem,
        )
    }
}
