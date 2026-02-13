use chimera_lang::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};
use sysinfo::Pid;

pub struct ProcessAgent {
    pub pid: Pid,
    pub vm: ChimeraVM,
    pub name: String,
    pub last_stats: (f32, u64), // CPU, Memory
}

impl ProcessAgent {
    pub fn new(pid: Pid, name: String, dna: Dna) -> Self {
        Self {
            pid,
            vm: ChimeraVM::new(dna),
            name,
            last_stats: (0.0, 0),
        }
    }

    pub fn update(&mut self, cpu: f32, memory: u64) {
        self.last_stats = (cpu, memory);

        // Inject stats into Grid
        // (0,0) = CPU Usage (as Int)
        // (0,1) = Memory Usage (as Int, MB maybe?)
        self.vm.grid[0][0] = Value::Int(cpu as i64);
        self.vm.grid[0][1] = Value::Int((memory / 1024 / 1024) as i64);

        // CPU drives metabolism: Higher CPU = More ticks per update
        let iterations = 1 + (cpu as u64 / 10).min(10);

        // Give energy if active
        if cpu > 1.0 {
            self.vm.energy = self.vm.energy.saturating_add(1);
        }

        for _ in 0..iterations {
            if !self.vm.halted {
                self.vm.step();
            }
        }
    }
}
