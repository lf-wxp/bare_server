use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
  left_width: i64,
  right_width: i64,
  top_height: i64,
  bottom_height: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bubble {
  name: String,
  value: String,
  image: String,
  sprite: Sprite,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
  value: String,
  color: String,
  stroke: Option<Stroke>,
  background: Option<Background>,
  shadow: Option<Shadow>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}
