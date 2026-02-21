#[cfg(feature = "nova")]
use super::nova_chronos::Spore;
#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, MAX_AKASHIC_SIZE};
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
const AKASHIC_FILE: &str = ".chimera_akashic.json";

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkashicRecords {
    pub storage: HashMap<String, Value>,
    pub karma: i64,
    #[serde(default)]
    pub memories: HashMap<String, Spore>,
    #[serde(skip)]
    pub corrupted: bool,
}

#[cfg(feature = "nova")]
impl AkashicRecords {
    pub fn new() -> Self {
        Self::load().unwrap_or_else(|_| Self {
            storage: HashMap::new(),
            karma: 0,
            memories: HashMap::new(),
            corrupted: true,
        })
    }

    pub fn load() -> Result<Self, String> {
        match std::fs::File::open(AKASHIC_FILE) {
            Ok(mut file) => {
                // Check size
                if let Ok(metadata) = file.metadata() {
                    if metadata.len() > MAX_AKASHIC_SIZE {
                        return Err(format!(
                            "Akashic Record too large (> {} bytes)",
                            MAX_AKASHIC_SIZE
                        ));
                    }
                }

                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    serde_json::from_str(&content).map_err(|e| format!("Parse Error: {}", e))
                } else {
                    Err("Read Error".to_string())
                }
            }
            Err(_) => Ok(Self {
                storage: HashMap::new(),
                karma: 0,
                memories: HashMap::new(),
                corrupted: false,
            }),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        if self.corrupted {
            return Err(
                "Cannot save: Akashic Record is corrupted on disk. Fix manually.".to_string(),
            );
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;

        if content.len() as u64 > MAX_AKASHIC_SIZE {
            return Err(format!("Akashic Record limit exceeded"));
        }

        let temp_file = format!("{}.tmp", AKASHIC_FILE);
        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&temp_file)
                .map_err(|e| e.to_string())?;
            file.write_all(content.as_bytes())
                .map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
        }
        std::fs::rename(&temp_file, AKASHIC_FILE).map_err(|e| e.to_string())
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
                    vm.akashic.storage.insert(key.clone(), value);
                    if let Err(e) = vm.akashic.save() {
                        vm.output.push(format!("AKASHIC ERROR: {}", e));
                    } else {
                        vm.output.push(format!("AKASHIC: Wrote '{}'", key));
                    }
                    vm.energy = vm.energy.saturating_sub(10);
                } else {
                    vm.output.push("Error: Key must be string".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow".to_string());
            }
        }
        OpCode::AkashicSave => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(key) = val {
                    let spore = super::nova_chronos::create_spore(vm);
                    vm.akashic.memories.insert(key.clone(), spore);
                    if let Err(e) = vm.akashic.save() {
                        vm.output.push(format!("AKASHIC ERROR: {}", e));
                    } else {
                        vm.output.push(format!("AKASHIC: Saved Memory '{}'", key));
                    }
                    vm.energy = vm.energy.saturating_sub(100);
                } else {
                    vm.output.push("Error: Key must be string".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow".to_string());
            }
        }
        OpCode::AkashicLoad => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(key) = val {
                    if let Some(spore) = vm.akashic.memories.get(&key).cloned() {
                        super::nova_chronos::restore_state(vm, &spore);
                        vm.output
                            .push(format!("AKASHIC: Restored Memory '{}'", key));
                    } else {
                        vm.output
                            .push(format!("AKASHIC: Memory '{}' not found", key));
                    }
                } else {
                    vm.output.push("Error: Key must be string".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow".to_string());
            }
        }
        OpCode::AkashicRead => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(key) = val {
                    if let Some(v) = vm.akashic.storage.get(&key) {
                        vm.stack.push(v.clone());
                        vm.output.push(format!("AKASHIC: Read '{}'", key));
                    } else {
                        vm.stack.push(Value::Int(0));
                        vm.output.push(format!("AKASHIC: Key '{}' not found", key));
                    }
                    vm.energy = vm.energy.saturating_sub(5);
                } else {
                    vm.output.push("Error: Key must be string".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow".to_string());
            }
        }
        OpCode::Karma => {
            if let Some(Value::Int(amount)) = vm.stack.pop() {
                vm.akashic.karma = vm.akashic.karma.saturating_add(amount);
                let _ = vm.akashic.save();
                vm.output.push(format!("KARMA: Total {}", vm.akashic.karma));
            }
        }
        OpCode::Miracle => {
            if let Some(Value::Int(id)) = vm.stack.pop() {
                let cost = match id {
                    0 => 1000,  // Resurrection
                    1 => 5000,  // Terraform
                    2 => 2000,  // Wealth
                    3 => 500,   // Cleanse
                    4 => 10000, // Ascension
                    _ => 0,
                };

                if cost > 0 && vm.akashic.karma >= cost {
                    vm.akashic.karma -= cost;
                    perform_miracle(vm, id);
                    let _ = vm.akashic.save();
                } else if cost > 0 {
                    vm.output
                        .push(format!("MIRACLE: Insufficient Karma (Need {})", cost));
                } else {
                    vm.output.push("MIRACLE: Unknown ID".to_string());
                }
            }
        }
        _ => {}
    }
}

#[cfg(feature = "nova")]
fn perform_miracle(vm: &mut ChimeraVM, id: i64) {
    match id {
        0 => {
            // Resurrection
            vm.output.push("MIRACLE: The Dead Rise!".to_string());
            while let Some(strand) = vm.graveyard.pop() {
                vm.dna.helix.strands.push(strand);
                vm.telomeres.push(100);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }
            }
        }
        1 => {
            // Terraform
            vm.output.push("MIRACLE: A New World!".to_string());
            for row in vm.biome_grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = crate::vm::nova_biome::Biome::Garden;
                }
            }
        }
        2 => {
            // Wealth
            vm.output.push("MIRACLE: Abundance!".to_string());
            vm.energy = 5000; // Massive energy boost
        }
        3 => {
            // Cleanse
            vm.output.push("MIRACLE: Purification!".to_string());
            for row in vm.viral_grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = None;
                }
            }
            for row in vm.entropy_grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = 0;
                }
            }
        }
        4 => {
            // Ascension
            vm.output.push("MIRACLE: ASCENSION ACHIEVED!".to_string());
            vm.output
                .push("You have transcended the simulation.".to_string());
            vm.halted = true;
        }
        _ => {}
    }
}
