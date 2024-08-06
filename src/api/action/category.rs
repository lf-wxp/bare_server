use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{ActionCategories, CollectionOperations},
  document::ActionCategory,
  guard,
  responder::DocumentActionResponder,
};

#[get("/action_category?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<ActionCategory> {
  let action_categories = ActionCategories::new();
  action_categories.list(&filter).await
}
#[get("/action_category/<category_name>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  category_name: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<ActionCategory> {
  let action_categories = ActionCategories::new();
  action_categories
    .find_one(doc! { "name": category_name }, &filter)
    .await
}

#[post("/action_category", format = "json", data = "<category>")]
pub async fn add_item(
  _auth: guard::Auth,
  category: guard::CustomJson<ActionCategory>,
) -> DocumentActionResponder<ActionCategory> {
  let action_categories = ActionCategories::new();
  let mut category = (*category).clone();
  action_categories.insert(&mut category).await
}

#[put(
  "/action_category/<category_name>?<filter..>",
  format = "json",
  data = "<category>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  category_name: &str,
  filter: HashMap<&str, &str>,
  category: Json<ActionCategory>,
) -> DocumentActionResponder<ActionCategory> {
  let action_categories = ActionCategories::new();
  let category = (*category).clone();
  action_categories
    .update(doc! { "name": category_name}, &filter, category)
    .await
}

#[delete("/action_category/<category_name>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  category_name: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<ActionCategory> {
  let action_categories = ActionCategories::new();
  action_categories
    .delete(doc! { "name": category_name }, &filter)
    .await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
