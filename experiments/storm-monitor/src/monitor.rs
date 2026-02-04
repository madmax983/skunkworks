use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        // Initial refresh to populate data
        sys.refresh_cpu();
        sys.refresh_memory();
        Self { sys }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
    }

    pub fn get_cpu_usage(&self) -> f64 {
        let cpus = self.sys.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        let sum: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        (sum as f64) / (cpus.len() as f64)
    }

    pub fn get_memory_usage(&self) -> f64 {
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        if total == 0 {
            return 0.0;
        }
        (used as f64) / (total as f64)
    }

    // Map to Lorenz Parameters
    // Sigma (Standard 10.0). Maps to Volatility/Prandtl number.
    // Driven by CPU. Range [10.0, 50.0]
    pub fn get_sigma(&self) -> f64 {
        let cpu = self.get_cpu_usage(); // 0-100
        10.0 + (cpu * 0.4)
    }

    // Rho (Standard 28.0). Maps to Rayleigh number (Instability).
    // Driven by Memory. Range [28.0, 78.0]
    pub fn get_rho(&self) -> f64 {
        let mem = self.get_memory_usage(); // 0-1.0
        // Convert to percentage for mapping consistency if needed, but here it's 0-1
        28.0 + (mem * 50.0)
    }

    // Beta (Standard 8.0/3.0 ~= 2.66). Maps to geometric aspect ratio.
    // Driven by... let's say CPU variance or just a constant drift?
    // Let's use Swap usage if available, else a sine wave of time?
    // Let's stick to system stats. Maybe "used swap / total swap".
    pub fn get_beta(&self) -> f64 {
         let total = self.sys.total_swap();
         let used = self.sys.used_swap();
         let ratio = if total == 0 { 0.0 } else { used as f64 / total as f64 };
         2.666 + (ratio * 5.0)
    }
}
