use anyhow::{Context, Error as AnyError, Ok, Result as AnyResult};

use crate::spreadsheet_2007::models::{ColumnIndex, RowIndex};

pub struct ConverterUtil;

impl ConverterUtil {
    /// Return int Id of the column
    pub fn get_column_index(cell_ref: &str) -> AnyResult<u16, AnyError> {
        let column_part: String = cell_ref.chars().take_while(|c| c.is_alphabetic()).collect();
        if column_part.is_empty() {
            return Err(AnyError::msg(
                "draviavemal-openxml_office::Failed to Convert to Column Key Id",
            ));
        }
        let mut index = 0;
        for (i, c) in column_part.chars().rev().enumerate() {
            let char_value = c.to_ascii_uppercase() as u16 - 'A' as u16 + 1;
            index += char_value * 26_usize.pow(i as u32) as u16;
        }
        Ok(index)
    }
    /// Return String ref of the column
    pub fn get_column_ref(column_id: u16) -> AnyResult<String, AnyError> {
        if column_id == 0 {
            return Err(AnyError::msg(
                "draviavemal-openxml_office::Index must be greater than 0",
            ));
        }
        let mut index = column_id;
        let mut column_name = String::new();

        while index > 0 {
            index -= 1;
            let char_value = (index % 26) as u8 + b'A';
            column_name.insert(0, char_value as char);
            index /= 26;
        }

        Ok(column_name)
    }

    /// Convert Row & Column Index in to Cell Ref
    /// # Arguments
    /// - `row_index` (`u32`) - Row Index Starting from 1
    /// - `column_index` (`u16`) - Column Index Starting from 1.
    pub fn get_cell_ref(
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) -> AnyResult<String, AnyError> {
        Ok(format!(
            "{}{}",
            ConverterUtil::get_column_ref(column_index)
                .context("draviavemal-openxml_office::Failed to Convert Column to ref")?,
            row_index
        ))
    }

    /// Conver Cell Ref to Row & Column Index
    /// # Arguments
    /// - `cell_ref` (`&str`) - Cell Ref "A1" to convert.
    pub fn get_cell_index(cell_ref: &str) -> AnyResult<(RowIndex, ColumnIndex), AnyError> {
        Ok((
            ConverterUtil::extract_digits(cell_ref)
                .context("draviavemal-openxml_office::Failed to extract int key")?,
            ConverterUtil::get_column_index(cell_ref)
                .context("draviavemal-openxml_office::Failed to Convert to int key")?,
        ))
    }

    /// convert open-xml bool flag property
    pub(crate) fn normalize_bool_property_u8(value: &str) -> u8 {
        match value.trim() {
            "true" | "1" => 1,
            _ => 0,
        }
    }
    /// convert bool to open-xml flag property
    pub(crate) fn bool_xml_flag(value: &bool) -> String {
        if *value {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }
    /// convert open-xml bool flag property
    pub(crate) fn normalize_bool_property_bool(value: &str) -> bool {
        match value.trim() {
            "true" | "1" => true,
            _ => false,
        }
    }

    fn extract_digits(input: &str) -> AnyResult<u32> {
        input
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .context("draviavemal-openxml_office::Failed to Extract Digits")
    }
}
