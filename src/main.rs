mod model;
mod generator;
mod pipeline;
mod storage;

use generator::DefaultTransferGenerator;
use pipeline::calculate_user_stats;
use std::env;
use std::fs;
use serde_json;

use crate::generator::TransferGenerator;
use crate::storage::{Storage, StorageConfig};

#[tokio::main]
async fn main() {

    let mut cng_path: String = "rust_challenge.json".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        cng_path = args[1].clone();
    }
    let store_cng : StorageConfig;
    match fs::read_to_string(cng_path.clone()) {
        Ok(s) => {
            store_cng = serde_json::from_str::<StorageConfig>(&s).expect("Error parse config");
        },
        Err(_) => panic!("Error read config {}", cng_path)
    };
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
