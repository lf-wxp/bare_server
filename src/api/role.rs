use mongodb::bson::{doc, Bson};
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, Roles, Pagination},
  document::Role,
  guard,
};

#[get("/?<pagination..>")]
pub async fn get_role_list(_auth: guard::Auth, pagination: Pagination) -> Json<Vec<Role>> {
  let roles = Roles::new();
  let list = roles.list(pagination).await.unwrap();
  Json(list)
}
#[get("/<role>")]
pub async fn get_role(_auth: guard::Auth, role: &str) -> Json<Option<Role>> {
  let roles = Roles::new();
  let role = roles
    .find_one(doc! { "role": role })
    .await
    .unwrap();
  Json(role)
}

#[post("/", format = "json", data = "<role>")]
pub async fn add_role(_auth: guard::Auth, role: guard::CustomJson<Role> ) -> Json<Bson> {
  let roles = Roles::new();
  let mut role = (*role).clone();
  let role = roles.insert(&mut role).await.unwrap();
  Json(role.inserted_id)
}

#[put("/<role_id>", format = "json", data = "<role>")]
pub async fn update_role(
  _auth: guard::Auth,
  role_id: &str,
  role: Json<Role>,
) -> Json<Option<Role>> {
  print!("update {:?}, {:?}", role, doc! {  "role": role_id });
  let roles = Roles::new();
  let role = (*role).clone();
  let role = roles
    .update(doc! { "role": role_id }, role)
    .await
    .unwrap();
  Json(role)
}

#[delete("/<role>")]
pub async fn delete_role(_auth: guard::Auth, role: &str) -> Json<Option<Role>> {
  let roles = Roles::new();
  let role = roles.delete(doc! { "role": role }).await.unwrap();
  Json(role)
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_role, get_role_list, add_role, update_role, delete_role]
}
