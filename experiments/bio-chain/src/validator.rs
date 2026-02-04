use crate::types::*;
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::ChimeraVM,
};

pub struct Validator {
    pub id: ValidatorId,
    pub vm: ChimeraVM,
    pub energy: i64,
    pub lineage: Vec<BlockHash>,
    pub fitness: f64,
    pub blocks_validated: usize,
    pub correct_votes: usize,
    pub total_votes: usize,
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
        }
    }

    /// Create validator from parent via mitosis
    pub fn from_parent(parent: &Validator, new_id: ValidatorId) -> Self {
        // Clone parent's DNA and create new VM
        let child_dna = parent.vm.dna.clone();
        let child_vm = ChimeraVM::new(child_dna);
        // TODO: Apply small mutations to child DNA

        Validator {
            id: new_id,
            vm: child_vm,
            energy: parent.energy / 2, // Split energy
            lineage: parent.lineage.clone(),
            fitness: parent.fitness,
            blocks_validated: 0,
            correct_votes: 0,
            total_votes: 0,
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

    /// Vote on a block by secreting hormones
    pub fn vote_on_block(&mut self, block: &Block, is_valid: bool) {
        let channel = block.hash_as_channel();
        let vote_strength = self.energy.min(100); // Vote proportional to stake

        if is_valid {
            // TODO: Call vm.secrete(channel, vote_strength)
            self.total_votes += 1;
        } else {
            // Secrete on INVALID channel
            // TODO: vm.secrete(INVALID_CHANNEL, vote_strength)
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
}
