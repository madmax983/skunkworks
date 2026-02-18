use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Resource, Default, Debug, Clone)]
pub struct SystemStats {
    pub total_cpu_usage: f32,
    pub total_memory_usage: f32,
    // Top processes: (PID, Name, CPU%, Memory%)
    pub top_processes: Vec<(u32, String, f32, u64)>,
}

pub struct SystemMonitorPlugin;

impl Plugin for SystemMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SystemStats::default())
            .add_systems(Startup, start_system_monitor)
            .add_systems(Update, update_system_stats);
    }
}

// Shared state between thread and Bevy system
#[derive(Resource, Clone)]
struct MonitorBridge(Arc<Mutex<SystemStats>>);

fn start_system_monitor(mut commands: Commands) {
    let bridge = Arc::new(Mutex::new(SystemStats::default()));
    let bridge_clone = bridge.clone();

    // Spawn background thread
    thread::spawn(move || {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );

        loop {
            // Refresh system stats
            sys.refresh_cpu();
            sys.refresh_memory();
            sys.refresh_processes();

            let total_cpu = sys.global_cpu_info().cpu_usage();
            let total_mem = sys.used_memory() as f32 / sys.total_memory() as f32;

            // Collect processes
            let mut processes: Vec<_> = sys
                .processes()
                .iter()
                .map(|(pid, process)| {
                    (
                        pid.as_u32(),
                        process.name().to_string(),
                        process.cpu_usage(),
                        process.memory(),
                    )
                })
                .collect();

            // Sort by CPU usage descending and take top 50
            processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
            processes.truncate(50);

            // Update shared state
            if let Ok(mut stats) = bridge_clone.lock() {
                stats.total_cpu_usage = total_cpu;
                stats.total_memory_usage = total_mem;
                stats.top_processes = processes;
            }

            thread::sleep(Duration::from_millis(100));
        }
    });

    commands.insert_resource(MonitorBridge(bridge));
}

fn update_system_stats(bridge: Res<MonitorBridge>, mut stats: ResMut<SystemStats>) {
    if let Ok(latest) = bridge.0.try_lock() {
        *stats = latest.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_system_stats_update() {
        let bridge = Arc::new(Mutex::new(SystemStats::default()));
        let bridge_clone = bridge.clone();

        thread::spawn(move || {
            let mut sys = System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );

            // First refresh might be 0 for CPU
            sys.refresh_cpu();
            sys.refresh_memory();
            thread::sleep(Duration::from_millis(200));
            sys.refresh_cpu();
            sys.refresh_memory();
            sys.refresh_processes();

            if let Ok(mut stats) = bridge_clone.lock() {
                stats.total_cpu_usage = sys.global_cpu_info().cpu_usage();
                stats.total_memory_usage = sys.used_memory() as f32 / sys.total_memory() as f32;
                stats.top_processes = sys.processes().iter().take(5).map(|(pid, p)| (pid.as_u32(), p.name().to_string(), p.cpu_usage(), p.memory())).collect();
            }
        });

        thread::sleep(Duration::from_secs(1));
        let stats = bridge.lock().unwrap();
        println!("Test Stats: {:?}", *stats);
        assert!(stats.total_memory_usage > 0.0, "Memory usage should be > 0");
    }
}
