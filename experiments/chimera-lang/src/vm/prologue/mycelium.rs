use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use std::collections::VecDeque;

/// Applies Mycelium Runes during propagation phase.
pub fn apply_mycelium_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    mycelium_buffer: &mut VecDeque<Value>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "📥" => {
            // Inject: West (Signal) -> Mycelium Buffer
            // We just detect signal here. Pushing happens in Sink to avoid duplication?
            // Or if we want instant availability for 📤 in same tick?
            // If we push here, 📤 can read it in same tick IF processed later or in loop.
            // But iteration loop runs multiple times. We risk pushing multiple times.
            // So Pushing should be a Sink action (once per tick).
            // But Reading (📤) should be a Propagation action (available immediately).

            // So 📥 Propagation: Just acts as a wire/indicator?
            if w_sig.is_some() {
                 if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "📤" => {
            // Extract: Mycelium Buffer -> Self (Signal)
            // Reads from buffer front (Peek)
            if !mycelium_buffer.is_empty() {
                if let Some(val) = mycelium_buffer.front() {
                     if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(val.clone());
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }

    changes
}

/// Applies Mycelium Sinks (Side Effects).
pub fn apply_mycelium_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "📥" => {
            // Inject Sink: West (Signal) -> Push to Buffer
            if let Some(val) = w_sig {
                // Avoid flooding: Limit buffer size?
                if vm.prologue_state.mycelium_buffer.len() < 100 {
                    vm.prologue_state.mycelium_buffer.push_back(val.clone());
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
                    vm.output.push(format!("MYCELIUM: Injected {:?}", val));
                }
            }
        }
        "📤" => {
            // Extract Sink: Trigger (West) -> Pop Buffer -> South
            // NOTE: Propagation peeks. Sink pops (consumes).
            // Only pop if triggered by West signal?
            // If not triggered, it acts as a passive source (in propagation).
            // If triggered, it consumes the value and writes it to South.

            if w_sig.is_some() {
                if let Some(val) = vm.prologue_state.mycelium_buffer.pop_front() {
                     if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = val.clone();
                        vm.output.push(format!("MYCELIUM: Extracted {:?}", val));
                        vm.prologue_state.signal_grid[y][x] = Some(val);
                     }
                }
            }
        }
        "🍄" => {
            // Spore: Register in Network (Idempotent)
            vm.prologue_state.mycelium_network.insert((y, x));
        }
        "🦋" => {
            // Metamorphosis: West (Trigger) -> Mutate Agent at Location
            if w_sig.is_some() {
                // Check for agent at location
                let mut mutated = false;
                for agent in vm.prologue_state.agents.iter_mut() {
                    if agent.x == x && agent.y == y {
                        // Mutate agent state using buffer?
                        if let Some(dna) = vm.prologue_state.mycelium_buffer.front() {
                             // Apply DNA/Value to agent
                             agent.state = dna.clone();
                             mutated = true;
                             vm.output.push(format!("MYCELIUM: Metamorphosis of Agent at {},{}", x, y));
                        }
                    }
                }
                if mutated {
                     vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        _ => {}
    }
}
