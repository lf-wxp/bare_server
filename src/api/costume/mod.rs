pub mod category;
pub mod costume;

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  for route in vec![category::routes(), costume::routes()] {
    routes.extend(route);
  }
  routes
}
