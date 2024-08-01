use reqwest::{header::COOKIE, Client, Error, Response};
use serde::Deserialize;

use crate::config::get_config;

#[derive(Deserialize, Debug)]
pub struct Profile {
  pub paas_admin: Option<bool>,
  pub user_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileResp {
  pub response: Profile,
}

async fn request(path: &str, cookies: String) -> Result<Response, Error> {
  let server = get_config("AUTH_SERVER");
  let auth_url = format!("{}/api/{}", server, path);
  let client = Client::new();
  client.get(&auth_url).header(COOKIE, cookies).send().await
}

pub async fn who(cookies: String) -> Result<Response, Error> {
  request("auth/who", cookies).await
}

pub async fn profile(cookies: String) -> Result<Response, Error> {
  let paas_id = get_config("PAAS_ID");
  let path = format!("auth/profile?paas_id={}", paas_id);
  request(&path, cookies).await
}

pub async fn is_paas_admin(cookies: String) -> Result<bool, Error> {
  let paas_id = get_config("PAAS_ID");
  let path = format!("auth/profile?paas_id={}", paas_id);
  let response = request(&path, cookies).await?;
  match response.json::<ProfileResp>().await {
    Ok(json) => Ok(json.response.paas_admin.unwrap_or(false)),
    Err(err) => Err(err),
  }
}
