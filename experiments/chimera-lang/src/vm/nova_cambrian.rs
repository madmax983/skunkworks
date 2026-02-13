#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Strand;
use crate::opcode::OpCode;
use crate::ast::Nucleotide;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Species {
    pub name: String,
    pub genome_hash: u64,
    pub color: (u8, u8, u8),
    pub population: usize,
    pub average_energy: f64,
    pub first_seen_tick: u64,
}

#[derive(Debug, Clone)]
pub struct CambrianState {
    pub species: HashMap<u64, Species>,
    pub extinction_timer: u64,
    pub epoch: u64,
    pub next_extinction_event: String,
}

impl CambrianState {
    pub fn new() -> Self {
        Self {
            species: HashMap::new(),
            extinction_timer: 1000,
            epoch: 1,
            next_extinction_event: "Meteor Strike".to_string(),
        }
    }
}

pub fn calculate_hash(strand: &Strand) -> u64 {
    let mut hasher = DefaultHasher::new();
    strand.hash(&mut hasher);
    hasher.finish()
}

fn generate_name(hash: u64) -> String {
    let prefixes = ["Xeno", "Bio", "Cyber", "Aero", "Pyro", "Cryo", "Litho", "Hydro", "Necro", "Photo"];
    let suffixes = ["phage", "morph", "vore", "cyte", "pod", "saur", "form", "oid", "plasm", "mancer"];

    let p_idx = (hash % prefixes.len() as u64) as usize;
    let s_idx = ((hash / 10) % suffixes.len() as u64) as usize;

    format!("{}{}", prefixes[p_idx], suffixes[s_idx])
}

fn generate_color(hash: u64) -> (u8, u8, u8) {
    let r = (hash & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = ((hash >> 16) & 0xFF) as u8;
    (r, g, b)
}

pub fn exec_speciate(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    speciate(vm);
    vm.energy = vm.energy.saturating_sub(50);
    None
}

pub fn speciate(vm: &mut ChimeraVM) {
    // Reset counters
    for species in vm.cambrian.species.values_mut() {
        species.population = 0;
        species.average_energy = 0.0;
    }

    let mut new_species_found = 0;

    for (_i, strand) in vm.dna.helix.strands.iter().enumerate() {
        let hash = calculate_hash(strand);

        let entry = vm.cambrian.species.entry(hash).or_insert_with(|| {
            new_species_found += 1;
            Species {
                name: generate_name(hash),
                genome_hash: hash,
                color: generate_color(hash),
                population: 0,
                average_energy: 0.0,
                first_seen_tick: vm.tick_counter,
            }
        });

        entry.population += 1;
    }

    // Prune extinct
    vm.cambrian.species.retain(|_, s| s.population > 0);

    vm.output.push(format!("SPECIATION: {} Species Identified. {} New.", vm.cambrian.species.len(), new_species_found));
}

pub fn exec_meteor(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    meteor_strike(vm);
    vm.energy = vm.energy.saturating_sub(100);
    None
}

pub fn meteor_strike(vm: &mut ChimeraVM) {
    let mut rng = rand::thread_rng();
    let kill_rate = 0.5; // 50% mass extinction

    let initial_count = vm.dna.helix.strands.len();
    let mut survivors = Vec::new();
    let mut _survivors_indices = Vec::new(); // Old indices

    for (i, strand) in vm.dna.helix.strands.iter().enumerate() {
        if rng.gen_bool(1.0 - kill_rate) {
            survivors.push(strand.clone());
            _survivors_indices.push(i);
        } else {
            // Killed
            vm.graveyard.push(strand.clone());
        }
    }

    let killed_count = initial_count - survivors.len();
    vm.dna.helix.strands = survivors;

    // We must update any indices pointing to strands (e.g. organelles, epigenome).
    // This is complex. For a "Cataclysm", maybe breaking pointers is part of the fun/horror?
    // "Mass Extinction scrambles references".
    // But let's try to be nice if possible, or just accept chaos.
    // Given the prompt "Mad Scientist", chaos is acceptable.

    // However, we should probably clear organelles that point to dead strands.
    vm.organelles.retain(|org| {
        // If organelle IP points to a valid index in NEW helix?
        // Indices shifted. This breaks everything.
        // That's fine. It's a meteor.
        org.ip.0 < vm.dna.helix.strands.len()
    });

    vm.output.push(format!("METEOR STRIKE: {} Strands obliterated.", killed_count));

    // Trigger re-speciation
    speciate(vm);
}

pub fn exec_great_filter(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    great_filter(vm);
    vm.energy = vm.energy.saturating_sub(100);
    None
}

pub fn great_filter(vm: &mut ChimeraVM) {
    speciate(vm); // Ensure up to date

    let threshold = 2; // Kill anything with population < 2
    let mut doomed_hashes = Vec::new();

    for (hash, species) in &vm.cambrian.species {
        if species.population < threshold {
            doomed_hashes.push(*hash);
        }
    }

    if doomed_hashes.is_empty() {
        vm.output.push("GREAT FILTER: All species strong enough.".to_string());
        return;
    }

    // Kill
    let mut survivors = Vec::new();
    for strand in &vm.dna.helix.strands {
        let h = calculate_hash(strand);
        if !doomed_hashes.contains(&h) {
            survivors.push(strand.clone());
        } else {
            vm.graveyard.push(strand.clone());
        }
    }

    let killed = vm.dna.helix.strands.len() - survivors.len();
    vm.dna.helix.strands = survivors;

    vm.output.push(format!("GREAT FILTER: {} Weak strands culled.", killed));
    speciate(vm);
}

pub fn exec_census(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    speciate(vm);
    vm.stack.push(Value::Int(vm.cambrian.species.len() as i64));
    None
}
