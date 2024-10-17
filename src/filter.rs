use core::f32;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, str::FromStr};

pub trait Filter {
  fn parse(&self, fields: &[&str]) -> (Document, Document, f32, f32) {
    let _ = fields;
    (doc! {}, doc! {}, 0.0, 0.0)
  }
  fn query(&self, fields: &[&str]) -> Document {
    let _ = fields;
    doc! {}
  }
  fn exact(&self, fields: &[&str]) -> Document {
    let _ = fields;
    doc! {}
  }
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
  fn parse(&self, fields: &[&str]) -> (Document, Document, f32, f32) {
    let mut sort = doc! { "create_timestamp": -1 };
    let mut page = 1f32;
    let mut limit = u32::MAX as f32;

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
          limit = value_str.parse::<f32>().unwrap_or(u32::MAX as f32);
        }
        _ => {}
      }
    }

    if let Some(sort_field) = sort_name {
      sort = doc! { sort_field: sort_order };
    }

    let query = self.query(fields);

    let skip = (page - 1f32) * limit;

    (query, sort, skip, limit)
  }

  fn query(&self, fields: &[&str]) -> Document {
    let mut query = doc! {};
    for (key, value) in self {
      let key_str = key.to_string();
      let value_str = value.to_string();
      if !["sort_name", "sort", "page", "size"].contains(&key_str.as_str()) {
        let operator = match key_str.rfind("_") {
          Some(index) => &key_str[index + 1..],
          None => "equal",
        };
        let field_name = if operator == "equal" {
          &key_str
        } else {
          &key_str[..key_str.len() - operator.len() - 1]
        };

        if fields.contains(&field_name) {
          let condition = match operator {
            "gte" => doc! { "$gte": value_str.parse().unwrap_or(0) },
            "gt" => doc! { "$gt": value_str.parse().unwrap_or(0) },
            "lte" => doc! { "$lte": value_str.parse().unwrap_or(0) },
            "lt" => doc! { "$lt": value_str.parse().unwrap_or(0) },
            "contains" => doc! { "$regex": value_str, "$options": "i" },
            "equal" => match bool::from_str(&value_str) {
              Ok(val) => doc! { "$eq": val },
              Err(_) => doc! { "$eq": value_str },
            },
            _ => doc! {},
          };
          query.insert(field_name, condition);
        }
      }
    }
    query
  }

  fn exact(&self, fields: &[&str]) -> Document {
    let mut query = doc! {};
    for (key, value) in self {
      let key_str = key.to_string();
      let value_str = value.to_string();
      if fields.contains(&key_str.as_str()) {
        query.insert(key_str, value_str);
      }
    }
    query
  }
}

impl<K: ToString, V: ToString> Filter for Vec<HashMap<K, V>> {
  fn query(&self, fields: &[&str]) -> Document {
    let doc = self
      .iter()
      .map(|item| {
        let mut query = doc! {};
        for (key, value) in item {
          let key_str = key.to_string();
          let value_str = value.to_string();
          if fields.contains(&key_str.as_str()) {
            query.insert(key_str, value_str);
          }
        }
        query
      })
      .collect::<Vec<Document>>();
    doc! {
      "$or": doc
    }
  }
}
