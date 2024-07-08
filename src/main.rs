#[macro_use]
extern crate rocket;

mod fairing;
mod guard;
mod params;
mod utils;
mod forward;

#[get("/")]
fn index(_auth: guard::Auth) -> String {
  "HELLO WORLD".to_string()
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(fairing::Gzip)
    .mount("/", routes![index])
}
