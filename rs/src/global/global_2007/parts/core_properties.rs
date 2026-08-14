use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{
            XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush,
            XmlDocumentPartInitializing,
        },
    },
    namespaces::DCTERMS_NS,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use chrono::Utc;
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument, XmlElementContentType};
use std::{cell::RefCell, rc::Weak};

#[derive(Debug)]
pub(crate) struct CorePropertiesPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
}

impl Drop for CorePropertiesPart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for CorePropertiesPart {}

impl XmlDocumentPartClose for CorePropertiesPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        // Update Last modified date part
        if let Some(xml_document_ref) = self.xml_document.upgrade() {
            let mut xml_document = xml_document_ref
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Pull Office document")?;
            let root_id = xml_document.get_root_id();
            if let Ok(Some(modified_id)) =
                xml_document.find_first_child_ns(root_id, "modified", &DCTERMS_NS)
            {
                xml_document
                    .clear_element_content_mut(modified_id)
                    .context("draviavemal-openxml_office::Failed to Clear Elemenent node")?;
                if let Ok(element) = xml_document.get_element_mut(modified_id) {
                    element
                        .add_text_mut(
                            &Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                        )
                        .context("draviavemal-openxml_office::Failed to Add Text to Element")?;
                }
            }
            if let Ok(Some(created_id)) =
                xml_document.find_first_child_ns(root_id, "created", &DCTERMS_NS)
            {
                let has_value = xml_document
                    .get_element(created_id)
                    .map(|element| {
                        element
                            .get_child_contents()
                            .as_ref()
                            .map_or(false, |contents| {
                                contents.iter().any(|content| {
                                    matches!(content, XmlElementContentType::Text(_))
                                })
                            })
                    })
                    .unwrap_or(false);
                if !has_value {
                    if let Ok(element) = xml_document.get_element_mut(created_id) {
                        element
                            .add_text_mut(
                                &Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                            )
                            .context("draviavemal-openxml_office::Failed to Add Text to Element")?;
                    }
                }
            }
        }
        // Update the current state to DB before dropping the object
        if let Some(xml_tree) = self.office_document.upgrade() {
            xml_tree
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Pull XML Handle")?
                .close_xml_document(&self.file_path)?;
        }
        Ok(())
    }
}

impl XmlDocumentPartInitializing for CorePropertiesPart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = COMMON_TYPE_COLLECTION
            .get("docProps_core")
            .context("Failed to read Common type collection")?;
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(
                include_str!("core_properties.xml").as_bytes().to_vec(),
            )
            .context("draviavemal-openxml_office::Initializing Core Property Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

/// ######################### Train implementation of XML Part - Only accessible within crate ##############
impl XmlDocumentPart for CorePropertiesPart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<CorePropertiesPart, AnyError> {
        let file_name =
            CorePropertiesPart::get_core_properties_file_name(&parent_relationship_part)
                .context("draviavemal-openxml_office::Failed to pull Core Property file name")?
                .to_string();
        let xml_document = CorePropertiesPart::get_xml_document(&office_document, &file_name)?;
        Ok(CorePropertiesPart {
            office_document,
            xml_document,
            file_path: file_name,
        })
    }
}

impl CorePropertiesPart {
    fn get_core_properties_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let relationship_content = COMMON_TYPE_COLLECTION
            .get("docProps_core")
            .context("Failed to read Common type collection")?;
        if let Some(relations_part) = relations_part.upgrade() {
            relations_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &relationship_content.schemas_type,
                    relationship_content,
                    None,
                    None,
                )
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to upgrade relation part",
            ))
        }
    }
}
