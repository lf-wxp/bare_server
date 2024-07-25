use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IdleTransition {
  role: String,
  value: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Idle {
  role: String,
  name: String,
  value: String,
  color: String,
  static_image: String,
  motion_image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdleMapping {
  role: String,
  start: String, 
  end: String,
  transition: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

