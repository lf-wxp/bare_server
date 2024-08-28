use rocket::{Catcher, Request};

#[catch(460)]
fn data_field_missing(req: &Request<'_>) -> String {
  let data = req.local_cache(|| "".to_string());
  format!("{data}")
}

#[catch(default)]
fn default(_: &Request<'_>) -> String {
  "internal error".to_string()
}

pub fn catcher() -> Vec<Catcher> {
  catchers![data_field_missing, default]
}
