use anyhow::Result;
use std::collections::HashMap;

use crate::{
  collection_wrapper,
  document::{Action, ActionCategory, ActionWithCategory},
  responder::FindAllData,
};

use super::CollectionOperations;

collection_wrapper!(Actions, Action, "action", ["value", "role"]);

collection_wrapper!(
  ActionCategories,
  ActionCategory,
  "action-category",
  ["name", "role"]
);

impl Actions {
  pub async fn aggregate(
    &self,
    filter: &HashMap<String, String>,
  ) -> Result<FindAllData<ActionWithCategory>> {
    let action_categories = ActionCategories::new();
    let FindAllData {
      list: actions,
      count,
    } = self.list_pure(filter).await?;
    let FindAllData {
      list: categories, ..
    } = action_categories.list_pure(filter).await?;
    let list = categories
      .into_iter()
      .map(|category| {
        let ActionCategory { name, role, .. } = category;
        ActionWithCategory {
          role,
          category: name.clone(),
          action: actions
            .clone()
            .into_iter()
            .filter_map(|x| {
              if x.category == Some(name.clone()) {
                Some(x)
              } else {
                None
              }
            })
            .collect(),
        }
      })
      .collect();
    Ok(FindAllData { list, count })
  }
}
