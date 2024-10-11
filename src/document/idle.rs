use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct IdleTransition {
  role: String,
  value: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for IdleTransition {}

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

impl GenOptionValue for Idle {}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct IdleMapping {
  role: String,
  start: String,
  end: String,
  transition: String,
  pub value: Option<String>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for IdleMapping {
  fn set_value(&mut self) {
    let Self {
      role,
      start,
      end,
      transition,
      ..
    } = self;
    self.value = Some(format!("{role}_{start}_{end}_{transition}"));
  }
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
