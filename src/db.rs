use mongodb::Client;
use once_cell::sync::OnceCell;
use std::error::Error;

use crate::config::get_config;

static MONGO_CLIENT: OnceCell<Client> = OnceCell::new();

pub async fn init_db() -> Result<(), Box<dyn Error>> {
  let client_uri = get_config("MONGO_URI");
  let client = Client::with_uri_str(client_uri).await?;
  MONGO_CLIENT
    .set(client)
    .map_err(|_| "MongoDB client has already been initialized")?;
  Ok(())
}

pub fn get_db() -> mongodb::Database {
  MONGO_CLIENT
    .get()
    .expect("MongoDB client is not initialized")
    .clone()
    .database("aidh-config")
}
