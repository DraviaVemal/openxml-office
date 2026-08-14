use crate::element_dictionary::EXCEL_TYPE_COLLECTION;
use crate::global_2007::parts::RelationsPart;
use crate::global_2007::traits::{
    XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing,
};
use crate::log_elapsed;
use crate::namespaces::SPREADSHEET_NS;
use crate::{files::OfficeDocument, global_2007::traits::XmlDocumentPart};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlDocument, XmlElementContentType};
use std::{cell::RefCell, collections::HashSet, rc::Weak};

#[derive(Debug)]
pub(crate) struct ShareStringPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    parent_relationship_part: Weak<RefCell<RelationsPart>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    share_string_collection: Vec<String>,
    file_path: String,
}

impl Drop for ShareStringPart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for ShareStringPart {}

impl XmlDocumentPartClose for ShareStringPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if let Some(office_doc_ref) = self.office_document.upgrade() {
                    if self.share_string_collection.len() > 0 {
                        if let Some(xml_document) = self.xml_document.upgrade() {
                            let mut xml_doc_mut = xml_document.try_borrow_mut().context(
                                "draviavemal-openxml_office::Failed to Pull Doc Reference",
                            )?;
                            let root_id = xml_doc_mut.get_root_id();
                            // Update count & uniqueCount in root
                            {
                                let unique_count = self
                                    .share_string_collection
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect::<HashSet<String>>()
                                    .len()
                                    .to_string();
                                let count = self.share_string_collection.len().to_string();
                                if let Ok(root) = xml_doc_mut.get_element_mut(root_id) {
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
                                let parent_id = xml_doc_mut
                                    .append_child_element_mut(root_id, "si", None)
                                    .context("draviavemal-openxml_office::Failed to Add Child")?;
                                let text_id = xml_doc_mut
                                    .append_child_element_mut(parent_id, "t", None)
                                    .context("draviavemal-openxml_office::Creating Share String Child Failed")?;
                                xml_doc_mut
                                    .get_element_mut(text_id)
                                    .context("draviavemal-openxml_office::Failed to pull text element")?
                                    .add_text_mut(&string)
                                    .context("draviavemal-openxml_office::Failed to set share string value")?;
                            }
                        }
                        office_doc_ref
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
                            office_doc_ref
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

impl XmlDocumentPart for ShareStringPart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<ShareStringPart, AnyError> {
        let file_name = ShareStringPart::get_share_string_file_name(&parent_relationship_part)
            .context("draviavemal-openxml_office::Failed to pull share string file name")?
            .to_string();
        let mut xml_document = ShareStringPart::get_xml_document(&office_document, &file_name)?;
        let share_string_collection = ShareStringPart::deserialize_share_string(&mut xml_document)
            .context("draviavemal-openxml_office::Load Share String To DB Failed")?;
        Ok(ShareStringPart {
            office_document,
            parent_relationship_part,
            xml_document,
            share_string_collection,
            file_path: file_name,
        })
    }
}

impl ShareStringPart {
    fn get_share_string_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let share_string_content = EXCEL_TYPE_COLLECTION
            .get("share_string")
            .context("Failed to read Excel type collection")?;
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &share_string_content.schemas_type,
                    share_string_content,
                    None,
                    None,
                )
                .context("draviavemal-openxml_office::Pull Path From Existing File Failed")?)
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to upgrade relation part",
            ))
        }
    }

    fn deserialize_share_string(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<String>, AnyError> {
        let mut share_string_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::xml doc borrow failed")?;
            let root_id = xml_doc_mut.get_root_id();
            if let Some(si_ids) = xml_doc_mut
                .find_all_child(root_id, "si")
                .context("draviavemal-openxml_office::Failed to find si elements")?
            {
                for si_id in si_ids {
                    let first_child = xml_doc_mut
                        .get_element(si_id)
                        .context("draviavemal-openxml_office::Failed to pull si element")?
                        .get_child_contents()
                        .as_ref()
                        .and_then(|contents| {
                            contents.iter().find_map(|content| match content {
                                XmlElementContentType::Element((id, _, _)) => Some(*id),
                                _ => None,
                            })
                        });
                    if let Some(child_id) = first_child {
                        let text_element = xml_doc_mut
                            .get_element(child_id)
                            .context("draviavemal-openxml_office::Failed to pull child element")?;
                        let value = text_element
                            .get_child_contents()
                            .as_ref()
                            .map(|contents| {
                                contents
                                    .iter()
                                    .filter_map(|content| match content {
                                        XmlElementContentType::Text(text) => Some(text.clone()),
                                        _ => None,
                                    })
                                    .collect::<String>()
                            })
                            .unwrap_or_default();
                        share_string_collection.push(value);
                    }
                    xml_doc_mut
                        .remove_element_mut(si_id)
                        .context("draviavemal-openxml_office::Failed To remove element")?;
                }
            }
        }
        Ok(share_string_collection)
    }
}

impl ShareStringPart {
    pub(crate) fn get_string_id_mut(&mut self, value: String) -> AnyResult<String, AnyError> {
        Ok(
            if let Some(position) = self
                .share_string_collection
                .iter()
                .position(|predicate| predicate == &value)
            {
                position.to_string()
            } else {
                self.share_string_collection.push(value);
                (self.share_string_collection.len() - 1).to_string()
            },
        )
    }

    pub(crate) fn get_string_id_value(&self, id: String) -> AnyResult<String, AnyError> {
        let actual_string = self
            .share_string_collection
            .get(
                id.parse::<usize>()
                    .context("draviavemal-openxml_office::Failed to parse Share String Id")?,
            )
            .context("draviavemal-openxml_office::Failed to find the value in the Vec")?;
        Ok(actual_string.to_string())
    }
}
