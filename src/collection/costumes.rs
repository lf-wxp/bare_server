use crate::{collection_wrapper, document::{Costume, CostumeCategory}};

collection_wrapper!(Costumes, Costume, "costume", ["value", "role"]);

collection_wrapper!(CostumeCategories, CostumeCategory, "costume-category", ["name", "role"]);
