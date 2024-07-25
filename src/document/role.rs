use serde::{Deserialize, Serialize};

use super::{algorithm::AlgType, Location};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
  role: String,
  name: String,
  avatar: String,
  brief: String,
  support_action_predict: bool,
  support_action_generate: bool,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  look_at: Option<Location>,
  alg_support: Vec<AlgType>,
  idle_weight_support: bool,
  idle_expression_support: bool,
  idle_expression_smile: Option<String>,
}
