use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Actions, CollectionOperations},
  document::{Action, ActionWithCategory},
  guard,
  responder::DocumentActionResponder,
};

#[get("/action_aggregate?<filter..>")]
pub async fn get_aggregate_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<ActionWithCategory> {
  let actions = Actions::new();
  let data = actions.aggregate(&filter).await;
  DocumentActionResponder::FindAll(data)
}

#[get("/action?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.list(&filter).await
}
#[get("/action/<action>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  action: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.find_one(doc! { "value": action }, &filter).await
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

#[put("/action/<action_id>?<filter..>", format = "json", data = "<action>")]
pub async fn update_item(
  _auth: guard::Auth,
  action_id: &str,
  action: Json<Action>,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  let action = (*action).clone();
  actions
    .update(doc! { "value": action_id }, &filter, action)
    .await
}

#[delete("/action/<action>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  action: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.delete(doc! { "value": action }, &filter).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![
    get_item,
    get_list,
    get_aggregate_list,
    add_item,
    update_item,
    delete_item
  ]
}
