use crate::{collection_wrapper, document::FavoriteAction};

collection_wrapper!(
  FavoriteActions,
  FavoriteAction,
  "favorite-action",
  ["value", "role", "user"]
);
