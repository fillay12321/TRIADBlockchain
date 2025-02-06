use sha2::{Sha256, Digest};
use log::{info, debug, warn, error};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;  
use chrono::Utc; 


#[derive(Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transaction: String,
    pub consensus_algorithm: String,
    pub signature: Option<Signature>,
    pub hash: Option<String>,
    pub nonce: u64,
    pub miner_reward: u64,  
    pub transaction_fee: u64,  
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: u64, transaction: String, consensus_algorithm: String) -> Block {
        Block {
            index,
            previous_hash,
            timestamp,
            transaction,
            consensus_algorithm,
            signature: None,
            hash: None,
            nonce: 0,
            miner_reward: 50,  
            transaction_fee: 5,  
        }
    }

    pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u64, transaction: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}{}", index, previous_hash, timestamp, transaction, nonce);
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn sign(&mut self, keypair: &Keypair) {
        let message = format!("{}{}{}{}", self.index, self.previous_hash, self.timestamp, self.transaction);
        let signature = keypair.sign(message.as_bytes());
        self.signature = Some(signature);
        
        let timestamp = Utc::now().to_rfc3339();
        info!(
            "Block {} signed at {} with signature: {}",
            self.index, timestamp, self.signature.unwrap_or_else(|| Signature::from_bytes(&[0u8; 64]).unwrap())
        );
    }

    pub fn verify_signature(&self, public_key: &PublicKey) -> bool {
        if let Some(signature) = &self.signature {
            let message = format!("{}{}{}{}", self.index, self.previous_hash, self.timestamp, self.transaction);
            public_key.verify(message.as_bytes(), signature).is_ok()
        } else {
            false
        }
    }

    pub fn mine(&mut self, difficulty: u64) {
        let target = "0".repeat(difficulty as usize); 
        let timestamp = Utc::now().to_rfc3339();
        info!("Mining block {} started at {}", self.index, timestamp);
        
        while !self.hash.clone().unwrap_or_default().starts_with(&target) {
            self.nonce += 1;
            self.hash = Some(Block::calculate_hash(self.index, &self.previous_hash, self.timestamp, &self.transaction, self.nonce));
            debug!(
                "Block {} - Trying nonce {}: {} at {}",
                self.index, self.nonce, self.hash.clone().unwrap_or_default(), timestamp
            );
        }
        
        info!(
            "Block {} mined at {} with hash: {}",
            self.index, Utc::now().to_rfc3339(), self.hash.clone().unwrap_or_default()
        );
    }
}


#[derive(Default, Clone)]
pub struct TokenWallet {
    pub balance: u64,
}

impl TokenWallet {
    pub fn new() -> Self {
        TokenWallet { balance: 1000 }  
    }

    pub fn add_tokens(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn subtract_tokens(&mut self, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            info!("Successfully deducted {} tokens. New balance: {}", amount, self.balance);
            Ok(())
        } else {
            let error_message = format!("Insufficient funds: tried to deduct {} but only {} tokens available.", amount, self.balance);
            error!("{}", error_message);
            Err(error_message)
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}

pub trait Consensus {
    fn validate(&self, block: &Block) -> bool;
}

pub struct PoW;
pub struct PoS;
pub struct DPoS;
pub struct Tendermint;
pub struct PoSpace;

impl Consensus for PoW {
    fn validate(&self, block: &Block) -> bool {
        debug!("Validating block {} with Proof of Work...", block.index);
        let target = "0000";
        let is_valid = block.hash.clone().unwrap_or_default().starts_with(target);
        if is_valid {
            info!("Block {} validated with PoW.", block.index);
        } else {
            warn!("Block {} failed PoW validation.", block.index);
        }
        is_valid
    }
}

impl Consensus for PoS {
    fn validate(&self, block: &Block) -> bool {
        debug!("Validating block {} with Proof of Stake...", block.index);
        let is_valid = block.index % 2 == 0;
        if is_valid {
            info!("Block {} validated with PoS.", block.index);
        } else {
            warn!("Block {} failed PoS validation.", block.index);
        }
        is_valid
    }
}

impl Consensus for DPoS {
    fn validate(&self, block: &Block) -> bool {
        debug!("Validating block {} with Delegated Proof of Stake...", block.index);
        let is_valid = block.index % 3 == 0;
        if is_valid {
            info!("Block {} validated with DPoS.", block.index);
        } else {
            warn!("Block {} failed DPoS validation.", block.index);
        }
        is_valid
    }
}

impl Consensus for Tendermint {
    fn validate(&self, block: &Block) -> bool {
        debug!("Validating block {} with Tendermint...", block.index);
        let is_valid = block.index % 4 == 0;
        if is_valid {
            info!("Block {} validated with Tendermint.", block.index);
        } else {
            warn!("Block {} failed Tendermint validation.", block.index);
        }
        is_valid
    }
}

impl Consensus for PoSpace {
    fn validate(&self, block: &Block) -> bool {
        debug!("Validating block {} with Proof of Space...", block.index);
        let is_valid = block.index % 5 == 0;
        if is_valid {
            info!("Block {} validated with PoSpace.", block.index);
        } else {
            warn!("Block {} failed PoSpace validation.", block.index);
        }
        is_valid
    }
}


pub enum TransactionType {
    Large,
    Small,
    Delegation,
    Fast,
    Storage,
}

pub fn validate_transaction(transaction_type: TransactionType, block: &Block) -> bool {
    let validation_result = match transaction_type {
        TransactionType::Large => {
            let pow = PoW;
            pow.validate(block)
        }
        TransactionType::Small => {
            let pos = PoS;
            pos.validate(block)
        }
        TransactionType::Delegation => {
            let dpos = DPoS;
            dpos.validate(block)
        }
        TransactionType::Fast => {
            let tendermint = Tendermint;
            tendermint.validate(block)
        }
        TransactionType::Storage => {
            let pospace = PoSpace;
            pospace.validate(block)
        }
    };

    let timestamp = Utc::now().to_rfc3339();
    if validation_result {
        info!("Block {} validated successfully at {} using {}", block.index, timestamp, transaction_type_to_string(transaction_type));
    } else {
        warn!("Block {} failed validation at {} using {}", block.index, timestamp, transaction_type_to_string(transaction_type));
    }

    validation_result
}

fn transaction_type_to_string(transaction_type: TransactionType) -> &'static str {
    match transaction_type {
        TransactionType::Large => "Large",
        TransactionType::Small => "Small",
        TransactionType::Delegation => "Delegation",
        TransactionType::Fast => "Fast",
        TransactionType::Storage => "Storage",
    }
}


fn main() {
    env_logger::init();

    let mut rng = OsRng;
    let keypair = Keypair::generate(&mut rng);
    info!("Blockchain simulation with TRIAD started...");

    let mut miner_wallet = TokenWallet::new();
    let mut user_wallet = TokenWallet::new();

    let mut i = 1; 
    loop {
        let previous_hash = if i == 1 { String::from("0") } else { format!("{:x}", i - 1) };
        let timestamp = 1675303065 + (i * 1000);
        let transaction = format!("Transaction {}", i);
        let consensus_algorithm = match i {
            1 => "PoW".to_string(),
            2 => "PoS".to_string(),
            3 => "DPoS".to_string(),
            4 => "Tendermint".to_string(),
            5 => "PoSpace".to_string(),
            _ => "Unknown".to_string(),
        };

        let mut block = Block::new(i, previous_hash, timestamp, transaction, consensus_algorithm);
        block.sign(&keypair);

        
        let difficulty = 4 + (i / 2);  
        block.mine(difficulty);

        
        miner_wallet.add_tokens(block.miner_reward);
        info!("Miner receives {} TRD for mining block {}", block.miner_reward, block.index);

      
        user_wallet.subtract_tokens(block.transaction_fee).unwrap_or_else(|e| {
            warn!("Transaction failed: {}", e);
        });
        info!("Transaction fee of {} TRD paid", block.transaction_fee);

       
        info!("Consensus validation: {}", validate_transaction(TransactionType::Large, &block));

       
        if block.verify_signature(&keypair.public) {
            info!("Block {} validated with {} and signature is valid", block.index, block.consensus_algorithm);
        } else {
            warn!("Block {} failed validation or signature is invalid", block.index);
        }

       
        i += 1;

        
        if i > 20 {  
            break;
        }
    }

    
    info!("Miner's balance: {} TRD", miner_wallet.get_balance());
    info!("User's balance: {} TRD", user_wallet.get_balance());

    info!("Blockchain Simulation with TRIAD Finished...");
}
