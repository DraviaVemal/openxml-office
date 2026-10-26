use std::{
    cmp::{max, min},
    collections::{BTreeMap, VecDeque},
};

use anyhow::{Context, Error as AnyError, Result as AnyResult};
use log::debug;

use crate::{
    converters::ConverterUtil,
    global::traits::XmlDocumentPartFlush,
    spreadsheet_2007::{
        
            CellDataType, CellProperty, ColumnIndex, ColumnProperties, ExcelPictureSetting,
            HyperLinks, ReferenceRange, RowIndex, RowProperties,
        services::CalculationChain,
        worksheet::struct_items::RowRecords,
        WorkSheet,
    },
};

// ##################################### Mut Feature Function ################################
impl WorkSheet {
    /// Set Active cell of the current sheet
    pub(crate) fn set_active_cell_mut(&mut self, _selected_range: Vec<&str>) {}

    pub fn add_picture_mut(
        &mut self,
        image_path: &str,
        picture_setting: ExcelPictureSetting,
    ) -> AnyResult<(), AnyError> {
        let mut drawing_part = self
            .drawing_part
            .try_borrow_mut()
            .context("Failed to get Drawing part")?;
        drawing_part
            .add_picture_mut(image_path, picture_setting)
            .context("Failed to add picture to drawing")?;
        Ok(())
    }

    /// Set Column property
    /// # Arguments
    /// - `cell_ref` (`&str`) - Provide column reference name.
    /// - `column_properties` (`Option<ColumnProperties>`) - Set the current column property.
    pub fn set_column_ref_properties_mut(
        &mut self,
        cell_ref: &str,
        column_properties: Option<ColumnProperties>,
    ) -> AnyResult<(), AnyError> {
        let col_index = ConverterUtil::get_column_index(cell_ref)
            .context("draviavemal-openxml_office::Failed to Get Column index from reference")?;
        self.set_column_index_properties_mut(&col_index, column_properties)
    }

    /// Set Column property
    /// # Arguments
    /// - `col_index` (`&u16`) - Provide column index. Starts From 1.
    /// - `column_properties` (`Option<ColumnProperties>`) - Set the current column property.
    pub fn set_column_index_properties_mut(
        &mut self,
        col_index: &ColumnIndex,
        column_properties: Option<ColumnProperties>,
    ) -> AnyResult<(), AnyError> {
        if self.column_collection.is_none() {
            self.column_collection = Some(VecDeque::new());
        }
        if let Some(column_collection) = self.column_collection.as_mut() {
            let mut new_ranges = VecDeque::new();
            // Delete Old Record
            column_collection.retain_mut(|range| {
                if range.min == *col_index && range.max == *col_index {
                    // Fully matched range, remove it
                    return false;
                } else if range.min <= *col_index && *col_index <= range.max {
                    // Value lies within the range
                    if range.min == *col_index {
                        // Trim the start
                        range.min = col_index + 1;
                    } else if range.max == *col_index {
                        // Trim the end
                        range.max = col_index - 1;
                    } else {
                        // Split the range
                        new_ranges.push_back(ColumnProperties {
                            min: col_index + 1,
                            max: range.max,
                            ..ColumnProperties::default()
                        });
                        range.max = col_index - 1;
                    }
                }
                true
            });
            if let Some(column_properties) = column_properties {
                column_collection.push_back(ColumnProperties {
                    max: *col_index,
                    min: *col_index,
                    ..column_properties
                });
            }
            column_collection.append(&mut new_ranges);
        }
        Ok(())
    }

    /// Set/Reset Row property.
    /// # Arguments
    /// - `row_index` (`&u32`) - Provide row index. Starts From 1
    /// - `row_properties` (`RowProperties`) - Pass Default value or Setting for row
    /// Warning: 0 value will be ignored
    pub fn set_row_index_properties_mut(
        &mut self,
        row_index: &RowIndex,
        row_properties: RowProperties,
    ) -> AnyResult<(), AnyError> {
        if let Some(sheet_data) = self.sheet_data.as_mut() {
            if let Some(row) = sheet_data.get_mut(row_index) {
                row.row_property = row_properties;
            } else {
                sheet_data.insert(
                    *row_index,
                    RowRecords {
                        row_property: row_properties,
                        cell_records: None,
                    },
                );
            }
        } else {
            let mut map = BTreeMap::new();
            map.insert(
                *row_index,
                RowRecords {
                    row_property: row_properties,
                    cell_records: None,
                },
            );
            self.sheet_data = Some(map);
        }
        Ok(())
    }

    /// Set data for same row multiple columns along with row property
    /// # Arguments
    /// - `cell_ref` (`&str`) -  Provide column reference name.
    /// - `column_cells` (`Vec<CellProperty>`) - Set the list of column values auto increment from start ref.
    pub fn set_cell_ref_value_mut(
        &mut self,
        cell_ref: &str,
        column_cells: Vec<CellProperty>,
    ) -> AnyResult<(), AnyError> {
        let (row_index, col_index) = ConverterUtil::get_cell_index(cell_ref)
            .context("draviavemal-openxml_office::Failed to extract cell key")?;
        self.set_cell_index_value_mut(row_index, col_index, column_cells)
    }

    /// Set data for same row multiple columns along with row property.
    /// - Calculation Execution order follow the order of insert.
    /// - Inserting multi column formula will add execution order starting vec 0 of inserted columns
    /// # Arguments
    /// - `row_index` (`u32`) - Provide row index. Starts From 1
    /// - `mut col_index` (`u16`) - Provide column index. Starts From 1
    /// - `mut column_cells` (`Vec<CellProperty>`) - Describe this parameter.
    pub fn set_cell_index_value_mut(
        &mut self,
        row_index: RowIndex,
        mut col_index: ColumnIndex,
        mut column_cells: Vec<CellProperty>,
    ) -> AnyResult<(), AnyError> {
        // Map Start Normalization
        for cell_data in column_cells.iter_mut() {
            if let Some(cell_value) = cell_data.value.as_ref() {
                match cell_data.data_type {
                    CellDataType::Auto => {
                        if cell_value.parse::<f64>().is_ok() {
                            cell_data.data_type = CellDataType::Number;
                        } else if cell_value.parse::<bool>().is_ok() {
                            cell_data.data_type = CellDataType::Boolean;
                            if cell_value
                                .parse::<bool>()
                                .context("draviavemal-openxml_office::Parse Fail")?
                            {
                                cell_data.value = Some("1".to_string());
                            } else {
                                cell_data.value = Some("0".to_string());
                            }
                        } else {
                            cell_data.data_type = CellDataType::ShareString;
                            cell_data.value = Some(self.update_share_string(cell_value)?);
                        }
                    }
                    CellDataType::ShareString => {
                        cell_data.value = Some(self.update_share_string(cell_value)?);
                    }
                    CellDataType::Boolean => {
                        cell_data.value = match cell_value.to_lowercase().as_str() {
                            "false" | "0" | "" => Some("0".to_string()),
                            _ => Some("1".to_string()),
                        }
                    }
                    _ => {}
                }
            } else {
                cell_data.data_type = CellDataType::Number;
            }
        }
        col_index -= 1; // Reduce 1 to normalize the loop increment
        let column_cells = column_cells
            .into_iter()
            .map(|item| {
                col_index += 1;
                if item.formula.is_some() {
                    // Check and add Calculation entry
                    if let Some(common_service) = self.common_service.upgrade() {
                        if let Some(sheet_collection) = self.sheet_collection.upgrade() {
                            debug!("Sheet name: {}", self.display_sheet_name);
                            let sheet_id = (sheet_collection
                                .borrow()
                                .iter()
                                .position(|(sheet_name, _, _)| {
                                    *sheet_name == self.display_sheet_name
                                })
                                .context("draviavemal-openxml_office::Sheet Not Found To ID")?
                                + 1) as u32;
                            common_service
                                .borrow_mut()
                                .add_replace_calculation_chain(CalculationChain {
                                    cell_ref: ConverterUtil::get_cell_ref(row_index, col_index)
                                        .context("draviavemal-openxml_office::Failed to convert Cell Ref")?,
                                    sheet_id: sheet_id,
                                    level_calcualtion: None,
                                    formula_type: None,
                                    share_formula: None,
                                    array_formula: None,
                                })
                                .context("draviavemal-openxml_office::Failed to insert Calculation Chain Order")?;
                        }
                    }
                }
                self.dimension.start_col = min(self.dimension.start_col, col_index);
                self.dimension.end_col = max(self.dimension.end_col, col_index);
                Ok((col_index, item))
            })
            .collect::<Result<Vec<(ColumnIndex, CellProperty)>, AnyError>>()
            .context("draviavemal-openxml_office::Failed to Generate column cells")?;
        // Load If Sheet Data Exist
        if let Some(sheet_data) = self.sheet_data.as_mut() {
            // Load If Row Exits
            if let Some(row) = sheet_data.get_mut(&row_index) {
                // Check if the row already has cell data
                if let Some(cell_records) = row.cell_records.as_mut() {
                    // TODO : If Existing Cell Getting Replaced Confirm clean up of Calculation Chain
                    cell_records.extend(column_cells);
                } else {
                    //Create cells If new
                    let mut cell_records = BTreeMap::new();
                    cell_records.extend(column_cells);
                    row.cell_records = Some(cell_records);
                }
            } else {
                // Create If new Row
                let mut cell_records = BTreeMap::new();
                cell_records.extend(column_cells);
                sheet_data.insert(
                    row_index,
                    RowRecords {
                        row_property: RowProperties::default(),
                        cell_records: Some(cell_records),
                    },
                );
            }
        } else {
            // Create Sheet Data
            let mut sheet_data = BTreeMap::new();
            let mut cell_records = BTreeMap::new();
            cell_records.extend(column_cells);
            sheet_data.insert(
                row_index,
                RowRecords {
                    row_property: RowProperties::default(),
                    cell_records: Some(cell_records),
                },
            );
            self.sheet_data = Some(sheet_data);
        }
        Ok(())
    }

    /// Set Cell Range to merge
    /// # Arguments
    /// - `ref_range` (`ReferenceRange`) - Pass the rect. Range to merge cells.
    pub fn set_merge_cell_mut(&mut self, ref_range: ReferenceRange) -> AnyResult<(), AnyError> {
        if let Some(merge_cells) = self.merge_cells.as_mut() {
            if merge_cells
                .iter()
                .any(|existing| ref_range.overlaps(existing))
            {
                return Err(AnyError::msg(
                    "draviavemal-openxml_office::New Record overlap with existing range",
                ));
            }
            merge_cells.push(ref_range);
        } else {
            self.merge_cells = Some(vec![ref_range]);
        }
        Ok(())
    }

    /// Set Hyper Link
    /// # Arguments
    /// - `display` (`Option<String>`) - Diplay name if not provided `link` will be used.
    /// - `link` (`String`) - URL link.
    /// - `range` (`ReferenceRange`) - Provide rect. range for Hyperlink cell range.
    pub fn set_hyperlink_mut(
        &mut self,
        display: Option<String>,
        link: String,
        range: ReferenceRange,
    ) -> AnyResult<(), AnyError> {
        if let Some(hyperlinks) = self.hyperlinks.as_mut() {
            if hyperlinks
                .iter()
                .any(|existing| range.overlaps(&existing.range))
            {
                return Err(AnyError::msg(
                    "draviavemal-openxml_office::New Record overlap with existing range",
                ));
            }
            hyperlinks.push(HyperLinks {
                id: Some("New".to_string()),
                display,
                link,
                range,
            });
        } else {
            self.hyperlinks = Some(vec![HyperLinks {
                id: Some("New".to_string()),
                display,
                link,
                range,
            }]);
        }
        Ok(())
    }

    /// Remove Hyper Link
    /// # Arguments
    /// - `range` (`ReferenceRange`) - Provide rect. range for Hyperlink cell range.
    pub fn remove_hyperlink_mut(&mut self, range: ReferenceRange) -> AnyResult<(), AnyError> {
        if let Some(hyperlinks) = self.hyperlinks.as_mut() {
            hyperlinks.retain(|link| {
                !(link.range.row_start == range.row_start
                    && link.range.row_end == range.row_end
                    && link.range.column_start == range.column_start
                    && link.range.column_end == range.column_end)
            });
        }
        Ok(())
    }

    /// Remove merged cell range
    /// # Arguments
    /// - `range` (`ReferenceRange`) - Provide rect. range for merge cell range.
    pub fn remove_merge_cell_mut(&mut self, range: ReferenceRange) -> AnyResult<(), AnyError> {
        if let Some(merge_range) = self.merge_cells.as_mut() {
            merge_range.retain(|reference_range| {
                !(reference_range.row_start == range.row_start
                    && reference_range.row_end == range.row_end
                    && reference_range.column_start == range.column_start
                    && reference_range.column_end == range.column_end)
            });
        }
        Ok(())
    }

    /// Delete Current sheet and all its components
    pub fn delete_sheet_mut(self) -> AnyResult<(), AnyError> {
        if let Some(sheet_collection) = self.sheet_collection.upgrade() {
            sheet_collection
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull Sheets Collection")?
                .retain(|item| item.0 != self.display_sheet_name);
        }
        if let Some(workbook_relationship_part) = self.workbook_relationship_part.upgrade() {
            workbook_relationship_part
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull workbook relationship handle")?
                .delete_relationship_mut(&self.file_path);
        }
        if let Some(xml_tree) = self.office_document.upgrade() {
            xml_tree
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to Pull XML Handle")?
                .delete_document_mut(&self.file_path);
        }
        self.flush()
            .context("draviavemal-openxml_office::Failed to flush the worksheet")?;
        Ok(())
    }
}
