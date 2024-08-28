pub mod action;
pub mod category;

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  for route in vec![category::routes(), action::routes()] {
    routes.extend(route);
  }
  routes
}
