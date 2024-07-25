use crate::{collection_wrapper, document::Hairdo};

collection_wrapper!(Hairdos, Hairdo, "hairdo", ["value", "role"]);
