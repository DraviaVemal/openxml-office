mod edit;
mod internal;
mod read;
mod traits;

pub(crate) mod package;
#[path = "struct.rs"]
mod struct_items;

pub(crate) use struct_items::{RelationsPart, RelationshipEntry};
