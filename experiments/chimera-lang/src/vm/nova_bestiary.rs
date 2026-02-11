#[cfg(feature = "nova")]
use crate::ast::Strand;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use rand::{rngs::StdRng, Rng, SeedableRng};

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
    if let Some(&count) = op_counts.get(&OpCode::Augury) {
        if count > 0 {
            traits.push("Prophetic".to_string());
        }
    }
    if let Some(&count) = op_counts.get(&OpCode::Divinate) {
        if count > 0 {
            traits.push("Prophetic".to_string());
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
    } else if traits.contains(&"Prophetic".to_string()) {
        "Oracle"
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
    } else if traits.contains(&"Prophetic".to_string()) {
        ["*", "*"]
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

#[cfg(feature = "nova")]
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub hp: i64,
    pub max_hp: i64,
    pub attack: i64,
    pub defense: i64,
    pub speed: i64,
}

#[cfg(feature = "nova")]
pub fn generate_combat_stats(traits: &[String]) -> CombatStats {
    let mut hp = 100;
    let mut attack = 10;
    let mut defense = 5;
    let mut speed = 5;

    for t in traits {
        match t.as_str() {
            "Voracious" => {
                hp += 20;
                attack += 2;
            }
            "Parasitic" => {
                attack += 5;
                defense -= 2;
            }
            "Constructive" => {
                defense += 5;
                speed -= 1;
            }
            "Nomadic" => {
                speed += 5;
            }
            "Phased" => {
                defense += 10;
                hp -= 10;
            }
            "Zealot" => {
                attack += 10;
                defense -= 5;
            }
            "Fungal" => {
                hp += 50;
                speed -= 2;
            }
            "Dreamer" => {
                // Glass cannon
                attack += 15;
                hp -= 20;
            }
            _ => {}
        }
    }

    CombatStats {
        hp,
        max_hp: hp,
        attack,
        defense,
        speed,
    }
}
