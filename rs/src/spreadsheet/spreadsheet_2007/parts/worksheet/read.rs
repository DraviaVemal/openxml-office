use crate::spreadsheet_2007::{
    CellPackage, ReferenceRange,
    WorkSheet,
};
use anyhow::{Error as AnyError, Result as AnyResult};

// ##################################### Non Mut Feature Function ################################
impl WorkSheet {
    /// Get the excel cell value properies for provided range
    /// # Arguments
    /// - `range` (`ReferenceRange`) - Provide rect. range to get data.
    /// - 0 value is considered as till the end or start
    pub fn get_range_cell_properties(
        &self,
        range: ReferenceRange,
    ) -> AnyResult<Vec<CellPackage>, AnyError> {
        let mut data_range = Vec::new();
        if let Some(rows) = self.sheet_data.as_ref() {
            if range.row_end == 0 {
                for (row_index, row_records) in rows.range(range.row_start..) {
                    self.extract_cell_records(&range, &mut data_range, row_index, row_records)?;
                }
            } else {
                for (row_index, row_records) in rows.range(range.row_start..=range.row_end) {
                    self.extract_cell_records(&range, &mut data_range, row_index, row_records)?;
                }
            };
        }
        Ok(data_range)
    }

    /// List all Cell Range merged
    pub fn list_merge_cell_(&self) -> Option<Vec<ReferenceRange>> {
        self.merge_cells.clone()
    }

    pub fn for_each_merge_cell(&self, mut callback: impl FnMut(&ReferenceRange)) {
        if let Some(ranges) = self.merge_cells.as_ref() {
            for range in ranges {
                callback(range);
            }
        }
    }

    /// List All hyperlink in the sheet
    pub fn list_hyperlinks(&self) -> Option<Vec<(Option<String>, String, ReferenceRange)>> {
        if let Some(links) = self.hyperlinks.as_ref() {
            Some(
                links
                    .iter()
                    .map(|item| (item.display.clone(), item.link.clone(), item.range.clone()))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn for_each_hyperlink(
        &self,
        mut callback: impl FnMut(Option<&str>, &str, &ReferenceRange),
    ) {
        if let Some(links) = self.hyperlinks.as_ref() {
            for link in links {
                callback(link.display.as_deref(), &link.link, &link.range);
            }
        }
    }
}
