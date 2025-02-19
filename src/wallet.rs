use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use ed25519_dalek::PublicKey;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TokenWallet {
    pub address: String,
    pub balances: HashMap<String, u64>,
}

impl TokenWallet {
    pub fn new(address: String) -> Self {
        let mut balances = HashMap::new();
        balances.insert("TRD".into(), 1000);
        TokenWallet { address, balances }
    }
    
    pub fn add_tokens(&mut self, asset: &str, amount: u64) {
        *self.balances.entry(asset.into()).or_insert(0) += amount;
    }
    
    pub fn subtract_tokens(&mut self, asset: &str, amount: u64) -> Result<(), String> {
        let bal = self.balances.entry(asset.into()).or_insert(0);
        if *bal >= amount { *bal -= amount; Ok(()) } else { Err(format!("Insufficient funds: {} available, {} requested", *bal, amount)) }
    }
    
    pub fn transfer_tokens(&mut self, recipient: &mut TokenWallet, asset: &str, amount: u64) -> Result<(), String> {
        self.subtract_tokens(asset, amount)?;
        recipient.add_tokens(asset, amount);
        Ok(())
    }
    
    pub fn get_balance(&self, asset: &str) -> u64 {
        *self.balances.get(asset).unwrap_or(&0)
    }
}

pub fn generate_address(public_key: &PublicKey) -> String {
    let mut hasher = Sha256::new();
    hasher.update(public_key.as_bytes());
    format!("{:x}", hasher.finalize())
}
