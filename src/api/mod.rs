pub mod action;
pub mod camera;
pub mod costume;
pub mod hairdo;
pub mod material;
pub mod role;
pub mod scene;
pub mod text;
pub mod timbre;
pub mod algorithm;
pub mod idle;

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  for route in vec![
    role::routes(),
    action::routes(),
    camera::routes(),
    costume::routes(),
    hairdo::routes(),
    material::routes(),
    text::routes(),
    timbre::routes(),
    scene::routes(),
    algorithm::routes(),
    idle::routes(),
  ] {
    routes.extend(route);
  }
  routes
}
