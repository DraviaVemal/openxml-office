use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Weak,
};

use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    global_2007::files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartInitializing},
    },
    log_elapsed,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDeserializer, XmlDocument};

use super::StylePart;

impl_xml_part_lifecycle!(StylePart);

impl XmlDocumentPartClose for StylePart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                log_elapsed!(
                    || self
                        .save_content_to_tree_mut()
                        .context("draviavemal-openxml_office::Style Save Content Failed"),
                    "Save Tree"
                )?;
                if let Some(xml_tree) = self.office_document.upgrade() {
                    log_elapsed!(
                        || {
                            xml_tree
                                .try_borrow_mut()
                                .context("draviavemal-openxml_office::Failed To pull XML Handle")?
                                .close_xml_document(&self.file_path)
                        },
                        "Close Document"
                    )?;
                }
                Ok(())
            },
            "Close Style Service"
        )
    }
}

impl XmlDocumentPartInitializing for StylePart {
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION
            .get("style")
            .context("Failed to read Excel type collection")?;
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(include_str!("style.xml").as_bytes().to_vec())
                .context("draviavemal-openxml_office::Initializing Theme Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

impl XmlDocumentPart for StylePart {
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<StylePart, AnyError> {
        let file_path = StylePart::get_style_file_name(&parent_relationship_part)
            .context("draviavemal-openxml_office::Failed to pull style file name")?
            .to_string();
        let mut xml_document = StylePart::get_xml_document(&office_document, &file_path)?;
        let (
            number_format_collection,
            font_collection,
            fill_collection,
            border_collection,
            cell_style_collection,
            cell_collection,
        ) = StylePart::deserialize_content(&mut xml_document)
            .context("draviavemal-openxml_office::Load Share String To Object Failed")?;
        Ok(StylePart {
            office_document,
            xml_document,
            file_path,
            cache_id: HashMap::new(),
            cache_order: VecDeque::new(),
            cache_capacity: 25,
            number_format_collection,
            font_collection,
            fill_collection,
            border_collection,
            cell_style_xfs_collection: cell_style_collection,
            cell_xfs_collection: cell_collection,
        })
    }
}
