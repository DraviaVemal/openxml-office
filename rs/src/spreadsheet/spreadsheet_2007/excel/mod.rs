use crate::{
    files::OfficeDocument,
    global_2007::{
        parts::{CorePropertiesPart, RelationsPart},
        traits::{XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush},
    },
    log_elapsed,
    spreadsheet_2007::{
        models::{CellStyleSetting, StyleId},
        parts::{WorkSheet, WorkbookPart},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, rc::Rc};

mod edit;
mod internal;
mod model;
mod read;

pub use model::{Excel, ExcelPropertiesModel, WorkbookSettings};

#[cfg(feature = "ffi")]
pub mod ffi;

pub use edit::*;
pub use read::*;
