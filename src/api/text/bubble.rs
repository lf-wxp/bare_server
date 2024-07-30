use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Bubbles, CollectionOperations},
  document::Bubble,
  guard,
  responder::DocumentActionResponder,
};

#[get("/bubble?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  bubbles.list(&filter).await
}
#[get("/bubble/<bubble>")]
pub async fn get_item(_auth: guard::Auth, bubble: &str) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  bubbles.find_one(doc! { "value": bubble }).await
}

#[post("/bubble", format = "json", data = "<bubble>")]
pub async fn add_item(
  _auth: guard::Auth,
  bubble: guard::CustomJson<Bubble>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  let mut bubble = (*bubble).clone();
  bubbles.insert(&mut bubble).await
}

#[put("/bubble/<bubble_id>", format = "json", data = "<bubble>")]
pub async fn update_item(
  _auth: guard::Auth,
  bubble_id: &str,
  bubble: Json<Bubble>,
) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  let bubble = (*bubble).clone();
  bubbles.update(doc! { "value": bubble_id }, bubble).await
}

#[delete("/bubble/<bubble>")]
pub async fn delete_item(_auth: guard::Auth, bubble: &str) -> DocumentActionResponder<Bubble> {
  let bubbles = Bubbles::new();
  bubbles.delete(doc! { "value": bubble }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
