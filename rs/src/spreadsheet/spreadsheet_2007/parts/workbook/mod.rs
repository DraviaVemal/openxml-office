use crate::{
    converters::ConverterUtil,
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::{RelationsPart, ThemePart},
        service::MediaFiles,
        traits::{
            XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush,
            XmlDocumentPartInitializing,
        },
    },
    log_elapsed,
    namespaces::{RELATIONSHIP_OFFICE_DOC_NS, SPREADSHEET_NS},
    order_dictionary::EXCEL_ORDER_COLLECTION,
    spreadsheet_2007::{
        excel::WorkbookSettings,
        models::{CellStyleSetting, StyleId},
        parts::WorkSheet,
        services::{CalculationChainPart, CommonServices, ShareStringPart, StylePart},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{
    NamespaceDeclaration, NodeId, XmlAttribute, XmlDocument, XmlElementContentType,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

mod deserialize;
mod edit;
mod internal;
mod model;
mod read;
mod serialize;

pub(crate) use model::WorkbookPart;
pub(crate) use model::WorkbookView;
