#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_ward(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., persistence, strand_idx ]
    if vm.stack.len() >= 2 {
        let idx_val = vm.stack.pop().unwrap();
        let persist_val = vm.stack.pop().unwrap();

        if let (Value::Int(p), Value::Int(idx)) = (persist_val, idx_val) {
            let (cy, cx) = vm.context_loc;
            if idx >= 0 && (idx as usize) < vm.dna.helix.strands.len() {
                // Format: "WARD:{strand_idx}:{persistence}"
                let ward_str = format!("WARD:{}:{}", idx, p);
                vm.grid[cy][cx] = Value::Str(ward_str);
                vm.energy = vm.energy.saturating_sub(10);
                vm.output.push(format!("WARD: Inscribed at {},{}", cx, cy));
            } else {
                vm.output
                    .push("Error: Invalid strand index for ward".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for ward".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for ward".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn check_ward_trigger(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;

    // Optimization: Check if string without cloning (requires double lookup or careful borrowing)
    // Value::Str is hidden in enum.
    let cell_val = vm.grid[cy][cx].clone();

    if let Value::Str(s) = cell_val {
        if s.starts_with("WARD:") {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() >= 3 {
                if let (Ok(idx), Ok(persistence)) =
                    (parts[1].parse::<usize>(), parts[2].parse::<i64>())
                {
                    if idx < vm.dna.helix.strands.len() {
                        if vm.call_stack.len() < crate::vm::MAX_CALL_STACK_DEPTH {
                            // Push return address (next instruction after current one)
                            // Note: This function is called from Migrate/Osmosis.
                            // If Migrate returned None, IP would increment.
                            // Since we return Some, IP is set to target.
                            // So we must save (current_ip + 1) as return.
                            vm.call_stack.push((vm.ip.0, vm.ip.1 + 1));

                            vm.output
                                .push(format!("WARD: Triggered strand {} at {},{}", idx, cx, cy));

                            // Handle persistence
                            if persistence <= 0 {
                                vm.grid[cy][cx] = Value::Int(0); // Clear
                            } else {
                                // Permanent
                            }

                            return Some((idx, 0));
                        } else {
                            vm.output
                                .push("WARD: Trigger failed (Call stack full)".to_string());
                        }
                    } else {
                        vm.output.push("WARD: Invalid strand index".to_string());
                    }
                }
            }
        }
    }
    None
}
