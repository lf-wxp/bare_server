use reqwest::{header::COOKIE, Client, Error, Response};
use serde::Deserialize;

use crate::config::get_config;

#[derive(Deserialize)]
struct Profile {
  paas_admin: bool,
}

#[derive(Deserialize)]
struct ProfileResp {
  response: Profile,
}

async fn request(path: &str, cookies: String) -> Result<Response, Error> {
  let auth_url = format!("{}/api/{}", get_config("AUTH_SERVER"), path);
  let client = Client::new();
  client.get(&auth_url).header(COOKIE, cookies).send().await
}

pub async fn who(cookies: String) -> Result<Response, Error> {
  request("auth/who", cookies).await
}

pub async fn profile(cookies: String) -> Result<Response, Error> {
  let path = format!("auth/profile?paas_id={}", get_config("PASS_ID"));
  request(&path, cookies).await
}

pub async fn is_paas_admin(cookies: String) -> Result<bool, Error> {
  let path = format!("auth/profile?paas_id={}", get_config("PASS_ID"));
  let response = request(&path, cookies).await?;
  match response.json::<ProfileResp>().await {
    Ok(json) => Ok(json.response.paas_admin),
    Err(err) => Err(err),
  }
}
