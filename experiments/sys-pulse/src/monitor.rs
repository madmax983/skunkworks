use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct SystemStats {
    pub cpu_usage: f32,      // 0.0 - 100.0 (Global average)
    pub memory_usage: f32,   // 0.0 - 1.0 (Used / Total)
    pub swap_usage: f32,     // 0.0 - 1.0
}

pub struct Monitor {
    stats: Arc<RwLock<SystemStats>>,
    running: Arc<RwLock<bool>>,
}

impl Monitor {
    pub fn new(stats: Arc<RwLock<SystemStats>>) -> Self {
        Self {
            stats,
            running: Arc::new(RwLock::new(true)),
        }
    }

    pub fn spawn(self) {
        thread::spawn(move || {
            let mut sys = System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );

            // Initial refresh
            sys.refresh_cpu();
            sys.refresh_memory();

            while *self.running.read().unwrap() {
                // Sleep first to allow CPU usage calculation over time
                thread::sleep(Duration::from_millis(200));

                sys.refresh_cpu();
                sys.refresh_memory();

                let cpus = sys.cpus();
                let cpu_usage = if !cpus.is_empty() {
                    cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
                } else {
                    0.0
                };

                let total_mem = sys.total_memory() as f32;
                let used_mem = sys.used_memory() as f32;
                let mem_usage = if total_mem > 0.0 { used_mem / total_mem } else { 0.0 };

                let total_swap = sys.total_swap() as f32;
                let used_swap = sys.used_swap() as f32;
                let swap_usage = if total_swap > 0.0 { used_swap / total_swap } else { 0.0 };

                let mut lock = self.stats.write().unwrap();
                lock.cpu_usage = cpu_usage;
                lock.memory_usage = mem_usage;
                lock.swap_usage = swap_usage;
            }
        });
    }

    pub fn stop(&self) {
        let mut lock = self.running.write().unwrap();
        *lock = false;
    }
}
