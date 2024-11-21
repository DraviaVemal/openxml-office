use crate::{
    files::{OfficeDocument, XmlElement, XmlSerializer},
    global_2007::traits::XmlDocumentPart,
};
use anyhow::{Error as AnyError, Result as AnyResult};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct WorkSheetPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    file_tree: Weak<RefCell<XmlElement>>,
    file_name: String,
}

impl Drop for WorkSheetPart {
    fn drop(&mut self) {
        if let Some(xml_tree) = self.office_document.upgrade() {
            let _ = xml_tree
                .try_borrow_mut()
                .unwrap()
                .close_xml_tree(&self.file_name);
        }
    }
}

impl XmlDocumentPart for WorkSheetPart {
    /// Create New object for the group
    fn new(
        office_document: &Rc<RefCell<OfficeDocument>>,
        sheet_name: Option<String>,
    ) -> AnyResult<Self, AnyError> {
        let mut file_name: String = "xl/worksheets/sheet1.xml".to_string();
        if let Some(sheet_name) = sheet_name {
            file_name = sheet_name.to_string();
        }
        let file_tree = Self::get_xml_tree(&office_document, &file_name)?;
        return Ok(Self {
            office_document: Rc::downgrade(office_document),
            file_tree,
            file_name,
        });
    }

    fn flush(self) {}

    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<XmlElement, AnyError> {
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"></worksheet>"#;
        XmlSerializer::xml_str_to_xml_tree(template_core_properties.as_bytes().to_vec())
    }
}

impl WorkSheetPart {}
