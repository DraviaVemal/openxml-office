use crate::global_2007::{models::ExcelHyperlinkProperties, traits::Enum};
use crate::spreadsheet_2007::models::StyleId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellDataType {
    /// Use if you want the package to auto detect best fit
    Auto,
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
            CellDataType::Boolean => "b".to_string(),
            CellDataType::String => "str".to_string(),
            CellDataType::ShareString => "s".to_string(),
            CellDataType::InlineString => "inlineStr".to_string(),
            CellDataType::Error => "e".to_string(),
            CellDataType::Number => "n".to_string(),
            _ => "a".to_string(),
        }
    }
    fn get_enum(input_string: &str) -> CellDataType {
        match input_string {
            "a" => CellDataType::Auto,
            "b" => CellDataType::Boolean,
            "str" => CellDataType::String,
            "s" => CellDataType::ShareString,
            "inlineStr" => CellDataType::InlineString,
            "e" => CellDataType::Error,
            _ => CellDataType::Number,
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct CellProperties {
    pub formula: Option<String>,
    pub value: Option<String>,
    pub data_type: CellDataType,
    pub style_id: Option<StyleId>,
    // TODO: Future Items
    pub(crate) metadata: Option<String>,
    pub(crate) comment_id: Option<usize>,
    pub(crate) place_holder: Option<bool>,
}

impl Default for CellProperties {
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
pub struct ReferenceRange {
    pub column_start: u16,
    pub column_end: u16,
    pub row_start: u32,
    pub row_end: u32,
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

