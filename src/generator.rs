use crate::model::Transfer;
use rand::Rng;

pub trait TransferGenerator {
    fn generate(&self, count: usize) -> Vec<Transfer>;
}

#[derive(Debug, Clone)]
pub struct TransferGenConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
    pub min_uid: i32,
    pub max_uid: i32,
}

impl Default for TransferGenConfig {
    fn default() -> Self {
        Self {
            min_amount: 1.0,
            max_amount: 1000.0,
            min_price: 0.1,
            max_price: 2.0,
            max_age_secs: 1000,
            min_uid: 1,
            max_uid: 10,
        }
    }
}

pub struct DefaultTransferGenerator {
    pub config: TransferGenConfig,
}

impl Default for DefaultTransferGenerator {
    fn default() -> Self {
        Self {
            config: TransferGenConfig::default(),
        }
    }
}

impl TransferGenerator for DefaultTransferGenerator {
    fn generate(&self, count: usize) -> Vec<Transfer> {
        let mut rng = rand::thread_rng();
        
        (0..count)
            .map(|_| {
                let from = rng.gen_range(self.config.min_uid..self.config.max_uid);
                let mut to = rng.gen_range(self.config.min_uid..self.config.max_uid);
                while to == from {
                    to = rng.gen_range(self.config.min_uid..self.config.max_uid);
                }
                let amount = rng.gen_range(self.config.min_amount..self.config.max_amount);
                let usd_price = rng.gen_range(self.config.min_price..self.config.max_price);
                let ts = rng.gen_range(1..self.config.max_age_secs);

                Transfer {
                    ts,
                    from,
                    to,
                    amount,
                    usd_price,
                }
            })
            .collect()
    }
}
