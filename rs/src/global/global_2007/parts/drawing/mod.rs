mod part;

#[path = "enum.rs"]
pub(crate) mod enum_items;
#[path = "struct.rs"]
pub(crate) mod struct_items;

pub use enum_items::*;
pub(crate) use part::DrawingPartGlobal;
pub use struct_items::*;
