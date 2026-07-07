use crate::block::Block;
use crate::errors::{BlockchainError, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        Blockchain {
            chain: vec![Block::genesis(difficulty)],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: impl Into<String>) -> Result<&Block> {
        let previous = self.chain.last().ok_or(BlockchainError::EmptyChain)?;
        let new_block = Block::mine(
            previous.index + 1,
            data.into(),
            previous.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
        Ok(self.chain.last().unwrap())
    }

    pub fn validate(&self) -> Result<()> {
        if self.chain.is_empty() {
            return Err(BlockchainError::EmptyChain);
        }

        self.chain[0].validate(self.difficulty)?;

        for window in self.chain.windows(2) {
            let (previous, current) = (&window[0], &window[1]);

            current.validate(self.difficulty)?;

            if current.previous_hash != previous.hash {
                return Err(BlockchainError::BrokenLink {
                    index: current.index,
                });
            }
        }
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    pub fn blocks(&self) -> &[Block] {
        &self.chain
    }

    /** Mutable access to blocks
     *
     * Thats for testing purposes, simiklar to attacks modifying the blockchain,
     *  this should not be used in production code
     */
    pub fn blocks_mut(&mut self) -> &mut [Block] {
        &mut self.chain
    }

    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    /**
     * save chain as JSON to the specified file path
     */
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string(&self.chain)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: impl AsRef<Path>, difficulty: usize) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let chain: Vec<Block> = serde_json::from_str(&json)?;
        Ok(Blockchain { chain, difficulty })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chain_has_only_genesis() {
        let chain = Blockchain::new(1);
        assert_eq!(chain.len(), 1);
        assert!(chain.is_valid());
    }

    #[test]
    fn adding_blocks_keeps_chain_valid() {
        let mut chain = Blockchain::new(1);
        chain.add_block("tx1").unwrap();
        chain.add_block("tx2").unwrap();
        assert_eq!(chain.len(), 3);
        assert!(chain.is_valid());
    }

    #[test]
    fn tampering_breaks_validation() {
        let mut chain = Blockchain::new(1);
        chain.add_block("tx1").unwrap();

        // Alteramos los datos de un bloque intermedio sin re-minarlo.
        let blocks = &mut chain.chain;
        blocks[1].data = "tx_falsa".to_string();

        assert!(!chain.is_valid());
        match chain.validate() {
            Err(BlockchainError::InvalidHash { index }) => assert_eq!(index, 1),
            other => panic!("se esperaba InvalidHash, se obtuvo {other:?}"),
        }
    }

    #[test]
    fn broken_link_is_detected() {
        let mut chain = Blockchain::new(1);
        chain.add_block("tx1").unwrap();
        chain.add_block("tx2").unwrap();

        // Re-minamos el bloque 1 con datos distintos: su hash cambia,
        // pero el bloque 2 sigue apuntando al hash viejo.
        chain.chain[1] = Block::mine(1, "otra_cosa".to_string(), chain.chain[0].hash.clone(), 1);

        assert!(!chain.is_valid());
        match chain.validate() {
            Err(BlockchainError::BrokenLink { index }) => assert_eq!(index, 2),
            other => panic!("se esperaba BrokenLink, se obtuvo {other:?}"),
        }
    }

    #[test]
    fn save_and_load_roundtrip() {
        let mut chain = Blockchain::new(1);
        chain.add_block("tx1").unwrap();

        let path = std::env::temp_dir().join("mini_blockchain_test.json");
        chain.save_to_file(&path).unwrap();

        let loaded = Blockchain::load_from_file(&path, 1).unwrap();
        assert_eq!(loaded.len(), chain.len());
        assert!(loaded.is_valid());

        let _ = std::fs::remove_file(path);
    }
}
