use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type BlockHash = [u8; 32];
pub type Address = [u8; 20];
pub type ValidatorId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub fee: u64,
}

impl Transaction {
    pub fn hash(&self) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).unwrap());
        hasher.finalize().into()
    }

    pub fn is_valid(&self) -> bool {
        self.amount > 0 && self.fee > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: BlockHash,
    pub parent: BlockHash,
    pub height: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proposer: ValidatorId,
    pub hormone_proof: HormoneProof,
}

impl Block {
    pub fn new(
        parent: BlockHash,
        height: u64,
        transactions: Vec<Transaction>,
        proposer: ValidatorId,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut block = Block {
            hash: [0; 32],
            parent,
            height,
            timestamp,
            transactions,
            proposer,
            hormone_proof: HormoneProof::empty(),
        };

        block.hash = block.compute_hash();
        block
    }

    fn compute_hash(&self) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(&self.parent);
        hasher.update(self.height.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        for tx in &self.transactions {
            hasher.update(tx.hash());
        }
        hasher.update(self.proposer.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn total_fees(&self) -> u64 {
        self.transactions.iter().map(|tx| tx.fee).sum()
    }

    pub fn hash_as_channel(&self) -> i64 {
        // Convert hash to channel ID for hormones
        i64::from_le_bytes(self.hash[0..8].try_into().unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneProof {
    pub channel: i64,
    pub final_level: i64,
    pub voters: Vec<ValidatorId>,
}

impl HormoneProof {
    pub fn empty() -> Self {
        HormoneProof {
            channel: 0,
            final_level: 0,
            voters: vec![],
        }
    }
}
