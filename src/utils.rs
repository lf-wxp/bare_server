use rocket::Request;
use serde::Serializer;
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
