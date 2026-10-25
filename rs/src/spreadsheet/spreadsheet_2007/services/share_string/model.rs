use std::{cell::RefCell, rc::Weak};

use draviavemal_xml_rs::XmlDocument;

use crate::{files::OfficeDocument, global_2007::parts::RelationsPart};

#[derive(Debug)]
pub(crate) struct ShareStringPart {
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) parent_relationship_part: Weak<RefCell<RelationsPart>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) share_string_collection: Vec<String>,
    pub(crate) file_path: String,
}
