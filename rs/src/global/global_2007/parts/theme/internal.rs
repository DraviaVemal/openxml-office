use crate::{
    element_dictionary::COMMON_TYPE_COLLECTION,
    global_2007::{
        files::OfficeDocument,
        parts::{theme::ThemePart, RelationsPart},
        traits::{XmlDocumentPart, XmlDocumentPartInitializing},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::{cell::RefCell, rc::Weak};

impl XmlDocumentPart for ThemePart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<ThemePart, AnyError> {
        let file_name = Self::get_theme_file_name(&parent_relationship_part)
            .context("draviavemal-openxml_office::Failed to pull theme file name")?;
        let _xml_document = Self::get_xml_document(&office_document, &file_name)?;
        Ok(ThemePart {
            office_document,
            _xml_document,
            file_path: file_name,
        })
    }
}

impl ThemePart {
    fn get_theme_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let theme_content = COMMON_TYPE_COLLECTION
            .get("theme")
            .context("Failed to read Common type collection")?;
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &theme_content.schemas_type,
                    theme_content,
                    Some(format!("xl/{}", theme_content.default_path)),
                    None,
                )
                .context("draviavemal-openxml_office::Pull Path From Existing File Failed")?)
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to upgrade relation part",
            ))
        }
    }
}
