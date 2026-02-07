use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        // Only refresh what we need to save resources
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

    pub fn get_cpu_usage_per_core(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage() / 100.0).collect()
    }

    pub fn get_global_cpu_usage(&self) -> f32 {
        self.sys.global_cpu_info().cpu_usage() / 100.0
    }

    pub fn get_memory_usage(&self) -> f32 {
        if self.sys.total_memory() == 0 {
            return 0.0;
        }
        self.sys.used_memory() as f32 / self.sys.total_memory() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_init() {
        let mut monitor = SystemMonitor::new();
        // First refresh usually returns 0 or needs time.
        // Sysinfo needs two refreshes to calculate CPU usage delta.
        // But we don't want to sleep too long in tests.
        // Just verify it doesn't crash and returns valid range.

        monitor.update();
        std::thread::sleep(std::time::Duration::from_millis(100));
        monitor.update();

        let cpu = monitor.get_global_cpu_usage();
        println!("CPU: {}", cpu);
        assert!(cpu >= 0.0 && cpu <= 1.0); // Assuming 0-100 is normalized to 0-1 by get_global_cpu_usage dividing by 100.
        // Wait, sysinfo returns 0-100 usually. My method divides by 100.0.

        let mem = monitor.get_memory_usage();
        println!("Memory: {}", mem);
        assert!(mem >= 0.0 && mem <= 1.0);

        let cores = monitor.get_cpu_usage_per_core();
        println!("Cores: {:?}", cores);
        assert!(!cores.is_empty());
    }
}
