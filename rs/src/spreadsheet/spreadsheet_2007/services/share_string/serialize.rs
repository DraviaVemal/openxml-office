use std::collections::HashSet;

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlDocument};

use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    global::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing},
    log_elapsed,
    namespaces::SPREADSHEET_NS,
    spreadsheet_2007::services::ShareStringPart,
};

impl_xml_part_lifecycle!(ShareStringPart);

impl XmlDocumentPartClose for ShareStringPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if let Some(office_document_ref) = self.office_document.upgrade() {
                    if self.share_string_collection.len() > 0 {
                        if let Some(xml_document) = self.xml_document.upgrade() {
                            let mut xml_document_mut = xml_document.try_borrow_mut().context(
                                "draviavemal-openxml_office::Failed to Pull Doc Reference",
                            )?;
                            let root_id = xml_document_mut.get_root_id();
                            // Update count & uniqueCount in root
                            {
                                let unique_count = self
                                    .share_string_collection
                                    .iter()
                                    .map(|shared_string| shared_string.to_string())
                                    .collect::<HashSet<String>>()
                                    .len()
                                    .to_string();
                                let count = self.share_string_collection.len().to_string();
                                if let Ok(root) = xml_document_mut.get_element_mut(root_id) {
                                    root.remove_attribute_mut("count");
                                    root.remove_attribute_mut("uniqueCount");
                                    root.add_attribute_mut("count", &count).context(
                                        "draviavemal-openxml_office::Failed to set count attribute",
                                    )?;
                                    root.add_attribute_mut(
                                        "uniqueCount",
                                        &unique_count,
                                    )
                                    .context("draviavemal-openxml_office::Failed to set uniqueCount attribute")?;
                                }
                            }
                            for string in self.share_string_collection.to_owned() {
                                let parent_id = xml_document_mut
                                    .append_child_element_mut(root_id, "si", None)
                                    .context("draviavemal-openxml_office::Failed to Add Child")?;
                                let text_id = xml_document_mut
                                    .append_child_element_mut(parent_id, "t", None)
                                    .context("draviavemal-openxml_office::Creating Share String Child Failed")?;
                                xml_document_mut
                                    .get_element_mut(text_id)
                                    .context("draviavemal-openxml_office::Failed to pull text element")?
                                    .add_text_mut(&string)
                                    .context("draviavemal-openxml_office::Failed to set share string value")?;
                            }
                        }
                        office_document_ref
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed To pull XML Handle")?
                            .close_xml_document(&self.file_path)
                            .context("draviavemal-openxml_office::Failed to close XML Document Share String")?;
                    } else {
                        if let Some(relationship_part) = self.parent_relationship_part.upgrade() {
                            relationship_part
                                .try_borrow_mut()
                                .context(
                                    "draviavemal-openxml_office::Failed To pull parent relation ship part of Share String",
                                )?
                                .delete_relationship_mut(&self.file_path);
                            office_document_ref
                                .try_borrow_mut()
                                .context("draviavemal-openxml_office::Failed To pull XML Handle")?
                                .delete_document_mut(&self.file_path);
                        }
                    }
                }
                Ok(())
            },
            "Close Share String"
        )
    }
}

impl XmlDocumentPartInitializing for ShareStringPart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION
            .get("share_string")
            .context("Failed to read Excel type collection")?;
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_element_ns_mut(
                "sst",
                &NamespaceDeclaration {
                    default_alias: SPREADSHEET_NS.default_alias,
                    uri: SPREADSHEET_NS.uri,
                    alias_override: Some(""),
                },
                None,
            )
            .context("draviavemal-openxml_office::Create Root Element Failed")?;
        Ok((
            xml_document,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}
