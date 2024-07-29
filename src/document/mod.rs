use rocket_validation::{Validate, Validated};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::ValidationError;

pub mod action;
pub mod algorithm;
pub mod camera;
pub mod costume;
pub mod hairdo;
pub mod idle;
pub mod material;
pub mod role;
pub mod timbre;
pub mod scene;

pub use action::*;
pub use algorithm::*;
pub use camera::*;
pub use costume::*;
pub use hairdo::*;
pub use idle::*;
pub use material::*;
pub use role::*;
pub use timbre::*;
pub use scene::*;

fn validate_required(value: &Value) -> Result<(), ValidationError> {
  match value {
    Value::Null => Err(ValidationError::new("required")),
    Value::Number(n) if n.is_f64() && n.as_f64().unwrap() == 0.0 => {
      Err(ValidationError::new("required"))
    }
    Value::String(s) if s.is_empty() => Err(ValidationError::new("required")),
    _ => Ok(()),
  }
}

#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Location {
  x: f64,
  y: f64,
  z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Option<T> {
  value: T,
  label: String,
}
