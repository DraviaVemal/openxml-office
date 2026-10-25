use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use draviavemal_xml_rs::XmlDocument;

use crate::{
    files::OfficeDocument,
    global_2007::parts::{DrawingPartGlobal, RelationsPart},
    spreadsheet_2007::{models::DrawingAnchor, services::CommonServices},
};

#[derive(Debug)]
pub(crate) struct DrawingPart {
    pub(crate) drawing_global: DrawingPartGlobal,
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) common_service: Weak<RefCell<CommonServices>>,
    pub(crate) worksheet_relationship_part: Weak<RefCell<RelationsPart>>,
    pub(crate) drawing_relationship_part: Rc<RefCell<RelationsPart>>,
    pub(crate) anchor_collection: Option<VecDeque<DrawingAnchor>>,
    pub(crate) is_new: bool,
    pub(crate) file_path: String,
}
