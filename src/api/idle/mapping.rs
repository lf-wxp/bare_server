use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, IdleMappings},
  document::IdleMapping,
  guard,
  responder::DocumentActionResponder,
  utils::GenOptionValue,
};

#[get("/idle_mapping?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  idle_mappings.list(&filter).await
}
#[get("/idle_mapping/<mapping_value>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  mapping_value: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  idle_mappings
    .find_one(doc! { "value": mapping_value }, &filter)
    .await
}

#[post("/idle_mapping", format = "json", data = "<idle_mapping>")]
pub async fn add_item(
  _auth: guard::Auth,
  idle_mapping: guard::CustomJson<IdleMapping>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  let mut idle_mapping = (*idle_mapping).clone();
  idle_mapping.set_value();
  idle_mappings.insert(&mut idle_mapping).await
}

#[put(
  "/idle_mapping/<mapping_value>?<filter..>",
  format = "json",
  data = "<idle_mapping>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  mapping_value: &str,
  filter: HashMap<&str, &str>,
  idle_mapping: Json<IdleMapping>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  let mut idle_mapping = (*idle_mapping).clone();
  idle_mapping.set_value();
  idle_mappings
    .update(doc! { "value": mapping_value }, &filter, idle_mapping)
    .await
}

#[delete("/idle_mapping/<mapping_value>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  mapping_value: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  idle_mappings
    .delete(doc! { "value": mapping_value }, &filter)
    .await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
