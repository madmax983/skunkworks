#[cfg(feature = "nova")]
use crate::ast::{Dna, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::nova::OrganelleType;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Organelle, Value};
#[cfg(feature = "nova")]
use rand::{rngs::StdRng, Rng, SeedableRng};
#[cfg(feature = "nova")]
use std::collections::hash_map::DefaultHasher;
#[cfg(feature = "nova")]
use std::fs;
#[cfg(feature = "nova")]
use std::hash::{Hash, Hasher};

#[cfg(feature = "nova")]
pub fn exec_summon(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: filename_str (top)
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(filename) = val {
            // Sanitize filename: Just ensure it's a simple name
            if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
                vm.output
                    .push("SUMMON: Invalid filename (must be simple)".to_string());
                return None;
            }

            let path = vm.sandbox_root.join("bestiary").join(&filename);
            if !path.exists() {
                vm.output
                    .push(format!("SUMMON: File not found: {}", filename));
                return None;
            }

            match fs::read_to_string(&path) {
                Ok(content) => {
                    match crate::compiler::compile(
                        &content,
                        Some(&vm.sandbox_root.join("bestiary")),
                    ) {
                        Ok(mut new_dna) => {
                            // Relocate jumps
                            let offset = vm.dna.helix.strands.len();
                            relocate_dna(&mut new_dna, offset);

                            // Append to Helix
                            vm.dna.helix.strands.extend(new_dna.helix.strands);

                            // Reset telomeres/cortex for new strands
                            vm.telomeres.resize(vm.dna.helix.strands.len(), 50);
                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.resize(vm.dna.helix.strands.len(), 0);
                                vm.synapse_map
                                    .resize(vm.dna.helix.strands.len(), Vec::new());
                            }

                            // Register new strands in Cladistics
                            let start_idx = offset;
                            let end_idx = vm.dna.helix.strands.len();
                            for i in start_idx..end_idx {
                                vm.cladistics.register_strand(
                                    i,
                                    Some(vm.ip.0),
                                    vm.tick_counter,
                                    format!("Summon({})", filename),
                                );
                            }

                            // Spawn Organelle
                            if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
                                let target_strand = &vm.dna.helix.strands[offset];
                                let mut hasher = DefaultHasher::new();
                                target_strand.hash(&mut hasher);
                                let genome_id = hasher.finish();
                                let traits = analyze_traits(target_strand);
                                let name = generate_name(genome_id, &traits);

                                let organelle = Organelle {
                                    stack: Vec::new(),
                                    ip: (offset, 0),
                                    context_loc: vm.context_loc,
                                    call_stack: Vec::new(),
                                    recursion_depth: 0,
                                    halted: false,
                                    kind: OrganelleType::Worker, // Default summon type
                                    direction: (0, 0),
                                    ttl: None,
                                    name,
                                    traits,
                                    genome_id,
                                };
                                vm.organelles.push(organelle);
                                vm.energy = vm.energy.saturating_sub(50); // Cost
                                vm.output
                                    .push(format!("SUMMON: {} emerged from {}", filename, offset));
                            } else {
                                vm.output
                                    .push("SUMMON: Organelle limit reached".to_string());
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("SUMMON ERROR: {}", e));
                        }
                    }
                }
                Err(e) => {
                    vm.output.push(format!("SUMMON IO ERROR: {}", e));
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for summon".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for summon".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn relocate_dna(dna: &mut Dna, offset: usize) {
    if offset == 0 {
        return;
    }
    let offset_i64 = offset as i64;

    for strand in &mut dna.helix.strands {
        for gene in &mut strand.genes {
            match gene.op {
                OpCode::Jump | OpCode::Brz | OpCode::Call => {
                    if !gene.args.is_empty() {
                        if let Nucleotide::Number(n) = &mut gene.args[0] {
                            // Relocate only if it looks like a valid index (>=0)
                            if *n >= 0 {
                                *n += offset_i64;
                            }
                        }
                    }
                }
                // Note: Stack-based ops cannot be relocated statically.
                // Immediate args for Spawn/Mitosis are rare (usually stack).
                _ => {}
            }
        }
    }
}

#[cfg(feature = "nova")]
pub fn analyze_traits(strand: &Strand) -> Vec<String> {
    let mut traits = Vec::new();
    let mut op_counts = std::collections::HashMap::new();

    for gene in &strand.genes {
        *op_counts.entry(gene.op.clone()).or_insert(0) += 1;
    }

    if let Some(&count) = op_counts.get(&OpCode::Consume) {
        if count > 2 {
            traits.push("Voracious".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Photosynthesize) {
        if count > 0 {
            traits.push("Autotrophic".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Migrate) {
        if count > 0 {
            traits.push("Nomadic".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::GWrite) {
        if count > 0 {
            traits.push("Constructive".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Virus) {
        if count > 0 {
            traits.push("Parasitic".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::SporeCloud) {
        if count > 0 {
            traits.push("Fungal".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::EgregoreSummon) {
        if count > 0 {
            traits.push("Cultist".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Sacrifice) {
        if count > 0 {
            traits.push("Zealot".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Piet) {
        if count > 0 {
            traits.push("Artistic".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Sing) {
        if count > 0 {
            traits.push("Melodic".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Dream) {
        if count > 0 {
            traits.push("Dreamer".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::QuantumJump) {
        if count > 0 {
            traits.push("Phased".to_string());
        }
    }

    if traits.is_empty() {
        traits.push("Dormant".to_string());
    }

    traits
}

#[cfg(feature = "nova")]
pub fn generate_name(seed: u64, traits: &[String]) -> String {
    let mut rng = StdRng::seed_from_u64(seed);

    let prefixes = [
        "Greater", "Lesser", "Ancient", "Neon", "Void", "Star", "Cyber", "Fungal", "Crystal",
        "Shadow", "Radiant", "Toxic", "Quantum", "Echo",
    ];

    let nouns = [
        "Slime",
        "Wisp",
        "Golem",
        "Spore",
        "Drone",
        "Spirit",
        "Beast",
        "Construct",
        "Virus",
        "Echo",
        "Wraith",
        "Titan",
        "Larva",
        "Wyrm",
    ];

    let suffix = if traits.contains(&"Voracious".to_string()) {
        "Devourer"
    } else if traits.contains(&"Parasitic".to_string()) {
        "Leech"
    } else if traits.contains(&"Constructive".to_string()) {
        "Architect"
    } else if traits.contains(&"Melodic".to_string()) {
        "Siren"
    } else if traits.contains(&"Dreamer".to_string()) {
        "Visionary"
    } else {
        nouns[rng.gen_range(0..nouns.len())]
    };

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];

    format!("{} {}", prefix, suffix)
}

#[cfg(feature = "nova")]
pub fn generate_face(seed: u64, traits: &[String]) -> Vec<String> {
    let mut rng = StdRng::seed_from_u64(seed);

    let eyes = if traits.contains(&"Voracious".to_string()) {
        ["o", "o"]
    } else if traits.contains(&"Dreamer".to_string()) {
        ["-", "-"]
    } else if traits.contains(&"Zealot".to_string()) {
        ["x", "x"]
    } else if traits.contains(&"Phased".to_string()) {
        ["@", "@"]
    } else {
        [".", "."]
    };

    let mouth = if traits.contains(&"Voracious".to_string()) {
        "ww"
    } else if traits.contains(&"Melodic".to_string()) {
        "o"
    } else if traits.contains(&"Aggressive".to_string()) {
        "^^"
    } else {
        "-"
    };

    let horns = if rng.gen_bool(0.3) { "/\\" } else { "  " };

    vec![
        format!("  {}  ", horns),
        format!(
            " ({}{}{}) ",
            eyes[0],
            if rng.gen_bool(0.1) { "*" } else { " " },
            eyes[1]
        ),
        format!("  \\{}/  ", mouth),
        format!("   --   "),
    ]
}
