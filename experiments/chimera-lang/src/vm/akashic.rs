#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, MAX_AKASHIC_SIZE};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use std::collections::HashMap;
#[cfg(feature = "nova")]
use std::fs::OpenOptions;
#[cfg(feature = "nova")]
use std::io::{Read, Write};

#[cfg(feature = "nova")]
const AKASHIC_FILE: &str = ".chimera_akashic.json";

#[cfg(feature = "nova")]
fn load_records() -> Result<HashMap<String, Value>, String> {
    match std::fs::File::open(AKASHIC_FILE) {
        Ok(file) => {
            let mut handle = file.take(MAX_AKASHIC_SIZE + 1);
            let mut content = String::new();
            if handle.read_to_string(&mut content).is_ok() {
                if content.len() as u64 > MAX_AKASHIC_SIZE {
                    return Err("Database too large".to_string());
                }
                match serde_json::from_str(&content) {
                    Ok(map) => Ok(map),
                    Err(_) => Err("Database corrupted".to_string()),
                }
            } else {
                Err("Failed to read database".to_string())
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashMap::new()),
        Err(_) => Err("Failed to open database".to_string()),
    }
}

#[cfg(feature = "nova")]
fn save_records(records: &HashMap<String, Value>) -> Result<(), String> {
    match serde_json::to_string_pretty(records) {
        Ok(content) => {
            if content.len() as u64 > MAX_AKASHIC_SIZE {
                return Err("Database full".to_string());
            }
            let temp_file = format!("{}.tmp", AKASHIC_FILE);
            match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&temp_file)
            {
                Ok(mut file) => {
                    if file.write_all(content.as_bytes()).is_ok() {
                        if std::fs::rename(&temp_file, AKASHIC_FILE).is_ok() {
                             Ok(())
                        } else {
                             let _ = std::fs::remove_file(&temp_file);
                             Err("Failed to commit database".to_string())
                        }
                    } else {
                         let _ = std::fs::remove_file(&temp_file);
                         Err("Failed to write to database".to_string())
                    }
                },
                Err(_) => Err("Failed to create temp database".to_string()),
            }
        },
        Err(_) => Err("Failed to serialize records".to_string()),
    }
}

#[cfg(feature = "nova")]
pub fn exec_akashic_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::AkashicWrite => {
            if vm.stack.len() >= 2 {
                let value = vm.stack.pop().unwrap();
                let key_val = vm.stack.pop().unwrap();

                if let Value::Str(key) = key_val {
                    match load_records() {
                        Ok(mut records) => {
                            records.insert(key.clone(), value);
                            match save_records(&records) {
                                Ok(()) => {
                                    vm.output.push(format!("AKASHIC: Wrote '{}'", key));
                                    vm.energy = vm.energy.saturating_sub(10);
                                },
                                Err(e) => vm.output.push(format!("Error: {}", e)),
                            }
                        },
                        Err(e) => vm.output.push(format!("Error: {}", e)),
                    }
                } else {
                    vm.output
                        .push("Error: Key must be a string for AkashicWrite".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for AkashicWrite".to_string());
            }
        }
        OpCode::AkashicRead => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(key) = val {
                    match load_records() {
                        Ok(records) => {
                            if let Some(value) = records.get(&key) {
                                vm.stack.push(value.clone());
                                vm.output.push(format!("AKASHIC: Read '{}'", key));
                            } else {
                                vm.stack.push(Value::Int(0)); // Default if missing
                                vm.output.push(format!("AKASHIC: Key '{}' not found", key));
                            }
                            vm.energy = vm.energy.saturating_sub(5);
                        },
                        Err(e) => {
                            vm.stack.push(Value::Int(0));
                            vm.output.push(format!("Error: {}", e));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Key must be a string for AkashicRead".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for AkashicRead".to_string());
            }
        }
        _ => {}
    }
}
