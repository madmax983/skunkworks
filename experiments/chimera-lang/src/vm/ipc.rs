#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use std::fs;
#[cfg(feature = "nova")]
use std::path::Path;
#[cfg(feature = "nova")]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(feature = "nova")]
use rand::Rng;

#[cfg(feature = "nova")]
const ETHER_DIR: &str = ".chimera_ether";

#[cfg(feature = "nova")]
pub fn signal(vm: &mut ChimeraVM) {
    // stack: channel, value
    if vm.stack.len() >= 2 {
        let value = vm.stack.pop().unwrap();
        let channel_val = vm.stack.pop().unwrap();

        if let Value::Int(channel) = channel_val {
            let channel_dir = Path::new(ETHER_DIR).join(channel.to_string());
            if fs::create_dir_all(&channel_dir).is_ok() {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_micros();
                let mut rng = rand::thread_rng();
                let random_suffix: u32 = rng.gen();
                let filename = format!("{}_{}.json", timestamp, random_suffix);
                let filepath = channel_dir.join(filename);

                if let Ok(content) = serde_json::to_string(&value) {
                    if fs::write(filepath, content).is_ok() {
                        vm.output
                            .push(format!("SIGNAL: Sent to channel {}", channel));
                        vm.energy = vm.energy.saturating_sub(10);
                    } else {
                        vm.output
                            .push("Error: Failed to write to ether".to_string());
                    }
                }
            } else {
                vm.output
                    .push("Error: Failed to access ether directory".to_string());
            }
        } else {
            vm.output.push("Error: Channel must be Int".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for signal".to_string());
    }
}

#[cfg(feature = "nova")]
pub fn receive(vm: &mut ChimeraVM) {
    // stack: channel
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(channel) = val {
            let channel_dir = Path::new(ETHER_DIR).join(channel.to_string());
            if channel_dir.exists() {
                if let Ok(entries) = fs::read_dir(channel_dir) {
                    let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                    // Sort by filename (timestamp)
                    files.sort_by_key(|e| e.file_name());

                    let mut found = false;
                    for entry in files {
                        let path = entry.path();
                        // Attempt to lock by renaming
                        let lock_path = path.with_extension("json.lock");
                        if fs::rename(&path, &lock_path).is_ok() {
                            if let Ok(content) = fs::read_to_string(&lock_path) {
                                if let Ok(value) = serde_json::from_str::<Value>(&content) {
                                    vm.stack.push(value);
                                    // Consume message
                                    let _ = fs::remove_file(lock_path);
                                    vm.output.push(format!("RECEIVE: Read from channel {}", channel));
                                    vm.energy = vm.energy.saturating_sub(5);
                                    found = true;
                                    break;
                                } else {
                                    // Corrupt but locked, delete it
                                    let _ = fs::remove_file(lock_path);
                                }
                            } else {
                                // Locked but unreadable?
                                let _ = fs::remove_file(lock_path);
                            }
                        }
                        // If rename failed, someone else took it, continue loop
                    }

                    if !found {
                        // No messages
                        vm.stack.push(Value::Int(0));
                        vm.output
                            .push(format!("RECEIVE: Channel {} empty", channel));
                    }
                } else {
                     vm.output.push("Error: Failed to read ether directory".to_string());
                }
            } else {
                vm.stack.push(Value::Int(0));
                vm.output
                    .push(format!("RECEIVE: Channel {} empty", channel));
            }
        } else {
            vm.output.push("Error: Channel must be Int".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for receive".to_string());
    }
}
