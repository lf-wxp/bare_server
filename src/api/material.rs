use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Materials},
  document::Material,
  guard,
  responder::DocumentActionResponder,
  utils::GenOptionValue,
};

#[get("/material?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  materials.list(&filter).await
}

#[get("/material/<material>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  material: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  materials
    .find_one(doc! { "value": material }, &filter)
    .await
}

#[post("/material", format = "json", data = "<material>")]
pub async fn add_item(
  _auth: guard::Auth,
  material: guard::CustomJson<Material>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  let mut material = (*material).clone();
  material.set_value();
  materials.insert(&mut material).await
}

#[put(
  "/material/<material_id>?<filter..>",
  format = "json",
  data = "<material>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  material_id: &str,
  filter: HashMap<&str, &str>,
  material: Json<Material>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  let material = (*material).clone();
  materials
    .update(doc! { "value": material_id }, &filter, material)
    .await
}

#[delete("/material/<material>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  material: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  materials.delete(doc! { "value": material }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
