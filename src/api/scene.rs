use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Scenes},
  document::Scene,
  guard,
  responder::DocumentActionResponder,
};

#[get("/scene?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Scene> {
  let scenes = Scenes::new();
  scenes.list(&filter).await
}
#[get("/scene/<scene>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  scene: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Scene> {
  let scenes = Scenes::new();
  scenes.find_one(doc! { "value": scene }, &filter).await
}

#[post("/scene", format = "json", data = "<scene>")]
pub async fn add_item(
  _auth: guard::Auth,
  scene: guard::CustomJson<Scene>,
) -> DocumentActionResponder<Scene> {
  let scenes = Scenes::new();
  let mut scene = (*scene).clone();
  scenes.insert(&mut scene).await
}

#[put("/scene/<scene_id>?<filter..>", format = "json", data = "<scene>")]
pub async fn update_item(
  _auth: guard::Auth,
  scene_id: &str,
  filter: HashMap<&str, &str>,
  scene: Json<Scene>,
) -> DocumentActionResponder<Scene> {
  let scenes = Scenes::new();
  let scene = (*scene).clone();
  scenes
    .update(doc! { "value": scene_id }, &filter, scene)
    .await
}

#[delete("/scene/<scene>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  scene: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Scene> {
  let scenes = Scenes::new();
  scenes.delete(doc! { "value": scene }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
