use anyhow::Result;
use clickhouse::Client;
use serde::{Deserialize, Serialize};
use crate::model::Transfer;

#[derive(Clone, Serialize, Deserialize)]
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

    pub async fn load_transfers(&self, transfers: &[Transfer])->Result<()>{
        let clt = self.new_client();       
        let mut insert = clt.inserter("tblUserTransfer")?
                                                .with_max_rows(transfers.len() as u64);
        for t in transfers{
            if let Err(err) = insert.write(t){
                return Err(err.into());
            }
        }
        insert.end().await?;
        return Ok(());
    }

    pub async fn calc_total_volume(&self, time_diap: (u64, u64))->Result<Vec<(String, f64)>>{
        let clt = self.new_client();       
        let mut cursor = clt.query("SELECT from, sum(amount*usd_price) \
                                                              FROM tblUserTransfer \
                                                              WHERE ts BETWEEN ? AND ? \
                                                              GROUP BY from;")
                                                              .bind(time_diap.0)
                                                              .bind(time_diap.1)
                                                              .fetch::<(String, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            res.push(row);
        }
        return Ok(res);
    }

    pub async fn calc_avg_buy_price(&self, time_diap: (u64, u64))->Result<Vec<(String, f64)>>{
        let clt = self.new_client();       

        #[derive(Row, Deserialize)]
        struct MyRow<'a> {
            to: &'a str,
        }

        let mut cursor = clt.query(
            "SELECT to FROM tblUserTransfer WHERE ts BETWEEN 1 AND 10;")
             .fetch::<String>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            println!("{}",row.to);
            //let avg = if row.2 > 0.0 {row.1 / row.2} else {0.0};
            //res.push((row.0, avg));
        }
        return Ok(res);
    }

    pub async fn calc_avg_sell_price(&self, time_diap: (u64, u64))->Result<Vec<(String, f64)>>{
        let clt = self.new_client();       
        let mut cursor = clt.query("SELECT from, sum(amount*usd_price), sum(amount) \
                                                              FROM tblUserTransfer \
                                                              WHERE ts BETWEEN ? AND ? \
                                                              GROUP BY from;")
                                                              .bind(time_diap.0)
                                                              .bind(time_diap.1)
                                                              .fetch::<(String, f64, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            let avg = if row.2 > 0.0 {row.1 / row.2} else {0.0};
            res.push((row.0, avg));
        }
        return Ok(res);
    }

    pub async fn calc_max_balance(&self, time_diap: (u64, u64))->Result<Vec<(String, f64)>>{
        let clt = self.new_client();

        // SELECT
        // item,
        // ts,
        // value,
        // sum(value) OVER (PARTITION BY item ORDER BY ts ASC) AS stock_balance
        // FROM warehouse
        // ORDER BY
        // item ASC,
        // ts ASC;

        let mut cursor = clt.query(
            "SELECT from, to, sum(-amount*usd_price) OVER (PARTITION BY from ORDER BY ts ASC) AS stock_balance \
             FROM tblUserTransfer \
             WHERE ts BETWEEN ? AND ? \
             GROUP BY from;")
             .bind(time_diap.0)
             .bind(time_diap.1)
             .fetch::<(String, f64, f64)>()?;
        let mut res = vec![];
        while let Some(row) = cursor.next().await? { 
            let avg = if row.2 > 0.0 {row.1 / row.2} else {0.0};
            res.push((row.0, avg));
        }
        return Ok(res);
    }
}