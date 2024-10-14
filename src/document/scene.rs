use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

use super::{LinkRole, LinkRoleFilter};

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Scene {
  pub role: String,
  value: String,
  image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Scene {}

impl LinkRole for Scene {
  fn role(&self) -> String {
    self.role.clone()
  }
}

impl LinkRoleFilter<Scene> for Vec<Scene> {}
