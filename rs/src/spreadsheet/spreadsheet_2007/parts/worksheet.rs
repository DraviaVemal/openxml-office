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
    files::{OfficeDocument, XmlDeSerializer, XmlDocument},
    global_2007::{
        parts::RelationsPart,
        traits::{Enum, XmlDocumentPartClose, XmlDocumentPartInitializing},
    },
    log_elapsed,
    order_dictionary::EXCEL_ORDER_COLLECTION,
    spreadsheet_2007::{
        models::{
            CellDataType, CellPackage, CellProperties, ColumnProperties, HyperLinks,
            ReferenceRange, RowProperties, StyleId,
        },
        services::{CalculationChain, CommonServices},
    },
};
use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::{BTreeMap, HashMap, VecDeque},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub(crate) struct RowRecords {
    row_property: RowProperties,
    cell_records: Option<BTreeMap<u16, CellProperties>>,
}

#[derive(Debug)]
pub(crate) struct Dimension {
    start_col: u16,
    end_col: u16,
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
    sheet_collection: Weak<RefCell<Vec<(String, String, bool, bool)>>>,
    sheet_relationship_part: Rc<RefCell<RelationsPart>>,
    dimension: Dimension,
    // sheet_property: Option<_>,
    sheet_views: WorkSheetViews,
    // sheet_format_property: Option<_>,
    column_collection: Option<VecDeque<ColumnProperties>>,
    sheet_data: Option<BTreeMap<u32, RowRecords>>,
    // sheet_calculation_property:Option<_>
    // protected_range:Option<_>
    merge_cells: Option<Vec<ReferenceRange>>,
    hyperlinks: Option<Vec<HyperLinks>>,
    file_path: String,
    sheet_name: String,
}

impl Drop for WorkSheet {
    fn drop(&mut self) {
        let _ = self.close_document();
    }
}

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
                        .context("Failed to pull office document")?;
                    if let Some(xml_document) = self.xml_document.upgrade() {
                        let mut xml_doc_mut = xml_document
                            .try_borrow_mut()
                            .context("Failed to Pull XML Handle")?;
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
                        if let Some(root_element) = xml_doc_mut.get_root_mut() {
                            log_elapsed!(root_element
                                .order_child_mut(
                                    EXCEL_ORDER_COLLECTION
                                        .get("worksheet")
                                        .context("Failed to get worksheet default order")?,
                                )
                                .context("Failed Reorder the element child's"))?;
                        }
                    }
                    log_elapsed!(
                        || {
                            office_doc_mut
                                .close_xml_document(&self.file_path)
                                .context("Failed to close the current tree document")
                        },
                        "Close worksheet document"
                    )?;
                }
                log_elapsed!(
                    || {
                        self.sheet_relationship_part
                            .try_borrow_mut()
                            .context("Failed to pull relationship handle")?
                            .close_document()
                            .context("Failed to Close relationship part")
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
        let content = EXCEL_TYPE_COLLECTION.get("worksheet").unwrap();
        let template_core_properties = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
    xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <sheetData />
</worksheet>"#;
        Ok((
            XmlDeSerializer::vec_to_xml_doc_tree(
                template_core_properties.as_bytes().to_vec(),
                "Default Worksheet",
            )
            .context("Initializing Worksheet Failed")?,
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
        sheet_collection: Weak<RefCell<Vec<(String, String, bool, bool)>>>,
        workbook_relationship_part: Weak<RefCell<RelationsPart>>,
        common_service: Weak<RefCell<CommonServices>>,
        sheet_name: Option<String>,
    ) -> AnyResult<Self, AnyError> {
        let (file_path, sheet_name) = Self::get_sheet_file_name(
            sheet_name,
            &office_document,
            &sheet_collection,
            &workbook_relationship_part,
        )
        .context("Failed to pull worksheet file name")?;
        let xml_document = Self::get_xml_document(&office_document, &file_path)?;
        let sheet_relationship_part = Rc::new(RefCell::new(
            RelationsPart::new(
                office_document.clone(),
                &format!(
                    "{}/_rels/{}.rels",
                    &file_path[..file_path.rfind('/').unwrap()],
                    file_path.rsplit('/').next().unwrap()
                ),
            )
            .context("Creating Relation ship part for workbook failed.")?,
        ));
        let (column_collection, sheet_data, merge_cells, hyperlinks, sheet_views, dimension) = log_elapsed!(
            || {
                Self::initialize_worksheet(&xml_document, Rc::clone(&sheet_relationship_part))
                    .context("Failed to open Worksheet")
            },
            "Worksheet Initialize Time"
        )?;
        Ok(Self {
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
            sheet_name,
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
                .context("Failed to get XML doc handle")?;
            // unwrap dimension
            xml_doc_mut.pop_elements_by_tag_mut("dimension", None);
            let worksheet_views = log_elapsed!(
                || {
                    WorkSheet::deserialize_worksheet_views(&mut xml_doc_mut)
                        .context("Failed to deserialize Worksheet View")
                },
                "Worksheet View Deserialization"
            )?;
            // unwrap columns to local collection
            let column_collection = log_elapsed!(
                || {
                    WorkSheet::deserialize_cols(&mut xml_doc_mut)
                        .context("Failed To Deserialize Cols")
                },
                "Column deserialize"
            )?;
            // unwrap sheet data into object
            let (sheet_data, dimension) = log_elapsed!(
                || {
                    WorkSheet::deserialize_sheet_data(&mut xml_doc_mut)
                        .context("Failed To Deserialize Sheet Data")
                },
                "Sheet Data Deserialize"
            )?;
            let merge_cells = log_elapsed!(
                || {
                    WorkSheet::deserialize_merge_cells(&mut xml_doc_mut)
                        .context("Failed To Deserialize Merge Cells")
                },
                "Merge Cell Deserialize"
            )?;
            let hyperlinks = log_elapsed!(
                || {
                    WorkSheet::deserialize_hyperlinks(&mut xml_doc_mut, &relationship_part)
                        .context("Failed To Deserialize hyperlinks")
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
            let mut dimension_attribute = HashMap::new();
            dimension_attribute.insert("ref".to_string(), "A1".to_string());
            xml_doc_mut
                .append_child_mut("dimension", None)
                .context("Failed to Add Dimension node to worksheet")?
                .set_attribute_mut(dimension_attribute)
                .context("Failed to set attribute value to dimension")?;
            Ok(())
        }
        if let Some(sheet_data) = self.sheet_data.as_ref() {
            if let Some(first_item) = sheet_data.first_key_value() {
                let mut dimension_attribute = HashMap::new();
                dimension_attribute.insert(
                    "ref".to_string(),
                    format!(
                        "{}{}:{}{}",
                        ConverterUtil::get_column_ref(self.dimension.start_col)
                            .context("Failed to convert dim col start")?,
                        first_item.0,
                        ConverterUtil::get_column_ref(self.dimension.end_col)
                            .context("Failed to convert dim col end")?,
                        if let Some(row_end) = sheet_data.last_key_value() {
                            row_end.0
                        } else {
                            first_item.0
                        }
                    ),
                );
                xml_doc_mut
                    .append_child_mut("dimension", None)
                    .context("Failed to Add Dimension node to worksheet")?
                    .set_attribute_mut(dimension_attribute)
                    .context("Failed to set attribute value to dimension")?;
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
                let cols_id = xml_doc_mut
                    .insert_children_after_tag_mut("cols", "sheetFormatPr", None)
                    .context("Failed to Insert Cols Element")?
                    .get_id();
                loop {
                    if let Some(item) = column_collection.pop_front() {
                        let mut attribute = HashMap::new();
                        attribute.insert("min".to_string(), item.min.to_string());
                        attribute.insert("max".to_string(), item.max.to_string());
                        if let Some(width) = item.width {
                            attribute.insert("customWidth".to_string(), "1".to_string());
                            attribute.insert("width".to_string(), width.to_string());
                        }
                        if let Some(style_id) = item.style_id {
                            attribute.insert("style".to_string(), style_id.id.to_string());
                        }
                        if item.hidden.is_some() {
                            attribute.insert("hidden".to_string(), "1".to_string());
                        }
                        if item.best_fit.is_some() {
                            attribute.insert("bestFit".to_string(), "1".to_string());
                        }
                        xml_doc_mut
                            .append_child_mut("col", Some(&cols_id))
                            .context("Failed to insert col record")?
                            .set_attribute_mut(attribute)
                            .context("Failed to Add Attribute to col element")?;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn serialize_sheet_views(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        let sheet_views_id = xml_doc_mut
            .insert_children_after_tag_mut("sheetViews", "dimension", None)
            .context("Failed to Insert Sheet Views Element")?
            .get_id();
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
                    .append_child_mut("sheetView", Some(&sheet_views_id))
                    .context("Failed to insert sheetView record")?
                    .set_attribute_mut(attribute)
                    .context("Failed to Add Attribute to sheetView element")?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn serialize_sheet_data(&mut self, xml_doc_mut: &mut XmlDocument) -> AnyResult<(), AnyError> {
        if let Some(sheet_data) = self.sheet_data.take() {
            let sheet_data_id = xml_doc_mut
                .insert_children_after_tag_mut("sheetData", "cols", None)
                .context("Failed to Insert Cols Element")?
                .get_id();
            for (row_index, db_row) in sheet_data {
                let row_element = xml_doc_mut
                    .append_child_mut("row", Some(&sheet_data_id))
                    .context("Failed to insert row element")?;
                let row_element_id = row_element.get_id();
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
                row_element
                    .set_attribute_mut(row_attribute)
                    .context("Failed to set attribute for row")?;
                if let Some(cols) = db_row.cell_records {
                    for (col_index, cell_record) in cols {
                        // Create cell element
                        let cell_element = xml_doc_mut
                            .append_child_mut("c", Some(&row_element_id))
                            .context("Failed to insert row element")?;
                        let cell_id = cell_element.get_id();
                        let mut cell_attribute = HashMap::new();
                        cell_attribute.insert(
                            "r".to_string(),
                            format!(
                                "{}{}",
                                ConverterUtil::get_column_ref(col_index)
                                    .context("Failed to get Char Id from Int")?,
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
                        cell_element
                            .set_attribute_mut(cell_attribute)
                            .context("Failed to Set Attribute for cell")?;
                        // Create cell's child element
                        match cell_record.data_type {
                            CellDataType::InlineString => {
                                let inline_string_id = xml_doc_mut
                                    .append_child_mut("is", Some(&cell_id))
                                    .context("Failed to insert Inline string element")?
                                    .get_id();
                                let text_element = xml_doc_mut
                                    .append_child_mut("t", Some(&inline_string_id))
                                    .context("Failed To insert Text Value to inline string")?;
                                text_element.set_value_mut(
                                    if let Some(value) = cell_record.value {
                                        value
                                    } else {
                                        "".to_string()
                                    },
                                );
                            }
                            _ => {
                                if let Some(formula) = cell_record.formula {
                                    let formula_element = xml_doc_mut
                                        .append_child_mut("f", Some(&cell_id))
                                        .context("Failed to insert Inline string element")?;
                                    formula_element.set_value_mut(formula);
                                }
                                if let Some(value) = cell_record.value {
                                    xml_doc_mut
                                        .append_child_mut("v", Some(&cell_id))
                                        .context("Failed to insert Inline string element")?
                                        .set_value_mut(value);
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
            let merge_cells_id = xml_doc_mut
                .insert_children_after_tag_mut("mergeCells", "sheetData", None)
                .context("Failed to Insert Cols Element")?
                .get_id();
            {
                let merge_cells_element = xml_doc_mut
                    .get_element_mut(&merge_cells_id)
                    .context("Failed to get element")?;
                let mut attributes = HashMap::new();
                attributes.insert("count".to_string(), merge_cells.len().to_string());
                merge_cells_element
                    .set_attribute_mut(attributes)
                    .context("Failed to set Merge Cells Attribute")?;
            }
            for merge_cell in merge_cells {
                let element = xml_doc_mut
                    .append_child_mut("mergeCell", Some(&merge_cells_id))
                    .context("Failed to add MergeCell Node")?;
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
                element
                    .set_attribute_mut(attribute)
                    .context("Failed to add Merge Cell Attribute")?;
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
            let hyperlinks_id = xml_doc_mut
                .insert_children_after_tag_mut("hyperlinks", "mergeCells", None)
                .context("Failed to Insert Cols Element")?
                .get_id();
            for hyperlink in hyperlinks {
                let hyperlink_element_id = xml_doc_mut
                    .append_child_mut("hyperlink", Some(&hyperlinks_id))
                    .context("Failed tp Add element")?;
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
                    let content = COMMON_TYPE_COLLECTION.get("hyperlink").unwrap();
                    let r_id = relationship_part
                        .borrow_mut()
                        .set_new_relationship_mut(&content, hyperlink.link)
                        .context("Failed to Create Hyperlink Relationship")?;
                    attributes.insert("r:id".to_string(), r_id);
                } else {
                    attributes.insert("location".to_string(), hyperlink.link);
                }
                hyperlink_element_id
                    .set_attribute_mut(attributes)
                    .context("Failed to set hyperlink attribute element")?;
            }
        }
        Ok(())
    }

    fn deserialize_cols(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<Option<VecDeque<ColumnProperties>>, AnyError> {
        if let Some(mut cols_element) = xml_doc_mut.pop_elements_by_tag_mut("cols", None) {
            // Process the columns record if parent node exist
            if let Some(cols) = cols_element.pop() {
                let mut column_collection = VecDeque::with_capacity(cols.get_child_count());
                loop {
                    if let Some((col_elements, _)) = cols.pop_child_mut() {
                        if let Some(col) = xml_doc_mut.pop_element_mut(&col_elements) {
                            let mut column_properties = ColumnProperties::default();
                            let attributes =
                                col.get_attribute().context("Error Getting col attribute")?;
                            if let Some(min) = attributes.get("min") {
                                column_properties.min =
                                    min.parse().context("Failed to parse min value")?;
                            }
                            if let Some(max) = attributes.get("max") {
                                column_properties.max =
                                    max.parse().context("Failed to parse min value")?;
                            }
                            if let Some(best_fit) = attributes.get("bestFit") {
                                column_properties.best_fit =
                                    if best_fit == "1" { Some(true) } else { None }
                            }
                            if let Some(hidden) = attributes.get("hidden") {
                                column_properties.hidden =
                                    if hidden == "1" { Some(true) } else { None }
                            }
                            if let Some(style) = attributes.get("style") {
                                column_properties.style_id = Some(StyleId::new(
                                    style.parse().context("Failed to parse style ID")?,
                                ));
                            }
                            if let Some(outline_level) = attributes.get("outlineLevel") {
                                column_properties.group_level =
                                    outline_level.parse().context("Failed to parse style ID")?;
                            }
                            if let Some(custom_width) = attributes.get("customWidth") {
                                if custom_width == "1" {
                                    column_properties.width = Some(
                                        attributes
                                            .get("width")
                                            .context("Failed to get custom width")?
                                            .parse()
                                            .context("Failed to parse custom width")?,
                                    );
                                }
                            }
                            if let Some(collapsed) = attributes.get("collapsed") {
                                column_properties.collapsed =
                                    if collapsed == "1" { Some(true) } else { None }
                            }
                            column_collection.push_back(column_properties);
                        }
                    } else {
                        break;
                    }
                }
                return Ok(Some(column_collection));
            }
        }
        Ok(None)
    }

    /// DeSerializing Worksheet View
    fn deserialize_worksheet_views(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<WorkSheetViews, AnyError> {
        let mut worksheet_views = WorkSheetViews::default();
        if let Some(mut sheet_views_elements) =
            xml_doc_mut.pop_elements_by_tag_mut("sheetViews", None)
        {
            if let Some(sheet_views_element) = sheet_views_elements.pop() {
                loop {
                    if let Some((element_id, element_tag)) = sheet_views_element.pop_child_mut() {
                        // Validate element that are not accounted
                        if element_tag != "sheetView" {
                            return Err(anyhow!("Failed to Process Sheet Views child"));
                        } else {
                            let sheet_view_element = xml_doc_mut
                                .pop_element_mut(&element_id)
                                .context("Failed to get Sheet View Element")?;
                            let mut worksheet_view = WorkSheetView::default();
                            let attributes = sheet_view_element
                                .get_attribute()
                                .context("Failed to Get Attributes of Sheet View")?;
                            worksheet_view.workbook_view_id = attributes
                                .get("workbookViewId")
                                .context(
                                    "Mandatory attribute \"workbookViewId\" is missing from sheetView",
                                )?
                                .clone();
                            // Windows protection
                            if let Some(window_protection) = attributes.get("windowProtection") {
                                worksheet_view.window_protection = Some(
                                    ConverterUtil::normalize_bool_property_bool(&window_protection),
                                );
                            }
                            // Show formula
                            if let Some(show_formula_bar) = attributes.get("showFormulas") {
                                worksheet_view.show_formula_bar = Some(
                                    ConverterUtil::normalize_bool_property_bool(&show_formula_bar),
                                );
                            }
                            // Show Grid Line
                            if let Some(show_grid_line) = attributes.get("showGridLines") {
                                worksheet_view.show_grid_line = Some(
                                    ConverterUtil::normalize_bool_property_bool(&show_grid_line),
                                );
                            }
                            // Show row column header
                            if let Some(show_row_col_header) = attributes.get("showRowColHeaders") {
                                worksheet_view.show_row_col_header =
                                    Some(ConverterUtil::normalize_bool_property_bool(
                                        &show_row_col_header,
                                    ));
                            }
                            // Show Zero
                            if let Some(show_zero) = attributes.get("showZeros") {
                                worksheet_view.show_zero =
                                    Some(ConverterUtil::normalize_bool_property_bool(&show_zero));
                            }
                            // Right to Left
                            if let Some(view_right_to_left) = attributes.get("rightToLeft") {
                                worksheet_view.view_right_to_left =
                                    Some(ConverterUtil::normalize_bool_property_bool(
                                        &view_right_to_left,
                                    ));
                            }
                            // Tab Selected
                            if let Some(tab_selected) = attributes.get("tabSelected") {
                                worksheet_view.tab_selected = Some(
                                    ConverterUtil::normalize_bool_property_bool(&tab_selected),
                                );
                            }
                            // show ruler
                            if let Some(show_ruler) = attributes.get("showRuler") {
                                worksheet_view.show_ruler =
                                    Some(ConverterUtil::normalize_bool_property_bool(&show_ruler));
                            }
                            // show white space
                            if let Some(show_white_space) = attributes.get("showWhiteSpace") {
                                worksheet_view.show_white_space = Some(
                                    ConverterUtil::normalize_bool_property_bool(&show_white_space),
                                );
                            }
                            // Show outlined Symbols
                            if let Some(show_outline_symbol) = attributes.get("showOutlineSymbols")
                            {
                                worksheet_view.show_outline_symbol =
                                    Some(ConverterUtil::normalize_bool_property_bool(
                                        &show_outline_symbol,
                                    ));
                            }
                            // Default Grid Color
                            if let Some(default_grid_color) = attributes.get("defaultGridColor") {
                                worksheet_view.default_grid_color =
                                    Some(ConverterUtil::normalize_bool_property_bool(
                                        &default_grid_color,
                                    ));
                            }
                            // Top Left Cell
                            if let Some(top_left_cell) = attributes.get("topLeftCell") {
                                worksheet_view.top_left_cell = Some(top_left_cell.clone());
                            }
                            // View Setting
                            if let Some(view) = attributes.get("view") {
                                worksheet_view.view = Some(view.clone());
                            }
                            // Zoom Scale
                            if let Some(zoom_scale) = attributes.get("zoomScale") {
                                worksheet_view.zoom_scale = Some(
                                    zoom_scale
                                        .parse()
                                        .context("Failed to Convert Zoom Normal to i16")?,
                                );
                            }
                            // Zoom Scale Normal
                            if let Some(zoom_scale_normal) = attributes.get("zoomScaleNormal") {
                                worksheet_view.zoom_scale_normal = Some(
                                    zoom_scale_normal
                                        .parse()
                                        .context("Failed to Convert Zoom Normal to i16")?,
                                );
                            }
                            // Zoom Scale Sheet Layout View
                            if let Some(zoom_scale_sheet_layout) =
                                attributes.get("zoomScaleSheetLayoutView")
                            {
                                worksheet_view.zoom_scale_sheet_layout = Some(
                                    zoom_scale_sheet_layout
                                        .parse()
                                        .context("Failed to Convert Zoom Normal to i16")?,
                                );
                            }
                            // Zoom Scale Page Layout View
                            if let Some(zoom_scale_page_layout) =
                                attributes.get("zoomScalePageLayoutView")
                            {
                                worksheet_view.zoom_scale_page_layout = Some(
                                    zoom_scale_page_layout
                                        .parse()
                                        .context("Failed to Convert Zoom Normal to i16")?,
                                );
                            }
                            worksheet_views.view_collection.push(worksheet_view);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(worksheet_views)
    }

    /// Deserialize Sheet Data
    fn deserialize_sheet_data(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<(Option<BTreeMap<u32, RowRecords>>, Dimension), AnyError> {
        let mut dimension = Dimension::default();
        if let Some(mut sheet_data_element) = xml_doc_mut.pop_elements_by_tag_mut("sheetData", None)
        {
            if let Some(sheet_data) = sheet_data_element.pop() {
                let mut sheet_data_collection: BTreeMap<u32, RowRecords> = BTreeMap::new();
                // Loop All rows of sheet data
                loop {
                    if let Some((row_element_id, _)) = sheet_data.pop_child_mut() {
                        if let Some(row_element) = xml_doc_mut.pop_element_mut(&row_element_id) {
                            let mut row_record = RowProperties::default();
                            let row_attribute = row_element
                                .get_attribute()
                                .context("Failed to pull Row Attribute.")?;
                            // Get Row Id
                            let row_index = row_attribute
                                .get("r")
                                .context("Missing mandatory row id attribute")?
                                .parse()
                                .context("Failed to parse row id")?;
                            if let Some(row_span) = row_attribute.get("spans") {
                                row_record.span = Some(row_span.to_string());
                            }
                            if let Some(style_id) = row_attribute.get("s") {
                                if let Some(custom_formant) = row_attribute.get("customFormat") {
                                    row_record.style_id = if custom_formant == "1" {
                                        Some(StyleId::new(
                                            style_id
                                                .parse()
                                                .context("Failed to parse the row style id")?,
                                        ))
                                    } else {
                                        None
                                    };
                                }
                            }
                            if let Some(hidden) = row_attribute.get("hidden") {
                                row_record.hidden = if hidden == "1" { Some(true) } else { None };
                            }
                            if let Some(height) = row_attribute.get("ht") {
                                if let Some(custom_height) = row_attribute.get("customHeight") {
                                    row_record.height = if custom_height == "1" {
                                        Some(
                                            height
                                                .parse()
                                                .context("Failed to parse the row height")?,
                                        )
                                    } else {
                                        None
                                    };
                                }
                            }
                            if let Some(row_group_level) = row_attribute.get("outlineLevel") {
                                let outline_level = row_group_level
                                    .parse()
                                    .context("Failed to parse the row group level")?;
                                row_record.group_level = if outline_level > 0 {
                                    Some(outline_level)
                                } else {
                                    None
                                };
                            }
                            if let Some(collapsed) = row_attribute.get("collapsed") {
                                row_record.collapsed =
                                    if collapsed == "1" { Some(true) } else { None };
                            }
                            if let Some(thick_top) = row_attribute.get("thickTop") {
                                row_record.thick_top =
                                    if thick_top == "1" { Some(true) } else { None };
                            }
                            if let Some(thick_bottom) = row_attribute.get("thickBot") {
                                row_record.thick_bottom = if thick_bottom == "1" {
                                    Some(true)
                                } else {
                                    None
                                };
                            }
                            if let Some(place_holder) = row_attribute.get("ph") {
                                row_record.place_holder = if place_holder == "1" {
                                    Some(true)
                                } else {
                                    None
                                };
                            }
                            let mut cell_records: BTreeMap<u16, CellProperties> = BTreeMap::new();
                            // Loop All Columns of row
                            loop {
                                let mut cell_record = CellProperties::default();
                                if let Some((col_element_id, _)) = row_element.pop_child_mut() {
                                    if let Some(col_element) =
                                        xml_doc_mut.pop_element_mut(&col_element_id)
                                    {
                                        let cell_attribute = col_element
                                            .get_attribute()
                                            .context("Failed to pull attribute for Column")?;
                                        // Get Col Id
                                        let col_index = ConverterUtil::get_column_index(
                                            cell_attribute
                                                .get("r")
                                                .context("Missing mandatory col id attribute")?,
                                        )
                                        .context("Failed to Convert col worksheet initialize")?;
                                        if let Some(style_id) = cell_attribute.get("s") {
                                            cell_record.style_id =
                                                Some(StyleId::new(style_id.parse().context(
                                                    "Failed to parse the col style id",
                                                )?));
                                        }
                                        if let Some(cell_type) = cell_attribute.get("t") {
                                            cell_record.data_type =
                                                CellDataType::get_enum(&cell_type);
                                        } else {
                                            cell_record.data_type = CellDataType::Number;
                                        }
                                        if let Some(comment_id) = cell_attribute.get("cm") {
                                            cell_record.comment_id =
                                                Some(comment_id.parse().context(
                                                    "Failed to parse the col comment id",
                                                )?);
                                        };
                                        if let Some(value_meta_id) = cell_attribute.get("vm") {
                                            cell_record.metadata =
                                                Some(value_meta_id.parse().context(
                                                    "Failed to parse the col value meta id",
                                                )?);
                                        };
                                        if let Some(place_holder) = cell_attribute.get("ph") {
                                            cell_record.place_holder = if place_holder == "1" {
                                                Some(true)
                                            } else {
                                                None
                                            };
                                        };
                                        loop {
                                            if let Some((cell_child_id, _)) =
                                                col_element.pop_child_mut()
                                            {
                                                if let Some(element) =
                                                    xml_doc_mut.pop_element_mut(&cell_child_id)
                                                {
                                                    match element.get_tag() {
                                                        "v" => {
                                                            cell_record.value =
                                                                element.get_value().clone();
                                                        }
                                                        "f" => {
                                                            cell_record.formula =
                                                                element.get_value().clone();
                                                        }
                                                        "is" => {
                                                            if let Some((text_id, _)) =
                                                                element.pop_child_mut()
                                                            {
                                                                if let Some(text_element) =
                                                                    xml_doc_mut
                                                                        .pop_element_mut(&text_id)
                                                                {
                                                                    cell_record.value =
                                                                        text_element
                                                                            .get_value()
                                                                            .clone();
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            return Err(anyhow!(
                                                                "Found un-know element cell child"
                                                            ));
                                                        }
                                                    }
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                        dimension.start_col = min(dimension.start_col, col_index);
                                        dimension.end_col = max(dimension.end_col, col_index);
                                        cell_records.insert(col_index, cell_record);
                                    }
                                } else {
                                    break;
                                }
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
                    } else {
                        break;
                    }
                }
                return Ok((Some(sheet_data_collection), dimension));
            }
        }
        Ok((None, dimension))
    }

    /// Deserialize Merge Cell Collection
    fn deserialize_merge_cells(
        xml_doc_mut: &mut XmlDocument,
    ) -> AnyResult<Option<Vec<ReferenceRange>>> {
        if let Some(mut merge_cells_element) =
            xml_doc_mut.pop_elements_by_tag_mut("mergeCells", None)
        {
            let mut merge_cell_collection = Vec::new();
            if let Some(merge_cells) = merge_cells_element.pop() {
                loop {
                    if let Some((merge_cell_id, _)) = merge_cells.pop_child_mut() {
                        let merge_cell_element = xml_doc_mut
                            .pop_element_mut(&merge_cell_id)
                            .context("Failed to Get Element")?;
                        let attribute = merge_cell_element
                            .get_attribute()
                            .context("Failed to pull Mandatory Attributes")?;
                        let merge_range =
                            attribute.get("ref").context("Failed to get merge ref")?;
                        if merge_range.contains(':') {
                            let range: Vec<&str> = merge_range.split(':').collect();
                            let (row_start, column_start) = ConverterUtil::get_cell_index(range[0])
                                .context("Failed to parse Cell Ref")?;
                            let (row_end, column_end) = ConverterUtil::get_cell_index(range[1])
                                .context("Failed to parse Cell Ref")?;
                            merge_cell_collection.push(ReferenceRange {
                                column_start,
                                row_start,
                                column_end,
                                row_end,
                            });
                        } else {
                            let (row, col) = ConverterUtil::get_cell_index(merge_range)
                                .context("Failed to parse Cell Ref")?;
                            merge_cell_collection.push(ReferenceRange {
                                column_start: col,
                                row_start: row,
                                column_end: col,
                                row_end: row,
                            });
                        }
                    } else {
                        break;
                    }
                }
            }
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
        if let Some(mut hyperlinks_element) =
            xml_doc_mut.pop_elements_by_tag_mut("hyperlinks", None)
        {
            let mut hyperlink_collection = Vec::new();
            if let Some(hyperlinks) = hyperlinks_element.pop() {
                loop {
                    if let Some((hyperlink_id, _)) = hyperlinks.pop_child_mut() {
                        let merge_cell_element = xml_doc_mut
                            .pop_element_mut(&hyperlink_id)
                            .context("Failed to Get Element")?;
                        let attribute = merge_cell_element
                            .get_attribute()
                            .context("Failed to pull Mandatory Attributes")?;
                        let display = attribute.get("display").cloned();
                        let hyperlink_ref = attribute
                            .get("ref")
                            .context("Failed to get hyperlink ref")?;
                        let hyperlink_id = attribute.get("r:id").cloned();
                        let range_reference = if hyperlink_ref.contains(':') {
                            let range: Vec<&str> = hyperlink_ref.split(':').collect();
                            let (row_start, column_start) = ConverterUtil::get_cell_index(range[0])
                                .context("Failed to parse Cell Ref")?;
                            let (row_end, column_end) = ConverterUtil::get_cell_index(range[1])
                                .context("Failed to parse Cell Ref")?;
                            ReferenceRange {
                                column_start,
                                row_start,
                                column_end,
                                row_end,
                            }
                        } else {
                            let (row, col) = ConverterUtil::get_cell_index(hyperlink_ref)
                                .context("Failed to parse Cell Ref")?;
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
                                .context("Failed to Pull Target From Relationship file")?
                                .context("No Target Found in the relationship")?;
                            relationship_part
                                .borrow_mut()
                                .delete_relationship_by_id_mut(&id);
                            link
                        } else {
                            attribute
                                .get("location")
                                .context("Failed to Get Internal Location")?
                                .clone()
                        };
                        hyperlink_collection.push(HyperLinks {
                            id: hyperlink_id,
                            display,
                            link,
                            range: range_reference,
                        });
                    } else {
                        break;
                    }
                }
            }
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
        sheet_name: Option<String>,
        office_document: &Weak<RefCell<OfficeDocument>>,
        sheet_collection: &Weak<RefCell<Vec<(String, String, bool, bool)>>>,
        workbook_relationship_part: &Weak<RefCell<RelationsPart>>,
    ) -> AnyResult<(String, String), AnyError> {
        let worksheet_content = EXCEL_TYPE_COLLECTION.get("worksheet").unwrap();
        if let Some(sheet_collection) = sheet_collection.upgrade() {
            if let Some(workbook_relationship_part) = workbook_relationship_part.upgrade() {
                if let Some(sheet_name) = sheet_name.clone() {
                    // If the Sheet name already exist get the path of sheet name
                    if let Some((_, rel_id, _, _)) = sheet_collection
                        .try_borrow()
                        .context("Failed to Get Sheet Collection")?
                        .iter()
                        .find(|item| item.0 == sheet_name)
                    {
                        return Ok((
                            workbook_relationship_part
                                .try_borrow()
                                .context("Failed to Get Workbook relationship")?
                                .get_target_path_by_id(&rel_id)
                                .context("Failed to Get Target Path")?
                                .context("Failed to Get Relationship path")?,
                            sheet_name,
                        ));
                    }
                }
                let mut sheet_count = sheet_collection
                    .try_borrow()
                    .context("Failed to pull Sheet Name Collection")?
                    .len()
                    + 1;
                let relative_path = workbook_relationship_part
                    .try_borrow_mut()
                    .context("Failed to pull relationship connection")?
                    .get_relative_path()
                    .context("Get Relative Path for Part File")?;
                if let Some(office_doc) = office_document.upgrade() {
                    let document = office_doc
                        .try_borrow()
                        .context("Failed to Borrow Document")?;
                    loop {
                        if document.check_file_exist(format!(
                            "{}{}/{}{}.{}",
                            relative_path,
                            worksheet_content.default_path,
                            worksheet_content.default_name,
                            sheet_count,
                            worksheet_content.extension
                        )) {
                            sheet_count += 1;
                        } else {
                            break;
                        }
                    }
                }
                let file_path = format!("{}{}", relative_path, worksheet_content.default_path);
                let sheet_name = format!(
                    "{}",
                    sheet_name.clone().unwrap_or(format!(
                        "{}{}",
                        worksheet_content.default_name, &sheet_count
                    ))
                );
                let relationship_id = workbook_relationship_part
                    .try_borrow_mut()
                    .context("Failed to Get Relationship Handle")?
                    .set_new_relationship_path_mut(
                        worksheet_content,
                        Some(file_path.clone()),
                        Some(format!(
                            "{}{}",
                            worksheet_content.default_name, &sheet_count
                        )),
                    )
                    .context("Setting New Calculation Chain Relationship Failed.")?;
                sheet_collection
                    .try_borrow_mut()
                    .context("Failed To pull Sheet Collection Handle")?
                    .push((sheet_name.clone(), relationship_id, false, false));
                return Ok((
                    format!(
                        "{}/{}{}.{}",
                        file_path,
                        worksheet_content.default_name,
                        &sheet_count,
                        worksheet_content.extension
                    ),
                    sheet_name,
                ));
            }
        }
        Err(anyhow!("Failed to upgrade relation part"))
    }

    fn update_share_string(&mut self, cell_value: &String) -> AnyResult<String, AnyError> {
        if let Some(common_service) = self.common_service.upgrade() {
            common_service
                .try_borrow_mut()
                .context("Failed to Get Share String Handle")?
                .get_string_id_mut(cell_value.to_owned())
                .context("Failed to get share string id")
        } else {
            Err(anyhow!("Failed to update Share String Record"))
        }
    }

    fn extract_cell_records(
        &self,
        range: &ReferenceRange,
        data_range: &mut Vec<CellPackage>,
        row_index: &u32,
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
                        .context("Failed to parse Cell Ref")?,
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
                        .context("Failed to parse Cell Ref")?,
                        row_index: row_index.clone(),
                        column_index: column_index.clone(),
                        cell_property: self.normalize_cell_property(cell_property)?,
                    });
                }
            }
        }
        Ok(())
    }

    fn normalize_cell_property(
        &self,
        cell_prop: &CellProperties,
    ) -> Result<CellProperties, AnyError> {
        let mut parsed_property = cell_prop.clone();
        if parsed_property.data_type == CellDataType::ShareString {
            if let Some(cmn_service) = self.common_service.upgrade() {
                let actual_value = cmn_service
                    .borrow()
                    .get_string_id_value(parsed_property.value.unwrap())
                    .context("Failed to normalize share string")?;
                parsed_property.value = Some(actual_value);
            }
        }
        Ok(parsed_property)
    }
}

// ##################################### Mut Feature Function ################################
impl WorkSheet {
    /// Set Active cell of the current sheet
    pub(crate) fn set_active_cell_mut(&mut self, selected_range: Vec<&str>) {}

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
            .context("Failed to Get Column index from reference")?;
        self.set_column_index_properties_mut(&col_index, column_properties)
    }

    /// Set Column property
    /// # Arguments
    /// - `col_index` (`&u16`) - Provide column index. Starts From 1.
    /// - `column_properties` (`Option<ColumnProperties>`) - Set the current column property.
    pub fn set_column_index_properties_mut(
        &mut self,
        col_index: &u16,
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
        row_index: &u32,
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
    pub fn set_row_value_ref_mut(
        &mut self,
        cell_ref: &str,
        column_cell: Vec<CellProperties>,
    ) -> AnyResult<(), AnyError> {
        let (row_index, col_index) =
            ConverterUtil::get_cell_index(cell_ref).context("Failed to extract cell key")?;
        self.set_row_value_index_mut(row_index, col_index, column_cell)
    }

    /// Set data for same row multiple columns along with row property.
    /// - Calculation Execution order follow the order of insert.
    /// - Inserting multi column formula will add execution order starting vec 0 of inserted columns
    /// # Arguments
    /// - `row_index` (`u32`) - Provide row index. Starts From 1
    /// - `mut col_index` (`u16`) - Provide column index. Starts From 1
    /// - `mut column_cell` (`Vec<CellProperties>`) - Describe this parameter.
    pub fn set_row_value_index_mut(
        &mut self,
        row_index: u32,
        mut col_index: u16,
        mut column_cell: Vec<CellProperties>,
    ) -> AnyResult<(), AnyError> {
        // Map Start Normalization
        for cell_data in column_cell.iter_mut() {
            if let Some(cell_value) = cell_data.value.as_ref() {
                match cell_data.data_type {
                    CellDataType::Auto => {
                        if cell_value.parse::<f64>().is_ok() {
                            cell_data.data_type = CellDataType::Number;
                        } else if cell_value.parse::<bool>().is_ok() {
                            cell_data.data_type = CellDataType::Boolean;
                            if cell_value.parse::<bool>().context("Parse Fail")? {
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
        let column_cells = column_cell
            .iter_mut()
            .map(|item| {
                col_index += 1;
                if item.formula.is_some() {
                    // Check and add Calculation entry
                    if let Some(common_service) = self.common_service.upgrade() {
                        if let Some(sheet_collection) = self.sheet_collection.upgrade() {
                            common_service
                                .borrow_mut()
                                .add_replace_calculation_chain(CalculationChain {
                                    cell_ref: ConverterUtil::get_cell_ref(row_index, col_index)
                                        .context("Failed to convert Cell Ref")?,
                                    sheet_id: (sheet_collection
                                        .borrow()
                                        .iter()
                                        .position(|(sheet_name, _, _, _)| {
                                            *sheet_name == self.sheet_name
                                        })
                                        .context("Sheet Not Found To ID")?
                                        + 1) as u32,
                                    level_calcualtion: None,
                                    formula_type: None,
                                    share_formula: None,
                                    array_formula: None,
                                })
                                .context("Failed to insert Calculation Chain Order")?;
                        }
                    }
                }
                self.dimension.start_col = min(self.dimension.start_col, col_index);
                self.dimension.end_col = max(self.dimension.end_col, col_index);
                Ok((col_index, item.clone()))
            })
            .collect::<Result<Vec<(u16, CellProperties)>, AnyError>>()
            .context("Failed to Generate column cells")?;
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
            if !merge_cells.iter().any(|range| {
                ref_range.row_end <= range.row_start
                    || ref_range.row_start >= range.row_end
                    || ref_range.column_end >= range.column_start
                    || ref_range.column_start >= range.column_end
            }) {
                return Err(anyhow!("Failed Range Overlap"));
            } else {
                merge_cells.push(ref_range);
            }
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
            if hyperlinks.iter().any(|link| {
                (link.range.row_start >= range.row_start && link.range.row_end <= range.row_start)
                    || (link.range.row_start >= range.row_end
                        && link.range.row_end <= range.row_end)
                    || (link.range.column_start >= range.column_start
                        && link.range.column_end <= range.column_start)
                    || (link.range.column_start >= range.column_end
                        && link.range.column_end <= range.column_end)
            }) {
                // Error if existing any range overlap
                return Err(anyhow!("New Record overlap with existing range"));
            } else {
                hyperlinks.push(HyperLinks {
                    id: Some("New".to_string()),
                    display,
                    link,
                    range,
                });
            }
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
                .context("Failed to pull Sheets Collection")?
                .retain(|item| item.0 != self.sheet_name);
        }
        if let Some(workbook_relationship_part) = self.workbook_relationship_part.upgrade() {
            workbook_relationship_part
                .try_borrow_mut()
                .context("Failed to pull workbook relationship handle")?
                .delete_relationship_mut(&self.file_path);
        }
        if let Some(xml_tree) = self.office_document.upgrade() {
            xml_tree
                .try_borrow_mut()
                .context("Failed to Pull XML Handle")?
                .delete_document_mut(&self.file_path);
        }
        self.flush().context("Failed to flush the worksheet")?;
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
