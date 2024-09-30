use mongodb::bson::doc;
use rocket::serde::json::Json;
use std::collections::HashMap;

use crate::{
  collection::{CollectionOperations, Roles},
  document::{Role, RoleAggregate},
  guard,
  responder::DocumentActionResponder,
};

#[get("/role_aggregate?<filter..>")]
pub async fn get_aggregate_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<RoleAggregate> {
  let roles = Roles::new();
  roles.aggregate(&filter).await.into()
}

#[get("/role?<filter..>")]
pub async fn get_list(
  _auth: guard::Auth,
  filter: HashMap<String, String>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  roles.list(&filter).await
}
#[get("/role/<role>?<filter..>")]
pub async fn get_item(
  _auth: guard::Auth,
  role: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  roles.find_one(doc! { "role": role }, &filter).await
}

#[post("/role", format = "json", data = "<role>")]
pub async fn add_item(
  _auth: guard::Auth,
  role: guard::CustomJson<Role>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  let mut role = (*role).clone();
  roles.insert(&mut role).await
}

#[put("/role/<role_id>?<filter..>", format = "json", data = "<role>")]
pub async fn update_item(
  _auth: guard::Auth,
  role_id: &str,
  filter: HashMap<&str, &str>,
  role: Json<Role>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  let role = (*role).clone();
  roles.update(doc! { "role": role_id }, &filter, role).await
}

#[delete("/role/<role>?<filter..>")]
pub async fn delete_item(
  _auth: guard::Auth,
  role: &str,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  roles.delete(doc! { "role": role }, &filter).await
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
