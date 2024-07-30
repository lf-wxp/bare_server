use serde::{Deserialize, Serialize};

use super::LinkRole;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionCategory {
  role: String,
  name: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
  role: String,
  name: String,
  value: String,
  static_image: String,
  motion_image: String,
  duration: f32,
  category: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  associated_idle: String,
  support_pointer: bool,
  pointer_start: i8,
  pointer_end: i8,
  support_mirror: bool,
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
