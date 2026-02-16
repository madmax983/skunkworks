use bevy::prelude::*;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};

#[derive(Resource)]
pub struct SystemMonitor {
    pub sys: System,
    pub cpu_usage: f32, // 0.0 to 100.0 (avg across cores)
    pub ram_usage: f32, // 0.0 to 1.0 (used / total)
    pub timer: Timer,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything())
            ),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub fn update_system_stats(mut monitor: ResMut<SystemMonitor>, time: Res<Time>) {
    monitor.timer.tick(time.delta());
    if monitor.timer.just_finished() {
        monitor.sys.refresh_cpu();
        monitor.sys.refresh_memory();

        let cpus = monitor.sys.cpus();
        let usage_sum: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        monitor.cpu_usage = if !cpus.is_empty() {
            usage_sum / cpus.len() as f32
        } else {
            0.0
        };

        let total_mem = monitor.sys.total_memory();
        let used_mem = monitor.sys.used_memory();
        monitor.ram_usage = if total_mem > 0 {
            used_mem as f32 / total_mem as f32
        } else {
            0.0
        };
    }
}
