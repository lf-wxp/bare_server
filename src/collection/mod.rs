use chrono::Utc;
use futures::StreamExt;
use mongodb::{
  bson::{self, doc, from_document, to_document, Document},
  error, Collection,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, io};

pub mod actions;
pub mod algorithms;
pub mod cameras;
pub mod costumes;
pub mod hairdos;
pub mod idles;
pub mod materials;
pub mod roles;
pub mod scenes;
pub mod timbres;
pub mod texts;

pub use roles::*;
pub use actions::*;
pub use cameras::*;
pub use costumes::*;
pub use hairdos::*;
pub use idles::*;
pub use materials::*;
pub use scenes::*;
pub use timbres::*;
pub use texts::*;
pub use algorithms::*;

use crate::{
  filter::Filter,
  responder::{DocumentActionResponder, FindAllData},
};

pub trait DocWrap: Serialize + Debug + for<'de> Deserialize<'de> + Send + Sync {}
impl<T> DocWrap for T where T: Serialize + Debug + for<'de> Deserialize<'de> + Send + Sync {}

pub trait CollectionOperations {
  type Doc: DocWrap;
  fn collection(&self) -> &Collection<Self::Doc>;
  async fn insert(&self, item: &mut Self::Doc) -> DocumentActionResponder<Self::Doc> {
    let now = Utc::now().timestamp();
    let mut doc = to_document(&item).unwrap();
    doc.insert("create_timestamp", now);
    doc.insert("update_timestamp", now);
    let doc = from_document::<Self::Doc>(doc).unwrap();
    let result = self.collection().insert_one(doc).await;
    DocumentActionResponder::Insert(result)
  }

  async fn list_pure(&self, filter: &HashMap<&str, &str>) -> error::Result<FindAllData<Self::Doc>>{
    let (query, sort, skip, limit) = filter.parse();
    let pipeline = vec![
      doc! {
        "$match":query
      },
      doc! {
          "$sort": sort,
      },
      doc! {
        "$facet": {
          "list": [
            { "$skip": skip},
            { "$limit": limit },
          ],
          "count": [
            { "$count": "count"},
          ]
        }
      },
      doc! {
        "$project": {
          "list": 1,
          "count": { "$arrayElemAt": [ "$count", 0 ] },
        }
      },
    ];
    let result = self.collection().aggregate(pipeline).await;
    match result {
      Ok(mut cursor) => {
        let result = cursor.next().await;
        if let Some(Ok(doc)) = result {
          let list: Vec<Self::Doc> = doc
            .get_array("list")
            .map(|bson_array| {
              bson_array
                .iter()
                .map(|bson| bson::from_bson(bson.clone()))
                .collect::<Result<Vec<Self::Doc>, _>>()
            })
            .unwrap_or(Ok(Vec::new()))
            .unwrap_or_default();

          let count = doc
            .get_document("count")
            .and_then(|count_doc| count_doc.get_i32("count"))
            .unwrap_or(0) as usize;
          Ok(FindAllData { list, count })
        } else {
          Err(error::Error::from(io::Error::new(
            io::ErrorKind::Other,
            "Failed to get result",
          )))
        }
      }
      Err(e) => Err(e),
    }
  }

  async fn list(&self, filter: &HashMap<&str, &str>) -> DocumentActionResponder<Self::Doc> {
    let result = self.list_pure(&filter).await;
    DocumentActionResponder::FindAll(result)
  }

  async fn find_one(&self, filter: Document) -> DocumentActionResponder<Self::Doc> {
    DocumentActionResponder::FindOne(self.collection().find_one(filter).await)
  }

  async fn update(&self, filter: Document, item: Self::Doc) -> DocumentActionResponder<Self::Doc> {
    let now = Utc::now().timestamp();
    let mut item = to_document(&item).unwrap();
    item.insert("update_timestamp", now);
    let item = doc! { "$set": item };
    DocumentActionResponder::Update(self.collection().find_one_and_update(filter, item).await)
  }

  async fn delete(&self, filter: Document) -> DocumentActionResponder<Self::Doc> {
    DocumentActionResponder::Delete(self.collection().find_one_and_delete(filter).await)
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
  Actions::create_unique_index().await.unwrap();
  Algorithms::create_unique_index().await.unwrap();
  Cameras::create_unique_index().await.unwrap();
  Hairdos::create_unique_index().await.unwrap();
  Costumes::create_unique_index().await.unwrap();
  CostumeCategories::create_unique_index().await.unwrap();
  Idles::create_unique_index().await.unwrap();
  IdleTransitions::create_unique_index().await.unwrap();
  IdleMappings::create_unique_index().await.unwrap();
  Materials::create_unique_index().await.unwrap();
  Texts::create_unique_index().await.unwrap();
  Bubbles::create_unique_index().await.unwrap();
  Scenes::create_unique_index().await.unwrap();
  Timbres::create_unique_index().await.unwrap();
}
