use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Timbres, CollectionOperations},
  document::Timbre,
  guard,
  responder::DocumentActionResponder,
};

#[get("/timbre?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  timbres.list(&filter).await
}
#[get("/timbre/<timbre>")]
pub async fn get_item(_auth: guard::Auth, timbre: &str) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  timbres.find_one(doc! { "value": timbre }).await
}

#[post("/timbre", format = "json", data = "<timbre>")]
pub async fn add_item(
  _auth: guard::Auth,
  timbre: guard::CustomJson<Timbre>,
) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  let mut timbre = (*timbre).clone();
  timbres.insert(&mut timbre).await
}

#[put("/timbre/<timbre_id>", format = "json", data = "<timbre>")]
pub async fn update_item(
  _auth: guard::Auth,
  timbre_id: &str,
  timbre: Json<Timbre>,
) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  let timbre = (*timbre).clone();
  timbres.update(doc! { "value": timbre_id }, timbre).await
}

#[delete("/timbre/<timbre>")]
pub async fn delete_item(_auth: guard::Auth, timbre: &str) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  timbres.delete(doc! { "value": timbre }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
