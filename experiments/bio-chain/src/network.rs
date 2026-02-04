use crate::types::*;
use crate::validator::Validator;
use std::collections::HashMap;

/// Consensus threshold: % of total stake needed to finalize
const CONSENSUS_THRESHOLD: f64 = 0.67;

/// How much hormone decays per tick
const HORMONE_DECAY: i64 = 5;

/// Energy threshold for reproduction
const MITOSIS_THRESHOLD: i64 = 1200;

/// Energy cost to create child
const MITOSIS_COST: i64 = 300;

/// Energy cost per tick (metabolism)
const METABOLISM_COST: i64 = 5;

pub struct Network {
    pub validators: Vec<Validator>,
    pub pending_blocks: Vec<Block>,
    pub finalized_chain: Vec<Block>,
    pub hormone_levels: HashMap<i64, i64>, // channel -> total hormone
    pub hormone_voters: HashMap<i64, Vec<ValidatorId>>, // channel -> voters
    pub tick: u64,
    pub next_validator_id: ValidatorId,
    pub births: usize,
    pub deaths: usize,
}

impl Network {
    pub fn new() -> Self {
        Network {
            validators: vec![],
            pending_blocks: vec![],
            finalized_chain: vec![],
            hormone_levels: HashMap::new(),
            hormone_voters: HashMap::new(),
            tick: 0,
            next_validator_id: 0,
            births: 0,
            deaths: 0,
        }
    }

    pub fn add_validator(&mut self, validator: Validator) {
        if validator.id >= self.next_validator_id {
            self.next_validator_id = validator.id + 1;
        }
        self.validators.push(validator);
    }

    /// Total staked energy in the network
    pub fn total_stake(&self) -> i64 {
        self.validators.iter().map(|v| v.energy).sum()
    }

    /// Calculate consensus threshold
    fn consensus_threshold(&self) -> i64 {
        (self.total_stake() as f64 * CONSENSUS_THRESHOLD) as i64
    }

    /// Propose a new block
    pub fn propose_block(&mut self, proposer_id: ValidatorId, transactions: Vec<Transaction>) {
        let parent = self
            .finalized_chain
            .last()
            .map(|b| b.hash)
            .unwrap_or([0; 32]);
        let height = self.finalized_chain.len() as u64;

        let block = Block::new(parent, height, transactions, proposer_id);
        let channel = block.hash_as_channel();

        println!(
            "📢 Validator {} proposes block {} (channel: {})",
            proposer_id, height, channel
        );

        // Proposer votes for their own block (initial hormone)
        let proposer_stake = self.validators[proposer_id].energy;
        self.hormone_levels
            .entry(channel)
            .and_modify(|e| *e += proposer_stake)
            .or_insert(proposer_stake);
        self.hormone_voters
            .entry(channel)
            .or_insert_with(Vec::new)
            .push(proposer_id);

        println!(
            "  💉 Proposer secretes {} (total: {})",
            proposer_stake,
            self.hormone_levels[&channel]
        );

        self.pending_blocks.push(block);
    }

    /// Validators vote on pending blocks
    pub fn vote_on_blocks(&mut self) {
        let blocks = self.pending_blocks.clone();

        for block in &blocks {
            let channel = block.hash_as_channel();

            // Each validator votes
            for i in 0..self.validators.len() {
                // Skip if already voted
                if self
                    .hormone_voters
                    .get(&channel)
                    .map(|voters| voters.contains(&i))
                    .unwrap_or(false)
                {
                    continue;
                }

                let is_malicious = self.validators[i].is_malicious;

                // Validate block (simplified for now)
                let actual_validity = !block.transactions.is_empty();

                // Malicious validators vote OPPOSITE of validity
                let vote_valid = if is_malicious {
                    !actual_validity
                } else {
                    actual_validity
                };

                if vote_valid {
                    let vote_strength = self.validators[i].energy;

                    // Vote by secreting hormone
                    self.hormone_levels
                        .entry(channel)
                        .and_modify(|e| *e += vote_strength)
                        .or_insert(vote_strength);
                    self.hormone_voters
                        .entry(channel)
                        .or_insert_with(Vec::new)
                        .push(i);

                    self.validators[i].total_votes += 1;

                    let marker = if is_malicious { "🔴" } else { "✅" };
                    println!(
                        "  {} Validator {} votes {} (strength: {}, total: {})",
                        marker,
                        i,
                        if is_malicious { "INVALID" } else { "YES" },
                        vote_strength,
                        self.hormone_levels[&channel]
                    );
                }
            }
        }
    }

    /// Check for consensus and finalize blocks
    pub fn check_consensus(&mut self) {
        let threshold = self.consensus_threshold();
        let mut finalized_blocks = vec![];

        // Collect blocks that reached consensus
        let pending = self.pending_blocks.clone();
        for block in &pending {
            let channel = block.hash_as_channel();
            let level = *self.hormone_levels.get(&channel).unwrap_or(&0);

            if level >= threshold {
                println!(
                    "🎉 CONSENSUS REACHED! Block {} finalized ({}/{} threshold)",
                    block.height, level, threshold
                );

                // Create hormone proof
                let mut block_with_proof = block.clone();
                block_with_proof.hormone_proof = HormoneProof {
                    channel,
                    final_level: level,
                    voters: self.hormone_voters.get(&channel).cloned().unwrap_or_default(),
                };

                finalized_blocks.push(block_with_proof);
            }
        }

        // Reward and finalize
        for block in finalized_blocks {
            self.reward_validators(&block);
            self.finalized_chain.push(block.clone());

            // Remove from pending
            self.pending_blocks.retain(|b| b.hash != block.hash);
        }
    }

    /// Validate a block using validator's DNA
    fn validate_block(&mut self, validator: &mut Validator, block: &Block) -> bool {
        // Basic validation
        if block.transactions.is_empty() {
            return false;
        }

        // Validate each transaction
        for tx in &block.transactions {
            if !validator.validate_transaction(tx) {
                return false;
            }
        }

        true
    }

    /// Reward validators who voted for finalized block
    fn reward_validators(&mut self, block: &Block) {
        let total_fees = block.total_fees();

        // Only reward HONEST voters (malicious validators wasted their vote)
        let honest_voters: Vec<_> = block
            .hormone_proof
            .voters
            .iter()
            .filter(|&&id| !self.validators.get(id).map(|v| v.is_malicious).unwrap_or(false))
            .cloned()
            .collect();

        if honest_voters.is_empty() {
            return;
        }

        let reward_per_voter = total_fees / honest_voters.len() as u64;

        for voter_id in &honest_voters {
            if let Some(validator) = self.validators.get_mut(*voter_id) {
                validator.earn_fees(reward_per_voter);
                println!(
                    "  💰 Validator {} earns {} fees (energy: {})",
                    voter_id, reward_per_voter, validator.energy
                );
            }
        }

        // Malicious voters get nothing (wasted energy on invalid vote)
        let malicious_voters: Vec<_> = block
            .hormone_proof
            .voters
            .iter()
            .filter(|&&id| self.validators.get(id).map(|v| v.is_malicious).unwrap_or(false))
            .cloned()
            .collect();

        for voter_id in malicious_voters {
            println!("  ⚠️  Validator {} (malicious) earns nothing", voter_id);
        }
    }

    /// Decay hormones (proposals timeout)
    pub fn decay_hormones(&mut self) {
        for level in self.hormone_levels.values_mut() {
            *level = (*level - HORMONE_DECAY).max(0);
        }

        // Remove dead proposals
        self.hormone_levels.retain(|_, v| *v > 0);
    }

    /// Metabolism: validators burn energy each tick
    pub fn metabolism(&mut self) {
        for validator in &mut self.validators {
            validator.consume_energy(METABOLISM_COST);
        }
    }

    /// Reproduction: successful validators undergo mitosis
    pub fn reproduce_successful(&mut self) {
        let mut new_validators = vec![];

        for validator in &self.validators {
            if validator.can_reproduce(MITOSIS_THRESHOLD) {
                let child_id = self.next_validator_id;
                self.next_validator_id += 1;

                let child = Validator::from_parent(validator, child_id);
                println!(
                    "🧬 MITOSIS! Validator {} → Validator {} (energy: {} → {})",
                    validator.id,
                    child_id,
                    validator.energy,
                    validator.energy - MITOSIS_COST
                );

                new_validators.push((validator.id, child));
            }
        }

        // Add children and deduct cost from parents
        for (parent_id, child) in new_validators {
            if let Some(parent) = self.validators.iter_mut().find(|v| v.id == parent_id) {
                parent.consume_energy(MITOSIS_COST);
            }
            self.births += 1;
            self.validators.push(child);
        }
    }

    /// Death: validators with no energy undergo apoptosis
    pub fn kill_failures(&mut self) {
        let initial_count = self.validators.len();

        self.validators.retain(|v| {
            if v.is_dead() {
                let marker = if v.is_malicious { "🔴💀" } else { "💀" };
                println!(
                    "{} APOPTOSIS! Validator {} died (malicious: {}, fitness: {:.2})",
                    marker, v.id, v.is_malicious, v.fitness
                );
                self.deaths += 1;
                false
            } else {
                true
            }
        });

        let killed = initial_count - self.validators.len();
        if killed > 0 {
            println!("  ⚰️  {} validators eliminated", killed);
        }
    }

    /// Step the network forward one tick
    pub fn step(&mut self) {
        self.tick += 1;
        println!("\n⏱️  Tick {}", self.tick);

        // Metabolism (validators burn energy)
        self.metabolism();

        // Decay hormones
        self.decay_hormones();

        // Validators vote on pending blocks
        self.vote_on_blocks();

        // Check for consensus
        self.check_consensus();

        // Natural selection
        self.reproduce_successful();
        self.kill_failures();
    }

    /// Print network status
    pub fn status(&self) {
        println!("\n📊 Network Status:");
        println!(
            "  Validators: {} (total stake: {})",
            self.validators.len(),
            self.total_stake()
        );
        println!("  Pending blocks: {}", self.pending_blocks.len());
        println!("  Finalized blocks: {}", self.finalized_chain.len());
        println!("  Active proposals: {}", self.hormone_levels.len());
        println!("  Consensus threshold: {}", self.consensus_threshold());
        println!("  Births: {} | Deaths: {}", self.births, self.deaths);

        if !self.validators.is_empty() {
            let avg_fitness: f64 = self.validators.iter().map(|v| v.fitness).sum::<f64>()
                / self.validators.len() as f64;
            let avg_energy: i64 = self.total_stake() / self.validators.len() as i64;
            println!("  Avg Fitness: {:.2} | Avg Energy: {}", avg_fitness, avg_energy);
        }
    }
}
