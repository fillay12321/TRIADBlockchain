use crate::block::Block;
use log::{info, warn};

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
        let target = "0000";
        let valid = block.hash.as_deref().unwrap_or("").starts_with(target);
        if valid { info!("Block {} valid with PoW", block.index); } else { warn!("Block {} invalid with PoW", block.index); }
        valid
    }
}

impl Consensus for PoS {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 2 == 0;
        if valid { info!("Block {} valid with PoS", block.index); } else { warn!("Block {} invalid with PoS", block.index); }
        valid
    }
}

impl Consensus for DPoS {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 3 == 0;
        if valid { info!("Block {} valid with DPoS", block.index); } else { warn!("Block {} invalid with DPoS", block.index); }
        valid
    }
}

impl Consensus for Tendermint {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 4 == 0;
        if valid { info!("Block {} valid with Tendermint", block.index); } else { warn!("Block {} invalid with Tendermint", block.index); }
        valid
    }
}

impl Consensus for PoSpace {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 5 == 0;
        if valid { info!("Block {} valid with PoSpace", block.index); } else { warn!("Block {} invalid with PoSpace", block.index); }
        valid
    }
}

#[derive(Clone, Debug)]
pub enum ConsensusType {
    PoW,
    PoS,
    DPoS,
    Tendermint,
    PoSpace,
}

pub fn get_consensus(consensus_type: ConsensusType) -> Box<dyn Consensus> {
    match consensus_type {
        ConsensusType::PoW => Box::new(PoW),
        ConsensusType::PoS => Box::new(PoS),
        ConsensusType::DPoS => Box::new(DPoS),
        ConsensusType::Tendermint => Box::new(Tendermint),
        ConsensusType::PoSpace => Box::new(PoSpace),
    }
}

pub fn choose_consensus(block_index: u64) -> String {
    match block_index % 5 {
        0 => "PoW".into(),
        1 => "PoS".into(),
        2 => "DPoS".into(),
        3 => "Tendermint".into(),
        _ => "PoSpace".into(),
    }
}

pub struct HybridConsensus {
    pub mechanisms: Vec<Box<dyn Consensus>>,
}

impl HybridConsensus {
    pub fn new(mechanisms: Vec<Box<dyn Consensus>>) -> Self {
        HybridConsensus { mechanisms }
    }
    
    pub fn validate(&self, block: &Block) -> bool {
        self.mechanisms.iter().all(|c| c.validate(block))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    use crate::block::Block;

    #[test]
    fn test_hybrid_consensus_usage() {
        // Создаем фиктивный блок с hash для прохождения всех проверок.
        let block = Block {
            index: 60,
            previous_hash: "0".into(),
            timestamp: 0,
            transactions: vec![],
            consensus_algorithm: "Hybrid".into(),
            signature: None,
            hash: Some("0000dummyhash".into()),
            nonce: 0,
            miner_reward: 50,
            transaction_fee: 5,
        };
        let hybrid = HybridConsensus::new(vec![
            Box::new(PoW),
            Box::new(PoS),
            Box::new(DPoS),
            Box::new(Tendermint),
            Box::new(PoSpace),
        ]);
        assert!(hybrid.validate(&block));
    }
}
