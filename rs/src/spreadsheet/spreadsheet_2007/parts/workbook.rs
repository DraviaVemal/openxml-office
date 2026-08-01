// TODO
// Move Sheet Is pending - Note : Should hadle calculation chain ref when adding
use crate::{
    converters::ConverterUtil,
    element_dictionary::EXCEL_TYPE_COLLECTION,
    files::OfficeDocument,
    global_2007::{
        parts::{RelationsPart, ThemePart},
        service::MediaFiles,
        traits::{
            XmlDocumentPart, XmlDocumentPartClose, XmlDocumentPartFlush,
            XmlDocumentPartInitializing,
        },
    },
    log_elapsed,
    order_dictionary::EXCEL_ORDER_COLLECTION,
    spreadsheet_2007::{
        models::{CellStyleSetting, StyleId},
        parts::WorkSheet,
        services::{CalculationChainPart, CommonServices, ShareStringPart, StylePart},
    },
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{
    NodeId, XmlAttribute, XmlDeserializer, XmlDocument, XmlElementContentType,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub(crate) struct WorkbookPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
    common_service: Rc<RefCell<CommonServices>>,
    workbook_relationship_part: Rc<RefCell<RelationsPart>>,
    theme_part: ThemePart,
    /// This contain the (display sheet name, relationId, hide sheet)
    sheet_collection: Rc<RefCell<Vec<(String, String, bool)>>>,
    workbook_view: Option<WorkbookView>,
}

#[derive(Debug)]
pub(crate) struct WorkbookView {
    active_tab: Option<String>,
    first_sheet: Option<String>,
    visibility: Option<String>,
    minimize: bool,
    hide_horizontal_scroll: bool,
    hide_vertical_scroll: bool,
    hide_sheet_tab: bool,
    sheet_tab_ratio: Option<i16>,
    auto_filter_date_grouping: Option<bool>,
}

impl Default for WorkbookView {
    fn default() -> Self {
        WorkbookView {
            active_tab: None,
            first_sheet: None,
            minimize: false,
            auto_filter_date_grouping: None,
            hide_sheet_tab: false,
            hide_vertical_scroll: false,
            hide_horizontal_scroll: false,
            sheet_tab_ratio: None,
            visibility: None,
        }
    }
}

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
                    .context("Failed to pull common Service Handle")?
                    .close_service()
                    .context("Failed to Close Common Service From Workbook")?;
                self.workbook_relationship_part
                    .try_borrow_mut()
                    .context("Failed to pull relationship handle")?
                    .close_document()
                    .context("Failed to Close work")?;
                // Write Sheet Records to Workbook
                if let Some(xml_document_mut) = self.xml_document.upgrade() {
                    let mut xml_doc_mut = xml_document_mut
                        .try_borrow_mut()
                        .context("Borrow XML Document Failed")?;
                    let mut sheet_count = 1;
                    let root_id = xml_doc_mut.get_root_id();
                    if let Some(workbook_view) = &self.workbook_view {
                        // Create and Set BookViews
                        let book_views_id = xml_doc_mut
                            .inser_child_element_after_last_tag_mut(
                                root_id,
                                "bookViews",
                                "fileVersion",
                                None,
                            )
                            .context("Create book viewsD Node Failed")?;
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
                            .context("Failed to create workbook view")?;
                    }
                    // Create and set Sheets
                    let sheets_id = xml_doc_mut
                        .inser_child_element_after_last_tag_mut(
                            root_id,
                            "sheets",
                            "bookViews",
                            None,
                        )
                        .context("Create Sheets Node Failed")?;
                    for (sheet_display_name, relationship_id, hide) in &self
                        .sheet_collection
                        .try_borrow_mut()
                        .context("Failed to pull Sheet Name Collection")?
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
                            .context("Create Sheet Node Failed")?;
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
                        .context("Failed To Pull XML Handle")?
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
        let content = EXCEL_TYPE_COLLECTION.get("workbook").unwrap();
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
    xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <fileVersion appName="openxml-office" lastEdited="7" lowestEdited="7" />
</workbook>"#;
        Ok((
            XmlDeserializer::vec_to_xml_doc_tree(template_core_properties.as_bytes().to_vec())
                .context("Initializing Workbook Failed")?,
            Some(content.content_type.to_string()),
            content.extension.to_string(),
            content.extension_type.to_string(),
        ))
    }
}

/// ######################### Trait implementation of XML Part - Only accessible within crate ##############
impl XmlDocumentPart for WorkbookPart {
    /// Create workbook
    fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        parent_relationship_part: Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<WorkbookPart, AnyError> {
        log_elapsed!(
            || {
                let file_name = WorkbookPart::get_workbook_file_name(&parent_relationship_part)
                    .context("Failed to pull workbook file name")?
                    .to_string();
                let mut file_tree = WorkbookPart::get_xml_document(&office_document, &file_name)?;
                let workbook_relationship_part = Rc::new(RefCell::new(
                    RelationsPart::new(
                        office_document.clone(),
                        &format!(
                            "{}/_rels/workbook.xml.rels",
                            &file_name[..file_name.rfind("/").unwrap()]
                        ),
                    )
                    .context("Creating Relation ship part for workbook failed.")?,
                ));
                // Theme
                let theme_part = ThemePart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("Loading Theme Part Failed")?;
                // Share String
                let meadia_files = MediaFiles::new().context("Loading Share String Failed")?;
                // Share String
                let share_string = ShareStringPart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("Loading Share String Failed")?;
                // Calculation chain
                let calculation_chain = CalculationChainPart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("Loading Calculation Chain Failed")?;
                // Style
                let style = StylePart::new(
                    office_document.clone(),
                    Rc::downgrade(&workbook_relationship_part),
                )
                .context("Loading Style Part Failed")?;
                let common_service = Rc::new(RefCell::new(CommonServices::new(
                    meadia_files,
                    calculation_chain,
                    share_string,
                    style,
                )));
                let (sheet_collection, workbook_view) =
                    WorkbookPart::load_sheet_names(&mut file_tree)
                        .context("Loading Sheet Names Failed")?;
                Ok(WorkbookPart {
                    office_document,
                    xml_document: file_tree,
                    file_path: file_name,
                    common_service,
                    workbook_relationship_part,
                    theme_part,
                    sheet_collection: Rc::new(RefCell::new(sheet_collection)),
                    workbook_view,
                })
            },
            "Create New Workbook"
        )
    }
}

// ############################# Internal Function ######################################
// ############################# mut Function ######################################
impl WorkbookPart {
    fn load_sheet_names(
        xml_document: &mut Weak<RefCell<XmlDocument>>,
    ) -> AnyResult<(Vec<(String, String, bool)>, Option<WorkbookView>), AnyError> {
        log_elapsed!(
            || {
                let mut sheet_collection = Vec::new();
                let mut workbook_view = None;
                if let Some(xml_document) = xml_document.upgrade() {
                    let mut xml_doc_mut = xml_document
                        .try_borrow_mut()
                        .context("xml doc borrow failed")?;
                    let root_id = xml_doc_mut.get_root_id();
                    // Deconstruct Book View for sheet collection data
                    if let Some(book_views_id) = xml_doc_mut
                        .find_first_child(root_id, "bookViews")
                        .context("Failed to find bookViews element")?
                    {
                        let workbook_view_ids: Vec<NodeId> = xml_doc_mut
                            .get_element(book_views_id)
                            .context("Failed to pull bookViews element")?
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
                            let workbook_view_element =
                                xml_doc_mut
                                    .get_element(workbook_view_id)
                                    .context("Failed to pull workbookView element")?;
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
                                            .context("Failed to Parse Tab Ratio Numeric")?,
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
                        xml_doc_mut
                            .remove_element_mut(book_views_id)
                            .context("Failed remove bookViews element")?
                    }
                    // Deconstruct Sheets into collection
                    if let Some(sheets_id) = xml_doc_mut
                        .find_first_child(root_id, "sheets")
                        .context("Failed to find sheets element")?
                    {
                        let sheet_ids: Vec<NodeId> = xml_doc_mut
                            .get_element(sheets_id)
                            .context("Failed to pull sheets element")?
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
                            let sheet = xml_doc_mut
                                .get_element(sheet_id)
                                .context("Failed to pull sheet element")?;
                            let name = sheet
                                .get_attribute("name")
                                .context("Error When Trying to read Sheet Details.")?;
                            let r_id = sheet
                                .get_attribute_ns("r:id")
                                .context("Error When Trying to read Sheet Details.")?;
                            let state = sheet.get_attribute("state");
                            sheet_collection.push((
                                name.get_value().to_string(),
                                r_id.get_value().to_string(),
                                state.map_or(false, |state| state.get_value() == "hidden"),
                            ));
                        }
                        // Delete Deconstructed Sheets from XML
                        xml_doc_mut
                            .remove_element_mut(sheets_id)
                            .context("Failed remove sheets element")?
                    }
                }
                Ok((sheet_collection, workbook_view))
            },
            "Load Existing Workbook"
        )
    }
}

// ############################# Feature Function ######################################

// ############################# im-mut Function ######################################
impl WorkbookPart {
    fn get_workbook_file_name(
        relations_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<String, AnyError> {
        let relationship_content = EXCEL_TYPE_COLLECTION.get("workbook").unwrap();
        if let Some(relations_part) = relations_part.upgrade() {
            Ok(relations_part
                .try_borrow_mut()
                .context("Failed to pull relationship connection")?
                .get_relationship_target_path_by_type_mut(
                    &relationship_content.schemas_type,
                    relationship_content,
                    None,
                    None,
                )
                .context("Pull Path From Existing File Failed")?)
        } else {
            Err(anyhow!("Failed to upgrade relation part"))
        }
    }

    pub(crate) fn list_sheet_names(&self) -> AnyResult<Vec<String>, AnyError> {
        Ok(self
            .sheet_collection
            .try_borrow()
            .context("Failed to pull Sheet Name Collection")?
            .iter()
            .map(|(sheet_name, _, _)| sheet_name.to_string())
            .collect::<Vec<String>>())
    }
}

// ############################# mut Function ######################################
impl WorkbookPart {
    pub(crate) fn add_sheet_mut(
        &mut self,
        sheet_name: Option<String>,
    ) -> AnyResult<WorkSheet, AnyError> {
        Ok(WorkSheet::new(
            self.office_document.clone(),
            Rc::downgrade(&self.sheet_collection),
            Rc::downgrade(&self.workbook_relationship_part),
            Rc::downgrade(&self.common_service),
            sheet_name,
        )
        .context("Worksheet Creation Failed")?)
    }

    pub(crate) fn get_worksheet_mut(&mut self, sheet_name: &str) -> AnyResult<WorkSheet, AnyError> {
        if self
            .sheet_collection
            .borrow()
            .iter()
            .any(|(ref_sheet_name, _, _)| ref_sheet_name == sheet_name)
        {
            log_elapsed!(
                || {
                    WorkSheet::new(
                        self.office_document.clone(),
                        Rc::downgrade(&self.sheet_collection),
                        Rc::downgrade(&self.workbook_relationship_part),
                        Rc::downgrade(&self.common_service),
                        Some(sheet_name.to_string()),
                    )
                    .context("Failed to Find Worksheet")
                },
                "Get Exiting Workbook"
            )
        } else {
            Err(anyhow!("Sheet Not Found!"))
        }
    }

    /// Set Active sheet on opening the excel
    pub(crate) fn set_active_sheet_mut(&mut self, sheet_name: &str) -> AnyResult<(), AnyError> {
        let mut tab_count: i32 = 0;
        for (current_sheet_name, _, _) in self
            .sheet_collection
            .try_borrow()
            .context("Failed to pull Sheet Collection Handle")?
            .iter()
        {
            if current_sheet_name == sheet_name {
                self.workbook_view
                    .get_or_insert(WorkbookView::default())
                    .active_tab = Some(tab_count.to_string());
                break;
            }
            tab_count += 1;
        }
        Ok(())
    }

    /// Set workbook visibility
    pub(crate) fn set_visibility_mut(&mut self, is_visible: bool) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.visibility = if is_visible {
                Some("visible".to_string())
            } else {
                Some("hidden".to_string())
            }
        } else {
            self.workbook_view = Some(WorkbookView {
                visibility: if is_visible {
                    Some("visible".to_string())
                } else {
                    Some("hidden".to_string())
                },
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set workbook minimized
    pub(crate) fn minimize_workbook_mut(&mut self, is_minimized: bool) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.minimize = is_minimized;
        } else {
            self.workbook_view = Some(WorkbookView {
                minimize: is_minimized,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set visibility of sheet tabs in workbook
    pub(crate) fn hide_sheet_tabs_mut(&mut self, hide_sheet_tab: bool) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.hide_sheet_tab = hide_sheet_tab;
        } else {
            self.workbook_view = Some(WorkbookView {
                hide_sheet_tab,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set workbook Vertical Scroll Visibility
    pub(crate) fn hide_vertical_scroll_mut(
        &mut self,
        hide_vertical_scroll: bool,
    ) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.hide_vertical_scroll = hide_vertical_scroll;
        } else {
            self.workbook_view = Some(WorkbookView {
                hide_vertical_scroll,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set workbook Horizontal Scroll Visibility
    pub(crate) fn hide_horizontal_scroll_mut(
        &mut self,
        hide_horizontal_scroll: bool,
    ) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.hide_horizontal_scroll = hide_horizontal_scroll;
        } else {
            self.workbook_view = Some(WorkbookView {
                hide_horizontal_scroll,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Hide sheet on opening the excel
    pub(crate) fn hide_sheet_mut(&mut self, sheet_name: &str) -> AnyResult<(), AnyError> {
        for (current_sheet_name, _, hide_sheet) in self
            .sheet_collection
            .try_borrow_mut()
            .context("Failed to pull Sheet Collection Handle")?
            .iter_mut()
        {
            if current_sheet_name == sheet_name {
                *hide_sheet = true
            } else {
                *hide_sheet = false
            }
        }
        Ok(())
    }

    pub(crate) fn rename_sheet_name_mut(
        &mut self,
        old_sheet_name: &str,
        new_sheet_name: &str,
    ) -> AnyResult<(), AnyError> {
        // Check if sheet with same name exist
        if self
            .sheet_collection
            .try_borrow()
            .context("Failed to pull Sheet Name Collection")?
            .iter()
            .any(|item| new_sheet_name == item.0)
        {
            Err(anyhow!("New Sheet Name Already exist in the stack"))
        } else {
            if let Some(record) = self
                .sheet_collection
                .try_borrow_mut()
                .context("Failed to pull Sheet Name Collection")?
                .iter_mut()
                .find(|item| item.0 == old_sheet_name)
            {
                record.0 = new_sheet_name.to_string();
                Ok(())
            } else {
                Err(anyhow!("Old Sheet Name not found in the stack"))
            }
        }
    }

    /// Return Style Id for the said combination
    pub(crate) fn get_style_id_mut(
        &mut self,
        style_setting: CellStyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        self.common_service
            .try_borrow_mut()
            .context("Failed to get Style Handle")?
            .get_style_id_mut(style_setting)
    }
}
