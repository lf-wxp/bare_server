use mongodb::{bson::doc, error::Error};
use rocket::serde::json::Json;
use std::collections::HashMap;

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
#[get("/alg/<algorithm>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  algorithm: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  algorithms
    .find_one(doc! { "value": algorithm }, &filter)
    .await
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

#[put(
  "/alg/<algorithm_id>?<filter..>",
  format = "json",
  data = "<algorithm>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  algorithm_id: &str,
  filter: HashMap<&str, &str>,
  algorithm: Json<Algorithm>,
) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  let algorithm = (*algorithm).clone();
  algorithms
    .update(doc! { "value": algorithm_id }, &filter, algorithm)
    .await
}

#[delete("/alg/<algorithm>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  algorithm: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  algorithms
    .delete(doc! { "value": algorithm }, &filter)
    .await
}

#[delete("/alg?<confirm..>")]
pub async fn drop(
  _auth: guard::Auth,
  confirm: HashMap<&str, &str>,
) -> DocumentActionResponder<Algorithm> {
  let algorithms = Algorithms::new();
  let confirm = confirm.get("confirm").map_or(false, |x| *x == "true");
  if confirm {
    return algorithms.drop().await;
  }
  return DocumentActionResponder::Drop(Err(Error::custom("cannot operate")));
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item, drop]
}
