use std::{cell::RefCell, rc::Weak};

use draviavemal_xml_rs::XmlDocument;

use crate::{files::OfficeDocument, global_2007::parts::RelationsPart};

#[derive(Debug)]
pub(crate) struct CalculationChain {
    pub(crate) cell_ref: String,
    pub(crate) sheet_id: u32,
    pub(crate) level_calcualtion: Option<bool>,
    pub(crate) formula_type: Option<String>,
    pub(crate) share_formula: Option<bool>,
    pub(crate) array_formula: Option<bool>,
}

#[derive(Debug)]
pub(crate) struct CalculationChainPart {
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) parent_relationship_part: Weak<RefCell<RelationsPart>>,
    pub(crate) calculation_collection: Vec<CalculationChain>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) file_path: String,
}
