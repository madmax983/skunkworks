use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked, // Waiting for IO / Starved
    Zombie,  // Finished but not reaped
}

#[derive(Clone, Debug)]
pub struct ProcessTree {
    pub id: usize,
    pub priority: u8, // 0-255, higher is better
    pub cpu_needed: f32, // Total height needed to finish
    pub progress: f32, // Current height
    pub state: ProcessState,
    pub pos: f32, // X position
    pub width: f32,
    pub color: Color,
    pub creation_time: f64,
    pub last_scheduled: f64,
}

impl ProcessTree {
    pub fn new(id: usize, pos: f32, width: f32, priority: u8, cpu_needed: f32, creation_time: f64) -> Self {
        Self {
            id,
            priority,
            cpu_needed,
            progress: 0.0,
            state: ProcessState::Ready,
            pos,
            width,
            color: Self::priority_color(priority),
            creation_time,
            last_scheduled: 0.0,
        }
    }

    fn priority_color(priority: u8) -> Color {
        // Map priority to green/brown spectrum
        // High priority -> Bright Green
        // Low priority -> Brownish Green
        let t = priority as f32 / 255.0;
        Color::new(
            0.4 - t * 0.2,
            0.4 + t * 0.6,
            0.1 + t * 0.1,
            1.0,
        )
    }

    pub fn grow(&mut self, amount: f32) {
        if self.state == ProcessState::Running {
            self.progress += amount;
            if self.progress >= self.cpu_needed {
                self.progress = self.cpu_needed;
                self.state = ProcessState::Zombie;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SchedulingAlgorithm {
    RoundRobin,
    FCFS, // First Come First Served
    Priority, // Highest Priority
    ShortestJobFirst,
}

#[derive(Debug, PartialEq)]
pub enum SchedulerEvent {
    ContextSwitch { from: Option<usize>, to: Option<usize> },
    ProcessTick { pid: usize, pos: f32, priority: u8 },
}

pub struct Scheduler {
    pub processes: Vec<ProcessTree>,
    pub current_process_idx: Option<usize>,
    pub algorithm: SchedulingAlgorithm,
    pub quantum: f32, // Time slice length
    pub time_left: f32, // Remaining time in slice
    pub sun_pos: f32, // Visual position of the "Sun" (Listener)
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_process_idx: None,
            algorithm: SchedulingAlgorithm::RoundRobin,
            quantum: 1.0, // 1 second quantum
            time_left: 0.0,
            sun_pos: 0.0,
        }
    }

    pub fn add_process(&mut self, process: ProcessTree) {
        self.processes.push(process);
    }

    pub fn kill_current(&mut self) {
        if let Some(idx) = self.current_process_idx {
             self.processes.remove(idx);
             self.current_process_idx = None;
             self.time_left = 0.0;
        }
    }

    pub fn update(&mut self, dt: f32, current_time: f64) -> Vec<SchedulerEvent> {
        let mut events = Vec::new();

        // Update time left in quantum
        if self.current_process_idx.is_some() {
            self.time_left -= dt;
        }

        // Check if we need to switch context
        let current_state_is_zombie = if let Some(idx) = self.current_process_idx {
             // Handle case where index might be out of bounds if killed?
             // But kill_current sets current_process_idx to None.
             if idx < self.processes.len() {
                 self.processes[idx].state == ProcessState::Zombie
             } else {
                 true
             }
        } else {
            false
        };

        let need_switch = self.current_process_idx.is_none()
            || self.time_left <= 0.0
            || current_state_is_zombie;

        if need_switch {
            let from = self.current_process_idx;
            self.schedule_next();
            let to = self.current_process_idx;

            if from != to {
                events.push(SchedulerEvent::ContextSwitch { from, to });
            }
        }

        // Grow the current process
        if let Some(idx) = self.current_process_idx {
             if idx < self.processes.len() {
                let process = &mut self.processes[idx];
                if process.state == ProcessState::Running {
                    process.grow(dt * 10.0); // Growth speed
                    process.last_scheduled = current_time;

                    // Emit Tick Event
                    events.push(SchedulerEvent::ProcessTick {
                        pid: process.id,
                        pos: process.pos,
                        priority: process.priority,
                    });

                    // Update Sun/Listener Position
                    let target_sun = process.pos + process.width / 2.0;
                    self.sun_pos += (target_sun - self.sun_pos) * 5.0 * dt;
                }
             } else {
                 self.current_process_idx = None;
             }
        }

        events
    }

    fn schedule_next(&mut self) {
        // Reset state of current running process
        if let Some(idx) = self.current_process_idx {
            if idx < self.processes.len() && self.processes[idx].state == ProcessState::Running {
                self.processes[idx].state = ProcessState::Ready;
            }
        }

        if self.processes.is_empty() {
            self.current_process_idx = None;
            return;
        }

        match self.algorithm {
            SchedulingAlgorithm::RoundRobin => {
                let start_idx = self.current_process_idx.map(|i| i + 1).unwrap_or(0);
                let mut found = None;
                for i in 0..self.processes.len() {
                    let idx = (start_idx + i) % self.processes.len();
                    if self.processes[idx].state != ProcessState::Zombie {
                        found = Some(idx);
                        break;
                    }
                }
                self.current_process_idx = found;
            }
            SchedulingAlgorithm::FCFS => {
                 let mut found = None;
                for (i, p) in self.processes.iter().enumerate() {
                    if p.state != ProcessState::Zombie {
                        found = Some(i);
                        break;
                    }
                }
                self.current_process_idx = found;
                self.time_left = 9999.0;
                if let Some(idx) = found {
                    self.processes[idx].state = ProcessState::Running;
                }
                return;
            }
             SchedulingAlgorithm::Priority => {
                let mut best_idx = None;
                let mut max_prio = 0;

                for (i, p) in self.processes.iter().enumerate() {
                    if p.state != ProcessState::Zombie {
                        if best_idx.is_none() || p.priority > max_prio {
                            max_prio = p.priority;
                            best_idx = Some(i);
                        }
                    }
                }
                self.current_process_idx = best_idx;
            }
            SchedulingAlgorithm::ShortestJobFirst => {
                let mut best_idx = None;
                let mut min_remaining = f32::MAX;

                for (i, p) in self.processes.iter().enumerate() {
                    if p.state != ProcessState::Zombie {
                        let remaining = p.cpu_needed - p.progress;
                        if remaining < min_remaining {
                            min_remaining = remaining;
                            best_idx = Some(i);
                        }
                    }
                }
                self.current_process_idx = best_idx;
            }
        }

        if let Some(idx) = self.current_process_idx {
            self.processes[idx].state = ProcessState::Running;
            self.time_left = self.quantum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_events() {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(ProcessTree::new(0, 0.0, 10.0, 10, 100.0, 0.0));
        scheduler.add_process(ProcessTree::new(1, 0.0, 10.0, 10, 100.0, 0.0));

        // Initial update - should produce a ContextSwitch
        let events = scheduler.update(0.1, 0.1);

        let has_switch = events.iter().any(|e| matches!(e, SchedulerEvent::ContextSwitch { .. }));
        assert!(has_switch, "Should produce ContextSwitch on first run");

        let has_tick = events.iter().any(|e| matches!(e, SchedulerEvent::ProcessTick { .. }));
        assert!(has_tick, "Should produce ProcessTick");
    }

    #[test]
    fn test_context_switch_round_robin() {
        let mut scheduler = Scheduler::new();
        scheduler.quantum = 0.5;
        scheduler.add_process(ProcessTree::new(0, 0.0, 10.0, 10, 100.0, 0.0));
        scheduler.add_process(ProcessTree::new(1, 0.0, 10.0, 10, 100.0, 0.0));

        // Run Process 0
        scheduler.update(0.1, 0.1);
        assert_eq!(scheduler.current_process_idx, Some(0));

        // Exhaust quantum
        scheduler.time_left = 0.0;
        let events = scheduler.update(0.1, 0.2);

        // Should switch to Process 1
        assert_eq!(scheduler.current_process_idx, Some(1));

        // Verify event
        let switch_event = events.iter().find(|e| matches!(e, SchedulerEvent::ContextSwitch { .. }));
        assert!(switch_event.is_some());

        if let Some(SchedulerEvent::ContextSwitch { from, to }) = switch_event {
            assert_eq!(*from, Some(0));
            assert_eq!(*to, Some(1));
        }
    }
}
