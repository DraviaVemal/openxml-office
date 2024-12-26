use crate::global_2007::models::HyperlinkProperties;
use crate::global_2007::traits::Enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellDataType {
    Number,
    Boolean,
    String,
    ShareString,
    InlineString,
    Error,
}

impl Enum<CellDataType> for CellDataType {
    fn get_string(input_enum: CellDataType) -> String {
        match input_enum {
            CellDataType::Number => "n".to_string(),
            CellDataType::Boolean => "b".to_string(),
            CellDataType::String => "str".to_string(),
            CellDataType::ShareString => "s".to_string(),
            CellDataType::InlineString => "inlineStr".to_string(),
            CellDataType::Error => "e".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> CellDataType {
        match input_string {
            "b" => CellDataType::Boolean,
            "str" => CellDataType::String,
            "s" => CellDataType::ShareString,
            "inlineStr" => CellDataType::InlineString,
            "e" => CellDataType::Error,
            _ => CellDataType::Number,
        }
    }
}

#[derive(Debug)]
pub struct ColumnProperties {
    // Start Column index
    pub(crate) min: usize,
    // End Column Index
    pub(crate) max: usize,
    // width value
    pub width: Option<f32>,
    // hide the specific column
    pub hide: Option<()>,
    // Column level style setting
    pub style_id: Option<usize>,
    // Best fit/auto fit column
    pub best_fit: Option<()>,
    // Column group to use with collapse expand
    pub(crate) group_level: usize,
    // Collapse the current column
    pub(crate) collapsed: Option<()>,
}

impl ColumnProperties {
    pub fn default() -> Self {
        Self {
            min: 1,
            max: 1,
            best_fit: None,
            collapsed: None,
            group_level: 0,
            hide: None,
            style_id: None,
            width: None,
        }
    }
}

#[derive(Debug)]
pub struct RowProperties {
    // Set Custom height for the row
    pub height: Option<usize>,
    // Hide The Specific Row
    pub hidden: bool,
    pub style_id: Option<usize>,
    pub thick_top: bool,
    pub thick_bottom: bool,
    // Column group to use with collapse expand
    pub(crate) group_level: usize,
    // Collapse the current column
    pub(crate) collapsed: bool,
    pub(crate) place_holder: bool,
}

impl RowProperties {
    pub fn default() -> Self {
        Self {
            height: None,
            hidden: false,
            style_id: None,
            group_level: 0,
            collapsed: false,
            thick_top: false,
            thick_bottom: false,
            place_holder: false,
        }
    }
}

#[derive(Debug)]
pub struct ColumnCell {
    pub value: Option<String>,
    pub data_type: Option<CellDataType>,
    pub hyperlink_properties: Option<HyperlinkProperties>,
    pub style_id: Option<usize>,
    pub(crate) comment_id: Option<usize>,
    pub(crate) place_holder: bool,
}

impl ColumnCell {
    pub fn default() -> Self {
        Self {
            value: None,
            data_type: None,
            hyperlink_properties: None,
            style_id: None,
            comment_id: None,
            place_holder: false,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct RowRecord {
    pub(crate) row_id: usize,
    pub(crate) row_hide: Option<bool>,
    pub(crate) row_span: Option<String>,
    pub(crate) row_height: Option<f32>,
    pub(crate) row_style_id: Option<usize>,
    pub(crate) row_thick_top: Option<bool>,
    pub(crate) row_thick_bottom: Option<bool>,
    pub(crate) row_group_level: Option<usize>,
    pub(crate) row_collapsed: Option<bool>,
    pub(crate) row_place_holder: Option<bool>,
    pub(crate) col_id: usize,
    pub(crate) col_style_id: Option<usize>,
    pub(crate) cell_value: Option<String>,
    pub(crate) cell_formula: Option<String>,
    pub(crate) cell_type: Option<CellDataType>,
    pub(crate) cell_metadata: Option<String>,
    pub(crate) cell_place_holder: Option<bool>,
    pub(crate) cell_comment_id: Option<usize>,
}
