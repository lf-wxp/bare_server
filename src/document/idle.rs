use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct IdleTransition {
  role: String,
  value: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
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

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct IdleMapping {
  role: String,
  start: String,
  end: String,
  transition: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl LinkRole for Idle {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for IdleMapping {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for IdleTransition {
  fn role(&self) -> String {
    self.role.clone()
  }
}
