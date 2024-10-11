use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  batch_params::BatchUpdateItem,
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

#[patch("/idle_mapping_batch", format = "json", data = "<batch_filter>")]
pub async fn batch_delete(
  _auth: guard::Auth,
  batch_filter: Json<Vec<HashMap<String, String>>>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  idle_mappings.batch_delete(batch_filter.into()).await
}

#[put("/idle_mapping_batch", format = "json", data = "<batch_update>")]
pub async fn batch_update(
  _auth: guard::Auth,
  batch_update: Json<Vec<BatchUpdateItem<IdleMapping>>>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  idle_mappings.batch_update(batch_update.into()).await
}

#[post("/idle_mapping_batch", format = "json", data = "<batch_insert>")]
pub async fn batch_insert(
  _auth: guard::Auth,
  batch_insert: guard::CustomJson<Vec<IdleMapping>>,
) -> DocumentActionResponder<IdleMapping> {
  let idle_mappings = IdleMappings::new();
  idle_mappings.batch_insert(batch_insert.into()).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![
    get_item,
    get_list,
    add_item,
    update_item,
    delete_item,
    batch_delete,
    batch_update,
    batch_insert
  ]
}
