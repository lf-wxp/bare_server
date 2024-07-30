use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
pub trait Filter {
  fn parse(&self) -> (Document, Document, f32, f32);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
  Gte,
  Gt,
  Lte,
  Lt,
  Contains,
  Equal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReserveKey {
  Sort,
  SortName,
  Page,
  Size,
}

impl<K: ToString, V: ToString> Filter for HashMap<K, V> {
  fn parse(&self) -> (Document, Document, f32, f32) {
    let mut query = doc! {};
    let mut sort = doc! { "create_timestamp": -1 };
    let mut page = 1f32;
    let mut limit = 10f32;

    let mut sort_name = None;
    let mut sort_order = 1;

    for (key, value) in self {
      let key_str = key.to_string();
      let value_str = value.to_string();

      match key_str.as_str() {
        "sort_name" => {
          sort_name = Some(value_str);
        }
        "sort" => {
          sort_order = if value_str == "-1" { -1 } else { 1 };
        }
        "page" => {
          page = value_str.parse::<f32>().unwrap_or(1f32);
        }
        "size" => {
          limit = value_str.parse::<f32>().unwrap_or(10000f32);
        }
        _ => {
          let operator = match key_str.rfind("_") {
            Some(index) => &key_str[index + 1..],
            None => "equal",
          };
          let field_name = if operator == "equal" {
            &key_str
          } else {
            &key_str[..key_str.len() - operator.len() - 1]
          };

          let condition = match operator {
            "gte" => doc! { "$gte": value_str },
            "gt" => doc! { "$gt": value_str },
            "lte" => doc! { "$lte": value_str },
            "lt" => doc! { "$lt": value_str },
            "contains" => doc! { "$regex": value_str, "$options": "i" },
            _ => doc! {},
          };

          query.insert(field_name, condition);
        }
      }
    }

    if let Some(sort_field) = sort_name {
      sort = doc! { sort_field: sort_order };
    }

    let skip = (page - 1f32) * limit;

    (query, sort, skip, limit)
  }
}
