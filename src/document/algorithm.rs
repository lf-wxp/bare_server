use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AlgType {
  AiLab,
  ArKit,
  Mha,  
}


#[derive(Serialize, Deserialize, Debug, Clone, FieldNamesAsSlice)]
pub struct Algorithm {
  role: String,
  value: AlgType,
  options: Vec<Option<String>>,
  create_timestamp: Option<i64>,
  update_timestamp: Option<i64>,
}
