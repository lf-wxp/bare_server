use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::{algorithm::AlgType, CostumeWithCategory, Hairdo, LinkRole, Options, Timbre};
use crate::utils::{serialize_bool_option, serialize_string_option, serialize_vec_option};

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Role {
  pub role: String,
  name: String,
  avatar: String,
  #[serde(serialize_with = "serialize_string_option")]
  brief: Option<String>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
  #[serde(serialize_with = "serialize_vec_option")]
  alg_support: Option<Vec<AlgType>>,
  #[serde(serialize_with = "serialize_bool_option")]
  idle_expression_support: Option<bool>,
  #[serde(serialize_with = "serialize_vec_option")]
  idle_expression_options: Option<Vec<Options<String>>>,
  #[serde(serialize_with = "serialize_bool_option")]
  idle_action_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  expression_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  look_at_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  custom_expression_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  action_predict_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  action_generate_support: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct RoleAggregate {
  role: String,
  name: String,
  avatar: String,
  #[serde(serialize_with = "serialize_string_option")]
  brief: Option<String>,
  #[serde(serialize_with = "serialize_vec_option")]
  alg_support: Option<Vec<AlgType>>,
  #[serde(serialize_with = "serialize_bool_option")]
  custom_expression_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  idle_expression_support: Option<bool>,
  #[serde(serialize_with = "serialize_vec_option")]
  idle_expression_options: Option<Vec<Options<String>>>,
  #[serde(serialize_with = "serialize_bool_option")]
  idle_action_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  action_predict_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  action_generate_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  look_at_support: Option<bool>,
  #[serde(serialize_with = "serialize_bool_option")]
  expression_support: Option<bool>,
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
      alg_support,
      idle_expression_support,
      idle_expression_options,
      idle_action_support,
      expression_support,
      look_at_support,
      custom_expression_support,
      ..
    } = value;

    Self {
      role,
      name,
      avatar,
      brief,
      action_predict_support,
      action_generate_support,
      alg_support,
      expression_support,
      idle_expression_support,
      idle_expression_options,
      idle_action_support,
      custom_expression_support,
      look_at_support,
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
