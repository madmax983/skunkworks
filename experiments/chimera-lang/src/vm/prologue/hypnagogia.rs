use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

/// Updates the Hypnagogia state based on runes.
pub fn apply_hypnagogia_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    dream_intensity: &mut f32,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "☾" => {
            // Moon (Dream Anchor): Input West increases Dream Intensity
            if let Some(Value::Int(v)) = w_sig {
                if v > 0 {
                    // Only process once per tick (when we haven't emitted yet)
                    if next_signals[y][x].is_none() {
                        *dream_intensity += v as f32 * 0.5; // Gain intensity
                        if *dream_intensity > 100.0 {
                            *dream_intensity = 100.0;
                        }
                        // Emits current intensity
                        next_signals[y][x] = Some(Value::Int(*dream_intensity as i64));
                        changes = true;
                    }
                }
            } else {
                 // Passive decay if no input? Or maintain?
                 // Let's maintain for now, decay happens globally maybe.
            }
        }
        "☀" => {
            // Sun (Awaken): Input West wakes up (resets intensity)
            if w_sig.is_some() {
                *dream_intensity = 0.0;
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(0));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}

/// Applies global dream effects if intensity is high.
/// This should be called once per tick in `exec_prologue_tick`.
pub fn process_dream_logic(vm: &mut ChimeraVM) {
    if vm.prologue_state.dream_intensity <= 0.0 {
        return;
    }

    // Decay naturally
    vm.prologue_state.dream_intensity -= 0.1;
    if vm.prologue_state.dream_intensity < 0.0 {
        vm.prologue_state.dream_intensity = 0.0;
    }

    let intensity = vm.prologue_state.dream_intensity;
    let mut rng = rand::thread_rng();

    // Level 1: Lucidity (Low) - No negative effects, just visual or minor
    if intensity > 20.0 {
        // Occasional "Spark" (Visual only, implemented as ephemeral signal)
        if rng.gen_bool(0.05) {
             let ry = rng.gen_range(0..crate::vm::GRID_SIZE);
             let rx = rng.gen_range(0..crate::vm::GRID_SIZE);
             if vm.prologue_state.signal_grid[ry][rx].is_none() {
                 vm.prologue_state.signal_grid[ry][rx] = Some(Value::Int(1)); // Spark
             }
        }
    }

    // Level 2: REM (Medium) - Structural Changes (Wires)
    if intensity > 50.0 {
        if rng.gen_bool(0.02) { // 2% chance per tick
            let ry = rng.gen_range(0..crate::vm::GRID_SIZE);
            let rx = rng.gen_range(0..crate::vm::GRID_SIZE);
            // Spawn a wire in empty space
            if matches!(vm.grid[ry][rx], Value::Int(0)) {
                 vm.grid[ry][rx] = Value::Str("~".to_string());
                 vm.output.push(format!("HYPNAGOGIA: Dream wire manifested at {},{}", rx, ry));
            }
        }
    }

    // Level 3: Deep Sleep / Nightmare (High) - Chaos Agents
    if intensity > 80.0 {
        if rng.gen_bool(0.01) { // 1% chance per tick
            let ry = rng.gen_range(0..crate::vm::GRID_SIZE);
            let rx = rng.gen_range(0..crate::vm::GRID_SIZE);
            // Spawn Chaos Agent
            if matches!(vm.grid[ry][rx], Value::Int(0)) {
                 vm.grid[ry][rx] = Value::Str("K".to_string());
                 vm.prologue_state.agents.push(super::PrologueAgent {
                     x: rx,
                     y: ry,
                     state: Value::Int(0),
                 });
                 vm.output.push(format!("HYPNAGOGIA: Nightmare manifested at {},{}", rx, ry));
            }
        }
    }
}
