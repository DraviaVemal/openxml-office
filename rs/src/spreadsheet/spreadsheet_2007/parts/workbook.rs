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
        models::{StyleId, StyleSetting},
        parts::WorkSheet,
        services::{CalculationChainPart, CommonServices, ShareStringPart, StylePart},
    },
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{XmlDeSerializer, XmlDocument};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct WorkbookPart {
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    file_path: String,
    common_service: Rc<RefCell<CommonServices>>,
    workbook_relationship_part: Rc<RefCell<RelationsPart>>,
    theme_part: ThemePart,
    /// This contain the sheet name, relationId, active sheet, hide sheet
    sheet_collection: Rc<RefCell<Vec<(String, String, bool, bool)>>>,
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
                    if let Some(workbook_view) = &self.workbook_view {
                        // Create and Set BookViews
                        let book_views_id = xml_doc_mut
                            .insert_children_after_tag_mut("bookViews", "fileVersion", None, None)
                            .context("Create book viewsD Node Failed")?
                            .get_id();
                        let workbook_view_element = xml_doc_mut
                            .append_child_mut("workbookView", Some(&book_views_id), None)
                            .context("Failed to create workbook view")?;
                        let mut attributes: HashMap<String, String> = HashMap::new();
                        if let Some(active_tab) = &workbook_view.active_tab {
                            attributes.insert("activeTab".to_string(), active_tab.clone());
                        }
                        if let Some(first_sheet) = &workbook_view.first_sheet {
                            attributes.insert("firstSheet".to_string(), first_sheet.clone());
                        }
                        if let Some(tab_ratio) = &workbook_view.sheet_tab_ratio {
                            attributes.insert("tabRatio".to_string(), tab_ratio.to_string());
                        }
                        if let Some(auto_filter_date_grouping) =
                            &workbook_view.auto_filter_date_grouping
                        {
                            attributes.insert(
                                "tabRatio".to_string(),
                                if auto_filter_date_grouping.to_owned() {
                                    "1".to_string()
                                } else {
                                    "0".to_string()
                                },
                            );
                        }
                        attributes.insert(
                            "minimized".to_string(),
                            if workbook_view.minimize {
                                "1".to_string()
                            } else {
                                "0".to_string()
                            },
                        );
                        if let Some(visibility) = &workbook_view.visibility {
                            attributes.insert("visibility".to_string(), visibility.clone());
                        }
                        attributes.insert(
                            "showSheetTabs".to_string(),
                            if workbook_view.hide_sheet_tab {
                                "0".to_string()
                            } else {
                                "1".to_string()
                            },
                        );
                        attributes.insert(
                            "showVerticalScroll".to_string(),
                            if workbook_view.hide_vertical_scroll {
                                "0".to_string()
                            } else {
                                "1".to_string()
                            },
                        );
                        attributes.insert(
                            "showHorizontalScroll".to_string(),
                            if workbook_view.hide_horizontal_scroll {
                                "0".to_string()
                            } else {
                                "1".to_string()
                            },
                        );
                        workbook_view_element
                            .set_attribute_mut(attributes)
                            .context("Failed to set workbook view attributes")?;
                    }
                    // Create and set Sheets
                    let sheets_id = xml_doc_mut
                        .insert_children_after_tag_mut("sheets", "bookViews", None, None)
                        .context("Create Sheets Node Failed")?
                        .get_id();
                    for (sheet_display_name, relationship_id, _, hide) in &self
                        .sheet_collection
                        .try_borrow_mut()
                        .context("Failed to pull Sheet Name Collection")?
                        .clone()
                    {
                        let sheet = xml_doc_mut
                            .append_child_mut("sheet", Some(&sheets_id), None)
                            .context("Create Sheet Node Failed")?;
                        let mut attributes = HashMap::new();
                        attributes.insert("name".to_string(), sheet_display_name.to_string());
                        attributes.insert("sheetId".to_string(), sheet_count.to_string());
                        attributes.insert("r:id".to_string(), relationship_id.to_string());
                        if *hide {
                            attributes.insert("state".to_string(), "hidden".to_string());
                        }
                        sheet
                            .set_attribute_mut(attributes)
                            .context("Sheet Attributes Failed")?;
                        sheet_count += 1;
                    }
                    if let Some(root_element) = xml_doc_mut.get_root_mut() {
                        root_element
                            .order_child_mut(
                                EXCEL_ORDER_COLLECTION
                                    .get("workbook")
                                    .context("Failed to get workbook default order")?,
                            )
                            .context("Failed Reorder the element child's")?;
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
            XmlDeSerializer::vec_to_xml_doc_tree(
                template_core_properties.as_bytes().to_vec(),
                "Default workbook",
            )
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
    ) -> AnyResult<Self, AnyError> {
        log_elapsed!(
            || {
                let file_name = Self::get_workbook_file_name(&parent_relationship_part)
                    .context("Failed to pull workbook file name")?
                    .to_string();
                let mut file_tree = Self::get_xml_document(&office_document, &file_name)?;
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
                    Self::load_sheet_names(&mut file_tree).context("Loading Sheet Names Failed")?;
                Ok(Self {
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
    ) -> AnyResult<(Vec<(String, String, bool, bool)>, Option<WorkbookView>), AnyError> {
        log_elapsed!(
            || {
                let mut sheet_collection = Vec::new();
                let mut workbook_view = None;
                if let Some(xml_document) = xml_document.upgrade() {
                    let mut xml_doc_mut = xml_document
                        .try_borrow_mut()
                        .context("xml doc borrow failed")?;
                    // Deconstruct Book View for sheet collection data
                    if let Some(mut book_views_vec) =
                        xml_doc_mut.pop_elements_by_tag_mut("bookViews", None)
                    {
                        if let Some(book_views) = book_views_vec.pop() {
                            loop {
                                if let Some((workbook_view_id, _)) = book_views.pop_child_mut() {
                                    if let Some(workbook_view_element) =
                                        xml_doc_mut.pop_element_mut(&workbook_view_id)
                                    {
                                        if let Some(attributes) =
                                            workbook_view_element.get_attribute()
                                        {
                                            workbook_view = Some(WorkbookView {
                                                auto_filter_date_grouping: if let Some(
                                                    auto_filter_data_group,
                                                ) =
                                                    attributes.get("autoFilterDateGrouping")
                                                {
                                                    Some(
                                                        ConverterUtil::normalize_bool_property_u8(
                                                            auto_filter_data_group,
                                                        ) == 1,
                                                    )
                                                } else {
                                                    None
                                                },
                                                sheet_tab_ratio: if let Some(tab_ratio) =
                                                    attributes.get("tabRatio")
                                                {
                                                    Some(tab_ratio.parse().context(
                                                        "Failed to Parse Tab Ratio Numeric",
                                                    )?)
                                                } else {
                                                    None
                                                },
                                                active_tab: if let Some(active_tab) =
                                                    attributes.get("activeTab")
                                                {
                                                    Some(active_tab.to_string())
                                                } else {
                                                    None
                                                },
                                                first_sheet: if let Some(first_sheet) =
                                                    attributes.get("firstSheet")
                                                {
                                                    Some(first_sheet.to_string())
                                                } else {
                                                    None
                                                },
                                                hide_sheet_tab: if let Some(hide_sheet_tab) =
                                                    attributes.get("showSheetTabs")
                                                {
                                                    ConverterUtil::normalize_bool_property_u8(
                                                        hide_sheet_tab,
                                                    ) == 1
                                                } else {
                                                    false
                                                },
                                                visibility: if let Some(visibility) =
                                                    attributes.get("visibility")
                                                {
                                                    Some(visibility.to_string())
                                                } else {
                                                    None
                                                },
                                                minimize: if let Some(minimize) =
                                                    attributes.get("minimized")
                                                {
                                                    ConverterUtil::normalize_bool_property_u8(
                                                        minimize,
                                                    ) == 1
                                                } else {
                                                    false
                                                },
                                                hide_horizontal_scroll: if let Some(
                                                    hide_horizontal_scroll,
                                                ) =
                                                    attributes.get("showHorizontalScroll")
                                                {
                                                    ConverterUtil::normalize_bool_property_u8(
                                                        hide_horizontal_scroll,
                                                    ) == 0
                                                } else {
                                                    false
                                                },
                                                hide_vertical_scroll: if let Some(
                                                    hide_vertical_scroll,
                                                ) =
                                                    attributes.get("showVerticalScroll")
                                                {
                                                    ConverterUtil::normalize_bool_property_u8(
                                                        hide_vertical_scroll,
                                                    ) == 0
                                                } else {
                                                    false
                                                },
                                            })
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    // Deconstruct Sheets into collection
                    if let Some(mut sheets_vec) =
                        xml_doc_mut.pop_elements_by_tag_mut("sheets", None)
                    {
                        if let Some(sheets) = sheets_vec.pop() {
                            // Load Sheet from File if exist
                            loop {
                                if let Some((sheet_id, _)) = sheets.pop_child_mut() {
                                    if let Some(sheet) = xml_doc_mut.pop_element_mut(&sheet_id) {
                                        if let Some(attributes) = sheet.get_attribute() {
                                            let name = attributes.get("name").context(
                                                "Error When Trying to read Sheet Details.",
                                            )?;
                                            let r_id = attributes.get("r:id").context(
                                                "Error When Trying to read Sheet Details.",
                                            )?;
                                            let state = attributes.get("state");
                                            sheet_collection.push((
                                                name.to_string(),
                                                r_id.to_string(),
                                                false,
                                                if let Some(state) = state {
                                                    state == "hidden"
                                                } else {
                                                    false
                                                },
                                            ));
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
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
            .map(|(sheet_name, _, _, _)| sheet_name.to_string())
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
            .any(|(ref_sheet_name, _, _, _)| ref_sheet_name == sheet_name)
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
                    .context("Worksheet Creation Failed")
                },
                "Get Exiting Workbook"
            )
        } else {
            Err(anyhow!("Sheet Not Found!"))
        }
    }

    /// Set Active sheet on opening the excel
    pub(crate) fn set_active_sheet_mut(&mut self, sheet_name: &str) -> AnyResult<(), AnyError> {
        for (current_sheet_name, _, active_sheet, _) in self
            .sheet_collection
            .try_borrow_mut()
            .context("Failed to pull Sheet Collection Handle")?
            .iter_mut()
        {
            if current_sheet_name == sheet_name {
                *active_sheet = true
            } else {
                *active_sheet = false
            }
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
        for (current_sheet_name, _, _, hide_sheet) in self
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
        style_setting: StyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        self.common_service
            .try_borrow_mut()
            .context("Failed to get Style Handle")?
            .get_style_id_mut(style_setting)
    }
}
