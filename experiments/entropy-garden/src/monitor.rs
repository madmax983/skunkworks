use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything())
            ),
        }
    }

    pub fn get_parameters(&mut self) -> (f64, f64, f32, f64) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        let cpu_usage = self.sys.global_cpu_usage(); // returns f32 usually 0-100
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let mem_usage = if total_mem > 0 {
            (used_mem as f64 / total_mem as f64) * 100.0
        } else {
            0.0
        };

        // Map CPU to Feed rate (F)
        // Range: 0.020 (starved) to 0.080 (flooded)
        let f = 0.020 + (cpu_usage as f64 / 100.0).clamp(0.0, 1.0) * (0.080 - 0.020);

        // Map Memory to Kill rate (k)
        // Range: 0.045 to 0.070
        let k = 0.045 + (mem_usage / 100.0).clamp(0.0, 1.0) * (0.070 - 0.045);

        (f, k, cpu_usage, mem_usage)
    }
}
