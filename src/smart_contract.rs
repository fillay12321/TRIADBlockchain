use std::collections::HashMap;

pub trait SmartContract {
    fn init(&mut self, params: &str) -> Result<(), String>;
    fn execute(&mut self, input: &str) -> Result<String, String>;
}

pub struct ContractManager {
    pub contracts: HashMap<String, Box<dyn SmartContract>>,
}

impl ContractManager {
    pub fn new() -> Self {
        ContractManager { contracts: HashMap::new() }
    }
    
    pub fn deploy(&mut self, address: String, contract: Box<dyn SmartContract>) {
        self.contracts.insert(address, contract);
    }
    
    pub fn execute_contract(&mut self, address: &str, input: &str) -> Result<String, String> {
        if let Some(contract) = self.contracts.get_mut(address) {
            contract.execute(input)
        } else {
            Err("Contract not found".into())
        }
    }
}
