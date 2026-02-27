#![cfg(feature = "silicon")]
//! # The Silicon System 🔌
//!
//! The `Silicon` module adds **Circuitry** and **Digital Logic** to the grid.
//!
//! It implements a modified **Wireworld** cellular automaton, allowing the construction of complex logic gates,
//! memory, and signal processing on the Petri Dish.
//!
//! ## Wireworld Rules
//!
//! 1.  **Empty (0)** -> Remains Empty.
//! 2.  **Electron Head (2)** -> Becomes **Electron Tail (3)**.
//! 3.  **Electron Tail (3)** -> Becomes **Conductor (1)**.
//! 4.  **Conductor (1)** -> Becomes **Electron Head (2)** if exactly **1 or 2** neighbors are Electron Heads.
//!
//! ## Components
//!
//! - **Emitter**: A clock source. `EMIT:{freq}:{phase}`. pulses every `freq` ticks.
//! - **Receiver**: An interrupt trigger. `RECV:{strand_idx}`. jumps to `strand_idx` when powered.
//! - **Latch**: A 1-bit memory cell. `LATCH:{state}`.
//!     - **Clock Input**: South neighbor.
//!     - **Data Input**: North neighbor.
//!     - Updates state when Clock is powered.
//! - **Logic Gates**: `G:{type}:{dir}`.
//!     - AND, OR, XOR, NAND, NOT.
//!     - Inputs are adjacent cells (excluding output direction).
//!     - Output is written to the cell in `dir`.
//!
//! ## Example: A Blinker
//!
//! To make a wire blink, you need a loop of length 3+ or an Emitter.
//!
//! ```ignore
//! // Create a wire loop
//! wire(5, 5)
//! wire(5, 6)
//! wire(5, 7)
//! wire(6, 7)
//! wire(6, 5)
//! pulse(5, 5) // Inject electron
//! ```

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Executes Silicon OpCodes (Wireworld and Circuits).
///
/// Handles the construction and manual manipulation of circuit components.
///
/// # Supported Enzymes
///
/// - `Conduct`: Manually trigger a simulation step (if auto-mode is off).
/// - `Wire`: Place a conductor (1).
/// - `Pulse`: Place an electron head (2).
/// - `Silicon`: Toggle automatic circuit simulation.
/// - `Construct`: Build logic gates.
/// - `PinIn`/`PinOut`: Create I/O pins for stack interactions.
///
/// # Examples
///
/// ```rust
/// use chimera_lang::prelude::*;
///
/// // Create a simple wire segment at (0,0)
/// // [ Push(0), Push(0), Wire ] -> Grid[0][0] = 1
///
/// let genes = vec![
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
///     Gene { op: OpCode::Wire, args: vec![] },
/// ];
///
/// let dna = Dna { evolution_config: None, helix: Helix { strands: vec![Strand { genes }] } };
/// let mut vm = ChimeraVM::new(dna);
///
/// // Execute 3 instructions
/// for _ in 0..3 {
///     vm.step();
/// }
///
/// assert_eq!(vm.grid[0][0], Value::Int(1));
/// ```
pub fn exec_silicon_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Conduct => {
            step_circuit(vm);
            vm.output.push("SILICON: Conducted one step".to_string());
        }
        OpCode::Wire => {
            // Fallback to context_loc if stack empty
            let (y, x) = if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    (y, x)
                } else {
                    (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
                }
            } else {
                (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            };

            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                vm.grid[ny][nx] = Value::Int(1); // Conductor
            }
        }
        OpCode::Pulse => {
            let (y, x) = if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    (y, x)
                } else {
                    (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
                }
            } else {
                (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            };

            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                vm.grid[ny][nx] = Value::Int(2); // Head
            }
        }
        OpCode::Silicon => {
            vm.silicon_mode = !vm.silicon_mode;
            let status = if vm.silicon_mode { "ON" } else { "OFF" };
            vm.output
                .push(format!("SILICON: Auto-conduction {}", status));
        }
        OpCode::Construct => {
            // Check args first (from Trace)
            let (t, d, y, x) = if _args.len() >= 2 {
                let t = match &_args[0] {
                    Nucleotide::Number(n) => *n,
                    _ => 0,
                };
                let d = match &_args[1] {
                    Nucleotide::Number(n) => *n,
                    _ => 0,
                };
                (t, d, vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            } else if vm.stack.len() >= 4 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let dir_val = vm.stack.pop().unwrap();
                let type_val = vm.stack.pop().unwrap();
                if let (Value::Int(t), Value::Int(d), Value::Int(y), Value::Int(x)) =
                    (type_val, dir_val, y_val, x_val)
                {
                    (t, d, y, x)
                } else {
                    (0, 0, 0, 0)
                }
            } else {
                (0, 0, -1, -1) // Invalid
            };

            if y != -1 {
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
                }
            }
        }
        OpCode::LogicGate => {
            // Placeholder
        }
        OpCode::PinIn => {
            let (y, x) = if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    (y, x)
                } else {
                    (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
                }
            } else {
                (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            };

            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                vm.grid[ny][nx] = Value::Str("PIN:IN".to_string());
                vm.output.push(format!("PIN_IN: Created at {},{}", nx, ny));
            }
        }
        OpCode::PinOut => {
            let (y, x) = if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    (y, x)
                } else {
                    (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
                }
            } else {
                (vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            };

            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                vm.grid[ny][nx] = Value::Str("PIN:OUT".to_string());
                vm.output.push(format!("PIN_OUT: Created at {},{}", nx, ny));
            }
        }
        OpCode::Emitter => {
            let (f, y, x) = if _args.len() >= 1 {
                let f = match &_args[0] {
                    Nucleotide::Number(n) => *n,
                    _ => 1,
                };
                (f, vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            } else if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let freq_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(f)) = (y_val, x_val, freq_val) {
                    (f, y, x)
                } else {
                    (1, -1, -1)
                }
            } else {
                (1, -1, -1)
            };

            if y != -1 {
                if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                    vm.grid[ny][nx] = Value::Str(format!("EMIT:{}:0", f.max(1)));
                    vm.output.push(format!("EMITTER: Created at {},{}", nx, ny));
                }
            }
        }
        OpCode::Receiver => {
            let (s, y, x) = if _args.len() >= 1 {
                let s = match &_args[0] {
                    Nucleotide::Number(n) => *n,
                    _ => 0,
                };
                (s, vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            } else if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(s)) = (y_val, x_val, s_val) {
                    (s, y, x)
                } else {
                    (0, -1, -1)
                }
            } else {
                (0, -1, -1)
            };

            if y != -1 {
                if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                    vm.grid[ny][nx] = Value::Str(format!("RECV:{}", s));
                    vm.output
                        .push(format!("RECEIVER: Created at {},{}", nx, ny));
                }
            }
        }
        OpCode::Latch => {
            let (state, y, x) = if _args.len() >= 1 {
                let s = match &_args[0] {
                    Nucleotide::Number(n) => *n,
                    _ => 0,
                };
                (s, vm.context_loc.0 as i64, vm.context_loc.1 as i64)
            } else if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(s)) = (y_val, x_val, s_val) {
                    (s, y, x)
                } else {
                    (0, -1, -1)
                }
            } else {
                (0, -1, -1)
            };

            if y != -1 {
                if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                    let s = if state != 0 { 1 } else { 0 };
                    vm.grid[ny][nx] = Value::Str(format!("LATCH:{}", s));
                    vm.output
                        .push(format!("LATCH: Created at {},{} state {}", nx, ny, s));
                }
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
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        Value::Str(s) if s == "LATCH:1" => true,
                        _ => false,
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
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                        {
                            // Only energize wires, don't overwrite components
                            if let Value::Int(1) = vm.grid[ny][nx] {
                                vm.grid[ny][nx] = Value::Int(2);
                            }
                        }
                    }
                }
                vm.output.push(format!("ADC: Wrote {}", val));
            } else {
                vm.output
                    .push("Error: Stack underflow or type mismatch for ADC".to_string());
            }
        }
        OpCode::Trace => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        let strand_idx = trace_circuit(vm, ny, nx);
                        vm.stack.push(Value::Int(strand_idx as i64));
                        vm.output.push(format!(
                            "TRACE: Compiled circuit at {},{} to strand {}",
                            nx, ny, strand_idx
                        ));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for trace".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for trace".to_string());
            }
        }
        OpCode::Fabricate => {
            // stack: strand_idx, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();
                if let (Value::Int(s), Value::Int(y), Value::Int(x)) = (s_val, y_val, x_val) {
                    let s_idx = s as usize;
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        fabricate_circuit(vm, s_idx, ny, nx);
                        vm.output.push(format!(
                            "FABRICATE: Built circuit from strand {} at {},{}",
                            s_idx, nx, ny
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for fabricate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for fabricate".to_string());
            }
        }
        _ => {}
    }
}

fn is_silicon_component(val: &Value) -> bool {
    match val {
        Value::Int(n) if *n >= 1 && *n <= 3 => true,
        Value::Str(s) => {
            s.starts_with("G:")
                || s.starts_with("PIN")
                || s.starts_with("EMIT")
                || s.starts_with("RECV")
                || s.starts_with("LATCH")
        }
        _ => false,
    }
}

fn trace_circuit(vm: &mut ChimeraVM, start_y: usize, start_x: usize) -> usize {
    let mut genes = Vec::new();
    let mut visited = std::collections::HashSet::new();

    // Recursive DFS helper
    fn visit(
        vm: &ChimeraVM,
        y: usize,
        x: usize,
        visited: &mut std::collections::HashSet<(usize, usize)>,
        genes: &mut Vec<crate::ast::Gene>,
    ) {
        visited.insert((y, x));

        // 1. Identify Component and add OpCode
        let val = &vm.grid[y][x];
        match val {
            Value::Int(1) => genes.push(crate::ast::Gene {
                op: OpCode::Wire,
                args: vec![],
            }),
            Value::Int(2) => genes.push(crate::ast::Gene {
                op: OpCode::Pulse,
                args: vec![],
            }),
            Value::Int(3) => genes.push(crate::ast::Gene {
                op: OpCode::Wire,
                args: vec![],
            }), // Tail -> Wire
            Value::Str(s) => {
                if s == "PIN:IN" {
                    genes.push(crate::ast::Gene {
                        op: OpCode::PinIn,
                        args: vec![],
                    });
                } else if s == "PIN:OUT" {
                    genes.push(crate::ast::Gene {
                        op: OpCode::PinOut,
                        args: vec![],
                    });
                } else if s.starts_with("G:") {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 3 {
                        let type_map = match parts[1] {
                            "AND" => 0,
                            "OR" => 1,
                            "XOR" => 2,
                            "NAND" => 3,
                            "NOT" => 4,
                            _ => 0,
                        };
                        let dir = parts[2].parse::<i64>().unwrap_or(0);
                        genes.push(crate::ast::Gene {
                            op: OpCode::Construct,
                            args: vec![
                                crate::ast::Nucleotide::Number(type_map),
                                crate::ast::Nucleotide::Number(dir),
                            ],
                        });
                    }
                } else if s.starts_with("EMIT:") {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() >= 2 {
                        let freq = parts[1].parse::<i64>().unwrap_or(1);
                        genes.push(crate::ast::Gene {
                            op: OpCode::Emitter,
                            args: vec![crate::ast::Nucleotide::Number(freq)],
                        });
                    }
                } else if s.starts_with("RECV:") {
                    let s_idx = s.trim_start_matches("RECV:").parse::<i64>().unwrap_or(0);
                    genes.push(crate::ast::Gene {
                        op: OpCode::Receiver,
                        args: vec![crate::ast::Nucleotide::Number(s_idx)],
                    });
                } else if s.starts_with("LATCH:") {
                    let state = s.trim_start_matches("LATCH:").parse::<i64>().unwrap_or(0);
                    genes.push(crate::ast::Gene {
                        op: OpCode::Latch,
                        args: vec![crate::ast::Nucleotide::Number(state)],
                    });
                }
            }
            _ => {}
        }

        // 2. Visit neighbors
        let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                if !visited.contains(&(ny, nx)) && is_silicon_component(&vm.grid[ny][nx]) {
                    // Move there: Push dy, dx -> Migrate
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Number(dy)],
                    });
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Number(dx)],
                    });
                    genes.push(crate::ast::Gene {
                        op: OpCode::Migrate,
                        args: vec![],
                    });

                    visit(vm, ny, nx, visited, genes);

                    // Move back
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Number(-dy)],
                    });
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Number(-dx)],
                    });
                    genes.push(crate::ast::Gene {
                        op: OpCode::Migrate,
                        args: vec![],
                    });
                }
            }
        }
    }

    if is_silicon_component(&vm.grid[start_y][start_x]) {
        visit(vm, start_y, start_x, &mut visited, &mut genes);
    }

    // Create strand
    #[cfg(feature = "nova")]
    if vm.dna.helix.strands.len() < crate::vm::MAX_STRANDS {
        vm.dna.helix.strands.push(crate::ast::Strand { genes });
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }
        return vm.dna.helix.strands.len() - 1;
    }

    // Fallback if not Nova (should typically be enabled with silicon?)
    #[cfg(not(feature = "nova"))]
    {
        vm.dna.helix.strands.push(crate::ast::Strand { genes });
        return vm.dna.helix.strands.len() - 1;
    }

    #[cfg(feature = "nova")]
    0 // Failure
}

fn fabricate_circuit(vm: &mut ChimeraVM, strand_idx: usize, start_y: usize, start_x: usize) {
    if strand_idx >= vm.dna.helix.strands.len() {
        return;
    }

    // Save state
    let old_ip = vm.ip;
    let old_ctx = vm.context_loc;
    let old_mode = vm.silicon_mode;

    vm.silicon_mode = false; // Pause physics
    vm.context_loc = (start_y, start_x);
    vm.ip = (strand_idx, 0);

    // Execute strand synchronously
    let limit = 1000;
    for _ in 0..limit {
        let strand_len = vm.dna.helix.strands[vm.ip.0].genes.len();
        if vm.ip.1 >= strand_len {
            break;
        }

        // Execute gene (manually calling inner to bypass main loop overhead/checks)
        let (op, args) = {
            let gene = &vm.dna.helix.strands[vm.ip.0].genes[vm.ip.1];
            (gene.op.clone(), gene.args.clone())
        };

        // We use execute_gene_inner to run the op
        let jump = vm.execute_gene_inner(op, &args);

        if let Some(target) = jump {
            vm.ip = target;
        } else {
            vm.ip.1 += 1;
        }

        if vm.ip.0 != strand_idx {
            // Jumped out of strand? Stop fabrication.
            break;
        }
    }

    // Restore state
    vm.ip = old_ip;
    vm.context_loc = old_ctx;
    vm.silicon_mode = old_mode;
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
            // Evolve simple Wireworld cells
            // Using logic extracted from helper function
            if let Some(next_val) = step_cell_wireworld(vm, y, x) {
                next_grid[y][x] = next_val;
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
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                // Check for Head (2) or Firing Emitter
                                if let Value::Int(2) = vm.grid[ny][nx] {
                                    triggered = true;
                                } else if let Value::Str(es) = &vm.grid[ny][nx] {
                                    if es.starts_with("EMIT:") {
                                        let parts: Vec<&str> = es.split(':').collect();
                                        if parts.len() == 3 {
                                            if let Ok(phase) = parts[2].parse::<i64>() {
                                                if phase == 0 {
                                                    triggered = true;
                                                }
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
                                if dy == 0 && dx == 0 {
                                    continue;
                                }
                                if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                    if let Value::Int(2) = vm.grid[ny][nx] {
                                        triggered = true;
                                    } else if let Value::Str(es) = &vm.grid[ny][nx] {
                                        if es.starts_with("EMIT:") {
                                            let parts: Vec<&str> = es.split(':').collect();
                                            if parts.len() == 3 {
                                                if let Ok(phase) = parts[2].parse::<i64>() {
                                                    if phase == 0 {
                                                        triggered = true;
                                                    }
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
                            if let Value::Int(2) = vm.grid[cy][cx] {
                                clock_high = true;
                            } else if let Value::Str(es) = &vm.grid[cy][cx] {
                                if es.starts_with("EMIT:") {
                                    let parts: Vec<&str> = es.split(':').collect();
                                    if parts.len() == 3 {
                                        if let Ok(phase) = parts[2].parse::<i64>() {
                                            if phase == 0 {
                                                clock_high = true;
                                            }
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
                                if let Value::Int(2) = vm.grid[ny][nx] {
                                    data_high = 1;
                                } else if let Value::Str(es) = &vm.grid[ny][nx] {
                                    if es.starts_with("EMIT:") {
                                        let parts: Vec<&str> = es.split(':').collect();
                                        if parts.len() == 3 {
                                            if let Ok(phase) = parts[2].parse::<i64>() {
                                                if phase == 0 {
                                                    data_high = 1;
                                                }
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

/// Runs logic for a single Wireworld cell.
///
/// Returns `Some(Value)` if the cell state changes, `None` otherwise.
/// This allows external systems (like Reality Bubbles) to run Silicon physics locally.
pub fn step_cell_wireworld(vm: &ChimeraVM, y: usize, x: usize) -> Option<Value> {
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
                    if let (Ok(freq), Ok(phase)) =
                        (parts[1].parse::<i64>(), parts[2].parse::<i64>())
                    {
                        let mut new_phase = phase + 1;
                        if new_phase >= freq {
                            new_phase = 0;
                        }
                        return Some(Value::Str(format!("EMIT:{}:{}", freq, new_phase)));
                    }
                }
                // Emitter handles its own next state above
                0
            } else if s.starts_with("RECV:")
                || s == "PIN:IN"
                || s == "PIN:OUT"
                || s.starts_with("G:")
                || s.starts_with("LATCH:")
            {
                // Static components (physically)
                0
            } else {
                0
            }
        }
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
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
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
                                            if let (Ok(_freq), Ok(phase)) =
                                                (parts[1].parse::<i64>(), parts[2].parse::<i64>())
                                            {
                                                if phase == 0 {
                                                    // Firing phase
                                                    head_neighbors += 1;
                                                }
                                            }
                                        }
                                    } else if s == "LATCH:1" {
                                        head_neighbors += 1;
                                    }
                                }
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
            return Some(Value::Int(next_state));
        }
    }
    None
}
