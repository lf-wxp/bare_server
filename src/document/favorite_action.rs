use serde::{Deserialize, Serialize};

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
