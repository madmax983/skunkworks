use crate::sediment::SedimentParticle;
use macroquad::prelude::*;
use std::collections::HashMap;
use sysinfo::{Pid, System};

pub struct ProcessMonitor {
    pub system: System,
    pub known_pids: HashMap<Pid, Vec2>, // PID -> Position
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let mut known_pids = HashMap::new();

        for (pid, _) in system.processes() {
            known_pids.insert(
                *pid,
                vec2(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                ),
            );
        }

        Self { system, known_pids }
    }

    pub fn update(&mut self) -> Vec<SedimentParticle> {
        self.system.refresh_all(); // Refresh everything to get CPU usage
        let mut sediment = Vec::new();
        let mut current_pids = Vec::new();

        for (pid, process) in self.system.processes() {
            current_pids.push(*pid);

            if !self.known_pids.contains_key(pid) {
                // New process
                self.known_pids.insert(
                    *pid,
                    vec2(
                        rand::gen_range(0.0, screen_width()),
                        rand::gen_range(0.0, screen_height()),
                    ),
                );
            } else {
                // Existing process - shed dust if high CPU
                if process.cpu_usage() > 5.0 && rand::gen_range(0.0, 1.0) < 0.1 {
                    if let Some(&pos) = self.known_pids.get(pid) {
                        // Mass proportional to memory MB, but clamped
                        let mass = (process.memory() as f32 / 1024.0 / 1024.0).clamp(1.0, 10.0);
                        sediment.push(SedimentParticle::new(pos, mass));
                    }
                }
            }
        }

        // Detect dead processes
        let dead_pids: Vec<Pid> = self
            .known_pids
            .keys()
            .filter(|pid| !current_pids.contains(pid))
            .cloned()
            .collect();

        for pid in dead_pids {
            if let Some(pos) = self.known_pids.remove(&pid) {
                // Dead process becomes heavy sediment
                sediment.push(SedimentParticle::new(pos, 20.0)); // Big chunk
            }
        }

        // Update positions (brownian motion for living processes)
        for pos in self.known_pids.values_mut() {
            pos.x += rand::gen_range(-1.0, 1.0);
            pos.y += rand::gen_range(-1.0, 1.0);

            // Wrap
            if pos.x < 0.0 {
                pos.x = screen_width();
            }
            if pos.x > screen_width() {
                pos.x = 0.0;
            }
            if pos.y < 0.0 {
                pos.y = screen_height();
            }
            if pos.y > screen_height() {
                pos.y = 0.0;
            }
        }

        sediment
    }

    pub fn draw(&self) {
        for pos in self.known_pids.values() {
            draw_circle(pos.x, pos.y, 3.0, GREEN);
        }
    }
}
