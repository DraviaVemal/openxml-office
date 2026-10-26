use std::{cell::RefCell, rc::Weak};

use crate::global_2007::{files::OfficeDocument, traits::RelationshipRules};
use draviavemal_xml_rs::XmlDocument;

/// Holds ID, Target, Type, Target Mode
pub(crate) type RelationshipEntry = (String, String, String, Option<String>);

#[derive(Debug)]
pub(crate) struct RelationsPart {
    pub(crate) relationships: Vec<RelationshipEntry>,
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) file_path: String,
    pub(crate) rules: Box<dyn RelationshipRules>,
}
