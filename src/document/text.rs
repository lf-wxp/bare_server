use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::utils::GenOptionValue;

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct WeightOption{
  pub label: String,
  pub value: Option<String>,
  pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
  left_width: i64,
  right_width: i64,
  top_height: i64,
  bottom_height: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ScaleType {
  Proportional,
  Free,
}
#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Bubble {
  name: Option<String>,
  value: Option<String>,
  image: String,
  sprite: Option<Sprite>,
  scale_type: ScaleType,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Bubble {
  fn set_value(&mut self) {
    if self.value.is_none() {
      self.value = Some(nanoid!());
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stroke {
  color: String,
  width: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Background {
  color: String,
  opacity: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shadow {
  color: String,
  opacity: i64,
  blur: i64,
  angle: i64,
  distance: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Text {
  value: Option<String>,
  color: String,
  stroke: Option<Stroke>,
  background: Option<Background>,
  shadow: Option<Shadow>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Text {
  fn set_value(&mut self) {
    if self.value.is_none() {
      self.value = Some(nanoid!());
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Font {
  pub value: Option<String>,
  pub name: String,
  pub weight: String,
  pub url: String,
  provenance: Option<String>,
  expired: i64,
  enabled: bool,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}

impl GenOptionValue for Font {
  fn set_value(&mut self) {
    if self.value.is_none() {
      self.value = Some(nanoid!());
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct FontAggregate {
  pub name: String,
  pub weight_options: Vec<WeightOption>,
}
