use crate::element_dictionary::EXCEL_TYPE_COLLECTION;
use crate::global_2007::parts::RelationsPart;
use crate::global_2007::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing};
use crate::log_elapsed;
use crate::{
    files::{OfficeDocument, XmlDocument},
    global_2007::traits::XmlDocumentPart,
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Weak,
};

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
                            if let Some(root) = xml_doc_mut.get_root_mut() {
                                if let Some(attributes) = root.get_attribute_mut() {
                                    attributes.insert(
                                        "count".to_string(),
                                        self.share_string_collection.len().to_string(),
                                    );
                                    attributes.insert(
                                        "uniqueCount".to_string(),
                                        self.share_string_collection
                                            .iter()
                                            .map(|s| s.to_string())
                                            .collect::<HashSet<String>>()
                                            .len()
                                            .to_string(),
                                    );
                                }
                            }
                            for string in self.share_string_collection.to_owned() {
                                let parent_id = xml_doc_mut
                                    .append_child_mut("si", None)
                                    .context("Failed to Add Child")?
                                    .get_id();
                                xml_doc_mut
                                    .append_child_mut("t", Some(&parent_id))
                                    .context("Creating Share String Child Failed")?
                                    .set_value_mut(string);
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
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(
            "xmlns".to_string(),
            EXCEL_TYPE_COLLECTION
                .get("share_string")
                .unwrap()
                .schemas_namespace
                .to_string(),
        );
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_mut("sst")
            .context("Create Root Element Failed")?
            .set_attribute_mut(attributes)
            .context("Set Attribute Failed")?;
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
        let share_string_collection = Self::load_content_to_database(&mut xml_document)
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

    fn load_content_to_database(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<String>, AnyError> {
        let mut share_string_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("xml doc borrow failed")?;
            if let Some(elements) = xml_doc_mut.pop_elements_by_tag_mut("si", None) {
                for element in elements {
                    if let Some((child_id, _)) = element.pop_child_mut() {
                        if let Some(text_element) = xml_doc_mut.pop_element_mut(&child_id) {
                            let value = text_element.get_value().clone().unwrap_or("".to_string());
                            share_string_collection.push(value);
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
