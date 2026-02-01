use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        // We only care about CPU for now.
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        Self { sys }
    }

    /// Refreshes CPU stats and returns usage percentage (0.0 to 100.0) for each core.
    pub fn update(&mut self) -> Vec<f32> {
        self.sys.refresh_cpu_all();
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }

    #[allow(dead_code)]
    pub fn core_count(&self) -> usize {
        self.sys.cpus().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_init() {
        let mut monitor = SystemMonitor::new();
        // First update usually returns 0, so we just check count
        monitor.update();
        assert!(monitor.core_count() > 0, "Should detect at least one core");
    }
}
