use sha2::{Sha256, Digest};

pub struct HTLC {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub hashlock: String,
    pub timelock: u64,
    pub redeemed: bool,
}

impl HTLC {
    pub fn new(sender: String, recipient: String, amount: u64, secret: &str, timelock: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(secret);
        let hashlock = format!("{:x}", hasher.finalize());
        HTLC { sender, recipient, amount, hashlock, timelock, redeemed: false }
    }
    
    pub fn redeem(&mut self, preimage: &str, current_time: u64) -> Result<(), String> {
        if current_time > self.timelock { return Err("Timelock expired".into()); }
        let mut hasher = Sha256::new();
        hasher.update(preimage);
        if format!("{:x}", hasher.finalize()) != self.hashlock { return Err("Invalid preimage".into()); }
        if self.redeemed { return Err("Already redeemed".into()); }
        self.redeemed = true;
        Ok(())
    }
    
    pub fn refund(&mut self, current_time: u64) -> Result<(), String> {
        if current_time <= self.timelock { return Err("Timelock not expired".into()); }
        if self.redeemed { return Err("Already redeemed".into()); }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    
    #[test]
    fn test_htlc_usage() {
        let mut htlc = HTLC::new("Alice".into(), "Bob".into(), 100, "secret", 1_000_000);
        // Попытка выкупа с правильным preimage:
        assert!(htlc.redeem("secret", 500_000).is_ok());
        // Если уже выкуплен – следующий вызов должен вернуть ошибку:
        assert!(htlc.redeem("secret", 500_000).is_err());
    }
}
