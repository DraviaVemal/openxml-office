use crate::spreadsheet_2007::{Excel, WorkbookPart};

// ############################# Mut Function      ######################################
impl Excel {
    pub(crate) fn get_workbook_mut(&mut self) -> &mut WorkbookPart {
        &mut self.workbook
    }

    pub(crate) fn get_workbook(&self) -> &WorkbookPart {
        &self.workbook
    }
}
