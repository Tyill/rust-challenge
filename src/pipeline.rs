use std::collections::HashMap;
use anyhow::Result;

use crate::model::UserStats;
use crate::storage::Storage;

pub async fn calculate_user_stats(store: &Storage, time_diap: (u64, u64)) -> Result<Vec<UserStats>> {
   
    let mut stats: HashMap<i32, UserStats> = HashMap::new();
    
    let avg_buy_price = store.calc_avg_buy_price(time_diap).await?;
    for p in avg_buy_price{
        stats.entry(p.0).or_insert(UserStats::new(p.0)).avg_buy_price = p.1;
    }

    let avg_sell_price = store.calc_avg_sell_price(time_diap).await?;
    for p in avg_sell_price{
        stats.entry(p.0).or_insert(UserStats::new(p.0)).avg_sell_price = p.1;
    }

    let max_balance = store.calc_max_balance(time_diap).await?;
    for p in max_balance{
        stats.entry(p.0).or_insert(UserStats::new(p.0)).max_balance = p.1;
    }

    let total_volume = store.calc_total_volume(time_diap).await?;
    for p in total_volume{
        stats.entry(p.0).or_insert(UserStats::new(p.0)).total_volume += p.1;
    }
      
    Ok(stats.into_values().collect())
}
