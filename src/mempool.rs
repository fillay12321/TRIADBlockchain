use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Mempool {
    pub transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }
    
    pub fn take_all(&mut self) -> Vec<Transaction> {
        let txs = self.transactions.clone();
        self.transactions.clear();
        txs
    }
}
