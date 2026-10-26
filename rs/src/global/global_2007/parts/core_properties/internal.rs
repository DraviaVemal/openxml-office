use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    global_2007::{
        files::OfficeDocument,
        parts::{core_properties::CorePropertiesPart, RelationsPart},
        traits::{XmlDocumentPart, XmlDocumentPartInitializing},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, rc::Weak};

impl XmlDocumentPart for CorePropertiesPart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<CorePropertiesPart, AnyError> {
        let file_path = Self::get_core_properties_file_name(&parent_relationship_part)
            .context("draviavemal-openxml_office::Failed to pull Core Property file name")?;
        let xml_document = Self::get_xml_document(&office_document, &file_path)?;
        Ok(Self {
            office_document,
            xml_document,
            file_path,
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
