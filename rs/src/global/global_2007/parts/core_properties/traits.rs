use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    global::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing},
    global_2007::parts::core_properties::CorePropertiesPart,
    namespaces::DCTERMS_NS,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use chrono::Utc;
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument, XmlElementContentType};

impl_xml_part_lifecycle!(CorePropertiesPart);

impl XmlDocumentPartClose for CorePropertiesPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
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
