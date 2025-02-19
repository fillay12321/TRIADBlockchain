use log::info;

pub struct TokenEconomy {
    pub total_supply: u64,
    pub inflation_rate: f64,
    pub burn_rate: f64,
}

impl TokenEconomy {
    pub fn new(initial_supply: u64, inflation_rate: f64, burn_rate: f64) -> Self {
        TokenEconomy { total_supply: initial_supply, inflation_rate, burn_rate }
    }
    
    pub fn apply_inflation(&mut self) {
        let additional = (self.total_supply as f64 * self.inflation_rate) as u64;
        self.total_supply += additional;
        info!("Inflation applied: +{}, total: {}", additional, self.total_supply);
    }
    
    pub fn burn_tokens(&mut self, amount: u64) {
        self.total_supply = self.total_supply.saturating_sub(amount);
        info!("Burned {} tokens, total: {}", amount, self.total_supply);
    }
}
