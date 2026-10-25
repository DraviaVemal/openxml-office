use std::{cell::RefCell, rc::Weak};

use anyhow::{Context, Error as AnyError, Result as AnyResult};

use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{XmlDocumentPart, XmlDocumentPartInitializing},
    },
    spreadsheet_2007::services::ShareStringPart,
};

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
}
