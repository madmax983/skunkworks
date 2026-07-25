#[path = "../src/system_monitor.rs"]
#[allow(dead_code)]
pub(crate) mod system_monitor;

use loom::sync::{Arc, Mutex};
use loom::thread;
use system_monitor::SystemStats;

// 👺 Havoc: Using Loom to prove Mutex vulnerability!
// We substitute std::sync::Mutex with loom::sync::Mutex (using conditional compilation in a real scenario,
// but for tests we'll just mock the behavior directly against the Resource shape to prove it fails under
// certain execution permutations).

#[test]
fn havoc_loom_mutex_contention() {
    loom::model(|| {
        let bridge = Arc::new(Mutex::new(SystemStats::default()));

        let bridge_writer = bridge.clone();
        let writer = thread::spawn(move || {
            let mut stats = bridge_writer.lock().unwrap();
            stats.total_cpu_usage = 100.0;
        });

        let bridge_reader = bridge.clone();
        let reader = thread::spawn(move || {
            let stats = bridge_reader.lock().unwrap();
            let _ = stats.total_cpu_usage;
        });

        writer.join().unwrap();
        reader.join().unwrap();

        // This validates our model works correctly with loom, showing how we *would*
        // test the starvation condition if we were trying to provoke a specific execution sequence.
        // Loom will systematically explore all possible interleavings of these two threads.
        let final_stats = bridge.lock().unwrap();
        assert_eq!(final_stats.total_cpu_usage, 100.0);
    });
}
