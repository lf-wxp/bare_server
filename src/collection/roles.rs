use anyhow::Result;
use std::collections::HashMap;

use crate::{
  collection_wrapper,
  document::{LinkRole, Role, RoleAggregate},
  responder::FindAllData,
};

use super::{CollectionOperations, Costumes, Hairdos, Timbres};

collection_wrapper!(Roles, Role, "role", ["role"]);

impl Roles {
  fn filter_items<T: Clone + LinkRole>(items: Vec<T>, role_id: &str) -> Vec<T> {
    items
      .into_iter()
      .filter(|x| x.clone().role() == role_id)
      .collect()
  }

  pub async fn aggregate(
    &self,
    filter: &HashMap<String, String>,
  ) -> Result<FindAllData<RoleAggregate>> {
    let timbres = Timbres::new();
    let costumes = Costumes::new();
    let hairdos = Hairdos::new();
    let FindAllData { list: roles, count } = self.list_pure(filter).await?;
    let FindAllData { list: timbres, .. } = timbres.list_pure(filter).await?;
    let FindAllData { list: costumes, .. } = costumes.aggregate(filter).await?;
    let FindAllData { list: hairdos, .. } = hairdos.list_pure(filter).await?;
    let list = roles
      .into_iter()
      .map(|role| {
        let role_id = role.role.clone();
        let mut role_aggregate = RoleAggregate::from(role);
        role_aggregate.timbres = Self::filter_items(timbres.clone(), &role_id);
        role_aggregate.hairdos = Self::filter_items(hairdos.clone(), &role_id);
        role_aggregate.costumes = Self::filter_items(costumes.clone(), &role_id);
        role_aggregate
      })
      .collect();
    Ok(FindAllData { list, count })
  }
}
