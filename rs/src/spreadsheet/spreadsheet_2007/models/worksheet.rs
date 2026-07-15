use crate::global_2007::traits::Enum;
use crate::spreadsheet_2007::models::StyleId;

pub type ColumnIndex = u16;
pub type RowIndex = u32;

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
