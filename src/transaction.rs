use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use ed25519_dalek::{Verifier, Keypair, Signature, Signer};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TxType {
    Transfer,
    ContractCall,
    Stake,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxOutput {
    pub asset: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub tx_type: TxType,
    pub nonce: u64,
    pub sender: String,
    pub outputs: Vec<TxOutput>,
    pub fee: u64,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u64>,
    pub payload: Option<String>,
    pub timestamp: u64,
    pub signatures: Vec<Vec<u8>>,
    pub source_network: Option<String>,
    pub target_network: Option<String>,
    pub id: String,
}

impl Transaction {
    pub fn new(sender: String, outputs: Vec<TxOutput>, fee: u64) -> Self {
        let mut tx = Transaction {
            version: 1,
            tx_type: TxType::Transfer,
            nonce: 0,
            sender,
            outputs,
            fee,
            gas_limit: None,
            gas_price: None,
            payload: None,
            timestamp: Utc::now().timestamp() as u64,
            signatures: Vec::new(),
            source_network: None,
            target_network: None,
            id: String::new(),
        };
        tx.id = tx.calculate_id();
        tx
    }
    
    pub fn new_cross_chain(sender: String, outputs: Vec<TxOutput>, fee: u64, source_network: String, target_network: String) -> Self {
        let mut tx = Transaction {
            version: 1,
            tx_type: TxType::Transfer,
            nonce: 0,
            sender,
            outputs,
            fee,
            gas_limit: None,
            gas_price: None,
            payload: None,
            timestamp: Utc::now().timestamp() as u64,
            signatures: Vec::new(),
            source_network: Some(source_network),
            target_network: Some(target_network),
            id: String::new(),
        };
        tx.id = tx.calculate_id();
        tx
    }
    
    pub fn calculate_id(&self) -> String {
        let mut hasher = Sha256::new();
        let outputs_str = serde_json::to_string(&self.outputs).unwrap_or_default();
        let payload_str = self.payload.clone().unwrap_or_default();
        let data = format!("{}{}{}{}{}{:?}{:?}{}{}", self.version, self.nonce, self.sender, outputs_str, self.fee, self.source_network, self.target_network, self.timestamp, payload_str);
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    pub fn sign(&mut self, keypair: &Keypair) {
        let data = format!("{}{}", self.id, self.timestamp);
        let signature = keypair.sign(data.as_bytes());
        self.signatures.push(signature.to_bytes().to_vec());
    }
    
    pub fn verify(&self, public_key: &ed25519_dalek::PublicKey) -> bool {
        let data = format!("{}{}", self.id, self.timestamp);
        self.signatures.iter().any(|sig_bytes| {
            if let Ok(signature) = Signature::from_bytes(sig_bytes) {
                public_key.verify(data.as_bytes(), &signature).is_ok()
            } else {
                false
            }
        })
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.outputs.iter().map(|o| o.amount).sum::<u64>() + self.fee > 0 {
            if let TxType::ContractCall = self.tx_type {
                if self.gas_limit.is_none() || self.gas_price.is_none() || self.payload.is_none() {
                    return Err("For ContractCall transactions, gas_limit, gas_price and payload must be set".into());
                }
            }
            Ok(())
        } else {
            Err("Invalid transaction: outputs and fee do not match".into())
        }
    }
}

pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    if transactions.is_empty() {
        let mut hasher = Sha256::new();
        hasher.update("");
        return format!("{:x}", hasher.finalize());
    }
    let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.id.clone()).collect();
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            if let Some(last) = hashes.last().cloned() {
                hashes.push(last);
            }
        }
        let mut new_level = Vec::with_capacity(hashes.len() / 2);
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = &hashes[i+1];
            let mut hasher = Sha256::new();
            hasher.update(left);
            hasher.update(right);
            new_level.push(format!("{:x}", hasher.finalize()));
        }
        hashes = new_level;
    }
    hashes[0].clone()
}
#[cfg(test)]
mod tests {
    use super::*;
    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    use ed25519_dalek::{Keypair, Verifier};
    use rand::rngs::OsRng;

    #[test]
    fn test_transaction_id() {
        let tx_output = TxOutput {
            asset: "TRD".to_string(),
            recipient: "recipient".to_string(),
            amount: 50,
        };
        let tx = Transaction::new("sender".to_string(), vec![tx_output], 1);
        assert!(!tx.id.is_empty(), "Transaction ID should be computed");
    }

    #[test]
    fn test_transaction_signature() {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        let tx_output = TxOutput {
            asset: "TRD".to_string(),
            recipient: "recipient".to_string(),
            amount: 50,
        };
        let mut tx = Transaction::new("sender".to_string(), vec![tx_output], 1);
        tx.sign(&keypair);
        assert!(tx.verify(&keypair.public), "Signature should be valid");
    }
}
