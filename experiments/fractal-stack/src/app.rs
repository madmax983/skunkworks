use crate::l_system::{LSystem, presets};
use crate::turtle::Turtle;

pub struct App {
    pub l_system: LSystem,
    pub expanded_string: String,
    pub turtle: Turtle,
    pub pc: usize,
    pub is_paused: bool,
    pub speed: u64, // iterations per tick
    pub generation: usize,
}

impl App {
    pub fn new() -> Self {
        let sys = presets::recursive_tree();
        let gen = 4;
        let expanded = sys.expand(gen);
        // Center the turtle roughly? No, bounds handle it.
        // Start at 0,0 pointing UP (-90 degrees is usually up in screen coords? No, y is down in TUI usually?
        // In Cartesian (Canvas), y increases Up. So 90 is up.
        // Ratatui Canvas: y increases UP.
        // So 90 degrees is Up.
        Self {
            l_system: sys,
            expanded_string: expanded,
            turtle: Turtle::new(0.0, 0.0, 90.0, 25.0, 5.0),
            pc: 0,
            is_paused: false,
            speed: 1,
            generation: gen,
        }
    }

    pub fn tick(&mut self) {
        if !self.is_paused {
            for _ in 0..self.speed {
                if self.pc < self.expanded_string.len() {
                    let c = self.expanded_string.chars().nth(self.pc).unwrap();
                    self.turtle.process_char(c);
                    self.pc += 1;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.turtle.reset(0.0, 0.0, 90.0);
    }
}
