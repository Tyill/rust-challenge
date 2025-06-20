use anyhow::Result;
use clickhouse::Client;
use serde::{Deserialize, Serialize};
use crate::model::{Transfer, Balance};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct StorageConfig{
    db_url: String,
    db_name: String,
    db_user_name: String,
    db_user_password: String,
}

pub struct Storage{
    cng: StorageConfig,
}

impl Storage {

    pub fn new(cng: StorageConfig)->Storage{
        return Storage{
            cng
        }        
    }

    fn new_client(&self)->clickhouse::Client{
        let client = Client::default()
            .with_url(self.cng.db_url.clone())
            .with_user(self.cng.db_user_name.clone())
            .with_password(self.cng.db_user_password.clone())
            .with_database(self.cng.db_name.clone());
        return client; 
    }

    pub async fn load_transfers(&self, mut transfers: Vec<Transfer>)->Result<()>{
        let clt = self.new_client();       
        let mut insert_transfer = clt.inserter("tblUserTransfer")?
                                                        .with_max_rows(transfers.len() as u64);
        let mut insert_balance = clt.inserter("tblUserBalance")?
                                                        .with_max_rows(transfers.len() as u64 * 2);
        let mut balances: HashMap<i32, f64> = HashMap::new();
       
        transfers.sort_by(|l, r| l.ts.cmp(&r.ts));
       
        for t in transfers{
            if let Err(err) = insert_transfer.write(&t){
                return Err(err.into());
            }            
            *balances.entry(t.from).or_default() += t.amount * t.usd_price;
            *balances.entry(t.to).or_default() -= t.amount * t.usd_price;

            let to_balance = Balance{uid: t.to, ts: t.ts, balance: *balances.get(&t.to).unwrap()};
            if let Err(err) = insert_balance.write(&to_balance){
                return Err(err.into());
            }
            let from_balance = Balance{uid: t.from, ts: t.ts, balance: *balances.get(&t.from).unwrap()};
            if let Err(err) = insert_balance.write(&from_balance){
                return Err(err.into());
            }
        }
        insert_transfer.end().await?;
        insert_balance.end().await?;
        return Ok(());
    }

    pub async fn calc_total_volume(&self, time_diap: (u64, u64))->Result<Vec<(i32, f64)>>{
        let clt = self.new_client();       
        let mut cursor = clt.query(
            "SELECT to, sum(amount) \
             FROM tblUserTransfer \
             WHERE ts BETWEEN ? AND ? \
             GROUP BY to")
             .bind(time_diap.0)
             .bind(time_diap.1)
             .fetch::<(i32, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            res.push(row);
        }
        let mut cursor = clt.query(
            "SELECT from, sum(amount) \
             FROM tblUserTransfer \
             WHERE ts BETWEEN ? AND ? \
             GROUP BY from")
             .bind(time_diap.0)
             .bind(time_diap.1)
             .fetch::<(i32, f64)>()?;
        while let Some(row) = cursor.next().await? { 
            res.push(row);
        }
        return Ok(res);
    }

    pub async fn calc_avg_buy_price(&self, time_diap: (u64, u64))->Result<Vec<(i32, f64)>>{
        let clt = self.new_client();
        let mut cursor = clt.query(
            "SELECT to, sum(amount*usd_price), sum(amount) \
             FROM tblUserTransfer \
             WHERE ts BETWEEN ? AND ? \
             GROUP BY to")
             .bind(time_diap.0)
             .bind(time_diap.1)
             .fetch::<(i32, f64, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            let avg = if row.2 > 0.0 {row.1 / row.2} else {0.0};
            res.push((row.0, avg));
        }
        return Ok(res);
    }

    pub async fn calc_avg_sell_price(&self, time_diap: (u64, u64))->Result<Vec<(i32, f64)>>{
        let clt = self.new_client();       
        let mut cursor = clt.query(
            "SELECT from, sum(amount*usd_price), sum(amount) \
             FROM tblUserTransfer \
             WHERE ts BETWEEN ? AND ? \
             GROUP BY from")
             .bind(time_diap.0)
             .bind(time_diap.1)
             .fetch::<(i32, f64, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            let avg = if row.2 > 0.0 {row.1 / row.2} else {0.0};
            res.push((row.0, avg));
        }
        return Ok(res);
    }

    pub async fn calc_max_balance(&self, time_diap: (u64, u64))->Result<Vec<(i32, f64)>>{
        let clt = self.new_client();
        let mut cursor = clt.query(
            "SELECT uid, max(balance) \
             FROM tblUserBalance \
             WHERE ts BETWEEN ? AND ? \
             GROUP BY uid")
             .bind(time_diap.0)
             .bind(time_diap.1)
             .fetch::<(i32, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            res.push(row);
        }
        return Ok(res);
    }
}