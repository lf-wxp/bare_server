use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::{algorithm::AlgType, CostumeWithCategory, Hairdo, LinkRole, Location, Timbre};

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Role {
  pub role: String,
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
#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct RoleAggregate {
  role: String,
  name: String,
  avatar: String,
  brief: String,
  support_action_predict: bool,
  support_action_generate: bool,
  look_at: Option<Location>,
  alg_support: Vec<AlgType>,
  idle_weight_support: bool,
  idle_expression_support: bool,
  idle_expression_smile: Option<String>,
  pub timbres: Vec<Timbre>,
  pub hairdos: Vec<Hairdo>,
  pub costumes: Vec<CostumeWithCategory>,
}

impl From<Role> for RoleAggregate {
  fn from(value: Role) -> Self {
    let Role {
      role,
      name,
      avatar,
      brief,
      support_action_predict,
      support_action_generate,
      look_at,
      alg_support,
      idle_weight_support,
      idle_expression_support,
      idle_expression_smile,
      ..
    } = value;

    Self {
      role,
      name,
      avatar,
      brief,
      support_action_predict,
      support_action_generate,
      look_at,
      alg_support,
      idle_weight_support,
      idle_expression_support,
      idle_expression_smile,
      timbres: vec![],
      hairdos: vec![],
      costumes: vec![],
    }
  }
}

impl LinkRole for Role {
  fn role(&self) -> String {
    self.role.clone()
  }
}
impl LinkRole for RoleAggregate {
  fn role(&self) -> String {
    self.role.clone()
  }
}
