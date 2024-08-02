use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Scene {
  value: String,
  image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}
