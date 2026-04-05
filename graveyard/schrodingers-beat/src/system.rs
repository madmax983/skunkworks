use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn spawn_monitor() -> Arc<AtomicU8> {
    let cpu_load = Arc::new(AtomicU8::new(0));
    let cpu_load_clone = cpu_load.clone();

    thread::spawn(move || {
        let mut sys =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

        // Initial sleep to establish baseline for CPU usage calculation
        thread::sleep(Duration::from_millis(200));

        loop {
            sys.refresh_cpu();
            let load = sys.global_cpu_info().cpu_usage(); // 0.0 to 100.0
            cpu_load_clone.store(load as u8, Ordering::Relaxed);
            thread::sleep(Duration::from_millis(100));
        }
    });

    cpu_load
}
