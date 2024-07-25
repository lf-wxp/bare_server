use rocket::{
  http::{Method, Status},
  outcome::Outcome,
  request::{self, FromRequest, Request},
};

use crate::{forward, utils::get_cookies};

pub struct Auth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
    let cookies = get_cookies(request);
    let response = match forward::who(cookies.clone()).await {
      Ok(response) => response,
      Err(_) => {
        return Outcome::Error((Status::InternalServerError, ()));
      }
    };
    let method = request.method();
    if [Method::Post, Method::Put, Method::Delete, Method::Patch].contains(&method) {
      return match forward::is_paas_admin(cookies).await {
        Ok(auth) => {
          if auth {
            Outcome::Success(Auth {})
          } else {
            Outcome::Error((Status::Unauthorized, ()))
          }
        }
        Err(_) => Outcome::Error((Status::Unauthorized, ())),
      };
    }
    if response.status().is_success() {
      Outcome::Success(Auth {})
    } else {
      Outcome::Error((Status::Unauthorized, ()))
    }
  }
}
