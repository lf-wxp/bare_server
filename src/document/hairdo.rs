use serde::{Deserialize, Serialize};

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hairdo {
  pub role: String,
  name: String,
  value: String,
  image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl LinkRole for Hairdo {
  fn role(&self) -> String {
    self.role.clone()
  }
}
