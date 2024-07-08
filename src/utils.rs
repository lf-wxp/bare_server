use rocket::Request;
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
