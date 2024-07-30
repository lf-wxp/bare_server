use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{Algorithms, CollectionOperations},
  document::Algorithm,
  guard,
  responder::DocumentActionResponder,
};

#[get("/alg?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  algorithms.list(&filter).await
}
#[get("/alg/<algorithm>")]
pub async fn get_item(_auth: guard::Auth, algorithm: &str) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  algorithms.find_one(doc! { "value": algorithm }).await
}

#[post("/alg", format = "json", data = "<algorithm>")]
pub async fn add_item(
  _auth: guard::Auth,
  algorithm: guard::CustomJson<Algorithm>,
) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  let mut algorithm = (*algorithm).clone();
  algorithms.insert(&mut algorithm).await
}

#[put("/alg/<algorithm_id>", format = "json", data = "<algorithm>")]
pub async fn update_item(
  _auth: guard::Auth,
  algorithm_id: &str,
  algorithm: Json<Algorithm>,
) -> DocumentActionResponder<Algorithm> {
  print!("update {:?}, {:?}", algorithm, doc! {  "value": algorithm_id });
  let algorithms = Algorithms::new();
  let algorithm = (*algorithm).clone();
  algorithms.update(doc! { "value": algorithm_id }, algorithm).await
}

#[delete("/alg/<algorithm>")]
pub async fn delete_item(_auth: guard::Auth, algorithm: &str) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  algorithms.delete(doc! { "value": algorithm }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
