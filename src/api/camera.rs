use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Cameras, CollectionOperations},
  document::Camera,
  guard,
  responder::DocumentActionResponder,
};

#[get("/camera?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Camera> {
  let cameras = Cameras::new();
  cameras.list(&filter).await
}
#[get("/camera/<camera>")]
pub async fn get_item(_auth: guard::Auth, camera: &str) -> DocumentActionResponder<Camera> {
  let cameras = Cameras::new();
  cameras.find_one(doc! { "value": camera }).await
}

#[post("/camera", format = "json", data = "<camera>")]
pub async fn add_item(
  _auth: guard::Auth,
  camera: guard::CustomJson<Camera>,
) -> DocumentActionResponder<Camera> {
  let cameras = Cameras::new();
  let mut camera = (*camera).clone();
  cameras.insert(&mut camera).await
}

#[put("/camera/<camera_id>", format = "json", data = "<camera>")]
pub async fn update_item(
  _auth: guard::Auth,
  camera_id: &str,
  camera: Json<Camera>,
) -> DocumentActionResponder<Camera> {
  let cameras = Cameras::new();
  let camera = (*camera).clone();
  cameras.update(doc! { "value": camera_id }, camera).await
}

#[delete("/camera/<camera>")]
pub async fn delete_item(_auth: guard::Auth, camera: &str) -> DocumentActionResponder<Camera> {
  let cameras = Cameras::new();
  cameras.delete(doc! { "value": camera }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
