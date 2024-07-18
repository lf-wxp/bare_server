use db::init_db;

#[macro_use]
extern crate rocket;

mod db;
mod fairing;
mod forward;
mod guard;
mod params;
mod utils;
#[macro_use]
mod collection;

mod document;
mod api;

#[launch]
async fn rocket() -> _ {
  init_db().await.unwrap();
  rocket::build()
    .attach(fairing::JsonResponse)
    .attach(fairing::Gzip)
    .mount("/role", api::role::routes())
}
