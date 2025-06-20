mod model;
mod generator;
mod pipeline;
mod storage;

use generator::DefaultTransferGenerator;
use std::env;
use std::fs;
use serde_json;
use anyhow::Result;

use crate::generator::TransferGenerator;
use crate::storage::{Storage, StorageConfig};
use crate::pipeline::calculate_user_stats;
use crate::model::Transfer;

#[tokio::main]
async fn main() {

    let store_cng : StorageConfig;
    match read_config(){
        Ok(cng) => store_cng = cng,
        Err(err) => panic!("Error read_config {}", err),
    }    
    
    let transfers = DefaultTransferGenerator::default().generate(10);

    let store = Storage::new(store_cng);
    if let Err(err) = store.load_transfers(transfers).await{
        panic!("Error load_transfers {}", err)
    }
    match calculate_user_stats(&store, (1,5)).await{
        Ok(stats) => {
            for stat in stats.iter().take(10) {
                println!("{:?}", stat);
            }
        },
        Err(err) =>{
            panic!("Error calculate_user_stats {}", err);
        }
    }    
}

fn read_config()->Result<StorageConfig>{
    let mut cng_path: String = "rust_challenge.json".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        cng_path = args[1].clone();
    }
    let str = fs::read_to_string(cng_path.clone())?;
    let cng = serde_json::from_str::<StorageConfig>(&str)?;
    Ok(cng)
}


// let transfers = vec![
//     Transfer {
//         ts: 1,
//         from: 1,
//         to: 2,
//         amount: 1.0,
//         usd_price: 10.0},
//     Transfer {
//         ts: 2,
//         from: 2,
//         to: 3,
//         amount: 2.0,
//         usd_price: 5.0},
//     Transfer {
//         ts: 3,
//         from: 3,
//         to: 1,
//         amount: 3.0,
//         usd_price: 4.0},
//     Transfer {
//         ts: 4,
//         from: 2,
//         to: 4,
//         amount: 2.0,
//         usd_price: 3.0},
//     Transfer {
//         ts: 5,
//         from: 3,
//         to: 4,
//         amount: 1.0,
//         usd_price: 5.0},
//     ];
