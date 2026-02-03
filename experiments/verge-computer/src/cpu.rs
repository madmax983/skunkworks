use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuPhase {
    Fetch,
    Decode,
    Execute,
}

#[derive(Component)]
pub struct CpuState {
    pub pc: usize, // Program Counter
    pub phase: CpuPhase,
    pub instructions: usize, // Total executed
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            pc: 0,
            phase: CpuPhase::Fetch,
            instructions: 0,
        }
    }
}

// Event triggered by the escapement tick
#[derive(Event)]
pub struct TickEvent;

pub fn cpu_tick_system(mut cpu_query: Query<&mut CpuState>, mut events: EventReader<TickEvent>) {
    for _ in events.read() {
        for mut state in &mut cpu_query {
            match state.phase {
                CpuPhase::Fetch => {
                    state.phase = CpuPhase::Decode;
                    // info!("CPU Phase: Decode");
                }
                CpuPhase::Decode => {
                    state.phase = CpuPhase::Execute;
                    // info!("CPU Phase: Execute");
                }
                CpuPhase::Execute => {
                    state.phase = CpuPhase::Fetch;
                    state.pc += 1;
                    state.instructions += 1;
                    info!(
                        "CPU Executed Instruction {}. PC: {}",
                        state.instructions, state.pc
                    );
                }
            }
        }
    }
}
