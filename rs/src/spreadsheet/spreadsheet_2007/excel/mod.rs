use crate::{
    global_2007::files::OfficeDocument,
    global_2007::{
        parts::{CorePropertiesPart, RelationsPart},
        traits::{XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush},
    },
    log_elapsed,
    spreadsheet_2007::{
        CellStyleSetting, StyleId,
        parts::{WorkSheet, WorkbookPart},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, rc::Rc};

mod edit;
mod internal;
mod read;

#[path = "enum.rs"]
pub(crate) mod enum_items;
#[path = "struct.rs"]
pub(crate) mod struct_items;

pub use struct_items::{Excel, ExcelPropertiesModel, WorkbookSettings};

#[cfg(feature = "ffi")]
pub mod ffi;

pub use edit::*;
pub use read::*;
