/// TODO : Features to cover
/// Remove Cell
/// Remove row
/// Remove Column
/// Clear row
/// clear column
/// Insert New Row
/// Insert New Column
use crate::{
    converters::ConverterUtil,
    element_dictionary::{COMMON_TYPE_COLLECTION, EXCEL_TYPE_COLLECTION},
    files::OfficeDocument,
    global_2007::{
        parts::RelationsPart,
        traits::{Enum, XmlDocumentPartClose, XmlDocumentPartFlush, XmlDocumentPartInitializing},
    },
    log_elapsed,
    namespaces::{RELATIONSHIPS_NS, RELATIONSHIP_PKG_NS, SPREADSHEET_NS},
    order_dictionary::EXCEL_ORDER_COLLECTION,
    spreadsheet_2007::{
        models::{
            CellDataType, CellPackage, CellProperty, ColumnIndex, ColumnProperties,
            ExcelPictureSetting, HyperLinks, ReferenceRange, RowIndex, RowProperties, StyleId,
        },
        parts::DrawingPart,
        services::{CalculationChain, CommonServices},
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use draviavemal_xml_rs::{NodeId, XmlAttribute, XmlDocument, XmlElementContentType};
use log::debug;
use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::{BTreeMap, HashMap, VecDeque},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub(crate) struct RowRecords {
    row_property: RowProperties,
    cell_records: Option<BTreeMap<ColumnIndex, CellProperty>>,
}

#[derive(Debug)]
pub(crate) struct Dimension {
    start_col: ColumnIndex,
    end_col: ColumnIndex,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            start_col: 16384,
            end_col: 1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct WorkSheetViewSelection {
    pane: Option<String>,
    active_cell: Option<String>,
    active_cell_id: Option<String>,
    sq_ref: Option<String>,
}

#[derive(Debug)]
pub(crate) struct WorkSheetView {
    selection_collection: Option<Vec<WorkSheetViewSelection>>,
    workbook_view_id: String,
    default_grid_color: Option<bool>,
    view_right_to_left: Option<bool>,
    show_formula_bar: Option<bool>,
    show_grid_line: Option<bool>,
    show_outline_symbol: Option<bool>,
    show_row_col_header: Option<bool>,
    show_ruler: Option<bool>,
    show_white_space: Option<bool>,
    show_zero: Option<bool>,
    tab_selected: Option<bool>,
    top_left_cell: Option<String>,
    view: Option<String>,
    window_protection: Option<bool>,
    zoom_scale: Option<i16>,
    zoom_scale_normal: Option<i16>,
    zoom_scale_page_layout: Option<i16>,
    zoom_scale_sheet_layout: Option<i16>,
}

impl Default for WorkSheetView {
    fn default() -> Self {
        Self {
            workbook_view_id: "0".to_string(),
            selection_collection: None,
            default_grid_color: None,
            view_right_to_left: None,
            show_formula_bar: None,
            show_grid_line: None,
            show_outline_symbol: None,
            show_row_col_header: None,
            show_ruler: None,
            show_white_space: None,
            show_zero: None,
            tab_selected: None,
            top_left_cell: None,
            view: None,
            window_protection: None,
            zoom_scale: None,
            zoom_scale_normal: None,
            zoom_scale_page_layout: None,
            zoom_scale_sheet_layout: None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct WorkSheetViews {
    view_collection: Vec<WorkSheetView>,
}

impl Default for WorkSheetViews {
    fn default() -> Self {
        Self {
            view_collection: vec![WorkSheetView::default()],
        }
    }
}

#[derive(Debug)]
pub struct WorkSheet {
    office_document: Weak<RefCell<OfficeDocument>>,
    xml_document: Weak<RefCell<XmlDocument>>,
    common_service: Weak<RefCell<CommonServices>>,
    workbook_relationship_part: Weak<RefCell<RelationsPart>>,
    sheet_collection: Weak<RefCell<Vec<(String, String, bool)>>>,
    sheet_relationship_part: Rc<RefCell<RelationsPart>>,
    dimension: Dimension,
    // sheet_property: Option<_>,
    sheet_views: WorkSheetViews,
    // sheet_format_property: Option<_>,
    column_collection: Option<VecDeque<ColumnProperties>>,
    sheet_data: Option<BTreeMap<RowIndex, RowRecords>>,
    // sheet_calculation_property:Option<_>
    // protected_range:Option<_>
    merge_cells: Option<Vec<ReferenceRange>>,
    hyperlinks: Option<Vec<HyperLinks>>,
    file_path: String,
    display_sheet_name: String,
    drawing_part: Rc<RefCell<DrawingPart>>,
}

impl Drop for WorkSheet {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

impl XmlDocumentPartFlush for WorkSheet {}

impl XmlDocumentPartClose for WorkSheet {
    /// Close and save this part
    fn close_document(&mut self) -> AnyResult<(), AnyError>
    where
        Self: Sized,
    {
        log_elapsed!(
            || {
                if let Some(office_document) = self.office_document.upgrade() {
                    let mut office_doc_mut = office_document
                        .try_borrow_mut()
                        .context("draviavemal-openxml_office::Failed to pull office document")?;
                    if let Some(xml_document) = self.xml_document.upgrade() {
                        let mut xml_doc_mut = xml_document
                            .try_borrow_mut()
                            .context("draviavemal-openxml_office::Failed to Pull XML Handle")?;
                        // Add dimension
                        log_elapsed!(self.serialize_dimension(&mut xml_doc_mut))?;
                        // Add Cols Record to Document
                        log_elapsed!(self.serialize_cols(&mut xml_doc_mut))?;
                        // Add Sheet Views to Document
                        log_elapsed!(self.serialize_sheet_views(&mut xml_doc_mut))?;
                        // Add Sheet Data to Document
                        log_elapsed!(self.serialize_sheet_data(&mut xml_doc_mut))?;
                        // Add Merge Cell to Document
                        log_elapsed!(self.serialize_merge_cells(&mut xml_doc_mut))?;
                        // Add Hyperlink to Document
                        log_elapsed!(self.serialize_hyperlinks(
                            &mut xml_doc_mut,
                            Rc::clone(&self.sheet_relationship_part)
                        ))?;
                        if let Some(order) = EXCEL_ORDER_COLLECTION.get("worksheet") {
                            let root_id = xml_doc_mut.get_root_id();
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
                    log_elapsed!(
                        || {
                            office_doc_mut
                                .close_xml_document(&self.file_path)
                                .context("draviavemal-openxml_office::Failed to close the current tree document")
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
    /// Initialize xml content for this part from base template
    fn initialize_content_xml() -> AnyResult<(XmlDocument, Option<String>, String, String), AnyError>
    {
        let content = EXCEL_TYPE_COLLECTION
            .get("worksheet")
            .context("Type Collection Missing key value")?;
        let mut template_core_properties = XmlDocument::new();
        let root_id = template_core_properties
            .create_root_element_mut(
                "worksheet",
                Some(vec![
                    XmlAttribute::new("xmlns".to_string(), SPREADSHEET_NS.uri.to_string()),
                    XmlAttribute::new(
                        format!("xmlns:{}", RELATIONSHIP_PKG_NS.default_alias),
                        RELATIONSHIP_PKG_NS.uri.to_string(),
                    ),
                ]),
            )
            .context("Failed to create worksheet root element")?;
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

// ############################# Internal Function ######################################
impl WorkSheet {
    /// Create New object for the group
    pub(crate) fn new(
        office_document: Weak<RefCell<OfficeDocument>>,
        sheet_collection: Weak<RefCell<Vec<(String, String, bool)>>>,
        workbook_relationship_part: Weak<RefCell<RelationsPart>>,
        common_service: Weak<RefCell<CommonServices>>,
        display_sheet_name: Option<String>,
    ) -> AnyResult<WorkSheet, AnyError> {
        let (file_path, display_sheet_name) = WorkSheet::get_sheet_file_name(
            display_sheet_name,
            &office_document,
            &sheet_collection,
            &workbook_relationship_part,
        )
        .context("draviavemal-openxml_office::Failed to pull worksheet file name")?;
        let xml_document = WorkSheet::get_xml_document(&office_document, &file_path)?;
        let sheet_relationship_part = Rc::new(RefCell::new(
            RelationsPart::new(
                office_document.clone(),
                &format!(
                    "{}/_rels/{}.rels",
                    &file_path[..file_path.rfind('/').unwrap()],
                    file_path.rsplit('/').next().unwrap()
                ),
            )
            .context(
                "draviavemal-openxml_office::Creating Relation ship part for workbook failed.",
            )?,
        ));
        let (column_collection, sheet_data, merge_cells, hyperlinks, sheet_views, dimension) = log_elapsed!(
            || {
                WorkSheet::initialize_worksheet(&xml_document, Rc::clone(&sheet_relationship_part))
                    .context("draviavemal-openxml_office::Failed to open Worksheet")
            },
            "Worksheet Initialize Time"
        )?;
        let drawing_part = Rc::new(RefCell::new(
            DrawingPart::new(
                office_document.clone(),
                Rc::downgrade(&sheet_relationship_part),
                common_service.clone(),
                &EXCEL_TYPE_COLLECTION,
            )
            .context(
                "draviavemal-openxml_office::Failed to create/load drawing part of the sheet",
            )?,
        ));
        Ok(WorkSheet {
            office_document,
            xml_document,
            common_service,
            workbook_relationship_part,
            sheet_relationship_part,
            dimension,
            sheet_views,
            sheet_collection,
            column_collection,
            sheet_data,
            merge_cells,
            hyperlinks,
            file_path: file_path.to_string(),
            display_sheet_name,
            drawing_part,
        })
    }

    fn initialize_worksheet(
        xml_document: &Weak<RefCell<XmlDocument>>,
        relationship_part: Rc<RefCell<RelationsPart>>,
    ) -> AnyResult<
        (
            Option<VecDeque<ColumnProperties>>,
            Option<BTreeMap<u32, RowRecords>>,
            // Merge Range
            Option<Vec<ReferenceRange>>,
            // Hyperlinks
            Option<Vec<HyperLinks>>,
            WorkSheetViews,
            Dimension,
        ),
        AnyError,
    > {
        if let Some(xml_document) = xml_document.upgrade() {
            let mut xml_doc_mut = xml_document
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to get XML doc handle")?;
            // unwrap dimension
            let dimension_root_id = xml_doc_mut.get_root_id();
            if let Some(dimension_ids) = xml_doc_mut
                .find_all_child(dimension_root_id, "dimension")
                .context("draviavemal-openxml_office::Failed to find dimension elements")?
            {
                for dimension_id in dimension_ids {
                    xml_doc_mut.remove_element_mut(dimension_id).context(
                        "draviavemal-openxml_office::Failed to remove dimension element",
                    )?;
                }
            }
            let worksheet_views = log_elapsed!(
                || {
                    WorkSheet::deserialize_worksheet_views(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed to deserialize Worksheet View")
                },
                "Worksheet View Deserialization"
            )?;
            // unwrap columns to local collection
            let column_collection = log_elapsed!(
                || {
                    WorkSheet::deserialize_cols(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed To Deserialize Cols")
                },
                "Column deserialize"
            )?;
            // unwrap sheet data into object
            let (sheet_data, dimension) = log_elapsed!(
                || {
                    WorkSheet::deserialize_sheet_data(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed To Deserialize Sheet Data")
                },
                "Sheet Data Deserialize"
            )?;
            let merge_cells = log_elapsed!(
                || {
                    WorkSheet::deserialize_merge_cells(&mut xml_doc_mut)
                        .context("draviavemal-openxml_office::Failed To Deserialize Merge Cells")
                },
                "Merge Cell Deserialize"
            )?;
            let hyperlinks = log_elapsed!(
                || {
                    WorkSheet::deserialize_hyperlinks(&mut xml_doc_mut, &relationship_part)
                        .context("draviavemal-openxml_office::Failed To Deserialize hyperlinks")
                },
                "Hyperlink Deserialize"
            )?;
            Ok((
                column_collection,
                sheet_data,
                merge_cells,
                hyperlinks,
                worksheet_views,
                dimension,
            ))
        } else {
            Ok((
                None,
                None,
                None,
                None,
                WorkSheetViews::default(),
                Dimension::default(),
            ))
        }
    }

    fn serialize_dimension(&mut self, xml_doc_mut: &mut XmlDocument) -> Result<(), AnyError> {
        fn set_default(xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
            let dimension_attribute = vec![XmlAttribute::new("ref".to_string(), "A1".to_string())];
            let root_id = xml_doc_mut.get_root_id();
            xml_doc_mut
                .append_child_element_mut(root_id, "dimension", Some(dimension_attribute))
                .context("draviavemal-openxml_office::Failed to Add Dimension node to worksheet")?;
            Ok(())
        }
        if let Some(sheet_data) = self.sheet_data.as_ref() {
            if let Some(first_item) = sheet_data.first_key_value() {
                let dimension_attribute = vec![XmlAttribute::new(
                    "ref".to_string(),
                    format!(
                        "{}{}:{}{}",
                        ConverterUtil::get_column_ref(self.dimension.start_col).context(
                            "draviavemal-openxml_office::Failed to convert dim col start"
                        )?,
                        first_item.0,
                        ConverterUtil::get_column_ref(self.dimension.end_col)
                            .context("draviavemal-openxml_office::Failed to convert dim col end")?,
                        if let Some(row_end) = sheet_data.last_key_value() {
                            row_end.0
                        } else {
                            first_item.0
                        }
                    ),
                )];
                let root_id = xml_doc_mut.get_root_id();
                xml_doc_mut
                    .append_child_element_mut(root_id, "dimension", Some(dimension_attribute))
                    .context(
                        "draviavemal-openxml_office::Failed to Add Dimension node to worksheet",
                    )?;
            } else {
                set_default(xml_doc_mut)?;
            }
        } else {
            set_default(xml_doc_mut)?;
        }
        Ok(())
    }

    fn serialize_cols(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        if let Some(mut column_collection) = self.column_collection.take() {
            if column_collection.len() > 0 {
                let root_id = xml_doc_mut.get_root_id();
                let cols_id = xml_doc_mut
                    .insert_child_element_after_last_tag_mut(root_id, "cols", "sheetFormatPr", None)
                    .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
                loop {
                    if let Some(item) = column_collection.pop_front() {
                        let mut attribute: Vec<XmlAttribute> = Vec::new();
                        attribute.push(XmlAttribute::new("min".to_string(), item.min.to_string()));
                        attribute.push(XmlAttribute::new("max".to_string(), item.max.to_string()));
                        if let Some(width) = item.width {
                            attribute.push(XmlAttribute::new(
                                "customWidth".to_string(),
                                "1".to_string(),
                            ));
                            attribute
                                .push(XmlAttribute::new("width".to_string(), width.to_string()));
                        }
                        if let Some(style_id) = item.style_id {
                            attribute.push(XmlAttribute::new(
                                "style".to_string(),
                                style_id.id.to_string(),
                            ));
                        }
                        if item.hidden.is_some() {
                            attribute
                                .push(XmlAttribute::new("hidden".to_string(), "1".to_string()));
                        }
                        if item.best_fit.is_some() {
                            attribute
                                .push(XmlAttribute::new("bestFit".to_string(), "1".to_string()));
                        }
                        xml_doc_mut
                            .append_child_element_mut(cols_id, "col", Some(attribute))
                            .context("draviavemal-openxml_office::Failed to insert col record")?;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn serialize_sheet_views(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        let root_id = xml_doc_mut.get_root_id();
        let sheet_views_id = xml_doc_mut
            .insert_child_element_after_last_tag_mut(root_id, "sheetViews", "dimension", None)
            .context("draviavemal-openxml_office::Failed to Insert Sheet Views Element")?;
        loop {
            if let Some(sheet_view) = self.sheet_views.view_collection.pop() {
                let mut attribute: HashMap<String, String> = HashMap::new();
                attribute.insert("workbookViewId".to_string(), sheet_view.workbook_view_id);
                if let Some(window_protection) = sheet_view.window_protection {
                    attribute.insert(
                        "windowProtection".to_string(),
                        ConverterUtil::bool_xml_flag(&window_protection),
                    );
                }
                if let Some(show_formula_bar) = sheet_view.show_formula_bar {
                    attribute.insert(
                        "showFormulas".to_string(),
                        ConverterUtil::bool_xml_flag(&show_formula_bar),
                    );
                }
                if let Some(show_grid_line) = sheet_view.show_grid_line {
                    attribute.insert(
                        "showGridLines".to_string(),
                        ConverterUtil::bool_xml_flag(&show_grid_line),
                    );
                }
                if let Some(show_row_col_header) = sheet_view.show_row_col_header {
                    attribute.insert(
                        "showRowColHeaders".to_string(),
                        ConverterUtil::bool_xml_flag(&show_row_col_header),
                    );
                }
                if let Some(show_zero) = sheet_view.show_zero {
                    attribute.insert(
                        "showZeros".to_string(),
                        ConverterUtil::bool_xml_flag(&show_zero),
                    );
                }
                if let Some(view_right_to_left) = sheet_view.view_right_to_left {
                    attribute.insert(
                        "rightToLeft".to_string(),
                        ConverterUtil::bool_xml_flag(&view_right_to_left),
                    );
                }
                if let Some(tab_selected) = sheet_view.tab_selected {
                    attribute.insert(
                        "tabSelected".to_string(),
                        ConverterUtil::bool_xml_flag(&tab_selected),
                    );
                }
                if let Some(show_ruler) = sheet_view.show_ruler {
                    attribute.insert(
                        "showRuler".to_string(),
                        ConverterUtil::bool_xml_flag(&show_ruler),
                    );
                }
                if let Some(show_white_space) = sheet_view.show_white_space {
                    attribute.insert(
                        "showWhiteSpace".to_string(),
                        ConverterUtil::bool_xml_flag(&show_white_space),
                    );
                }
                if let Some(show_outline_symbol) = sheet_view.show_outline_symbol {
                    attribute.insert(
                        "showOutlineSymbols".to_string(),
                        ConverterUtil::bool_xml_flag(&show_outline_symbol),
                    );
                }
                if let Some(default_grid_color) = sheet_view.default_grid_color {
                    attribute.insert(
                        "defaultGridColor".to_string(),
                        ConverterUtil::bool_xml_flag(&default_grid_color),
                    );
                }
                if let Some(top_left_cell) = sheet_view.top_left_cell {
                    attribute.insert("topLeftCell".to_string(), top_left_cell);
                }
                if let Some(view) = sheet_view.view {
                    attribute.insert("view".to_string(), view);
                }
                if let Some(zoom_scale) = sheet_view.zoom_scale {
                    attribute.insert("zoomScale".to_string(), zoom_scale.to_string());
                }
                if let Some(zoom_scale_normal) = sheet_view.zoom_scale_normal {
                    attribute.insert("zoomScaleNormal".to_string(), zoom_scale_normal.to_string());
                }
                if let Some(zoom_scale_sheet_layout) = sheet_view.zoom_scale_sheet_layout {
                    attribute.insert(
                        "zoomScaleSheetLayoutView".to_string(),
                        zoom_scale_sheet_layout.to_string(),
                    );
                }
                if let Some(zoom_scale_page_layout) = sheet_view.zoom_scale_page_layout {
                    attribute.insert(
                        "zoomScalePageLayoutView".to_string(),
                        zoom_scale_page_layout.to_string(),
                    );
                }
                xml_doc_mut
                    .append_child_element_mut(
                        sheet_views_id,
                        "sheetView",
                        Some(
                            attribute
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed to insert sheetView record")?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn serialize_sheet_data(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        if let Some(sheet_data) = self.sheet_data.take() {
            let root_id = xml_doc_mut.get_root_id();
            let sheet_data_id = xml_doc_mut
                .insert_child_element_after_last_tag_mut(root_id, "sheetData", "cols", None)
                .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
            for (row_index, db_row) in sheet_data {
                let mut row_attribute = HashMap::new();
                row_attribute.insert("r".to_string(), row_index.to_string());
                if let Some(row_span) = db_row.row_property.span {
                    row_attribute.insert("spans".to_string(), row_span);
                }
                if let Some(row_style_id) = db_row.row_property.style_id {
                    row_attribute.insert("customFormat".to_string(), "1".to_string());
                    row_attribute.insert("s".to_string(), row_style_id.id.to_string());
                }
                if let Some(row_height) = db_row.row_property.height {
                    row_attribute.insert("customHeight".to_string(), "1".to_string());
                    row_attribute.insert("ht".to_string(), row_height.to_string());
                }
                if db_row.row_property.hidden.is_some() {
                    row_attribute.insert("hidden".to_string(), "1".to_string());
                }
                if let Some(row_group_level) = db_row.row_property.group_level {
                    row_attribute.insert("outlineLevel".to_string(), row_group_level.to_string());
                }
                if db_row.row_property.collapsed.is_some() {
                    row_attribute.insert("collapsed".to_string(), "1".to_string());
                }
                if db_row.row_property.thick_top.is_some() {
                    row_attribute.insert("thickTop".to_string(), "1".to_string());
                }
                if db_row.row_property.thick_bottom.is_some() {
                    row_attribute.insert("thickBot".to_string(), "1".to_string());
                }
                if db_row.row_property.place_holder.is_some() {
                    row_attribute.insert("ph".to_string(), "1".to_string());
                }
                let row_element_id = xml_doc_mut
                    .append_child_element_mut(
                        sheet_data_id,
                        "row",
                        Some(
                            row_attribute
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed to insert row element")?;
                if let Some(cols) = db_row.cell_records {
                    for (col_index, cell_record) in cols {
                        // Create cell element
                        let mut cell_attribute = HashMap::new();
                        cell_attribute.insert(
                            "r".to_string(),
                            format!(
                                "{}{}",
                                ConverterUtil::get_column_ref(col_index).context(
                                    "draviavemal-openxml_office::Failed to get Char Id from Int"
                                )?,
                                row_index
                            ),
                        );
                        if let Some(cell_style_id) = cell_record.style_id {
                            cell_attribute.insert("s".to_string(), cell_style_id.id.to_string());
                        }
                        if cell_record.data_type != CellDataType::Number {
                            cell_attribute.insert(
                                "t".to_string(),
                                CellDataType::get_string(cell_record.data_type),
                            );
                        }
                        if let Some(cell_comment_id) = cell_record.comment_id {
                            cell_attribute.insert("cm".to_string(), cell_comment_id.to_string());
                        }
                        if let Some(cell_metadata) = cell_record.metadata {
                            cell_attribute.insert("vm".to_string(), cell_metadata.to_string());
                        }
                        if cell_record.place_holder.is_some() {
                            cell_attribute.insert("ph".to_string(), "1".to_string());
                        }
                        let cell_id = xml_doc_mut
                            .append_child_element_mut(
                                row_element_id,
                                "c",
                                Some(
                                    cell_attribute
                                        .into_iter()
                                        .map(|(k, v)| XmlAttribute::new(k, v))
                                        .collect(),
                                ),
                            )
                            .context("draviavemal-openxml_office::Failed to insert row element")?;
                        // Create cell's child element
                        match cell_record.data_type {
                            CellDataType::InlineString => {
                                let inline_string_id = xml_doc_mut
                                    .append_child_element_mut(cell_id, "is", None)
                                    .context("draviavemal-openxml_office::Failed to insert Inline string element")?;
                                let text_id = xml_doc_mut
                                    .append_child_element_mut(inline_string_id, "t", None)
                                    .context("draviavemal-openxml_office::Failed To insert Text Value to inline string")?;
                                xml_doc_mut
                                    .get_element_mut(text_id)
                                    .context("draviavemal-openxml_office::Failed to get text element")?
                                    .add_text_mut(&if let Some(value) = cell_record.value {
                                        value
                                    } else {
                                        "".to_string()
                                    })
                                    .context("draviavemal-openxml_office::Failed to add text to inline string")?;
                            }
                            _ => {
                                if let Some(formula) = cell_record.formula {
                                    let formula_id = xml_doc_mut
                                        .append_child_element_mut(cell_id, "f", None)
                                        .context("draviavemal-openxml_office::Failed to insert Inline string element")?;
                                    xml_doc_mut
                                        .get_element_mut(formula_id)
                                        .context("draviavemal-openxml_office::Failed to get formula element")?
                                        .add_text_mut(&formula)
                                        .context("draviavemal-openxml_office::Failed to add formula text")?;
                                }
                                if let Some(value) = cell_record.value {
                                    let value_id = xml_doc_mut
                                        .append_child_element_mut(cell_id, "v", None)
                                        .context("draviavemal-openxml_office::Failed to insert Inline string element")?;
                                    xml_doc_mut
                                        .get_element_mut(value_id)
                                        .context("draviavemal-openxml_office::Failed to get value element")?
                                        .add_text_mut(&value)
                                        .context("draviavemal-openxml_office::Failed to add value text")?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn serialize_merge_cells(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        if let Some(merge_cells) = self.merge_cells.take() {
            let root_id = xml_doc_mut.get_root_id();
            let merge_cells_id = xml_doc_mut
                .insert_child_element_after_last_tag_mut(root_id, "mergeCells", "sheetData", None)
                .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
            {
                let merge_cells_element = xml_doc_mut
                    .get_element_mut(merge_cells_id)
                    .context("draviavemal-openxml_office::Failed to get element")?;
                merge_cells_element
                    .add_attribute_mut("count", &merge_cells.len().to_string())
                    .context("draviavemal-openxml_office::Failed to set Merge Cells Attribute")?;
            }
            for merge_cell in merge_cells {
                let mut attribute = HashMap::new();
                if merge_cell.row_start == merge_cell.row_end
                    && merge_cell.column_start == merge_cell.column_end
                {
                    attribute.insert(
                        "ref".to_string(),
                        ConverterUtil::get_cell_ref(merge_cell.row_start, merge_cell.column_start)?,
                    );
                } else {
                    attribute.insert(
                        "ref".to_string(),
                        format!(
                            "{}:{}",
                            ConverterUtil::get_cell_ref(
                                merge_cell.row_start,
                                merge_cell.column_start
                            )?,
                            ConverterUtil::get_cell_ref(merge_cell.row_end, merge_cell.column_end)?,
                        ),
                    );
                }
                xml_doc_mut
                    .append_child_element_mut(
                        merge_cells_id,
                        "mergeCell",
                        Some(
                            attribute
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed to add MergeCell Node")?;
            }
        }
        Ok(())
    }

    fn serialize_hyperlinks(
        &mut self,
        xml_doc_mut: &mut XmlDocument,
        relationship_part: Rc<RefCell<RelationsPart>>,
    ) -> AnyResult<(), AnyError> {
        if let Some(hyperlinks) = self.hyperlinks.take() {
            let root_id = xml_doc_mut.get_root_id();
            let hyperlinks_id = xml_doc_mut
                .insert_child_element_after_last_tag_mut(root_id, "hyperlinks", "mergeCells", None)
                .context("draviavemal-openxml_office::Failed to Insert Cols Element")?;
            for hyperlink in hyperlinks {
                let mut attributes = HashMap::new();
                if hyperlink.range.row_start == hyperlink.range.row_end
                    && hyperlink.range.column_start == hyperlink.range.column_end
                {
                    attributes.insert(
                        "ref".to_string(),
                        ConverterUtil::get_cell_ref(
                            hyperlink.range.row_start,
                            hyperlink.range.column_start,
                        )?,
                    );
                } else {
                    attributes.insert(
                        "ref".to_string(),
                        format!(
                            "{}:{}",
                            ConverterUtil::get_cell_ref(
                                hyperlink.range.row_start,
                                hyperlink.range.column_start
                            )?,
                            ConverterUtil::get_cell_ref(
                                hyperlink.range.row_end,
                                hyperlink.range.column_end
                            )?,
                        ),
                    );
                }
                if let Some(display_value) = hyperlink.display {
                    attributes.insert("display".to_string(), display_value);
                }
                // Insert Relationship link
                if hyperlink.id.is_some() {
                    let content = COMMON_TYPE_COLLECTION
                        .get("hyperlink")
                        .context("Failed to read Common type collection")?;
                    let r_id = relationship_part
                        .borrow_mut()
                        .set_new_relationship_mut(&content, hyperlink.link)
                        .context(
                            "draviavemal-openxml_office::Failed to Create Hyperlink Relationship",
                        )?;
                    attributes.insert("r:id".to_string(), r_id);
                } else {
                    attributes.insert("location".to_string(), hyperlink.link);
                }
                xml_doc_mut
                    .append_child_element_mut(
                        hyperlinks_id,
                        "hyperlink",
                        Some(
                            attributes
                                .into_iter()
                                .map(|(k, v)| XmlAttribute::new(k, v))
                                .collect(),
                        ),
                    )
                    .context("draviavemal-openxml_office::Failed tp Add element")?;
            }
        }
        Ok(())
    }

    fn deserialize_cols(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<Option<VecDeque<ColumnProperties>>, AnyError> {
        let root_id = xml_doc_mut.get_root_id();
        if let Some(cols_id) = xml_doc_mut
            .find_first_child(root_id, "cols")
            .context("draviavemal-openxml_office::Failed to find cols element")?
        {
            let child_ids: Vec<NodeId> = xml_doc_mut
                .get_element(cols_id)
                .context("draviavemal-openxml_office::Failed to get cols element")?
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
            let mut column_collection = VecDeque::with_capacity(child_ids.len());
            for col_id in child_ids {
                let col = xml_doc_mut
                    .get_element(col_id)
                    .context("draviavemal-openxml_office::Failed to get col element")?;
                let mut column_properties = ColumnProperties::default();
                if let Some(min) = col
                    .get_attribute("min")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.min = min
                        .parse()
                        .context("draviavemal-openxml_office::Failed to parse min value")?;
                }
                if let Some(max) = col
                    .get_attribute("max")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.max = max
                        .parse()
                        .context("draviavemal-openxml_office::Failed to parse min value")?;
                }
                if let Some(best_fit) = col
                    .get_attribute("bestFit")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.best_fit = if best_fit == "1" { Some(true) } else { None }
                }
                if let Some(hidden) = col
                    .get_attribute("hidden")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.hidden = if hidden == "1" { Some(true) } else { None }
                }
                if let Some(style) = col
                    .get_attribute("style")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.style_id =
                        Some(StyleId::new(style.parse().context(
                            "draviavemal-openxml_office::Failed to parse style ID",
                        )?));
                }
                if let Some(outline_level) = col
                    .get_attribute("outlineLevel")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.group_level = outline_level
                        .parse()
                        .context("draviavemal-openxml_office::Failed to parse style ID")?;
                }
                if let Some(custom_width) = col
                    .get_attribute("customWidth")
                    .map(|attribute| attribute.get_value())
                {
                    if custom_width == "1" {
                        column_properties.width = Some(
                            col.get_attribute("width")
                                .map(|attribute| attribute.get_value())
                                .context("draviavemal-openxml_office::Failed to get custom width")?
                                .parse()
                                .context(
                                    "draviavemal-openxml_office::Failed to parse custom width",
                                )?,
                        );
                    }
                }
                if let Some(collapsed) = col
                    .get_attribute("collapsed")
                    .map(|attribute| attribute.get_value())
                {
                    column_properties.collapsed = if collapsed == "1" { Some(true) } else { None }
                }
                column_collection.push_back(column_properties);
            }
            xml_doc_mut
                .remove_element_mut(cols_id)
                .context("draviavemal-openxml_office::Failed to remove cols element")?;
            return Ok(Some(column_collection));
        }
        Ok(None)
    }

    /// DeSerializing Worksheet View
    fn deserialize_worksheet_views(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<WorkSheetViews, AnyError> {
        let mut worksheet_views = WorkSheetViews::default();
        let root_id = xml_doc_mut.get_root_id();
        if let Some(sheet_views_id) = xml_doc_mut
            .find_first_child(root_id, "sheetViews")
            .context("draviavemal-openxml_office::Failed to find sheetViews element")?
        {
            let child_records: Vec<(NodeId, String)> = xml_doc_mut
                .get_element(sheet_views_id)
                .context("draviavemal-openxml_office::Failed to get sheetViews element")?
                .get_child_contents()
                .as_ref()
                .map(|contents| {
                    contents
                        .iter()
                        .filter_map(|content| match content {
                            XmlElementContentType::Element((id, tag, _)) => {
                                Some((*id, tag.clone()))
                            }
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_default();
            for (element_id, element_tag) in child_records {
                // Validate element that are not accounted
                if element_tag != "sheetView" {
                    return Err(AnyError::msg(
                        "draviavemal-openxml_office::Failed to Process Sheet Views child",
                    ));
                }
                let sheet_view_element = xml_doc_mut
                    .get_element(element_id)
                    .context("draviavemal-openxml_office::Failed to get Sheet View Element")?;
                let mut worksheet_view = WorkSheetView::default();
                worksheet_view.workbook_view_id = sheet_view_element
                    .get_attribute("workbookViewId")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Mandatory attribute \"workbookViewId\" is missing from sheetView")?
                    .to_string();
                // Windows protection
                if let Some(window_protection) = sheet_view_element
                    .get_attribute("windowProtection")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.window_protection = Some(
                        ConverterUtil::normalize_bool_property_bool(window_protection),
                    );
                }
                // Show formula
                if let Some(show_formula_bar) = sheet_view_element
                    .get_attribute("showFormulas")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_formula_bar = Some(
                        ConverterUtil::normalize_bool_property_bool(show_formula_bar),
                    );
                }
                // Show Grid Line
                if let Some(show_grid_line) = sheet_view_element
                    .get_attribute("showGridLines")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_grid_line =
                        Some(ConverterUtil::normalize_bool_property_bool(show_grid_line));
                }
                // Show row column header
                if let Some(show_row_col_header) = sheet_view_element
                    .get_attribute("showRowColHeaders")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_row_col_header = Some(
                        ConverterUtil::normalize_bool_property_bool(show_row_col_header),
                    );
                }
                // Show Zero
                if let Some(show_zero) = sheet_view_element
                    .get_attribute("showZeros")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_zero =
                        Some(ConverterUtil::normalize_bool_property_bool(show_zero));
                }
                // Right to Left
                if let Some(view_right_to_left) = sheet_view_element
                    .get_attribute("rightToLeft")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.view_right_to_left = Some(
                        ConverterUtil::normalize_bool_property_bool(view_right_to_left),
                    );
                }
                // Tab Selected
                if let Some(tab_selected) = sheet_view_element
                    .get_attribute("tabSelected")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.tab_selected =
                        Some(ConverterUtil::normalize_bool_property_bool(tab_selected));
                }
                // show ruler
                if let Some(show_ruler) = sheet_view_element
                    .get_attribute("showRuler")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_ruler =
                        Some(ConverterUtil::normalize_bool_property_bool(show_ruler));
                }
                // show white space
                if let Some(show_white_space) = sheet_view_element
                    .get_attribute("showWhiteSpace")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_white_space = Some(
                        ConverterUtil::normalize_bool_property_bool(show_white_space),
                    );
                }
                // Show outlined Symbols
                if let Some(show_outline_symbol) = sheet_view_element
                    .get_attribute("showOutlineSymbols")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.show_outline_symbol = Some(
                        ConverterUtil::normalize_bool_property_bool(show_outline_symbol),
                    );
                }
                // Default Grid Color
                if let Some(default_grid_color) = sheet_view_element
                    .get_attribute("defaultGridColor")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.default_grid_color = Some(
                        ConverterUtil::normalize_bool_property_bool(default_grid_color),
                    );
                }
                // Top Left Cell
                if let Some(top_left_cell) = sheet_view_element
                    .get_attribute("topLeftCell")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.top_left_cell = Some(top_left_cell.to_string());
                }
                // View Setting
                if let Some(view) = sheet_view_element
                    .get_attribute("view")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.view = Some(view.to_string());
                }
                // Zoom Scale
                if let Some(zoom_scale) = sheet_view_element
                    .get_attribute("zoomScale")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale = Some(zoom_scale.parse().context(
                        "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                    )?);
                }
                // Zoom Scale Normal
                if let Some(zoom_scale_normal) = sheet_view_element
                    .get_attribute("zoomScaleNormal")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale_normal = Some(zoom_scale_normal.parse().context(
                        "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                    )?);
                }
                // Zoom Scale Sheet Layout View
                if let Some(zoom_scale_sheet_layout) = sheet_view_element
                    .get_attribute("zoomScaleSheetLayoutView")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale_sheet_layout =
                        Some(zoom_scale_sheet_layout.parse().context(
                            "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                        )?);
                }
                // Zoom Scale Page Layout View
                if let Some(zoom_scale_page_layout) = sheet_view_element
                    .get_attribute("zoomScalePageLayoutView")
                    .map(|attribute| attribute.get_value())
                {
                    worksheet_view.zoom_scale_page_layout =
                        Some(zoom_scale_page_layout.parse().context(
                            "draviavemal-openxml_office::Failed to Convert Zoom Normal to i16",
                        )?);
                }
                worksheet_views.view_collection.push(worksheet_view);
            }
            xml_doc_mut
                .remove_element_mut(sheet_views_id)
                .context("draviavemal-openxml_office::Failed to remove sheetViews element")?;
        }
        Ok(worksheet_views)
    }

    /// Deserialize Sheet Data
    fn deserialize_sheet_data(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(Option<BTreeMap<u32, RowRecords>>, Dimension), AnyError> {
        let mut dimension = Dimension::default();
        let root_id = xml_doc_mut.get_root_id();
        if let Some(sheet_data_id) = xml_doc_mut
            .find_first_child(root_id, "sheetData")
            .context("draviavemal-openxml_office::Failed to find sheetData element")?
        {
            let mut sheet_data_collection: BTreeMap<u32, RowRecords> = BTreeMap::new();
            let row_ids: Vec<NodeId> = xml_doc_mut
                .get_element(sheet_data_id)
                .context("draviavemal-openxml_office::Failed to get sheetData element")?
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
            // Loop All rows of sheet data
            for row_id in row_ids {
                let mut row_record = RowProperties::default();
                let row_element = xml_doc_mut
                    .get_element(row_id)
                    .context("draviavemal-openxml_office::Failed to get row element")?;
                // Get Row Id
                let row_index: u32 = row_element
                    .get_attribute("r")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Missing mandatory row id attribute")?
                    .parse()
                    .context("draviavemal-openxml_office::Failed to parse row id")?;
                if let Some(row_span) = row_element
                    .get_attribute("spans")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.span = Some(row_span.to_string());
                }
                if let Some(style_id) = row_element
                    .get_attribute("s")
                    .map(|attribute| attribute.get_value())
                {
                    if let Some(custom_formant) = row_element
                        .get_attribute("customFormat")
                        .map(|attribute| attribute.get_value())
                    {
                        row_record.style_id = if custom_formant == "1" {
                            Some(StyleId::new(style_id.parse().context(
                                "draviavemal-openxml_office::Failed to parse the row style id",
                            )?))
                        } else {
                            None
                        };
                    }
                }
                if let Some(hidden) = row_element
                    .get_attribute("hidden")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.hidden = if hidden == "1" { Some(true) } else { None };
                }
                if let Some(height) = row_element
                    .get_attribute("ht")
                    .map(|attribute| attribute.get_value())
                {
                    if let Some(custom_height) = row_element
                        .get_attribute("customHeight")
                        .map(|attribute| attribute.get_value())
                    {
                        row_record.height = if custom_height == "1" {
                            Some(height.parse().context(
                                "draviavemal-openxml_office::Failed to parse the row height",
                            )?)
                        } else {
                            None
                        };
                    }
                }
                if let Some(row_group_level) = row_element
                    .get_attribute("outlineLevel")
                    .map(|attribute| attribute.get_value())
                {
                    let outline_level = row_group_level.parse().context(
                        "draviavemal-openxml_office::Failed to parse the row group level",
                    )?;
                    row_record.group_level = if outline_level > 0 {
                        Some(outline_level)
                    } else {
                        None
                    };
                }
                if let Some(collapsed) = row_element
                    .get_attribute("collapsed")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.collapsed = if collapsed == "1" { Some(true) } else { None };
                }
                if let Some(thick_top) = row_element
                    .get_attribute("thickTop")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.thick_top = if thick_top == "1" { Some(true) } else { None };
                }
                if let Some(thick_bottom) = row_element
                    .get_attribute("thickBot")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.thick_bottom = if thick_bottom == "1" {
                        Some(true)
                    } else {
                        None
                    };
                }
                if let Some(place_holder) = row_element
                    .get_attribute("ph")
                    .map(|attribute| attribute.get_value())
                {
                    row_record.place_holder = if place_holder == "1" {
                        Some(true)
                    } else {
                        None
                    };
                }
                let mut cell_records: BTreeMap<ColumnIndex, CellProperty> = BTreeMap::new();
                let col_ids: Vec<NodeId> = xml_doc_mut
                    .get_element(row_id)
                    .context("draviavemal-openxml_office::Failed to get row element")?
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
                // Loop All Columns of row
                for col_id in col_ids {
                    let mut cell_record = CellProperty::default();
                    let col_element = xml_doc_mut
                        .get_element(col_id)
                        .context("draviavemal-openxml_office::Failed to get col element")?;
                    // Get Col Id
                    let col_index = ConverterUtil::get_column_index(
                        col_element
                            .get_attribute("r")
                            .map(|attribute| attribute.get_value())
                            .context(
                                "draviavemal-openxml_office::Missing mandatory col id attribute",
                            )?,
                    )
                    .context(
                        "draviavemal-openxml_office::Failed to Convert col worksheet initialize",
                    )?;
                    if let Some(style_id) = col_element
                        .get_attribute("s")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.style_id = Some(StyleId::new(style_id.parse().context(
                            "draviavemal-openxml_office::Failed to parse the col style id",
                        )?));
                    }
                    if let Some(cell_type) = col_element
                        .get_attribute("t")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.data_type = CellDataType::get_enum(cell_type);
                    } else {
                        cell_record.data_type = CellDataType::Number;
                    }
                    if let Some(comment_id) = col_element
                        .get_attribute("cm")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.comment_id = Some(comment_id.parse().context(
                            "draviavemal-openxml_office::Failed to parse the col comment id",
                        )?);
                    };
                    if let Some(value_meta_id) = col_element
                        .get_attribute("vm")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.metadata = Some(value_meta_id.parse().context(
                            "draviavemal-openxml_office::Failed to parse the col value meta id",
                        )?);
                    };
                    if let Some(place_holder) = col_element
                        .get_attribute("ph")
                        .map(|attribute| attribute.get_value())
                    {
                        cell_record.place_holder = if place_holder == "1" {
                            Some(true)
                        } else {
                            None
                        };
                    };
                    let cell_child_records: Vec<(NodeId, String)> = xml_doc_mut
                        .get_element(col_id)
                        .context("draviavemal-openxml_office::Failed to get col element")?
                        .get_child_contents()
                        .as_ref()
                        .map(|contents| {
                            contents
                                .iter()
                                .filter_map(|content| match content {
                                    XmlElementContentType::Element((id, tag, _)) => {
                                        Some((*id, tag.clone()))
                                    }
                                    _ => None,
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    for (cell_child_id, cell_child_tag) in cell_child_records {
                        match cell_child_tag.as_str() {
                            "v" => {
                                cell_record.value =
                                    WorkSheet::get_element_text(xml_doc_mut, cell_child_id)?;
                            }
                            "f" => {
                                cell_record.formula =
                                    WorkSheet::get_element_text(xml_doc_mut, cell_child_id)?;
                            }
                            "is" => {
                                if let Some(text_id) = xml_doc_mut
                                    .find_first_child(cell_child_id, "t")
                                    .context("draviavemal-openxml_office::Failed to find inline string text element")?
                                {
                                    cell_record.value =
                                        WorkSheet::get_element_text(xml_doc_mut, text_id)?;
                                }
                            }
                            _ => {
                                return Err(AnyError::msg("draviavemal-openxml_office::Found un-know element cell child"));
                            }
                        }
                    }
                    dimension.start_col = min(dimension.start_col, col_index);
                    dimension.end_col = max(dimension.end_col, col_index);
                    cell_records.insert(col_index, cell_record);
                }
                sheet_data_collection.insert(
                    row_index,
                    RowRecords {
                        row_property: row_record,
                        cell_records: if cell_records.len() > 0 {
                            Some(cell_records)
                        } else {
                            None
                        },
                    },
                );
            }
            xml_doc_mut
                .remove_element_mut(sheet_data_id)
                .context("draviavemal-openxml_office::Failed to remove sheetData element")?;
            return Ok((Some(sheet_data_collection), dimension));
        }
        Ok((None, dimension))
    }

    /// Reads the first text content of an element, returning None if there is no text.
    fn get_element_text(
        xml_doc_mut: &XmlDocument,
        element_id: NodeId,
    ) -> AnyResult<Option<String>, AnyError> {
        Ok(xml_doc_mut
            .get_element(element_id)
            .context("draviavemal-openxml_office::Failed to get element for text read")?
            .get_child_contents()
            .as_ref()
            .and_then(|contents| {
                contents.iter().find_map(|content| match content {
                    XmlElementContentType::Text(text) => Some(text.clone()),
                    _ => None,
                })
            }))
    }

    /// Deserialize Merge Cell Collection
    fn deserialize_merge_cells(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<Option<Vec<ReferenceRange>>> {
        let root_id = xml_doc_mut.get_root_id();
        if let Some(merge_cells_id) = xml_doc_mut
            .find_first_child(root_id, "mergeCells")
            .context("draviavemal-openxml_office::Failed to find mergeCells element")?
        {
            let mut merge_cell_collection = Vec::new();
            let merge_cell_ids: Vec<NodeId> = xml_doc_mut
                .get_element(merge_cells_id)
                .context("draviavemal-openxml_office::Failed to get mergeCells element")?
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
            for merge_cell_id in merge_cell_ids {
                let merge_cell_element = xml_doc_mut
                    .get_element(merge_cell_id)
                    .context("draviavemal-openxml_office::Failed to Get Element")?;
                let merge_range = merge_cell_element
                    .get_attribute("ref")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Failed to get merge ref")?;
                if merge_range.contains(':') {
                    let range: Vec<&str> = merge_range.split(':').collect();
                    let (row_start, column_start) = ConverterUtil::get_cell_index(range[0])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    let (row_end, column_end) = ConverterUtil::get_cell_index(range[1])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    merge_cell_collection.push(ReferenceRange {
                        column_start,
                        row_start,
                        column_end,
                        row_end,
                    });
                } else {
                    let (row, col) = ConverterUtil::get_cell_index(merge_range)
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    merge_cell_collection.push(ReferenceRange {
                        column_start: col,
                        row_start: row,
                        column_end: col,
                        row_end: row,
                    });
                }
            }
            xml_doc_mut
                .remove_element_mut(merge_cells_id)
                .context("draviavemal-openxml_office::Failed to remove mergeCells element")?;
            if merge_cell_collection.len() > 0 {
                Ok(Some(merge_cell_collection))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Deserialize Hyperlink Collection
    fn deserialize_hyperlinks(
        xml_doc_mut: &mut XmlDocument,
        relationship_part: &Rc<RefCell<RelationsPart>>,
    ) -> AnyResult<Option<Vec<HyperLinks>>> {
        let root_id = xml_doc_mut.get_root_id();
        if let Some(hyperlinks_id) = xml_doc_mut
            .find_first_child(root_id, "hyperlinks")
            .context("draviavemal-openxml_office::Failed to find hyperlinks element")?
        {
            let mut hyperlink_collection = Vec::new();
            let hyperlink_ids: Vec<NodeId> = xml_doc_mut
                .get_element(hyperlinks_id)
                .context("draviavemal-openxml_office::Failed to get hyperlinks element")?
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
            for hyperlink_element_id in hyperlink_ids {
                let hyperlink_element = xml_doc_mut
                    .get_element(hyperlink_element_id)
                    .context("draviavemal-openxml_office::Failed to Get Element")?;
                let display = hyperlink_element
                    .get_attribute("display")
                    .map(|attribute| attribute.get_value().to_string());
                let hyperlink_ref = hyperlink_element
                    .get_attribute("ref")
                    .map(|attribute| attribute.get_value())
                    .context("draviavemal-openxml_office::Failed to get hyperlink ref")?;
                let hyperlink_id = hyperlink_element
                    .get_attribute_by_uri(RELATIONSHIPS_NS.uri, "id")
                    .map(|attribute| attribute.get_value().to_string());
                let range_reference = if hyperlink_ref.contains(':') {
                    let range: Vec<&str> = hyperlink_ref.split(':').collect();
                    let (row_start, column_start) = ConverterUtil::get_cell_index(range[0])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    let (row_end, column_end) = ConverterUtil::get_cell_index(range[1])
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    ReferenceRange {
                        column_start,
                        row_start,
                        column_end,
                        row_end,
                    }
                } else {
                    let (row, col) = ConverterUtil::get_cell_index(hyperlink_ref)
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?;
                    ReferenceRange {
                        column_start: col,
                        row_start: row,
                        column_end: col,
                        row_end: row,
                    }
                };
                // If relationship ID exist pull from relationship part
                let link = if let Some(id) = hyperlink_id.as_ref() {
                    let link = relationship_part
                        .borrow()
                        .get_target_by_id(id)
                        .context("draviavemal-openxml_office::Failed to Pull Target From Relationship file")?
                        .context("draviavemal-openxml_office::No Target Found in the relationship")?;
                    relationship_part
                        .borrow_mut()
                        .delete_relationship_by_id_mut(&id);
                    link
                } else {
                    hyperlink_element
                        .get_attribute("location")
                        .map(|attribute| attribute.get_value())
                        .context("draviavemal-openxml_office::Failed to Get Internal Location")?
                        .to_string()
                };
                hyperlink_collection.push(HyperLinks {
                    id: hyperlink_id,
                    display,
                    link,
                    range: range_reference,
                });
            }
            xml_doc_mut
                .remove_element_mut(hyperlinks_id)
                .context("draviavemal-openxml_office::Failed to remove hyperlinks element")?;
            if hyperlink_collection.len() > 0 {
                Ok(Some(hyperlink_collection))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn get_sheet_file_name(
        display_sheet_name: Option<String>,
        office_document: &Weak<RefCell<OfficeDocument>>,
        sheet_collection: &Weak<RefCell<Vec<(String, String, bool)>>>,
        workbook_relationship_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<(String, String), AnyError> {
        let worksheet_content = EXCEL_TYPE_COLLECTION
            .get("worksheet")
            .context("Failed to read Excel type collection")?;
        if let Some(sheet_collection) = sheet_collection.upgrade() {
            if let Some(workbook_relationship_part) = workbook_relationship_part.upgrade() {
                if let Some(sheet_name) = display_sheet_name.clone() {
                    // If the Sheet name already exist get the path of sheet name
                    if let Some((_, rel_id, _)) = sheet_collection
                        .try_borrow()
                        .context("draviavemal-openxml_office::Failed to Get Sheet Collection")?
                        .iter()
                        .find(|item| item.0 == sheet_name)
                    {
                        return Ok((
                            workbook_relationship_part
                                .try_borrow()
                                .context("draviavemal-openxml_office::Failed to Get Workbook relationship")?
                                .get_target_path_by_id(&rel_id)
                                .context("draviavemal-openxml_office::Failed to Get Target Path")?
                                .context("draviavemal-openxml_office::Failed to Get Relationship path")?,
                            sheet_name,
                        ));
                    }
                }
                let relative_path = workbook_relationship_part
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to pull relationship connection")?
                    .get_relative_path()
                    .context("draviavemal-openxml_office::Get Relative Path for Part File")?;
                let file_path = format!("{}{}", relative_path, worksheet_content.default_path);
                let file_number = if let Some(office_doc) = office_document.upgrade() {
                    office_doc
                        .try_borrow()
                        .context("draviavemal-openxml_office::Failed to Borrow Document")?
                        .get_next_part_number(
                            &file_path,
                            worksheet_content.default_name,
                            worksheet_content.extension,
                        )
                } else {
                    sheet_collection
                        .try_borrow()
                        .context(
                            "draviavemal-openxml_office::Failed to pull Sheet Name Collection",
                        )?
                        .len()
                        + 1
                };
                let sheet_name = format!("{}{}", worksheet_content.default_name, &file_number);
                let relationship_id = workbook_relationship_part
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed to Get Relationship Handle")?
                    .set_new_relationship_path_mut(
                        worksheet_content,
                        Some(file_path.clone()),
                        Some(sheet_name.clone()),
                    )
                    .context(
                        "draviavemal-openxml_office::Setting New Worksheet Relationship Failed.",
                    )?;
                sheet_collection
                    .try_borrow_mut()
                    .context("draviavemal-openxml_office::Failed To pull Sheet Collection Handle")?
                    .push((
                        display_sheet_name.clone().unwrap_or(sheet_name.clone()),
                        relationship_id,
                        false,
                    ));
                return Ok((
                    format!(
                        "{}/{}.{}",
                        file_path, &sheet_name, worksheet_content.extension
                    ),
                    display_sheet_name.clone().unwrap_or(sheet_name.clone()),
                ));
            }
        }
        Err(AnyError::msg(
            "draviavemal-openxml_office::Failed to upgrade relation part",
        ))
    }

    fn update_share_string(&mut self, cell_value: &String) -> AnyResult<String, AnyError> {
        if let Some(common_service) = self.common_service.upgrade() {
            common_service
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Get Share String Handle")?
                .get_string_id_mut(cell_value.to_owned())
                .context("draviavemal-openxml_office::Failed to get share string id")
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to update Share String Record",
            ))
        }
    }

    fn extract_cell_records(
        &self,
        range: &ReferenceRange,
        data_range: &mut Vec<CellPackage>,
        row_index: &RowIndex,
        row_records: &RowRecords,
    ) -> Result<(), AnyError> {
        if let Some(cols) = row_records.cell_records.as_ref() {
            if range.column_end == 0 {
                for (column_index, cell_property) in cols.range(range.column_start..) {
                    data_range.push(CellPackage {
                        cell_ref: ConverterUtil::get_cell_ref(
                            row_index.clone(),
                            column_index.clone(),
                        )
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?,
                        row_index: row_index.clone(),
                        column_index: column_index.clone(),
                        cell_property: self.normalize_cell_property(cell_property)?,
                    });
                }
            } else {
                for (column_index, cell_property) in
                    cols.range(range.column_start..=range.column_end)
                {
                    data_range.push(CellPackage {
                        cell_ref: ConverterUtil::get_cell_ref(
                            row_index.clone(),
                            column_index.clone(),
                        )
                        .context("draviavemal-openxml_office::Failed to parse Cell Ref")?,
                        row_index: row_index.clone(),
                        column_index: column_index.clone(),
                        cell_property: self.normalize_cell_property(cell_property)?,
                    });
                }
            }
        }
        Ok(())
    }

    fn normalize_cell_property(&self, cell_prop: &CellProperty) -> Result<CellProperty, AnyError> {
        let mut parsed_property = cell_prop.clone();
        if parsed_property.data_type == CellDataType::ShareString {
            if let Some(cmn_service) = self.common_service.upgrade() {
                let actual_value = cmn_service
                    .borrow()
                    .get_string_id_value(parsed_property.value.context("Failed to get cell value")?)
                    .context("draviavemal-openxml_office::Failed to normalize share string")?;
                parsed_property.value = Some(actual_value);
            }
        }
        Ok(parsed_property)
    }
}

// ##################################### Mut Feature Function ################################
impl WorkSheet {
    /// Set Active cell of the current sheet
    pub(crate) fn set_active_cell_mut(&mut self, _selected_range: Vec<&str>) {}

    pub fn add_picture(
        &mut self,
        image_path: &str,
        picture_setting: ExcelPictureSetting,
    ) -> Result<(), AnyError> {
        Ok(())
    }

    /// Set Column property
    /// # Arguments
    /// - `cell_ref` (`&str`) - Provide column reference name.
    /// - `column_properties` (`Option<ColumnProperties>`) - Set the current column property.
    pub fn set_column_ref_properties_mut(
        &mut self,
        cell_ref: &str,
        column_properties: Option<ColumnProperties>,
    ) -> AnyResult<(), AnyError> {
        let col_index = ConverterUtil::get_column_index(cell_ref)
            .context("draviavemal-openxml_office::Failed to Get Column index from reference")?;
        self.set_column_index_properties_mut(&col_index, column_properties)
    }

    /// Set Column property
    /// # Arguments
    /// - `col_index` (`&u16`) - Provide column index. Starts From 1.
    /// - `column_properties` (`Option<ColumnProperties>`) - Set the current column property.
    pub fn set_column_index_properties_mut(
        &mut self,
        col_index: &ColumnIndex,
        column_properties: Option<ColumnProperties>,
    ) -> AnyResult<(), AnyError> {
        if self.column_collection.is_none() {
            self.column_collection = Some(VecDeque::new());
        }
        if let Some(column_collection) = self.column_collection.as_mut() {
            let mut new_ranges = VecDeque::new();
            // Delete Old Record
            column_collection.retain_mut(|range| {
                if range.min == *col_index && range.max == *col_index {
                    // Fully matched range, remove it
                    return false;
                } else if range.min <= *col_index && *col_index <= range.max {
                    // Value lies within the range
                    if range.min == *col_index {
                        // Trim the start
                        range.min = col_index + 1;
                    } else if range.max == *col_index {
                        // Trim the end
                        range.max = col_index - 1;
                    } else {
                        // Split the range
                        new_ranges.push_back(ColumnProperties {
                            min: col_index + 1,
                            max: range.max,
                            ..ColumnProperties::default()
                        });
                        range.max = col_index - 1;
                    }
                }
                true
            });
            if let Some(column_properties) = column_properties {
                column_collection.push_back(ColumnProperties {
                    max: *col_index,
                    min: *col_index,
                    ..column_properties
                });
            }
            column_collection.append(&mut new_ranges);
        }
        Ok(())
    }

    /// Set/Reset Row property.
    /// # Arguments
    /// - `row_index` (`&u32`) - Provide row index. Starts From 1
    /// - `row_properties` (`RowProperties`) - Pass Default value or Setting for row
    /// Warning: 0 value will be ignored
    pub fn set_row_index_properties_mut(
        &mut self,
        row_index: &RowIndex,
        row_properties: RowProperties,
    ) -> AnyResult<(), AnyError> {
        if let Some(sheet_data) = self.sheet_data.as_mut() {
            if let Some(row) = sheet_data.get_mut(row_index) {
                row.row_property = row_properties;
            } else {
                sheet_data.insert(
                    *row_index,
                    RowRecords {
                        row_property: row_properties,
                        cell_records: None,
                    },
                );
            }
        } else {
            let mut map = BTreeMap::new();
            map.insert(
                *row_index,
                RowRecords {
                    row_property: row_properties,
                    cell_records: None,
                },
            );
            self.sheet_data = Some(map);
        }
        Ok(())
    }

    /// Set data for same row multiple columns along with row property
    /// # Arguments
    /// - `cell_ref` (`&str`) -  Provide column reference name.
    /// - `column_cell` (`Vec<CellProperties>`) - Set the list of column values auto increment from start ref.
    pub fn set_cell_ref_value_mut(
        &mut self,
        cell_ref: &str,
        column_cells: Vec<CellProperty>,
    ) -> AnyResult<(), AnyError> {
        let (row_index, col_index) = ConverterUtil::get_cell_index(cell_ref)
            .context("draviavemal-openxml_office::Failed to extract cell key")?;
        self.set_cell_index_value_mut(row_index, col_index, column_cells)
    }

    /// Set data for same row multiple columns along with row property.
    /// - Calculation Execution order follow the order of insert.
    /// - Inserting multi column formula will add execution order starting vec 0 of inserted columns
    /// # Arguments
    /// - `row_index` (`u32`) - Provide row index. Starts From 1
    /// - `mut col_index` (`u16`) - Provide column index. Starts From 1
    /// - `mut column_cell` (`Vec<CellProperties>`) - Describe this parameter.
    pub fn set_cell_index_value_mut(
        &mut self,
        row_index: RowIndex,
        mut col_index: ColumnIndex,
        mut column_cells: Vec<CellProperty>,
    ) -> AnyResult<(), AnyError> {
        // Map Start Normalization
        for cell_data in column_cells.iter_mut() {
            if let Some(cell_value) = cell_data.value.as_ref() {
                match cell_data.data_type {
                    CellDataType::Auto => {
                        if cell_value.parse::<f64>().is_ok() {
                            cell_data.data_type = CellDataType::Number;
                        } else if cell_value.parse::<bool>().is_ok() {
                            cell_data.data_type = CellDataType::Boolean;
                            if cell_value
                                .parse::<bool>()
                                .context("draviavemal-openxml_office::Parse Fail")?
                            {
                                cell_data.value = Some("1".to_string());
                            } else {
                                cell_data.value = Some("0".to_string());
                            }
                        } else {
                            cell_data.data_type = CellDataType::ShareString;
                            cell_data.value = Some(self.update_share_string(cell_value)?);
                        }
                    }
                    CellDataType::ShareString => {
                        cell_data.value = Some(self.update_share_string(cell_value)?);
                    }
                    CellDataType::Boolean => {
                        cell_data.value = match cell_value.to_lowercase().as_str() {
                            "false" | "0" | "" => Some("0".to_string()),
                            _ => Some("1".to_string()),
                        }
                    }
                    _ => {}
                }
            } else {
                cell_data.data_type = CellDataType::Number;
            }
        }
        col_index -= 1; // Reduce 1 to normalize the loop increment
        let column_cells = column_cells
            .iter_mut()
            .map(|item| {
                col_index += 1;
                if item.formula.is_some() {
                    // Check and add Calculation entry
                    if let Some(common_service) = self.common_service.upgrade() {
                        if let Some(sheet_collection) = self.sheet_collection.upgrade() {
                            debug!("Sheet name: {}", self.display_sheet_name);
                            let sheet_id = (sheet_collection
                                .borrow()
                                .iter()
                                .position(|(sheet_name, _, _)| {
                                    *sheet_name == self.display_sheet_name
                                })
                                .context("draviavemal-openxml_office::Sheet Not Found To ID")?
                                + 1) as u32;
                            common_service
                                .borrow_mut()
                                .add_replace_calculation_chain(CalculationChain {
                                    cell_ref: ConverterUtil::get_cell_ref(row_index, col_index)
                                        .context("draviavemal-openxml_office::Failed to convert Cell Ref")?,
                                    sheet_id: sheet_id,
                                    level_calcualtion: None,
                                    formula_type: None,
                                    share_formula: None,
                                    array_formula: None,
                                })
                                .context("draviavemal-openxml_office::Failed to insert Calculation Chain Order")?;
                        }
                    }
                }
                self.dimension.start_col = min(self.dimension.start_col, col_index);
                self.dimension.end_col = max(self.dimension.end_col, col_index);
                Ok((col_index, item.clone()))
            })
            .collect::<Result<Vec<(ColumnIndex, CellProperty)>, AnyError>>()
            .context("draviavemal-openxml_office::Failed to Generate column cells")?;
        // Load If Sheet Data Exist
        if let Some(sheet_data) = self.sheet_data.as_mut() {
            // Load If Row Exits
            if let Some(row) = sheet_data.get_mut(&row_index) {
                // Check if the row already has cell data
                if let Some(cell_records) = row.cell_records.as_mut() {
                    // TODO : If Existing Cell Getting Replaced Confirm clean up of Calculation Chain
                    cell_records.extend(column_cells);
                } else {
                    //Create cells If new
                    let mut cell_records = BTreeMap::new();
                    cell_records.extend(column_cells);
                    row.cell_records = Some(cell_records);
                }
            } else {
                // Create If new Row
                let mut cell_records = BTreeMap::new();
                cell_records.extend(column_cells);
                sheet_data.insert(
                    row_index,
                    RowRecords {
                        row_property: RowProperties::default(),
                        cell_records: Some(cell_records),
                    },
                );
            }
        } else {
            // Create Sheet Data
            let mut sheet_data = BTreeMap::new();
            let mut cell_records = BTreeMap::new();
            cell_records.extend(column_cells);
            sheet_data.insert(
                row_index,
                RowRecords {
                    row_property: RowProperties::default(),
                    cell_records: Some(cell_records),
                },
            );
            self.sheet_data = Some(sheet_data);
        }
        Ok(())
    }

    /// Set Cell Range to merge
    /// # Arguments
    /// - `ref_range` (`ReferenceRange`) - Pass the rect. Range to merge cells.
    pub fn set_merge_cell_mut(&mut self, ref_range: ReferenceRange) -> AnyResult<(), AnyError> {
        if let Some(merge_cells) = self.merge_cells.as_mut() {
            if merge_cells
                .iter()
                .any(|existing| ref_range.overlaps(existing))
            {
                return Err(AnyError::msg(
                    "draviavemal-openxml_office::New Record overlap with existing range",
                ));
            }
            merge_cells.push(ref_range);
        } else {
            self.merge_cells = Some(vec![ref_range]);
        }
        Ok(())
    }

    /// Set Hyper Link
    /// # Arguments
    /// - `display` (`Option<String>`) - Diplay name if not provided `link` will be used.
    /// - `link` (`String`) - URL link.
    /// - `range` (`ReferenceRange`) - Provide rect. range for Hyperlink cell range.
    pub fn set_hyperlink_mut(
        &mut self,
        display: Option<String>,
        link: String,
        range: ReferenceRange,
    ) -> AnyResult<(), AnyError> {
        if let Some(hyperlinks) = self.hyperlinks.as_mut() {
            if hyperlinks
                .iter()
                .any(|existing| range.overlaps(&existing.range))
            {
                return Err(AnyError::msg(
                    "draviavemal-openxml_office::New Record overlap with existing range",
                ));
            }
            hyperlinks.push(HyperLinks {
                id: Some("New".to_string()),
                display,
                link,
                range,
            });
        } else {
            self.hyperlinks = Some(vec![HyperLinks {
                id: Some("New".to_string()),
                display,
                link,
                range,
            }]);
        }
        Ok(())
    }

    /// Remove Hyper Link
    /// # Arguments
    /// - `range` (`ReferenceRange`) - Provide rect. range for Hyperlink cell range.
    pub fn remove_hyperlink_mut(&mut self, range: ReferenceRange) -> AnyResult<(), AnyError> {
        if let Some(hyperlinks) = self.hyperlinks.as_mut() {
            hyperlinks.retain(|link| {
                !(link.range.row_start == range.row_start
                    && link.range.row_end == range.row_end
                    && link.range.column_start == range.column_start
                    && link.range.column_end == range.column_end)
            });
        }
        Ok(())
    }

    /// Remove merged cell range
    /// # Arguments
    /// - `range` (`ReferenceRange`) - Provide rect. range for merge cell range.
    pub fn remove_merge_cell_mut(&mut self, range: ReferenceRange) -> AnyResult<(), AnyError> {
        if let Some(merge_range) = self.merge_cells.as_mut() {
            merge_range.retain(|reference_range| {
                !(reference_range.row_start == range.row_start
                    && reference_range.row_end == range.row_end
                    && reference_range.column_start == range.column_start
                    && reference_range.column_end == range.column_end)
            });
        }
        Ok(())
    }

    /// Delete Current sheet and all its components
    pub fn delete_sheet_mut(self) -> AnyResult<(), AnyError> {
        if let Some(sheet_collection) = self.sheet_collection.upgrade() {
            sheet_collection
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull Sheets Collection")?
                .retain(|item| item.0 != self.display_sheet_name);
        }
        if let Some(workbook_relationship_part) = self.workbook_relationship_part.upgrade() {
            workbook_relationship_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull workbook relationship handle")?
                .delete_relationship_mut(&self.file_path);
        }
        if let Some(xml_tree) = self.office_document.upgrade() {
            xml_tree
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Pull XML Handle")?
                .delete_document_mut(&self.file_path);
        }
        self.flush()
            .context("draviavemal-openxml_office::Failed to flush the worksheet")?;
        Ok(())
    }
}

// ##################################### Non Mut Feature Function ################################
impl WorkSheet {
    /// Get the excel cell value properies for provided range
    /// # Arguments
    /// - `range` (`ReferenceRange`) - Provide rect. range to get data.
    /// - 0 value is considered as till the end or start
    pub fn get_range_cell_properties(
        &self,
        range: ReferenceRange,
    ) -> AnyResult<Vec<CellPackage>, AnyError> {
        let mut data_range = Vec::new();
        if let Some(rows) = self.sheet_data.as_ref() {
            if range.row_end == 0 {
                for (row_index, row_records) in rows.range(range.row_start..) {
                    self.extract_cell_records(&range, &mut data_range, row_index, row_records)?;
                }
            } else {
                for (row_index, row_records) in rows.range(range.row_start..=range.row_end) {
                    self.extract_cell_records(&range, &mut data_range, row_index, row_records)?;
                }
            };
        }
        Ok(data_range)
    }

    /// List all Cell Range merged
    pub fn list_merge_cell_(&self) -> Option<Vec<ReferenceRange>> {
        self.merge_cells.clone()
    }

    /// List All hyperlink in the sheet
    pub fn list_hyperlinks(&self) -> Option<Vec<(Option<String>, String, ReferenceRange)>> {
        if let Some(links) = self.hyperlinks.as_ref() {
            Some(
                links
                    .iter()
                    .map(|item| (item.display.clone(), item.link.clone(), item.range.clone()))
                    .collect(),
            )
        } else {
            None
        }
    }
}
