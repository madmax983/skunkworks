use super::{ChimeraVM, Value};
use crate::ast::{Nucleotide, Strand};
use crate::opcode::OpCode;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_strand(strand: &Strand) -> u64 {
    let mut hasher = DefaultHasher::new();
    strand.hash(&mut hasher);
    hasher.finish()
}

pub fn exec_security_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Vaccinate => {
            // stack: strand_idx (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(idx) = val {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let hash = hash_strand(&vm.dna.helix.strands[s_idx]);
                        vm.immune_system.insert(hash);
                        vm.output.push(format!("VACCINATE: Immunized against strand {} (Hash: {:x})", s_idx, hash));
                        vm.energy = vm.energy.saturating_sub(10);
                    } else {
                        vm.output.push("Error: Strand index out of bounds for vaccinate".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for vaccinate".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for vaccinate".to_string());
            }
        }
        OpCode::Verify => {
            // stack: strand_idx (top) -> is_trusted (1 or 0)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(idx) = val {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let hash = hash_strand(&vm.dna.helix.strands[s_idx]);
                        let is_trusted = if vm.immune_system.contains(&hash) { 1 } else { 0 };
                        vm.stack.push(Value::Int(is_trusted));
                        vm.energy = vm.energy.saturating_sub(1);
                    } else {
                        vm.output.push("Error: Strand index out of bounds for verify".to_string());
                        vm.stack.push(Value::Int(0)); // Default to distrust
                    }
                } else {
                    vm.output.push("Error: Type mismatch for verify".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for verify".to_string());
            }
        }
        OpCode::Audit => {
            // stack: -> [ ... junction of untrusted indices ]
            let mut untrusted_indices = Vec::new();
            for (i, strand) in vm.dna.helix.strands.iter().enumerate() {
                let hash = hash_strand(strand);
                if !vm.immune_system.contains(&hash) {
                    untrusted_indices.push(Value::Int(i as i64));
                }
            }
            vm.stack.push(Value::Junction(crate::ast::JunctionType::All, untrusted_indices));
            vm.energy = vm.energy.saturating_sub(5);
            vm.output.push("AUDIT: Scan complete".to_string());
        }
        _ => {}
    }
    None
}
