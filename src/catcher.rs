use rocket::{serde::json::Json, Catcher, Request};

#[catch(460)]
fn data_field_missing(req: &Request<'_>) -> Json<Option<String>> {
  let data = req.local_cache(|| "".to_string());
  Json(Some(format!("missing {data} field")))
}

#[catch(461)]
fn data_parse_error(req: &Request<'_>) -> Json<Option<String>> {
  Json(Some("parse data error".to_string()))
}

pub fn catcher() -> Vec<Catcher> {
  catchers![data_field_missing, data_parse_error]
}
