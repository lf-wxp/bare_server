use crate::{collection_wrapper, document::{Action, ActionCategory}};

collection_wrapper!(Actions, Action, "action", ["value", "role"]);

collection_wrapper!(ActionCategories, ActionCategory, "action-category", ["name", "role"]);
