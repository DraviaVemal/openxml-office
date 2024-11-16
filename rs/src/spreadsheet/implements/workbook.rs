use crate::structs::workbook::Workbook;
use anyhow::{Error as AnyError, Result as AnyResult};
use openxmloffice_global::xml_file::XmlElement;
use openxmloffice_xml::OpenXmlFile;
use std::{cell::RefCell, rc::Rc};

impl Drop for Workbook {
    fn drop(&mut self) {
        self.xml_fs
            .borrow()
            .add_update_xml_content(&self.file_name, &self.file_content);
    }
}

impl XmlElement for Workbook {
    /// Create workbook
    fn new(xml_fs: &Rc<RefCell<OpenXmlFile>>, _: Option<&str>) -> AnyResult<Self, AnyError> {
        let file_name = "xl/workbook.xml".to_string();
        let file_content = Self::get_content_xml(&xml_fs, &file_name)?;
        return Ok(Self {
            xml_fs: Rc::clone(&xml_fs),
            file_content,
            file_name,
        });
    }

    fn flush(self) {}

    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> Vec<u8> {
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"></workbook>"#;
        return template_core_properties.as_bytes().to_vec();
    }
}
