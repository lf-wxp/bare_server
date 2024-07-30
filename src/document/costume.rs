use serde::{Deserialize, Serialize};

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostumeCategory {
  pub role: String,
  pub name: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Costume {
  role: String,
  name: String,
  value: String,
  image: String,
  pub category: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostumeWithCategory {
  pub role: String,
  pub category: String,
  pub costume: Vec<Costume>,
}

impl LinkRole for Costume {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for CostumeCategory {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for CostumeWithCategory {
  fn role(&self) -> String {
    self.role.clone()
  }
}
