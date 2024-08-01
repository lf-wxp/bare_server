use regex::Regex;
use rocket::{
  data::{Data, FromData, Limits, Outcome},
  http::Status,
  request::{local_cache, Request},
};
use serde::Deserialize;
use std::{io, ops::Deref};

#[derive(Debug)]
pub enum Error<'a> {
  Io(io::Error),
  Parse(&'a str, serde_json::error::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomJson<T>(pub T);

impl<'r, T: Deserialize<'r>> CustomJson<T> {
  fn from_str(s: &'r str) -> Result<Self, Error<'r>> {
    serde_json::from_str(s)
      .map(CustomJson)
      .map_err(|e| Error::Parse(s, e))
  }

  async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Result<Self, Error<'r>> {
    let limit = req.limits().get("json").unwrap_or(Limits::JSON);
    let string = match data.open(limit).into_string().await {
      Ok(s) if s.is_complete() => s.into_inner(),
      Ok(_) => {
        let eof = io::ErrorKind::UnexpectedEof;
        return Err(Error::Io(io::Error::new(eof, "data limit exceeded")));
      }
      Err(e) => return Err(Error::Io(e)),
    };

    Self::from_str(local_cache!(req, string))
  }
}

#[rocket::async_trait]
impl<'r, T: Deserialize<'r>> FromData<'r> for CustomJson<T> {
  type Error = Error<'r>;

  async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
    match Self::from_data(req, data).await {
      Ok(value) => Outcome::Success(value),
      Err(Error::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
        Outcome::Error((Status::PayloadTooLarge, Error::Io(e)))
      }
      Err(Error::Parse(s, e)) if e.classify() == serde_json::error::Category::Data => {
        let re = Regex::new(r"missing field `(.+?)`").unwrap();
        let err = e.to_string();
        let cap = &re.captures(&err).unwrap()[1];
        if !cap.is_empty() {
          req.local_cache(|| cap.to_string());
          return Outcome::Error((Status::from(Status { code: 460 }), Error::Parse(s, e)));
        }
        Outcome::Error((Status::from(Status { code: 461 }), Error::Parse(s, e)))
      }
      Err(e) => Outcome::Error((Status::from(Status { code: 461 }), e))
    }
  }
}

impl<T> From<T> for CustomJson<T> {
  fn from(value: T) -> Self {
    Self(value)
  }
}

impl<T> Deref for CustomJson<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.0
  }
}
