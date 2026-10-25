use crate::spreadsheet_2007::Excel;
use anyhow::{Error as AnyError, Result as AnyResult};

// ############################# Im-Mut Function   ######################################
impl Excel {
    pub fn list_sheet_names(&self) -> AnyResult<Vec<String>, AnyError> {
        self.get_workbook().list_sheet_names()
    }

    pub fn has_sheets(&self) -> AnyResult<bool, AnyError> {
        self.get_workbook().has_sheets()
    }

    pub fn for_each_sheet_name(&self, callback: impl FnMut(&str)) -> AnyResult<(), AnyError> {
        self.get_workbook().for_each_sheet_name(callback)
    }
}
