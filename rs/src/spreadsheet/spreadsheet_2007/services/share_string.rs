use crate::element_dictionary::EXCEL_TYPE_COLLECTION;
use crate::global_2007::parts::RelationsPart;
use crate::global_2007::traits::{
    XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing,
};
use crate::log_elapsed;
use crate::{files::OfficeDocument, global_2007::traits::XmlDocumentPart};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlAttribute, XmlDocument, XmlElementContentType};
use std::{cell::RefCell, collections::HashSet, rc::Weak};

#[derive(Debug)]
pub struct ShareStringPart {
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
                            let mut xml_doc_mut = xml_document
                                .try_borrow_mut()
                                .context("Failed to Pull Doc Reference")?;
                            // Update count & uniqueCount in root
                            let root_id = xml_doc_mut.get_root_id();
                            let root_element = xml_doc_mut
                                .get_element_mut(root_id)
                                .context("Failed to get root element")?;
                            root_element.add_attribute_mut(XmlAttribute::new(
                                "count".to_string(),
                                self.share_string_collection.len().to_string(),
                            ));
                            root_element.add_attribute_mut(XmlAttribute::new(
                                "uniqueCount".to_string(),
                                self.share_string_collection
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect::<HashSet<String>>()
                                    .len()
                                    .to_string(),
                            ));
                            for share_string_value in self.share_string_collection.to_owned() {
                                let parent_id = xml_doc_mut
                                    .append_child_element_mut(root_id, "si", None)
                                    .context("Failed to Add Child")?;
                                let t_id = xml_doc_mut
                                    .append_child_element_mut(parent_id, "t", None)
                                    .context("Creating Share String Child Failed")?;
                                xml_doc_mut
                                    .get_element_mut(t_id)
                                    .context("Failed to get t element")?
                                    .add_text_mut(&share_string_value);
                            }
                        }
                        office_doc_ref
                            .try_borrow_mut()
                            .context("Failed To pull XML Handle")?
                            .close_xml_document(&self.file_path)
                            .context("Failed to close XML Document Share String")?;
                    } else {
                        if let Some(relationship_part) = self.parent_relationship_part.upgrade() {
                            relationship_part
                                .try_borrow_mut()
                                .context(
                                    "Failed To pull parent relation ship part of Share String",
                                )?
                                .delete_relationship_mut(&self.file_path);
                            office_doc_ref
                                .try_borrow_mut()
                                .context("Failed To pull XML Handle")?
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
        let content = EXCEL_TYPE_COLLECTION.get("share_string").unwrap();
        let attributes = vec![XmlAttribute::new(
            "xmlns".to_string(),
            EXCEL_TYPE_COLLECTION
                .get("share_string")
                .unwrap()
                .schemas_namespace
                .to_string(),
        )];
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_element_mut("sst", Some(attributes))
            .context("Create Root Element Failed")?;
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
    ) -> AnyResult<Self, AnyError> {
        let file_name = Self::get_share_string_file_name(&parent_relationship_part)
            .context("Failed to pull share string file name")?
            .to_string();
        let mut xml_document = Self::get_xml_document(&office_document, &file_name)?;
        let share_string_collection = Self::deserialize_share_string(&mut xml_document)
            .context("Load Share String To DB Failed")?;
        Ok(Self {
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
        let share_string_content = EXCEL_TYPE_COLLECTION.get("share_string").unwrap();
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &share_string_content.schemas_type,
                    share_string_content,
                    None,
                    None,
                )
                .context("Pull Path From Existing File Failed")?)
        } else {
            Err(anyhow!("Failed to upgrade relation part"))
        }
    }

    fn deserialize_share_string(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<String>, AnyError> {
        let mut share_string_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let xml_document = xml_document.try_borrow().context("xml doc borrow failed")?;
            let root_id = xml_document.get_root_id();
            if let Some(si_ids) = xml_document
                .find_all_child(root_id, "si")
                .context("Failed to get si element group")?
            {
                for si_id in si_ids {
                    let si_element = xml_document
                        .get_element(si_id)
                        .context("Failed to pull si element")?;
                    if let Some(contents) = si_element.get_child_contents() {
                        for element in contents {
                            match element {
                                XmlElementContentType::Element((id, _, _)) => {
                                    let t_element = xml_document
                                        .get_element(*id)
                                        .context("Failed to pull t element")?;
                                    if let Some(child_contents) = t_element.get_child_contents() {
                                        let value = child_contents
                                            .iter()
                                            .filter_map(|item| {
                                                if let XmlElementContentType::Text(text) = item {
                                                    Some(text.to_owned())
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect::<Vec<String>>()
                                            .join(" ");
                                        share_string_collection.push(value);
                                    }
                                }
                                _ => {
                                    return Err(AnyError::msg(
                                        "Unhandled tag in Share string setup",
                                    ));
                                }
                            }
                        }
                    }
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
                    .context("Failed to parse Share String Id")?,
            )
            .context("Failed to find the value in the Vec")?;
        Ok(actual_string.to_string())
    }
}
