use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Scene {
  pub role: Option<Vec<String>>,
  value: String,
  image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Scene {}
