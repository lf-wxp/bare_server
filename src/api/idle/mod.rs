pub mod idle;
pub mod mapping;
pub mod transition;

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  for route in vec![idle::routes(), mapping::routes(), transition::routes()] {
    routes.extend(route);
  }
  routes
}
