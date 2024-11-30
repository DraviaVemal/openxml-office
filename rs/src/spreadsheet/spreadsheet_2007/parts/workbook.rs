use crate::{
    files::{OfficeDocument, XmlDocument, XmlSerializer},
    global_2007::{
        parts::{RelationsPart, ThemePart},
        traits::XmlDocumentPart,
    },
    spreadsheet_2007::{
        parts::WorkSheet,
        services::{CalculationChain, CommonServices, ShareString, Style},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::{
    cell::RefCell,
    path::Path,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct WorkbookPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    file_tree: Weak<RefCell<XmlDocument>>,
    file_path: String,
    common_service: Rc<RefCell<CommonServices>>,
    relations_part: RelationsPart,
    theme_part: ThemePart,
}

impl Drop for WorkbookPart {
    fn drop(&mut self) {
        if let Some(xml_tree) = self.office_document.upgrade() {
            let _ = xml_tree
                .try_borrow_mut()
                .unwrap()
                .close_xml_document(&self.file_path);
        }
    }
}
/// ######################### Train implementation of XML Part - Only accessible within crate ##############
impl XmlDocumentPart for WorkbookPart {
    /// Create workbook
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        file_path_option: Option<String>,
    ) -> AnyResult<Self, AnyError> {
        let mut file_path = "xl/workbook.xml".to_string();
        if let Some(path) = file_path_option {
            file_path = path.to_string()
        }
        let file_tree = Self::get_xml_document(&office_document, &file_path)?;
        let relation_path = Path::new(&file_path)
            .parent()
            .unwrap_or(Path::new(""))
            .display()
            .to_string();
        let relations_part =
            RelationsPart::new(office_document.clone(), Some(relation_path.clone()))
                .context("Creating Relation ship part for workbook failed.")?;
        let theme_part_path: Option<String> = relations_part
            .get_relationship_target_by_type(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
            )
            .context("Parsing Theme part path failed")?;
        let share_string_path: Option<String> = relations_part
            .get_relationship_target_by_type(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
            )
            .context("Parsing Theme part path failed")?;
        let calculation_chain_path: Option<String> = relations_part
            .get_relationship_target_by_type(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
            )
            .context("Parsing Theme part path failed")?;
        let style_path: Option<String> = relations_part
            .get_relationship_target_by_type(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
            )
            .context("Parsing Theme part path failed")?;
        let theme_part = ThemePart::new(
            office_document.clone(),
            Some(theme_part_path.unwrap_or(format!("{}/theme/theme1.xml", relation_path))),
        )
        .context("Creating Theme part for workbook failed")?;
        let share_string = ShareString::new(
            office_document.clone(),
            Some(share_string_path.unwrap_or(format!("{}/sharedStrings.xml", relation_path))),
        )
        .context("Share String Service Object Creation Failure")?;
        let calculation_chain = CalculationChain::new(
            office_document.clone(),
            Some(calculation_chain_path.unwrap_or(format!("{}/calcChain.xml", relation_path))),
        )
        .context("Calculation Chain Service Object Creation Failure")?;
        let style = Style::new(
            office_document.clone(),
            Some(style_path.unwrap_or(format!("{}/styles.xml", relation_path))),
        )
        .context("Style Service Object Creation Failure")?;
        let common_service = Rc::new(RefCell::new(CommonServices::new(
            calculation_chain,
            share_string,
            style,
        )));
        return Ok(Self {
            office_document,
            file_tree,
            file_path,
            common_service,
            relations_part,
            theme_part,
        });
    }

    fn flush(self) {}

    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<XmlDocument, AnyError> {
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"></workbook>"#;
        XmlSerializer::vec_to_xml_doc_tree(template_core_properties.as_bytes().to_vec())
    }
}

// ############################# Internal Function ######################################
impl WorkbookPart {
    fn load_share_string_database() {}

    pub(crate) fn add_sheet(
        &mut self,
        file_name: Option<String>,
    ) -> AnyResult<WorkSheet, AnyError> {
        Ok(WorkSheet::new(
            self.office_document.clone(),
            Rc::downgrade(&self.common_service),
            file_name,
        )
        .context("Worksheet Creation Failed")?)
    }

    pub(crate) fn set_active_sheet(&mut self, sheet_name: &str) {}

    pub(crate) fn get_sheet_name(&self, sheet_name: &str) {}

    pub(crate) fn rename_sheet_name(&self, sheet_name: &str, new_sheet_name: &str) {}
}
