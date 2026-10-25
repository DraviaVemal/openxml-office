use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NamespaceDeclaration, XmlAttribute, XmlDocument, XmlElementContentType};

use crate::{
    element_dictionary::EXCEL_TYPE_COLLECTION,
    global_2007::traits::{
        XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing,
    },
    log_elapsed,
    namespaces::{RELATIONSHIP_OFFICE_DOC_NS, SPREADSHEET_NS},
    order_dictionary::EXCEL_ORDER_COLLECTION,
    spreadsheet_2007::WorkbookPart,
};

impl Drop for WorkbookPart {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for WorkbookPart {}

impl XmlDocumentPartClose for WorkbookPart {
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                self.theme_part.close_document()?;
                self.common_service
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to pull common Service Handle")?
                    .close_service()
                    .context(
                        "draviavemal-openxml_office::Failed to Close Common Service From Workbook",
                    )?;
                self.workbook_relationship_part
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to pull relationship handle")?
                    .close_document()
                    .context("draviavemal-openxml_office::Failed to Close work")?;
                // Write Sheet Records to Workbook
                if let Some(xml_document_mut) = self.xml_document.upgrade() {
                    let mut xml_doc_mut = xml_document_mut
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::Borrow XML Document Failed")?;
                    let mut sheet_count = 1;
                    let root_id = xml_doc_mut.get_root_id();
                    if let Some(workbook_view) = &self.workbook_view {
                        // Create and Set BookViews
                        let book_views_id = xml_doc_mut
                            .insert_child_element_after_last_tag_mut(
                                root_id,
                                "bookViews",
                                "fileVersion",
                                None,
                            )
                            .context(
                                "draviavemal-openxml_office::Create book viewsD Node Failed",
                            )?;
                        let mut attributes: Vec<XmlAttribute> = Vec::new();
                        if let Some(active_tab) = &workbook_view.active_tab {
                            attributes.push(XmlAttribute::new(
                                "activeTab".to_string(),
                                active_tab.clone(),
                            ));
                        }
                        if let Some(first_sheet) = &workbook_view.first_sheet {
                            attributes.push(XmlAttribute::new(
                                "firstSheet".to_string(),
                                first_sheet.clone(),
                            ));
                        }
                        if let Some(tab_ratio) = &workbook_view.sheet_tab_ratio {
                            attributes.push(XmlAttribute::new(
                                "tabRatio".to_string(),
                                tab_ratio.to_string(),
                            ));
                        }
                        if let Some(auto_filter_date_grouping) =
                            &workbook_view.auto_filter_date_grouping
                        {
                            attributes.push(XmlAttribute::new(
                                "tabRatio".to_string(),
                                if auto_filter_date_grouping.to_owned() {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                },
                            ));
                        }
                        attributes.push(XmlAttribute::new(
                            "minimized".to_string(),
                            if workbook_view.minimize {
                                "1".to_string()
                            } else {
                                "0".to_string()
                            },
                        ));
                        if let Some(visibility) = &workbook_view.visibility {
                            attributes.push(XmlAttribute::new(
                                "visibility".to_string(),
                                visibility.clone(),
                            ));
                        }
                        attributes.push(XmlAttribute::new(
                            "showSheetTabs".to_string(),
                            if workbook_view.hide_sheet_tab {
                                "0".to_string()
                            } else {
                                "1".to_string()
                            },
                        ));
                        attributes.push(XmlAttribute::new(
                            "showVerticalScroll".to_string(),
                            if workbook_view.hide_vertical_scroll {
                                "0".to_string()
                            } else {
                                "1".to_string()
                            },
                        ));
                        attributes.push(XmlAttribute::new(
                            "showHorizontalScroll".to_string(),
                            if workbook_view.hide_horizontal_scroll {
                                "0".to_string()
                            } else {
                                "1".to_string()
                            },
                        ));
                        xml_doc_mut
                            .append_child_element_mut(
                                book_views_id,
                                "workbookView",
                                Some(attributes),
                            )
                            .context(
                                "draviavemal-openxml_office::Failed to create workbook view",
                            )?;
                    }
                    // Create and set Sheets
                    let sheets_id = xml_doc_mut
                        .insert_child_element_after_last_tag_mut(
                            root_id,
                            "sheets",
                            "bookViews",
                            None,
                        )
                        .context("draviavemal-openxml_office::Create Sheets Node Failed")?;
                    for (sheet_display_name, relationship_id, hide) in &self
                        .sheet_collection
                        .try_borrow_mut()
                        .context(
                            "draviavemal-openxml_office::Failed to pull Sheet Name Collection",
                        )?
                        .clone()
                    {
                        let mut attributes = Vec::new();
                        attributes.push(XmlAttribute::new(
                            "name".to_string(),
                            sheet_display_name.to_string(),
                        ));
                        attributes.push(XmlAttribute::new(
                            "sheetId".to_string(),
                            sheet_count.to_string(),
                        ));
                        attributes.push(XmlAttribute::new(
                            "r:id".to_string(),
                            relationship_id.to_string(),
                        ));
                        if *hide {
                            attributes
                                .push(XmlAttribute::new("state".to_string(), "hidden".to_string()));
                        }
                        xml_doc_mut
                            .append_child_element_mut(sheets_id, "sheet", Some(attributes))
                            .context("draviavemal-openxml_office::Create Sheet Node Failed")?;
                        sheet_count += 1;
                    }
                    if let Some(order) = EXCEL_ORDER_COLLECTION.get("workbook") {
                        if let Ok(root_element) = xml_doc_mut.get_element_mut(root_id) {
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
                if let Some(xml_tree) = self.office_document.upgrade() {
                    xml_tree
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::Failed To Pull XML Handle")?
                        .close_xml_document(&self.file_path)?;
                }
                Ok(())
            },
            "Workbook Closed"
        )
    }
}

impl XmlDocumentPartInitializing for WorkbookPart {
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION
            .get("workbook")
            .context("Failed to read excel type collection")?;
        let mut template_core_properties = XmlDocument::new();
        let root_id = template_core_properties
            .create_root_element_ns_mut(
                "workbook",
                &NamespaceDeclaration {
                    uri: SPREADSHEET_NS.uri,
                    default_alias: SPREADSHEET_NS.default_alias,
                    alias_override: Some(""),
                },
                None,
            )
            .context("draviavemal-openxml_office::Initializing Workbook Failed")?;
        template_core_properties
            .get_element_mut(root_id)
            .context("draviavemal-openxml_office::Failed to fetch workbook root element")?
            .add_namespaces_mut(&[RELATIONSHIP_OFFICE_DOC_NS]);
        template_core_properties
            .append_child_element_mut(
                root_id,
                "fileVersion",
                Some(vec![
                    XmlAttribute::new("appName".to_string(), "openxml-office".to_string()),
                    XmlAttribute::new("lastEdited".to_string(), "7".to_string()),
                    XmlAttribute::new("lowestEdited".to_string(), "7".to_string()),
                ]),
            )
            .context("Failed to add child to root element")?;
        Ok((
            template_core_properties,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}
