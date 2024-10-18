use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, CostumeCategories},
  document::CostumeCategory,
  guard,
  responder::DocumentActionResponder, utils::GenOptionValue,
};

#[get("/costume_category?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  costume_categories.list(&filter).await
}
#[get("/costume_category/<category_value>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  category_value: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  costume_categories
    .find_one(doc! { "value": category_value }, &filter)
    .await
}

#[post("/costume_category", format = "json", data = "<category>")]
pub async fn add_item(
  _auth: guard::Auth,
  category: guard::CustomJson<CostumeCategory>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  let mut category = (*category).clone();
  category.set_value();
  costume_categories.insert(&mut category).await
}

#[put(
  "/costume_category/<category_value>?<filter..>",
  format = "json",
  data = "<category>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  category_value: &str,
  category: Json<CostumeCategory>,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  let category = (*category).clone();
  costume_categories
    .update(doc! { "value": category_value}, &filter, category)
    .await
}

#[delete("/costume_category/<category_value>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  category_value: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  costume_categories
    .delete(doc! { "value": category_value }, &filter)
    .await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
