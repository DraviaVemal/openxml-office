use crate::{
    element_dictionary::{Content, COMMON_TYPE_COLLECTION},
    files::OfficeDocument,
    global_2007::traits::{
        XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing,
    },
    utils,
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlAttribute, XmlDocument};
use std::{cell::RefCell, path::Path, rc::Weak};

#[derive(Debug)]
pub(crate) struct RelationsPart {
    /// Holds ID, Target, Type, Target Mode
    relationships: Vec<(String, String, String, Option<String>)>,
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
}

impl Drop for RelationsPart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for RelationsPart {}

impl XmlDocumentPartClose for RelationsPart {
    /// Close the Current Relation Document
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        if let Some(xml_document) = self.office_document.upgrade() {
            if self
                .save_relationship_to_doc()
                .context("Failed to Insert relationship to Relationships")?
            {
                // Remove Links that are not valid
                xml_document
                    .try_borrow_mut()
                    .context("Failed to Pull Open XML Relations Handle")?
                    .close_xml_document(&self.file_path)?;
            } else {
                xml_document
                    .try_borrow_mut()
                    .context("Failed to Pull Open XML Relations Handle")?
                    .delete_document_mut(&self.file_path);
            }
        }
        Ok(())
    }
}

impl XmlDocumentPartInitializing for RelationsPart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let relationship_content = COMMON_TYPE_COLLECTION.get("rels").unwrap();
        let mut attributes = Vec::new();
        attributes.push(XmlAttribute::new(
            "xmlns".to_string(),
            relationship_content.schemas_namespace.to_string(),
        ));
        let mut xml_document = XmlDocument::new();
        xml_document
            .create_root_element_mut("Relationships", Some(attributes))
            .context("Create XML Root Element Failed")?;
        Ok((
            xml_document,
            None,
            relationship_content.extension.to_string(),
            relationship_content.extension_type.to_string(),
        ))
    }
}

/// ######################### Train implementation of XML Part - Only accessible within crate ##############
impl RelationsPart {
    pub(crate) fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        file_name: &str,
    ) -> AnyResult<Self, AnyError> {
        let mut xml_document = Self::get_xml_document(&office_document, &file_name)?;
        let relationships =
            Self::load_relations(&mut xml_document).context("Failed to decode Relations")?;
        Ok(Self {
            office_document,
            xml_document,
            relationships,
            file_path: file_name.to_string(),
        })
    }
}

/// ####################### Im-Mut Access Functions ####################
impl RelationsPart {
    pub(crate) fn load_relations(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<(String, String, String, Option<String>)>, AnyError> {
        let mut relationships = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("Failed for get XML Handle")?;
            let root_id = xml_doc_mut.get_root_id();
            if let Some(relationship_ids) = xml_doc_mut
                .find_all_child(root_id, "Relationship")
                .context("Failed to find Relationship elements")?
            {
                for relationship_id in relationship_ids {
                    let relationship_element = xml_doc_mut
                        .get_element(relationship_id)
                        .context("Failed! Relationship element missing")?;
                    relationships.push((
                        relationship_element
                            .get_attribute("Id")
                            .context("Failed. Id in relationship Not Fount!")?
                            .get_value()
                            .to_string(),
                        relationship_element
                            .get_attribute("Target")
                            .context("Failed. Target in relationship Not Fount!")?
                            .get_value()
                            .to_string(),
                        relationship_element
                            .get_attribute("Type")
                            .context("Failed. Type in relationship Not Fount!")?
                            .get_value()
                            .to_string(),
                        relationship_element
                            .get_attribute("TargetMode")
                            .map(|attribute| attribute.get_value().to_string()),
                    ));
                }
            }
        }
        Ok(relationships)
    }

    pub(crate) fn get_relative_path(&self) -> AnyResult<String, AnyError> {
        let rels_position = self
            .file_path
            .find("_rels")
            .context("Failed to string Prefix path from relation")?;
        if rels_position > 0 {
            Ok(format!("{}/", &self.file_path[..rels_position - 1]))
        } else {
            Ok("".to_string())
        }
    }

    pub(crate) fn get_target_path_by_id(
        &self,
        relationship_id: &str,
    ) -> AnyResult<Option<String>, AnyError> {
        if let Some(record) = self
            .relationships
            .iter()
            .find(|item| item.0 == relationship_id)
        {
            let file_path = record.1.clone();
            let relative_path = self
                .get_relative_path()
                .context("Get Relative Path for Part File")?;
            if file_path.starts_with('/') {
                Ok(Some(file_path.strip_prefix('/').unwrap().to_string()))
            } else {
                Ok(Some(format!("{}{}", relative_path, file_path)))
            }
        } else {
            Ok(None)
        }
    }

    /// User "get_target_path_by_id" If you are trying to pull relative file path to target
    pub(crate) fn get_target_by_id(
        &self,
        relationship_id: &str,
    ) -> AnyResult<Option<String>, AnyError> {
        if let Some(record) = self
            .relationships
            .iter()
            .find(|item| item.0 == relationship_id)
        {
            Ok(Some(record.1.clone()))
        } else {
            Ok(None)
        }
    }

    /// Get Relation Target based on Type
    /// Note: This will get the first element match the criteria
    pub(crate) fn get_relationship_target_path_by_type_mut(
        &mut self,
        content_type: &str,
        content: &Content,
        file_path: Option<String>,
        file_name: Option<String>,
    ) -> AnyResult<String, AnyError> {
        if let Some(relationship) = self
            .relationships
            .iter()
            .find(|item| item.2 == content_type)
        {
            let file_path = relationship.1.clone();
            let relative_path = self
                .get_relative_path()
                .context("Get Relative Path for Part File")?;
            if file_path.starts_with('/') {
                Ok(file_path.strip_prefix('/').unwrap().to_string())
            } else {
                Ok(utils::clean_file_path(&format!(
                    "{}",
                    Path::new(&relative_path).join(file_path).display()
                )))
            }
        } else {
            self.set_new_relationship_path_mut(content, file_path.clone(), file_name.clone())
                .context("Setting New Theme Relationship Failed.")?;
            Ok(format!(
                "{}/{}.{}",
                file_path.unwrap_or(content.default_path.to_string()),
                file_name.unwrap_or(content.default_name.to_string()),
                content.extension
            ))
        }
    }

    /// Generate Next Relationship ID to add
    fn get_next_relationship_id(&self) -> String {
        let mut children = self.relationships.len() + 1;
        loop {
            if self
                .relationships
                .iter()
                .position(|item| item.0 == format!("rId{}", children))
                .is_some()
            {
                children += 1;
            } else {
                break;
            }
        }
        format!("rId{}", children)
    }
}

/// ####################### Mut Access Functions ####################
impl RelationsPart {
    /// Save the relationship detail to xml document
    pub(crate) fn save_relationship_to_doc(&mut self) -> AnyResult<bool, AnyError> {
        if let Some(xml_tree_ref) = self.xml_document.upgrade() {
            let mut xml_tree = xml_tree_ref
                .try_borrow_mut()
                .context("Failed to pull XML Handle")?;
            let root_id = xml_tree.get_root_id();
            let child_count = xml_tree
                .get_element(root_id)
                .context("No Root Relationship Element Found")?
                .get_child_element_count()
                .unwrap_or(0);
            for relationship in self.relationships.clone() {
                let mut attributes = Vec::new();
                attributes.push(XmlAttribute::new("Id".to_string(), relationship.0));
                attributes.push(XmlAttribute::new("Target".to_string(), relationship.1));
                attributes.push(XmlAttribute::new("Type".to_string(), relationship.2));
                if let Some(target_mode) = relationship.3 {
                    attributes.push(XmlAttribute::new("TargetMode".to_string(), target_mode));
                }
                xml_tree
                    .append_child_element_mut(root_id, "Relationship", Some(attributes))
                    .context("Failed to add relationship element")?;
            }
            Ok(child_count > 0 || self.relationships.len() > 0)
        } else {
            Err(anyhow!("Failed to Get XML Handle"))
        }
    }

    /// Create new Relation Path
    pub(crate) fn set_new_relationship_path_mut(
        &mut self,
        content: &Content,
        file_path: Option<String>,
        file_name: Option<String>,
    ) -> AnyResult<String, AnyError> {
        let next_id = self.get_next_relationship_id();
        self.relationships.push((
            next_id.clone(),
            format!(
                "/{}/{}.{}",
                file_path.unwrap_or(content.default_path.to_string()),
                file_name.unwrap_or(content.default_name.to_string()),
                content.extension
            ),
            content.schemas_type.to_string(),
            None,
        ));
        Ok(next_id)
    }

    /// Create new Relation
    pub(crate) fn set_new_relationship_mut(
        &mut self,
        content: &Content,
        target: String,
    ) -> AnyResult<String, AnyError> {
        let next_id = self.get_next_relationship_id();
        self.relationships.push((
            next_id.clone(),
            target,
            content.schemas_type.to_string(),
            // TODO : Make Dynamic on demand
            Some("External".to_string()),
        ));
        Ok(next_id)
    }

    /// Delete Record by ID
    pub(crate) fn delete_relationship_by_id_mut(&mut self, id: &str) {
        self.relationships.retain(|item| item.0 != id)
    }

    /// Delete the target file path
    pub(crate) fn delete_relationship_mut(&mut self, file_path: &str) {
        self.relationships.retain(|item| {
            item.1
                != if file_path.starts_with('/') {
                    file_path.to_string()
                } else {
                    format!("/{}", file_path)
                }
        })
    }
}
