use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MaterialCategory {
  Sticker,
  Logo,
}


#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Material {
  name: String,
  value: String,
  image: String,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}
