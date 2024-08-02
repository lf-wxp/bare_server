use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct FavoriteAction {
  role: String,
  pub user: Option<String>,
  value: String,
}

impl LinkRole for FavoriteAction {
  fn role(&self) -> String {
    self.role.clone()
  }
}
