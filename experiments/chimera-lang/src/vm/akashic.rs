#[cfg(feature = "nova")]
use super::nova_chronos::Spore;
#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, MAX_AKASHIC_SIZE};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
#[cfg(feature = "nova")]
use comfy_table::presets::UTF8_FULL;
#[cfg(feature = "nova")]
use comfy_table::Color;
#[cfg(feature = "nova")]
use comfy_table::ContentArrangement;
#[cfg(feature = "nova")]
use comfy_table::Table;
#[cfg(feature = "nova")]
use rand::Rng;
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
fn is_test_env() -> bool {
    cfg!(test)
        || std::env::var("CHIMERA_TEST").is_ok()
        || std::env::current_exe()
            .map(|p| p.to_string_lossy().contains("deps/chimera_lang-"))
            .unwrap_or(false)
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkashicRecords {
    pub storage: HashMap<String, Value>,
    pub karma: i64,
    #[serde(default)]
    pub memories: HashMap<String, Spore>,
    #[serde(skip)]
    pub corrupted: bool,
    #[serde(skip)]
    pub file_path: String,
}

#[cfg(feature = "nova")]
impl std::fmt::Display for AkashicRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["Attribute", "Value"]);

        // Karma
        let karma_color = if self.karma >= 0 {
            Color::Green
        } else {
            Color::Red
        };
        table.add_row(vec![
            comfy_table::Cell::new("Karma"),
            comfy_table::Cell::new(self.karma).fg(karma_color),
        ]);

        // Integrity
        let integrity_str = if self.corrupted {
            "CORRUPTED"
        } else {
            "STABLE"
        };
        let integrity_color = if self.corrupted {
            Color::Red
        } else {
            Color::Green
        };
        table.add_row(vec![
            comfy_table::Cell::new("Integrity"),
            comfy_table::Cell::new(integrity_str).fg(integrity_color),
        ]);

        // Storage Count
        table.add_row(vec!["Records", &self.storage.len().to_string()]);

        // Memories Count
        table.add_row(vec!["Memories", &self.memories.len().to_string()]);

        // Storage Detail (if any)
        if !self.storage.is_empty() {
            let mut sub_table = Table::new();
            sub_table.load_preset(UTF8_FULL);
            sub_table.set_header(vec!["Key", "Value"]);
            for (k, v) in self.storage.iter().take(5) {
                sub_table.add_row(vec![k, &format!("{:?}", v)]);
            }
            if self.storage.len() > 5 {
                sub_table.add_row(vec!["...", &format!("{} more", self.storage.len() - 5)]);
            }
            table.add_row(vec![
                comfy_table::Cell::new("Preview"),
                comfy_table::Cell::new(sub_table),
            ]);
        }

        write!(f, "{}", table)
    }
}

#[cfg(feature = "nova")]
impl AkashicRecords {
    pub fn new() -> Self {
        Self::load().unwrap_or_else(|_| Self {
            storage: HashMap::new(),
            karma: 0,
            memories: HashMap::new(),
            corrupted: true,
            file_path: if is_test_env() {
                format!(
                    ".chimera_akashic_test_{}.json",
                    rand::thread_rng().gen::<u64>()
                )
            } else {
                AKASHIC_FILE.to_string()
            },
        })
    }

    pub fn load() -> Result<Self, String> {
        let file_path = if is_test_env() {
            format!(
                ".chimera_akashic_test_{}.json",
                rand::thread_rng().gen::<u64>()
            )
        } else {
            AKASHIC_FILE.to_string()
        };
        Self::load_from(&file_path)
    }

    pub fn load_from(file_path: &str) -> Result<Self, String> {
        match std::fs::File::open(file_path) {
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
                    let mut records: Self = serde_json::from_str(&content)
                        .map_err(|e| format!("Parse Error: {}", e))?;
                    records.file_path = file_path.to_string();
                    Ok(records)
                } else {
                    Err("Read Error".to_string())
                }
            }
            Err(_) => Ok(Self {
                storage: HashMap::new(),
                karma: 0,
                memories: HashMap::new(),
                corrupted: false,
                file_path: file_path.to_string(),
            }),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        if self.corrupted {
            return Err(
                "Cannot save: Akashic Record is corrupted on disk. Fix manually.".to_string(),
            );
        }

        let path = if self.file_path.is_empty() {
            AKASHIC_FILE
        } else {
            &self.file_path
        };

        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;

        if content.len() as u64 > MAX_AKASHIC_SIZE {
            return Err(format!("Akashic Record limit exceeded"));
        }

        let temp_file = format!("{}.tmp", path);
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
        std::fs::rename(&temp_file, path).map_err(|e| e.to_string())
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

#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_akashic_display() {
        let mut records = AkashicRecords::new();
        records.karma = 100;
        records.corrupted = false;
        records.storage.insert("Wisdom".to_string(), Value::Int(42));

        let output = format!("{}", records);
        println!("{}", output);

        // Basic Checks
        assert!(output.contains("Attribute"));
        assert!(output.contains("Value"));

        // Karma (100)
        assert!(output.contains("Karma"));
        assert!(output.contains("100"));

        // Integrity (STABLE)
        assert!(output.contains("Integrity"));
        assert!(output.contains("STABLE"));

        // Storage content
        assert!(output.contains("Wisdom"));
        assert!(output.contains("42"));

        // Table formatting characters (from comfy-table)
        assert!(output.contains("╭"));
        assert!(output.contains("│"));
    }
}
