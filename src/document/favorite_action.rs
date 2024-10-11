use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct FavoriteAction {
  role: String,
  pub user: Option<String>,
  value: String,
}

impl GenOptionValue for FavoriteAction {}

impl LinkRole for FavoriteAction {
  fn role(&self) -> String {
    self.role.clone()
  }
}
