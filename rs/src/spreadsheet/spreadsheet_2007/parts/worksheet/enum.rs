use crate::global::traits::Enum;

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
