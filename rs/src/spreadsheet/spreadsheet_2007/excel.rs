use crate::{
    files::OfficeDocument,
    global_2007::{
        parts::{CorePropertiesPart, RelationsPart},
        traits::{XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush},
    },
    log_elapsed,
    spreadsheet_2007::{
        models::{StyleId, CellStyleSetting},
        parts::{WorkSheet, WorkbookPart},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Excel {
    office_document: Rc<RefCell<OfficeDocument>>,
    root_relations: Rc<RefCell<RelationsPart>>,
    core_properties: CorePropertiesPart,
    workbook: WorkbookPart,
}

#[derive(Debug)]
pub struct ExcelPropertiesModel {
    pub is_editable: bool,
}

impl Default for ExcelPropertiesModel {
    fn default() -> ExcelPropertiesModel {
        ExcelPropertiesModel { is_editable: true }
    }
}

// ##################################### Feature Function ################################
impl Excel {
    /// Create new or clone source file to start working on Excel
    pub fn new(
        file_name: Option<String>,
        _excel_setting: ExcelPropertiesModel,
    ) -> AnyResult<Self, AnyError> {
        log_elapsed!(
            || {
                let office_document = Rc::new(RefCell::new(
                    OfficeDocument::new(file_name.clone())
                        .context("Creating Office Document Struct Failed")?,
                ));
                let root_relations = Rc::new(RefCell::new(
                    RelationsPart::new(Rc::downgrade(&office_document), "_rels/.rels")
                        .context("Initialize Root Relation Part failed")?,
                ));
                // Load relevant parts from root relations part
                let core_properties = CorePropertiesPart::new(
                    Rc::downgrade(&office_document),
                    Rc::downgrade(&root_relations),
                )
                .context("Creating Core Property Part Failed.")?;
                let workbook = WorkbookPart::new(
                    Rc::downgrade(&office_document),
                    Rc::downgrade(&root_relations),
                )
                .context("Creating Workbook part Failed")?;
                let excel = Self {
                    office_document,
                    root_relations,
                    core_properties,
                    workbook,
                };
                Ok(excel)
            },
            if file_name.is_some() {
                "New Excel Edit File Object"
            } else {
                "New Excel Blank File Object"
            }
        )
    }

    /// Add sheet to the current excel
    pub fn add_sheet_mut(&mut self, sheet_name: Option<String>) -> AnyResult<WorkSheet, AnyError> {
        log_elapsed!(self.get_workbook_mut().add_sheet_mut(sheet_name))
    }

    /// Add sheet to the current excel
    pub fn rename_sheet_name_mut(
        &mut self,
        old_sheet_name: String,
        new_sheet_name: String,
    ) -> AnyResult<(), AnyError> {
        log_elapsed!(self
            .get_workbook_mut()
            .rename_sheet_name_mut(&old_sheet_name, &new_sheet_name))
    }

    pub fn set_active_sheet_mut(&mut self, sheet_name: String) -> AnyResult<(), AnyError> {
        self.get_workbook_mut().set_active_sheet_mut(&sheet_name)
    }

    pub fn set_visibility_mut(&mut self, is_visible: bool) -> AnyResult<(), AnyError> {
        self.get_workbook_mut().set_visibility_mut(is_visible)
    }

    pub fn minimize_workbook_mut(&mut self, is_minimized: bool) -> AnyResult<(), AnyError> {
        self.get_workbook_mut().minimize_workbook_mut(is_minimized)
    }

    pub fn hide_sheet_tabs_mut(&mut self, hide_tab: bool) -> AnyResult<(), AnyError> {
        self.get_workbook_mut().hide_sheet_tabs_mut(hide_tab)
    }

    pub fn hide_vertical_scroll_mut(
        &mut self,
        hide_vertical_scroll: bool,
    ) -> AnyResult<(), AnyError> {
        self.get_workbook_mut()
            .hide_vertical_scroll_mut(hide_vertical_scroll)
    }

    pub fn hide_horizontal_scroll_mut(
        &mut self,
        hide_horizontal_scroll: bool,
    ) -> AnyResult<(), AnyError> {
        self.get_workbook_mut()
            .hide_horizontal_scroll_mut(hide_horizontal_scroll)
    }

    pub fn hide_sheet_mut(&mut self, sheet_name: String) -> AnyResult<(), AnyError> {
        log_elapsed!(self.get_workbook_mut().hide_sheet_mut(&sheet_name))
    }

    /// Get Worksheet handle by sheet name
    pub fn get_worksheet_mut(&mut self, sheet_name: String) -> AnyResult<WorkSheet, AnyError> {
        log_elapsed!(self.get_workbook_mut().get_worksheet_mut(&sheet_name))
    }

    /// Return Style Id for the said combination
    pub fn get_style_id_mut(
        &mut self,
        style_setting: CellStyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        log_elapsed!(self.get_workbook_mut().get_style_id_mut(style_setting))
    }

    /// Save/Replace the current file into target destination
    pub fn save_as(mut self, file_name: &str) -> AnyResult<String, AnyError> {
        if self
            .list_sheet_names()
            .context("Failed to get Sheet Name List")?
            .len()
            == 0
        {
            self.add_sheet_mut(None)
                .context("Failed To Add Default Sheet to excel")?;
        }
        log_elapsed!(
            || {
                self.workbook.flush()?;
                self.core_properties.flush()?;
                self.root_relations
                    .try_borrow_mut()
                    .context("Failed To Pull Relation Handle")?
                    .close_document()?;
                self.office_document
                    .try_borrow_mut()
                    .context("Save Office Document handle Failed")?
                    .save_as(file_name)
                    .context("File Save Failed for the target path.")
            },
            "File Save"
        )
    }
}

// ############################# Internal Function ######################################
// ############################# Mut Function      ######################################
impl Excel {
    fn get_workbook_mut(&mut self) -> &mut WorkbookPart {
        &mut self.workbook
    }
}

// ############################# Im-Mut Function   ######################################
impl Excel {
    fn get_workbook(&self) -> &WorkbookPart {
        &self.workbook
    }
    pub fn list_sheet_names(&self) -> AnyResult<Vec<String>, AnyError> {
        self.get_workbook().list_sheet_names()
    }
}
