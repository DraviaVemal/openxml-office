use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    rc::{Rc, Weak},
};

use crate::{
    global_2007::files::OfficeDocument,
    global_2007::parts::RelationsPart,
    spreadsheet_2007::{
        StyleId,
        parts::{worksheet::enum_items::CellDataType, DrawingPart},
        services::CommonServices,
    },
};
use draviavemal_xml_rs::XmlDocument;

pub type ColumnIndex = u16;

pub type RowIndex = u32;

#[derive(Debug, Default)]
pub struct RowProperties {
    // Set Custom height for the row
    pub height: Option<f32>,
    // Hide The Specific Row
    pub style_id: Option<StyleId>,
    pub hidden: Option<bool>,
    pub thick_top: Option<bool>,
    pub thick_bottom: Option<bool>,
    // Column group to use with collapse expand
    pub(crate) group_level: Option<u8>,
    // Collapse the current column
    pub(crate) collapsed: Option<bool>,
    pub(crate) place_holder: Option<bool>,
    pub(crate) span: Option<String>,
}

impl RowProperties {
    pub fn set_height(mut self, height: Option<f32>) -> RowProperties {
        self.height = height;
        self
    }

    pub fn set_style_id(mut self, style_id: Option<StyleId>) -> RowProperties {
        self.style_id = style_id;
        self
    }

    pub fn set_hidden(mut self, hidden: Option<bool>) -> RowProperties {
        self.hidden = hidden;
        self
    }

    pub fn set_thick_top(mut self, thick_top: Option<bool>) -> RowProperties {
        self.thick_top = thick_top;
        self
    }

    pub fn set_thick_bottom(mut self, thick_bottom: Option<bool>) -> RowProperties {
        self.thick_bottom = thick_bottom;
        self
    }
}

#[derive(Debug)]
pub struct ColumnProperties {
    // Start Column index
    pub(crate) min: u16,
    // End Column Index
    pub(crate) max: u16,
    // width value
    pub width: Option<f32>,
    // hide the specific column
    pub hidden: Option<bool>,
    // Column level style setting
    pub style_id: Option<StyleId>,
    // Best fit/auto fit column
    pub best_fit: Option<bool>,
    // Column group to use with collapse expand
    pub(crate) group_level: usize,
    // Collapse the current column
    pub(crate) collapsed: Option<bool>,
}

impl ColumnProperties {
    pub fn set_width(mut self, width: Option<f32>) -> ColumnProperties {
        self.width = width;
        self
    }

    pub fn set_hidden(mut self, hidden: Option<bool>) -> ColumnProperties {
        self.hidden = hidden;
        self
    }

    pub fn set_style_id(mut self, style_id: Option<StyleId>) -> ColumnProperties {
        self.style_id = style_id;
        self
    }

    pub fn set_best_fit(mut self, best_fit: Option<bool>) -> ColumnProperties {
        self.best_fit = best_fit;
        self
    }
}

impl Default for ColumnProperties {
    fn default() -> Self {
        Self {
            min: 1,
            max: 1,
            best_fit: None,
            collapsed: None,
            group_level: 0,
            hidden: None,
            style_id: None,
            width: None,
        }
    }
}

/// Describe this struct.
/// # Fields
/// - `formula` (`Option<String>`) - Describe this field.
/// - `value` (`Option<String>`) - Describe this field.
/// - `data_type` (`CellDataType`) - Describe this field.
/// - `style_id` (`Option<StyleId>`) - Describe this field.
/// - `metadata` (`Option<String>`) - Describe this field.
/// - `comment_id` (`Option<usize>`) - Describe this field.
/// - `place_holder` (`Option<bool>`) - Describe this field.
#[derive(Debug, Clone)]
pub struct CellProperty {
    pub formula: Option<String>,
    pub value: Option<String>,
    pub data_type: CellDataType,
    pub style_id: Option<StyleId>,
    // TODO: Future Items
    pub(crate) metadata: Option<String>,
    pub(crate) comment_id: Option<usize>,
    pub(crate) place_holder: Option<bool>,
}

impl CellProperty {
    /// Cell Formula of the cell. Use set_value to update resolved intital value.
    /// # Arguments
    /// - `formula` (`Option<String>`) - Set Formula for the cell.
    pub fn set_formula(mut self, formula: Option<String>) -> CellProperty {
        self.formula = formula;
        self
    }
    /// Set the value of the current cell
    /// # Arguments
    /// - `value` (`Option<String>`) - Set Formula for the cell.
    pub fn set_value(mut self, value: Option<String>) -> CellProperty {
        self.value = value;
        self
    }
    /// Set data type of the value cell. Use "set_formula" is you are trying to insert formula
    /// # Arguments
    /// - `data_type` (`CellDataType`) - Describe this parameter.
    pub fn set_data_type(mut self, data_type: CellDataType) -> CellProperty {
        self.data_type = data_type;
        self
    }
    /// Update the style id from result of excel book
    /// # Arguments
    /// - `style_id` (`Option<StyleId>`) - Set style value for the current cell.
    pub fn set_style_id(mut self, style_id: Option<StyleId>) -> CellProperty {
        self.style_id = style_id;
        self
    }
}

impl Default for CellProperty {
    fn default() -> Self {
        Self {
            formula: None,
            value: None,
            data_type: CellDataType::Auto,
            style_id: None,
            metadata: None,
            comment_id: None,
            place_holder: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellPackage {
    pub cell_ref: String,
    pub cell_property: CellProperty,
    pub row_index: RowIndex,
    pub column_index: ColumnIndex,
}

#[derive(Debug, Clone)]
pub struct ReferenceRange {
    pub column_start: ColumnIndex,
    pub column_end: ColumnIndex,
    pub row_start: RowIndex,
    pub row_end: RowIndex,
}

impl ReferenceRange {
    pub fn overlaps(&self, other: &ReferenceRange) -> bool {
        self.column_start <= other.column_end
            && self.column_end >= other.column_start
            && self.row_start <= other.row_end
            && self.row_end >= other.row_start
    }
}

impl Default for ReferenceRange {
    fn default() -> Self {
        Self {
            column_start: 1,
            column_end: 1,
            row_start: 1,
            row_end: 1,
        }
    }
}

#[derive(Debug, Clone)]
/// Document Hyperlink
pub struct HyperLinks {
    /// Optional identifier for the hyperlink
    pub id: Option<String>,
    /// Optional display text for the hyperlink
    pub display: Option<String>,
    /// The URL/Target of the hyperlink
    pub link: String,
    /// The range of reference in the document
    pub range: ReferenceRange,
}

#[derive(Debug)]
pub(crate) struct RowRecords {
    pub(crate) row_property: RowProperties,
    pub(crate) cell_records: Option<BTreeMap<ColumnIndex, CellProperty>>,
}

#[derive(Debug)]
pub(crate) struct Dimension {
    pub(crate) start_col: ColumnIndex,
    pub(crate) end_col: ColumnIndex,
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
    pub(crate) pane: Option<String>,
    pub(crate) active_cell: Option<String>,
    pub(crate) active_cell_id: Option<String>,
    pub(crate) sq_ref: Option<String>,
}

#[derive(Debug)]
pub(crate) struct WorkSheetView {
    pub(crate) selection_collection: Option<Vec<WorkSheetViewSelection>>,
    pub(crate) workbook_view_id: String,
    pub(crate) default_grid_color: Option<bool>,
    pub(crate) view_right_to_left: Option<bool>,
    pub(crate) show_formula_bar: Option<bool>,
    pub(crate) show_grid_line: Option<bool>,
    pub(crate) show_outline_symbol: Option<bool>,
    pub(crate) show_row_col_header: Option<bool>,
    pub(crate) show_ruler: Option<bool>,
    pub(crate) show_white_space: Option<bool>,
    pub(crate) show_zero: Option<bool>,
    pub(crate) tab_selected: Option<bool>,
    pub(crate) top_left_cell: Option<String>,
    pub(crate) view: Option<String>,
    pub(crate) window_protection: Option<bool>,
    pub(crate) zoom_scale: Option<i16>,
    pub(crate) zoom_scale_normal: Option<i16>,
    pub(crate) zoom_scale_page_layout: Option<i16>,
    pub(crate) zoom_scale_sheet_layout: Option<i16>,
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
    pub(crate) view_collection: Vec<WorkSheetView>,
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
    pub(crate) office_document: Weak<RefCell<OfficeDocument>>,
    pub(crate) xml_document: Weak<RefCell<XmlDocument>>,
    pub(crate) common_service: Weak<RefCell<CommonServices>>,
    pub(crate) workbook_relationship_part: Weak<RefCell<RelationsPart>>,
    pub(crate) sheet_collection: Weak<RefCell<Vec<(String, String, bool)>>>,
    pub(crate) sheet_relationship_part: Rc<RefCell<RelationsPart>>,
    pub(crate) dimension: Dimension,
    pub(crate) sheet_views: WorkSheetViews,
    pub(crate) column_collection: Option<VecDeque<ColumnProperties>>,
    pub(crate) sheet_data: Option<BTreeMap<RowIndex, RowRecords>>,
    pub(crate) merge_cells: Option<Vec<ReferenceRange>>,
    pub(crate) hyperlinks: Option<Vec<HyperLinks>>,
    pub(crate) file_path: String,
    pub(crate) display_sheet_name: String,
    pub(crate) drawing_part: Rc<RefCell<DrawingPart>>,
}
