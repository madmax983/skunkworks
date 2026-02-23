#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
pub fn exec_meta_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Define => {
            // stack: strand_idx, name_str (top)
            // Usage: push(1) push("name") define
            if vm.stack.len() >= 2 {
                let name_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();

                if let (Value::Str(name), Value::Int(idx)) = (name_val, s_val) {
                    if idx >= 0 && (idx as usize) < vm.dna.helix.strands.len() {
                        vm.dictionary.insert(name.clone(), idx as usize);
                        vm.output
                            .push(format!("DEFINE: {} -> Strand {}", name, idx));
                        vm.energy = vm.energy.saturating_sub(10);
                    } else {
                        vm.output
                            .push("Error: Invalid strand index for define".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for define".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for define".to_string());
            }
        }
        OpCode::Undefine => {
            // stack: name_str
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(name) = val {
                    if vm.dictionary.remove(&name).is_some() {
                        vm.output.push(format!("UNDEFINE: {}", name));
                        vm.energy = vm.energy.saturating_sub(5);
                    } else {
                        vm.output.push(format!("UNDEFINE: {} not found", name));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for undefine".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for undefine".to_string());
            }
        }
        OpCode::Dictionary => {
            let names: Vec<Value> = vm
                .dictionary
                .keys()
                .map(|k| Value::Str(k.clone()))
                .collect();
            vm.stack
                .push(Value::Junction(crate::ast::JunctionType::All, names));
            vm.energy = vm.energy.saturating_sub(2);
        }
        _ => {}
    }
    None
}
