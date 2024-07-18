use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
  bson::{doc, from_document, to_document, Document},
  error::Result,
  results::InsertOneResult,
  Collection,
};
use serde::{Deserialize, Serialize};

pub mod roles;

pub use roles::*;

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

  async fn list(&self, filter: Document) -> Result<Vec<Self::Doc>> {
    let cursor = self.collection().find(filter).await?;
    cursor.try_collect::<Vec<Self::Doc>>().await
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
  ($name:ident, $doc:ty, $collection_name:expr, $field_name:expr) => {
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

      pub async fn create_unique_index(&self) -> mongodb::error::Result<()> {
        let index_options = mongodb::options::IndexOptions::builder()
          .unique(true)
          .build();
        let index_model = mongodb::IndexModel::builder()
          .keys(mongodb::bson::doc! { $field_name: 1 })
          .options(index_options)
          .build();
        self.collection.create_index(index_model).await?;
        Ok(())
      }
    }
  };
}
