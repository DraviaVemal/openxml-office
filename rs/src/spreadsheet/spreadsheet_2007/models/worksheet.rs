use crate::global_2007::traits::Enum;
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

impl CellProperties {
    pub fn set_formula(mut self, formula: Option<String>) -> CellProperties {
        self.formula = formula;
        self
    }
    pub fn set_value(mut self, value: Option<String>) -> CellProperties {
        self.value = value;
        self
    }
    pub fn set_data_type(mut self, data_type: CellDataType) -> CellProperties {
        self.data_type = data_type;
        self
    }
    pub fn set_style_id(mut self, style_id: Option<StyleId>) -> CellProperties {
        self.style_id = style_id;
        self
    }
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
pub struct CellPackage {
    pub cell_ref: String,
    pub cell_property: CellProperties,
    pub row_index: u32,
    pub column_index: u16,
}

#[derive(Debug, Clone)]
pub struct ReferenceRange {
    pub column_start: u16,
    pub column_end: u16,
    pub row_start: u32,
    pub row_end: u32,
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
