use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::LinkRole;
use crate::utils::{serialize_bool_option, serialize_string_option};

#[derive(Serialize, Deserialize, Debug, FieldNamesAsSlice, Clone)]
pub struct ActionCategory {
  pub role: String,
  pub name: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Action {
  role: String,
  name: String,
  value: String,
  static_image: String,
  motion_image: String,
  duration: f64,
  #[serde(serialize_with = "serialize_string_option")]
  pub category: Option<String>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  #[serde(serialize_with = "serialize_string_option")]
  associated_idle: Option<String>,
  #[serde(serialize_with = "serialize_bool_option")]
  support_pointer: Option<bool>,
  pointer_start: Option<i8>,
  pointer_end: Option<i8>,
  pointer_key: Option<i8>,
  #[serde(serialize_with = "serialize_bool_option")]
  support_mirror: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  support_mouth_animation: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct ActionWithCategory {
  pub role: String,
  pub category: String,
  pub action: Vec<Action>,
}

impl LinkRole for ActionCategory {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for Action {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for ActionWithCategory {
  fn role(&self) -> String {
    self.role.clone()
  }
}
