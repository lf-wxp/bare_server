use mongodb::error;
use std::collections::HashMap;

use crate::{
  collection_wrapper,
  document::{Costume, CostumeCategory, CostumeWithCategory},
  responder::FindAllData,
};

use super::CollectionOperations;

collection_wrapper!(Costumes, Costume, "costume", ["value", "role"]);

collection_wrapper!(
  CostumeCategories,
  CostumeCategory,
  "costume-category",
  ["name", "role"]
);

impl Costumes {
  pub async fn aggregate(
    &self,
    filter: &HashMap<&str, &str>,
  ) -> error::Result<FindAllData<CostumeWithCategory>> {
    let costume_categories = CostumeCategories::new();
    let FindAllData {
      list: costumes,
      count,
    } = self.list_pure(filter).await?;
    let FindAllData {
      list: categories, ..
    } = costume_categories.list_pure(filter).await?;
    let list = categories
      .into_iter()
      .map(|category| {
        let CostumeCategory { name, role, required, .. } = category;
        CostumeWithCategory {
          role,
          category: name.clone(),
          required: required.unwrap_or(false),
          costume: costumes
            .clone()
            .into_iter()
            .filter_map(|x| if x.category == name { Some(x) } else { None })
            .collect(),
        }
      })
      .collect();
    Ok(FindAllData { list, count })
  }
}
