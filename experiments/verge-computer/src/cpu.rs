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
            match state.phase {
                CpuPhase::Fetch => {
                    if state.pc < program.0.len() {
                        state.phase = CpuPhase::Decode;
                    } else {
                        // Halt or loop?
                        // info!("CPU Halted (End of Program)");
                    }
                }
                CpuPhase::Decode => {
                    state.phase = CpuPhase::Execute;
                }
                CpuPhase::Execute => {
                    if let Some(instr) = program.0.get(state.pc) {
                        match instr {
                            Instruction::Load(reg, val) => {
                                if *reg < 4 {
                                    state.registers[*reg] = *val;
                                }
                                state.pc += 1;
                            }
                            Instruction::Add(dest, src) => {
                                if *dest < 4 && *src < 4 {
                                    state.registers[*dest] += state.registers[*src];
                                }
                                state.pc += 1;
                            }
                            Instruction::Mov(dest, src) => {
                                if *dest < 4 && *src < 4 {
                                    state.registers[*dest] = state.registers[*src];
                                }
                                state.pc += 1;
                            }
                            Instruction::Jmp(target) => {
                                state.pc = *target;
                            }
                            Instruction::Halt => {
                                // Do nothing, don't advance PC
                            }
                        }
                        state.instructions += 1;
                        info!("CPU Executed {:?}. Registers: {:?}", instr, state.registers);
                    }
                    state.phase = CpuPhase::Fetch;
                }
            }
        }
    }
}
