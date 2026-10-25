use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Weak,
};

use draviavemal_xml_rs::XmlDocument;

use crate::{
    files::OfficeDocument,
    spreadsheet_2007::models::{BorderStyle, CellXfs, FillStyle, FontStyle, NumberFormat},
};

#[derive(Debug)]
pub(crate) struct StylePart {
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) file_path: String,
    pub(crate) cache_id: HashMap<u64, u32>,
    pub(crate) cache_order: VecDeque<u64>,
    pub(crate) cache_capacity: u8,
    pub(crate) number_format_collection: Vec<(u64, NumberFormat)>,
    pub(crate) font_collection: Vec<(u64, FontStyle)>,
    pub(crate) fill_collection: Vec<(u64, FillStyle)>,
    pub(crate) border_collection: Vec<(u64, BorderStyle)>,
    pub(crate) cell_style_xfs_collection: Vec<(u64, CellXfs)>,
    pub(crate) cell_xfs_collection: Vec<(u64, CellXfs)>,
}
