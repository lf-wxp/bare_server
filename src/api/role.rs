use mongodb::bson::{doc, Bson};
use rocket::serde::json::Json;

use crate::{
  collection::{CollectionOperations, Roles},
  document::Role,
  guard,
};

#[get("/")]
pub async fn get_role_list(_auth: guard::Auth) -> Json<Vec<Role>> {
  let roles = Roles::new();
  let list = roles.list(doc! {}).await.unwrap();
  Json(list)
}
#[get("/<render_id>")]
pub async fn get_role(_auth: guard::Auth, render_id: &str) -> Json<Option<Role>> {
  let roles = Roles::new();
  let role = roles
    .find_one(doc! { "render_id": render_id })
    .await
    .unwrap();
  Json(role)
}

#[post("/", format = "json", data = "<role>")]
pub async fn add_role(_auth: guard::Auth, role: Json<Role>) -> Json<Bson> {
  let roles = Roles::new();
  let mut role = (*role).clone();
  let role = roles.insert(&mut role).await.unwrap();
  Json(role.inserted_id)
}

#[put("/<render_id>", format = "json", data = "<role>")]
pub async fn update_role(
  _auth: guard::Auth,
  render_id: &str,
  role: Json<Role>,
) -> Json<Option<Role>> {
  print!("update {:?}, {:?}", role, doc! {  "render_id": render_id });
  let roles = Roles::new();
  let role = (*role).clone();
  let role = roles
    .update(doc! { "render_id": render_id }, role)
    .await
    .unwrap();
  Json(role)
}

#[delete("/<render_id>")]
pub async fn delete_role(_auth: guard::Auth, render_id: &str) -> Json<Option<Role>> {
  let roles = Roles::new();
  let role = roles.delete(doc! { "render_id": render_id }).await.unwrap();
  Json(role)
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_role, get_role_list, add_role, update_role, delete_role]
}
