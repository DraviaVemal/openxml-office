use std::path::Path;

use crate::{
    element_dictionary::Content,
    global_2007::parts::relations::RelationsPart,
    utils,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::XmlAttribute;

impl RelationsPart {
    pub(crate) fn get_or_add_media_relationship_mut(
        &mut self,
        media_directory: &str,
        extension: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> AnyResult<String, AnyError> {
        let office_document = self
            .office_document
            .upgrade()
            .context("draviavemal-openxml_office::Failed to upgrade office document for media")?;
        let media_path = office_document
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Failed to borrow office document for media")?
            .get_or_add_media_part(media_directory, extension, content_type, data)?;
        let image_relationship_type = self.rules.image_relationship_type();

        for relationship in &self.relationships {
            if relationship.2 != image_relationship_type || relationship.3.is_some() {
                continue;
            }
            let resolved_target = match relationship.1.strip_prefix('/') {
                Some(absolute_path) => absolute_path.to_string(),
                None => utils::clean_file_path(&format!(
                    "{}/{}",
                    self.get_relative_path()?,
                    relationship.1
                )),
            };
            if resolved_target == media_path {
                return Ok(relationship.0.clone());
            }
        }

        let relationship_id = self.get_next_relationship_id();
        self.relationships.push((
            relationship_id.clone(),
            format!("/{}", media_path),
            image_relationship_type.to_string(),
            None,
        ));
        Ok(relationship_id)
    }

    /// Get Relation Target based on Type
    /// Note: This will get the first element match the criteria
    /// FIXME: Assumed implementation for Gen1 need to be cleaned up for standardized usecase
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
            let target_path = relationship.1.clone();
            if let Some(absolute_path) = target_path.strip_prefix('/') {
                return Ok(absolute_path.to_string());
            }
            let relative_path = self
                .get_relative_path()
                .context("draviavemal-openxml_office::Get Relative Path for Part File")?;
            return Ok(utils::clean_file_path(&format!(
                "{}",
                Path::new(&relative_path).join(target_path).display()
            )));
        }
        let directory_path = file_path
            .clone()
            .unwrap_or(content.default_path.to_string());
        let file_name = if let Some(file_name) = file_name {
            file_name
        } else {
            let office_document = self.office_document.upgrade().context(
                "draviavemal-openxml_office::Failed to upgrade office document for part numbering",
            )?;
            let next_number = office_document
                .try_borrow()
                .context(
                    "draviavemal-openxml_office::Failed to borrow office document for part numbering",
                )?
                .get_next_part_number(&directory_path, content.default_name, content.extension);
            if content.is_unique == &true {
                content.default_name.to_string()
            } else {
                format!("{}{}", content.default_name, next_number)
            }
        };
        self.set_new_relationship_path_mut(content, file_path.clone(), Some(file_name.clone()))
            .context("draviavemal-openxml_office::Setting New Part Relationship Failed.")?;
        Ok(format!(
            "{}/{}.{}",
            directory_path, file_name, content.extension
        ))
    }

    /// Save the relationship detail to xml document
    pub(crate) fn save_relationship_to_doc(&mut self) -> AnyResult<bool, AnyError> {
        let Some(xml_document) = self.xml_document.upgrade() else {
            return Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to Get XML Handle",
            ));
        };
        let mut xml_document_mut = xml_document
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Failed to pull XML Handle")?;
        let root_id = xml_document_mut.get_root_id();
        let child_count = xml_document_mut
            .get_element(root_id)
            .context("draviavemal-openxml_office::No Root Relationship Element Found")?
            .get_child_element_count()
            .unwrap_or(0);
        for relationship in self.relationships.clone() {
            let mut attributes = vec![
                XmlAttribute::new("Id".to_string(), relationship.0),
                XmlAttribute::new("Target".to_string(), relationship.1),
                XmlAttribute::new("Type".to_string(), relationship.2),
            ];
            if let Some(target_mode) = relationship.3 {
                attributes.push(XmlAttribute::new("TargetMode".to_string(), target_mode));
            }
            xml_document_mut
                .append_child_element_mut(root_id, "Relationship", Some(attributes))
                .context("draviavemal-openxml_office::Failed to add relationship element")?;
        }
        Ok(child_count > 0 || !self.relationships.is_empty())
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
        let absolute_target = if file_path.starts_with('/') {
            file_path.to_string()
        } else {
            format!("/{}", file_path)
        };
        self.relationships.retain(|item| item.1 != absolute_target)
    }
}
