use anyhow::Error;
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

    fn new_client(self)->clickhouse::Client{
        let client = Client::default()
            .with_url(self.cng.db_url)
           // .with_user(self.cng.db_user_name)
            .with_password(self.cng.db_user_password)
            .with_database(self.cng.db_name);
        return client; 
    }

    pub async fn load_transfers(self, transfers: &[Transfer])->Result<(), Error>{
        let clt = self.new_client();
       
        let mut insert = clt.inserter("tblUserTransfer")?;
        for t in transfers{
            if let Err(err) = insert.write(t){
                println!("{}", err)
            }
        }
        insert.end().await?;
        return Ok(());
    }
}