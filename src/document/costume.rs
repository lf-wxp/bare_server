use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::serialize_bool_option;

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct CostumeCategory {
  pub role: String,
  pub name: String,
  #[serde(serialize_with = "serialize_bool_option")]
  pub required: Option<bool>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Costume {
  role: String,
  name: String,
  value: String,
  image: String,
  pub category: String,
  #[serde(serialize_with = "serialize_bool_option")]
  pub is_default: Option<bool>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct CostumeWithCategory {
  pub role: String,
  pub category: String,
  pub required: bool,
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
