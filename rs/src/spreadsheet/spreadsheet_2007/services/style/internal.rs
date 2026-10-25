use std::cell::RefCell;
use std::rc::Weak;

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlDocument, XmlElementContentType};

use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION, global_2007::parts::RelationsPart,
    spreadsheet_2007::services::StylePart,
};

impl StylePart {
    pub(crate) fn get_style_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let style_content = EXCEL_TYPE_COLLECTION
            .get("style")
            .context("Failed to read Excel type collection")?;
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &style_content.schemas_type,
                    style_content,
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

    /// Collect the NodeId of every child element of the given parent
    pub(crate) fn collect_child_ids(
        xml_doc: &XmlDocument,
        parent_id: NodeId,
    ) -> AnyResult<Vec<NodeId>, AnyError> {
        Ok(xml_doc
            .get_element(parent_id)
            .context("draviavemal-openxml_office::Failed to pull parent element")?
            .get_child_contents()
            .as_ref()
            .map(|contents| {
                contents
                    .iter()
                    .filter_map(|content| match content {
                        XmlElementContentType::Element((id, _, _)) => Some(*id),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }
}
