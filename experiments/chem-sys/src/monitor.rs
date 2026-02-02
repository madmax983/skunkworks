use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        // Initialize with specific refresh kinds to avoid overhead
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
    }

    pub fn get_parameters(&self) -> (f64, f64) {
        // CPU usage: Average across all CPUs
        let cpu_usage = self.sys.global_cpu_info().cpu_usage() as f64; // 0.0 to 100.0

        // Memory usage
        let total_mem = self.sys.total_memory() as f64;
        let used_mem = self.sys.used_memory() as f64;
        let mem_ratio = if total_mem > 0.0 {
            used_mem / total_mem
        } else {
            0.0
        }; // 0.0 to 1.0

        // Map to Gray-Scott parameters
        // f (feed): Driven by CPU. Higher CPU = higher feed (more energy/instability)
        // Range: [0.01, 0.06]
        // Standard "spots" is around 0.055
        let f = 0.01 + (cpu_usage / 100.0).clamp(0.0, 1.0) * 0.05;

        // k (kill): Driven by Memory. Higher Memory = higher kill?
        // Range: [0.03, 0.07]
        // Standard "spots" is around 0.062
        let k = 0.03 + mem_ratio.clamp(0.0, 1.0) * 0.04;

        (f, k)
    }

    pub fn get_stats_string(&self) -> String {
        let cpu = self.sys.global_cpu_info().cpu_usage();
        let total_mem = self.sys.total_memory() / 1024 / 1024;
        let used_mem = self.sys.used_memory() / 1024 / 1024;
        format!("CPU: {:.1}% | MEM: {}/{} MB", cpu, used_mem, total_mem)
    }
}
