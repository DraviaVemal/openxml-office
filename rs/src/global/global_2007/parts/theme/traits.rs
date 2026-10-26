use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    global::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing},
    global_2007::parts::theme::ThemePart,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument};

impl_xml_part_lifecycle!(ThemePart);

impl XmlDocumentPartClose for ThemePart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        if let Some(xml_tree) = self.office_document.upgrade() {
            xml_tree
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull XML Handle")?
                .close_xml_document(&self.file_path)?;
        }
        Ok(())
    }
}

impl XmlDocumentPartInitializing for ThemePart {
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = COMMON_TYPE_COLLECTION
            .get("theme")
            .context("Failed to read Common type collection")?;
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(include_str!("theme.xml").as_bytes().to_vec())
                .context("draviavemal-openxml_office::Initializing Theme Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}
