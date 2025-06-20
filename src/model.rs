use serde::{Deserialize, Serialize};
use clickhouse::Row;

#[derive(Row, Serialize, Deserialize)]
pub struct Transfer {
    pub ts: u64,
    pub from: i32,
    pub to: i32,
    pub amount: f64,
    pub usd_price: f64,
}

#[derive(Row, Serialize, Deserialize)]
pub struct Balance {
    pub ts: u64,
    pub uid: i32,
    pub balance: f64,
}

#[derive(Debug)]
pub struct UserStats {
    pub uid: i32,
    pub total_volume: f64,
    pub avg_buy_price: f64,
    pub avg_sell_price: f64,
    pub max_balance: f64,
}

impl UserStats {
    pub fn new(uid: i32) -> Self {
        Self {
            uid,
            total_volume: 0.0,
            avg_buy_price: 0.0,
            avg_sell_price: 0.0,
            max_balance: 0.0,
        }
    }
}
