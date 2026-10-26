use crate::macros::lifecycle::impl_xml_part_lifecycle;
use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    global::traits::{XmlDocumentPartClose, XmlDocumentPartInitializing},
    log_elapsed,
    namespaces::{RELATIONSHIP_OFFICE_DOC_NS, SPREADSHEET_NS},
    order_dictionary::EXCEL_ORDER_COLLECTION,
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlDocument, XmlElementContentType};
use std::rc::Rc;

use super::WorkSheet;

impl_xml_part_lifecycle!(WorkSheet);

impl XmlDocumentPartClose for WorkSheet {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if let Some(office_document) = self.office_document.upgrade() {
                    let mut office_document_mut = office_document
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::Failed to pull office document")?;
                    if let Some(xml_document) = self.xml_document.upgrade() {
                        let mut xml_document_mut = xml_document
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to Pull XML Handle")?;
                        log_elapsed!(self.serialize_dimension(&mut xml_document_mut))?;
                        log_elapsed!(self.serialize_cols(&mut xml_document_mut))?;
                        log_elapsed!(self.serialize_sheet_views(&mut xml_document_mut))?;
                        log_elapsed!(self.serialize_sheet_data(&mut xml_document_mut))?;
                        log_elapsed!(self.serialize_merge_cells(&mut xml_document_mut))?;
                        log_elapsed!(self.serialize_hyperlinks(
                            &mut xml_document_mut,
                            Rc::clone(&self.sheet_relationship_part)
                        ))?;
                        if let Some(order) = EXCEL_ORDER_COLLECTION.get("worksheet") {
                            let root_id = xml_document_mut.get_root_id();
                            if let Ok(root_element) = xml_document_mut.get_element_mut(root_id) {
                                if let Some(contents) = root_element.get_child_contents_mut() {
                                    contents.sort_by_key(|content| match content {
                                        XmlElementContentType::Element((_, _, ns_tag)) => order
                                            .iter()
                                            .position(|item| *item == ns_tag.as_str())
                                            .unwrap_or(usize::MAX),
                                        _ => usize::MAX,
                                    });
                                }
                            }
                        }
                    }
                    log_elapsed!(
                        || {
                            office_document_mut
                                .close_xml_document(&self.file_path)
                                .context("draviavemal-openxml_office::Failed to close the current worksheet document")
                        },
                        "Close worksheet document"
                    )?;
                }
                log_elapsed!(
                    || {
                        self.drawing_part
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to pull Drawing handle")?
                            .close_document()
                            .context("draviavemal-openxml_office::Failed to Close Drawing part")
                    },
                    "Worksheet Drawing part closed"
                )?;
                log_elapsed!(
                    || {
                        self.sheet_relationship_part
                            .try_borrow_mut()
                            .context(
                                "draviavemal-openxml_office::Failed to pull relationship handle",
                            )?
                            .close_document()
                            .context(
                                "draviavemal-openxml_office::Failed to Close relationship part",
                            )
                    },
                    "Worksheet relation part closed"
                )?;
                Ok(())
            },
            "Close Worksheet"
        )
    }
}

impl XmlDocumentPartInitializing for WorkSheet {
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION
            .get("worksheet")
            .context("Type Collection Missing key value")?;
        let mut template_core_properties = XmlDocument::new();
        let root_id = template_core_properties
            .create_root_element_ns_mut(
                "worksheet",
                &NamespaceDeclaration {
                    default_alias: SPREADSHEET_NS.default_alias,
                    uri: SPREADSHEET_NS.uri,
                    alias_override: Some(""),
                },
                None,
            )
            .context("Failed to create worksheet root element")?;
        template_core_properties
            .get_element_mut(root_id)
            .context("draviavemal-openxml_office::Failed to fetch worksheet root element")?
            .add_namespaces_mut(&[RELATIONSHIP_OFFICE_DOC_NS]);
        template_core_properties
            .append_child_element_mut(root_id, "sheetData", None)
            .context("Failed to add Sheet Data to the worksheet")?;
        Ok((
            template_core_properties,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}
