use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  batch_params::BatchUpdateItem,
  collection::{Actions, CollectionOperations},
  document::{Action, ActionWithCategory},
  guard,
  responder::DocumentActionResponder,
};

#[get("/action_aggregate?<filter..>")]
pub async fn get_aggregate_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<ActionWithCategory> {
  let actions = Actions::new();
  actions.aggregate(&filter).await.into()
}

#[get("/action?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
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

#[patch("/action_batch", format = "json", data = "<batch_filter>")]
pub async fn batch_delete(
  _auth: guard::Auth,
  batch_filter: Json<Vec<HashMap<String, String>>>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.batch_delete(batch_filter.into()).await
}

#[put("/action_batch", format = "json", data = "<batch_update>")]
pub async fn batch_update(
  _auth: guard::Auth,
  batch_update: Json<Vec<BatchUpdateItem<Action>>>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.batch_update(batch_update.into()).await
}

#[post("/action_batch", format = "json", data = "<batch_insert>")]
pub async fn batch_insert(
  _auth: guard::Auth,
  batch_insert: guard::CustomJson<Vec<Action>>,
) -> DocumentActionResponder<Action> {
  let actions = Actions::new();
  actions.batch_insert(batch_insert.into()).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![
    get_item,
    get_list,
    get_aggregate_list,
    add_item,
    update_item,
    delete_item,
    batch_insert,
    batch_update,
    batch_delete
  ]
}
