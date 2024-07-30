use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Materials, CollectionOperations},
  document::Material,
  guard,
  responder::DocumentActionResponder,
};

#[get("/material?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  materials.list(&filter).await
}
#[get("/material/<material>")]
pub async fn get_item(_auth: guard::Auth, material: &str) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  materials.find_one(doc! { "value": material }).await
}

#[post("/material", format = "json", data = "<material>")]
pub async fn add_item(
  _auth: guard::Auth,
  material: guard::CustomJson<Material>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  let mut material = (*material).clone();
  materials.insert(&mut material).await
}

#[put("/material/<material_id>", format = "json", data = "<material>")]
pub async fn update_item(
  _auth: guard::Auth,
  material_id: &str,
  material: Json<Material>,
) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  let material = (*material).clone();
  materials.update(doc! { "value": material_id }, material).await
}

#[delete("/material/<material>")]
pub async fn delete_item(_auth: guard::Auth, material: &str) -> DocumentActionResponder<Material> {
  let materials = Materials::new();
  materials.delete(doc! { "value": material }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
