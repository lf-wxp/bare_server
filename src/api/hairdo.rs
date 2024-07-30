use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Hairdos, CollectionOperations},
  document::Hairdo,
  guard,
  responder::DocumentActionResponder,
};

#[get("/hairdo?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Hairdo> {
  let hairdos = Hairdos::new();
  hairdos.list(&filter).await
}
#[get("/hairdo/<hairdo>")]
pub async fn get_item(_auth: guard::Auth, hairdo: &str) -> DocumentActionResponder<Hairdo> {
  let hairdos = Hairdos::new();
  hairdos.find_one(doc! { "value": hairdo }).await
}

#[post("/hairdo", format = "json", data = "<hairdo>")]
pub async fn add_item(
  _auth: guard::Auth,
  hairdo: guard::CustomJson<Hairdo>,
) -> DocumentActionResponder<Hairdo> {
  let hairdos = Hairdos::new();
  let mut hairdo = (*hairdo).clone();
  hairdos.insert(&mut hairdo).await
}

#[put("/hairdo/<hairdo_id>", format = "json", data = "<hairdo>")]
pub async fn update_item(
  _auth: guard::Auth,
  hairdo_id: &str,
  hairdo: Json<Hairdo>,
) -> DocumentActionResponder<Hairdo> {
  let hairdos = Hairdos::new();
  let hairdo = (*hairdo).clone();
  hairdos.update(doc! { "value": hairdo_id }, hairdo).await
}

#[delete("/hairdo/<hairdo>")]
pub async fn delete_item(_auth: guard::Auth, hairdo: &str) -> DocumentActionResponder<Hairdo> {
  let hairdos = Hairdos::new();
  hairdos.delete(doc! { "value": hairdo }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
