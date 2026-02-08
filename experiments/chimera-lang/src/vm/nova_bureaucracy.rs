#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_bureaucracy_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::RedTape => exec_red_tape(vm),
        OpCode::Form => exec_form(vm),
        OpCode::Sign => exec_sign(vm),
        OpCode::Permit => exec_permit(vm),
        _ => None,
    }
}

fn exec_red_tape(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(amount)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let amt = amount.clamp(0, 100);
        vm.bureaucracy_grid[cy][cx] = vm.bureaucracy_grid[cy][cx].saturating_add(amt);
        vm.output
            .push(format!("RED TAPE: Increased by {} at {},{}", amt, cx, cy));
    } else {
        vm.output
            .push("Error: Type mismatch for red_tape".to_string());
    }
    None
}

fn exec_form(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(amount)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let current = vm.bureaucracy_grid[cy][cx];

        // Filling forms costs energy
        vm.energy = vm.energy.saturating_sub(5);

        let reduction = amount.clamp(0, current);
        vm.bureaucracy_grid[cy][cx] -= reduction;

        vm.output.push(format!(
            "FORM: Reduced bureaucracy by {} at {},{}",
            reduction, cx, cy
        ));

        // Pushes a "Form" object to stack
        vm.stack.push(Value::Str(format!("Form:{}", reduction)));
    } else {
        vm.output.push("Error: Type mismatch for form".to_string());
    }
    None
}

fn exec_sign(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            if s.starts_with("Form:") {
                // Costs energy to sign
                vm.energy = vm.energy.saturating_sub(2);
                let permit_str = s.replace("Form:", "Permit:");
                vm.stack.push(Value::Str(permit_str));
                vm.output.push("SIGN: Form signed".to_string());
            } else {
                vm.output.push("Error: Can only sign Forms".to_string());
                vm.stack.push(Value::Str(s)); // Return item
            }
        } else {
            vm.output.push("Error: Type mismatch for sign".to_string());
            vm.stack.push(val);
        }
    } else {
        vm.output
            .push("Error: Stack underflow for sign".to_string());
    }
    None
}

fn exec_permit(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            if s.starts_with("Permit:") {
                // Grant immunity for N ticks where N is the permit value
                if let Ok(value) = s.trim_start_matches("Permit:").parse::<usize>() {
                    // We use the 'buffs' map for this
                    // "Bureaucrat" buff prevents Red Tape drain
                    let duration = value * 10; // 1 unit of form = 10 ticks immunity
                    vm.buffs.insert("Bureaucrat".to_string(), duration);
                    vm.output
                        .push(format!("PERMIT: Immunity granted for {} ticks", duration));
                } else {
                    vm.output.push("Error: Invalid permit value".to_string());
                }
            } else {
                vm.output.push("Error: Invalid permit".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for permit".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for permit".to_string());
    }
    None
}

pub fn process_red_tape(vm: &mut ChimeraVM) {
    let (cy, cx) = vm.context_loc;
    let tape_level = vm.bureaucracy_grid[cy][cx];

    if tape_level > 0 {
        if vm.buffs.contains_key("Bureaucrat") {
            // Immune
            return;
        }

        // Drain energy based on level
        // 1-10: -1
        // 11-50: -2
        // 50+: -5
        let drain = if tape_level > 50 {
            5
        } else if tape_level > 10 {
            2
        } else {
            1
        };

        vm.energy = vm.energy.saturating_sub(drain);
        if vm.tick_counter.is_multiple_of(10) {
            vm.output.push(format!(
                "RED TAPE: Drained {} energy at {},{}",
                drain, cx, cy
            ));
        }
    }
}
