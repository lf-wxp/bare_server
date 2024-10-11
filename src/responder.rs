use anyhow::Error;
use mongodb::{
  error::Result,
  results::{DeleteResult, InsertManyResult, InsertOneResult, SummaryBulkWriteResult},
};
use nanoid::nanoid;
use rocket::{
  http::{ContentType, Status},
  response::{self, Responder, Response},
  Request,
};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::io::Cursor;

use crate::collection::DocWrap;

#[derive(Debug, Clone)]
pub struct BulkResult(SummaryBulkWriteResult);

impl Serialize for BulkResult {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let SummaryBulkWriteResult {
      inserted_count,
      matched_count,
      modified_count,
      upserted_count,
      deleted_count,
      ..
    } = self.0;
    let mut state = serializer.serialize_struct("BulkResult", 5)?;
    state.serialize_field("inserted_count", &inserted_count)?;
    state.serialize_field("matched_count", &matched_count)?;
    state.serialize_field("modified_count", &modified_count)?;
    state.serialize_field("upserted_count", &upserted_count)?;
    state.serialize_field("deleted_count", &deleted_count)?;
    state.end()
  }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub ret_code: u16,
  pub ret_msg: String,
  pub response: Option<T>,
  pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindAllData<T> {
  pub list: Vec<T>,
  pub count: usize,
}

pub struct DocumentActionResponder<T: DocWrap>(pub anyhow::Result<DocumentAction<T>>);

impl<T: DocWrap> From<anyhow::Result<FindAllData<T>>> for DocumentActionResponder<T> {
  fn from(value: anyhow::Result<FindAllData<T>>) -> Self {
    DocumentActionResponder(value.map(|value| DocumentAction::FindAll(value)))
  }
}

impl<T: DocWrap> From<Result<FindAllData<T>>> for DocumentActionResponder<T> {
  fn from(value: Result<FindAllData<T>>) -> Self {
    DocumentActionResponder(
      value
        .map(|value| DocumentAction::FindAll(value))
        .map_err(|e| e.into()),
    )
  }
}

impl<T: DocWrap> From<Result<InsertOneResult>> for DocumentActionResponder<T> {
  fn from(value: Result<InsertOneResult>) -> Self {
    DocumentActionResponder(
      value
        .map(|value| DocumentAction::Insert(value))
        .map_err(|e| e.into()),
    )
  }
}

impl<T: DocWrap> From<Result<DeleteResult>> for DocumentActionResponder<T> {
  fn from(value: Result<DeleteResult>) -> Self {
    DocumentActionResponder(
      value
        .map(|value| DocumentAction::DeleteMany(value))
        .map_err(|e| e.into()),
    )
  }
}

impl<T: DocWrap> From<Result<SummaryBulkWriteResult>> for DocumentActionResponder<T> {
  fn from(value: Result<SummaryBulkWriteResult>) -> Self {
    DocumentActionResponder(
      value
        .map(|value| DocumentAction::BulkOperation(BulkResult(value)))
        .map_err(|e| e.into()),
    )
  }
}

impl<T: DocWrap> From<Result<InsertManyResult>> for DocumentActionResponder<T> {
  fn from(value: Result<InsertManyResult>) -> Self {
    DocumentActionResponder(
      value
        .map(|value| DocumentAction::InsertMany(value))
        .map_err(|e| e.into()),
    )
  }
}

impl<T: DocWrap> From<Result<Option<T>>> for DocumentActionResponder<T> {
  fn from(value: Result<Option<T>>) -> Self {
    DocumentActionResponder(
      value
        .map(|value| DocumentAction::EffectOne(value))
        .map_err(|e| e.into()),
    )
  }
}

#[derive(Debug)]
pub enum DocumentAction<T: DocWrap> {
  Insert(InsertOneResult),
  InsertMany(InsertManyResult),
  FindAll(FindAllData<T>),
  EffectOne(Option<T>),
  DeleteMany(DeleteResult),
  BulkOperation(BulkResult),
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
    let (status, body_json) = match self.0 {
      Ok(result) => match result {
        DocumentAction::EffectOne(data) => match data {
          Some(data) => (Status::Ok, json!(data)),
          None => handle_not_found(),
        },
        DocumentAction::Insert(data) => (Status::Ok, json!(data)),
        DocumentAction::FindAll(data) => (Status::Ok, json!(data)),
        DocumentAction::DeleteMany(data) => (Status::Ok, json!(data)),
        DocumentAction::BulkOperation(data) => (Status::Ok, json!(data)),
        DocumentAction::InsertMany(data) => (Status::Ok, json!(data)),
      },
      Err(err) => handle_error(&err),
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
