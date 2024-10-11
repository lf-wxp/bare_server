use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

use super::{LinkRole, LinkRoleFilter};

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Hairdo {
  pub role: String,
  name: String,
  value: String,
  image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Hairdo {}

impl LinkRole for Hairdo {
  fn role(&self) -> String {
    self.role.clone()
  }
}

impl LinkRoleFilter<Hairdo> for Vec<Hairdo> {}
