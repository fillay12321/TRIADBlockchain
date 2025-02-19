use log::info;

pub trait BlockchainAdapter {
    fn lock_asset(&self, asset: &str, amount: u64, sender: &str) -> Result<(), String>;
    fn release_asset(&self, asset: &str, amount: u64, recipient: &str) -> Result<(), String>;
    fn call_smart_contract(&self, contract_address: &str, method: &str, params: &str) -> Result<String, String>;
}

pub struct DefaultBlockchainAdapter;

impl BlockchainAdapter for DefaultBlockchainAdapter {
    fn lock_asset(&self, asset: &str, amount: u64, sender: &str) -> Result<(), String> {
        info!("Locking {} {} from {}", asset, amount, sender);
        Ok(())
    }
    fn release_asset(&self, asset: &str, amount: u64, recipient: &str) -> Result<(), String> {
        info!("Releasing {} {} to {}", asset, amount, recipient);
        Ok(())
    }
    fn call_smart_contract(&self, contract_address: &str, method: &str, params: &str) -> Result<String, String> {
        info!("Calling contract {} method {} with params {}", contract_address, method, params);
        Ok("Simulated contract call succeeded".into())
    }
}
