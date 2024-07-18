use serde::{Deserialize, Serialize};

use super::Location;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
  render_id: String,
  name: String,
  avatar: String,
  brief: String,
  enable_action_predict: bool,
  enable_action_generate: bool,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  look_at: Option<Location>,
}
