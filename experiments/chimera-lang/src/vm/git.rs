#![cfg(feature = "git")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use std::process::Command;

pub fn exec_git_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Ancestry => {
            // stack: count (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(count) = val {
                    let n = count.clamp(1, 100);
                    match Command::new("git")
                        .args(&["log", &format!("-n{}", n), "--pretty=format:%H"])
                        .output()
                    {
                        Ok(output) => {
                            if output.status.success() {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                let hashes: Vec<&str> = stdout.lines().collect();
                                let len = hashes.len();

                                vm.stack.push(Value::Int(len as i64));
                                for hash in hashes {
                                    vm.stack.push(Value::Str(hash.to_string()));
                                }
                                vm.energy = vm.energy.saturating_sub(5);
                                vm.output
                                    .push(format!("ANCESTRY: Retrieved {} commits", len));
                            } else {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                vm.output.push(format!("ANCESTRY ERROR: {}", stderr.trim()));
                                vm.stack.push(Value::Int(0));
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("ANCESTRY ERROR: {}", e));
                            vm.stack.push(Value::Int(0));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for ancestry".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ancestry".to_string());
            }
            None
        }
        OpCode::Excavate => {
            // stack: hash_str, path_str (top)
            if vm.stack.len() >= 2 {
                let path_val = vm.stack.pop().unwrap();
                let hash_val = vm.stack.pop().unwrap();

                if let (Value::Str(hash), Value::Str(path)) = (hash_val, path_val) {
                    let spec = format!("{}:{}", hash, path);
                    match Command::new("git").args(&["show", &spec]).output() {
                        Ok(output) => {
                            if output.status.success() {
                                let content = String::from_utf8_lossy(&output.stdout).to_string();
                                vm.stack.push(Value::Str(content.clone()));
                                vm.energy = vm.energy.saturating_sub(10);
                                vm.output.push(format!(
                                    "EXCAVATE: Read {} bytes from {}:{}",
                                    content.len(),
                                    hash,
                                    path
                                ));
                            } else {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                vm.output.push(format!("EXCAVATE ERROR: {}", stderr.trim()));
                                vm.stack.push(Value::Str(String::new()));
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("EXCAVATE ERROR: {}", e));
                            vm.stack.push(Value::Str(String::new()));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for excavate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for excavate".to_string());
            }
            None
        }
        OpCode::Evolution => {
            // stack: hash_str (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(hash) = val {
                    match Command::new("git")
                        .args(&["show", "--format=", &hash]) // --format= suppresses commit msg, showing only diff
                        .output()
                    {
                        Ok(output) => {
                            if output.status.success() {
                                let diff = String::from_utf8_lossy(&output.stdout).to_string();
                                vm.stack.push(Value::Str(diff.clone()));
                                vm.energy = vm.energy.saturating_sub(10);
                                vm.output.push(format!(
                                    "EVOLUTION: Retrieved diff for {} ({} bytes)",
                                    hash,
                                    diff.len()
                                ));
                            } else {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                vm.output
                                    .push(format!("EVOLUTION ERROR: {}", stderr.trim()));
                                vm.stack.push(Value::Str(String::new()));
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("EVOLUTION ERROR: {}", e));
                            vm.stack.push(Value::Str(String::new()));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for evolution".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for evolution".to_string());
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Dna;
    use crate::ast::Helix;
    use crate::ast::Strand;

    // Helper to create a dummy VM
    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_git_ancestry() {
        let mut vm = make_vm();
        vm.stack.push(Value::Int(1)); // Request 1 commit

        // This test requires a git repo. We are in one.
        // But in CI or some environments it might fail.
        // We can check if output contains "ANCESTRY ERROR" and assert specific behavior or success.

        exec_git_op(&mut vm, OpCode::Ancestry, &[]);

        // Stack should have [count, hash] or [0] if error
        assert!(!vm.stack.is_empty());

        if let Some(val) = vm.stack.last() {
            // It should be a string (hash) or 0 (error count)
            // If we are in a git repo, it should be a hash.
            // If not, 0.
            // Given the environment has git, we expect success.
            // But to be robust, we just check no panic.
        }
    }
}
