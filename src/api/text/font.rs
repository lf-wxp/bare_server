use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Fonts},
  document::{Font, FontAggregate},
  guard,
  responder::DocumentActionResponder,
  utils::GenOptionValue,
};

#[get("/font_aggregate?<filter..>")]
pub async fn get_aggregate_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<FontAggregate> {
  let fonts = Fonts::new();
  let mut filter = filter;
  fonts.aggregate(&mut filter).await.into()
}

#[get("/font?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Font> {
  let fonts = Fonts::new();
  fonts.list(&filter).await
}
#[get("/font/<font>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  font: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Font> {
  let fonts = Fonts::new();
  fonts.find_one(doc! { "value": font }, &filter).await
}

#[post("/font", format = "json", data = "<font>")]
pub async fn add_item(
  _auth: guard::Auth,
  font: guard::CustomJson<Font>,
) -> DocumentActionResponder<Font> {
  let fonts = Fonts::new();
  let mut font = (*font).clone();
  font.set_value();
  fonts.insert(&mut font).await
}

#[put("/font/<font_id>?<filter..>", format = "json", data = "<font>")]
pub async fn update_item(
  _auth: guard::Auth,
  font_id: &str,
  filter: HashMap<&str, &str>,
  font: Json<Font>,
) -> DocumentActionResponder<Font> {
  let fonts = Fonts::new();
  let font = (*font).clone();
  fonts.update(doc! { "value": font_id }, &filter, font).await
}

#[delete("/font/<font>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  font: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Font> {
  let fonts = Fonts::new();
  fonts.delete(doc! { "value": font }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![
    get_item,
    get_list,
    add_item,
    update_item,
    delete_item,
    get_aggregate_list
  ]
}
