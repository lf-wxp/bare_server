use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, Idles},
  document::Idle,
  guard,
  responder::DocumentActionResponder,
};

#[get("/idle?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Idle> {
  let idles = Idles::new();
  idles.list(&filter).await
}
#[get("/idle/<idle>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  idle: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Idle> {
  let idles = Idles::new();
  idles.find_one(doc! { "value": idle }, &filter).await
}

#[post("/idle", format = "json", data = "<idle>")]
pub async fn add_item(
  _auth: guard::Auth,
  idle: guard::CustomJson<Idle>,
) -> DocumentActionResponder<Idle> {
  let idles = Idles::new();
  let mut idle = (*idle).clone();
  idles.insert(&mut idle).await
}

#[put("/idle/<idle_id>?<filter..>", format = "json", data = "<idle>")]
pub async fn update_item(
  _auth: guard::Auth,
  idle_id: &str,
  filter: HashMap<&str, &str>,
  idle: Json<Idle>,
) -> DocumentActionResponder<Idle> {
  let idles = Idles::new();
  let idle = (*idle).clone();
  idles.update(doc! { "value": idle_id }, &filter, idle).await
}

#[delete("/idle/<idle>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  idle: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Idle> {
  let idles = Idles::new();
  idles.delete(doc! { "value": idle }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
