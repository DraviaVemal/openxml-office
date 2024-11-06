use crate::{xml_file::XmlElement, ContentTypesPart};
use openxmloffice_xml::OpenXmlFile;
use std::{cell::RefCell, rc::Rc};

impl Drop for ContentTypesPart {
    fn drop(&mut self) {
        self.xml_fs
            .borrow()
            .add_update_xml_content(&self.file_name, &self.file_content)
            .expect("Content type part Save Failed");
    }
}

impl XmlElement for ContentTypesPart {
    fn new(xml_fs: &Rc<RefCell<OpenXmlFile>>, _: Option<&str>) -> ContentTypesPart {
        let file_name = "[Content_Types].xml".to_string();
        Self {
            xml_fs: Rc::clone(xml_fs),
            file_content: Self::get_content_xml(&xml_fs, &file_name),
            file_name,
        }
    }

    fn flush(self) {}

    /// Read and load workbook xml to work with
    fn get_content_xml(xml_fs: &Rc<RefCell<OpenXmlFile>>, file_name: &str) -> Vec<u8> {
        let results = xml_fs.borrow().get_xml_content(file_name);
        if let Some(results) = results {
            return results;
        } else {
            return Self::initialize_content_xml();
        }
    }

    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> Vec<u8> {
        let template_core_properties = include_str!("content_types.xml");
        return template_core_properties.as_bytes().to_vec();
    }
}

impl ContentTypesPart {}
