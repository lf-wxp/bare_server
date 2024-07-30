use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, CostumeCategories},
  document::CostumeCategory,
  guard,
  responder::DocumentActionResponder,
};

#[get("/costume_category?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  costume_categories.list(&filter).await
}
#[get("/costume_category/<category_name>")]
pub async fn get_item(
  _auth: guard::Auth,
  category_name: &str,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  costume_categories
    .find_one(doc! { "name": category_name })
    .await
}

#[post("/costume_category", format = "json", data = "<category>")]
pub async fn add_item(
  _auth: guard::Auth,
  category: guard::CustomJson<CostumeCategory>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  let mut category = (*category).clone();
  costume_categories.insert(&mut category).await
}

#[put("/costume_category/<category_name>", format = "json", data = "<category>")]
pub async fn update_item(
  _auth: guard::Auth,
  category_name: &str,
  category: Json<CostumeCategory>,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  let category = (*category).clone();
  costume_categories
    .update(doc! { "name": category_name}, category)
    .await
}

#[delete("/costume_category/<category_name>")]
pub async fn delete_item(
  _auth: guard::Auth,
  category_name: &str,
) -> DocumentActionResponder<CostumeCategory> {
  let costume_categories = CostumeCategories::new();
  costume_categories
    .delete(doc! { "name": category_name })
    .await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
