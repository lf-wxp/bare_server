use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, FieldNamesAsSlice)]
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
  duration: f32,
  pub category: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  associated_idle: String,
  support_pointer: bool,
  pointer_start: i8,
  pointer_end: i8,
  support_mirror: bool,
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


