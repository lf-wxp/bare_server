use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Idles},
  document::Idle,
  guard,
  responder::DocumentActionResponder,
};

#[get("/idle?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
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

#[delete("/idle_batch", format = "json", data = "<batch_filter>")]
pub async fn batch_delete_item(
  _auth: guard::Auth,
  batch_filter: Json<Vec<HashMap<String, String>>>,
) -> DocumentActionResponder<Idle> {
  let idles = Idles::new();
  idles.batch_delete(batch_filter.into()).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
