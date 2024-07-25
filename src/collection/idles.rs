use crate::{collection_wrapper, document::{Idle, IdleMapping, IdleTransition}};

collection_wrapper!(Idles, Idle, "idle", ["value", "role"]);

collection_wrapper!(IdleTransitions, IdleTransition, "idle-transition", ["value", "role"]);

collection_wrapper!(IdleMappings, IdleMapping, "idle-mapping", ["role", "start", "end", "transition"]);
