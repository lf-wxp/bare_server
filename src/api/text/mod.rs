pub mod bubble;
pub mod text;
pub mod font;

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  for route in vec![
    bubble::routes(),
    text::routes(),
    font::routes(),
  ] {
    routes.extend(route);
  }
  routes
}
