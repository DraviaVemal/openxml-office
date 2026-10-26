use std::{cell::RefCell, rc::Weak};

use crate::global_2007::parts::relations::{RelationsPart, RelationshipEntry};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::XmlDocument;

impl RelationsPart {
    pub(crate) fn load_relations(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<RelationshipEntry>, AnyError> {
        let mut relationships = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_document_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed for get XML Handle")?;
            let root_id = xml_document_mut.get_root_id();
            if let Some(relationship_ids) = xml_document_mut
                .find_all_child(root_id, "Relationship")
                .context("draviavemal-openxml_office::Failed to find Relationship elements")?
            {
                for relationship_id in relationship_ids {
                    let relationship_element = xml_document_mut
                        .get_element(relationship_id)
                        .context("draviavemal-openxml_office::Failed! Relationship element missing")?;
                    relationships.push((
                        relationship_element
                            .get_attribute("Id")
                            .context("draviavemal-openxml_office::Failed. Id in relationship Not Fount!")?
                            .get_value()
                            .to_string(),
                        relationship_element
                            .get_attribute("Target")
                            .context("draviavemal-openxml_office::Failed. Target in relationship Not Fount!")?
                            .get_value()
                            .to_string(),
                        relationship_element
                            .get_attribute("Type")
                            .context("draviavemal-openxml_office::Failed. Type in relationship Not Fount!")?
                            .get_value()
                            .to_string(),
                        relationship_element
                            .get_attribute("TargetMode")
                            .map(|attribute| attribute.get_value().to_string()),
                    ));
                    xml_document_mut
                        .remove_element_mut(relationship_id)
                        .context("draviavemal-openxml_office::Failed to remove Relationship node")?;
                }
            }
        }
        Ok(relationships)
    }

    pub(crate) fn get_relative_path(&self) -> AnyResult<String, AnyError> {
        let rels_position = self
            .file_path
            .find("_rels")
            .context("draviavemal-openxml_office::Failed to string Prefix path from relation")?;
        if rels_position > 0 {
            Ok(format!("{}/", &self.file_path[..rels_position - 1]))
        } else {
            Ok("".to_string())
        }
    }

    pub(crate) fn get_media_directory(&self) -> AnyResult<String, AnyError> {
        let relative_path = self.get_relative_path()?;
        self.rules
            .media_directory(&relative_path)
            .context("draviavemal-openxml_office::Failed to determine package media directory")
    }

    pub(crate) fn get_target_path_by_id(
        &self,
        relationship_id: &str,
    ) -> AnyResult<Option<String>, AnyError> {
        let Some(record) = self
            .relationships
            .iter()
            .find(|item| item.0 == relationship_id)
        else {
            return Ok(None);
        };
        let target_path = record.1.clone();
        if let Some(absolute_path) = target_path.strip_prefix('/') {
            return Ok(Some(absolute_path.to_string()));
        }
        let relative_path = self
            .get_relative_path()
            .context("draviavemal-openxml_office::Get Relative Path for Part File")?;
        Ok(Some(format!("{}{}", relative_path, target_path)))
    }

    /// Use "get_target_path_by_id" to get the file path relative to the package.
    pub(crate) fn get_target_by_id(
        &self,
        relationship_id: &str,
    ) -> AnyResult<Option<String>, AnyError> {
        Ok(self
            .relationships
            .iter()
            .find(|item| item.0 == relationship_id)
            .map(|record| record.1.clone()))
    }
}
