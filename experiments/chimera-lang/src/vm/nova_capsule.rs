#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::HashMap;
#[cfg(feature = "nova")]
use std::fs::OpenOptions;
#[cfg(feature = "nova")]
use std::io::{Read, Write};
#[cfg(feature = "nova")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "nova")]
const CAPSULE_FILE: &str = ".chimera_capsule.json";
#[cfg(feature = "nova")]
const MAX_CAPSULE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capsule {
    pub value: Value,
    pub unlock_time: u64,
}

#[cfg(feature = "nova")]
fn load_capsules() -> HashMap<String, Capsule> {
    if let Ok(mut file) = std::fs::File::open(CAPSULE_FILE) {
        if let Ok(metadata) = file.metadata() {
            if metadata.len() > MAX_CAPSULE_SIZE {
                return HashMap::new();
            }
        }
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
fn save_capsules(capsules: &HashMap<String, Capsule>) {
    if let Ok(content) = serde_json::to_string_pretty(capsules) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(CAPSULE_FILE)
        {
            let _ = file.write_all(content.as_bytes());
        }
    }
}

#[cfg(feature = "nova")]
pub fn exec_capsule_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Encapsulate => exec_encapsulate(vm),
        OpCode::Decapsulate => exec_decapsulate(vm),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn exec_encapsulate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., key, value, duration ] -> [ ... ]
    if vm.stack.len() >= 3 {
        let duration_val = vm.stack.pop().unwrap();
        let value = vm.stack.pop().unwrap();
        let key_val = vm.stack.pop().unwrap();

        if let (Value::Str(key), Value::Int(duration)) = (key_val, duration_val) {
            if duration >= 0 {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let unlock_time = now + duration as u64;

                let mut capsules = load_capsules();
                capsules.insert(key.clone(), Capsule { value, unlock_time });
                save_capsules(&capsules);

                vm.output
                    .push(format!("ENCAPSULATE: '{}' locked for {}s", key, duration));
                vm.energy = vm.energy.saturating_sub(20);
            } else {
                vm.output
                    .push("Error: Negative duration for encapsulate".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for encapsulate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for encapsulate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_decapsulate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., key ] -> [ ..., value_or_wait_time ]
    if let Some(key_val) = vm.stack.pop() {
        if let Value::Str(key) = key_val {
            let capsules = load_capsules();
            if let Some(capsule) = capsules.get(&key) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                if now >= capsule.unlock_time {
                    vm.stack.push(capsule.value.clone());
                    vm.output.push(format!("DECAPSULATE: '{}' opened", key));
                } else {
                    let remaining = capsule.unlock_time.saturating_sub(now);
                    vm.stack.push(Value::Int(remaining as i64));
                    vm.output.push(format!(
                        "DECAPSULATE: '{}' locked (wait {}s)",
                        key, remaining
                    ));
                }
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.stack.push(Value::Int(0)); // Void
                vm.output.push(format!("DECAPSULATE: '{}' not found", key));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for decapsulate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for decapsulate".to_string());
    }
    None
}
