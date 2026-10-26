use std::{cell::RefCell, rc::Rc};

use crate::{
    global_2007::files::OfficeDocument,
    global_2007::parts::{CorePropertiesPart, RelationsPart},
    spreadsheet_2007::WorkbookPart,
};

#[derive(Debug)]
pub struct Excel {
    pub(crate) office_document: Rc<RefCell<OfficeDocument>>,
    pub(crate) root_relations: Rc<RefCell<RelationsPart>>,
    pub(crate) core_properties: CorePropertiesPart,
    pub(crate) workbook: WorkbookPart,
}

#[derive(Debug)]
pub struct ExcelPropertiesModel {
    pub is_editable: bool,
}

#[derive(Debug, Default)]
pub struct WorkbookSettings {
    pub active_sheet: Option<String>,
    pub is_visible: Option<bool>,
    pub is_minimized: Option<bool>,
    pub hide_tab: Option<bool>,
    pub hide_vertical_scroll: Option<bool>,
    pub hide_horizontal_scroll: Option<bool>,
    pub hide_sheet: Option<String>,
}
