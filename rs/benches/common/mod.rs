#![allow(dead_code)]

use draviavemal_openxml_office::spreadsheet_2007::{
    models::CellProperty, Excel, ExcelPropertiesModel,
};

pub const COLUMNS_PER_ROW: usize = 100;

pub fn new_excel() -> Excel {
    Excel::new(None, ExcelPropertiesModel::default()).expect("Create new workbook failed")
}

pub fn open_excel(path: &str) -> Excel {
    Excel::new(Some(path.to_string()), ExcelPropertiesModel::default())
        .expect("Open existing workbook failed")
}

/// Populate numbered sheets with numeric cells.
pub fn populate(excel: &mut Excel, sheets: usize, cells_per_sheet: usize) {
    for sheet_index in 0..sheets {
        // Dropping flushes each sheet.
        let mut sheet = excel
            .add_sheet_mut(Some(format!("Sheet_{}", sheet_index)))
            .expect("Add sheet failed");
        let mut written = 0usize;
        let mut row = 1u32;
        while written < cells_per_sheet {
            let count = COLUMNS_PER_ROW.min(cells_per_sheet - written);
            let cells: Vec<CellProperty> = (0..count)
                .map(|column| {
                    CellProperty::default().set_value(Some((written + column).to_string()))
                })
                .collect();
            sheet
                .set_cell_index_value_mut(row, 1, cells)
                .expect("Set row failed");
            written += count;
            row += 1;
        }
    }
}

pub fn edit_first_cell(excel: &mut Excel, sheet_name: &str) {
    let mut sheet = excel
        .get_worksheet_mut(sheet_name.to_string())
        .expect("Worksheet not found");
    sheet
        .set_cell_ref_value_mut(
            "A1",
            vec![CellProperty::default().set_value(Some("edited".to_string()))],
        )
        .expect("Edit failed");
}

pub fn build_fixture(path: &str, sheets: usize, cells_per_sheet: usize) {
    let mut excel = new_excel();
    populate(&mut excel, sheets, cells_per_sheet);
    excel.save_as(path).expect("Save fixture failed");
}
