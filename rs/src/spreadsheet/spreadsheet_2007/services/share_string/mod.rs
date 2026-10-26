use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    global_2007::files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{
            XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush,
            XmlDocumentPartInitializing,
        },
    },
    log_elapsed,
    namespaces::SPREADSHEET_NS,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlDocument, XmlElementContentType};
use std::{cell::RefCell, collections::HashSet, rc::Weak};

mod deserialize;
mod edit;
mod internal;
mod read;
mod serialize;

#[path = "enum.rs"]
pub(crate) mod enum_items;
#[path = "struct.rs"]
pub(crate) mod struct_items;

pub(crate) use enum_items::*;
pub(crate) use struct_items::*;
