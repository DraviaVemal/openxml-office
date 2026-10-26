use std::{cell::RefCell, rc::Weak};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlDocument, XmlElementContentType};

use crate::{
    converters::ConverterUtil,
    log_elapsed,
    namespaces::RELATIONSHIP_OFFICE_DOC_NS,
    spreadsheet_2007::{WorkbookPart, WorkbookView},
};

impl WorkbookPart {
    pub(crate) fn load_sheet_names(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<(Vec<(String, String, bool)>, Option<WorkbookView>), AnyError> {
        log_elapsed!(
            || {
                let mut sheet_collection = Vec::new();
                let mut workbook_view = None;
                if let Some(xml_document) = xml_document.upgrade() {
                    let mut xml_document_mut = xml_document
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::xml doc borrow failed")?;
                    let root_id = xml_document_mut.get_root_id();
                    // Deconstruct Book View for sheet collection data
                    if let Some(book_views_id) = xml_document_mut
                        .find_first_child(root_id, "bookViews")
                        .context("draviavemal-openxml_office::Failed to find bookViews element")?
                    {
                        let workbook_view_ids: Vec<NodeId> = xml_document_mut
                            .get_element(book_views_id)
                            .context(
                                "draviavemal-openxml_office::Failed to pull bookViews element",
                            )?
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
                            .unwrap_or_default();
                        for workbook_view_id in workbook_view_ids {
                            let workbook_view_element = xml_document_mut
                                .get_element(workbook_view_id)
                                .context(
                                "draviavemal-openxml_office::Failed to pull workbookView element",
                            )?;
                            workbook_view = Some(WorkbookView {
                                auto_filter_date_grouping: match workbook_view_element
                                    .get_attribute("autoFilterDateGrouping")
                                {
                                    Some(auto_filter_data_group) => Some(
                                        ConverterUtil::normalize_bool_property_u8(
                                            auto_filter_data_group.get_value(),
                                        ) == 1,
                                    ),
                                    None => None,
                                },
                                sheet_tab_ratio: match workbook_view_element
                                    .get_attribute("tabRatio")
                                {
                                    Some(tab_ratio) => Some(
                                        tab_ratio
                                            .get_value()
                                            .parse()
                                            .context("draviavemal-openxml_office::Failed to Parse Tab Ratio Numeric")?,
                                    ),
                                    None => None,
                                },
                                active_tab: workbook_view_element
                                    .get_attribute("activeTab")
                                    .map(|attribute| attribute.get_value().to_string()),
                                first_sheet: workbook_view_element
                                    .get_attribute("firstSheet")
                                    .map(|attribute| attribute.get_value().to_string()),
                                hide_sheet_tab: workbook_view_element
                                    .get_attribute("showSheetTabs")
                                    .map_or(false, |attribute| {
                                        ConverterUtil::normalize_bool_property_u8(
                                            attribute.get_value(),
                                        ) == 1
                                    }),
                                visibility: workbook_view_element
                                    .get_attribute("visibility")
                                    .map(|attribute| attribute.get_value().to_string()),
                                minimize: workbook_view_element.get_attribute("minimized").map_or(
                                    false,
                                    |attribute| {
                                        ConverterUtil::normalize_bool_property_u8(
                                            attribute.get_value(),
                                        ) == 1
                                    },
                                ),
                                hide_horizontal_scroll: workbook_view_element
                                    .get_attribute("showHorizontalScroll")
                                    .map_or(false, |attribute| {
                                        ConverterUtil::normalize_bool_property_u8(
                                            attribute.get_value(),
                                        ) == 0
                                    }),
                                hide_vertical_scroll: workbook_view_element
                                    .get_attribute("showVerticalScroll")
                                    .map_or(false, |attribute| {
                                        ConverterUtil::normalize_bool_property_u8(
                                            attribute.get_value(),
                                        ) == 0
                                    }),
                            })
                        }
                        // Delete Deconstructed Book View from XML
                        xml_document_mut.remove_element_mut(book_views_id).context(
                            "draviavemal-openxml_office::Failed remove bookViews element",
                        )?
                    }
                    // Deconstruct Sheets into collection
                    if let Some(sheets_id) = xml_document_mut
                        .find_first_child(root_id, "sheets")
                        .context("draviavemal-openxml_office::Failed to find sheets element")?
                    {
                        let sheet_ids: Vec<NodeId> = xml_document_mut
                            .get_element(sheets_id)
                            .context("draviavemal-openxml_office::Failed to pull sheets element")?
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
                            .unwrap_or_default();
                        for sheet_id in sheet_ids {
                            let sheet = xml_document_mut.get_element(sheet_id).context(
                                "draviavemal-openxml_office::Failed to pull sheet element",
                            )?;
                            let name = sheet
                                .get_attribute("name")
                                .context("draviavemal-openxml_office::Error When Trying to read Sheet Details.")?;
                            let r_id = sheet
                                .get_attribute_by_uri(RELATIONSHIP_OFFICE_DOC_NS.uri, "id")
                                .context("draviavemal-openxml_office::Error When Trying to read Sheet Details.")?;
                            let state = sheet.get_attribute("state");
                            sheet_collection.push((
                                name.get_value().to_string(),
                                r_id.get_value().to_string(),
                                state.map_or(false, |state| state.get_value() == "hidden"),
                            ));
                        }
                        // Delete Deconstructed Sheets from XML
                        xml_document_mut
                            .remove_element_mut(sheets_id)
                            .context("draviavemal-openxml_office::Failed remove sheets element")?
                    }
                }
                Ok((sheet_collection, workbook_view))
            },
            "Load Existing Workbook"
        )
    }
}
