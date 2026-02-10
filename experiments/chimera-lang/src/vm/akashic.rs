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
            if let Ok(metadata) = file.metadata() {
                if metadata.len() > MAX_AKASHIC_SIZE {
                    return Err(format!(
                        "Akashic Record too large (> {} bytes)",
                        MAX_AKASHIC_SIZE
                    ));
                }
            }
            let mut content = String::new();
            // Use take to strictly enforce limit even if metadata lied
            if file
                .take(MAX_AKASHIC_SIZE)
                .read_to_string(&mut content)
                .is_ok()
            {
                serde_json::from_str(&content).map_err(|e| format!("Akashic Parse Error: {}", e))
            } else {
                Err("Failed to read Akashic Record".to_string())
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashMap::new()),
        Err(e) => Err(format!("Failed to open Akashic Record: {}", e)),
    }
}

#[cfg(feature = "nova")]
fn save_records(records: &HashMap<String, Value>) -> Result<(), String> {
    let content = serde_json::to_string_pretty(records).map_err(|e| e.to_string())?;

    if content.len() as u64 > MAX_AKASHIC_SIZE {
        return Err(format!(
            "Akashic Record exceeds limit ({} > {})",
            content.len(),
            MAX_AKASHIC_SIZE
        ));
    }

    let temp_file = format!("{}.tmp", AKASHIC_FILE);
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_file)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;
        file.sync_all()
            .map_err(|e| format!("Failed to sync temp file: {}", e))?;
    }

    std::fs::rename(&temp_file, AKASHIC_FILE)
        .map_err(|e| format!("Failed to commit Akashic Record: {}", e))
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
                                Ok(_) => {
                                    vm.output.push(format!("AKASHIC: Wrote '{}'", key));
                                    vm.energy = vm.energy.saturating_sub(10);
                                }
                                Err(e) => {
                                    vm.output.push(format!("Error: Akashic Write Failed: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("Error: Akashic Load Failed: {}", e));
                        }
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
                        }
                        Err(e) => {
                            vm.output.push(format!("Error: Akashic Load Failed: {}", e));
                            vm.stack.push(Value::Int(0)); // Maintain stack balance
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
