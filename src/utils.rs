use mongodb::bson::{doc, Document};
use rocket::Request;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use serde_json::Value;

pub fn is_valid_json(json: &str) -> bool {
  serde_json::from_str::<Value>(json).is_ok()
}

pub fn get_cookies(request: &Request) -> String {
  request
    .headers()
    .get("Cookie")
    .collect::<Vec<&str>>()
    .join("; ")
}

pub fn serialize_bool_option<S>(maybe_bool: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let val = maybe_bool.unwrap_or(false);
  serializer.serialize_bool(val)
}

pub fn serialize_vec_option<S, T>(
  maybe_vec: &Option<Vec<T>>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize,
{
  match maybe_vec {
    Some(vec) => vec.serialize(serializer),
    None => {
      let seq = serializer.serialize_seq(Some(0))?;
      seq.end()
    }
  }
}

pub fn serialize_string_option<S>(
  maybe_string: &Option<String>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let string = maybe_string.as_ref().map_or("", String::as_str);
  serializer.serialize_str(string)
}

pub trait GenOptionValue {
  fn set_value(&mut self) {}
}

pub fn get_compare_doc(operator: &str, value: String) -> Document {
  match value.parse::<i64>() {
    Ok(num) => doc! { operator: num},
    Err(_) => doc! { operator: value },
  }
}

fn recurse_and_modify(value: &mut Value) {
  match value {
    Value::Object(obj) => {
      println!("bbbbbbb");
      for (_, val) in obj.iter_mut() {
        recurse_and_modify(val);
      }
    }
    Value::Array(arr) => {
      for item in arr.iter_mut() {
        recurse_and_modify(item);
      }
    }
    Value::String(s) => {
      println!("deal string {}", s);
      let new_val = s.replace("\\\"", "\"");
      *value = Value::String(new_val);
    }
    _ => {} // 对于非对象和非字符串类型不做处理
  }
}

pub fn escape_value(value: Option<Value>) -> Option<Value> {
  match value {
    Some(mut value) => {
      recurse_and_modify(&mut value);
      Some(value)
    }
    None => None,
  }
}
