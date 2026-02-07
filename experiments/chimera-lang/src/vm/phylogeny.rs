#[cfg(feature = "phylogeny")]
use super::{ChimeraVM, Value};
#[cfg(feature = "phylogeny")]
use crate::ast::Nucleotide;
#[cfg(feature = "phylogeny")]
use crate::opcode::OpCode;
#[cfg(feature = "phylogeny")]
use std::fs;
#[cfg(feature = "phylogeny")]
use std::io::Write;
#[cfg(feature = "phylogeny")]
use std::process::Command;

#[cfg(feature = "phylogeny")]
pub fn exec_phylogeny_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Crawl => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(path) = val {
                    match fs::read_dir(&path) {
                        Ok(entries) => {
                            let mut files = Vec::new();
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    if let Ok(name) = entry.file_name().into_string() {
                                        files.push(Value::Str(name));
                                    }
                                }
                            }
                            vm.stack
                                .push(Value::Junction(crate::ast::JunctionType::All, files));
                            vm.output.push(format!("CRAWL: Scanned {}", path));
                        }
                        Err(e) => {
                            vm.output.push(format!("CRAWL ERROR: {}", e));
                            vm.stack
                                .push(Value::Junction(crate::ast::JunctionType::Any, vec![]));
                            // Empty junction on error
                        }
                    }
                } else {
                    vm.output.push("Error: Type mismatch for crawl".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for crawl".to_string());
            }
        }
        OpCode::Sequencing => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(path) = val {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            vm.stack.push(Value::Str(content));
                            vm.output.push(format!("SEQUENCING: Read {}", path));
                        }
                        Err(e) => {
                            vm.output.push(format!("SEQUENCING ERROR: {}", e));
                            vm.stack.push(Value::Str("".to_string()));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for sequencing".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sequencing".to_string());
            }
        }
        OpCode::Synthesize => {
            if vm.stack.len() >= 2 {
                let content_val = vm.stack.pop().unwrap();
                let path_val = vm.stack.pop().unwrap();
                if let (Value::Str(path), Value::Str(content)) = (path_val, content_val) {
                    match fs::write(&path, content) {
                        Ok(_) => vm.output.push(format!("SYNTHESIZE: Wrote to {}", path)),
                        Err(e) => vm.output.push(format!("SYNTHESIZE ERROR: {}", e)),
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for synthesize".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for synthesize".to_string());
            }
        }
        OpCode::Infect => {
            if vm.stack.len() >= 2 {
                let content_val = vm.stack.pop().unwrap();
                let path_val = vm.stack.pop().unwrap();
                if let (Value::Str(path), Value::Str(content)) = (path_val, content_val) {
                    let mut file =
                        match fs::OpenOptions::new().append(true).create(true).open(&path) {
                            Ok(f) => f,
                            Err(e) => {
                                vm.output.push(format!("INFECT ERROR: {}", e));
                                return;
                            }
                        };
                    if let Err(e) = write!(file, "{}", content) {
                        vm.output.push(format!("INFECT ERROR: {}", e));
                    } else {
                        vm.output.push(format!("INFECT: Appended to {}", path));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for infect".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for infect".to_string());
            }
        }
        OpCode::Shell => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(cmd_str) = val {
                    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
                    if parts.is_empty() {
                        vm.output.push("SHELL: Empty command".to_string());
                        vm.stack.push(Value::Str("".to_string()));
                        return;
                    }

                    let cmd = parts[0];
                    let args = &parts[1..];

                    match Command::new(cmd).args(args).output() {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                            if !output.stderr.is_empty() {
                                vm.output.push(format!(
                                    "SHELL STDERR: {}",
                                    String::from_utf8_lossy(&output.stderr)
                                ));
                            }
                            vm.stack.push(Value::Str(stdout));
                            vm.output.push(format!("SHELL: Executed {}", cmd_str));
                        }
                        Err(e) => {
                            vm.output.push(format!("SHELL ERROR: {}", e));
                            vm.stack.push(Value::Str("".to_string()));
                        }
                    }
                } else {
                    vm.output.push("Error: Type mismatch for shell".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for shell".to_string());
            }
        }
        _ => {}
    }
}
