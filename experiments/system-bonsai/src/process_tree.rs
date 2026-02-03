use sysinfo::{Pid, Process, System};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ProcessNode {
    pub pid: Pid,
    #[allow(dead_code)]
    pub name: String,
    pub memory: u64,
    pub cpu_usage: f32,
    pub children: Vec<ProcessNode>,
}

pub struct ProcessTree {
    pub roots: Vec<ProcessNode>,
    pub total_processes: usize,
}

impl ProcessTree {
    pub fn new(sys: &System) -> Self {
        let processes = sys.processes();
        let mut adj: HashMap<Pid, Vec<Pid>> = HashMap::new();
        let mut pids: HashSet<Pid> = HashSet::new();

        // 1. Build Adjacency List
        for (pid, process) in processes {
            pids.insert(*pid);
            if let Some(parent_pid) = process.parent() {
                // Only add if parent exists in our snapshot
                if processes.contains_key(&parent_pid) {
                    adj.entry(parent_pid).or_default().push(*pid);
                }
            }
        }

        // 2. Find Roots (nodes with no parent in the set)
        let mut roots = Vec::new();
        for (pid, process) in processes {
            let is_root = match process.parent() {
                Some(parent_pid) => !processes.contains_key(&parent_pid),
                None => true,
            };

            if is_root {
                roots.push(Self::build_node(*pid, processes, &adj));
            }
        }

        // Sort roots by PID for stability
        roots.sort_by_key(|n| n.pid);

        Self {
            roots,
            total_processes: processes.len(),
        }
    }

    fn build_node(
        pid: Pid,
        processes: &HashMap<Pid, Process>,
        adj: &HashMap<Pid, Vec<Pid>>,
    ) -> ProcessNode {
        let process = &processes[&pid];

        let mut children = Vec::new();
        if let Some(child_pids) = adj.get(&pid) {
            for child_pid in child_pids {
                children.push(Self::build_node(*child_pid, processes, adj));
            }
        }

        // Sort children by CPU usage (descending), then Memory
        children.sort_by(|a, b| {
            b.cpu_usage.partial_cmp(&a.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.memory.cmp(&a.memory))
        });

        ProcessNode {
            pid,
            name: process.name().to_string(),
            memory: process.memory(),
            cpu_usage: process.cpu_usage(),
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_tree_construction() {
        let mut sys = System::new_all();
        sys.refresh_all();

        let tree = ProcessTree::new(&sys);

        assert!(tree.total_processes > 0);
        assert!(!tree.roots.is_empty());
    }
}
