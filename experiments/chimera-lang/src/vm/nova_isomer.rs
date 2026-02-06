use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

/// Inverts the operation for Chiral (D-isomer) strands.
///
/// Arithmetic:
/// - Add <-> Sub
/// - Mul <-> Div
///
/// Logic:
/// - Brz <-> Brnz
///
/// This simulates "Chirality" where the molecule is a mirror image,
/// causing enzymes to behave in reverse or complementary ways.
pub fn transform_op(op: OpCode) -> OpCode {
    match op {
        OpCode::Add => OpCode::Sub,
        OpCode::Sub => OpCode::Add,
        OpCode::Mul => OpCode::Div,
        OpCode::Div => OpCode::Mul,
        OpCode::Brz => OpCode::Brnz,
        OpCode::Brnz => OpCode::Brz,
        _ => op,
    }
}

/// Executes Isomer-specific operations.
pub fn exec_isomer_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Isomerize => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(idx) = val {
                    let idx = idx as usize;
                    if idx < vm.dna.helix.strands.len() {
                        if vm.isomers.contains(&idx) {
                            vm.isomers.remove(&idx);
                            vm.output.push(format!("ISOMERIZE: Strand {} -> L-isomer (Normal)", idx));
                        } else {
                            vm.isomers.insert(idx);
                            vm.output.push(format!("ISOMERIZE: Strand {} -> D-isomer (Chiral)", idx));
                        }
                    } else {
                        vm.output.push("Error: Strand index out of bounds for isomerize".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for isomerize".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for isomerize".to_string());
            }
        }
        _ => {}
    }
}
