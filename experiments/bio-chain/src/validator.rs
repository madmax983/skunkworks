use crate::types::*;
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::ChimeraVM,
};

#[cfg(feature = "nova")]
use crate::virus::{ViralEffect, Virus};

pub struct Validator {
    pub id: ValidatorId,
    pub vm: ChimeraVM,
    pub energy: i64,
    pub lineage: Vec<BlockHash>,
    pub fitness: f64,
    pub blocks_validated: usize,
    pub correct_votes: usize,
    pub total_votes: usize,
    pub is_malicious: bool, // Byzantine validator
    #[cfg(feature = "nova")]
    pub infections: Vec<Virus>,
}

impl Validator {
    /// Create a genesis validator with default validation DNA
    pub fn genesis(id: ValidatorId, initial_energy: i64) -> Self {
        let dna = Self::default_validator_dna();
        let vm = ChimeraVM::new(dna);

        Validator {
            id,
            vm,
            energy: initial_energy,
            lineage: vec![],
            fitness: 1.0,
            blocks_validated: 0,
            correct_votes: 0,
            total_votes: 0,
            is_malicious: false,
            #[cfg(feature = "nova")]
            infections: vec![],
        }
    }

    /// Create a malicious (Byzantine) validator
    pub fn malicious(id: ValidatorId, initial_energy: i64) -> Self {
        let dna = Self::default_validator_dna();
        let vm = ChimeraVM::new(dna);

        Validator {
            id,
            vm,
            energy: initial_energy,
            lineage: vec![],
            fitness: 1.0,
            blocks_validated: 0,
            correct_votes: 0,
            total_votes: 0,
            is_malicious: true,
            #[cfg(feature = "nova")]
            infections: vec![],
        }
    }

    /// Create validator from parent via mitosis
    pub fn from_parent(parent: &Validator, new_id: ValidatorId) -> Self {
        // Clone parent's DNA and apply mutations
        let mut child_dna = parent.vm.dna.clone();
        Self::mutate_dna(&mut child_dna, 0.05); // 5% mutation rate
        let child_vm = ChimeraVM::new(child_dna);

        #[cfg(feature = "nova")]
        let child_infections = parent.infections.clone(); // Pass infections to child

        Validator {
            id: new_id,
            vm: child_vm,
            energy: parent.energy / 2, // Split energy
            lineage: parent.lineage.clone(),
            fitness: parent.fitness,
            blocks_validated: 0,
            correct_votes: 0,
            total_votes: 0,
            is_malicious: parent.is_malicious, // Children inherit behavior
            #[cfg(feature = "nova")]
            infections: child_infections,
        }
    }

    /// Apply random mutations to DNA (genetic variation)
    fn mutate_dna(dna: &mut Dna, mutation_rate: f64) {
        use chimera_lang::opcode::OpCode;
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for strand in &mut dna.helix.strands {
            for gene in &mut strand.genes {
                // Randomly mutate gene arguments
                if rng.r#gen::<f64>() < mutation_rate {
                    // Mutate nucleotide values
                    for arg in &mut gene.args {
                        if let Nucleotide::Number(n) = arg {
                            // Add small random variation (-10% to +10%)
                            let variation = (*n as f64 * 0.1) as i64;
                            let delta = rng.gen_range(-variation.max(1)..=variation.max(1));
                            *n = (*n + delta).max(0); // Keep non-negative
                        }
                    }
                }

                // Very rarely, mutate the opcode itself
                if rng.r#gen::<f64>() < mutation_rate * 0.1 {
                    // For now, just toggle between similar opcodes
                    // In a full implementation, we'd have a more sophisticated mutation strategy
                    gene.op = match &gene.op {
                        OpCode::Push => OpCode::Dup,
                        OpCode::Dup => OpCode::Push,
                        OpCode::Add => OpCode::Sub,
                        OpCode::Sub => OpCode::Add,
                        _ => gene.op.clone(), // Keep others unchanged
                    };
                }
            }
        }
    }

    /// Default DNA for honest validators
    fn default_validator_dna() -> Dna {
        // Strand 0: Transaction validation logic
        // For now, simple: check if transaction has positive amount and fee
        let validation_strand = Strand {
            genes: vec![
                // Input: transaction on stack as data
                // Output: 1 if valid, 0 if invalid
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)], // Assume valid for now
                },
            ],
        };

        // Strand 1: Block proposal (not used yet)
        let proposal_strand = Strand { genes: vec![] };

        // Strand 2: Consensus voting
        // Receives block_hash channel, validates, then votes
        let voting_strand = Strand { genes: vec![] };

        Dna {
            helix: Helix {
                strands: vec![validation_strand, proposal_strand, voting_strand],
            },
        }
    }

    /// Validate a transaction using DNA
    pub fn validate_transaction(&mut self, _tx: &Transaction) -> bool {
        // For MVP, just do simple validation
        // Later: execute Strand 0 DNA with tx as input
        true // Simplified
    }

    /// Vote on a block (hormone secretion handled by network layer)
    ///
    /// Note: Actual hormone aggregation happens in Network::vote_on_blocks()
    /// which collects votes from all validators and updates the shared hormone_pool.
    /// This method just validates and tracks voting statistics.
    pub fn vote_on_block(&mut self, block: &Block, is_valid: bool) {
        let _channel = block.hash_as_channel();
        let _vote_strength = self.energy; // Vote proportional to stake

        // Malicious validators vote randomly or opposite
        let actual_vote = if self.is_malicious {
            !is_valid // Byzantine behavior: vote opposite
        } else {
            is_valid
        };

        // Track voting statistics
        if actual_vote {
            self.total_votes += 1;
        } else {
            self.total_votes += 1;
        }
    }

    /// Earn fees from successful validation
    pub fn earn_fees(&mut self, amount: u64) {
        self.energy += amount as i64;
        self.correct_votes += 1;
        self.blocks_validated += 1;
        self.update_fitness();
    }

    /// Lose energy (failed validation, time passing)
    pub fn consume_energy(&mut self, amount: i64) {
        self.energy -= amount;
    }

    /// Check if validator can reproduce
    pub fn can_reproduce(&self, threshold: i64) -> bool {
        #[cfg(feature = "nova")]
        if self.has_infection(&ViralEffect::Sterility) {
            return false;
        }
        self.energy > threshold
    }

    /// Check if validator is dead
    pub fn is_dead(&self) -> bool {
        self.energy <= 0
    }

    /// Update fitness score
    fn update_fitness(&mut self) {
        if self.total_votes > 0 {
            let accuracy = self.correct_votes as f64 / self.total_votes as f64;
            let productivity = self.blocks_validated as f64;
            self.fitness = accuracy * productivity.ln().max(1.0);
        }
    }

    #[cfg(feature = "nova")]
    pub fn infect(&mut self, virus: Virus) {
        // Check if already infected with this virus (by name)
        if !self.infections.iter().any(|v| v.name == virus.name) {
            self.infections.push(virus);
        }
    }

    #[cfg(feature = "nova")]
    pub fn has_infection(&self, effect: &ViralEffect) -> bool {
        self.infections.iter().any(|v| v.effect == *effect)
    }
}
