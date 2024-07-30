use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Actions, CollectionOperations},
  document::Action,
  guard,
  responder::DocumentActionResponder,
};

#[get("/action?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.list(&filter).await
}
#[get("/action/<action>")]
pub async fn get_item(_auth: guard::Auth, action: &str) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.find_one(doc! { "value": action }).await
}

#[post("/action", format = "json", data = "<action>")]
pub async fn add_item(
  _auth: guard::Auth,
  action: guard::CustomJson<Action>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  let mut action = (*action).clone();
  actions.insert(&mut action).await
}

#[put("/action/<action_id>", format = "json", data = "<action>")]
pub async fn update_item(
  _auth: guard::Auth,
  action_id: &str,
  action: Json<Action>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  let action = (*action).clone();
  actions.update(doc! { "value": action_id }, action).await
}

#[delete("/action/<action>")]
pub async fn delete_item(_auth: guard::Auth, action: &str) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.delete(doc! { "value": action }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
