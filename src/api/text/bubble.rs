use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{Bubbles, CollectionOperations},
  document::Bubble,
  guard,
  responder::DocumentActionResponder,
  utils::GenOptionValue,
};

#[get("/bubble?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  bubbles.list(&filter).await
}
#[get("/bubble/<bubble>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  bubble: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  bubbles.find_one(doc! { "value": bubble }, &filter).await
}

#[post("/bubble", format = "json", data = "<bubble>")]
pub async fn add_item(
  _auth: guard::Auth,
  bubble: guard::CustomJson<Bubble>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  let mut bubble = (*bubble).clone();
  bubble.set_value();
  bubbles.insert(&mut bubble).await
}

#[put("/bubble/<bubble_id>?<filter..>", format = "json", data = "<bubble>")]
pub async fn update_item(
  _auth: guard::Auth,
  bubble_id: &str,
  filter: HashMap<&str, &str>,
  bubble: Json<Bubble>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  let bubble = (*bubble).clone();
  bubbles
    .update(doc! { "value": bubble_id }, &filter, bubble)
    .await
}

#[delete("/bubble/<bubble>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  bubble: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  bubbles.delete(doc! { "value": bubble }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
