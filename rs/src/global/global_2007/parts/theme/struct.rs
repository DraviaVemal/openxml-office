use crate::global_2007::files::OfficeDocument;
use draviavemal_xml_rs::XmlDocument;
use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
pub struct ThemePart {
    pub(super) office_document: Weak<RefCell<OfficeDocument>>,
    pub(super) _xml_document: Weak<RefCell<XmlDocument>>,
    pub(super) file_path: String,
}
