use mongodb::{
  error::{Error, Result},
  results::InsertOneResult,
};
use nanoid::nanoid;
use rocket::{
  http::{ContentType, Status},
  response::{self, Responder, Response},
  Request,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::io::Cursor;

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub ret_code: u16,
  pub ret_msg: String,
  pub response: Option<T>,
  pub session_id: String,
}

use crate::collection::DocWrap;

#[derive(Serialize, Deserialize, Debug)]
pub struct FindAllData<T> {
  pub list: Vec<T>,
  pub count: usize,
}

#[derive(Debug)]
pub enum DocumentActionResponder<T: DocWrap> {
  Insert(Result<InsertOneResult>),
  FindOne(Result<Option<T>>),
  FindAll(Result<FindAllData<T>>),
  Delete(Result<Option<T>>),
  Update(Result<Option<T>>),
}

fn handle_error(err: &Error) -> (Status, Value) {
  (Status::InternalServerError, json!(err.to_string()))
}

fn handle_not_found() -> (Status, Value) {
  (Status::NotFound, json!("Document not found"))
}
impl<'r, T: DocWrap> Responder<'r, 'static> for DocumentActionResponder<T> {
  fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
    let session_id = request
      .query_value("session_id")
      .unwrap_or(Ok(nanoid!()))
      .unwrap();
    let (status, body_json) = match self {
      DocumentActionResponder::Insert(result) => match result {
        Ok(data) => (Status::Ok, json!(data)),
        Err(err) => handle_error(&err),
      },
      DocumentActionResponder::FindAll(result) => match result {
        Ok(docs) => (Status::Ok, json!(docs)),
        Err(err) => handle_error(&err),
      },
      DocumentActionResponder::FindOne(result)
      | DocumentActionResponder::Delete(result)
      | DocumentActionResponder::Update(result) => match result {
        Ok(Some(doc)) => (Status::Ok, json!(doc)),
        Ok(None) => handle_not_found(),
        Err(err) => handle_error(&err),
      },
    };

    let ret_code = if status == Status::Ok { 0 } else { status.code };
    let (message, data) = if status == Status::Ok {
      ("success".to_string(), Some(body_json))
    } else {
      (body_json.as_str().unwrap().to_string(), None)
    };
    let api_response = ApiResponse {
      ret_code,
      ret_msg: message,
      response: data,
      session_id,
    };
    let api_response_string = to_string(&api_response).unwrap();
    Response::build()
      .status(Status::Ok)
      .header(ContentType::JSON)
      .sized_body(api_response_string.len(), Cursor::new(api_response_string))
      .ok()
  }
}
