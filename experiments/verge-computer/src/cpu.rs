use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuPhase {
    Fetch,
    Decode,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Load(usize, i32),  // Reg, Value
    Add(usize, usize), // Dest, Src (Dest += Src)
    Mov(usize, usize), // Dest, Src (Dest = Src)
    Jmp(usize),        // Target PC
    Halt,
}

#[derive(Resource, Default)]
pub struct Program(pub Vec<Instruction>);

#[derive(Component)]
pub struct CpuState {
    pub pc: usize,
    pub phase: CpuPhase,
    pub instructions: usize,
    pub registers: [i32; 4],
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            pc: 0,
            phase: CpuPhase::Fetch,
            instructions: 0,
            registers: [0; 4],
        }
    }
}

impl CpuState {
    pub fn tick(&mut self, program: &Program) {
        match self.phase {
            CpuPhase::Fetch => {
                if self.pc < program.0.len() {
                    self.phase = CpuPhase::Decode;
                }
            }
            CpuPhase::Decode => {
                self.phase = CpuPhase::Execute;
            }
            CpuPhase::Execute => {
                if let Some(instr) = program.0.get(self.pc) {
                    self.execute_instruction(instr);
                    self.instructions += 1;
                    info!("CPU Executed {:?}. Registers: {:?}", instr, self.registers);
                }
                self.phase = CpuPhase::Fetch;
            }
        }
    }

    fn execute_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Load(reg, val) => {
                if *reg < 4 {
                    self.registers[*reg] = *val;
                }
                self.pc += 1;
            }
            Instruction::Add(dest, src) => {
                if *dest < 4 && *src < 4 {
                    self.registers[*dest] += self.registers[*src];
                }
                self.pc += 1;
            }
            Instruction::Mov(dest, src) => {
                if *dest < 4 && *src < 4 {
                    self.registers[*dest] = self.registers[*src];
                }
                self.pc += 1;
            }
            Instruction::Jmp(target) => {
                self.pc = *target;
            }
            Instruction::Halt => {
                // Do nothing, don't advance PC
            }
        }
    }
}

// Event triggered by the escapement tick
#[derive(Event)]
pub struct TickEvent;

pub fn cpu_tick_system(
    mut cpu_query: Query<&mut CpuState>,
    mut events: EventReader<TickEvent>,
    program: Res<Program>,
) {
    for _ in events.read() {
        for mut state in &mut cpu_query {
            state.tick(&program);
        }
    }
}
