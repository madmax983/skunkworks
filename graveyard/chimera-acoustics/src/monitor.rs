use sysinfo::{Cpu, System};

pub struct SystemMonitor {
    sys: System,
    pub cpu_usage: f32,
    pub mem_usage: f32,
    pub swap_usage: f32,
    pub load_avg: f32,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            swap_usage: 0.0,
            load_avg: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        // Average CPU usage across cores
        let cpus = self.sys.cpus();
        if !cpus.is_empty() {
            let total_cpu: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
            self.cpu_usage = (total_cpu / cpus.len() as f32) / 100.0; // 0.0 - 1.0
        } else {
            self.cpu_usage = 0.0;
        }

        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        self.mem_usage = if total_mem > 0 {
            used_mem as f32 / total_mem as f32
        } else {
            0.0
        };

        let total_swap = self.sys.total_swap();
        let used_swap = self.sys.used_swap();
        self.swap_usage = if total_swap > 0 {
            used_swap as f32 / total_swap as f32
        } else {
            0.0
        };

        self.load_avg = System::load_average().one as f32;
    }
}
