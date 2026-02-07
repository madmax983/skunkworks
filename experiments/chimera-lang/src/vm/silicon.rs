#![cfg(feature = "silicon")]
use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Executes Silicon OpCodes (Wireworld and Circuits).
pub fn exec_silicon_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Conduct => {
            step_circuit(vm);
            vm.output.push("SILICON: Conducted one step".to_string());
        }
        OpCode::Wire => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Int(1); // Conductor
                    }
                } else {
                    vm.output.push("Error: Type mismatch for wire".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for wire".to_string());
            }
        }
        OpCode::Pulse => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Int(2); // Electron Head
                    }
                } else {
                    vm.output.push("Error: Type mismatch for pulse".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for pulse".to_string());
            }
        }
        OpCode::Silicon => {
            vm.silicon_mode = !vm.silicon_mode;
            let status = if vm.silicon_mode { "ON" } else { "OFF" };
            vm.output
                .push(format!("SILICON: Auto-conduction {}", status));
        }
        OpCode::Construct => {
            // stack: type, dir, y, x (top)
            if vm.stack.len() >= 4 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let dir_val = vm.stack.pop().unwrap();
                let type_val = vm.stack.pop().unwrap();

                if let (Value::Int(t), Value::Int(d), Value::Int(y), Value::Int(x)) =
                    (type_val, dir_val, y_val, x_val)
                {
                    let type_str = match t {
                        0 => "AND",
                        1 => "OR",
                        2 => "XOR",
                        3 => "NAND",
                        4 => "NOT",
                        _ => "AND",
                    };
                    let dir_idx = d.rem_euclid(4);
                    let s = format!("G:{}:{}", type_str, dir_idx);
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(s);
                        vm.output.push(format!(
                            "CONSTRUCT: {} gate facing {} at {},{}",
                            type_str, dir_idx, nx, ny
                        ));
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for construct".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for construct".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for construct".to_string());
            }
        }
        OpCode::LogicGate => {
            // Manual placement logic if needed, or introspection
            // For now, no-op or placeholder
        }
        OpCode::PinIn => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str("PIN:IN".to_string());
                        vm.output.push(format!("PIN_IN: Created at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for pin_in".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for pin_in".to_string());
            }
        }
        OpCode::PinOut => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str("PIN:OUT".to_string());
                        vm.output.push(format!("PIN_OUT: Created at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for pin_out".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for pin_out".to_string());
            }
        }
        OpCode::Emitter => {
            // stack: freq, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let freq_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(f)) = (y_val, x_val, freq_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("EMIT:{}:0", f.max(1)));
                        vm.output.push(format!("EMITTER: Created at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for emitter".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for emitter".to_string());
            }
        }
        OpCode::Receiver => {
            // stack: strand_idx, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(s)) = (y_val, x_val, s_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("RECV:{}", s));
                        vm.output.push(format!("RECEIVER: Created at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for receiver".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for receiver".to_string());
            }
        }
        OpCode::Latch => {
            // stack: state, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let state_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(s)) = (y_val, x_val, state_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        let state = if s != 0 { 1 } else { 0 };
                        vm.grid[ny][nx] = Value::Str(format!("LATCH:{}", state));
                        vm.output.push(format!("LATCH: Created at {},{} state {}", nx, ny, state));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for latch".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for latch".to_string());
            }
        }
        OpCode::DAC => {
            // Read 4 neighbors.
            let (cy, cx) = vm.context_loc;
            let mut val = 0;

            // Order: N=8, E=4, S=2, W=1
            let neighbors = [(-1, 0, 8), (0, 1, 4), (1, 0, 2), (0, -1, 1)];

            for (dy, dx, bit) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    let cell = &vm.grid[ny][nx];
                    let active = match cell {
                        Value::Int(2) => true, // Electron Head
                        Value::Str(s) if s.starts_with("EMIT:") => {
                             // Check if firing (phase == 0)
                             let parts: Vec<&str> = s.split(':').collect();
                             if parts.len() == 3 {
                                 if let Ok(phase) = parts[2].parse::<i64>() {
                                     phase == 0
                                 } else { false }
                             } else { false }
                        },
                        Value::Str(s) if s == "LATCH:1" => true,
                        _ => false
                    };

                    if active {
                        val |= bit;
                    }
                }
            }
            vm.stack.push(Value::Int(val));
            vm.output.push(format!("DAC: Read {}", val));
        }
        OpCode::ADC => {
            // Pop value, write pulses
            if let Some(Value::Int(val)) = vm.stack.pop() {
                let (cy, cx) = vm.context_loc;
                let neighbors = [(-1, 0, 8), (0, 1, 4), (1, 0, 2), (0, -1, 1)];

                for (dy, dx, bit) in neighbors {
                    if (val & bit) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                            // Only energize wires, don't overwrite components
                            if let Value::Int(1) = vm.grid[ny][nx] {
                                vm.grid[ny][nx] = Value::Int(2);
                            }
                        }
                    }
                }
                vm.output.push(format!("ADC: Wrote {}", val));
            } else {
                vm.output.push("Error: Stack underflow or type mismatch for ADC".to_string());
            }
        }
        _ => {}
    }
}

/// Runs one step of the Circuit (Wireworld + Gates + Pins + Chaos) on the grid.
pub fn step_circuit(vm: &mut ChimeraVM) {
    let rows = vm.grid.len();
    if rows == 0 {
        return;
    }
    let cols = vm.grid[0].len();
    let mut next_grid = vm.grid.clone();

    // Helper to get neighbor coords
    let get_neighbor =
        |vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64| -> Option<(usize, usize)> {
            vm.normalize_coords(y as i64 + dy, x as i64 + dx)
        };

    // Pass 1: Wireworld Automata & Active Components
    for y in 0..rows {
        for x in 0..cols {
            let current_cell = &vm.grid[y][x];

            // Map values to Wireworld states for evolution of simple cells
            // 0=Empty, 1=Conductor, 2=Head, 3=Tail
            // Complex cells (Str) handle their own state update or stay static
            let state = match current_cell {
                Value::Int(1) => 1,
                Value::Int(2) => 2,
                Value::Int(3) => 3,
                Value::Str(s) => {
                    if s.starts_with("EMIT:") {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() == 3 {
                            if let (Ok(freq), Ok(phase)) = (parts[1].parse::<i64>(), parts[2].parse::<i64>()) {
                                let mut new_phase = phase + 1;
                                if new_phase >= freq {
                                    new_phase = 0;
                                }
                                next_grid[y][x] = Value::Str(format!("EMIT:{}:{}", freq, new_phase));
                            }
                        }
                        // Emitter handles its own next state, doesn't evolve via WW rules
                        0
                    } else if s.starts_with("RECV:") || s == "PIN:IN" || s == "PIN:OUT" || s.starts_with("G:") || s.starts_with("LATCH:") {
                        // Static components (physically)
                        0
                    } else {
                        0
                    }
                },
                _ => 0,
            };

            // Evolve simple Wireworld cells
            if state != 0 {
                let next_state = match state {
                    2 => 3, // Head -> Tail
                    3 => 1, // Tail -> Conductor
                    1 => {
                        // Conductor -> Head if 1 or 2 heads nearby
                        let mut head_neighbors = 0;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dy == 0 && dx == 0 {
                                    continue;
                                }
                                if let Some((ny, nx)) =
                                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    match &vm.grid[ny][nx] {
                                        Value::Int(2) => head_neighbors += 1,
                                        Value::Str(s) => {
                                            if s == "PIN:IN" {
                                                if let Some(Value::Int(val)) = vm.stack.last() {
                                                    if *val > 0 {
                                                        head_neighbors += 1;
                                                    }
                                                }
                                            } else if s.starts_with("EMIT:") {
                                                // Check if Emitter is firing
                                                let parts: Vec<&str> = s.split(':').collect();
                                                if parts.len() == 3 {
                                                    if let (Ok(_freq), Ok(phase)) = (parts[1].parse::<i64>(), parts[2].parse::<i64>()) {
                                                        if phase == 0 { // Firing phase
                                                            head_neighbors += 1;
                                                        }
                                                    }
                                                }
                                            } else if s == "LATCH:1" {
                                                head_neighbors += 1;
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        }
                        if head_neighbors == 1 || head_neighbors == 2 {
                            2
                        } else {
                            1
                        }
                    }
                    _ => state,
                };

                if next_state != state {
                    next_grid[y][x] = Value::Int(next_state);
                }
            }
        }
    }

    // Pass 2: Logic Gates, Pin Output, Receiver, Latch Logic
    let mut pin_outs = 0;
    let mut interrupts = Vec::new();

    for y in 0..rows {
        for x in 0..cols {
            if let Value::Str(s) = &vm.grid[y][x] {
                // Pin Output Logic
                if s == "PIN:OUT" {
                    // Check neighbors for Electron Head (2)
                    let mut triggered = false;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 { continue; }
                            if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                // Check for Head (2) or Firing Emitter
                                if let Value::Int(2) = vm.grid[ny][nx] {
                                    triggered = true;
                                } else if let Value::Str(es) = &vm.grid[ny][nx] {
                                    if es.starts_with("EMIT:") {
                                        let parts: Vec<&str> = es.split(':').collect();
                                        if parts.len() == 3 {
                                            if let Ok(phase) = parts[2].parse::<i64>() {
                                                if phase == 0 { triggered = true; }
                                            }
                                        }
                                    } else if es == "LATCH:1" {
                                        triggered = true;
                                    }
                                }
                            }
                        }
                    }
                    if triggered {
                        pin_outs += 1;
                    }
                }
                // Receiver Logic
                else if s.starts_with("RECV:") {
                    if let Ok(strand_idx) = s.trim_start_matches("RECV:").parse::<usize>() {
                        let mut triggered = false;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dy == 0 && dx == 0 { continue; }
                                if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                    if let Value::Int(2) = vm.grid[ny][nx] {
                                        triggered = true;
                                    } else if let Value::Str(es) = &vm.grid[ny][nx] {
                                        if es.starts_with("EMIT:") {
                                            let parts: Vec<&str> = es.split(':').collect();
                                            if parts.len() == 3 {
                                                if let Ok(phase) = parts[2].parse::<i64>() {
                                                    if phase == 0 { triggered = true; }
                                                }
                                            }
                                        } else if es == "LATCH:1" {
                                            triggered = true;
                                        }
                                    }
                                }
                            }
                        }
                        if triggered {
                            interrupts.push(strand_idx);
                        }
                    }
                }
                // Logic Gate
                else if s.starts_with("G:") {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 3 {
                        let gate_type = parts[1];
                        let dir_idx = parts[2].parse::<usize>().unwrap_or(0) % 4;
                        let (out_dy, out_dx) = match dir_idx {
                            0 => (-1, 0),
                            1 => (0, 1),
                            2 => (1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };

                        let mut active_inputs = 0;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dy == 0 && dx == 0 {
                                    continue;
                                }
                                if dy == out_dy && dx == out_dx {
                                    continue;
                                }
                                if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                    if let Value::Int(2) = vm.grid[ny][nx] {
                                        active_inputs += 1;
                                    }
                                }
                            }
                        }

                        let output_high = match gate_type {
                            "AND" => active_inputs >= 2,
                            "OR" => active_inputs >= 1,
                            "XOR" => active_inputs % 2 == 1,
                            "NAND" => active_inputs < 2,
                            "NOT" => active_inputs == 0,
                            _ => false,
                        };

                        if output_high {
                            if let Some((ny, nx)) = get_neighbor(vm, y, x, out_dy, out_dx) {
                                if let Value::Int(1) = vm.grid[ny][nx] {
                                    next_grid[ny][nx] = Value::Int(2);
                                }
                            }
                        }
                    }
                }
                // Latch Logic
                else if s.starts_with("LATCH:") {
                    if let Ok(state) = s.trim_start_matches("LATCH:").parse::<i64>() {
                        // Clock: South (1, 0), Data: North (-1, 0)
                        let mut clock_high = false;
                        if let Some((cy, cx)) = get_neighbor(vm, y, x, 1, 0) {
                             if let Value::Int(2) = vm.grid[cy][cx] { clock_high = true; }
                             else if let Value::Str(es) = &vm.grid[cy][cx] {
                                 if es.starts_with("EMIT:") {
                                     let parts: Vec<&str> = es.split(':').collect();
                                     if parts.len() == 3 {
                                         if let Ok(phase) = parts[2].parse::<i64>() {
                                             if phase == 0 { clock_high = true; }
                                         }
                                     }
                                 } else if es == "LATCH:1" {
                                     clock_high = true;
                                 }
                             }
                        }

                        let mut next_state = state;
                        if clock_high {
                            let mut data_high = 0;
                            if let Some((ny, nx)) = get_neighbor(vm, y, x, -1, 0) {
                                if let Value::Int(2) = vm.grid[ny][nx] { data_high = 1; }
                                else if let Value::Str(es) = &vm.grid[ny][nx] {
                                     if es.starts_with("EMIT:") {
                                         let parts: Vec<&str> = es.split(':').collect();
                                         if parts.len() == 3 {
                                             if let Ok(phase) = parts[2].parse::<i64>() {
                                                 if phase == 0 { data_high = 1; }
                                             }
                                         }
                                     } else if es == "LATCH:1" {
                                         data_high = 1;
                                     }
                                }
                            }
                            next_state = data_high;
                        }

                        // Check if state changed or if we need to propagate LATCH string to next_grid
                        if next_state != state {
                            next_grid[y][x] = Value::Str(format!("LATCH:{}", next_state));
                        }
                        // Note: If state didn't change, the LATCH string is already preserved in next_grid clone
                        // because LATCH strings are not evolved in Pass 1.
                        // Wait, Pass 1:
                        // Value::Str(s) -> if starts with EMIT ... else 0.
                        // if state != 0, next_grid updated.
                        // But LATCH returns 0 in Pass 1 match, so it's not updated there.
                        // So next_grid[y][x] is still LATCH:{old_state}.
                        // So we only update if next_state != state.
                    }
                }
            }
        }
    }

    vm.grid = next_grid;

    // Apply deferred effects
    for _ in 0..pin_outs {
        vm.stack.push(Value::Int(1));
    }
    for strand_idx in interrupts {
        vm.interrupt(strand_idx);
    }
}
