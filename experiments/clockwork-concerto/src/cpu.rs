use bevy::prelude::*;
use crate::mechanism::TickEvent;

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
    Note(u8),          // Play Note
    Jmp(usize),        // Target PC
}

#[derive(Resource, Default)]
pub struct Program(pub Vec<Instruction>);

#[derive(Event)]
pub struct NoteEvent(pub u8);

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

pub struct CpuPlugin;

impl Plugin for CpuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NoteEvent>()
           .init_resource::<Program>()
           .add_systems(Update, cpu_tick_system);
    }
}

pub fn cpu_tick_system(
    mut cpu_query: Query<&mut CpuState>,
    mut events: EventReader<TickEvent>,
    program: Res<Program>,
    mut note_events: EventWriter<NoteEvent>,
) {
    for _ in events.read() {
        for mut state in &mut cpu_query {
            match state.phase {
                CpuPhase::Fetch => {
                    if state.pc < program.0.len() {
                        state.phase = CpuPhase::Decode;
                    } else {
                        // Loop program for music box behavior
                        state.pc = 0;
                        state.phase = CpuPhase::Fetch;
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
                            Instruction::Note(note) => {
                                note_events.send(NoteEvent(*note));
                                state.pc += 1;
                            }
                            Instruction::Jmp(target) => {
                                state.pc = *target;
                            }
                        }
                        state.instructions += 1;
                        // info!("Exec: {:?} -> Regs: {:?}", instr, state.registers);
                    }
                    state.phase = CpuPhase::Fetch;
                }
            }
        }
    }
}
