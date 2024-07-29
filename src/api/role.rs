use std::collections::HashMap;

use mongodb::bson::doc;
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, Roles},
  document::Role,
  guard,
  responder::DocumentActionResponder,
};

#[get("/?<filter..>")]
pub async fn get_role_list(
  _auth: guard::Auth,
  filter: HashMap<&str, &str>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  roles.list(filter).await
}
#[get("/<role>")]
pub async fn get_role(_auth: guard::Auth, role: &str) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  roles.find_one(doc! { "role": role }).await
}

#[post("/", format = "json", data = "<role>")]
pub async fn add_role(
  _auth: guard::Auth,
  role: guard::CustomJson<Role>,
) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  let mut role = (*role).clone();
  roles.insert(&mut role).await
}

#[put("/<role_id>", format = "json", data = "<role>")]
pub async fn update_role(
  _auth: guard::Auth,
  role_id: &str,
  role: Json<Role>,
) -> DocumentActionResponder<Role> {
  print!("update {:?}, {:?}", role, doc! {  "role": role_id });
  let roles = Roles::new();
  let role = (*role).clone();
  roles.update(doc! { "role": role_id }, role).await
}

#[delete("/<role>")]
pub async fn delete_role(_auth: guard::Auth, role: &str) -> DocumentActionResponder<Role> {
  let roles = Roles::new();
  roles.delete(doc! { "role": role }).await
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_role, get_role_list, add_role, update_role, delete_role]
}
