use std::collections::HashMap;

use mongodb::bson::doc;

use crate::{
  collection::{CollectionOperations, FavoriteActions},
  document::FavoriteAction,
  guard,
  responder::DocumentActionResponder,
};

#[get("/favorite_action?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<FavoriteAction> {
  let favorite_actions = FavoriteActions::new();
  favorite_actions.list(&filter).await
}

#[post("/favorite_action", format = "json", data = "<favorite_action>")]
pub async fn add_item(
  auth: guard::Auth,
  favorite_action: guard::CustomJson<FavoriteAction>,
) -> DocumentActionResponder<FavoriteAction> {
  let favorite_actions = FavoriteActions::new();
  let user = auth.user;
  let mut favorite_action = (*favorite_action).clone();
  favorite_action.user = user;
  favorite_actions.insert(&mut favorite_action).await
}

#[delete("/favorite_action/<favorite_action>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  favorite_action: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<FavoriteAction> {
  let favorite_actions = FavoriteActions::new();
  favorite_actions
    .delete(doc! { "value": favorite_action }, &filter)
    .await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_list, add_item, delete_item]
}
