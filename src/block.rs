use crate::errors::{BlockchainError, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub const GENESIS_PREVIOUS_HASH: &'static str =
        "0000000000000000000000000000000000000000000000000000000000000000";

    pub fn genesis(difficulty: usize) -> Self {
        Self::mine(
            0,
            "Genesis Block".to_string(),
            Self::GENESIS_PREVIOUS_HASH.to_string(),
            difficulty,
        )
    }

    pub fn mine(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp();
        let target = "0".repeat(difficulty);
        let mut nonce = 0;

        let hash = loop {
            let candidate = Self::calculate_hash(index, timestamp, &data, &previous_hash, nonce);
            if candidate.starts_with(&target) {
                break candidate;
            }
            nonce += 1;
        };

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: i64,
        data: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{index}{timestamp}{data}{previous_hash}{nonce}");
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn recalculate_hash(&self) -> String {
        Self::calculate_hash(
            self.index,
            self.timestamp,
            &self.data,
            &self.previous_hash,
            self.nonce,
        )
    }

    pub fn validate(&self, difficulty: usize) -> Result<()> {
        if self.hash != self.recalculate_hash() {
            return Err(BlockchainError::InvalidHash { index: self.index });
        }

        if !self.hash.starts_with(&"0".repeat(difficulty)) {
            return Err(BlockchainError::DifficultyNotMet {
                index: self.index,
                difficulty,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_block_has_expected_prev_hash() {
        let block = Block::genesis(1);
        assert_eq!(block.previous_hash, Block::GENESIS_PREVIOUS_HASH);
        assert_eq!(block.index, 0);
    }

    #[test]
    fn mined_block_satisfies_difficulty() {
        let difficulty = 2;
        let block = Block::mine(1, "datos".to_string(), "abc".to_string(), difficulty);
        assert!(block.hash.starts_with("00"));
    }

    #[test]
    fn hash_is_deterministic() {
        let h1 = Block::calculate_hash(1, 100, "data", "prev", 42);
        let h2 = Block::calculate_hash(1, 100, "data", "prev", 42);
        assert_eq!(h1, h2);
    }

    #[test]
    fn tampered_data_invalidates_hash() {
        let mut block = Block::mine(1, "original".to_string(), "prev".to_string(), 1);
        block.data = "manipulado".to_string();
        assert!(block.validate(1).is_err());
    }
}
