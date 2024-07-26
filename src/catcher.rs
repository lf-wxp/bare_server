use rocket::{Catcher, Request};

#[catch(460)]
fn data_field_missing(req: &Request<'_>) -> String {
  let data = req.local_cache(|| "".to_string());
  format!("missing {data} field")
}

#[catch(461)]
fn data_parse_error(_: &Request<'_>) -> String {
  "parse data error".to_string()
}

#[catch(default)]
fn default(_: &Request<'_>) -> String {
  "internal error".to_string()
}

pub fn catcher() -> Vec<Catcher> {
  catchers![data_field_missing, data_parse_error, default] 
}
