use crate::vm::{ChimeraVM, Value};
use super::normalize_coords;

pub fn apply_symbiosis_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
    } else {
        None
    };

    match rune {
        "u" => {
            // Upload: West (Value) -> Push to Stack
            if let Some(val) = w_sig {
                vm.stack.push(val.clone());
                vm.output.push(format!("SYMBIOSIS: Uploaded {:?}", val));
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
            }
        }
        "y" => {
            // Yank: West (Trigger) -> Pop from Stack -> Write South
            if w_sig.is_some() {
                if let Some(val) = vm.stack.pop() {
                     if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = val.clone();
                        vm.prologue_state.signal_grid[y][x] = Some(val.clone()); // Light up with value
                        vm.output.push(format!("SYMBIOSIS: Yanked {:?}", val));
                    }
                } else {
                    vm.output.push("SYMBIOSIS: Stack Underflow on Yank".to_string());
                }
            }
        }
        #[cfg(feature = "nova")]
        "w" => {
            // Write Akashic: West (Value), North (Key) -> Akashic
            if let (Some(val), Some(Value::Str(key))) = (w_sig, n_sig) {
                vm.akashic.storage.insert(key.clone(), val.clone());
                if let Err(e) = vm.akashic.save() {
                    vm.output.push(format!("SYMBIOSIS ERROR: {}", e));
                } else {
                    vm.output.push(format!("SYMBIOSIS: Wrote Akashic '{}'", key));
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        #[cfg(feature = "nova")]
        "j" => {
            // Join (Read) Akashic: North (Key) -> Grid (South)
            if let Some(Value::Str(key)) = n_sig {
                if let Some(val) = vm.akashic.storage.get(&key) {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = val.clone();
                        vm.prologue_state.signal_grid[y][x] = Some(val.clone());
                        vm.output.push(format!("SYMBIOSIS: Read Akashic '{}'", key));
                    }
                } else {
                    vm.output.push(format!("SYMBIOSIS: Akashic Key '{}' not found", key));
                }
            }
        }
        _ => {}
    }
}
