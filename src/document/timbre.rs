use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
  Male,
  Female,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timbre {
  role: String,
  name: String,
  value: String,
  image: String,
  gender: Gender,
  emotion: String,
  audio: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}
