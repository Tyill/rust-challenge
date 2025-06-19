mod model;
mod generator;
mod pipeline;
mod storage;

use generator::DefaultTransferGenerator;
use pipeline::calculate_user_stats;
use std::env;
use std::fs;
use serde_json;
use anyhow::Result;

use crate::generator::TransferGenerator;
use crate::storage::{Storage, StorageConfig};

#[tokio::main]
async fn main() {

    let store_cng : StorageConfig;
    match read_config(){
        Ok(cng) => store_cng = cng,
        Err(err) => panic!("{}", err),
    }    
    let store = Storage::new(store_cng);
    
    let transfers = DefaultTransferGenerator::default().generate(10);
    
    if let Err(err) = store.load_transfers(&transfers).await{
        panic!("Error load_transfers {}", err)
    }

    let stats = calculate_user_stats(&transfers);

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
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
