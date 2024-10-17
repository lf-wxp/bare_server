use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;

use crate::{
  collection_wrapper,
  document::{Bubble, Font, FontAggregate, Text, WeightOption},
  responder::FindAllData,
};

use super::CollectionOperations;

collection_wrapper!(Texts, Text, "text", ["value"]);

collection_wrapper!(Bubbles, Bubble, "bubble", ["value"]);

collection_wrapper!(Fonts, Font, "font", ["value"]);

impl Fonts {
  pub async fn aggregate(
    &self,
    filter: &mut HashMap<String, String>,
  ) -> Result<FindAllData<FontAggregate>> {
    let fonts = Fonts::new();
    let timestamp_str = Utc::now().timestamp().to_string();
    filter.insert("expired_gt".to_string(), timestamp_str);
    filter.insert("enabled".to_string(), true.to_string());
    let FindAllData { list, .. } = fonts.list_pure(filter).await?;
    let mut font_map: HashMap<String, Vec<WeightOption>> = HashMap::new();
    list.into_iter().for_each(|font| {
      font_map
        .entry(font.name.clone())
        .or_insert_with(Vec::new)
        .push( WeightOption {
          label: font.weight.clone(),
          value: font.value,
          url: font.url,
        });
    });
    let list: Vec<FontAggregate> = font_map
      .into_iter()
      .map(|(name, weight_options)| FontAggregate {
        name,
        weight_options,
      })
      .collect();
    let count = list.len();
    Ok(FindAllData { list, count })
  }
}
