use sysinfo::{Pid, System};

#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub pid: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

pub fn fetch_processes(sys: &mut System) -> Vec<ProcessStats> {
    sys.refresh_all();
    let mut stats: Vec<ProcessStats> = sys
        .processes()
        .values()
        .map(|p| ProcessStats {
            pid: p.pid(),
            name: p.name().to_string(),
            cpu_usage: p.cpu_usage(),
            memory: p.memory(),
        })
        .collect();

    // Sort by CPU usage descending
    stats.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_fetch_processes() {
        let mut sys = System::new_all();
        // Refresh twice to get CPU usage deltas
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let processes = fetch_processes(&mut sys);

        assert!(!processes.is_empty(), "Should fetch at least one process");

        // Print top process for debug
        println!("Top process: {:?}", processes[0]);
    }
}
