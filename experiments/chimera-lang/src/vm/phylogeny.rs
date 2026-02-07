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
use std::path::{Component, Path, PathBuf};

#[cfg(feature = "phylogeny")]
fn sanitize_path(root: &Path, user_path: &str) -> Option<PathBuf> {
    let path = Path::new(user_path);
    // Reject absolute paths
    if path.is_absolute() {
        return None;
    }

    // Check for suspicious components
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => {} // Allowed if resolved within root, but checked later
            _ => return None,          // RootDir, Prefix, etc.
        }
    }

    // Join and Canonicalize
    let full_path = root.join(path);

    // Get canonical root for comparison
    let root_canon = root.canonicalize().ok()?;

    // If path exists, check canonical path
    if full_path.exists() {
        if let Ok(canon) = full_path.canonicalize() {
            if canon.starts_with(&root_canon) {
                return Some(canon);
            }
        }
        return None;
    }

    // If path doesn't exist (write), check parent
    if let Some(parent) = full_path.parent() {
        // If parent exists, we can canonicalize it
        if parent.exists() {
            if let Ok(canon_parent) = parent.canonicalize() {
                if canon_parent.starts_with(&root_canon) {
                    // Parent is safe.
                    return Some(canon_parent.join(full_path.file_name()?));
                }
            }
        }
    }

    None
}

#[cfg(feature = "phylogeny")]
pub fn exec_phylogeny_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Crawl => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(path_str) = val {
                    if let Some(path) = sanitize_path(&vm.sandbox_root, &path_str) {
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
                                vm.stack.push(Value::Junction(crate::ast::JunctionType::All, files));
                                vm.output.push(format!("CRAWL: Scanned {}", path_str));
                            }
                            Err(e) => {
                                vm.output.push(format!("CRAWL ERROR: {}", e));
                                vm.stack.push(Value::Junction(crate::ast::JunctionType::Any, vec![]));
                            }
                        }
                    } else {
                        vm.output.push("CRAWL ERROR: Path Access Denied".to_string());
                        vm.stack.push(Value::Junction(crate::ast::JunctionType::Any, vec![]));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for crawl".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for crawl".to_string());
            }
        }
        OpCode::Sequencing => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(path_str) = val {
                    if let Some(path) = sanitize_path(&vm.sandbox_root, &path_str) {
                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                vm.stack.push(Value::Str(content));
                                vm.output.push(format!("SEQUENCING: Read {}", path_str));
                            }
                            Err(e) => {
                                vm.output.push(format!("SEQUENCING ERROR: {}", e));
                                vm.stack.push(Value::Str("".to_string()));
                            }
                        }
                    } else {
                        vm.output.push("SEQUENCING ERROR: Path Access Denied".to_string());
                        vm.stack.push(Value::Str("".to_string()));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for sequencing".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for sequencing".to_string());
            }
        }
        OpCode::Synthesize => {
            if vm.stack.len() >= 2 {
                let content_val = vm.stack.pop().unwrap();
                let path_val = vm.stack.pop().unwrap();
                if let (Value::Str(path_str), Value::Str(content)) = (path_val, content_val) {
                    if let Some(path) = sanitize_path(&vm.sandbox_root, &path_str) {
                        match fs::write(&path, content) {
                            Ok(_) => vm.output.push(format!("SYNTHESIZE: Wrote to {}", path_str)),
                            Err(e) => vm.output.push(format!("SYNTHESIZE ERROR: {}", e)),
                        }
                    } else {
                        vm.output.push("SYNTHESIZE ERROR: Path Access Denied".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for synthesize".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for synthesize".to_string());
            }
        }
        OpCode::Infect => {
            if vm.stack.len() >= 2 {
                let content_val = vm.stack.pop().unwrap();
                let path_val = vm.stack.pop().unwrap();
                if let (Value::Str(path_str), Value::Str(content)) = (path_val, content_val) {
                    if let Some(path) = sanitize_path(&vm.sandbox_root, &path_str) {
                        let mut file = match fs::OpenOptions::new().append(true).create(true).open(&path) {
                            Ok(f) => f,
                            Err(e) => {
                                vm.output.push(format!("INFECT ERROR: {}", e));
                                return;
                            }
                        };
                        if let Err(e) = write!(file, "{}", content) {
                            vm.output.push(format!("INFECT ERROR: {}", e));
                        } else {
                            vm.output.push(format!("INFECT: Appended to {}", path_str));
                        }
                    } else {
                        vm.output.push("INFECT ERROR: Path Access Denied".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for infect".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for infect".to_string());
            }
        }
        OpCode::Shell => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(_cmd_str) = val {
                    vm.output.push("SHELL: Command execution disabled for security".to_string());
                    vm.stack.push(Value::Str("SHELL DISABLED".to_string()));
                } else {
                    vm.output.push("Error: Type mismatch for shell".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for shell".to_string());
            }
        }
        _ => {}
    }
}
