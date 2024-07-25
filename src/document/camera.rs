use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CameraCategory {
  Position,
  Crane,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
  name: String,
  value_field: String,
  speed_field: String,
  step: f32,
  category: CameraCategory,
  use_weight: bool,
  value_range: [i64;2],
  speed_range: [i64;2],
  mark: Vec<Option<i64>>,
  tip: String,
  image: String,
  default_speed: i64,
  direction_options: Vec<Option<bool>>,
}
