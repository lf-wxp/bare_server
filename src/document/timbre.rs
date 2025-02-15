use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

use super::{LinkRole, LinkRoleFilter};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
  Male,
  Female,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Timbre {
  pub role: String,
  name: String,
  value: usize,
  image: Option<String>,
  gender: Gender,
  emotion: String,
  audio: Option<String>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Timbre {}

impl LinkRole for Timbre {
  fn role(&self) -> String {
    self.role.clone()
  }
}

impl LinkRoleFilter<Timbre> for Vec<Timbre> {}
