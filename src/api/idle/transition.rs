use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, IdleTransitions},
  document::IdleTransition,
  guard,
  responder::DocumentActionResponder,
};

#[get("/idle_transition?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<IdleTransition> {
  let idle_transitions = IdleTransitions::new();
  idle_transitions.list(&filter).await
}
#[get("/idle_transition/<idle_transition>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  idle_transition: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<IdleTransition> {
  let idle_transitions = IdleTransitions::new();
  idle_transitions
    .find_one(doc! { "value": idle_transition }, &filter)
    .await
}

#[post("/idle_transition", format = "json", data = "<idle_transition>")]
pub async fn add_item(
  _auth: guard::Auth,
  idle_transition: guard::CustomJson<IdleTransition>,
) -> DocumentActionResponder<IdleTransition> {
  let idle_transitions = IdleTransitions::new();
  let mut idle_transition = (*idle_transition).clone();
  idle_transitions.insert(&mut idle_transition).await
}

#[put(
  "/idle_transition/<transition_id>?<filter..>",
  format = "json",
  data = "<idle_transition>"
)]
pub async fn update_item(
  _auth: guard::Auth,
  transition_id: &str,
  filter: HashMap<&str, &str>,
  idle_transition: Json<IdleTransition>,
) -> DocumentActionResponder<IdleTransition> {
  let idle_transitions = IdleTransitions::new();
  let idle_transition = (*idle_transition).clone();
  idle_transitions
    .update(doc! { "value": transition_id }, &filter, idle_transition)
    .await
}

#[delete("/idle_transition/<idle_transition>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  idle_transition: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<IdleTransition> {
  let idle_transitions = IdleTransitions::new();
  idle_transitions
    .delete(doc! { "value": idle_transition }, &filter)
    .await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_item, get_list, add_item, update_item, delete_item]
}
