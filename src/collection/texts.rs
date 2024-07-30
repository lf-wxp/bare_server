use crate::{collection_wrapper, document::{Text, Bubble}};

collection_wrapper!(Texts,Text, "text", ["value"]);

collection_wrapper!(Bubbles, Bubble, "bubble", ["value"]);
