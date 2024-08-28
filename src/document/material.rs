use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MaterialCategory {
  Sticker,
  Logo,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Material {
  name: Option<String>,
  value: Option<String>,
  image: String,
  category: MaterialCategory,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Material {
  fn set_value(&mut self) {
    self.value = Some(nanoid!());
  }
}
