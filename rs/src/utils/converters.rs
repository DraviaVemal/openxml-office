use anyhow::{anyhow, Error as AnyError, Result as AnyResult};

pub struct ConverterUtil;

impl ConverterUtil {
    pub fn get_column_int(cell_key: &str) -> AnyResult<usize, AnyError> {
        let column_part: String = cell_key.chars().take_while(|c| c.is_alphabetic()).collect();
        if column_part.is_empty() {
            return Err(anyhow!("Failed to Convert to Column Key Id"));
        }
        let mut index = 0;
        for (i, c) in column_part.chars().rev().enumerate() {
            let char_value = c.to_ascii_uppercase() as usize - 'A' as usize + 1;
            index += char_value * 26_usize.pow(i as u32);
        }
        Ok(index)
    }

    pub fn get_column_key(cell_id: usize) -> AnyResult<String, AnyError> {
        if cell_id == 0 {
            return Err(anyhow!("Index must be greater than 0"));
        }
        let mut index = cell_id;
        let mut column_name = String::new();

        while index > 0 {
            index -= 1;
            let char_value = (index % 26) as u8 + b'A';
            column_name.insert(0, char_value as char);
            index /= 26;
        }

        Ok(column_name)
    }
}
