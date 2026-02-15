use std::time::{Duration, Instant};
use sysinfo::System;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub feed: f32,
    pub kill: f32,
    pub dt: f32,
    pub diff_u: f32,
    pub diff_v: f32,
    pub _padding1: f32,
    pub _padding2: f32,
    pub _padding3: f32,
}

pub struct SystemMonitor {
    sys: System,
    last_update: Instant,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            last_update: Instant::now(),
        }
    }

    pub fn refresh(&mut self) -> (f32, f32) {
        if self.last_update.elapsed() >= Duration::from_millis(500) {
            self.sys.refresh_all();
            self.last_update = Instant::now();
        }

        let cpu_usage = self.sys.global_cpu_info().cpu_usage(); // 0.0 to 100.0
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let ram_usage = if total_mem > 0 {
            (used_mem as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        };

        (cpu_usage, ram_usage)
    }
}
