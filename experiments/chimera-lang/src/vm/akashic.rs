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
fn get_akashic_path() -> String {
    std::env::var("CHIMERA_AKASHIC_FILE").unwrap_or_else(|_| ".chimera_akashic.json".to_string())
}

#[cfg(feature = "nova")]
fn get_lock_path() -> String {
    std::env::var("CHIMERA_LOCK_FILE").unwrap_or_else(|_| ".chimera_akashic.lock".to_string())
}

#[cfg(feature = "nova")]
struct FileLock {
    path: String,
}

#[cfg(feature = "nova")]
impl FileLock {
    fn lock() -> Self {
        let path = get_lock_path();
        let start = std::time::Instant::now();
        loop {
            if OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .is_ok()
            {
                return FileLock { path };
            }

            // Timeout after 60 seconds to prevent permanent deadlock if a process crashes
            // This is a safety valve, not a primary mechanism.
            if start.elapsed().as_secs() > 60 {
                let _ = std::fs::remove_file(&path);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

#[cfg(feature = "nova")]
impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(feature = "nova")]
fn load_records() -> HashMap<String, Value> {
    let path = get_akashic_path();
    if let Ok(mut file) = std::fs::File::open(path) {
        if let Ok(metadata) = file.metadata() {
            if metadata.len() > MAX_AKASHIC_SIZE {
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
fn save_records(records: &HashMap<String, Value>) {
    if let Ok(content) = serde_json::to_string_pretty(records) {
        let path = get_akashic_path();
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
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
                    let _lock = FileLock::lock();
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
                    let _lock = FileLock::lock();
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
        OpCode::AkashicAdd => {
            if vm.stack.len() >= 2 {
                let value = vm.stack.pop().unwrap();
                let key_val = vm.stack.pop().unwrap();

                if let Value::Str(key) = key_val {
                    if let Value::Int(inc) = value {
                        let _lock = FileLock::lock();
                        let mut records = load_records();
                        let current = match records.get(&key) {
                            Some(Value::Int(n)) => *n,
                            _ => 0,
                        };
                        records.insert(key.clone(), Value::Int(current + inc));
                        save_records(&records);
                        vm.output.push(format!("AKASHIC: Add {} to '{}'", inc, key));
                    } else {
                        vm.output.push("Error: Value must be Int for AkashicAdd".to_string());
                    }
                    vm.energy = vm.energy.saturating_sub(10);
                } else {
                    vm.output.push("Error: Key must be a string for AkashicAdd".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for AkashicAdd".to_string());
            }
        }
        _ => {}
    }
}
