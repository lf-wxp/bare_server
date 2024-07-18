use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionType {
  render_id: String,
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
  render_id: String,
  action_id: String,
  name: String,
}
