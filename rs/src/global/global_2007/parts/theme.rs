use crate::element_dictionary::COMMON_TYPE_COLLECTION;
use crate::global_2007::traits::{XmlDocumentPartClose, XmlDocumentPartFlush};
use crate::{
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{XmlDocumentPart, XmlDocumentPartInitializing},
    },
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument};
use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
pub struct ThemePart {
    office_document: Weak<RefCell<OfficeDocument>>,
    _xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
}

impl Drop for ThemePart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for ThemePart {}

impl XmlDocumentPartClose for ThemePart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        if let Some(xml_tree) = self.office_document.upgrade() {
            xml_tree
                .try_borrow_mut()
                .context("Failed to pull XML Handle")?
                .close_xml_document(&self.file_path)?;
        }
        Ok(())
    }
}

impl XmlDocumentPartInitializing for ThemePart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = COMMON_TYPE_COLLECTION.get("theme").unwrap();
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(include_str!("theme.xml").as_bytes().to_vec())
                .context("Initializing Theme Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

/// ######################### Train implementation of XML Part - Only accessible within crate ##############
impl XmlDocumentPart for ThemePart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<Self, AnyError> {
        let file_name = Self::get_theme_file_name(&parent_relationship_part)
            .context("Failed to pull theme file name")?
            .to_string();
        let xml_document = Self::get_xml_document(&office_document, &file_name)?;
        Ok(Self {
            office_document,
            _xml_document: xml_document,
            file_path: file_name.to_string(),
        })
    }
}

impl ThemePart {
    fn get_theme_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let theme_content = COMMON_TYPE_COLLECTION.get("theme").unwrap();
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &theme_content.schemas_type,
                    theme_content,
                    Some(format!("xl/{}", theme_content.default_path)),
                    None,
                )
                .context("Pull Path From Existing File Failed")?)
        } else {
            Err(anyhow!("Failed to upgrade relation part"))
        }
    }
}
