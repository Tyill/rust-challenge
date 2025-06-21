mod model;
mod generator;
mod pipeline;
mod storage;

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
    #[arg(short, long, default_value_t = 100)]
    endsec: u64,

    /// Path of config file
    #[arg(short, long, default_value = "rust_challenge.json")]
    cngpath: String,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();

    let store_cng : StorageConfig;
    match read_config(args.cngpath){
        Ok(cng) => store_cng = cng,
        Err(err) => panic!("Error read_config {}", err),
    }    
    
    let transfers = DefaultTransferGenerator::default().generate(args.gencount);

    let store = Storage::new(store_cng);
    if let Err(err) = store.load_transfers(transfers).await{
        panic!("Error load_transfers {}", err)
    }
    match calculate_user_stats(&store, (args.beginsec, args.endsec)).await{
        Ok(stats) => {
            for stat in stats.iter().take(args.showcount) {
                println!("{:?}", stat);
            }
        },
        Err(err) =>{
            panic!("Error calculate_user_stats {}", err);
        }
    }    
}

fn read_config(cng_path: String)->Result<StorageConfig>{
    let str = fs::read_to_string(cng_path.clone())?;
    let cng = serde_json::from_str::<StorageConfig>(&str)?;
    Ok(cng)
}


#[cfg(test)]
mod tests {

    use crate::model::Transfer;
    use crate::storage::{Storage, StorageConfig};
    use crate::pipeline::calculate_user_stats;
    
    fn get_transfers()->Vec<Transfer>{
        let transfers = vec![
        Transfer {
            ts: 1,
            from: 1,
            to: 2,
            amount: 1.0,
            usd_price: 10.0},
        Transfer {
            ts: 2,
            from: 2,
            to: 3,
            amount: 2.0,
            usd_price: 5.0},
        Transfer {
            ts: 3,
            from: 3,
            to: 1,
            amount: 3.0,
            usd_price: 4.0},
        Transfer {
            ts: 4,
            from: 2,
            to: 4,
            amount: 2.0,
            usd_price: 3.0},
        Transfer {
            ts: 5,
            from: 3,
            to: 4,
            amount: 1.0,
            usd_price: 5.0},
        ];
        return transfers;
    }


    #[tokio::test]
    async fn stats_test() {
                
        let store = Storage::new(StorageConfig::default());
        store.clear().await;

        let trans = get_transfers();
        store.load_transfers(trans).await;

        if let Ok(stats) = calculate_user_stats(&store, (1,10)).await{
            for stat in stats {
                println!("{:?}", stat);
            }
        }

        //assert_eq!(result, 4);
    }
}

