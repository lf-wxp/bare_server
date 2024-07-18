use serde::{Deserialize, Serialize};

pub mod action;
pub mod role;

pub use role::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
  x: f64,
  y: f64,
  z: f64,
}
