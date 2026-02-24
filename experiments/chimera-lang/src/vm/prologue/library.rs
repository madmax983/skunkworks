use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

/// Applies Library Sinks (Side Effects).
///
/// Runes:
/// *   `📖` (Book) - **Library Access**:
///     *   Mode 0 (Read): Reads West (Key). Emits Value to Self.
///     *   Mode 1 (Write): Reads West (Key) + South (Value). Writes to Library.
///     *   Mode 2 (Exec): Reads West (Key). Compiles and Executes as Strand.
/// *   `📚` (Shelf) - **Browse**:
///     *   Reads West (Index). Emits Key at Index to Self.
/// *   `🔖` (Bookmark) - **Pointer**:
///     *   Reads West (Key). Emits Reference (String) to Self.
pub fn apply_library_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "📖" => apply_book_rune(vm, y, x),
        "📚" => apply_shelf_rune(vm, y, x),
        "🔖" => apply_bookmark_rune(vm, y, x),
        _ => {}
    }
}

fn apply_book_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Key (String)
    // North: Mode (Int) -> 0=Read, 1=Write, 2=Exec
    // South: Value (Any) -> For Write mode

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

    if let Some(Value::Str(key)) = w_sig {
        let mode = if let Some(Value::Int(m)) = n_sig {
            m
        } else {
            0 // Default Read
        };

        match mode {
            0 => { // READ
                if let Some(val) = vm.prologue_state.library.get(&key) {
                    vm.prologue_state.signal_grid[y][x] = Some(val.clone());
                }
            },
            1 => { // WRITE
                // South: Value (From Signal Grid)
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if let Some(val) = &vm.prologue_state.signal_grid[sy][sx] {
                        vm.prologue_state.library.insert(key.clone(), val.clone());
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Ack
                        vm.output.push(format!("LIBRARY: Wrote '{}'", key));
                    }
                }
            },
            2 => { // EXEC
                if let Some(val) = vm.prologue_state.library.get(&key) {
                    if let Value::Str(code) = val {
                        match crate::compiler::compile(code, None) {
                            Ok(mut dna) => {
                                // Append strands to helix
                                let start_idx = vm.dna.helix.strands.len();
                                vm.dna.helix.strands.append(&mut dna.helix.strands);

                                if vm.dna.helix.strands.len() > start_idx {
                                    vm.output.push(format!("LIBRARY: Executing '{}' (Strand {})", key, start_idx));
                                    vm.context_loc = (y, x);
                                    vm.interrupt(start_idx);
                                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                }
                            },
                            Err(e) => {
                                vm.output.push(format!("LIBRARY: Exec Failed '{}': {}", key, e));
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn apply_shelf_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Index (Int)
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if let Some(Value::Int(idx)) = w_sig {
        if idx >= 0 {
            let mut keys: Vec<&String> = vm.prologue_state.library.keys().collect();
            keys.sort(); // Deterministic order
            if (idx as usize) < keys.len() {
                let key = keys[idx as usize];
                vm.prologue_state.signal_grid[y][x] = Some(Value::Str(key.clone()));
            }
        }
    }
}

fn apply_bookmark_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Key (String)
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if let Some(Value::Str(key)) = w_sig {
        if vm.prologue_state.library.contains_key(&key) {
             vm.prologue_state.signal_grid[y][x] = Some(Value::Str(key));
        }
    }
}
