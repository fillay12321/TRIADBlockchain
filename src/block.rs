use serde::{Serialize, Deserialize};
use crate::transaction::{Transaction, calculate_merkle_root};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use chrono::Utc;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub consensus_algorithm: String,
    #[serde(with = "serde_bytes")]
    pub signature: Option<Vec<u8>>,
    pub hash: Option<String>,
    pub nonce: u64,
    pub miner_reward: u64,
    pub transaction_fee: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: u64, transactions: Vec<Transaction>, consensus_algorithm: String) -> Self {
        Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            consensus_algorithm,
            signature: None,
            hash: None,
            nonce: 0,
            miner_reward: 50,
            transaction_fee: 5,
        }
    }
    
    pub fn calculate_merkle_root(&self) -> String {
        calculate_merkle_root(&self.transactions)
    }
    
    pub fn sign(&mut self, keypair: &Keypair) {
        let transactions_str = serde_json::to_string(&self.transactions).unwrap_or_default();
        let message = format!("{}{}{}{}", self.index, self.previous_hash, self.timestamp, transactions_str);
        let signature = keypair.sign(message.as_bytes());
        self.signature = Some(signature.to_bytes().to_vec());
    }
    
    pub fn verify_signature(&self, public_key: &PublicKey) -> bool {
        if let Some(sig_bytes) = &self.signature {
            if let Ok(signature) = Signature::from_bytes(sig_bytes) {
                let transactions_str = serde_json::to_string(&self.transactions).unwrap_or_default();
                let message = format!("{}{}{}{}", self.index, self.previous_hash, self.timestamp, transactions_str);
                return public_key.verify(message.as_bytes(), &signature).is_ok();
            }
        }
        false
    }
    
    pub fn mine(&mut self, difficulty: u64, last_block_time: i64) {
        let target = "0".repeat(difficulty as usize);
        let elapsed = Utc::now().timestamp() - last_block_time;
        let adjusted = if elapsed < 60 { difficulty.saturating_add(1) } else if elapsed > 120 { difficulty.saturating_sub(1) } else { difficulty };
        let merkle_root = self.calculate_merkle_root();
        self.nonce = (0..=u64::MAX).into_par_iter().find_first(|&nonce| {
            let mut hasher = Sha256::new();
            let input = format!("{}{}{}{}{}", self.index, self.previous_hash, self.timestamp, merkle_root, nonce);
            hasher.update(input);
            let hash = format!("{:x}", hasher.finalize());
            hash.starts_with(&target)
        }).unwrap_or(0);
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}{}", self.index, self.previous_hash, self.timestamp, merkle_root, self.nonce);
        hasher.update(input);
        self.hash = Some(format!("{:x}", hasher.finalize()));
    }
    
    pub fn is_unique_hash(blockchain: &[Block], hash: &str) -> bool {
        !blockchain.iter().any(|b| b.hash.as_deref() == Some(hash))
    }
    
    pub fn save_to_file(blocks: &[Block], path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, blocks)?;
        Ok(())
    }
    
    pub fn load_from_file(path: &str) -> io::Result<Vec<Block>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let blocks = serde_json::from_reader(reader)?;
        Ok(blocks)
    }
}
