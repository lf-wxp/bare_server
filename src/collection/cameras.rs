use crate::{collection_wrapper, document::Camera};

collection_wrapper!(Cameras, Camera, "camera", ["value_field", "speed_field"]);
