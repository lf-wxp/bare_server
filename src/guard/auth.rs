use rocket::{
  http::{Method, Status},
  outcome::Outcome,
  request::{self, FromRequest, Request},
};

use crate::{
  forward::{self, ProfileResp},
  utils::get_cookies,
};

pub struct Auth {
  pub user: Option<String>,
}

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
    let is_success = response.status().is_success();
    let user = response.json::<ProfileResp>().await.map_or(None, |x| {
      Some(x.response.user_name.unwrap_or("".to_string()))
    });
    let method = request.method();
    if [Method::Post, Method::Put, Method::Delete, Method::Patch].contains(&method) {
      return match forward::is_paas_admin(cookies).await {
        Ok(auth) => {
          if auth {
            Outcome::Success(Auth { user })
          } else {
            Outcome::Error((Status::Unauthorized, ()))
          }
        }
        Err(_) => Outcome::Error((Status::Unauthorized, ())),
      };
    }
    if is_success {
      Outcome::Success(Auth { user })
    } else {
      Outcome::Error((Status::Unauthorized, ()))
    }
  }
}
