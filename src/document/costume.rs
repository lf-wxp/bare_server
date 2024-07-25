use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CostumeCategory {
  role: String,
  name: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Costume {
  role: String,
  name: String,
  value: String,
  image: String,
  category: String, 
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}
