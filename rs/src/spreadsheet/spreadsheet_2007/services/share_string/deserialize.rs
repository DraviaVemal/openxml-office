use std::{cell::RefCell, rc::Weak};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDocument, XmlElementContentType};

use crate::spreadsheet_2007::services::ShareStringPart;

impl ShareStringPart {
    pub(crate) fn deserialize_share_string(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<Vec<String>, AnyError> {
        let mut share_string_collection = Vec::new();
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_document_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::xml doc borrow failed")?;
            let root_id = xml_document_mut.get_root_id();
            if let Some(si_ids) = xml_document_mut
                .find_all_child(root_id, "si")
                .context("draviavemal-openxml_office::Failed to find si elements")?
            {
                for si_id in si_ids {
                    let first_child = xml_document_mut
                        .get_element(si_id)
                        .context("draviavemal-openxml_office::Failed to pull si element")?
                        .get_child_contents()
                        .as_ref()
                        .and_then(|contents| {
                            contents.iter().find_map(|content| match content {
                                XmlElementContentType::Element((id, _, _)) => Some(*id),
                                _ => None,
                            })
                        });
                    if let Some(child_id) = first_child {
                        let text_element = xml_document_mut
                            .get_element(child_id)
                            .context("draviavemal-openxml_office::Failed to pull child element")?;
                        let value = text_element
                            .get_child_contents()
                            .as_ref()
                            .map(|contents| {
                                contents
                                    .iter()
                                    .filter_map(|content| match content {
                                        XmlElementContentType::Text(text) => Some(text.clone()),
                                        _ => None,
                                    })
                                    .collect::<String>()
                            })
                            .unwrap_or_default();
                        share_string_collection.push(value);
                    }
                    xml_document_mut
                        .remove_element_mut(si_id)
                        .context("draviavemal-openxml_office::Failed To remove element")?;
                }
            }
        }
        Ok(share_string_collection)
    }
}
