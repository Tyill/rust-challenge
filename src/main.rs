mod model;
mod generator;
mod pipeline;
mod storage;
mod test;

use generator::DefaultTransferGenerator;
use std::fs;
use serde_json;
use anyhow::Result;

use crate::generator::TransferGenerator;
use crate::storage::{Storage, StorageConfig};
use crate::pipeline::calculate_user_stats;
use clap::Parser;

/// Program for generate user transfer and calc stats
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of generate transfer
    #[arg(short, long, default_value_t = 10)]
    gencount: usize,

    /// Number of show stats
    #[arg(short, long, default_value_t = 10)]
    showcount: usize,

    /// Begin of time interval for stats
    #[arg(short, long, default_value_t = 1)]
    beginsec: u64,

    /// End of time interval for stats
    #[arg(short, long, default_value_t = 1000)]
    endsec: u64,

    /// Path of config file
    #[arg(short, long, default_value = "rust_challenge.json")]
    cngpath: String,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();

    let store_cng : StorageConfig;
    match read_config(&args.cngpath){
        Ok(cng) => store_cng = cng,
        Err(err) => panic!("Error read_config {}", err),
    }    
    
    let transfers = DefaultTransferGenerator::default().generate(args.gencount);

    let store = Storage::new(store_cng);
    if let Err(err) = store.clear().await{
        panic!("Error store.clear {}", err)
    }
    if let Err(err) = store.load_transfers(transfers).await{
        panic!("Error store.load_transfers {}", err)
    }
    match calculate_user_stats(&store, (args.beginsec, args.endsec)).await{
        Ok(stats) => {
            println!("stats len {}", stats.len());

            for stat in stats.iter().take(args.showcount) {
                println!("{:?}", stat);
            }
        },
        Err(err) =>{
            panic!("Error calculate_user_stats {}", err);
        }
    }    
}

fn read_config(cng_path: &str)->Result<StorageConfig>{
    let str = fs::read_to_string(cng_path)?;
    let cng = serde_json::from_str::<StorageConfig>(&str)?;
    Ok(cng)
}

