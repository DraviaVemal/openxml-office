use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use draviavemal_xml_rs::XmlDocument;

use crate::{
    files::OfficeDocument,
    global_2007::parts::{RelationsPart, ThemePart},
    spreadsheet_2007::services::CommonServices,
};

#[derive(Debug)]
pub(crate) struct WorkbookPart {
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) file_path: String,
    pub(crate) common_service: Rc<RefCell<CommonServices>>,
    pub(crate) workbook_relationship_part: Rc<RefCell<RelationsPart>>,
    pub(crate) theme_part: ThemePart,
    /// This contain the (display sheet name, relationId, hide sheet)
    pub(crate) sheet_collection: Rc<RefCell<Vec<(String, String, bool)>>>,
    pub(crate) workbook_view: Option<WorkbookView>,
}

#[derive(Debug)]
pub(crate) struct WorkbookView {
    pub(crate) active_tab: Option<String>,
    pub(crate) first_sheet: Option<String>,
    pub(crate) visibility: Option<String>,
    pub(crate) minimize: bool,
    pub(crate) hide_horizontal_scroll: bool,
    pub(crate) hide_vertical_scroll: bool,
    pub(crate) hide_sheet_tab: bool,
    pub(crate) sheet_tab_ratio: Option<i16>,
    pub(crate) auto_filter_date_grouping: Option<bool>,
}

impl Default for WorkbookView {
    fn default() -> Self {
        WorkbookView {
            active_tab: None,
            first_sheet: None,
            minimize: false,
            auto_filter_date_grouping: None,
            hide_sheet_tab: false,
            hide_vertical_scroll: false,
            hide_horizontal_scroll: false,
            sheet_tab_ratio: None,
            visibility: None,
        }
    }
}
