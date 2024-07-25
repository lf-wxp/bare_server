use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
  bson::{self, doc, from_bson, from_document, to_document, Bson, Document},
  error::{Error, Result},
  results::InsertOneResult,
  Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::io;

pub mod actions;
pub mod algorithms;
pub mod cameras;
pub mod costumes;
pub mod hairdos;
pub mod idles;
pub mod materials;
pub mod roles;
pub mod timbres;

pub use roles::*;

#[derive(Debug, FromForm)]
pub struct Pagination {
  page: Option<usize>,
  size: Option<usize>,
  filter: Option<String>,
}

pub trait DocWrap: Serialize + for<'de> Deserialize<'de> + Send + Sync {}
impl<T> DocWrap for T where T: Serialize + for<'de> Deserialize<'de> + Send + Sync {}

pub trait CollectionOperations {
  type Doc: DocWrap;
  fn collection(&self) -> &Collection<Self::Doc>;
  async fn insert(&self, item: &mut Self::Doc) -> Result<InsertOneResult> {
    let now = Utc::now().timestamp();
    let mut doc = to_document(&item).unwrap();
    doc.insert("create_timestamp", now);
    doc.insert("update_timestamp", now);
    let doc = from_document::<Self::Doc>(doc).unwrap();
    self.collection().insert_one(doc).await
  }

  async fn list(&self, pagination: Pagination) -> Result<Vec<Self::Doc>> {
    let page = pagination.page.unwrap_or(1);
    let size = pagination.size.unwrap_or(10000);
    let skip = (page - 1) * size;
    let filter = match pagination.filter {
      Some(filter_str) => match from_str::<Value>(&filter_str) {
        Ok(json_value) => bson::to_bson(&json_value)
          .unwrap()
          .as_document()
          .unwrap()
          .clone(),
        Err(_) => doc! {},
      },
      None => doc! {},
    };
    let pipeline = vec![
      doc! { "$match": filter},
      doc! { "$skip": skip as f32},
      doc! { "$limit": size as f32 },
    ];
    self
      .collection()
      .aggregate(pipeline)
      .await?
      .and_then(|doc_result| async move {
        from_bson(Bson::Document(doc_result))
          .map_err(|e| Error::from(io::Error::new(io::ErrorKind::Other, e)))
      })
      .try_collect::<Vec<Self::Doc>>()
      .await
  }

  async fn find_one(&self, filter: Document) -> Result<Option<Self::Doc>> {
    self.collection().find_one(filter).await
  }

  async fn update(&self, filter: Document, item: Self::Doc) -> Result<Option<Self::Doc>> {
    let now = Utc::now().timestamp();
    let mut item = to_document(&item).unwrap();
    item.insert("update_timestamp", now);
    let item = doc! { "$set": item };
    self.collection().find_one_and_update(filter, item).await
  }

  async fn delete(&self, filter: Document) -> Result<Option<Self::Doc>> {
    self.collection().find_one_and_delete(filter).await
  }
}

#[macro_export]
macro_rules! collection_wrapper {
  ($name:ident, $doc:ty, $collection_name:expr, $fields:expr) => {
    pub struct $name {
      collection: mongodb::Collection<$doc>,
    }
    impl crate::collection::CollectionOperations for $name {
      type Doc = $doc;

      fn collection(&self) -> &mongodb::Collection<$doc> {
        &self.collection
      }
    }
    impl $name {
      pub fn new() -> Self {
        let db = crate::db::get_db();
        let collection = db.collection($collection_name);
        Self { collection }
      }

      pub async fn create_unique_index() -> mongodb::error::Result<()> {
        let index_options = mongodb::options::IndexOptions::builder()
          .unique(true)
          .build();
        let mut index_keys = mongodb::bson::Document::new();
        for field in $fields {
          index_keys.insert(field, 1);
        }
        let index_model = mongodb::IndexModel::builder()
          .keys(index_keys)
          .options(index_options)
          .build();
        let db = crate::db::get_db();
        let collection: mongodb::Collection<$doc> = db.collection($collection_name);
        collection.create_index(index_model).await?;
        Ok(())
      }
    }
  };
}

pub async fn create_db_index() {
  Roles::create_unique_index().await.unwrap();
}
