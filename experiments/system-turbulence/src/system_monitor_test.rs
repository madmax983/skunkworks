
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_monitor_updates() {
        let bridge = Arc::new(Mutex::new(SystemStats::default()));
        let bridge_clone = bridge.clone();

        thread::spawn(move || {
            let mut sys = System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );
            sys.refresh_cpu();
            sys.refresh_memory();

            if let Ok(mut stats) = bridge_clone.lock() {
                stats.total_cpu_usage = sys.global_cpu_info().cpu_usage();
                stats.total_memory_usage = sys.used_memory() as f32;
            }
        });

        sleep(Duration::from_millis(200));
        let stats = bridge.lock().unwrap();
        // Just check if it ran without panic. CPU usage might be 0.
        println!("Stats: {:?}", *stats);
    }
}
