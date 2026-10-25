use crate::{
    files::OfficeDocument,
    global_2007::parts::RelationsPart,
    spreadsheet_2007::{
        models::{
            CellProperty, ColumnIndex, ColumnProperties, HyperLinks, ReferenceRange, RowIndex,
            RowProperties,
        },
        parts::DrawingPart,
        services::CommonServices,
    },
};
use draviavemal_xml_rs::XmlDocument;
use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    rc::{Rc, Weak},
};

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
    // sheet_property: Option<_>,
    pub(crate) sheet_views: WorkSheetViews,
    // sheet_format_property: Option<_>,
    pub(crate) column_collection: Option<VecDeque<ColumnProperties>>,
    pub(crate) sheet_data: Option<BTreeMap<RowIndex, RowRecords>>,
    // sheet_calculation_property:Option<_>
    // protected_range:Option<_>
    pub(crate) merge_cells: Option<Vec<ReferenceRange>>,
    pub(crate) hyperlinks: Option<Vec<HyperLinks>>,
    pub(crate) file_path: String,
    pub(crate) display_sheet_name: String,
    pub(crate) drawing_part: Rc<RefCell<DrawingPart>>,
}
