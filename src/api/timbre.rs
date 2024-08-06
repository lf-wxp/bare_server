use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, Timbres},
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
#[get("/timbre/<timbre>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  timbre: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  timbres.find_one(doc! { "value": timbre }, &filter).await
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

#[put("/timbre/<timbre_id>?<filter..>", format = "json", data = "<timbre>")]
pub async fn update_item(
  _auth: guard::Auth,
  timbre_id: &str,
  filter: HashMap<&str, &str>,
  timbre: Json<Timbre>,
) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  let timbre = (*timbre).clone();
  timbres
    .update(doc! { "value": timbre_id }, &filter, timbre)
    .await
}

#[delete("/timbre/<timbre>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  timbre: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Timbre> {
  let timbres = Timbres::new();
  timbres.delete(doc! { "value": timbre }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
