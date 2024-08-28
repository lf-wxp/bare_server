use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Texts},
  document::Text,
  guard,
  responder::DocumentActionResponder,
  utils::GenOptionValue,
};

#[get("/text?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  texts.list(&filter).await
}
#[get("/text/<text>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  text: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  texts.find_one(doc! { "value": text }, &filter).await
}

#[post("/text", format = "json", data = "<text>")]
pub async fn add_item(
  _auth: guard::Auth,
  text: guard::CustomJson<Text>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  let mut text = (*text).clone();
  text.set_value();
  texts.insert(&mut text).await
}

#[put("/text/<text_id>?<filter..>", format = "json", data = "<text>")]
pub async fn update_item(
  _auth: guard::Auth,
  text_id: &str,
  filter: HashMap<&str, &str>,
  text: Json<Text>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  let text = (*text).clone();
  texts.update(doc! { "value": text_id }, &filter, text).await
}

#[delete("/text/<text>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  text: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Text> {
  let texts = Texts::new();
  texts.delete(doc! { "value": text }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
