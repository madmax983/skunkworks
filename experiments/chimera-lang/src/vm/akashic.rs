#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
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
fn load_records() -> HashMap<String, Value> {
    if let Ok(mut file) = std::fs::File::open(AKASHIC_FILE) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            if let Ok(map) = serde_json::from_str(&content) {
                return map;
            }
        }
    }
    HashMap::new()
}

#[cfg(feature = "nova")]
fn save_records(records: &HashMap<String, Value>) {
    if let Ok(content) = serde_json::to_string_pretty(records) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(AKASHIC_FILE)
        {
            let _ = file.write_all(content.as_bytes());
        }
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
                    let mut records = load_records();
                    records.insert(key.clone(), value);
                    save_records(&records);
                    vm.output.push(format!("AKASHIC: Wrote '{}'", key));
                    vm.energy = vm.energy.saturating_sub(10);
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
                    let records = load_records();
                    if let Some(value) = records.get(&key) {
                        vm.stack.push(value.clone());
                        vm.output.push(format!("AKASHIC: Read '{}'", key));
                    } else {
                        vm.stack.push(Value::Int(0)); // Default if missing
                        vm.output.push(format!("AKASHIC: Key '{}' not found", key));
                    }
                    vm.energy = vm.energy.saturating_sub(5);
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
