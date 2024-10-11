use serde::{Deserialize, Serialize};

pub mod action;
pub mod algorithm;
pub mod camera;
pub mod costume;
pub mod favorite_action;
pub mod hairdo;
pub mod idle;
pub mod material;
pub mod role;
pub mod scene;
pub mod text;
pub mod timbre;

pub use action::*;
pub use algorithm::*;
pub use camera::*;
pub use costume::*;
pub use favorite_action::*;
pub use hairdo::*;
pub use idle::*;
pub use material::*;
pub use role::*;
pub use scene::*;
pub use text::*;
pub use timbre::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
  x: f64,
  y: f64,
  z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Options<T> {
  pub value: T,
  pub label: String,
}

pub trait LinkRole {
  fn role(&self) -> String;
}

pub trait LinkRoleFilter<T: LinkRole>: IntoIterator<Item = T> + Sized {
  fn filter_items(self, role_id: &str) -> Vec<T> {
    self
      .into_iter()
      .filter(|item| item.role() == role_id)
      .collect()
  }
}
