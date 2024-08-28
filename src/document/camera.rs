use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use super::Options;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CameraCategory {
  Position,
  Crane,
}

#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Camera {
  name: String,
  value_field: String,
  speed_field: String,
  step: f64,
  category: CameraCategory,
  use_weight: bool,
  value_range: [f64; 2],
  speed_range: [f64; 2],
  mark: Vec<Options<f64>>,
  tip: String,
  desc: String,
  image: String,
  default_speed: f64,
  direction_options: Vec<Options<bool>>,
}
