use crate::process::{Process, ProcessState};
use market_sim::{Grid, Particle};
use rand::Rng;
use std::fs;
use std::path::Path;

pub struct Simulation {
    pub market: Grid,
    pub processes: Vec<Process>,
    pub history: Vec<f64>, // Price history
    pub cpu_capacity: usize, // Cycles per tick
    pub tick_count: usize,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Self {
            market: Grid::new(width, height),
            processes: Vec::new(),
            history: Vec::new(),
            cpu_capacity: width / 2, // Default capacity
            tick_count: 0,
        };
        sim.scan_processes();
        sim
    }

    fn scan_processes(&mut self) {
        let mut id = 1;
        let mut rng = rand::thread_rng();

        if let Ok(entries) = fs::read_dir("experiments") {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let work = rng.gen_range(100..1000);
                        let credits = rng.gen_range(50.0..500.0);
                        let column = id % self.market.width;

                        self.processes.push(Process::new(id, name, work, credits, column));
                        id += 1;

                        if id >= self.market.width * 2 { break; }
                    }
                }
            }
        }

        if self.processes.is_empty() {
             for i in 0..self.market.width {
                self.processes.push(Process::new(
                    i + 1,
                    format!("kworker/{}", i),
                    500,
                    100.0,
                    i % self.market.width
                ));
             }
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.tick_count += 1;

        // 1. CPU Spawning (Supply) - Asks start at Top (0) and move Down
        for _ in 0..self.cpu_capacity {
            let col = rng.gen_range(0..self.market.width);
            if matches!(self.market.get(col, 0), Particle::Empty) {
                // println!("Spawning Ask at col {}", col);
                self.market.set(col, 0, Particle::Ask(usize::MAX));
            }
        }

        // 2. Agent Spidding (Demand) - Bids start at Bottom (Height-1) and move Up
        for process in &mut self.processes {
            if process.work_remaining > 0 && process.credits > 1.0 {
                if matches!(self.market.get(process.column, self.market.height - 1), Particle::Empty) {
                     // println!("Spawning Bid for process {} at col {}", process.id, process.column);
                     self.market.set(process.column, self.market.height - 1, Particle::Bid(process.id));
                } else {
                     // println!("Col {} blocked for process {}", process.column, process.id);
                }
            }
        }

        // 3. Market Update
        let events = self.market.update();

        // 4. Settlement
        for event in events {
            if let Some(process) = self.processes.iter_mut().find(|p| p.id == event.buyer) {
                process.credits -= event.price;
                if process.work_remaining > 0 {
                    process.work_remaining -= 1;
                }
                process.credits += 0.5;
            }
            self.history.push(event.price as f64);
        }

        if self.history.len() > 200 {
            self.history.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_execution() {
        let mut sim = Simulation::new(10, 20);
        sim.processes.clear();
        sim.processes.push(Process::new(1, "TestProc".to_string(), 10, 100.0, 0));

        sim.cpu_capacity = 20; // OVERKILL CPU

        for _ in 0..100 {
            sim.update();
        }

        let proc = &sim.processes[0];
        assert!(proc.work_remaining < 10, "Process work should decrease after trading");
        assert!(proc.credits < 100.0, "Process credits should decrease after trading");
    }
}
