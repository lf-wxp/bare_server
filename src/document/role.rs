use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::{algorithm::AlgType, CostumeWithCategory, Hairdo, LinkRole, Location, Options, Timbre};

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Role {
  pub role: String,
  name: String,
  avatar: String,
  brief: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  look_at: Option<Location>,
  alg_support: Option<Vec<AlgType>>,
  idle_expression_support: Option<bool>,
  idle_expression_options: Option<Vec<Options<String>>>,
  idle_action_support: Option<bool>,
  custom_expression_support: Option<bool>,
  action_predict_support: Option<bool>,
  action_generate_support: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct RoleAggregate {
  role: String,
  name: String,
  avatar: String,
  brief: String,
  look_at: Option<Location>,
  alg_support: Option<Vec<AlgType>>,
  custom_expression_support: Option<bool>,
  idle_expression_support: Option<bool>,
  idle_expression_options: Option<Vec<Options<String>>>,
  idle_action_support: Option<bool>,
  action_predict_support: Option<bool>,
  action_generate_support: Option<bool>,
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
      action_predict_support,
      action_generate_support,
      look_at,
      alg_support,
      idle_expression_support,
      idle_expression_options,
      idle_action_support,
      custom_expression_support,
      ..
    } = value;

    Self {
      role,
      name,
      avatar,
      brief,
      look_at,
      action_predict_support,
      action_generate_support,
      alg_support,
      idle_expression_support,
      idle_expression_options,
      idle_action_support,
      custom_expression_support,
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
