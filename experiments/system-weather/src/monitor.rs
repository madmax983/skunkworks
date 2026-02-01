use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
            ),
        }
    }

    pub fn get_cpu_usage(&mut self) -> f32 {
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()
    }

    pub fn get_chaos_parameter(&mut self) -> f64 {
        let cpu = self.get_cpu_usage();
        // Map 0-100% CPU to 10.0 - 50.0 Rho
        10.0 + (cpu as f64 / 100.0) * 40.0
    }
}
