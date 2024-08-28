use crate::{collection_wrapper, document::Material};

collection_wrapper!(Materials, Material, "material", ["value", "category"]);
