

#[cfg(test)]
mod tests {

    use crate::model::Transfer;
    use crate::storage::{Storage, StorageConfig};
    use crate::pipeline::calculate_user_stats;
    use crate::read_config;
    
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
                
        let store_cng : StorageConfig;
        match read_config("rust_challenge.json"){
            Ok(cng) => store_cng = cng,
            Err(err) => panic!("Error read_config {}", err),
        }  

        let store = Storage::new(store_cng);
        if let Err(err) = store.clear().await{
            panic!("Error store.clear {}", err);
        }

        let trans = get_transfers();
        if let Err(err) = store.load_transfers(trans).await{
            panic!("Error load_transfers {}", err);
        }

        if let Ok(stats) = calculate_user_stats(&store, (1,10)).await{
            for stat in stats {
                match stat._uid {
                    1 => {
                        assert_eq!(stat.total_volume, 4.0);
                        assert_eq!(stat.avg_buy_price, 4.0);
                        assert_eq!(stat.avg_sell_price, 10.0);
                        assert_eq!(stat.max_balance, 10.0);
                    },
                    2 => {
                        assert_eq!(stat.total_volume, 5.0);
                        assert_eq!(stat.avg_buy_price, 10.0);
                        assert_eq!(stat.avg_sell_price, 4.0);
                        assert_eq!(stat.max_balance, 6.0);
                    },
                    3 => {
                        assert_eq!(stat.total_volume, 6.0);
                        assert_eq!(stat.avg_buy_price, 5.0);
                        assert_eq!(stat.avg_sell_price, 4.25);
                        assert_eq!(stat.max_balance, 7.0);
                    },
                    4 => {
                        assert_eq!(stat.total_volume, 3.0);
                        assert_eq!((stat.avg_buy_price - 3.666) < 0.001, true);
                        assert_eq!(stat.avg_sell_price, 0.0);
                        assert_eq!(stat.max_balance, -6.0);
                    },
                    i32::MIN..=0_i32 | 5_i32..=i32::MAX => {},
                }
            }
        }else{
            panic!("Error calculate_user_stats");
        }        
    }
}
