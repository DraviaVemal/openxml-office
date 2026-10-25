use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
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
mod model;
mod read;
mod serialize;

pub(crate) use model::ShareStringPart;
