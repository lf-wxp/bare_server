use config::set_config;
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
mod api;
mod batch_params;
mod catcher;
mod config;
mod document;
mod filter;
mod responder;

#[launch]
async fn rocket() -> _ {
  set_config();
  init_db().await.unwrap();
  collection::create_db_index().await;
  rocket::build()
    .attach(fairing::Log)
    .attach(fairing::JsonResponse)
    .attach(fairing::Gzip)
    .register("/", catcher::catcher())
    .mount("/", api::routes())
}
