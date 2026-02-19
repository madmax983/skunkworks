use sysinfo::System;

pub struct SystemMonitor {
    sys: System,
    pub cpu_usage: f32,
    pub mem_usage: f32,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu();
        sys.refresh_memory();
        Self {
            sys,
            cpu_usage: 0.0,
            mem_usage: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        // cpu_usage() returns percentage (0.0 to 100.0)
        // In sysinfo 0.30, `global_cpu_info()` returns a `&Cpu`.
        // `Cpu` has `cpu_usage()` method.
        self.cpu_usage = self.sys.global_cpu_info().cpu_usage();

        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        if total_mem > 0 {
            self.mem_usage = (used_mem as f32 / total_mem as f32) * 100.0;
        } else {
            self.mem_usage = 0.0;
        }
    }

    pub fn get_lorenz_params(&self) -> (f32, f32, f32) {
        // Sigma (Volatility) - Fixed usually 10.
        let sigma = 10.0;

        // Rho (Rayleigh number) - The driver of chaos.
        // Critical value is ~24.74.
        // We want the system to be stable at low load and chaotic at high load.
        // CPU 0% -> Rho 14 (Stable spiral)
        // CPU 100% -> Rho 40 (Chaotic)
        // Map: 14 + (cpu * 0.26)
        // 0 -> 14
        // 50 -> 27 (Chaotic)
        // 100 -> 40
        let rho = 14.0 + (self.cpu_usage * 0.26);

        // Beta (Geometric factor)
        // Standard is 8/3 ~ 2.66.
        // Let's make memory affect the "shape".
        // Memory 0% -> 2.0
        // Memory 100% -> 4.0
        // 2.0 + (mem_usage * 0.02)
        // If mem_usage is 100, result is 4.0
        let beta = 2.0 + (self.mem_usage * 0.02);

        (sigma, rho, beta)
    }
}
