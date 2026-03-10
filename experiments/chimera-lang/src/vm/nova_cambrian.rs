use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_morphogen(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., channel, amount ] -> [ ... ]
    if vm.stack.len() >= 2 {
        let amt_val = vm.stack.pop().unwrap();
        let chan_val = vm.stack.pop().unwrap();

        if let (Value::Int(c), Value::Int(a)) = (chan_val, amt_val) {
            let channel = c.clamp(0, 2) as usize;
            let amount = a; // Can be negative to inhibit? Let's say yes.

            let (cy, cx) = vm.context_loc;

            // Add to current level
            let current = vm.hormone_grid[cy][cx][channel];
            let new_val = current.saturating_add(amount).clamp(0, 1000);
            vm.hormone_grid[cy][cx][channel] = new_val;

            vm.energy = vm.energy.saturating_sub(1);
            vm.output.push(format!(
                "MORPHOGEN: Emit Ch{} Amt{} at {},{}",
                channel, amount, cx, cy
            ));
        } else {
            vm.output
                .push("Error: Type mismatch for Morphogen".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Morphogen".to_string());
    }
    None
}

pub fn exec_hox_switch(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., channel, threshold, strand_idx ] -> [ ... ]
    if vm.stack.len() >= 3 {
        let strand_val = vm.stack.pop().unwrap();
        let thresh_val = vm.stack.pop().unwrap();
        let chan_val = vm.stack.pop().unwrap();

        if let (Value::Int(c), Value::Int(t), Value::Int(s_idx)) =
            (chan_val, thresh_val, strand_val)
        {
            let channel = c.clamp(0, 2) as usize;
            let (cy, cx) = vm.context_loc;
            let level = vm.hormone_grid[cy][cx][channel];

            if level > t {
                let target = s_idx as usize;
                if target < vm.dna.helix.strands.len() {
                    // Branch taken (Hox Activation)
                    // Push current IP to call stack? No, Hox is usually a switch.
                    // Let's treat it like a Jump if true.
                    vm.output.push(format!(
                        "HOX: Activated Ch{} ({} > {}) -> Jump {}",
                        channel, level, t, target
                    ));
                    return Some((target, 0));
                } else {
                    vm.output
                        .push("Error: Invalid strand index for HoxSwitch".to_string());
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for HoxSwitch".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for HoxSwitch".to_string());
    }
    None
}

pub fn exec_adhere(vm: &mut ChimeraVM, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Wrapper for Bond (Nova Metazoa)
    // Uses the same logic but explicitly for tissue formation in Cambrian context.
    crate::vm::nova_metazoa::exec_bond(vm, op, args)
}
