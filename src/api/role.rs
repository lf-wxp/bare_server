use futures::StreamExt;
use mongodb::{
  bson::{doc, to_document, Bson},
  error::Error,
};
use rocket::serde::json::Json;

use crate::{collection::Role, db::get_db, guard};

#[get("/")]
pub async fn get_role_list(_auth: guard::Auth) -> Json<Vec<Role>> {
  let db = get_db();
  let data = db.collection::<Role>("role").find(doc! {}).await.unwrap();
  let collect: Vec<Result<Role, Error>> = data.collect().await;
  Json(
    collect
      .into_iter()
      .filter_map(|x| match x {
        Ok(role) => Some(role),
        Err(_) => None,
      })
      .collect::<Vec<Role>>(),
  )
}
#[get("/<render_id>")]
pub async fn get_role(_auth: guard::Auth, render_id: &str) -> Json<Option<Role>> {
  let db = get_db();
  let role = db
    .collection::<Role>("role")
    .find_one(doc! { "render_id": render_id })
    .await
    .unwrap();
  Json(role)
}

#[post("/", format = "json", data = "<role>")]
pub async fn add_role(_auth: guard::Auth, role: Json<Role>) -> Json<Bson> {
  let db = get_db();
  let role = db
    .collection::<Role>("role")
    .insert_one(&*role)
    .await
    .unwrap();
  Json(role.inserted_id)
}

#[put("/<render_id>", format = "json", data = "<role>")]
pub async fn update_role(
  _auth: guard::Auth,
  render_id: &str,
  role: Json<Role>,
) -> Json<Option<Role>> {
  let db = get_db();
  let role = to_document(&*role).unwrap();
  let role = doc! { "$set": role };
  print!("update {:?}, {:?}", role, doc! {  "render_id": render_id });
  let role = db
    .collection::<Role>("role")
    .find_one_and_update(doc! { "render_id": render_id }, role)
    .await
    .unwrap();
  Json(role)
}

#[delete("/<render_id>")]
pub async fn delete_role(_auth: guard::Auth, render_id: &str) -> Json<Option<Role>> {
  let db = get_db();
  let role = db
    .collection::<Role>("role")
    .find_one_and_delete(doc! { "render_id": render_id })
    .await
    .unwrap();
  Json(role)
}

pub fn routes() -> Vec<rocket::Route> {
  routes![get_role, get_role_list, add_role, update_role, delete_role]
}
