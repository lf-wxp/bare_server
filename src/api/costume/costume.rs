use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Costumes},
  document::Costume,
  guard,
  responder::DocumentActionResponder,
};

#[get("/costume?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Costume> {
  let costumes = Costumes::new();
  costumes.list(&filter).await
}
#[get("/costume/<costume>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  costume: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Costume> {
  let costumes = Costumes::new();
  costumes.find_one(doc! { "value": costume }, &filter).await
}

#[post("/costume", format = "json", data = "<costume>")]
pub async fn add_item(
  _auth: guard::Auth,
  costume: guard::CustomJson<Costume>,
) -> DocumentActionResponder<Costume> {
  let costumes = Costumes::new();
  let mut costume = (*costume).clone();
  costumes.insert(&mut costume).await
}

#[put(
  "/costume/<costume_id>?<filter..>",
  format = "json",
  data = "<costume>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  costume_id: &str,
  filter: HashMap<&str, &str>,
  costume: Json<Costume>,
) -> DocumentActionResponder<Costume> {
  let costumes = Costumes::new();
  let costume = (*costume).clone();
  costumes
    .update(doc! { "value": costume_id }, &filter, costume)
    .await
}

#[delete("/costume/<costume>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  costume: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Costume> {
  let costumes = Costumes::new();
  costumes.delete(doc! { "value": costume }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
