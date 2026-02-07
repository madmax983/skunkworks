use sysinfo::System;

pub struct Monitor {
    sys: System,
    pub cpu_usage: f64, // 0.0 - 1.0
    pub ram_usage: f64, // 0.0 - 1.0
}

impl Monitor {
    pub fn new() -> Self {
        // Initialize system but don't refresh everything yet to save startup time?
        // Actually we need initial data.
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys,
            cpu_usage: 0.0,
            ram_usage: 0.0,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        // sysinfo 0.30: global_cpu_info().cpu_usage() returns f32 percentage (0-100)
        let global_cpu = self.sys.global_cpu_info().cpu_usage();
        self.cpu_usage = (global_cpu / 100.0) as f64;

        let total_ram = self.sys.total_memory();
        let used_ram = self.sys.used_memory();

        if total_ram > 0 {
            self.ram_usage = used_ram as f64 / total_ram as f64;
        }
    }
}
