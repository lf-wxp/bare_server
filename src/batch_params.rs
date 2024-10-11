use chrono::Utc;
use mongodb::{
  bson::{doc, to_document},
  options::{DeleteOneModel, InsertOneModel, UpdateOneModel, WriteModel},
  Namespace,
};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{collection::DocWrap, filter::Filter, guard};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchUpdateItem<T> {
  filter: HashMap<String, String>,
  data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchUpdate<T>(Vec<BatchUpdateItem<T>>);

impl<T: DocWrap> BatchUpdate<T> {
  pub fn model(&self, namespace: Namespace) -> Vec<WriteModel> {
    self
      .0
      .iter()
      .map(|item| {
        let BatchUpdateItem { filter, data } = item;
        let filter = filter.exact(T::FIELD_NAMES_AS_SLICE);
        let now = Utc::now().timestamp();
        let mut item = to_document(&data).unwrap();
        item.insert("update_timestamp", now);
        let item = doc! { "$set": item };
        WriteModel::UpdateOne(
          UpdateOneModel::builder()
            .namespace(namespace.clone())
            .filter(filter)
            .update(item)
            .upsert(true)
            .build(),
        )
      })
      .collect::<Vec<WriteModel>>()
  }
}

impl<T: DocWrap> From<Json<Vec<BatchUpdateItem<T>>>> for BatchUpdate<T> {
  fn from(value: Json<Vec<BatchUpdateItem<T>>>) -> Self {
    Self(value.into_inner())
  }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchInsert<T>(Vec<T>);

impl<T: DocWrap> BatchInsert<T> {
  pub fn model(&mut self, namespace: Namespace) -> Vec<WriteModel> {
    self
      .0
      .iter_mut()
      .map(|item| {
        let now = Utc::now().timestamp();
        item.set_value();
        let mut doc = to_document(&item).unwrap();
        doc.insert("create_timestamp", now);
        doc.insert("update_timestamp", now);
        WriteModel::InsertOne(
          InsertOneModel::builder()
            .namespace(namespace.clone())
            .document(doc)
            .build(),
        )
      })
      .collect::<Vec<WriteModel>>()
  }
}
impl<T: DocWrap> From<guard::CustomJson<Vec<T>>> for BatchInsert<T> {
  fn from(value: guard::CustomJson<Vec<T>>) -> Self {
    Self(value.into_inner())
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchDelete(Vec<HashMap<String, String>>);

impl BatchDelete {
  pub fn model(&self, namespace: Namespace, fields: &[&str]) -> Vec<WriteModel> {
    self
      .0
      .iter()
      .map(|item| {
        let filter = item.exact(fields);
        WriteModel::DeleteOne(
          DeleteOneModel::builder()
            .namespace(namespace.clone())
            .filter(filter)
            .build(),
        )
      })
      .collect::<Vec<WriteModel>>()
  }
}

impl From<Json<Vec<HashMap<String, String>>>> for BatchDelete {
  fn from(value: Json<Vec<HashMap<String, String>>>) -> Self {
    Self(value.into_inner())
  }
}
