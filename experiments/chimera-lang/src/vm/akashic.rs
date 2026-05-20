#![cfg(feature = "nova")]

use std::io::Read;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::ChimeraVM;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

/// Constant `AKASHIC_FILE`.
pub const AKASHIC_FILE: &str = ".chimera_akashic.json";
/// Constant `MAX_AKASHIC_SIZE`.
pub const MAX_AKASHIC_SIZE: u64 = 1024 * 1024; // 1MB limit for safety

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Represents a `AkashicSpore`.
pub struct AkashicSpore {
    /// The `dna_hash` field.
    pub dna_hash: u64,
    /// The `generation` field.
    pub generation: u64,
    /// The `energy` field.
    pub energy: i64,
    /// The `ip` field.
    pub ip: (usize, usize),
    #[serde(skip)]
    /// The `stack_snapshot` field.
    pub stack_snapshot: Vec<Value>, // Too complex to safely serialize/deserialize dynamically without bounds checking
    /// The `active_strands` field.
    pub active_strands: usize,
    /// The `telomeres` field.
    pub telomeres: Vec<i64>,
    /// The `ether` field.
    pub ether: HashMap<i64, std::collections::VecDeque<Value>>,
}

/// The Akashic Records represent a persistent storage mechanism for the ChimeraVM.
///
/// It acts as a key-value store (`storage`) that persists across VM runs, allowing agents
/// to leave messages, save states, or accumulate resources (like `karma`).
///
/// It also handles memory snapshots (`memories`) via `AkashicSave` and `AkashicLoad`,
/// though complex dynamic data (like full stack states) are omitted from serialization
/// to prevent DoS via deep recursion during parsing.
///
/// # Security
/// The records are protected by a strict size limit (`MAX_AKASHIC_SIZE`) to prevent
/// memory exhaustion attacks if an agent writes excessively large values. File
/// access uses atomic renames for safety.
#[derive(Serialize, Deserialize, Clone)]
/// Represents a `AkashicRecords`.
pub struct AkashicRecords {
    /// The `storage` field.
    pub storage: HashMap<String, Value>,
    /// The `karma` field.
    pub karma: i64,
    /// The `memories` field.
    pub memories: HashMap<String, AkashicSpore>,
    #[serde(skip)]
    /// The `corrupted` field.
    pub corrupted: bool,
    #[serde(skip)]
    /// The `file_path` field.
    pub file_path: String,
}

impl std::fmt::Debug for AkashicRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AkashicRecords {{ entries: {}, karma: {}, memories: {} }}",
            self.storage.len(),
            self.karma,
            self.memories.len()
        )
    }
}

impl std::fmt::Display for AkashicRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = comfy_table::Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec!["Attribute", "Value"]);

        table.add_row(vec![
            comfy_table::Cell::new("Karma").fg(comfy_table::Color::Yellow),
            comfy_table::Cell::new(self.karma.to_string()).fg(comfy_table::Color::Green),
        ]);

        table.add_row(vec![
            comfy_table::Cell::new("Memories").fg(comfy_table::Color::Cyan),
            comfy_table::Cell::new(self.memories.len().to_string()),
        ]);

        let integrity = if self.corrupted {
            "CORRUPTED"
        } else {
            "STABLE"
        };
        let integrity_color = if self.corrupted {
            comfy_table::Color::Yellow
        } else {
            comfy_table::Color::Green
        };

        table.add_row(vec![
            comfy_table::Cell::new("Integrity").fg(comfy_table::Color::Magenta),
            comfy_table::Cell::new(integrity).fg(integrity_color),
        ]);

        write!(f, "{}", table)?;

        if !self.storage.is_empty() {
            write!(f, "\n\n  [ Storage ]\n")?;
            let mut storage_table = comfy_table::Table::new();
            storage_table
                .load_preset(comfy_table::presets::UTF8_FULL)
                .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
                .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
                .set_header(vec!["Key", "Value"]);

            for (k, v) in &self.storage {
                storage_table.add_row(vec![
                    comfy_table::Cell::new(k).fg(comfy_table::Color::Cyan),
                    comfy_table::Cell::new(v.to_string()),
                ]);
            }
            write!(f, "{}", storage_table)?;
        }

        Ok(())
    }
}

impl Default for AkashicRecords {
    fn default() -> Self {
        Self::new()
    }
}

impl AkashicRecords {
    /// Creates a new `AkashicRecords` instance by attempting to load from the default file (`.chimera_akashic.json`).
    /// If the file does not exist or cannot be parsed, it initializes a fresh, empty record.
    /// If an alternative path is set via the environment variable `CHIMERA_AKASHIC_PATH`, it will use that instead.
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        let file_path = if let Ok(path) = std::env::var("CHIMERA_AKASHIC_PATH") {
            path
        } else {
            AKASHIC_FILE.to_string()
        };
        Self::load_from(&file_path).unwrap_or_else(|_| Self {
            storage: HashMap::new(),
            karma: 0,
            memories: HashMap::new(),
            corrupted: false,
            file_path: file_path.to_string(),
        })
    }

    /// Directly loads an Akashic Record from a specified JSON file path.
    ///
    /// # Panics
    /// Will not panic, but returns an `Err(String)` if the file exceeds the `MAX_AKASHIC_SIZE`
    /// or if the JSON is malformed.
    ///
    /// # Examples
    /// ```rust
    /// # fn main() {
    /// # use chimera_lang::vm::akashic::AkashicRecords;
    /// // Attempt to load a custom save file
    /// let result = AkashicRecords::load_from("my_save.json");
    /// if let Ok(records) = result {
    ///     println!("Loaded {}", records.storage.len());
    /// }
    /// # }
    /// ```
    /// Performs the `load_from` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of load_from
    /// ```
    pub fn load_from(file_path: &str) -> Result<Self, String> {
        match std::fs::File::open(file_path) {
            Ok(file) => {
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
                let limit = MAX_AKASHIC_SIZE;
                match file.take(limit + 1).read_to_string(&mut content,) {
                    Ok(bytes) if bytes as u64 <= limit => {
                        let mut records: Self =
                            serde_json::from_str(&content).unwrap_or_else(|_| Self {
                                storage: HashMap::new(),
                                karma: 0,
                                memories: HashMap::new(),
                                corrupted: true,
                                file_path: file_path.to_string(),
                            });
                        records.file_path = file_path.to_string();
                        Ok(records)
                    }
                    Ok(_) => Err(format!("Akashic Record too large (> {} bytes)", limit)),
                    Err(_) => Ok(Self {
                        storage: HashMap::new(),
                        karma: 0,
                        memories: HashMap::new(),
                        corrupted: true,
                        file_path: file_path.to_string(),
                    }),
                }
            }
            Err(_) => Ok(Self {
                storage: HashMap::new(),
                karma: 0,
                memories: HashMap::new(),
                corrupted: true,
                file_path: file_path.to_string(),
            }),
        }
    }

    /// Serializes the current state of the records and writes it to disk.
    ///
    /// The save operation uses an atomic write pattern (writing to a `.tmp` file
    /// and then renaming it) to prevent data corruption during crashes.
    ///
    /// # Examples
    /// ```rust
    /// # fn main() {
    /// # use chimera_lang::vm::akashic::AkashicRecords;
    /// # use chimera_lang::vm::Value;
    /// let mut records = AkashicRecords::new();
    /// records.storage.insert("Highscore".to_string(), Value::Int(9999));
    /// // Save immediately returns Ok(()) if successful
    /// // records.save();
    /// # }
    /// ```
    /// Performs the `save` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of save
    /// ```
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
            return Err("Akashic Record limit exceeded".to_string());
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
/// Performs the `exec_akashic_op` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_akashic_op
/// ```
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
                    let spore = crate::vm::akashic::AkashicSpore {
                        dna_hash: 0,
                        generation: 0,
                        energy: vm.energy,
                        ip: vm.ip,
                        stack_snapshot: vm.stack.clone(),
                        active_strands: vm.dna.helix.strands.len(),
                        telomeres: vm.telomeres.clone(),
                        ether: vm.ether.clone(),
                    };
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
                        vm.energy = spore.energy;
                        vm.ip = spore.ip;
                        vm.stack = spore.stack_snapshot.clone();
                        vm.telomeres = spore.telomeres.clone();
                        vm.ether = spore.ether.clone();
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
                    vm.synapse_map.push(Vec::with_capacity(4));
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
