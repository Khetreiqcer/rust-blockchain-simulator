use crate::block::{Block, Transaction};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_transaction = Transaction {
            sender: "System".to_string(),
            receiver: "Genesis".to_string(),
            amount: 0.0,
        };

        let mut genesis_block = Block::new(
            0,
            vec![genesis_transaction],
            "0".to_string(),
        );

        genesis_block.mine_block(self.difficulty);
        self.chain.push(genesis_block);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
        );

        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, filename: &str) {
        let serialized = serde_json::to_string(&self).expect("Failed to serialize blockchain");
        fs::write(filename, serialized).expect("Failed to save blockchain to file");
    }

    pub fn load_from_file(filename: &str) -> Option<Self> {
        let data = fs::read_to_string(filename).ok()?;
        serde_json::from_str(&data).ok()
    }
}
