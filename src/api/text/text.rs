use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Texts, CollectionOperations},
  document::Text,
  guard,
  responder::DocumentActionResponder,
};

#[get("/text?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  texts.list(&filter).await
}
#[get("/tex/<text>")]
pub async fn get_item(_auth: guard::Auth, text: &str) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  texts.find_one(doc! { "value": text }).await
}

#[post("/tex", format = "json", data = "<text>")]
pub async fn add_item(
  _auth: guard::Auth,
  text: guard::CustomJson<Text>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  let mut text = (*text).clone();
  texts.insert(&mut text).await
}

#[put("/tex/<text_id>", format = "json", data = "<text>")]
pub async fn update_item(
  _auth: guard::Auth,
  text_id: &str,
  text: Json<Text>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  let text = (*text).clone();
  texts.update(doc! { "value": text_id }, text).await
}

#[delete("/tex/<text>")]
pub async fn delete_item(_auth: guard::Auth, text: &str) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  texts.delete(doc! { "value": text }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
