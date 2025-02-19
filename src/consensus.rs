use crate::block::Block;
use log::{info, warn};

/// Базовый трейт для консенсусных алгоритмов.
pub trait Consensus {
    fn validate(&self, block: &Block) -> bool;
}

/// Реализации базовых механизмов консенсуса.
pub struct PoW;
pub struct PoS;
pub struct DPoS;
pub struct Tendermint;
pub struct PoSpace;

impl Consensus for PoW {
    fn validate(&self, block: &Block) -> bool {
        let target = "0000";
        let valid = block.hash.as_deref().unwrap_or("").starts_with(target);
        if valid {
            info!("Block {} valid with PoW", block.index);
        } else {
            warn!("Block {} invalid with PoW", block.index);
        }
        valid
    }
}

impl Consensus for PoS {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 2 == 0;
        if valid {
            info!("Block {} valid with PoS", block.index);
        } else {
            warn!("Block {} invalid with PoS", block.index);
        }
        valid
    }
}

impl Consensus for DPoS {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 3 == 0;
        if valid {
            info!("Block {} valid with DPoS", block.index);
        } else {
            warn!("Block {} invalid with DPoS", block.index);
        }
        valid
    }
}

impl Consensus for Tendermint {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 4 == 0;
        if valid {
            info!("Block {} valid with Tendermint", block.index);
        } else {
            warn!("Block {} invalid with Tendermint", block.index);
        }
        valid
    }
}

impl Consensus for PoSpace {
    fn validate(&self, block: &Block) -> bool {
        let valid = block.index % 5 == 0;
        if valid {
            info!("Block {} valid with PoSpace", block.index);
        } else {
            warn!("Block {} invalid with PoSpace", block.index);
        }
        valid
    }
}

/// --- ПЛАГИН-СИСТЕМА ---
/// Импортируем трейд ConsensusPlugin из модуля consensus_plugin
use crate::consensus_plugin::ConsensusPlugin;

impl ConsensusPlugin for PoW {
    fn name(&self) -> &'static str {
        "PoW"
    }
    fn validate(&self, block: &Block) -> bool {
        <Self as Consensus>::validate(self, block)
    }
}

impl ConsensusPlugin for PoS {
    fn name(&self) -> &'static str {
        "PoS"
    }
    fn validate(&self, block: &Block) -> bool {
        <Self as Consensus>::validate(self, block)
    }
}

impl ConsensusPlugin for DPoS {
    fn name(&self) -> &'static str {
        "DPoS"
    }
    fn validate(&self, block: &Block) -> bool {
        <Self as Consensus>::validate(self, block)
    }
}

impl ConsensusPlugin for Tendermint {
    fn name(&self) -> &'static str {
        "Tendermint"
    }
    fn validate(&self, block: &Block) -> bool {
        <Self as Consensus>::validate(self, block)
    }
}

impl ConsensusPlugin for PoSpace {
    fn name(&self) -> &'static str {
        "PoSpace"
    }
    fn validate(&self, block: &Block) -> bool {
        <Self as Consensus>::validate(self, block)
    }
}
