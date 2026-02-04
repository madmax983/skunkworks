#![cfg(feature = "cortex")]
use super::ChimeraVM;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::Value;

#[cfg(feature = "cortex")]
pub fn exec_cortex_op(vm: &mut ChimeraVM, op: OpCode, args: &[Nucleotide]) {
    match op {
        OpCode::Link => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(target) => {
                        let target_idx = target as usize;
                        let s_idx = vm.ip.0;
                        // Check bounds using activation_levels as proxy for strand count
                        if target_idx < vm.activation_levels.len()
                            && s_idx < vm.synapse_map.len()
                        {
                            if !vm.synapse_map[s_idx].contains(&target_idx) {
                                vm.synapse_map[s_idx].push(target_idx);
                                vm.output
                                    .push(format!("LINK: {} -> {}", s_idx, target_idx));
                            }
                        } else {
                            vm.output
                                .push("Error: Invalid strand index for link".to_string());
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for link".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for link".to_string());
            }
        }
        OpCode::Sever => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(target) => {
                        let target_idx = target as usize;
                        let s_idx = vm.ip.0;
                        if s_idx < vm.synapse_map.len() {
                            if let Some(pos) = vm.synapse_map[s_idx]
                                .iter()
                                .position(|&x| x == target_idx)
                            {
                                vm.synapse_map[s_idx].remove(pos);
                                vm.output
                                    .push(format!("SEVER: {} -x {}", s_idx, target_idx));
                            }
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for sever".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sever".to_string());
            }
        }
        OpCode::Spark => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(amount) => {
                        let s_idx = vm.ip.0;
                        if s_idx < vm.synapse_map.len() {
                            let targets = vm.synapse_map[s_idx].clone();
                            let count = targets.len();
                            for target_idx in targets {
                                if target_idx < vm.activation_levels.len() {
                                    vm.activation_levels[target_idx] = vm.activation_levels
                                        [target_idx]
                                        .saturating_add(amount);
                                }
                            }
                            vm.energy = vm.energy.saturating_sub((count as i64) + 1);
                            vm.output
                                .push(format!("SPARK: Fired {} to {} targets", amount, count));
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for spark".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for spark".to_string());
            }
        }
        OpCode::Sense => {
            let s_idx = vm.ip.0;
            if s_idx < vm.activation_levels.len() {
                let level = vm.activation_levels[s_idx];
                vm.stack.push(Value::Int(level));
            } else {
                vm.stack.push(Value::Int(0));
            }
        }
        OpCode::Gate => {
            if let Some(Nucleotide::Number(threshold)) = args.first() {
                let s_idx = vm.ip.0;
                if s_idx < vm.activation_levels.len()
                    && vm.activation_levels[s_idx] < *threshold
                {
                    // Skip next instruction
                    vm.ip.1 += 1;
                }
            } else {
                vm.output.push("Error: Invalid arg for gate".to_string());
            }
        }
        _ => {}
    }
}
