use crate::vm::{ChimeraVM, Value};
use crate::vm::prologue::{PrologueAgent, normalize_coords};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperaState {
    pub active: bool,
    pub tempo: u64, // BPM
    pub key: i64,   // 0=C, 1=C#, etc.
    pub history: VecDeque<String>,
}

impl OperaState {
    pub fn new() -> Self {
        Self {
            active: true,
            tempo: 120,
            key: 0,
            history: VecDeque::new(),
        }
    }
}

pub fn apply_opera_runes(
    rune: &str,
    y: usize,
    x: usize,
    opera_state: &mut OperaState,
    signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    // Read from signals (West)
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "𝄇" => {
             // Repeat: If signal from West, emit to East (Self for now)
             if let Some(sig) = w_sig {
                 signals[y][x] = Some(sig);
                 opera_state.history.push_back("Loop".to_string());
                 return true;
             }
        }
        "♯" => {
            // Modulate Up on Signal
            if w_sig.is_some() {
                opera_state.key += 1;
                opera_state.history.push_back(format!("Key Change: {}", opera_state.key));
                signals[y][x] = Some(Value::Int(opera_state.key));
                return true;
            }
        }
        "♭" => {
            // Modulate Down on Signal
            if w_sig.is_some() {
                opera_state.key -= 1;
                signals[y][x] = Some(Value::Int(opera_state.key));
                return true;
            }
        }
        "♮" => {
            // Reset Key on Signal
            if w_sig.is_some() {
                opera_state.key = 0;
                signals[y][x] = Some(Value::Int(0));
                return true;
            }
        }
        _ => {}
    }
    false
}

pub fn process_conductor_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    // Conductor 𝄞 moves East.
    let (y, x) = (agent.y, agent.x);

    // Emit Signal (Presence) to trigger Runes
    // Write to delayed_signals so it propagates in the next tick
    vm.prologue_state.delayed_signals[y][x] = Some(Value::Int(1));

    let mut updated_agent = agent.clone();

    // 1. Scan Scan Scan
    // Check neighbors for Critters to "Scan"
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == "C" {
                    // Extract DNA from register
                    // We need access to registers, which are in vm.prologue_state
                    // But we have mutable vm access.
                    // However, accessing registers might be tricky if we are in process_agents which iterates agents.
                    // process_agents holds a copy of agents, but we have mutable access to VM.

                    // Let's just create a visual effect for now
                    vm.output.push(format!("OPERA: Conductor scanning lifeform at {},{}", nx, ny));

                    // Add to history
                    vm.prologue_state.opera_state.history.push_back(format!("Scanned C at {},{}", nx, ny));
                    if vm.prologue_state.opera_state.history.len() > 10 {
                        vm.prologue_state.opera_state.history.pop_front();
                    }
                }
            }
        }
    }

    // 2. Play Cell
    if let Value::Int(n) = grid_snapshot[y][x] {
        if n > 0 {
            let key = vm.prologue_state.opera_state.key;
            let note = n + key;
            let msg = format!("Note: {}", note);

            // Avoid duplicates
            if vm.prologue_state.opera_state.history.back() != Some(&msg) {
                vm.prologue_state.opera_state.history.push_back(msg);
                if vm.prologue_state.opera_state.history.len() > 10 {
                    vm.prologue_state.opera_state.history.pop_front();
                }
            }

            #[cfg(feature = "resonance")]
            if let Some(tx) = &vm.audio_tx {
                use resonance_audio::audio::AudioCommand;
                // MIDI to Freq
                let freq = 440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0);
                let _ = tx.send(AudioCommand::Tone {
                    x,
                    y,
                    frequency: freq,
                    strength: 0.5,
                    duration_ms: 200,
                });
            }
        }
    }

    // 3. Move East
    let next_x = (x + 1) % crate::vm::GRID_SIZE;
    let next_y = y; // Stay on line

    // Check if blocked? Conductor is ghost-like, passes through?
    // Let's say it passes through everything except edge (wraps).
    // But process_agents logic requires us to return target.
    // If target is occupied, it might overwrite or be blocked.
    // Let's check grid_snapshot.

    // If next cell is another agent, we might collide.
    // But Conductor is "Ethereal".
    // We'll move anyway. The main process_agents handles collision by overwriting or special logic.
    // If we overwrite, we kill.
    // Let's try to avoid killing.

    let dest_val = &grid_snapshot[next_y][next_x];
    if matches!(dest_val, Value::Int(0)) || matches!(dest_val, Value::Str(s) if s == ".") {
         updated_agent.x = next_x;
         updated_agent.y = next_y;
         return Some((updated_agent, Some((next_y, next_x))));
    } else if let Value::Str(s) = dest_val {
        if s == "𝄞" {
             // Another conductor? Block.
             return Some((updated_agent, None));
        }
        // Pass through runes, but don't erase them?
        // process_agents sets new pos grid to agent char.
        // So we erase whatever was there.
        // We should only move if empty.

        // Wrap around?
        // If blocked, just wait.
        return Some((updated_agent, None));
    }

    // Wrap around logic handles x+1 % size.
    // So if 15 -> 0.

    updated_agent.x = next_x;
    updated_agent.y = next_y;
    Some((updated_agent, Some((next_y, next_x))))
}
